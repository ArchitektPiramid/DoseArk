mod history;
mod mapping;
mod nutrient_snapshot_writer;

use async_trait::async_trait;
use chrono::Utc;
use doseark_application::{
    validated_serving_multiplier, ApplicationError, ApplicationResult, DoseLogRepository,
};
use doseark_domain::{CreateDoseLogInput, DoseLog, DoseLogListInput, DoseLogPage, UpdateDoseLogInput};
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

use self::mapping::{DoseInventoryLotRow, DoseLogRow};
use self::nutrient_snapshot_writer::insert_dose_log_nutrient_snapshots;
use crate::repositories::inventory_lot::normalize_stock_quantity;
use crate::repositories::transaction::begin_immediate;
use crate::repositories::stock_movement::writer::{
    insert_stock_movement, InsertStockMovementInput,
};

#[derive(Clone)]
pub struct SqliteDoseLogRepository {
    pool: SqlitePool,
}

impl SqliteDoseLogRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DoseLogRepository for SqliteDoseLogRepository {
    async fn list_dose_logs(&self, input: DoseLogListInput) -> ApplicationResult<DoseLogPage> {
        history::list_dose_logs(&self.pool, input).await
    }

    async fn create_dose_log(&self, input: CreateDoseLogInput) -> ApplicationResult<DoseLog> {
        let now = Utc::now();
        let dose_log_id = Uuid::new_v4();

        tracing::info!(
            dose_log_id = %dose_log_id,
            product_id = %input.product_id,
            inventory_lot_id = ?input.inventory_lot_id,
            units_taken = input.units_taken,
            "Creating SQLite dose log",
        );

        let mut tx = begin_immediate(&self.pool).await?;

        // Use the persisted product definition rather than trusting a client calculation.
        let mut input = input;
        input.serving_multiplier = validated_multiplier_for_product(
            &mut tx,
            input.product_id,
            input.units_taken,
            &input.unit,
            input.serving_multiplier,
        )
        .await?;

        let lot_row = if let Some(lot_id) = input.inventory_lot_id {
            Some(get_lot(&mut tx, lot_id).await?)
        } else {
            None
        };

        validate_lot_for_dose(lot_row.as_ref(), input.product_id, input.units_taken)?;

        let row = insert_dose_log(&mut tx, dose_log_id, input, now).await?;

        insert_dose_log_nutrient_snapshots(
            &mut tx,
            row.id_uuid(),
            row.product_id_uuid(),
            row.serving_multiplier,
        )
        .await?;

        if let Some(lot) = lot_row {
            let quantity_after = normalize_stock_quantity(lot.units_remaining - row.units_taken);
            update_lot_stock(&mut tx, lot.id_uuid(), quantity_after).await?;

            insert_stock_movement(
                &mut tx,
                InsertStockMovementInput {
                    inventory_lot_id: lot.id_uuid(),
                    product_id: lot.product_id_uuid(),
                    dose_log_id: Some(row.id_uuid()),
                    movement_type: "dose",
                    quantity_delta: -row.units_taken,
                    quantity_after,
                    reason: "dose_log_create",
                    notes: row.notes.clone(),
                },
            )
            .await?;
        }

        tx.commit()
            .await
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        tracing::info!(dose_log_id = %row.id.0, "SQLite dose log created");

        DoseLog::try_from(row)
    }

    async fn update_dose_log(
        &self,
        id: Uuid,
        input: UpdateDoseLogInput,
    ) -> ApplicationResult<DoseLog> {
        let now = Utc::now();

        tracing::info!(
            dose_log_id = %id,
            product_id = %input.product_id,
            inventory_lot_id = ?input.inventory_lot_id,
            units_taken = input.units_taken,
            "Updating SQLite dose log",
        );

        let mut tx = begin_immediate(&self.pool).await?;
        let previous = get_dose_log(&mut tx, id).await?;

        if previous.status != "active" {
            return Err(ApplicationError::Validation(
                "Only active dose logs can be edited".into(),
            ));
        }

        // Use the persisted product definition rather than trusting a client calculation.
        let mut input = input;
        input.serving_multiplier = validated_multiplier_for_product(
            &mut tx,
            input.product_id,
            input.units_taken,
            &input.unit,
            input.serving_multiplier,
        )
        .await?;

        if let Some(previous_lot_id) = previous.inventory_lot_id_uuid() {
            let previous_lot = get_lot(&mut tx, previous_lot_id).await?;
            let quantity_after = normalize_stock_quantity(
                previous_lot.units_remaining + previous.units_taken,
            );

            update_lot_stock(&mut tx, previous_lot.id_uuid(), quantity_after).await?;

            insert_stock_movement(
                &mut tx,
                InsertStockMovementInput {
                    inventory_lot_id: previous_lot.id_uuid(),
                    product_id: previous_lot.product_id_uuid(),
                    dose_log_id: Some(previous.id_uuid()),
                    movement_type: "correction",
                    quantity_delta: previous.units_taken,
                    quantity_after,
                    reason: "dose_log_update_reversal",
                    notes: previous.notes.clone(),
                },
            )
            .await?;
        }

        let row = update_dose_log_row(&mut tx, id, input, now).await?;

        if let Some(new_lot_id) = row.inventory_lot_id_uuid() {
            let new_lot = get_lot(&mut tx, new_lot_id).await?;
            validate_lot_for_dose(Some(&new_lot), row.product_id_uuid(), row.units_taken)?;

            let quantity_after = normalize_stock_quantity(new_lot.units_remaining - row.units_taken);
            update_lot_stock(&mut tx, new_lot.id_uuid(), quantity_after).await?;

            insert_stock_movement(
                &mut tx,
                InsertStockMovementInput {
                    inventory_lot_id: new_lot.id_uuid(),
                    product_id: new_lot.product_id_uuid(),
                    dose_log_id: Some(row.id_uuid()),
                    movement_type: "dose",
                    quantity_delta: -row.units_taken,
                    quantity_after,
                    reason: "dose_log_update_apply",
                    notes: row.notes.clone(),
                },
            )
            .await?;
        }

        sqlx::query("DELETE FROM dose_log_nutrients WHERE dose_log_id = ?1")
            .bind(row.id.0.to_string())
            .execute(&mut *tx)
            .await
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        insert_dose_log_nutrient_snapshots(
            &mut tx,
            row.id_uuid(),
            row.product_id_uuid(),
            row.serving_multiplier,
        )
        .await?;

        tx.commit()
            .await
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        tracing::info!(dose_log_id = %row.id.0, "SQLite dose log updated");

        DoseLog::try_from(row)
    }

