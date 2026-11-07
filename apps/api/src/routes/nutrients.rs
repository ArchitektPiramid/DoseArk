//! Nutrient catalog HTTP routes for CRUD operations.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch, post},
    Json, Router,
};
use doseark_domain::{CreateNutrientInput, Nutrient, UpdateNutrientInput};
use uuid::Uuid;

use crate::state::AppState;

use super::error::{map_application_error, ApiError};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/nutrients", get(list_nutrients))
        .route("/api/nutrients", post(create_nutrient))
        .route("/api/nutrients/{nutrient_id}", patch(update_nutrient))
}

async fn list_nutrients(
    State(state): State<AppState>,
) -> Result<Json<Vec<Nutrient>>, (StatusCode, Json<ApiError>)> {
    tracing::debug!("API handler started: list nutrients");

    state
        .services
        .nutrients
        .list_nutrients()
        .await
        .map(Json)
        .map_err(map_application_error)
}

async fn create_nutrient(
    State(state): State<AppState>,
    Json(input): Json<CreateNutrientInput>,
) -> Result<(StatusCode, Json<Nutrient>), (StatusCode, Json<ApiError>)> {
    tracing::debug!(
        canonical_name = %input.canonical_name,
        display_name = %input.display_name,
        "API handler started: create nutrient",
    );

    let nutrient = state
        .services
        .nutrients
        .create_nutrient(input)
        .await
        .map_err(map_application_error)?;

    Ok((StatusCode::CREATED, Json(nutrient)))
}

async fn update_nutrient(
    State(state): State<AppState>,
    Path(nutrient_id): Path<Uuid>,
    Json(input): Json<UpdateNutrientInput>,
) -> Result<Json<Nutrient>, (StatusCode, Json<ApiError>)> {
    tracing::debug!(
        nutrient_id = %nutrient_id,
        canonical_name = %input.canonical_name,
        display_name = %input.display_name,
        "API handler started: update nutrient",
    );

    state
        .services
        .nutrients
        .update_nutrient(nutrient_id, input)
        .await
        .map(Json)
        .map_err(map_application_error)
}
