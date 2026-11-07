//! Inventory lot HTTP routes and stock-level requests.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch, post},
    Json, Router,
};
use doseark_domain::{CreateInventoryLotInput, InventoryLot, UpdateInventoryLotInput};
use uuid::Uuid;

use crate::state::AppState;

use super::error::{map_application_error, ApiError};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/inventory-lots", get(list_inventory_lots))
        .route("/api/inventory-lots", post(create_inventory_lot))
        .route("/api/inventory-lots/{lot_id}", patch(update_inventory_lot))
}

async fn list_inventory_lots(
    State(state): State<AppState>,
) -> Result<Json<Vec<InventoryLot>>, (StatusCode, Json<ApiError>)> {
    tracing::debug!("API handler started: list inventory lots");

    state
        .services
        .inventory_lots
        .list_inventory_lots()
        .await
        .map(Json)
        .map_err(map_application_error)
}

async fn create_inventory_lot(
    State(state): State<AppState>,
    Json(input): Json<CreateInventoryLotInput>,
) -> Result<(StatusCode, Json<InventoryLot>), (StatusCode, Json<ApiError>)> {
    tracing::debug!(
        product_id = %input.product_id,
        "API handler started: create inventory lot",
    );

    let lot = state
        .services
        .inventory_lots
        .create_inventory_lot(input)
        .await
        .map_err(map_application_error)?;

    Ok((StatusCode::CREATED, Json(lot)))
}

async fn update_inventory_lot(
    State(state): State<AppState>,
    Path(lot_id): Path<Uuid>,
    Json(input): Json<UpdateInventoryLotInput>,
) -> Result<Json<InventoryLot>, (StatusCode, Json<ApiError>)> {
    tracing::debug!(
        lot_id = %lot_id,
        product_id = %input.product_id,
        "API handler started: update inventory lot",
    );

    state
        .services
        .inventory_lots
        .update_inventory_lot(lot_id, input)
        .await
        .map(Json)
        .map_err(map_application_error)
}
