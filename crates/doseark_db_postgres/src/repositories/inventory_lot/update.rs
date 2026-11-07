//! Updates inventory lots and records explicit manual stock corrections.

use chrono::Utc;
use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::{InventoryLot, UpdateInventoryLotInput};
use sqlx::PgPool;
use uuid::Uuid;

use super::support::{
    normalize_stock_quantity, select_inventory_lot_for_update, update_inventory_lot_row,
};
use crate::repositories::stock_movement::writer::{
    insert_stock_movement, InsertStockMovementInput,
};

pub(super) async fn update_inventory_lot(
    pool: &PgPool,
    id: Uuid,
    input: UpdateInventoryLotInput,
) -> ApplicationResult<InventoryLot> {
    tracing::info!(
        lot_id = %id,
        product_id = %input.product_id,
        storage_location_id = ?input.storage_location_id,
        units_initial = input.units_initial,
        units_remaining = input.units_remaining,
        "Updating inventory lot",
    );

    let now = Utc::now();
    let mut tx = pool
        .begin()
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    let previous = select_inventory_lot_for_update(&mut tx, id).await?;
    let row = update_inventory_lot_row(&mut tx, id, input, now).await?;
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
                inventory_lot_id: row.id,
                product_id: row.product_id,
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

    tracing::info!(lot_id = %row.id, "Inventory lot updated");

    InventoryLot::try_from(row)
}
