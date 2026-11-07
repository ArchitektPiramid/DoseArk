//! Agregacja ilości składników w kolejnych dniach

use std::collections::BTreeMap;

use chrono::Days;
use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::{NutrientTrendDay, NutrientTrendReportInput};
use sqlx::SqlitePool;

use super::super::{map_sqlx_error, mapping::NutrientTrendDaySql};

/// load the day totals for either one product or one nutrient
pub(super) async fn load(
    pool: &SqlitePool,
    input: &NutrientTrendReportInput,
) -> ApplicationResult<Vec<NutrientTrendDay>> {
    let days = if let Some(product_id) = input.product_id {
        let unit = input.unit.as_deref().unwrap_or("unit");
        let day_sql = sqlx::query_as::<_, NutrientTrendDaySql>(
            r#"
            SELECT
                date(datetime(
                    dl.taken_at,
                    printf('%+d minutes', -?5),
                    printf('%+d hours', -?6)
                )) AS date,
                SUM(dl.units_taken) AS amount_total,
                ?4 AS unit,
                NULL AS reference_daily_amount,
                NULL AS reference_daily_unit,
                NULL AS reference_source,
                NULL AS percent_of_reference_total,
                COUNT(DISTINCT dl.id) AS dose_count
            FROM dose_logs dl
            WHERE dl.taken_at >= ?1
              AND dl.taken_at < ?2
              AND dl.status = 'active'
              AND dl.product_id = ?3
            GROUP BY date(datetime(
                dl.taken_at,
                printf('%+d minutes', -?5),
                printf('%+d hours', -?6)
            ))
            ORDER BY date ASC
            "#,
        )
        .bind(input.start_at)
        .bind(input.end_at)
        .bind(product_id.to_string())
        .bind(unit)
        .bind(input.timezone_offset_minutes)
        .bind(i32::from(input.day_start_hour))
        .fetch_all(pool)
        .await
        .map_err(map_sqlx_error)?;

        let reported_days = day_sql
            .into_iter()
            .map(NutrientTrendDay::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        fill_missing_days(input, unit, reported_days)?
    } else {
        match (&input.nutrient_id, &input.unit) {
            (Some(nutrient_id), Some(unit)) => {
                let day_sql = sqlx::query_as::<_, NutrientTrendDaySql>(
                    r#"
                    SELECT
                        date(datetime(
                            dl.taken_at,
                            printf('%+d minutes', -?5),
                            printf('%+d hours', -?6)
                        )) AS date,
                        SUM(dln.amount) AS amount_total,
                        dln.unit,
                        MAX(dln.reference_daily_amount) AS reference_daily_amount,
                        MAX(dln.reference_daily_unit) AS reference_daily_unit,
                        MAX(dln.reference_source) AS reference_source,
                        CASE
                            WHEN MAX(dln.reference_daily_amount) IS NOT NULL
                                AND MAX(dln.reference_daily_amount) > 0
                                AND MAX(dln.reference_daily_unit) = dln.unit
                            THEN (SUM(dln.amount) / MAX(dln.reference_daily_amount)) * 100.0
                            ELSE SUM(dln.percent_of_reference)
                        END AS percent_of_reference_total,
                        COUNT(DISTINCT dln.dose_log_id) AS dose_count
                    FROM dose_log_nutrients dln
                    INNER JOIN dose_logs dl ON dl.id = dln.dose_log_id
                    WHERE dl.taken_at >= ?1
                      AND dl.taken_at < ?2
                      AND dl.status = 'active'
                      AND dln.nutrient_id = ?3
                      AND dln.unit = ?4
                    GROUP BY
                        date(datetime(
                            dl.taken_at,
                            printf('%+d minutes', -?5),
                            printf('%+d hours', -?6)
                        )),
                        dln.unit
                    ORDER BY date ASC
                    "#,
                )
                .bind(input.start_at)
                .bind(input.end_at)
                .bind(nutrient_id.to_string())
                .bind(unit)
                .bind(input.timezone_offset_minutes)
                .bind(i32::from(input.day_start_hour))
                .fetch_all(pool)
                .await
                .map_err(map_sqlx_error)?;

                let reported_days = day_sql
                    .into_iter()
                    .map(NutrientTrendDay::try_from)
                    .collect::<Result<Vec<_>, _>>()?;

                fill_missing_days(input, unit, reported_days)?
            }
            _ => Vec::new(),
        }
    };
    Ok(days)
}

/// Fill gaps in the selected range so charts can show days with no doses
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
