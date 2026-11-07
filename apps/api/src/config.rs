//! Runtime settings for HTTP listeners, storage selection and admin diagnostics.
//!
//! Source metadata is retained for the runtime settings UI while database credentials
//! are redacted before connection details are exposed over HTTP.

use std::{env, path::PathBuf, time::Duration};

use anyhow::{anyhow, Context};

mod helpers;
use self::helpers::{
    default_backup_dir, env_or_legacy_with_name, env_value_or_legacy, explicit_backup_dir,
    summarize_database_url,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageBackend {
    Postgres,
    Sqlite,
}

impl StorageBackend {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Postgres => "postgres",
            Self::Sqlite => "sqlite",
        }
    }

    fn from_env() -> anyhow::Result<Self> {
        let value = env::var("DOSEARK_STORAGE")
            .context("required environment variable DOSEARK_STORAGE is missing")?;

        match value.trim().to_ascii_lowercase().as_str() {
            "postgres" | "postgresql" => Ok(Self::Postgres),
            "sqlite" => Ok(Self::Sqlite),
            other => Err(anyhow!(
                "invalid DOSEARK_STORAGE value '{other}'; expected 'postgres' or 'sqlite'"
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConfigValueSource {
    pub kind: &'static str,
    pub variable: Option<&'static str>,
}

impl ConfigValueSource {
    fn environment(variable: &'static str) -> Self {
        Self {
            kind: "env",
            variable: Some(variable),
        }
    }

    fn default() -> Self {
        Self {
            kind: "default",
            variable: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApiConfigSources {
    pub storage_backend: ConfigValueSource,
    pub database_target: ConfigValueSource,
    pub backup_dir: ConfigValueSource,
    pub bind_addr: ConfigValueSource,
}

const DEFAULT_DB_CONNECT_ATTEMPTS: u32 = 3;
const DEFAULT_DB_CONNECT_TIMEOUT_MS: u64 = 5_000;
const DEFAULT_DB_RETRY_DELAY_MS: u64 = 1_500;
const DEFAULT_DB_MAX_CONNECTIONS: u32 = 10;
const DEFAULT_MAX_BACKUP_DOWNLOAD_BYTES: u64 = 256 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub storage_backend: StorageBackend,
    pub database_url: String,
    pub sqlite_path: PathBuf,
    pub bind_addr: String,
    pub backup_dir: PathBuf,
    pub profiles_file: PathBuf,
    pub credentials_file: PathBuf,
    pub env_file: Option<PathBuf>,
    pub sources: ApiConfigSources,
    pub max_backup_download_bytes: u64,
    pub database_connect_attempts: u32,
    pub database_connect_timeout: Duration,
    pub database_retry_delay: Duration,
    pub database_max_connections: u32,
}

impl ApiConfig {
    /// Reads the local API and storage configuration from environment variables.
    pub fn from_env(env_file: Option<PathBuf>) -> anyhow::Result<Self> {
        let storage_backend = StorageBackend::from_env()?;
        let database_url = env::var("DATABASE_URL").unwrap_or_default();
        if storage_backend == StorageBackend::Postgres && database_url.trim().is_empty() {
            return Err(anyhow!(
                "required environment variable DATABASE_URL is missing while DOSEARK_STORAGE=postgres"
            ));
        }

        let sqlite_path_env = env::var("DOSEARK_SQLITE_PATH").ok();
        let sqlite_path = sqlite_path_env
            .as_deref()
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("./data/doseark.sqlite"));

        let bind_addr_env = env_or_legacy_with_name("DOSEARK_API_BIND", "MEDTAKER_API_BIND");
        let bind_addr = bind_addr_env
            .as_ref()
            .map(|(_, value)| value.clone())
            .unwrap_or_else(|| "127.0.0.1:8080".to_string());

        let backup_dir_env = explicit_backup_dir(storage_backend);
        let backup_dir = backup_dir_env
            .as_ref()
            .map(|(_, value)| PathBuf::from(value.as_str()))
            .unwrap_or_else(|| default_backup_dir(storage_backend, &sqlite_path));

        let profiles_file = env::var("DOSEARK_PROFILES_FILE")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("./config/profiles.json"));
        let credentials_file = env::var("DOSEARK_CREDENTIALS_FILE")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("./config/credentials.local.json"));

        let max_backup_download_bytes = env_value_or_legacy(
            "DOSEARK_MAX_BACKUP_DOWNLOAD_BYTES",
            "MEDTAKER_MAX_BACKUP_DOWNLOAD_BYTES",
            DEFAULT_MAX_BACKUP_DOWNLOAD_BYTES,
        )?;
        let database_connect_attempts = env_value_or_legacy(
            "DOSEARK_DB_CONNECT_ATTEMPTS",
            "MEDTAKER_DB_CONNECT_ATTEMPTS",
            DEFAULT_DB_CONNECT_ATTEMPTS,
        )?
        .max(1);
        let database_connect_timeout = Duration::from_millis(env_value_or_legacy(
            "DOSEARK_DB_CONNECT_TIMEOUT_MS",
            "MEDTAKER_DB_CONNECT_TIMEOUT_MS",
            DEFAULT_DB_CONNECT_TIMEOUT_MS,
        )?);
        let database_retry_delay = Duration::from_millis(env_value_or_legacy(
            "DOSEARK_DB_RETRY_DELAY_MS",
            "MEDTAKER_DB_RETRY_DELAY_MS",
            DEFAULT_DB_RETRY_DELAY_MS,
        )?);
        let database_max_connections = env_value_or_legacy(
            "DOSEARK_DB_MAX_CONNECTIONS",
            "MEDTAKER_DB_MAX_CONNECTIONS",
            DEFAULT_DB_MAX_CONNECTIONS,
        )?
        .max(1);

        let database_source = match storage_backend {
            StorageBackend::Postgres => ConfigValueSource::environment("DATABASE_URL"),
            StorageBackend::Sqlite if sqlite_path_env.is_some() => {
                ConfigValueSource::environment("DOSEARK_SQLITE_PATH")
            }
            StorageBackend::Sqlite => ConfigValueSource::default(),
        };
        let backup_source = backup_dir_env
            .as_ref()
            .map(|(name, _)| ConfigValueSource::environment(*name))
            .unwrap_or_else(ConfigValueSource::default);
        let bind_source = bind_addr_env
            .as_ref()
            .map(|(name, _)| ConfigValueSource::environment(*name))
            .unwrap_or_else(ConfigValueSource::default);

        Ok(Self {
            storage_backend,
            database_url,
            sqlite_path,
            bind_addr,
            backup_dir,
            profiles_file,
            credentials_file,
            env_file,
            sources: ApiConfigSources {
                storage_backend: ConfigValueSource::environment("DOSEARK_STORAGE"),
                database_target: database_source,
                backup_dir: backup_source,
                bind_addr: bind_source,
            },
            max_backup_download_bytes,
            database_connect_attempts,
            database_connect_timeout,
            database_retry_delay,
            database_max_connections,
        })
    }

    /// Returns a credential-free database target suitable for console logs and diagnostics.
    pub fn database_target(&self) -> String {
        match self.storage_backend {
            StorageBackend::Postgres => summarize_database_url(&self.database_url),
            StorageBackend::Sqlite => self.sqlite_path.display().to_string(),
        }
    }

    /// Returns the non-secret DoseArk runtime variables currently present in the process
    /// environment. Values loaded from `.env` are present here after dotenv initialization.
    /// DATABASE_URL is reduced to a credential-free database target before it reaches the UI.
    pub fn visible_environment_variables(&self) -> Vec<(String, String)> {
        const VARIABLES: &[&str] = &[
            "DOSEARK_STORAGE",
            "DATABASE_URL",
            "DOSEARK_SQLITE_PATH",
            "DOSEARK_API_BIND",
            "RUST_LOG",
            "DOSEARK_LOG_CONSOLE_TARGET",
            "DOSEARK_RUNTIME_SETTINGS_PATH",
            "DOSEARK_BACKUP_DIR",
            "DOSEARK_MAX_BACKUP_DOWNLOAD_BYTES",
            "DOSEARK_DB_CONNECT_ATTEMPTS",
            "DOSEARK_DB_CONNECT_TIMEOUT_MS",
            "DOSEARK_DB_RETRY_DELAY_MS",
            "DOSEARK_DB_MAX_CONNECTIONS",
            "DOSEARK_ENV_FILE",
            "DOSEARK_PROFILES_FILE",
            "DOSEARK_CREDENTIALS_FILE",
        ];

        VARIABLES
            .iter()
            .filter_map(|name| {
                let raw = env::var(name).ok()?;
                let value = if *name == "DATABASE_URL" {
                    summarize_database_url(&raw)
                } else {
                    raw
                };
                Some(((*name).to_string(), value))
            })
            .collect()
    }
}

pub fn env_or_legacy(primary: &str, legacy: &str) -> Option<String> {
    env::var(primary).ok().or_else(|| env::var(legacy).ok())
}

#[cfg(test)]
#[path = "../tests/unit/config.rs"]
mod tests;