    async fn cancel_dose_log(&self, id: Uuid) -> ApplicationResult<DoseLog> {
        let now = Utc::now();

        tracing::info!(dose_log_id = %id, "Cancelling SQLite dose log");

        let mut tx = begin_immediate(&self.pool).await?;
        let previous = get_dose_log(&mut tx, id).await?;

        if previous.status != "active" {
            return Err(ApplicationError::Validation(
                "Only active dose logs can be cancelled".into(),
            ));
        }

        if let Some(lot_id) = previous.inventory_lot_id_uuid() {
            let lot = get_lot(&mut tx, lot_id).await?;
            let quantity_after = normalize_stock_quantity(lot.units_remaining + previous.units_taken);

            update_lot_stock(&mut tx, lot.id_uuid(), quantity_after).await?;

            insert_stock_movement(
                &mut tx,
                InsertStockMovementInput {
                    inventory_lot_id: lot.id_uuid(),
                    product_id: lot.product_id_uuid(),
                    dose_log_id: Some(previous.id_uuid()),
                    movement_type: "correction",
                    quantity_delta: previous.units_taken,
                    quantity_after,
                    reason: "dose_log_cancel",
                    notes: previous.notes.clone(),
                },
            )
            .await?;
        }

        let row = sqlx::query_as::<_, DoseLogRow>(
            r#"
            UPDATE dose_logs
            SET
                status = 'cancelled',
                cancelled_at = ?2,
                cancellation_reason = ?3,
                updated_at = ?2
            WHERE id = ?1
            RETURNING
                id,
                product_id,
                inventory_lot_id,
                taken_at,
                units_taken,
                unit,
                serving_multiplier,
                reason,
                notes,
                status,
                cancelled_at,
                cancellation_reason,
                created_at,
                updated_at
            "#,
        )
        .bind(id.to_string())
        .bind(now)
        .bind("cancelled_from_dosing_history")
        .fetch_one(&mut *tx)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        tx.commit()
            .await
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        tracing::info!(dose_log_id = %row.id.0, "SQLite dose log cancelled");

        DoseLog::try_from(row)
    }
}

async fn insert_dose_log(
    tx: &mut Transaction<'_, Sqlite>,
    dose_log_id: Uuid,
    input: CreateDoseLogInput,
    now: chrono::DateTime<Utc>,
) -> ApplicationResult<DoseLogRow> {
    sqlx::query_as::<_, DoseLogRow>(
        r#"
        INSERT INTO dose_logs (
            id,
            product_id,
            inventory_lot_id,
            taken_at,
            units_taken,
            unit,
            serving_multiplier,
            reason,
            notes,
            status,
            cancelled_at,
            cancellation_reason,
            created_at,
            updated_at
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 'active', NULL, NULL, ?10, ?11)
        RETURNING
            id,
            product_id,
            inventory_lot_id,
            taken_at,
            units_taken,
            unit,
            serving_multiplier,
            reason,
            notes,
            status,
            cancelled_at,
            cancellation_reason,
            created_at,
            updated_at
        "#,
    )
    .bind(dose_log_id.to_string())
    .bind(input.product_id.to_string())
    .bind(input.inventory_lot_id.map(|id| id.to_string()))
    .bind(input.taken_at)
    .bind(input.units_taken)
    .bind(input.unit)
    .bind(input.serving_multiplier)
    .bind(input.reason)
    .bind(input.notes)
    .bind(now)
    .bind(now)
    .fetch_one(&mut **tx)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))
}

