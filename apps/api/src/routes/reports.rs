//! Daily intake and nutrient trend HTTP endpoints.

mod query;

use axum::{extract::{Query, State}, http::StatusCode, routing::get, Json, Router};
use doseark_domain::{DailyIntakeReport, NutrientTrendReport};
use crate::state::AppState;
use super::error::{map_application_error, ApiError};
use self::query::{
    build_daily_intake_input, build_nutrient_trend_input,
    DailyIntakeReportQuery, NutrientTrendReportQuery,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/reports/daily-intake", get(get_daily_intake_report))
        .route("/api/reports/nutrient-trend", get(get_nutrient_trend_report))
}

async fn get_daily_intake_report(
    State(state): State<AppState>,
    Query(query): Query<DailyIntakeReportQuery>,
) -> Result<Json<DailyIntakeReport>, (StatusCode, Json<ApiError>)> {
    let input = build_daily_intake_input(query)?;

    tracing::debug!(
        date = %input.date,
        start_at = %input.start_at,
        end_at = %input.end_at,
        "API handler started: daily intake report",
    );

    state
        .services.reports
        .get_daily_intake_report(input)
        .await
        .map(Json)
        .map_err(map_application_error)
}

async fn get_nutrient_trend_report(
    State(state): State<AppState>,
    Query(query): Query<NutrientTrendReportQuery>,
) -> Result<Json<NutrientTrendReport>, (StatusCode, Json<ApiError>)> {
    let input = build_nutrient_trend_input(query)?;

    tracing::debug!(
        start_date = %input.start_date,
        end_date = %input.end_date,
        start_at = %input.start_at,
        end_at = %input.end_at,
        nutrient_id = ?input.nutrient_id,
        product_id = ?input.product_id,
        unit = ?input.unit,
        day_start_hour = input.day_start_hour,
        "API handler started: trend report",
    );

    state
        .services.reports
        .get_nutrient_trend_report(input)
        .await
        .map(Json)
        .map_err(map_application_error)
}
