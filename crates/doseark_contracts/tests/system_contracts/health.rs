//! Health responses retain their serialized keys and nullable database fields.

use chrono::{DateTime, Utc};
use doseark_contracts::{DatabaseHealthResponse, HealthResponse};
use serde_json::{json, to_value};

#[test]
fn health_response_keeps_nested_status_and_utc_timestamps() {
    let now = DateTime::parse_from_rfc3339("2026-11-07T12:30:00Z")
        .expect("valid timestamp")
        .with_timezone(&Utc);

    let response = HealthResponse {
        status: "degraded".into(),
        service: "doseark-api".into(),
        version: "0.1.0".into(),
        storage_mode: "remote-sqlite".into(),
        checked_at: now.clone(),
        started_at: now,
        uptime_seconds: 120,
        database: DatabaseHealthResponse {
            status: "error".into(),
            engine: "sqlite".into(),
            database_name: None,
            server_version: None,
            schema_version: "0.1.0".into(),
            expected_migration_revision: 5,
            applied_migrations: None,
            applied_migration_revision: None,
            compatible: false,
            latency_ms: 18,
            error: Some("Storage health check failed".into()),
        },
    };

    assert_eq!(
        to_value(response).expect("serializable health response"),
        json!({
            "status": "degraded",
            "service": "doseark-api",
            "version": "0.1.0",
            "storage_mode": "remote-sqlite",
            "checked_at": "2026-11-07T12:30:00Z",
            "started_at": "2026-11-07T12:30:00Z",
            "uptime_seconds": 120,
            "database": {
                "status": "error",
                "engine": "sqlite",
                "database_name": null,
                "server_version": null,
                "schema_version": "0.1.0",
                "expected_migration_revision": 5,
                "applied_migrations": null,
                "applied_migration_revision": null,
                "compatible": false,
                "latency_ms": 18,
                "error": "Storage health check failed"
            }
        })
    );
}
