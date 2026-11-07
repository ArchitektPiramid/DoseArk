//! Server-side lookup of credentials referenced by runtime profiles.
//!
//! Credentials are read only when explicitly resolved and never serialized by
//! this crate. Debug output deliberately hides field values to avoid accidental
//! exposure in logs or diagnostic error reports.

use std::{
    collections::HashMap,
    fmt,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context};
use serde::Deserialize;

/// Optional authentication values stored outside the public profile file.
#[derive(Clone, Deserialize, Default)]
pub struct ProfileCredentials {
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default, rename = "apiToken")]
    pub api_token: Option<String>,
}

impl fmt::Debug for ProfileCredentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Preserve the optional-field shape without logging credential contents
        f.debug_struct("ProfileCredentials")
            .field("username", &self.username.as_ref().map(|_| "[REDACTED]"))
            .field("password", &self.password.as_ref().map(|_| "[REDACTED]"))
            .field("api_token", &self.api_token.as_ref().map(|_| "[REDACTED]"))
            .finish()
    }
}

/// Resolves profile references against a local, Rust-only credentials file.
#[derive(Debug, Clone)]
pub struct CredentialsResolver {
    path: PathBuf,
}

impl CredentialsResolver {
    /// Selects the credentials JSON file without reading it yet.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// Returns the configured file path for server-side diagnostics.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Loads an entry by reference, returning an error for missing or invalid data.
    pub async fn resolve(&self, reference: &str) -> anyhow::Result<ProfileCredentials> {
        let raw = tokio::fs::read_to_string(&self.path)
            .await
            .with_context(|| format!("could not read credentials file {}", self.path.display()))?;
        let entries: HashMap<String, ProfileCredentials> = serde_json::from_str(&raw)
            .with_context(|| format!("invalid credentials JSON in {}", self.path.display()))?;

        entries
            .get(reference)
            .cloned()
            .ok_or_else(|| anyhow!("credentials reference '{reference}' was not found"))
    }
}
