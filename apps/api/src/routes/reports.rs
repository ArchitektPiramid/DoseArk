//! Daily intake and nutrient trend HTTP routes.

use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use chrono::{DateTime, Duration, NaiveDate, Utc};
use doseark_domain::{
    DailyIntakeReport, DailyIntakeReportInput, NutrientTrendReport, NutrientTrendReportInput,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::state::AppState;

use super::error::{map_application_error, ApiError};

#[derive(Debug, Deserialize)]
struct DailyIntakeReportQuery {
    date: String,
    timezone_offset_minutes: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct NutrientTrendReportQuery {
    start_date: String,
    end_date: String,
    timezone_offset_minutes: Option<i32>,
    day_start_hour: Option<u8>,
    nutrient_id: Option<Uuid>,
    product_id: Option<Uuid>,
    unit: Option<String>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/reports/daily-intake", get(get_daily_intake_report))
        .route(
            "/api/reports/nutrient-trend",
            get(get_nutrient_trend_report),
        )
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
        .services
        .reports
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
        .services
        .reports
        .get_nutrient_trend_report(input)
        .await
        .map(Json)
        .map_err(map_application_error)
}

fn build_daily_intake_input(
    query: DailyIntakeReportQuery,
) -> Result<DailyIntakeReportInput, (StatusCode, Json<ApiError>)> {
    let date = parse_date(&query.date, "date")?;
    let timezone_offset_minutes = query.timezone_offset_minutes.unwrap_or(0);
    let (start_at, end_at) = build_utc_range(date, date, timezone_offset_minutes, 0)?;

    Ok(DailyIntakeReportInput {
        date,
        start_at,
        end_at,
    })
}

fn build_nutrient_trend_input(
    query: NutrientTrendReportQuery,
) -> Result<NutrientTrendReportInput, (StatusCode, Json<ApiError>)> {
    let start_date = parse_date(&query.start_date, "start_date")?;
    let end_date = parse_date(&query.end_date, "end_date")?;

    if end_date < start_date {
        return Err(bad_request(
            "Invalid report range. end_date must not be before start_date.".to_string(),
        ));
    }

    let day_count = end_date.signed_duration_since(start_date).num_days() + 1;
    if day_count > 732 {
        return Err(bad_request(
            "Invalid report range. Maximum range is two years.".to_string(),
        ));
    }

    let timezone_offset_minutes = query.timezone_offset_minutes.unwrap_or(0);
    let day_start_hour = query.day_start_hour.unwrap_or(0);

    if day_start_hour > 23 {
        return Err(bad_request(
            "Invalid day_start_hour. Expected a value from 0 to 23.".to_string(),
        ));
    }

    let (start_at, end_at) = build_utc_range(
        start_date,
        end_date,
        timezone_offset_minutes,
        day_start_hour,
    )?;
    if query.nutrient_id.is_some() && query.product_id.is_some() {
        return Err(bad_request(
            "Select either nutrient_id or product_id, not both.".to_string(),
        ));
    }

    let unit = query
        .unit
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());

    if query.product_id.is_some() && unit.is_none() {
        return Err(bad_request(
            "Product reports require a dose unit.".to_string(),
        ));
    }

    Ok(NutrientTrendReportInput {
        start_date,
        end_date,
        start_at,
        end_at,
        timezone_offset_minutes,
        day_start_hour,
        nutrient_id: query.nutrient_id,
        product_id: query.product_id,
        unit,
    })
}

fn parse_date(value: &str, field_name: &str) -> Result<NaiveDate, (StatusCode, Json<ApiError>)> {
    NaiveDate::parse_from_str(value, "%Y-%m-%d").map_err(|err| {
        bad_request(format!(
            "Invalid {field_name} format. Expected YYYY-MM-DD. Got '{value}'. Error: {err}",
        ))
    })
}

fn build_utc_range(
    start_date: NaiveDate,
    end_date: NaiveDate,
    timezone_offset_minutes: i32,
    day_start_hour: u8,
) -> Result<(DateTime<Utc>, DateTime<Utc>), (StatusCode, Json<ApiError>)> {
    let local_start = start_date
        .and_hms_opt(u32::from(day_start_hour), 0, 0)
        .ok_or_else(|| bad_request("Failed to build report start time".to_string()))?;

    let next_date = end_date
        .succ_opt()
        .ok_or_else(|| bad_request("Failed to build report end date".to_string()))?;

    let local_end = next_date
        .and_hms_opt(u32::from(day_start_hour), 0, 0)
        .ok_or_else(|| bad_request("Failed to build report end time".to_string()))?;

    let start_utc_naive = local_start + Duration::minutes(i64::from(timezone_offset_minutes));
    let end_utc_naive = local_end + Duration::minutes(i64::from(timezone_offset_minutes));

    Ok((
        DateTime::<Utc>::from_naive_utc_and_offset(start_utc_naive, Utc),
        DateTime::<Utc>::from_naive_utc_and_offset(end_utc_naive, Utc),
    ))
}

fn bad_request(message: String) -> (StatusCode, Json<ApiError>) {
    (
        StatusCode::BAD_REQUEST,
        Json(ApiError {
            code: "BAD_REQUEST",
            message,
        }),
    )
}
