use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use doseark_domain::{ProductNutrient, SaveProductNutrientInput};
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
        .services.product_composition
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
        .services.product_composition
        .replace_product_nutrients(product_id, input)
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
