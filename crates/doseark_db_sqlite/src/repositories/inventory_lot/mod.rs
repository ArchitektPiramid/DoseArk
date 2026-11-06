mod mapping;

use async_trait::async_trait;
use chrono::Utc;
use doseark_application::{ApplicationError, ApplicationResult, InventoryLotRepository};
use doseark_domain::{
    CreateInventoryLotInput, InventoryLot, InventoryLotStatus, UpdateInventoryLotInput,
};
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

use self::mapping::{inventory_lot_status_to_db, InventoryLotRow};
use crate::repositories::stock_movement::writer::{
    insert_stock_movement, InsertStockMovementInput,
};
use crate::repositories::transaction::begin_immediate;

#[derive(Clone)]
pub struct SqliteInventoryLotRepository {
    pool: SqlitePool,
}

impl SqliteInventoryLotRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InventoryLotRepository for SqliteInventoryLotRepository {
    async fn list_inventory_lots(&self) -> ApplicationResult<Vec<InventoryLot>> {
        tracing::trace!("DB query started: list SQLite inventory lots");

        let rows = sqlx::query_as::<_, InventoryLotRow>(inventory_lot_select_sql())
            .fetch_all(&self.pool)
            .await
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        tracing::debug!(
            row_count = rows.len(),
            "DB query finished: list SQLite inventory lots",
        );

        rows.into_iter().map(InventoryLot::try_from).collect()
    }

    async fn create_inventory_lot(
        &self,
        input: CreateInventoryLotInput,
    ) -> ApplicationResult<InventoryLot> {
        let now = Utc::now();
        let id = Uuid::new_v4();
        let units_remaining = normalize_stock_quantity(
            input.units_remaining.unwrap_or(input.units_initial),
        );
        let status = input.status.unwrap_or(InventoryLotStatus::Active);

        tracing::info!(
            lot_id = %id,
            product_id = %input.product_id,
            storage_location_id = ?input.storage_location_id,
            units_initial = input.units_initial,
            units_remaining,
            "Creating SQLite inventory lot",
        );

        let mut tx = begin_immediate(&self.pool).await?;

        sqlx::query(
            r#"
            INSERT INTO inventory_lots (
                id,
                product_id,
                lot_number,
                storage_location_id,
                storage_location,
                units_initial,
                units_remaining,
                purchase_date,
                opened_date,
                expires_at,
                price_amount,
                price_currency,
                status,
                notes,
                created_at,
                updated_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)
            "#,
        )
        .bind(id.to_string())
        .bind(input.product_id.to_string())
        .bind(input.lot_number)
        .bind(input.storage_location_id.map(|value| value.to_string()))
        .bind(input.storage_location)
        .bind(input.units_initial)
        .bind(units_remaining)
        .bind(input.purchase_date)
        .bind(input.opened_date)
        .bind(input.expires_at)
        .bind(input.price_amount)
        .bind(input.price_currency)
        .bind(inventory_lot_status_to_db(&status))
        .bind(input.notes)
        .bind(now)
        .bind(now)
        .execute(&mut *tx)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        let row = select_inventory_lot(&mut tx, id).await?;

        if row.units_remaining > f64::EPSILON {
            insert_stock_movement(
                &mut tx,
                InsertStockMovementInput {
                    inventory_lot_id: row.id_uuid(),
                    product_id: row.product_id_uuid(),
                    dose_log_id: None,
                    movement_type: "manual_add",
                    quantity_delta: row.units_remaining,
                    quantity_after: row.units_remaining,
                    reason: "initial_lot_create",
                    notes: None,
                },
            )
            .await?;
        }

        tx.commit()
            .await
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        tracing::info!(lot_id = %row.id.0, "SQLite inventory lot created");

        InventoryLot::try_from(row)
    }

