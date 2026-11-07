//! Row locking, stock writes and precision normalization used by dose mutations.

use chrono::Utc;
use doseark_application::{ApplicationError, ApplicationResult};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use super::super::mapping::DoseInventoryLotRow;

pub(in crate::repositories::dose_log) async fn get_lot_for_update(
    tx: &mut Transaction<'_, Postgres>,
    lot_id: Uuid,
) -> ApplicationResult<DoseInventoryLotRow> {
    sqlx::query_as::<_, DoseInventoryLotRow>(
        r#"
        SELECT id, product_id, units_remaining
        FROM inventory_lots
        WHERE id = $1
        FOR UPDATE
        "#,
    )
    .bind(lot_id)
    .fetch_optional(&mut **tx)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?
    .ok_or_else(|| ApplicationError::NotFound(format!("Inventory lot not found: {lot_id}")))
}

pub(in crate::repositories::dose_log) async fn update_lot_stock(
    tx: &mut Transaction<'_, Postgres>,
    lot_id: Uuid,
    quantity_after: f64,
) -> ApplicationResult<()> {
    sqlx::query(
        r#"
        UPDATE inventory_lots
        SET
            units_remaining = $2,
            status = CASE
                WHEN $2 = 0 THEN 'empty'
                WHEN status = 'empty' AND $2 > 0 THEN 'active'
                ELSE status
            END,
            updated_at = $3
        WHERE id = $1
        "#,
    )
    .bind(lot_id)
    .bind(quantity_after)
    .bind(Utc::now())
    .execute(&mut **tx)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    Ok(())
}

pub(in crate::repositories::dose_log) fn normalize_stock_quantity(value: f64) -> f64 {
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
