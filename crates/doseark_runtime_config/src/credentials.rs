use std::{collections::HashMap, path::{Path, PathBuf}};

use anyhow::{anyhow, Context};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ProfileCredentials {
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default, rename = "apiToken")]
    pub api_token: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CredentialsResolver {
    path: PathBuf,
}

impl CredentialsResolver {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

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
