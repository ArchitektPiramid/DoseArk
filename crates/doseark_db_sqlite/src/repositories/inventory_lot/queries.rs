//! Zapytania odczytu pojedynczej partii i listy partii magazynowych

use doseark_application::{ApplicationError, ApplicationResult};
use sqlx::{Sqlite, Transaction};
use uuid::Uuid;

use super::mapping::InventoryLotRow;

/// Looks up a lot within the current write transaction
pub(super) async fn select_inventory_lot(
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

/// SQL for the inventory list, with an optional named location
pub(super) fn inventory_lot_select_sql() -> &'static str {
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
