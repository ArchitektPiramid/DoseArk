use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfilesFile {
    pub config_version: u32,
    pub default_api_profile_id: String,
    pub api_profiles: Vec<ProfileConfig>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileConfig {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default = "default_mode")]
    pub mode: String,
    #[serde(default)]
    pub base_url: String,
    #[serde(default = "default_health_path")]
    pub health_path: String,
    #[serde(default = "default_timeout")]
    pub request_timeout_ms: u64,
    #[serde(default)]
    pub database: ProfileDatabaseConfig,
    #[serde(default)]
    pub credentials_ref: Option<String>,
    #[serde(default)]
    pub backup_path: Option<PathBuf>,
    #[serde(default)]
    pub attachments_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProfileDatabaseConfig {
    #[serde(rename = "type", default = "default_database_type")]
    pub database_type: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: Option<PathBuf>,
    #[serde(default)]
    pub host: Option<String>,
    #[serde(default)]
    pub port: Option<u16>,
    #[serde(rename = "sslMode", default = "default_ssl_mode")]
    pub ssl_mode: String,
}

impl Default for ProfileDatabaseConfig {
    fn default() -> Self {
        Self {
            database_type: default_database_type(),
            name: String::new(),
            path: None,
            host: None,
            port: None,
            ssl_mode: default_ssl_mode(),
        }
    }
}

pub async fn load_profiles(path: &Path) -> anyhow::Result<ProfilesFile> {
    let raw = tokio::fs::read_to_string(path)
        .await
        .with_context(|| format!("could not read profile file {}", path.display()))?;
    let config: ProfilesFile = serde_json::from_str(&raw)
        .with_context(|| format!("invalid profile JSON in {}", path.display()))?;

    if config.config_version != 1 && config.config_version != 2 {
        return Err(anyhow!(
            "unsupported profiles configVersion {}; expected 1 or 2",
            config.config_version
        ));
    }

    if !config
        .api_profiles
        .iter()
        .any(|profile| profile.id == config.default_api_profile_id)
    {
        return Err(anyhow!(
            "defaultApiProfileId '{}' does not match any profile",
            config.default_api_profile_id
        ));
    }

    Ok(config)
}

fn default_mode() -> String { "web-remote".to_string() }
fn default_health_path() -> String { "/api/health".to_string() }
fn default_timeout() -> u64 { 5_000 }
fn default_database_type() -> String { "auto".to_string() }
fn default_ssl_mode() -> String { "prefer".to_string() }
