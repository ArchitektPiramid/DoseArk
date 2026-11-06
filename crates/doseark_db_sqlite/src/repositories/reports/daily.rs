use sqlx::SqlitePool;
use doseark_application::ApplicationResult;
use doseark_domain::{DailyIntakeReport, DailyIntakeReportInput, DailyIntakeReportRow, DoseLog};

use super::{
    map_sqlx_error,
    mapping::{DailyIntakeReportRowSql, ReportDoseLogRow},
};

/// loads daily nutrient totals together with the doses behind them
pub(super) async fn load(
    pool: &SqlitePool,
    input: DailyIntakeReportInput,
) -> ApplicationResult<DailyIntakeReport> {
    tracing::trace!(
        date = %input.date,
        start_at = %input.start_at,
        end_at = %input.end_at,
        "SQLite query started: daily intake report",
    );

    let row_sql = sqlx::query_as::<_, DailyIntakeReportRowSql>(
        r#"
        SELECT
            dln.nutrient_id,
            COALESCE(dln.nutrient_canonical_name, n.canonical_name) AS nutrient_canonical_name,
            COALESCE(dln.nutrient_display_name, n.display_name) AS nutrient_display_name,
            COALESCE(dln.nutrient_kind, n.kind) AS nutrient_kind,
            SUM(dln.amount) AS amount_total,
            dln.unit,
            dln.reference_daily_amount,
            dln.reference_daily_unit,
            dln.reference_source,
            CASE
                WHEN dln.reference_daily_amount IS NOT NULL
                    AND dln.reference_daily_amount > 0
                    AND dln.reference_daily_unit = dln.unit
                THEN (SUM(dln.amount) / dln.reference_daily_amount) * 100.0
                ELSE SUM(dln.percent_of_reference)
            END AS percent_of_reference_total,
            COUNT(DISTINCT dln.dose_log_id) AS dose_count
        FROM dose_log_nutrients dln
        INNER JOIN dose_logs dl ON dl.id = dln.dose_log_id
        LEFT JOIN nutrients n ON n.id = dln.nutrient_id
        WHERE dl.taken_at >= ?1
          AND dl.taken_at < ?2
          AND dl.status = 'active'
        GROUP BY
            dln.nutrient_id,
            COALESCE(dln.nutrient_canonical_name, n.canonical_name),
            COALESCE(dln.nutrient_display_name, n.display_name),
            COALESCE(dln.nutrient_kind, n.kind),
            dln.unit,
            dln.reference_daily_amount,
            dln.reference_daily_unit,
            dln.reference_source
        ORDER BY
            COALESCE(dln.nutrient_display_name, n.display_name) ASC,
            dln.unit ASC
        "#,
    )
    .bind(input.start_at)
    .bind(input.end_at)
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    let dose_log_rows = sqlx::query_as::<_, ReportDoseLogRow>(
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
        WHERE taken_at >= ?1
          AND taken_at < ?2
          AND status = 'active'
        ORDER BY taken_at DESC
        "#,
    )
    .bind(input.start_at)
    .bind(input.end_at)
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    let rows = row_sql
        .into_iter()
        .map(DailyIntakeReportRow::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    let dose_logs = dose_log_rows
        .into_iter()
        .map(DoseLog::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    tracing::debug!(
        date = %input.date,
        row_count = rows.len(),
        dose_log_count = dose_logs.len(),
        "SQLite query finished: daily intake report",
    );

    Ok(DailyIntakeReport {
        date: input.date,
        start_at: input.start_at,
        end_at: input.end_at,
        rows,
        dose_logs,
    })
}
