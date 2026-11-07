// Odczyt, walidacja i aktualizacja stanu partii
use chrono::Utc;
use doseark_application::{ApplicationError, ApplicationResult};
use sqlx::{Sqlite, Transaction};
use uuid::Uuid;

use super::mapping::DoseInventoryLotRow;

/// reads the current lot balance inside the dose transaction
pub(super) async fn get_lot(
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

/// Checks product ownership and available stock before a dose is applied
pub(super) fn validate_lot_for_dose(
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

/// Persists a new stock balance and adjusts empty/active status
pub(super) async fn update_lot_stock(
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
