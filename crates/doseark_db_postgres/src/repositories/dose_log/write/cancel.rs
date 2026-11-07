//! Cancels recorded doses and reverses their inventory consumption.

use chrono::Utc;
use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::DoseLog;
use sqlx::PgPool;
use uuid::Uuid;

use super::super::mapping::DoseLogRow;
use super::super::support::{get_lot_for_update, normalize_stock_quantity, update_lot_stock};
use crate::repositories::stock_movement::writer::{
    insert_stock_movement, InsertStockMovementInput,
};

pub(in crate::repositories::dose_log) async fn cancel_dose_log(
    pool: &PgPool,
    id: Uuid,
) -> ApplicationResult<DoseLog> {
    let now = Utc::now();

    tracing::info!(dose_log_id = %id, "Cancelling dose log");

    let mut tx = pool
        .begin()
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    let previous = sqlx::query_as::<_, DoseLogRow>(
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
        WHERE id = $1
        FOR UPDATE
        "#,
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?
    .ok_or_else(|| ApplicationError::NotFound(format!("Dose log not found: {id}")))?;

    if previous.status != "active" {
        return Err(ApplicationError::Validation(
            "Only active dose logs can be cancelled".into(),
        ));
    }

    if let Some(lot_id) = previous.inventory_lot_id {
        let lot = get_lot_for_update(&mut tx, lot_id).await?;
        let quantity_after = normalize_stock_quantity(lot.units_remaining + previous.units_taken);

        update_lot_stock(&mut tx, lot.id, quantity_after).await?;

        insert_stock_movement(
            &mut tx,
            InsertStockMovementInput {
                inventory_lot_id: lot.id,
                product_id: lot.product_id,
                dose_log_id: Some(previous.id),
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
            cancelled_at = $2,
            cancellation_reason = $3,
            updated_at = $2
        WHERE id = $1
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
    .bind(id)
    .bind(now)
    .bind("cancelled_from_dosing_history")
    .fetch_one(&mut *tx)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    tx.commit()
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    tracing::info!(dose_log_id = %row.id, "Dose log cancelled");

    DoseLog::try_from(row)
}
