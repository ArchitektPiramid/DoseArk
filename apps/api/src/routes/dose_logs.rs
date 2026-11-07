//! Dose history HTTP routes for listing, creating, editing and cancellation.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use doseark_domain::{
    CreateDoseLogInput, DoseLog, DoseLogListInput, DoseLogPage, UpdateDoseLogInput,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::state::AppState;

use super::error::{map_application_error, ApiError};

#[derive(Debug, Deserialize)]
struct DoseLogListQuery {
    limit: Option<i64>,
    offset: Option<i64>,
    product_id: Option<Uuid>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/dose-logs", get(list_dose_logs))
        .route("/api/dose-logs", post(create_dose_log))
        .route(
            "/api/dose-logs/{dose_log_id}",
            axum::routing::patch(update_dose_log),
        )
        .route(
            "/api/dose-logs/{dose_log_id}",
            axum::routing::delete(cancel_dose_log),
        )
}

async fn list_dose_logs(
    Query(query): Query<DoseLogListQuery>,
    State(state): State<AppState>,
) -> Result<Json<DoseLogPage>, (StatusCode, Json<ApiError>)> {
    let input = DoseLogListInput {
        limit: query.limit.unwrap_or(25),
        offset: query.offset.unwrap_or(0),
        product_id: query.product_id,
    };

    tracing::debug!(
        limit = input.limit,
        offset = input.offset,
        product_id = ?input.product_id,
        "API handler started: list dose logs",
    );

    state
        .services
        .dose_logs
        .list_dose_logs(input)
        .await
        .map(Json)
        .map_err(map_application_error)
}

async fn create_dose_log(
    State(state): State<AppState>,
    Json(input): Json<CreateDoseLogInput>,
) -> Result<(StatusCode, Json<DoseLog>), (StatusCode, Json<ApiError>)> {
    tracing::debug!(
        product_id = %input.product_id,
        inventory_lot_id = ?input.inventory_lot_id,
        units_taken = input.units_taken,
        "API handler started: create dose log",
    );

    let dose_log = state
        .services
        .dose_logs
        .create_dose_log(input)
        .await
        .map_err(map_application_error)?;

    Ok((StatusCode::CREATED, Json(dose_log)))
}

async fn update_dose_log(
    State(state): State<AppState>,
    Path(dose_log_id): Path<Uuid>,
    Json(input): Json<UpdateDoseLogInput>,
) -> Result<Json<DoseLog>, (StatusCode, Json<ApiError>)> {
    tracing::debug!(
        dose_log_id = %dose_log_id,
        product_id = %input.product_id,
        inventory_lot_id = ?input.inventory_lot_id,
        units_taken = input.units_taken,
        "API handler started: update dose log",
    );

    state
        .services
        .dose_logs
        .update_dose_log(dose_log_id, input)
        .await
        .map(Json)
        .map_err(map_application_error)
}

async fn cancel_dose_log(
    State(state): State<AppState>,
    Path(dose_log_id): Path<Uuid>,
) -> Result<Json<DoseLog>, (StatusCode, Json<ApiError>)> {
    tracing::debug!(
        dose_log_id = %dose_log_id,
        "API handler started: cancel dose log",
    );

    state
        .services
        .dose_logs
        .cancel_dose_log(dose_log_id)
        .await
        .map(Json)
        .map_err(map_application_error)
}
