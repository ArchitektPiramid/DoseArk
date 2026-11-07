//! Available nutrient groups and their totals for the selected report period.

use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::{NutrientTrendOption, NutrientTrendReportInput};
use sqlx::PgPool;

use super::super::mapping::NutrientTrendOptionSql;

pub(super) async fn load_options(pool: &PgPool, input: &NutrientTrendReportInput) -> ApplicationResult<Vec<NutrientTrendOption>> {
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
        .fetch_all(pool)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        option_sql
            .into_iter()
            .map(doseark_domain::NutrientTrendOption::try_from)
            .collect::<Result<Vec<_>, _>>()?
    } else {
        Vec::new()
    };

    Ok(nutrients)
}
