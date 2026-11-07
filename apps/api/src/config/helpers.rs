//! Environment fallback and credential-free storage target helpers.

use std::{env, fmt::Display, path::{Path, PathBuf}, str::FromStr};
use anyhow::anyhow;
use super::{env_or_legacy, StorageBackend};

pub(super) fn env_or_legacy_with_name(
    primary: &'static str,
    legacy: &'static str,
) -> Option<(&'static str, String)> {
    env::var(primary)
        .ok()
        .map(|value| (primary, value))
        .or_else(|| env::var(legacy).ok().map(|value| (legacy, value)))
}

pub(super) fn env_value_or_legacy<T>(primary: &str, legacy: &str, default: T) -> anyhow::Result<T>
where
    T: Copy + Display + FromStr,
    T::Err: Display,
{
    let Some(raw_value) = env_or_legacy(primary, legacy) else {
        return Ok(default);
    };

    raw_value.parse::<T>().map_err(|error| {
        anyhow!(
            "invalid value for {primary}: '{raw_value}' ({error}); expected a valid value instead of falling back silently"
        )
    })
}

pub(super) fn explicit_backup_dir(
    storage_backend: StorageBackend,
) -> Option<(&'static str, String)> {
    if let Ok(value) = env::var("DOSEARK_BACKUP_DIR") {
        return Some(("DOSEARK_BACKUP_DIR", value));
    }

    if storage_backend == StorageBackend::Postgres {
        return env::var("MEDTAKER_BACKUP_DIR")
            .ok()
            .map(|value| ("MEDTAKER_BACKUP_DIR", value));
    }

    None
}

pub(super) fn default_backup_dir(storage_backend: StorageBackend, sqlite_path: &Path) -> PathBuf {
    match storage_backend {
        StorageBackend::Sqlite => sqlite_path
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."))
            .join("backups"),
        StorageBackend::Postgres => PathBuf::from("./backups"),
    }
}

pub(super) fn summarize_database_url(value: &str) -> String {
    let without_scheme = value
        .split_once("://")
        .map_or(value, |(_, remainder)| remainder);
    let without_credentials = without_scheme
        .rsplit_once('@')
        .map_or(without_scheme, |(_, remainder)| remainder);
    let without_query = without_credentials
        .split_once('?')
        .map_or(without_credentials, |(target, _)| target);

    if without_query.trim().is_empty() {
        "<invalid-database-target>".to_string()
    } else {
        without_query.to_string()
    }
}
