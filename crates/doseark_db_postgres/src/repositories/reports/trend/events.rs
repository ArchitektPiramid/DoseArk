//! Individual recorded intakes corresponding to nutrient or product trend days.

use chrono::Duration;
use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::{NutrientTrendEvent, NutrientTrendReportInput};
use sqlx::PgPool;

use super::super::mapping::NutrientTrendEventSql;

pub(super) async fn load_events(pool: &PgPool, input: &NutrientTrendReportInput) -> ApplicationResult<Vec<NutrientTrendEvent>> {
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
        .fetch_all(pool)
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
                .fetch_all(pool)
                .await
                .map_err(|err| ApplicationError::Repository(err.to_string()))?;

                event_sql.into_iter().map(Into::into).collect()
            }
            _ => Vec::new(),
        }
    };

    Ok(events)
}
