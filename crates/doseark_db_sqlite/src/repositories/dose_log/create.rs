// Transakcja tworzenia dawki
use chrono::Utc;
use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::{CreateDoseLogInput, DoseLog};
use sqlx::SqlitePool;
use uuid::Uuid;

use super::nutrient_snapshot_writer::insert_dose_log_nutrient_snapshots;
use super::queries::insert_dose_log;
use super::serving::validated_multiplier_for_product;
use super::stock::{get_lot, update_lot_stock, validate_lot_for_dose};
use crate::repositories::inventory_lot::normalize_stock_quantity;
use crate::repositories::stock_movement::writer::{insert_stock_movement, InsertStockMovementInput};
use crate::repositories::transaction::begin_immediate;

/// Creates the dose, snapshots nutrients and records any stock usage atomically
pub(super) async fn create_dose_log(
    pool: &SqlitePool,
    input: CreateDoseLogInput,
) -> ApplicationResult<DoseLog> {
    let now = Utc::now();
    let dose_log_id = Uuid::new_v4();

    tracing::info!(
        dose_log_id = %dose_log_id,
        product_id = %input.product_id,
        inventory_lot_id = ?input.inventory_lot_id,
        units_taken = input.units_taken,
        "Creating SQLite dose log",
    );

    let mut tx = begin_immediate(pool).await?;

    // Validate against the stored product definition
    let mut input = input;
    input.serving_multiplier = validated_multiplier_for_product(
        &mut tx,
        input.product_id,
        input.units_taken,
        &input.unit,
        input.serving_multiplier,
    )
    .await?;

    let lot_row = if let Some(lot_id) = input.inventory_lot_id {
        Some(get_lot(&mut tx, lot_id).await?)
    } else {
        None
    };

    validate_lot_for_dose(lot_row.as_ref(), input.product_id, input.units_taken)?;

    let row = insert_dose_log(&mut tx, dose_log_id, input, now).await?;

    insert_dose_log_nutrient_snapshots(
        &mut tx,
        row.id_uuid(),
        row.product_id_uuid(),
        row.serving_multiplier,
    )
    .await?;

    if let Some(lot) = lot_row {
        let quantity_after = normalize_stock_quantity(lot.units_remaining - row.units_taken);
        update_lot_stock(&mut tx, lot.id_uuid(), quantity_after).await?;

        insert_stock_movement(
            &mut tx,
            InsertStockMovementInput {
                inventory_lot_id: lot.id_uuid(),
                product_id: lot.product_id_uuid(),
                dose_log_id: Some(row.id_uuid()),
                movement_type: "dose",
                quantity_delta: -row.units_taken,
                quantity_after,
                reason: "dose_log_create",
                notes: row.notes.clone(),
            },
        )
        .await?;
    }

    tx.commit()
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    tracing::info!(dose_log_id = %row.id.0, "SQLite dose log created");

    DoseLog::try_from(row)
}
