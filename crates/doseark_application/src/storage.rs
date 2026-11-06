use std::path::Path;

use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::ApplicationResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageEngine {
    Postgres,
    Sqlite,
}

impl StorageEngine {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Postgres => "postgresql",
            Self::Sqlite => "sqlite",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StorageCapabilities {
    pub connection_stats: bool,
    pub table_stats: bool,
    pub backups: bool,
    pub managed_backups: bool,
}

#[derive(Debug, Clone)]
pub struct StorageRuntimeInfo {
    pub engine: StorageEngine,
    pub database_name: String,
    pub engine_version: String,
    pub applied_migrations: i64,
    pub applied_migration_revision: i64,
}

#[derive(Debug, Clone)]
pub struct StorageOverview {
    pub runtime: StorageRuntimeInfo,
    pub database_size_bytes: i64,
    pub active_connections: Option<i64>,
    pub max_connections: Option<i64>,
    pub pool_size: Option<u32>,
    pub pool_idle: Option<usize>,
    pub capabilities: StorageCapabilities,
}

#[derive(Debug, Clone)]
pub struct StorageTableStats {
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

#[async_trait]
pub trait StorageDiagnostics: Send + Sync {
    fn engine(&self) -> StorageEngine;
    fn capabilities(&self) -> StorageCapabilities;

    async fn runtime_info(&self) -> ApplicationResult<StorageRuntimeInfo>;
    async fn overview(&self) -> ApplicationResult<StorageOverview>;
    async fn table_stats(&self) -> ApplicationResult<Vec<StorageTableStats>>;
}

/// Optional managed backup support for storage adapters that can safely create
/// a consistent backup while the application is running.
#[async_trait]
pub trait StorageBackupManager: Send + Sync {
    fn engine(&self) -> StorageEngine;
    async fn create_backup(&self, target: &Path) -> ApplicationResult<()>;
}
