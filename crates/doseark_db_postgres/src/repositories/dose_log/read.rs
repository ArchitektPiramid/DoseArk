//! Paginated dose history queries, including the existing stock projection.
//!
//! The projection derives historical balances from current stock and active doses;
//! it does not reconstruct separate manual adjustments.

use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::{DoseLog, DoseLogListInput, DoseLogPage};
use sqlx::PgPool;

use super::mapping::DoseLogHistoryRow;

pub(super) async fn list_dose_logs(
    pool: &PgPool,
    input: DoseLogListInput,
) -> ApplicationResult<DoseLogPage> {
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
    .fetch_one(pool)
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
    .fetch_all(pool)
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