async fn update_dose_log_row(
    tx: &mut Transaction<'_, Sqlite>,
    id: Uuid,
    input: UpdateDoseLogInput,
    now: chrono::DateTime<Utc>,
) -> ApplicationResult<DoseLogRow> {
    sqlx::query_as::<_, DoseLogRow>(
        r#"
        UPDATE dose_logs
        SET
            product_id = ?2,
            inventory_lot_id = ?3,
            taken_at = ?4,
            units_taken = ?5,
            unit = ?6,
            serving_multiplier = ?7,
            reason = ?8,
            notes = ?9,
            updated_at = ?10
        WHERE id = ?1
        RETURNING
            id,
            product_id,
            inventory_lot_id,
            taken_at,
            units_taken,
            unit,
            serving_multiplier,
            reason,
            notes,
            status,
            cancelled_at,
            cancellation_reason,
            created_at,
            updated_at
        "#,
    )
    .bind(id.to_string())
    .bind(input.product_id.to_string())
    .bind(input.inventory_lot_id.map(|value| value.to_string()))
    .bind(input.taken_at)
    .bind(input.units_taken)
    .bind(input.unit)
    .bind(input.serving_multiplier)
    .bind(input.reason)
    .bind(input.notes)
    .bind(now)
    .fetch_optional(&mut **tx)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?
    .ok_or_else(|| ApplicationError::NotFound(format!("Dose log not found: {id}")))
}

async fn get_dose_log(
    tx: &mut Transaction<'_, Sqlite>,
    id: Uuid,
) -> ApplicationResult<DoseLogRow> {
    sqlx::query_as::<_, DoseLogRow>(
        r#"
        SELECT
            id,
            product_id,
            inventory_lot_id,
            taken_at,
            units_taken,
            unit,
            serving_multiplier,
            reason,
            notes,
            status,
            cancelled_at,
            cancellation_reason,
            created_at,
            updated_at
        FROM dose_logs
        WHERE id = ?1
        "#,
    )
    .bind(id.to_string())
    .fetch_optional(&mut **tx)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?
    .ok_or_else(|| ApplicationError::NotFound(format!("Dose log not found: {id}")))
}

async fn get_lot(
    tx: &mut Transaction<'_, Sqlite>,
    lot_id: Uuid,
) -> ApplicationResult<DoseInventoryLotRow> {
    sqlx::query_as::<_, DoseInventoryLotRow>(
        r#"
        SELECT id, product_id, units_remaining
        FROM inventory_lots
        WHERE id = ?1
        "#,
    )
    .bind(lot_id.to_string())
    .fetch_optional(&mut **tx)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?
    .ok_or_else(|| ApplicationError::NotFound(format!("Inventory lot not found: {lot_id}")))
}

fn validate_lot_for_dose(
    lot: Option<&DoseInventoryLotRow>,
    product_id: Uuid,
    units_taken: f64,
) -> ApplicationResult<()> {
    let Some(lot) = lot else {
        return Ok(());
    };

    if lot.product_id_uuid() != product_id {
        return Err(ApplicationError::Validation(
            "Selected inventory lot belongs to another product".into(),
        ));
    }

    if lot.units_remaining < units_taken {
        return Err(ApplicationError::Validation(
            "Not enough stock in selected inventory lot".into(),
        ));
    }

    Ok(())
}

async fn update_lot_stock(
    tx: &mut Transaction<'_, Sqlite>,
    lot_id: Uuid,
    quantity_after: f64,
) -> ApplicationResult<()> {
    let result = sqlx::query(
        r#"
        UPDATE inventory_lots
        SET
            units_remaining = ?2,
            status = CASE
                WHEN ?2 = 0 THEN 'empty'
                WHEN status = 'empty' AND ?2 > 0 THEN 'active'
                ELSE status
            END,
            updated_at = ?3
        WHERE id = ?1
        "#,
    )
    .bind(lot_id.to_string())
    .bind(quantity_after)
    .bind(Utc::now())
    .execute(&mut **tx)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(ApplicationError::NotFound(format!(
            "Inventory lot not found: {lot_id}"
        )));
    }

    Ok(())
}

/// Loads the product serving definition within the ongoing SQLite write transaction.
async fn validated_multiplier_for_product(
    tx: &mut Transaction<'_, Sqlite>,
    product_id: Uuid,
    units_taken: f64,
    unit: &str,
    requested_multiplier: f64,
) -> ApplicationResult<f64> {
    let product: Option<(f64, String)> = sqlx::query_as(
        "SELECT serving_size_units, serving_unit FROM products WHERE id = ?1",
    )
    .bind(product_id.to_string())
    .fetch_optional(&mut **tx)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    let (serving_size, serving_unit) = product
        .ok_or_else(|| ApplicationError::NotFound(format!("Product not found: {product_id}")))?;
    validated_serving_multiplier(
        units_taken,
        unit,
        requested_multiplier,
        serving_size,
        &serving_unit,
    )
}
