//! Read-only nutrient snapshot HTTP routes.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use doseark_domain::DoseLogNutrient;
use uuid::Uuid;

use crate::state::AppState;

use super::error::{map_application_error, ApiError};

pub fn router() -> Router<AppState> {
    Router::new().route(
        "/api/dose-logs/{dose_log_id}/nutrients",
        get(list_dose_log_nutrients),
    )
}

async fn list_dose_log_nutrients(
    State(state): State<AppState>,
    Path(dose_log_id): Path<Uuid>,
) -> Result<Json<Vec<DoseLogNutrient>>, (StatusCode, Json<ApiError>)> {
    tracing::debug!(
        dose_log_id = %dose_log_id,
        "API handler started: list dose log nutrients",
    );

    state
        .services
        .dose_log_nutrients
        .list_dose_log_nutrients(dose_log_id)
        .await
        .map(Json)
        .map_err(map_application_error)
}
