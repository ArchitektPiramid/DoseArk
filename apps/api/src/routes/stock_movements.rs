//! Stock movement history HTTP routes.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use doseark_domain::StockMovement;
use uuid::Uuid;

use crate::state::AppState;

use super::error::{map_application_error, ApiError};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/stock-movements", get(list_stock_movements))
        .route(
            "/api/inventory-lots/{lot_id}/stock-movements",
            get(list_stock_movements_for_lot),
        )
}

async fn list_stock_movements(
    State(state): State<AppState>,
) -> Result<Json<Vec<StockMovement>>, (StatusCode, Json<ApiError>)> {
    tracing::debug!("API handler started: list stock movements");

    state
        .services
        .stock_movements
        .list_stock_movements()
        .await
        .map(Json)
        .map_err(map_application_error)
}

async fn list_stock_movements_for_lot(
    State(state): State<AppState>,
    Path(lot_id): Path<Uuid>,
) -> Result<Json<Vec<StockMovement>>, (StatusCode, Json<ApiError>)> {
    tracing::debug!(
        lot_id = %lot_id,
        "API handler started: list stock movements for lot",
    );

    state
        .services
        .stock_movements
        .list_stock_movements_for_lot(lot_id)
        .await
        .map(Json)
        .map_err(map_application_error)
}
