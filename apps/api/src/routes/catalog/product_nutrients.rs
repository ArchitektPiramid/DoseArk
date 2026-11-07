//! Product composition HTTP routes.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use doseark_domain::{ProductNutrient, SaveProductNutrientInput};
use uuid::Uuid;

use crate::state::AppState;

use crate::routes::error::{map_application_error, ApiError};

pub fn router() -> Router<AppState> {
    Router::new().route(
        "/api/products/{product_id}/nutrients",
        get(list_product_nutrients).put(replace_product_nutrients),
    )
}

async fn list_product_nutrients(
    State(state): State<AppState>,
    Path(product_id): Path<Uuid>,
) -> Result<Json<Vec<ProductNutrient>>, (StatusCode, Json<ApiError>)> {
    tracing::debug!(
        product_id = %product_id,
        "API handler started: list product nutrients",
    );

    state
        .services
        .product_composition
        .list_product_nutrients(product_id)
        .await
        .map(Json)
        .map_err(map_application_error)
}

async fn replace_product_nutrients(
    State(state): State<AppState>,
    Path(product_id): Path<Uuid>,
    Json(input): Json<Vec<SaveProductNutrientInput>>,
) -> Result<Json<Vec<ProductNutrient>>, (StatusCode, Json<ApiError>)> {
    tracing::debug!(
        product_id = %product_id,
        item_count = input.len(),
        "API handler started: replace product nutrients",
    );

    state
        .services
        .product_composition
        .replace_product_nutrients(product_id, input)
        .await
        .map(Json)
        .map_err(map_application_error)
}
