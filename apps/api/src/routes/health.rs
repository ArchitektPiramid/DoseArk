//! API health endpoint and storage migration compatibility diagnostics.
//!
//! Includes compiled component versions to match the shared TypeScript SystemHealth contract.

use std::time::Instant;

use axum::{extract::State, routing::get, Json, Router};
use chrono::Utc;
use doseark_application::{StorageEngine, DATABASE_SCHEMA_VERSION, EXPECTED_MIGRATION_REVISION};
use doseark_contracts::{
    BackendComponentVersionsResponse, ComponentVersionResponse, DatabaseHealthResponse, HealthResponse,
};

use crate::{state::AppState, version};

pub fn router() -> Router<AppState> {
    Router::new().route("/api/health", get(health))
}

async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    let checked_at = Utc::now();
    let started_at = state.started_at.clone();
    let uptime_seconds = checked_at
        .signed_duration_since(started_at)
        .num_seconds()
        .max(0);
    let storage_check_started = Instant::now();
    let storage_engine = state.storage_diagnostics.engine();

    let database = match state.storage_diagnostics.runtime_info().await {
        Ok(info) => {
            let compatible = info.applied_migration_revision == EXPECTED_MIGRATION_REVISION;

            DatabaseHealthResponse {
                status: if compatible { "ok" } else { "error" }.to_string(),
                engine: info.engine.as_str().to_string(),
                database_name: Some(info.database_name),
                server_version: Some(info.engine_version),
                schema_version: DATABASE_SCHEMA_VERSION.to_string(),
                expected_migration_revision: EXPECTED_MIGRATION_REVISION,
                applied_migrations: Some(info.applied_migrations),
                applied_migration_revision: Some(info.applied_migration_revision),
                compatible,
                latency_ms: storage_check_started.elapsed().as_millis() as u64,
                error: if compatible {
                    None
                } else {
                    Some("Storage migration revision does not match the API expectation".to_string())
                },
            }
        }
        Err(error) => {
            tracing::warn!(error = %error, "Storage health check failed");

            DatabaseHealthResponse {
                status: "error".to_string(),
                engine: storage_engine.as_str().to_string(),
                database_name: None,
                server_version: None,
                schema_version: DATABASE_SCHEMA_VERSION.to_string(),
                expected_migration_revision: EXPECTED_MIGRATION_REVISION,
                applied_migrations: None,
                applied_migration_revision: None,
                compatible: false,
                latency_ms: storage_check_started.elapsed().as_millis() as u64,
                error: Some("Storage health check failed".to_string()),
            }
        }
    };

    let status = if database.status == "ok" {
        "ok"
    } else {
        "degraded"
    };

    Json(HealthResponse {
        status: status.to_string(),
        service: "doseark-api".to_string(),
        version: version::BACKEND_VERSION.to_string(),
        channel: version::CHANNEL.to_string(),
        api: ComponentVersionResponse {
            version: version::BACKEND_VERSION.to_string(),
            channel: version::CHANNEL.to_string(),
        },
        backend_components: BackendComponentVersionsResponse {
            domain: version::DOMAIN_VERSION.to_string(),
            application: version::APPLICATION_VERSION.to_string(),
            contracts: version::CONTRACTS_VERSION.to_string(),
            db_postgres: version::DB_POSTGRES_VERSION.to_string(),
            db_sqlite: version::DB_SQLITE_VERSION.to_string(),
        },
        storage_mode: storage_mode(storage_engine).to_string(),
        checked_at,
        started_at,
        uptime_seconds,
        database,
    })
}

const fn storage_mode(engine: StorageEngine) -> &'static str {
    match engine {
        StorageEngine::Postgres => "remote-postgresql",
        StorageEngine::Sqlite => "remote-sqlite",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_mode_names_match_existing_clients() {
        assert_eq!(storage_mode(StorageEngine::Postgres), "remote-postgresql");
        assert_eq!(storage_mode(StorageEngine::Sqlite), "remote-sqlite");
    }

    #[test]
    fn compiled_component_versions_are_present() {
        for version in [
            version::BACKEND_VERSION,
            version::DOMAIN_VERSION,
            version::APPLICATION_VERSION,
            version::CONTRACTS_VERSION,
            version::DB_POSTGRES_VERSION,
            version::DB_SQLITE_VERSION,
        ] {
            assert!(!version.is_empty());
        }
    }
}
