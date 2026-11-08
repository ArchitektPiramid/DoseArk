//! Product catalog HTTP routes for listing, creating and updating products.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch, post},
    Json, Router,
};
use doseark_domain::{CreateProductInput, Product, UpdateProductInput};
use uuid::Uuid;

use crate::state::AppState;

use crate::routes::error::{map_application_error, ApiError};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/products", get(list_products))
        .route("/api/products", post(create_product))
        .route("/api/products/{product_id}", patch(update_product))
}

async fn list_products(
    State(state): State<AppState>,
) -> Result<Json<Vec<Product>>, (StatusCode, Json<ApiError>)> {
    state
        .services
        .products
        .list_products()
        .await
        .map(Json)
        .map_err(map_application_error)
}

async fn create_product(
    State(state): State<AppState>,
    Json(input): Json<CreateProductInput>,
) -> Result<(StatusCode, Json<Product>), (StatusCode, Json<ApiError>)> {
    let product = state
        .services
        .products
        .create_product(input)
        .await
        .map_err(map_application_error)?;

    Ok((StatusCode::CREATED, Json(product)))
}

async fn update_product(
    State(state): State<AppState>,
    Path(product_id): Path<Uuid>,
    Json(input): Json<UpdateProductInput>,
) -> Result<Json<Product>, (StatusCode, Json<ApiError>)> {
    tracing::debug!(
        product_id = %product_id,
        product_name = %input.name,
        "API handler started: update product",
    );

    state
        .services
        .products
        .update_product(product_id, input)
        .await
        .map(Json)
        .map_err(map_application_error)
}
