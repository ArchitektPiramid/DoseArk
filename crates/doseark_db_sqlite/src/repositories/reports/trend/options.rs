use sqlx::SqlitePool;
use doseark_application::ApplicationResult;
use doseark_domain::{NutrientTrendOption, NutrientTrendReportInput};

use super::super::{map_sqlx_error, mapping::NutrientTrendOptionSql};

/// Fetch nutrient choices available within the selected date range
pub(super) async fn load(
    pool: &SqlitePool,
    input: &NutrientTrendReportInput,
) -> ApplicationResult<Vec<NutrientTrendOption>> {
    let nutrients = if input.product_id.is_none() {
        let option_sql = sqlx::query_as::<_, NutrientTrendOptionSql>(
            r#"
            SELECT
                dln.nutrient_id,
                COALESCE(dln.nutrient_canonical_name, n.canonical_name) AS nutrient_canonical_name,
                COALESCE(dln.nutrient_display_name, n.display_name) AS nutrient_display_name,
                COALESCE(dln.nutrient_kind, n.kind) AS nutrient_kind,
                dln.unit,
                SUM(dln.amount) AS amount_total,
                COUNT(DISTINCT dln.dose_log_id) AS dose_count,
                COUNT(DISTINCT date(datetime(
                    dl.taken_at,
                    printf('%+d minutes', -?3),
                    printf('%+d hours', -?4)
                ))) AS recorded_day_count
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
        .map_err(map_sqlx_error)?;

        option_sql
            .into_iter()
            .map(doseark_domain::NutrientTrendOption::try_from)
            .collect::<Result<Vec<_>, _>>()?
    } else {
        Vec::new()
    };
    Ok(nutrients)
}
