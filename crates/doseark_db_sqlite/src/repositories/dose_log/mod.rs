mod history;
mod mapping;
mod nutrient_snapshot_writer;
mod queries;
mod serving;
mod stock;

use async_trait::async_trait;
use chrono::Utc;
use doseark_application::{
    ApplicationError, ApplicationResult, DoseLogRepository,
};
use doseark_domain::{CreateDoseLogInput, DoseLog, DoseLogListInput, DoseLogPage, UpdateDoseLogInput};
use sqlx::SqlitePool;
use uuid::Uuid;

use self::mapping::DoseLogRow;
use self::queries::{get_dose_log, insert_dose_log, update_dose_log_row};
use self::serving::validated_multiplier_for_product;
use self::stock::{get_lot, update_lot_stock, validate_lot_for_dose};
use self::nutrient_snapshot_writer::insert_dose_log_nutrient_snapshots;
use crate::repositories::inventory_lot::normalize_stock_quantity;
use crate::repositories::transaction::begin_immediate;
use crate::repositories::stock_movement::writer::{
    insert_stock_movement, InsertStockMovementInput,
};

#[derive(Clone)]
pub struct SqliteDoseLogRepository {
    pool: SqlitePool,
}

impl SqliteDoseLogRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DoseLogRepository for SqliteDoseLogRepository {
    async fn list_dose_logs(&self, input: DoseLogListInput) -> ApplicationResult<DoseLogPage> {
        history::list_dose_logs(&self.pool, input).await
    }

    async fn create_dose_log(&self, input: CreateDoseLogInput) -> ApplicationResult<DoseLog> {
        let now = Utc::now();
        let dose_log_id = Uuid::new_v4();

        tracing::info!(
            dose_log_id = %dose_log_id,
            product_id = %input.product_id,
            inventory_lot_id = ?input.inventory_lot_id,
            units_taken = input.units_taken,
            "Creating SQLite dose log",
        );

        let mut tx = begin_immediate(&self.pool).await?;

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

    async fn update_dose_log(
        &self,
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

        let mut tx = begin_immediate(&self.pool).await?;
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

    async fn cancel_dose_log(&self, id: Uuid) -> ApplicationResult<DoseLog> {
        let now = Utc::now();

        tracing::info!(dose_log_id = %id, "Cancelling SQLite dose log");

        let mut tx = begin_immediate(&self.pool).await?;
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
}
