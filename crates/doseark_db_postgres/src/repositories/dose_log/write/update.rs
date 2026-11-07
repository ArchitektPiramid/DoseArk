//! Updates active doses while keeping stock changes in a PostgreSQL transaction.

use chrono::Utc;
use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::{DoseLog, UpdateDoseLogInput};
use sqlx::PgPool;
use uuid::Uuid;

use super::super::mapping::DoseLogRow;
use super::super::support::insert_dose_log_nutrient_snapshots;
use super::super::support::validated_multiplier_for_product;
use super::super::support::{get_lot_for_update, normalize_stock_quantity, update_lot_stock};
use crate::repositories::stock_movement::writer::{
    insert_stock_movement, InsertStockMovementInput,
};

pub(in crate::repositories::dose_log) async fn update_dose_log(
    pool: &PgPool,
    id: Uuid,
    input: UpdateDoseLogInput,
) -> ApplicationResult<DoseLog> {
    let now = Utc::now();

    tracing::info!(
        dose_log_id = %id,
        product_id = %input.product_id,
        inventory_lot_id = ?input.inventory_lot_id,
        units_taken = input.units_taken,
        "Updating dose log",
    );

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
            "Only active dose logs can be edited".into(),
        ));
    }

    // Use the persisted product definition rather than trusting a client calculation.
    let mut input = input;
    input.serving_multiplier = validated_multiplier_for_product(
        &mut tx,
        input.product_id,
        input.units_taken,
        &input.unit,
        input.serving_multiplier,
    )
    .await?;

    if let Some(previous_lot_id) = previous.inventory_lot_id {
        let previous_lot = get_lot_for_update(&mut tx, previous_lot_id).await?;
        let quantity_after =
            normalize_stock_quantity(previous_lot.units_remaining + previous.units_taken);

        update_lot_stock(&mut tx, previous_lot.id, quantity_after).await?;

        insert_stock_movement(
            &mut tx,
            InsertStockMovementInput {
                inventory_lot_id: previous_lot.id,
                product_id: previous_lot.product_id,
                dose_log_id: Some(previous.id),
                movement_type: "correction",
                quantity_delta: previous.units_taken,
                quantity_after,
                reason: "dose_log_update_reversal",
                notes: previous.notes.clone(),
            },
        )
        .await?;
    }

    let row = sqlx::query_as::<_, DoseLogRow>(
        r#"
        UPDATE dose_logs
        SET
            product_id = $2,
            inventory_lot_id = $3,
            taken_at = $4,
            units_taken = $5,
            unit = $6,
            serving_multiplier = $7,
            reason = $8,
            notes = $9,
            updated_at = $10
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
    .bind(input.product_id)
    .bind(input.inventory_lot_id)
    .bind(input.taken_at)
    .bind(input.units_taken)
    .bind(input.unit)
    .bind(input.serving_multiplier)
    .bind(input.reason)
    .bind(input.notes.clone())
    .bind(now)
    .fetch_one(&mut *tx)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    if let Some(new_lot_id) = row.inventory_lot_id {
        let new_lot = get_lot_for_update(&mut tx, new_lot_id).await?;

        if new_lot.product_id != row.product_id {
            return Err(ApplicationError::Validation(
                "Selected inventory lot belongs to another product".into(),
            ));
        }

        if new_lot.units_remaining < row.units_taken {
            return Err(ApplicationError::Validation(
                "Not enough stock in selected inventory lot".into(),
            ));
        }

        let quantity_after = normalize_stock_quantity(new_lot.units_remaining - row.units_taken);

        update_lot_stock(&mut tx, new_lot.id, quantity_after).await?;

        insert_stock_movement(
            &mut tx,
            InsertStockMovementInput {
                inventory_lot_id: new_lot.id,
                product_id: new_lot.product_id,
                dose_log_id: Some(row.id),
                movement_type: "dose",
                quantity_delta: -row.units_taken,
                quantity_after,
                reason: "dose_log_update_apply",
                notes: row.notes.clone(),
            },
        )
        .await?;
    }

    sqlx::query("DELETE FROM dose_log_nutrients WHERE dose_log_id = $1")
        .bind(row.id)
        .execute(&mut *tx)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    insert_dose_log_nutrient_snapshots(&mut tx, row.id, row.product_id, row.serving_multiplier)
        .await?;

    tx.commit()
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    tracing::info!(dose_log_id = %row.id, "Dose log updated");

    DoseLog::try_from(row)
}
