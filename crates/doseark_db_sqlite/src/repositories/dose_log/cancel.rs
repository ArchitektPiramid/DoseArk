// anulowanie i przywrócenie zapasu
use chrono::Utc;
use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::DoseLog;
use sqlx::SqlitePool;
use uuid::Uuid;

use super::mapping::DoseLogRow;
use super::queries::get_dose_log;
use super::stock::{get_lot, update_lot_stock};
use crate::repositories::inventory_lot::normalize_stock_quantity;
use crate::repositories::stock_movement::writer::{insert_stock_movement, InsertStockMovementInput};
use crate::repositories::transaction::begin_immediate;

/// Cancels a dose and restores its lot quantity within the same transaction
pub(super) async fn cancel_dose_log(pool: &SqlitePool, id: Uuid) -> ApplicationResult<DoseLog> {
    let now = Utc::now();

    tracing::info!(dose_log_id = %id, "Cancelling SQLite dose log");

    let mut tx = begin_immediate(pool).await?;
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
