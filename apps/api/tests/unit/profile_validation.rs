//! Connectivity-free tests for profile dispatch and status reporting.

use super::{validate_profile, ValidationCheck};
use doseark_runtime_config::{CredentialsResolver, ProfileConfig};
use serde_json::json;

fn test_profile(mode: &str, database_type: &str) -> ProfileConfig {
    serde_json::from_value(json!({
        "id": "local",
        "name": "Local Test",
        "mode": mode,
        "database": { "type": database_type }
    })).expect("valid test profile")
}

fn has_check(checks: &[ValidationCheck], key: &str, status: &str) -> bool {
    checks.iter().any(|check| check.key == key && check.status == status)
}

#[tokio::test]
async fn tauri_local_skips_server_only_database_and_backup_probes() {
    let resolver = CredentialsResolver::new("nonexistent-test-credentials.json");
    let checks = validate_profile(&test_profile("tauri-local", "postgres"), &resolver).await;
    assert_eq!(checks.len(), 2);
    assert!(has_check(&checks, "runtime", "warning"));
}

#[tokio::test]
async fn auto_database_is_a_warning_without_network_access() {
    let resolver = CredentialsResolver::new("nonexistent-test-credentials.json");
    let checks = validate_profile(&test_profile("web-remote", "auto"), &resolver).await;
    assert!(has_check(&checks, "database", "warning"));
    assert!(has_check(&checks, "backup", "skipped"));
}

#[tokio::test]
async fn unsupported_database_type_is_rejected_without_connecting() {
    let resolver = CredentialsResolver::new("nonexistent-test-credentials.json");
    let checks = validate_profile(&test_profile("web-remote", "unknown"), &resolver).await;
    assert!(has_check(&checks, "database", "error"));
}
