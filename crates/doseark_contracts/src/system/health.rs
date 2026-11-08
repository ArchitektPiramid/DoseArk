//! Health-check responses returned by the system health endpoint
//!
//! These DTOs expose the API state and the result of its storage probe.
//! Optional database fields serialize as JSON null when the probe fails.

use chrono::{DateTime, Utc};
use serde::Serialize;

/// Result of the storage connectivity and migration compatibility check.
#[derive(Debug, Clone, Serialize)]
pub struct DatabaseHealthResponse {
    pub status: String,
    pub engine: String,
    pub database_name: Option<String>,
    pub server_version: Option<String>,
    pub schema_version: String,
    pub expected_migration_revision: i64,
    pub applied_migrations: Option<i64>,
    pub applied_migration_revision: Option<i64>,
    pub compatible: bool,
    /// Duration of the storage probe, in milliseconds
    pub latency_ms: u64,
    pub error: Option<String>,
}

/// Version metadata for the running HTTP API.
#[derive(Debug, Clone, Serialize)]
pub struct ComponentVersionResponse {
    pub version: String,
    pub channel: String,
}

/// Individual Rust component versions included in the current API build.
#[derive(Debug, Clone, Serialize)]
pub struct BackendComponentVersionsResponse {
    pub domain: String,
    pub application: String,
    pub contracts: String,
    pub db_postgres: String,
    pub db_sqlite: String,
}

/// Public service status, including version metadata and database health.
#[derive(Debug, Clone, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub channel: String,
    pub api: ComponentVersionResponse,
    pub backend_components: BackendComponentVersionsResponse,
    pub storage_mode: String,
    /// UTC timestamps serialized as RFC 3339 strings
    pub checked_at: DateTime<Utc>,
    pub started_at: DateTime<Utc>,
    pub uptime_seconds: i64,
    pub database: DatabaseHealthResponse,
}
