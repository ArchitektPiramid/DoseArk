//! Profile validation results retain every check and its status string.

use doseark_contracts::{ProfileValidationCheckResponse, ProfileValidationResponse};
use serde_json::{json, to_value};

#[test]
fn profile_validation_serializes_checks_without_remapping_status() {
    let response = ProfileValidationResponse {
        profile_id: "local".into(),
        profile_file: "./config/profiles.json".into(),
        credentials_file: "./config/credentials.local.json".into(),
        ok: false,
        checks: ["ok", "warning", "error", "skipped"]
            .into_iter()
            .map(|status| ProfileValidationCheckResponse {
                key: format!("check-{status}"),
                status: status.into(),
                message: format!("Result: {status}"),
            })
            .collect(),
    };

    assert_eq!(
        to_value(response).expect("serializable profile validation"),
        json!({
            "profile_id": "local",
            "profile_file": "./config/profiles.json",
            "credentials_file": "./config/credentials.local.json",
            "ok": false,
            "checks": [
                {"key": "check-ok", "status": "ok", "message": "Result: ok"},
                {"key": "check-warning", "status": "warning", "message": "Result: warning"},
                {"key": "check-error", "status": "error", "message": "Result: error"},
                {"key": "check-skipped", "status": "skipped", "message": "Result: skipped"}
            ]
        })
    );
}
