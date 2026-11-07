//! Locked row helpers and SQL definitions shared by inventory commands.

use chrono::Utc;
use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::UpdateInventoryLotInput;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use super::mapping::{inventory_lot_status_to_db, InventoryLotRow};

pub(super) async fn select_inventory_lot_for_update(
    tx: &mut Transaction<'_, Postgres>,
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
        WHERE il.id = $1
        FOR UPDATE OF il
        "#,
    )
    .bind(id)
    .fetch_optional(&mut **tx)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?
    .ok_or_else(|| ApplicationError::NotFound(format!("Inventory lot not found: {id}")))
}

pub(super) async fn update_inventory_lot_row(
    tx: &mut Transaction<'_, Postgres>,
    id: Uuid,
    input: UpdateInventoryLotInput,
    updated_at: chrono::DateTime<Utc>,
) -> ApplicationResult<InventoryLotRow> {
    let normalized_units_remaining = normalize_stock_quantity(input.units_remaining);

    sqlx::query_as::<_, InventoryLotRow>(
        r#"
        WITH updated AS (
            UPDATE inventory_lots
            SET
                product_id = $2,
                lot_number = $3,
                storage_location_id = $4,
                storage_location = $5,
                units_initial = $6,
                units_remaining = $7,
                purchase_date = $8,
                opened_date = $9,
                expires_at = $10,
                price_amount = $11,
                price_currency = $12,
                status = $13,
                notes = $14,
                updated_at = $15
            WHERE id = $1
            RETURNING
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
        SELECT
            updated.id,
            updated.product_id,
            updated.lot_number,
            updated.storage_location_id,
            COALESCE(sl.name, updated.storage_location) AS storage_location,
            updated.units_initial,
            updated.units_remaining,
            updated.purchase_date,
            updated.opened_date,
            updated.expires_at,
            updated.price_amount,
            updated.price_currency,
            updated.status,
            updated.notes,
            updated.created_at,
            updated.updated_at
        FROM updated
        LEFT JOIN storage_locations sl ON sl.id = updated.storage_location_id
        "#,
    )
    .bind(id)
    .bind(input.product_id)
    .bind(input.lot_number)
    .bind(input.storage_location_id)
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
    .bind(updated_at)
    .fetch_optional(&mut **tx)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?
    .ok_or_else(|| ApplicationError::NotFound(format!("Inventory lot not found: {id}")))
}

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
    ORDER BY il.created_at DESC
    "#
}

pub(super) fn normalize_stock_quantity(value: f64) -> f64 {
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
