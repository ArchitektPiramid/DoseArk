//! Aktualizacja dawki, przeliczenie zapasów i zapis składników

use chrono::Utc;
use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::{DoseLog, UpdateDoseLogInput};
use sqlx::SqlitePool;
use uuid::Uuid;

use super::super::support::nutrient_snapshot_writer::insert_dose_log_nutrient_snapshots;
use super::super::support::queries::{get_dose_log, update_dose_log_row};
use super::super::support::serving::validated_multiplier_for_product;
use super::super::support::stock::{get_lot, update_lot_stock, validate_lot_for_dose};
use crate::repositories::inventory_lot::normalize_stock_quantity;
use crate::repositories::stock_movement::writer::{insert_stock_movement, InsertStockMovementInput};
use crate::repositories::transaction::begin_immediate;

/// Reverses the old stock usage before applying the edited dose
/// Everything stays in one transaction so a failed edit leaves stock untouched
pub(in crate::repositories::dose_log) async fn update_dose_log(
    pool: &SqlitePool,
    id: Uuid,
    input: UpdateDoseLogInput,
) -> ApplicationResult<DoseLog> {
    let now = Utc::now();

    tracing::info!(
        dose_log_id = %id,
        product_id = %input.product_id,
        inventory_lot_id = ?input.inventory_lot_id,
        units_taken = input.units_taken,
        "Updating SQLite dose log",
    );

    let mut tx = begin_immediate(pool).await?;
    let previous = get_dose_log(&mut tx, id).await?;

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

    if let Some(previous_lot_id) = previous.inventory_lot_id_uuid() {
        let previous_lot = get_lot(&mut tx, previous_lot_id).await?;
        let quantity_after = normalize_stock_quantity(
            previous_lot.units_remaining + previous.units_taken,
        );

        update_lot_stock(&mut tx, previous_lot.id_uuid(), quantity_after).await?;

        insert_stock_movement(
            &mut tx,
            InsertStockMovementInput {
                inventory_lot_id: previous_lot.id_uuid(),
                product_id: previous_lot.product_id_uuid(),
                dose_log_id: Some(previous.id_uuid()),
                movement_type: "correction",
                quantity_delta: previous.units_taken,
                quantity_after,
                reason: "dose_log_update_reversal",
                notes: previous.notes.clone(),
            },
        )
        .await?;
    }

    let row = update_dose_log_row(&mut tx, id, input, now).await?;

    if let Some(new_lot_id) = row.inventory_lot_id_uuid() {
        let new_lot = get_lot(&mut tx, new_lot_id).await?;
        validate_lot_for_dose(Some(&new_lot), row.product_id_uuid(), row.units_taken)?;

        let quantity_after = normalize_stock_quantity(new_lot.units_remaining - row.units_taken);
        update_lot_stock(&mut tx, new_lot.id_uuid(), quantity_after).await?;

        insert_stock_movement(
            &mut tx,
            InsertStockMovementInput {
                inventory_lot_id: new_lot.id_uuid(),
                product_id: new_lot.product_id_uuid(),
                dose_log_id: Some(row.id_uuid()),
                movement_type: "dose",
                quantity_delta: -row.units_taken,
                quantity_after,
                reason: "dose_log_update_apply",
                notes: row.notes.clone(),
            },
        )
        .await?;
    }

    sqlx::query("DELETE FROM dose_log_nutrients WHERE dose_log_id = ?1")
        .bind(row.id.0.to_string())
        .execute(&mut *tx)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    insert_dose_log_nutrient_snapshots(
        &mut tx,
        row.id_uuid(),
        row.product_id_uuid(),
        row.serving_multiplier,
    )
    .await?;

    tx.commit()
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    tracing::info!(dose_log_id = %row.id.0, "SQLite dose log updated");

    DoseLog::try_from(row)
}
