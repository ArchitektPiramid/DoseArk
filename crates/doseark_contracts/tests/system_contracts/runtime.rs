//! Runtime administration DTOs keep provenance and optional values in JSON.

use doseark_contracts::{
    RuntimeConfigValueResponse, RuntimeConfigurationResponse, RuntimeEnvironmentVariableResponse,
};
use serde_json::{json, to_value};

#[test]
fn runtime_configuration_preserves_sources_and_visible_variables() {
    let environment = RuntimeConfigValueResponse {
        value: "sqlite".into(),
        source: "env".into(),
        variable: Some("DOSEARK_STORAGE".into()),
    };
    let default = |value: &str| RuntimeConfigValueResponse {
        value: value.into(),
        source: "default".into(),
        variable: None,
    };
    let response = RuntimeConfigurationResponse {
        env_file: None,
        environment_variables: vec![RuntimeEnvironmentVariableResponse {
            name: "DOSEARK_STORAGE".into(),
            value: "sqlite".into(),
        }],
        profiles_file: "./config/profiles.json".into(),
        credentials_file: "./config/credentials.local.json".into(),
        credentials_file_exists: false,
        storage_backend: environment,
        database_target: default("./data/doseark.sqlite"),
        backup_dir: default("./data/backups"),
        bind_addr: default("127.0.0.1:8080"),
    };

    assert_eq!(
        to_value(response).expect("serializable runtime configuration"),
        json!({
            "env_file": null,
            "environment_variables": [{"name": "DOSEARK_STORAGE", "value": "sqlite"}],
            "profiles_file": "./config/profiles.json",
            "credentials_file": "./config/credentials.local.json",
            "credentials_file_exists": false,
            "storage_backend": {
                "value": "sqlite", "source": "env", "variable": "DOSEARK_STORAGE"
            },
            "database_target": {
                "value": "./data/doseark.sqlite", "source": "default", "variable": null
            },
            "backup_dir": {
                "value": "./data/backups", "source": "default", "variable": null
            },
            "bind_addr": {
                "value": "127.0.0.1:8080", "source": "default", "variable": null
            }
        })
    );
}
