//! Database diagnostic responses exposed by administrative endpoints
//!
//! Capability flags describe which metrics and backup operations are available.
//! Statistics remain transport values; querying and interpreting them is the
//! responsibility of the storage adapter and application layers.

use chrono::{DateTime, Utc};
use serde::Serialize;

/// Diagnostics supported by the active database adapter.
#[derive(Debug, Clone, Serialize)]
pub struct StorageCapabilitiesResponse {
    pub connection_stats: bool,
    pub table_stats: bool,
    pub backups: bool,
    pub managed_backups: bool,
}

/// Storage overview and schema compatibility data shown in the admin UI.
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

/// Per-table usage and maintenance counters from storage diagnostics.
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
