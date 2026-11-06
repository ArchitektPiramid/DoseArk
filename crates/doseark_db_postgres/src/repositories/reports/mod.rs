mod mapping;

use std::collections::BTreeMap;

use async_trait::async_trait;
use chrono::{Days, Duration};
use doseark_application::{ApplicationError, ApplicationResult, DailyIntakeReportRepository};
use doseark_domain::{
    DailyIntakeReport, DailyIntakeReportInput, DailyIntakeReportRow, DoseLog, NutrientTrendDay,
    NutrientTrendReport, NutrientTrendReportInput,
};
use sqlx::PgPool;

use crate::repositories::reports::mapping::{
    DailyIntakeReportRowSql, NutrientTrendDaySql, NutrientTrendEventSql,
    NutrientTrendOptionSql, ReportDoseLogRow,
};

#[derive(Clone)]
pub struct PgDailyIntakeReportRepository {
    pool: PgPool,
}

impl PgDailyIntakeReportRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DailyIntakeReportRepository for PgDailyIntakeReportRepository {
    async fn get_daily_intake_report(
        &self,
        input: DailyIntakeReportInput,
    ) -> ApplicationResult<DailyIntakeReport> {
        tracing::trace!(
            date = %input.date,
            start_at = %input.start_at,
            end_at = %input.end_at,
            "DB query started: daily intake report",
        );

        let row_sql = sqlx::query_as::<_, DailyIntakeReportRowSql>(
            r#"
            SELECT
                dln.nutrient_id,

                COALESCE(dln.nutrient_canonical_name, n.canonical_name) AS nutrient_canonical_name,
                COALESCE(dln.nutrient_display_name, n.display_name) AS nutrient_display_name,
                COALESCE(dln.nutrient_kind, n.kind) AS nutrient_kind,

                SUM(dln.amount)::DOUBLE PRECISION AS amount_total,
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
                END::DOUBLE PRECISION AS percent_of_reference_total,

                COUNT(DISTINCT dln.dose_log_id)::BIGINT AS dose_count
            FROM dose_log_nutrients dln
            INNER JOIN dose_logs dl ON dl.id = dln.dose_log_id
            LEFT JOIN nutrients n ON n.id = dln.nutrient_id
            WHERE dl.taken_at >= $1
              AND dl.taken_at < $2
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
        .fetch_all(&self.pool)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

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
            WHERE taken_at >= $1
            AND taken_at < $2
            AND status = 'active'
            ORDER BY taken_at DESC
            "#,
        )
        .bind(input.start_at)
        .bind(input.end_at)
        .fetch_all(&self.pool)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

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
            "DB query finished: daily intake report",
        );

