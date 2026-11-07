//! Brand catalog HTTP routes and application error handling.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch, post},
    Json, Router,
};
use doseark_domain::{Brand, CreateBrandInput, UpdateBrandInput};
use uuid::Uuid;

use crate::state::AppState;

use super::error::{map_application_error, ApiError};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/brands", get(list_brands))
        .route("/api/brands", post(create_brand))
        .route("/api/brands/{brand_id}", patch(update_brand))
}

async fn list_brands(
    State(state): State<AppState>,
) -> Result<Json<Vec<Brand>>, (StatusCode, Json<ApiError>)> {
    tracing::debug!("API handler started: list brands");

    state
        .services
        .brands
        .list_brands()
        .await
        .map(Json)
        .map_err(map_application_error)
}

async fn create_brand(
    State(state): State<AppState>,
    Json(input): Json<CreateBrandInput>,
) -> Result<(StatusCode, Json<Brand>), (StatusCode, Json<ApiError>)> {
    tracing::debug!(brand_name = %input.name, "API handler started: create brand");

    let brand = state
        .services
        .brands
        .create_brand(input)
        .await
        .map_err(map_application_error)?;

    Ok((StatusCode::CREATED, Json(brand)))
}

async fn update_brand(
    State(state): State<AppState>,
    Path(brand_id): Path<Uuid>,
    Json(input): Json<UpdateBrandInput>,
) -> Result<Json<Brand>, (StatusCode, Json<ApiError>)> {
    tracing::debug!(brand_id = %brand_id, brand_name = %input.name, "API handler started: update brand");

    state
        .services
        .brands
        .update_brand(brand_id, input)
        .await
        .map(Json)
        .map_err(map_application_error)
}
