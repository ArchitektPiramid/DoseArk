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
        SELECT id, product_id, units_remaining, status
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

/// Reject held, reserve, expired, discarded or exhausted lots for new consumption.
pub(in crate::repositories::dose_log) fn validate_lot_for_dose(
    lot: &DoseInventoryLotRow,
    product_id: Uuid,
    units_taken: f64,
) -> ApplicationResult<()> {
    if lot.product_id != product_id {
        return Err(ApplicationError::Validation(
            "Selected inventory lot belongs to another product".into(),
        ));
    }
    if lot.status != "active" {
        return Err(ApplicationError::Validation(
            "Only active inventory lots can be used for a new dose".into(),
        ));
    }
    if lot.units_remaining < units_taken {
        return Err(ApplicationError::Validation(
            "Not enough stock in selected inventory lot".into(),
        ));
    }
    Ok(())
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

#[cfg(test)]
mod tests {
    use super::{validate_lot_for_dose, DoseInventoryLotRow};
    use uuid::Uuid;

    #[test]
    fn only_active_lots_are_consumable() {
        let product = Uuid::new_v4();
        for status in [
            "active",
            "reserve",
            "on_hold",
            "empty",
            "expired",
            "discarded",
            "archived",
        ] {
            let lot = DoseInventoryLotRow {
                id: Uuid::new_v4(),
                product_id: product,
                units_remaining: 3.0,
                status: status.to_string(),
            };
            assert_eq!(
                validate_lot_for_dose(&lot, product, 1.0).is_ok(),
                status == "active"
            );
        }
    }

    #[test]
    fn mismatched_product_and_insufficient_stock_are_rejected() {
        let product = Uuid::new_v4();
        let lot = DoseInventoryLotRow {
            id: Uuid::new_v4(),
            product_id: product,
            units_remaining: 1.0,
            status: "active".to_string(),
        };
        assert!(validate_lot_for_dose(&lot, Uuid::new_v4(), 1.0).is_err());
        assert!(validate_lot_for_dose(&lot, product, 2.0).is_err());
        assert!(validate_lot_for_dose(&lot, product, 1.0).is_ok());
    }
}
