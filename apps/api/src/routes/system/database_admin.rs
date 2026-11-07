//! Database diagnostics HTTP routes for administrative status views.

use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use doseark_application::{ApplicationError, DATABASE_SCHEMA_VERSION, EXPECTED_MIGRATION_REVISION};
use doseark_contracts::{
    DatabaseOverviewResponse, DatabaseTableStatsResponse, StorageCapabilitiesResponse,
};

use crate::state::AppState;

use crate::routes::error::ApiError;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/admin/database/overview", get(database_overview))
        .route("/api/admin/database/tables", get(database_tables))
}

async fn database_overview(
    State(state): State<AppState>,
) -> Result<Json<DatabaseOverviewResponse>, (StatusCode, Json<ApiError>)> {
    let overview = state
        .storage_diagnostics
        .overview()
        .await
        .map_err(map_storage_error)?;
    let capabilities = overview.capabilities;

    Ok(Json(DatabaseOverviewResponse {
        engine: overview.runtime.engine.as_str().to_string(),
        compatible: overview.runtime.applied_migration_revision == EXPECTED_MIGRATION_REVISION,
        database_name: overview.runtime.database_name,
        server_version: overview.runtime.engine_version,
        schema_version: DATABASE_SCHEMA_VERSION.to_string(),
        expected_migration_revision: EXPECTED_MIGRATION_REVISION,
        applied_migrations: overview.runtime.applied_migrations,
        applied_migration_revision: overview.runtime.applied_migration_revision,
        database_size_bytes: overview.database_size_bytes,
        active_connections: overview.active_connections.unwrap_or_default(),
        max_connections: overview.max_connections.unwrap_or_default(),
        pool_size: overview.pool_size.unwrap_or_default(),
        pool_idle: overview.pool_idle.unwrap_or_default(),
        capabilities: StorageCapabilitiesResponse {
            connection_stats: capabilities.connection_stats,
            table_stats: capabilities.table_stats,
            backups: capabilities.backups,
            managed_backups: capabilities.managed_backups,
        },
    }))
}

async fn database_tables(
    State(state): State<AppState>,
) -> Result<Json<Vec<DatabaseTableStatsResponse>>, (StatusCode, Json<ApiError>)> {
    let tables = state
        .storage_diagnostics
        .table_stats()
        .await
        .map_err(map_storage_error)?;

    Ok(Json(
        tables
            .into_iter()
            .map(|table| DatabaseTableStatsResponse {
                schema_name: table.schema_name,
                table_name: table.table_name,
                live_rows: table.live_rows,
                dead_rows: table.dead_rows,
                sequential_scans: table.sequential_scans,
                index_scans: table.index_scans,
                table_bytes: table.table_bytes,
                index_bytes: table.index_bytes,
                total_bytes: table.total_bytes,
                last_autovacuum: table.last_autovacuum,
                last_autoanalyze: table.last_autoanalyze,
            })
            .collect(),
    ))
}

fn map_storage_error(error: ApplicationError) -> (StatusCode, Json<ApiError>) {
    tracing::error!(error = %error, "Storage diagnostics operation failed");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiError {
            code: "STORAGE_DIAGNOSTICS_FAILED",
            message: "Storage diagnostics operation failed.".to_string(),
        }),
    )
}
