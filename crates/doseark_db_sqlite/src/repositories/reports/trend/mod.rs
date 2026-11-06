mod days;
mod events;
mod options;

use sqlx::SqlitePool;
use doseark_application::ApplicationResult;
use doseark_domain::{NutrientTrendReport, NutrientTrendReportInput};

/// Build the trend using options, daily totals and individual events
pub(super) async fn load(
    pool: &SqlitePool,
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
        "SQLite query started: trend report",
    );

    let nutrients = options::load(pool, &input).await?;
    let days = days::load(pool, &input).await?;
    let events = events::load(pool, &input).await?;

    tracing::debug!(
        start_date = %input.start_date,
        end_date = %input.end_date,
        nutrient_count = nutrients.len(),
        day_count = days.len(),
        event_count = events.len(),
        product_id = ?input.product_id,
        "SQLite query finished: trend report",
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
