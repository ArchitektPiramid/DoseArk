// historia dawek, paginacja i projekcja zapasów
use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::{DoseLog, DoseLogListInput, DoseLogPage};
use sqlx::SqlitePool;

use super::mapping::DoseLogHistoryRow;

/// Lists active doses with a projected stock balance for each history row
pub(super) async fn list_dose_logs(
    pool: &SqlitePool,
    input: DoseLogListInput,
) -> ApplicationResult<DoseLogPage> {
    tracing::trace!(
        limit = input.limit,
        offset = input.offset,
        product_id = ?input.product_id,
        "DB query started: list SQLite dose logs",
    );

    let product_id = input.product_id.map(|id| id.to_string());

    let (total,): (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM dose_logs
        WHERE status = 'active'
          AND (?1 IS NULL OR product_id = ?1)
        "#,
    )
    .bind(product_id.clone())
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
                    ELSE ROUND(
                        current_units_remaining
                        + total_active_units_taken
                        - cumulative_units_taken
                        + units_taken,
                        6
                    )
                END AS inventory_quantity_before,
                CASE
                    WHEN inventory_lot_id IS NULL OR current_units_remaining IS NULL THEN NULL
                    ELSE ROUND(
                        current_units_remaining
                        + total_active_units_taken
                        - cumulative_units_taken,
                        6
                    )
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
        WHERE (?1 IS NULL OR product_id = ?1)
        ORDER BY taken_at DESC, created_at DESC, id DESC
        LIMIT ?2
        OFFSET ?3
        "#,
    )
    .bind(product_id)
    .bind(input.limit)
    .bind(input.offset)
    .fetch_all(pool)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    tracing::debug!(
        row_count = rows.len(),
        total,
        "DB query finished: list SQLite dose logs",
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
