//! Database diagnostics keep stable field names, numeric values and timestamps.

use chrono::{DateTime, Utc};
use doseark_contracts::{
    DatabaseOverviewResponse, DatabaseTableStatsResponse, StorageCapabilitiesResponse,
};
use serde_json::{json, to_value};

#[test]
fn overview_serializes_capabilities_and_connection_counters() {
    let response = DatabaseOverviewResponse {
        engine: "postgresql".into(),
        database_name: "doseark".into(),
        server_version: "17.0".into(),
        schema_version: "0.1.0".into(),
        expected_migration_revision: 5,
        applied_migrations: 5,
        applied_migration_revision: 5,
        compatible: true,
        database_size_bytes: 2048,
        active_connections: 3,
        max_connections: 10,
        pool_size: 8,
        pool_idle: 5,
        capabilities: StorageCapabilitiesResponse {
            connection_stats: true,
            table_stats: true,
            backups: true,
            managed_backups: false,
        },
    };

    assert_eq!(
        to_value(response).expect("serializable database overview"),
        json!({
            "engine": "postgresql",
            "database_name": "doseark",
            "server_version": "17.0",
            "schema_version": "0.1.0",
            "expected_migration_revision": 5,
            "applied_migrations": 5,
            "applied_migration_revision": 5,
            "compatible": true,
            "database_size_bytes": 2048,
            "active_connections": 3,
            "max_connections": 10,
            "pool_size": 8,
            "pool_idle": 5,
            "capabilities": {
                "connection_stats": true,
                "table_stats": true,
                "backups": true,
                "managed_backups": false
            }
        })
    );
}

#[test]
fn table_stats_preserve_nullable_maintenance_dates() {
    let analyzed_at = DateTime::parse_from_rfc3339("2026-10-05T08:00:00Z")
        .expect("valid timestamp")
        .with_timezone(&Utc);
    let response = DatabaseTableStatsResponse {
        schema_name: "public".into(),
        table_name: "dose_logs".into(),
        live_rows: 25,
        dead_rows: 2,
        sequential_scans: 4,
        index_scans: 100,
        table_bytes: 8192,
        index_bytes: 4096,
        total_bytes: 12288,
        last_autovacuum: None,
        last_autoanalyze: Some(analyzed_at),
    };

    assert_eq!(
        to_value(response).expect("serializable table statistics"),
        json!({
            "schema_name": "public",
            "table_name": "dose_logs",
            "live_rows": 25,
            "dead_rows": 2,
            "sequential_scans": 4,
            "index_scans": 100,
            "table_bytes": 8192,
            "index_bytes": 4096,
            "total_bytes": 12288,
            "last_autovacuum": null,
            "last_autoanalyze": "2026-10-05T08:00:00Z"
        })
    );
}
