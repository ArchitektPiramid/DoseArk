//! Credential resolution and presence-only status reporting.
//!
//! This module reports whether credential fields exist without returning their contents.

use doseark_runtime_config::{CredentialsResolver, ProfileConfig, ProfileCredentials};
use super::ValidationCheck;

pub(super) struct CredentialsResolution {
    pub(super) check: ValidationCheck,
    pub(super) value: Option<ProfileCredentials>,
}

pub(super) async fn resolve_credentials(
    profile: &ProfileConfig,
    resolver: &CredentialsResolver,
) -> CredentialsResolution {
    let Some(reference) = profile.credentials_ref.as_deref() else {
        return CredentialsResolution {
            check: ValidationCheck {
                key: "credentials",
                status: "warning",
                message: "No credentialsRef configured; PostgreSQL validation will try without file credentials.".to_string(),
            },
            value: None,
        };
    };

    match resolver.resolve(reference).await {
        Ok(value) => CredentialsResolution {
            check: credentials_success(reference, resolver, &value),
            value: Some(value),
        },
        Err(error) => CredentialsResolution {
            check: ValidationCheck {
                key: "credentials",
                status: "error",
                message: format!("credentialsRef '{reference}' could not be resolved: {error}"),
            },
            value: None,
        },
    }
}

pub(super) async fn validate_credentials(
    profile: &ProfileConfig,
    resolver: &CredentialsResolver,
    required: bool,
) -> ValidationCheck {
    let Some(reference) = profile.credentials_ref.as_deref() else {
        return ValidationCheck {
            key: "credentials",
            status: if required { "warning" } else { "skipped" },
            message: if required {
                "No credentialsRef configured.".to_string()
            } else {
                "SQLite does not require credentials.".to_string()
            },
        };
    };

    match resolver.resolve(reference).await {
        Ok(value) => credentials_success(reference, resolver, &value),
        Err(error) => ValidationCheck {
            key: "credentials",
            status: "error",
            message: format!("credentialsRef '{reference}' could not be resolved: {error}"),
        },
    }
}

fn credentials_success(
    reference: &str,
    resolver: &CredentialsResolver,
    value: &ProfileCredentials,
) -> ValidationCheck {
    ValidationCheck {
        key: "credentials",
        status: "ok",
        message: format!(
            "credentialsRef '{reference}' resolved server-side from {} (username {}, password {}, API token {}).",
            resolver.path().display(),
            presence(value.username.as_deref()),
            presence(value.password.as_deref()),
            presence(value.api_token.as_deref())
        ),
    }
}

fn presence(value: Option<&str>) -> &'static str {
    if value.is_some_and(|value| !value.is_empty()) {
        "set"
    } else {
        "not set"
    }
}