        Ok(DailyIntakeReport {
            date: input.date,
            start_at: input.start_at,
            end_at: input.end_at,
            rows,
            dose_logs,
        })
    }

    async fn get_nutrient_trend_report(
        &self,
        input: NutrientTrendReportInput,
    ) -> ApplicationResult<NutrientTrendReport> {
        tracing::trace!(
            start_date = %input.start_date,
            end_date = %input.end_date,
            start_at = %input.start_at,
            end_at = %input.end_at,
            nutrient_id = ?input.nutrient_id,
            product_id = ?input.product_id,
            unit = ?input.unit,
            day_start_hour = input.day_start_hour,
            "DB query started: trend report",
        );

        let nutrients = if input.product_id.is_none() {
            let option_sql = sqlx::query_as::<_, NutrientTrendOptionSql>(
                r#"
                SELECT
                    dln.nutrient_id,
                    COALESCE(dln.nutrient_canonical_name, n.canonical_name) AS nutrient_canonical_name,
                    COALESCE(dln.nutrient_display_name, n.display_name) AS nutrient_display_name,
                    COALESCE(dln.nutrient_kind, n.kind) AS nutrient_kind,
                    dln.unit,
                    SUM(dln.amount)::DOUBLE PRECISION AS amount_total,
                    COUNT(DISTINCT dln.dose_log_id)::BIGINT AS dose_count,
                    COUNT(DISTINCT (((dl.taken_at AT TIME ZONE 'UTC') - ($3::INT * INTERVAL '1 minute') - ($4::INT * INTERVAL '1 hour'))::DATE))::BIGINT AS recorded_day_count
                FROM dose_log_nutrients dln
                INNER JOIN dose_logs dl ON dl.id = dln.dose_log_id
                LEFT JOIN nutrients n ON n.id = dln.nutrient_id
                WHERE dl.taken_at >= $1
                  AND dl.taken_at < $2
                  AND dl.status = 'active'
                GROUP BY
                    dln.nutrient_id,
                    COALESCE(dln.nutrient_canonical_name, n.canonical_name),
                    COALESCE(dln.nutrient_display_name, n.display_name),
                    COALESCE(dln.nutrient_kind, n.kind),
                    dln.unit
                ORDER BY
                    COALESCE(dln.nutrient_display_name, n.display_name) ASC,
                    dln.unit ASC
                "#,
            )
            .bind(input.start_at)
            .bind(input.end_at)
            .bind(input.timezone_offset_minutes)
            .bind(i32::from(input.day_start_hour))
            .fetch_all(&self.pool)
            .await
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;

            option_sql
                .into_iter()
                .map(doseark_domain::NutrientTrendOption::try_from)
                .collect::<Result<Vec<_>, _>>()?
        } else {
            Vec::new()
        };

        let days = if let Some(product_id) = input.product_id {
            let unit = input.unit.as_deref().unwrap_or("unit");
            let day_sql = sqlx::query_as::<_, NutrientTrendDaySql>(
                r#"
                SELECT
                    (((dl.taken_at AT TIME ZONE 'UTC') - ($5::INT * INTERVAL '1 minute') - ($6::INT * INTERVAL '1 hour'))::DATE) AS date,
                    SUM(dl.units_taken)::DOUBLE PRECISION AS amount_total,
                    $4::TEXT AS unit,
                    NULL::DOUBLE PRECISION AS reference_daily_amount,
                    NULL::TEXT AS reference_daily_unit,
                    NULL::TEXT AS reference_source,
                    NULL::DOUBLE PRECISION AS percent_of_reference_total,
                    COUNT(DISTINCT dl.id)::BIGINT AS dose_count
                FROM dose_logs dl
                WHERE dl.taken_at >= $1
                  AND dl.taken_at < $2
                  AND dl.status = 'active'
                  AND dl.product_id = $3
                GROUP BY (((dl.taken_at AT TIME ZONE 'UTC') - ($5::INT * INTERVAL '1 minute') - ($6::INT * INTERVAL '1 hour'))::DATE)
                ORDER BY date ASC
                "#,
            )
            .bind(input.start_at)
            .bind(input.end_at)
            .bind(product_id)
            .bind(unit)
            .bind(input.timezone_offset_minutes)
            .bind(i32::from(input.day_start_hour))
            .fetch_all(&self.pool)
            .await
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;

            let reported_days = day_sql
                .into_iter()
                .map(NutrientTrendDay::try_from)
                .collect::<Result<Vec<_>, _>>()?;

            fill_missing_days(&input, unit, reported_days)?
        } else {
            match (&input.nutrient_id, &input.unit) {
                (Some(nutrient_id), Some(unit)) => {
                    let day_sql = sqlx::query_as::<_, NutrientTrendDaySql>(
                        r#"
                        SELECT
                            (((dl.taken_at AT TIME ZONE 'UTC') - ($5::INT * INTERVAL '1 minute') - ($6::INT * INTERVAL '1 hour'))::DATE) AS date,
                            SUM(dln.amount)::DOUBLE PRECISION AS amount_total,
                            dln.unit,
                            MAX(dln.reference_daily_amount)::DOUBLE PRECISION AS reference_daily_amount,
                            MAX(dln.reference_daily_unit) AS reference_daily_unit,
                            MAX(dln.reference_source) AS reference_source,
                            CASE
                                WHEN MAX(dln.reference_daily_amount) IS NOT NULL
                                    AND MAX(dln.reference_daily_amount) > 0
                                    AND MAX(dln.reference_daily_unit) = dln.unit
                                THEN (SUM(dln.amount) / MAX(dln.reference_daily_amount)) * 100.0
                                ELSE SUM(dln.percent_of_reference)
                            END::DOUBLE PRECISION AS percent_of_reference_total,
                            COUNT(DISTINCT dln.dose_log_id)::BIGINT AS dose_count
                        FROM dose_log_nutrients dln
                        INNER JOIN dose_logs dl ON dl.id = dln.dose_log_id
                        WHERE dl.taken_at >= $1
                          AND dl.taken_at < $2
                          AND dl.status = 'active'
                          AND dln.nutrient_id = $3
                          AND dln.unit = $4
                        GROUP BY
                            (((dl.taken_at AT TIME ZONE 'UTC') - ($5::INT * INTERVAL '1 minute') - ($6::INT * INTERVAL '1 hour'))::DATE),
                            dln.unit
                        ORDER BY date ASC
                        "#,
                    )
                    .bind(input.start_at)
                    .bind(input.end_at)
                    .bind(nutrient_id)
                    .bind(unit)
                    .bind(input.timezone_offset_minutes)
                    .bind(i32::from(input.day_start_hour))
                    .fetch_all(&self.pool)
                    .await
                    .map_err(|err| ApplicationError::Repository(err.to_string()))?;

                    let reported_days = day_sql
                        .into_iter()
                        .map(NutrientTrendDay::try_from)
                        .collect::<Result<Vec<_>, _>>()?;

                    fill_missing_days(&input, unit, reported_days)?
                }
                _ => Vec::new(),
            }
        };

        let event_range_shift = Duration::hours(i64::from(input.day_start_hour));
        let event_start_at = input.start_at - event_range_shift;
        let event_end_at = input.end_at - event_range_shift;

        let events = if let Some(product_id) = input.product_id {
            let event_sql = sqlx::query_as::<_, NutrientTrendEventSql>(
                r#"
                SELECT
                    dl.id AS dose_log_id,
                    dl.product_id,
                    p.name AS product_name,
                    b.name AS product_brand_name,
                    dl.taken_at,
                    dl.units_taken::DOUBLE PRECISION AS amount,
                    dl.unit,
                    dl.units_taken AS dose_units_taken,
                    dl.unit AS dose_unit,
                    il.lot_number AS inventory_lot_number,
                    dl.notes
                FROM dose_logs dl
                INNER JOIN products p ON p.id = dl.product_id
                LEFT JOIN brands b ON b.id = p.brand_id
                LEFT JOIN inventory_lots il ON il.id = dl.inventory_lot_id
                WHERE dl.taken_at >= $1
                  AND dl.taken_at < $2
                  AND dl.status = 'active'
                  AND dl.product_id = $3
                ORDER BY dl.taken_at ASC, dl.id ASC
                "#,
            )
            .bind(event_start_at)
            .bind(event_end_at)
            .bind(product_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;

            event_sql.into_iter().map(Into::into).collect()
        } else {
            match (&input.nutrient_id, &input.unit) {
                (Some(nutrient_id), Some(unit)) => {
                    let event_sql = sqlx::query_as::<_, NutrientTrendEventSql>(
                        r#"
                        SELECT
                            dl.id AS dose_log_id,
                            dl.product_id,
                            p.name AS product_name,
                            b.name AS product_brand_name,
                            dl.taken_at,
                            SUM(dln.amount)::DOUBLE PRECISION AS amount,
                            dln.unit,
                            dl.units_taken AS dose_units_taken,
                            dl.unit AS dose_unit,
                            il.lot_number AS inventory_lot_number,
                            dl.notes
                        FROM dose_log_nutrients dln
                        INNER JOIN dose_logs dl ON dl.id = dln.dose_log_id
                        INNER JOIN products p ON p.id = dl.product_id
                        LEFT JOIN brands b ON b.id = p.brand_id
                        LEFT JOIN inventory_lots il ON il.id = dl.inventory_lot_id
                        WHERE dl.taken_at >= $1
                          AND dl.taken_at < $2
                          AND dl.status = 'active'
                          AND dln.nutrient_id = $3
                          AND dln.unit = $4
                        GROUP BY
                            dl.id,
                            dl.product_id,
                            p.name,
                            b.name,
                            dl.taken_at,
                            dln.unit,
                            dl.units_taken,
                            dl.unit,
                            il.lot_number,
                            dl.notes
                        ORDER BY dl.taken_at ASC, dl.id ASC
                        "#,
                    )
                    .bind(event_start_at)
                    .bind(event_end_at)
                    .bind(nutrient_id)
                    .bind(unit)
                    .fetch_all(&self.pool)
                    .await
                    .map_err(|err| ApplicationError::Repository(err.to_string()))?;

                    event_sql.into_iter().map(Into::into).collect()
                }
                _ => Vec::new(),
            }
        };

        tracing::debug!(
            start_date = %input.start_date,
            end_date = %input.end_date,
            nutrient_count = nutrients.len(),
            day_count = days.len(),
            event_count = events.len(),
            product_id = ?input.product_id,
            "DB query finished: trend report",
        );

        Ok(NutrientTrendReport {
            start_date: input.start_date,
            end_date: input.end_date,
            start_at: input.start_at,
            end_at: input.end_at,
            selected_nutrient_id: input.nutrient_id,
            selected_product_id: input.product_id,
            selected_unit: input.unit,
            day_start_hour: input.day_start_hour,
            nutrients,
            days,
            events,
        })
    }

}

fn fill_missing_days(
    input: &NutrientTrendReportInput,
    unit: &str,
    days: Vec<NutrientTrendDay>,
) -> ApplicationResult<Vec<NutrientTrendDay>> {
    let mut by_date = BTreeMap::new();

    for day in days {
        by_date.insert(day.date, day);
    }

    let mut filled_days = Vec::new();
    let mut current = input.start_date;

    loop {
        if let Some(day) = by_date.remove(&current) {
            filled_days.push(day);
        } else {
            filled_days.push(NutrientTrendDay {
                date: current,
                amount_total: 0.0,
                unit: unit.to_string(),
                reference_daily_amount: None,
                reference_daily_unit: None,
                reference_source: None,
                percent_of_reference_total: None,
                dose_count: 0,
            });
        }

        if current == input.end_date {
            break;
        }

        current = current
            .checked_add_days(Days::new(1))
            .ok_or_else(|| ApplicationError::Repository("Failed to iterate report dates".into()))?;
    }

    Ok(filled_days)
}
