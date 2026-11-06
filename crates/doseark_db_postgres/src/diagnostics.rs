use async_trait::async_trait;
use chrono::{DateTime, Utc};
use doseark_application::{
    ApplicationError, ApplicationResult, StorageCapabilities, StorageDiagnostics, StorageEngine,
    StorageOverview, StorageRuntimeInfo, StorageTableStats,
};
use sqlx::PgPool;

#[derive(Clone)]
pub struct PostgresDiagnostics {
    pool: PgPool,
}

impl PostgresDiagnostics {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn query_runtime_info(&self) -> Result<StorageRuntimeInfo, sqlx::Error> {
        let (database_name, server_version): (String, String) = sqlx::query_as(
            "SELECT current_database(), current_setting('server_version')",
        )
        .fetch_one(&self.pool)
        .await?;

        let (applied_migrations, applied_migration_revision): (i64, i64) = sqlx::query_as(
            r#"
            SELECT
                COUNT(*)::BIGINT,
                COALESCE(MAX(version), 0)::BIGINT
            FROM _sqlx_migrations
            WHERE success = TRUE
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(StorageRuntimeInfo {
            engine: StorageEngine::Postgres,
            database_name,
            engine_version: server_version,
            applied_migrations,
            applied_migration_revision,
        })
    }
}

#[async_trait]
impl StorageDiagnostics for PostgresDiagnostics {
    fn engine(&self) -> StorageEngine {
        StorageEngine::Postgres
    }

    fn capabilities(&self) -> StorageCapabilities {
        StorageCapabilities {
            connection_stats: true,
            table_stats: true,
            backups: true,
            managed_backups: false,
        }
    }

    async fn runtime_info(&self) -> ApplicationResult<StorageRuntimeInfo> {
        self.query_runtime_info().await.map_err(map_sqlx_error)
    }

    async fn overview(&self) -> ApplicationResult<StorageOverview> {
        let runtime = self.query_runtime_info().await.map_err(map_sqlx_error)?;
        let (database_size_bytes, active_connections, max_connections): (i64, i64, i64) =
            sqlx::query_as(
                r#"
                SELECT
                    pg_database_size(current_database())::BIGINT,
                    (
                        SELECT COUNT(*)::BIGINT
                        FROM pg_stat_activity
                        WHERE datname = current_database()
                    ),
                    current_setting('max_connections')::BIGINT
                "#,
            )
            .fetch_one(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        Ok(StorageOverview {
            runtime,
            database_size_bytes,
            active_connections: Some(active_connections),
            max_connections: Some(max_connections),
            pool_size: Some(self.pool.size()),
            pool_idle: Some(self.pool.num_idle()),
            capabilities: self.capabilities(),
        })
    }

    async fn table_stats(&self) -> ApplicationResult<Vec<StorageTableStats>> {
        type Row = (
            String,
            String,
            i64,
            i64,
            i64,
            i64,
            i64,
            i64,
            i64,
            Option<DateTime<Utc>>,
            Option<DateTime<Utc>>,
        );

        let rows = sqlx::query_as::<_, Row>(
            r#"
            SELECT
                schemaname::TEXT AS schema_name,
                relname::TEXT AS table_name,
                n_live_tup::BIGINT AS live_rows,
                n_dead_tup::BIGINT AS dead_rows,
                seq_scan::BIGINT AS sequential_scans,
                idx_scan::BIGINT AS index_scans,
                pg_relation_size(relid)::BIGINT AS table_bytes,
                pg_indexes_size(relid)::BIGINT AS index_bytes,
                pg_total_relation_size(relid)::BIGINT AS total_bytes,
                last_autovacuum,
                last_autoanalyze
            FROM pg_stat_user_tables
            ORDER BY pg_total_relation_size(relid) DESC, relname ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(rows
            .into_iter()
            .map(
                |(
                    schema_name,
                    table_name,
                    live_rows,
                    dead_rows,
                    sequential_scans,
                    index_scans,
                    table_bytes,
                    index_bytes,
                    total_bytes,
                    last_autovacuum,
                    last_autoanalyze,
                )| StorageTableStats {
                    schema_name,
                    table_name,
                    live_rows,
                    dead_rows,
                    sequential_scans,
                    index_scans,
                    table_bytes,
                    index_bytes,
                    total_bytes,
                    last_autovacuum,
                    last_autoanalyze,
                },
            )
            .collect())
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ApplicationError {
    ApplicationError::Repository(error.to_string())
}
