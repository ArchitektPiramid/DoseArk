//! Edycja metadanych partii i rejestracja ręcznej korekty ilości

use chrono::Utc;
use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::{InventoryLot, UpdateInventoryLotInput};
use sqlx::SqlitePool;
use uuid::Uuid;

use super::mapping::inventory_lot_status_to_db;
use super::quantity::normalize_stock_quantity;
use super::queries::select_inventory_lot;
use crate::repositories::stock_movement::writer::{
    insert_stock_movement, InsertStockMovementInput,
};
use crate::repositories::transaction::begin_immediate;

/// Updates lot details, recording only the net stock correction
pub(super) async fn update_inventory_lot(
    pool: &SqlitePool,
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
    let mut tx = begin_immediate(pool).await?;
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
