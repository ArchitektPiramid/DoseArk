//! Runtime configuration responses intended for administrative diagnostics
//!
//! Values must be sanitized by the API before constructing these DTOs.
//! The contracts do not read credentials, environment files, or process state.

use serde::Serialize;

/// Displayable configuration value with its origin and optional variable name.
#[derive(Debug, Clone, Serialize)]
pub struct RuntimeConfigValueResponse {
    /// Must not contain passwords, tokens, or raw credential-bearing URLs
    pub value: String,
    pub source: String,
    pub variable: Option<String>,
}

/// Environment variable name and a pre-filtered value safe for display.
#[derive(Debug, Clone, Serialize)]
pub struct RuntimeEnvironmentVariableResponse {
    pub name: String,
    pub value: String,
}

/// Paths, visible variables, and effective API configuration for the admin UI.
#[derive(Debug, Clone, Serialize)]
pub struct RuntimeConfigurationResponse {
    pub env_file: Option<String>,
    pub environment_variables: Vec<RuntimeEnvironmentVariableResponse>,
    pub profiles_file: String,
    pub credentials_file: String,
    pub credentials_file_exists: bool,
    pub storage_backend: RuntimeConfigValueResponse,
    pub database_target: RuntimeConfigValueResponse,
    pub backup_dir: RuntimeConfigValueResponse,
    pub bind_addr: RuntimeConfigValueResponse,
}
