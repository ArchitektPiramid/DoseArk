mod mapping;
mod nutrient_snapshot_writer;

use async_trait::async_trait;
use chrono::Utc;
use doseark_application::{
    validated_serving_multiplier, ApplicationError, ApplicationResult, DoseLogRepository,
};
use doseark_domain::{CreateDoseLogInput, DoseLog, DoseLogListInput, DoseLogPage, UpdateDoseLogInput};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::repositories::dose_log::mapping::{
    DoseInventoryLotRow, DoseLogHistoryRow, DoseLogRow,
};
use crate::repositories::dose_log::nutrient_snapshot_writer::insert_dose_log_nutrient_snapshots;
use crate::repositories::stock_movement::writer::{
    insert_stock_movement, InsertStockMovementInput,
};

#[derive(Clone)]
pub struct PgDoseLogRepository {
    pool: PgPool,
}

impl PgDoseLogRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DoseLogRepository for PgDoseLogRepository {
    async fn list_dose_logs(&self, input: DoseLogListInput) -> ApplicationResult<DoseLogPage> {
        tracing::trace!(
            limit = input.limit,
            offset = input.offset,
            product_id = ?input.product_id,
            "DB query started: list dose logs",
        );

        let (total,): (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM dose_logs
            WHERE status = 'active'
              AND ($1::uuid IS NULL OR product_id = $1)
            "#,
        )
        .bind(input.product_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        let rows = sqlx::query_as::<_, DoseLogHistoryRow>(
            r#"
            WITH active_doses AS (
                SELECT
                    dl.id,
                    dl.product_id,
                    dl.inventory_lot_id,
                    dl.taken_at,
                    dl.units_taken,
                    dl.unit,
                    dl.serving_multiplier,
                    dl.reason,
                    dl.notes,
                    dl.status,
                    dl.cancelled_at,
                    dl.cancellation_reason,
                    dl.created_at,
                    dl.updated_at,
                    il.units_initial AS inventory_units_initial,
                    il.units_remaining AS current_units_remaining,
                    SUM(dl.units_taken) OVER (
                        PARTITION BY dl.inventory_lot_id
                    ) AS total_active_units_taken,
                    SUM(dl.units_taken) OVER (
                        PARTITION BY dl.inventory_lot_id
                        ORDER BY dl.taken_at ASC, dl.created_at ASC, dl.id ASC
                        ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
                    ) AS cumulative_units_taken
                FROM dose_logs dl
                LEFT JOIN inventory_lots il ON il.id = dl.inventory_lot_id
                WHERE dl.status = 'active'
            ),
            history AS (
                SELECT
                    id,
                    product_id,
                    inventory_lot_id,
                    taken_at,
                    units_taken,
                    unit,
                    serving_multiplier,
                    CASE
                        WHEN inventory_lot_id IS NULL OR current_units_remaining IS NULL THEN NULL
                        ELSE ROUND((
                            current_units_remaining
                            + total_active_units_taken
                            - cumulative_units_taken
                            + units_taken
                        )::numeric, 6)::double precision
                    END AS inventory_quantity_before,
                    CASE
                        WHEN inventory_lot_id IS NULL OR current_units_remaining IS NULL THEN NULL
                        ELSE ROUND((
                            current_units_remaining
                            + total_active_units_taken
                            - cumulative_units_taken
                        )::numeric, 6)::double precision
                    END AS inventory_quantity_after,
                    inventory_units_initial,
                    reason,
                    notes,
                    status,
                    cancelled_at,
                    cancellation_reason,
                    created_at,
                    updated_at
                FROM active_doses
            )
            SELECT
                id,
                product_id,
                inventory_lot_id,
                taken_at,
                units_taken,
                unit,
                serving_multiplier,
                inventory_quantity_before,
                inventory_quantity_after,
                inventory_units_initial,
                reason,
                notes,
                status,
                cancelled_at,
                cancellation_reason,
                created_at,
                updated_at
            FROM history
            WHERE ($1::uuid IS NULL OR product_id = $1)
            ORDER BY taken_at DESC, created_at DESC, id DESC
            LIMIT $2
            OFFSET $3
            "#,
        )
        .bind(input.product_id)
        .bind(input.limit)
        .bind(input.offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        tracing::debug!(
            row_count = rows.len(),
            total,
            "DB query finished: list dose logs",
        );

        let items = rows
            .into_iter()
            .map(DoseLog::try_from)
            .collect::<ApplicationResult<Vec<_>>>()?;

        Ok(DoseLogPage {
            items,
            total,
            limit: input.limit,
            offset: input.offset,
        })
    }

    async fn create_dose_log(&self, input: CreateDoseLogInput) -> ApplicationResult<DoseLog> {
        let now = Utc::now();
        let dose_log_id = Uuid::new_v4();

        tracing::info!(
            dose_log_id = %dose_log_id,
            product_id = %input.product_id,
            inventory_lot_id = ?input.inventory_lot_id,
            units_taken = input.units_taken,
            "Creating dose log",
        );

        let mut tx = self
            .pool
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
            if lot.product_id != input.product_id {
                return Err(ApplicationError::Validation(
                    "Selected inventory lot belongs to another product".into(),
                ));
            }

            if lot.units_remaining < input.units_taken {
                return Err(ApplicationError::Validation(
                    "Not enough stock in selected inventory lot".into(),
                ));
            }
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

        insert_dose_log_nutrient_snapshots(
            &mut tx,
            row.id,
            row.product_id,
            row.serving_multiplier,
        )
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
            "Updating dose log",
        );

        let mut tx = self
            .pool
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
            let quantity_after = normalize_stock_quantity(
                previous_lot.units_remaining + previous.units_taken
            );

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

        insert_dose_log_nutrient_snapshots(
            &mut tx,
            row.id,
            row.product_id,
            row.serving_multiplier,
        )
        .await?;

        tx.commit()
            .await
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        tracing::info!(dose_log_id = %row.id, "Dose log updated");

        DoseLog::try_from(row)
    }

    async fn cancel_dose_log(&self, id: Uuid) -> ApplicationResult<DoseLog> {
        let now = Utc::now();

        tracing::info!(dose_log_id = %id, "Cancelling dose log");

        let mut tx = self
            .pool
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
}

async fn get_lot_for_update(
    tx: &mut Transaction<'_, Postgres>,
    lot_id: Uuid,
) -> ApplicationResult<DoseInventoryLotRow> {
    sqlx::query_as::<_, DoseInventoryLotRow>(
        r#"
        SELECT id, product_id, units_remaining
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

async fn update_lot_stock(
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

fn normalize_stock_quantity(value: f64) -> f64 {
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

/// Locks the serving definition so an intake cannot race with a product update.
async fn validated_multiplier_for_product(
    tx: &mut Transaction<'_, Postgres>,
    product_id: Uuid,
    units_taken: f64,
    unit: &str,
    requested_multiplier: f64,
) -> ApplicationResult<f64> {
    let product: Option<(f64, String)> = sqlx::query_as(
        "SELECT serving_size_units, serving_unit FROM products WHERE id = $1 FOR SHARE",
    )
    .bind(product_id)
    .fetch_optional(&mut **tx)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    let (serving_size, serving_unit) = product
        .ok_or_else(|| ApplicationError::NotFound(format!("Product not found: {product_id}")))?;
    validated_serving_multiplier(
        units_taken,
        unit,
        requested_multiplier,
        serving_size,
        &serving_unit,
    )
}
