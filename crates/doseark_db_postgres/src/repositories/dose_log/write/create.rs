//! Creates active dose records and atomically applies stock and nutrient snapshots.

use chrono::Utc;
use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::{CreateDoseLogInput, DoseLog};
use sqlx::PgPool;
use uuid::Uuid;

use super::super::mapping::DoseLogRow;
use super::super::support::insert_dose_log_nutrient_snapshots;
use super::super::support::validated_multiplier_for_product;
use super::super::support::{
    get_lot_for_update, normalize_stock_quantity, update_lot_stock, validate_lot_for_dose,
};
use crate::repositories::stock_movement::writer::{
    insert_stock_movement, InsertStockMovementInput,
};

pub(in crate::repositories::dose_log) async fn create_dose_log(
    pool: &PgPool,
    input: CreateDoseLogInput,
) -> ApplicationResult<DoseLog> {
    let now = Utc::now();
    let dose_log_id = Uuid::new_v4();

    tracing::info!(
        dose_log_id = %dose_log_id,
        product_id = %input.product_id,
        inventory_lot_id = ?input.inventory_lot_id,
        units_taken = input.units_taken,
        "Creating dose log",
    );

    let mut tx = pool
        .begin()
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

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

    let lot_row = if let Some(lot_id) = input.inventory_lot_id {
        Some(get_lot_for_update(&mut tx, lot_id).await?)
    } else {
        None
    };

    if let Some(lot) = &lot_row {
        validate_lot_for_dose(lot, input.product_id, input.units_taken)?;
    }

    let row = sqlx::query_as::<_, DoseLogRow>(
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
        VALUES (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7,
            $8,
            $9,
            'active',
            NULL,
            NULL,
            $10,
            $11
        )
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
    .bind(dose_log_id)
    .bind(input.product_id)
    .bind(input.inventory_lot_id)
    .bind(input.taken_at)
    .bind(input.units_taken)
    .bind(input.unit)
    .bind(input.serving_multiplier)
    .bind(input.reason)
    .bind(input.notes.clone())
    .bind(now)
    .bind(now)
    .fetch_one(&mut *tx)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    insert_dose_log_nutrient_snapshots(&mut tx, row.id, row.product_id, row.serving_multiplier)
        .await?;

    if let Some(lot) = lot_row {
        let quantity_after = normalize_stock_quantity(lot.units_remaining - row.units_taken);

        update_lot_stock(&mut tx, lot.id, quantity_after).await?;

        insert_stock_movement(
            &mut tx,
            InsertStockMovementInput {
                inventory_lot_id: lot.id,
                product_id: lot.product_id,
                dose_log_id: Some(row.id),
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

    tracing::info!(
        dose_log_id = %row.id,
        product_id = %row.product_id,
        "Dose log created",
    );

    DoseLog::try_from(row)
}
