//! Rejestrowanie nowej partii i ruchu początkowego w jednej transakcji

use chrono::Utc;
use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::{CreateInventoryLotInput, InventoryLot, InventoryLotStatus};
use sqlx::SqlitePool;
use uuid::Uuid;

use super::mapping::inventory_lot_status_to_db;
use super::quantity::normalize_stock_quantity;
use super::queries::select_inventory_lot;
use crate::repositories::stock_movement::writer::{
    insert_stock_movement, InsertStockMovementInput,
};
use crate::repositories::transaction::begin_immediate;

/// Inserts a lot and records the initial stock as a manual addition
pub(super) async fn create_inventory_lot(
    pool: &SqlitePool,
    input: CreateInventoryLotInput,
) -> ApplicationResult<InventoryLot> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    let units_remaining =
        normalize_stock_quantity(input.units_remaining.unwrap_or(input.units_initial));
    let status = input.status.unwrap_or(InventoryLotStatus::Active);

    tracing::info!(
        lot_id = %id,
        product_id = %input.product_id,
        storage_location_id = ?input.storage_location_id,
        units_initial = input.units_initial,
        units_remaining,
        "Creating SQLite inventory lot",
    );

    let mut tx = begin_immediate(pool).await?;

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
