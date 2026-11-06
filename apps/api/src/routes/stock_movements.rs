use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use doseark_domain::StockMovement;
use serde::Serialize;
use uuid::Uuid;

use crate::state::AppState;

#[derive(Debug, Serialize)]
struct ApiError {
    code: &'static str,
    message: String,
}

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
        .services.stock_movements
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
        .services.stock_movements
        .list_stock_movements_for_lot(lot_id)
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
