//! Builds nutrient and product trends from independent, aligned PostgreSQL queries.
//!
//! The report uses half-open time boundaries and a caller-supplied offset for
//! bucketing; real timezone transitions require richer inputs than a fixed offset.

mod days;
mod events;
mod options;

use doseark_application::ApplicationResult;
use doseark_domain::{NutrientTrendReport, NutrientTrendReportInput};
use sqlx::PgPool;

pub(super) async fn get_nutrient_trend_report(pool: &PgPool, input: NutrientTrendReportInput) -> ApplicationResult<NutrientTrendReport> {
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

    let nutrients = options::load_options(pool, &input).await?;
    let days = days::load_days(pool, &input).await?;
    let events = events::load_events(pool, &input).await?;

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
