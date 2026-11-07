//! Storage location HTTP routes and query handling.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, patch, post},
    Json, Router,
};
use doseark_domain::{CreateStorageLocationInput, StorageLocation, UpdateStorageLocationInput};
use serde::Deserialize;
use uuid::Uuid;

use crate::state::AppState;

use super::error::{map_application_error, ApiError};

#[derive(Debug, Deserialize)]
struct ListStorageLocationsQuery {
    include_archived: Option<bool>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/storage-locations", get(list_storage_locations))
        .route("/api/storage-locations", post(create_storage_location))
        .route(
            "/api/storage-locations/{storage_location_id}",
            patch(update_storage_location),
        )
        .route(
            "/api/storage-locations/{storage_location_id}",
            delete(archive_storage_location),
        )
}

async fn list_storage_locations(
    State(state): State<AppState>,
    Query(query): Query<ListStorageLocationsQuery>,
) -> Result<Json<Vec<StorageLocation>>, (StatusCode, Json<ApiError>)> {
    state
        .services
        .storage_locations
        .list_storage_locations(query.include_archived.unwrap_or(false))
        .await
        .map(Json)
        .map_err(map_application_error)
}

async fn create_storage_location(
    State(state): State<AppState>,
    Json(input): Json<CreateStorageLocationInput>,
) -> Result<Json<StorageLocation>, (StatusCode, Json<ApiError>)> {
    state
        .services
        .storage_locations
        .create_storage_location(input)
        .await
        .map(Json)
        .map_err(map_application_error)
}

async fn update_storage_location(
    State(state): State<AppState>,
    Path(storage_location_id): Path<Uuid>,
    Json(input): Json<UpdateStorageLocationInput>,
) -> Result<Json<StorageLocation>, (StatusCode, Json<ApiError>)> {
    state
        .services
        .storage_locations
        .update_storage_location(storage_location_id, input)
        .await
        .map(Json)
        .map_err(map_application_error)
}

async fn archive_storage_location(
    State(state): State<AppState>,
    Path(storage_location_id): Path<Uuid>,
) -> Result<Json<StorageLocation>, (StatusCode, Json<ApiError>)> {
    state
        .services
        .storage_locations
        .archive_storage_location(storage_location_id)
        .await
        .map(Json)
        .map_err(map_application_error)
}
