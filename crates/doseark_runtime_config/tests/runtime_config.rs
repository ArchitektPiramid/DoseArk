//! Integration checks for file-backed profile and credential contracts.
//!
//! Test files are isolated in the operating system's temporary directory;
//! no test reads or modifies the user's configured runtime files.

use std::{
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use doseark_runtime_config::{load_profiles, CredentialsResolver, ProfileCredentials};

static NEXT_FILE_ID: AtomicU64 = AtomicU64::new(0);

struct TestFile {
    path: PathBuf,
}

impl TestFile {
    fn path(&self) -> &Path {
        &self.path
    }

    fn unique_path() -> PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock must be valid")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "doseark-runtime-config-{}-{timestamp}-{}",
            std::process::id(),
            NEXT_FILE_ID.fetch_add(1, Ordering::Relaxed)
        ))
    }

    async fn with_contents(contents: &str) -> Self {
        let path = Self::unique_path();
        tokio::fs::write(&path, contents)
            .await
            .expect("temporary test file should be writable");
        Self { path }
    }
}

impl Drop for TestFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

#[tokio::test]
async fn supports_both_profile_versions_and_existing_defaults() {
    for version in [1, 2] {
        let json = format!(
            r#"{{"configVersion":{version},"defaultApiProfileId":"local","apiProfiles":[{{"id":"local","name":"Local"}}]}}"#
        );
        let file = TestFile::with_contents(&json).await;
        let loaded = load_profiles(file.path())
            .await
            .expect("valid profile file");
        let profile = &loaded.api_profiles[0];

        assert_eq!(loaded.config_version, version);
        assert_eq!(loaded.default_api_profile_id, "local");
        assert_eq!(profile.id, "local");
        assert_eq!(profile.mode, "web-remote");
        assert_eq!(profile.base_url, "");
        assert_eq!(profile.health_path, "/api/health");
        assert_eq!(profile.request_timeout_ms, 5_000);
        assert_eq!(profile.database.database_type, "auto");
        assert_eq!(profile.database.ssl_mode, "prefer");
        assert!(profile.credentials_ref.is_none());
        assert!(profile.database.path.is_none());
    }
}

#[tokio::test]
async fn preserves_explicit_profile_configuration() {
    let json = r#"{
        "configVersion": 2,
        "defaultApiProfileId": "primary",
        "apiProfiles": [{
            "id": "primary",
            "name": "Primary",
            "description": "Local profile",
            "mode": "tauri-local",
            "baseUrl": "http://localhost:8080",
            "healthPath": "/status",
            "requestTimeoutMs": 2500,
            "database": {"type":"postgres", "name":"doseark", "host":"db.local", "port":5433, "sslMode":"require"},
            "credentialsRef": "primary-db",
            "backupPath": "./backups",
            "attachmentsPath": "./attachments"
        }]
    }"#;
    let file = TestFile::with_contents(json).await;
    let loaded = load_profiles(file.path())
        .await
        .expect("custom profile is valid");
    let profile = &loaded.api_profiles[0];

    assert_eq!(profile.mode, "tauri-local");
    assert_eq!(profile.description.as_deref(), Some("Local profile"));
    assert_eq!(profile.health_path, "/status");
    assert_eq!(profile.request_timeout_ms, 2_500);
    assert_eq!(profile.database.database_type, "postgres");
    assert_eq!(profile.database.name, "doseark");
    assert_eq!(profile.database.host.as_deref(), Some("db.local"));
    assert_eq!(profile.database.port, Some(5433));
    assert_eq!(profile.database.ssl_mode, "require");
    assert_eq!(profile.credentials_ref.as_deref(), Some("primary-db"));
    assert_eq!(profile.backup_path.as_deref(), Some(Path::new("./backups")));
    assert_eq!(
        profile.attachments_path.as_deref(),
        Some(Path::new("./attachments"))
    );
}

#[tokio::test]
async fn rejects_unsupported_versions_and_missing_default_profile() {
    for version in [0, 3] {
        let json = format!(
            r#"{{"configVersion":{version},"defaultApiProfileId":"local","apiProfiles":[{{"id":"local","name":"Local"}}]}}"#
        );
        let file = TestFile::with_contents(&json).await;
        let message = load_profiles(file.path()).await.unwrap_err().to_string();
        assert!(message.contains("expected 1 or 2"), "{message}");
    }

    let file = TestFile::with_contents(
        r#"{"configVersion":2,"defaultApiProfileId":"missing","apiProfiles":[{"id":"local","name":"Local"}]}"#,
    )
    .await;
    let message = load_profiles(file.path()).await.unwrap_err().to_string();
    assert!(
        message.contains("defaultApiProfileId 'missing'"),
        "{message}"
    );
}

#[tokio::test]
async fn reports_missing_and_invalid_profile_files() {
    let missing_path = TestFile::unique_path();
    let missing = load_profiles(&missing_path).await.unwrap_err().to_string();
    assert!(missing.contains("could not read profile file"), "{missing}");

    let file = TestFile::with_contents("{invalid json}").await;
    let invalid = load_profiles(file.path()).await.unwrap_err().to_string();
    assert!(invalid.contains("invalid profile JSON"), "{invalid}");
}

#[tokio::test]
async fn resolves_only_the_referenced_credentials() {
    let file = TestFile::with_contents(
        r#"{"main":{"username":"operator","password":"private-password","apiToken":"private-token"},"other":{"password":"other-password"}}"#,
    )
    .await;
    let resolver = CredentialsResolver::new(file.path());
    assert_eq!(resolver.path(), file.path());

    let credentials = resolver.resolve("main").await.expect("reference exists");
    assert_eq!(credentials.username.as_deref(), Some("operator"));
    assert_eq!(credentials.password.as_deref(), Some("private-password"));
    assert_eq!(credentials.api_token.as_deref(), Some("private-token"));

    let missing = resolver.resolve("missing").await.unwrap_err().to_string();
    assert!(
        missing.contains("reference 'missing' was not found"),
        "{missing}"
    );
    assert!(!missing.contains("private-password"));
}

#[tokio::test]
async fn permits_missing_optional_credentials_and_rejects_invalid_json() {
    let file = TestFile::with_contents(r#"{"guest":{}}"#).await;
    let credentials = CredentialsResolver::new(file.path())
        .resolve("guest")
        .await
        .expect("empty credentials entry is supported");
    assert!(credentials.username.is_none());
    assert!(credentials.password.is_none());
    assert!(credentials.api_token.is_none());

    let invalid = TestFile::with_contents("{broken credentials}").await;
    let error = CredentialsResolver::new(invalid.path())
        .resolve("guest")
        .await
        .unwrap_err()
        .to_string();
    assert!(error.contains("invalid credentials JSON"), "{error}");
}

#[test]
fn debug_output_never_contains_secret_values() {
    let credentials = ProfileCredentials {
        username: Some("private-username".into()),
        password: Some("private-password".into()),
        api_token: Some("private-token".into()),
    };
    let debug = format!("{credentials:?}");
    for secret in ["private-username", "private-password", "private-token"] {
        assert!(!debug.contains(secret), "Debug exposed a credential");
    }
    assert!(debug.contains("[REDACTED]"));

    let empty = format!("{:?}", ProfileCredentials::default());
    assert!(empty.contains("None"));
}
