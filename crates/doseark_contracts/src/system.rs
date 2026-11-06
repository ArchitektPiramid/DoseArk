use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct StorageCapabilitiesResponse {
    pub connection_stats: bool,
    pub table_stats: bool,
    pub backups: bool,
    pub managed_backups: bool,
}

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
    pub latency_ms: u64,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub storage_mode: String,
    pub checked_at: DateTime<Utc>,
    pub started_at: DateTime<Utc>,
    pub uptime_seconds: i64,
    pub database: DatabaseHealthResponse,
}

#[derive(Debug, Clone, Serialize)]
pub struct DatabaseOverviewResponse {
    pub engine: String,
    pub database_name: String,
    pub server_version: String,
    pub schema_version: String,
    pub expected_migration_revision: i64,
    pub applied_migrations: i64,
    pub applied_migration_revision: i64,
    pub compatible: bool,
    pub database_size_bytes: i64,
    pub active_connections: i64,
    pub max_connections: i64,
    pub pool_size: u32,
    pub pool_idle: usize,
    pub capabilities: StorageCapabilitiesResponse,
}

#[derive(Debug, Clone, Serialize)]
pub struct DatabaseTableStatsResponse {
    pub schema_name: String,
    pub table_name: String,
    pub live_rows: i64,
    pub dead_rows: i64,
    pub sequential_scans: i64,
    pub index_scans: i64,
    pub table_bytes: i64,
    pub index_bytes: i64,
    pub total_bytes: i64,
    pub last_autovacuum: Option<DateTime<Utc>>,
    pub last_autoanalyze: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeConfigValueResponse {
    pub value: String,
    pub source: String,
    pub variable: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeEnvironmentVariableResponse {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeConfigurationResponse {
    pub env_file: Option<String>,
    pub environment_variables: Vec<RuntimeEnvironmentVariableResponse>,
    pub profiles_file: String,
    pub credentials_file: String,
    pub credentials_file_exists: bool,
    pub storage_backend: RuntimeConfigValueResponse,
    pub database_target: RuntimeConfigValueResponse,
    pub backup_dir: RuntimeConfigValueResponse,
    pub bind_addr: RuntimeConfigValueResponse,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProfileValidationCheckResponse {
    pub key: String,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProfileValidationResponse {
    pub profile_id: String,
    pub profile_file: String,
    pub credentials_file: String,
    pub ok: bool,
    pub checks: Vec<ProfileValidationCheckResponse>,
}