    async fn update_inventory_lot(
        &self,
        id: Uuid,
        input: UpdateInventoryLotInput,
    ) -> ApplicationResult<InventoryLot> {
        tracing::info!(
            lot_id = %id,
            product_id = %input.product_id,
            storage_location_id = ?input.storage_location_id,
            units_initial = input.units_initial,
            units_remaining = input.units_remaining,
            "Updating SQLite inventory lot",
        );

        let now = Utc::now();
        let mut tx = begin_immediate(&self.pool).await?;
        let previous = select_inventory_lot(&mut tx, id).await?;
        let normalized_units_remaining = normalize_stock_quantity(input.units_remaining);

        let result = sqlx::query(
            r#"
            UPDATE inventory_lots
            SET
                product_id = ?2,
                lot_number = ?3,
                storage_location_id = ?4,
                storage_location = ?5,
                units_initial = ?6,
                units_remaining = ?7,
                purchase_date = ?8,
                opened_date = ?9,
                expires_at = ?10,
                price_amount = ?11,
                price_currency = ?12,
                status = ?13,
                notes = ?14,
                updated_at = ?15
            WHERE id = ?1
            "#,
        )
        .bind(id.to_string())
        .bind(input.product_id.to_string())
        .bind(input.lot_number)
        .bind(input.storage_location_id.map(|value| value.to_string()))
        .bind(input.storage_location)
        .bind(input.units_initial)
        .bind(normalized_units_remaining)
        .bind(input.purchase_date)
        .bind(input.opened_date)
        .bind(input.expires_at)
        .bind(input.price_amount)
        .bind(input.price_currency)
        .bind(inventory_lot_status_to_db(&input.status))
        .bind(input.notes)
        .bind(now)
        .execute(&mut *tx)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(ApplicationError::NotFound(format!(
                "Inventory lot not found: {id}"
            )));
        }

        let row = select_inventory_lot(&mut tx, id).await?;
        let delta = normalize_stock_quantity(row.units_remaining - previous.units_remaining);

        if delta.abs() > f64::EPSILON {
            let movement_type = if delta > 0.0 {
                "manual_add"
            } else {
                "manual_remove"
            };

            insert_stock_movement(
                &mut tx,
                InsertStockMovementInput {
                    inventory_lot_id: row.id_uuid(),
                    product_id: row.product_id_uuid(),
                    dose_log_id: None,
                    movement_type,
                    quantity_delta: delta,
                    quantity_after: row.units_remaining,
                    reason: "manual_inventory_update",
                    notes: None,
                },
            )
            .await?;
        }

        tx.commit()
            .await
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        tracing::info!(lot_id = %row.id.0, "SQLite inventory lot updated");

        InventoryLot::try_from(row)
    }
}

async fn select_inventory_lot(
    tx: &mut Transaction<'_, Sqlite>,
    id: Uuid,
) -> ApplicationResult<InventoryLotRow> {
    sqlx::query_as::<_, InventoryLotRow>(
        r#"
        SELECT
            il.id,
            il.product_id,
            il.lot_number,
            il.storage_location_id,
            COALESCE(sl.name, il.storage_location) AS storage_location,
            il.units_initial,
            il.units_remaining,
            il.purchase_date,
            il.opened_date,
            il.expires_at,
            il.price_amount,
            il.price_currency,
            il.status,
            il.notes,
            il.created_at,
            il.updated_at
        FROM inventory_lots il
        LEFT JOIN storage_locations sl ON sl.id = il.storage_location_id
        WHERE il.id = ?1
        "#,
    )
    .bind(id.to_string())
    .fetch_optional(&mut **tx)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?
    .ok_or_else(|| ApplicationError::NotFound(format!("Inventory lot not found: {id}")))
}

fn inventory_lot_select_sql() -> &'static str {
    r#"
    SELECT
        il.id,
        il.product_id,
        il.lot_number,
        il.storage_location_id,
        COALESCE(sl.name, il.storage_location) AS storage_location,
        il.units_initial,
        il.units_remaining,
        il.purchase_date,
        il.opened_date,
        il.expires_at,
        il.price_amount,
        il.price_currency,
        il.status,
        il.notes,
        il.created_at,
        il.updated_at
    FROM inventory_lots il
    LEFT JOIN storage_locations sl ON sl.id = il.storage_location_id
    ORDER BY il.created_at DESC, il.id DESC
    "#
}

pub(crate) fn normalize_stock_quantity(value: f64) -> f64 {
    const SCALE: f64 = 1_000_000.0;

    if !value.is_finite() {
        return value;
    }

    let rounded = (value * SCALE).round() / SCALE;

    if rounded.abs() < 0.000_001 {
        0.0
    } else {
        rounded
    }
}
