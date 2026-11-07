//! Zapytania zapisu, aktualizacji i odczytu rekordu dawki

use chrono::{DateTime, Utc};
use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::{CreateDoseLogInput, UpdateDoseLogInput};
use sqlx::{Sqlite, Transaction};
use uuid::Uuid;

use super::super::mapping::DoseLogRow;

/// Inserts the dose row; stock and nutrient snapshots are handled by the caller
pub(in crate::repositories::dose_log) async fn insert_dose_log(
    tx: &mut Transaction<'_, Sqlite>,
    dose_log_id: Uuid,
    input: CreateDoseLogInput,
    now: DateTime<Utc>,
) -> ApplicationResult<DoseLogRow> {
    sqlx::query_as::<_, DoseLogRow>(
        r#"
        INSERT INTO dose_logs (
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
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 'active', NULL, NULL, ?10, ?11)
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
    .bind(dose_log_id.to_string())
    .bind(input.product_id.to_string())
    .bind(input.inventory_lot_id.map(|id| id.to_string()))
    .bind(input.taken_at)
    .bind(input.units_taken)
    .bind(input.unit)
    .bind(input.serving_multiplier)
    .bind(input.reason)
    .bind(input.notes)
    .bind(now)
    .bind(now)
    .fetch_one(&mut **tx)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))
}

/// Updates a dose row and returns its new persisted values
pub(in crate::repositories::dose_log) async fn update_dose_log_row(
    tx: &mut Transaction<'_, Sqlite>,
    id: Uuid,
    input: UpdateDoseLogInput,
    now: DateTime<Utc>,
) -> ApplicationResult<DoseLogRow> {
    sqlx::query_as::<_, DoseLogRow>(
        r#"
        UPDATE dose_logs
        SET
            product_id = ?2,
            inventory_lot_id = ?3,
            taken_at = ?4,
            units_taken = ?5,
            unit = ?6,
            serving_multiplier = ?7,
            reason = ?8,
            notes = ?9,
            updated_at = ?10
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
    .bind(input.product_id.to_string())
    .bind(input.inventory_lot_id.map(|value| value.to_string()))
    .bind(input.taken_at)
    .bind(input.units_taken)
    .bind(input.unit)
    .bind(input.serving_multiplier)
    .bind(input.reason)
    .bind(input.notes)
    .bind(now)
    .fetch_optional(&mut **tx)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?
    .ok_or_else(|| ApplicationError::NotFound(format!("Dose log not found: {id}")))
}

/// Fetches a dose before editing or cancelling it
pub(in crate::repositories::dose_log) async fn get_dose_log(
    tx: &mut Transaction<'_, Sqlite>,
    id: Uuid,
) -> ApplicationResult<DoseLogRow> {
    sqlx::query_as::<_, DoseLogRow>(
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
        WHERE id = ?1
        "#,
    )
    .bind(id.to_string())
    .fetch_optional(&mut **tx)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?
    .ok_or_else(|| ApplicationError::NotFound(format!("Dose log not found: {id}")))
}
