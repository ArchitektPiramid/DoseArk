use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use doseark_domain::DoseLogNutrient;
use serde::Serialize;
use uuid::Uuid;

use crate::state::AppState;

#[derive(Debug, Serialize)]
struct ApiError {
    code: &'static str,
    message: String,
}

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
        .services.dose_log_nutrients
        .list_dose_log_nutrients(dose_log_id)
        .await
        .map(Json)
        .map_err(map_application_error)
}

fn map_application_error(error: doseark_application::ApplicationError) -> (StatusCode, Json<ApiError>) {
    let status = match error {
        doseark_application::ApplicationError::Validation(_) => StatusCode::BAD_REQUEST,
        doseark_application::ApplicationError::NotFound(_) => StatusCode::NOT_FOUND,
        doseark_application::ApplicationError::Repository(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };

    (
        status,
        Json(ApiError {
            code: "API_ERROR",
            message: error.to_string(),
        }),
    )
}
