//! Runtime profile configuration and validation HTTP routes.

use crate::routes::error::ApiError;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use doseark_contracts::{
    ProfileValidationCheckResponse, ProfileValidationResponse, RuntimeConfigurationResponse,
    RuntimeConfigValueResponse, RuntimeEnvironmentVariableResponse,
};
use doseark_runtime_config::{load_profiles, CredentialsResolver};

use crate::{
    config::ConfigValueSource,
    profile_validation::validate_profile,
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/admin/runtime/config", get(runtime_configuration))
        .route("/api/admin/profiles/{profile_id}/validate", get(validate_profile_route))
}

async fn runtime_configuration(
    State(state): State<AppState>,
) -> Json<RuntimeConfigurationResponse> {
    let config = &state.config;

    Json(RuntimeConfigurationResponse {
        env_file: config.env_file.as_ref().map(|path| path.display().to_string()),
        environment_variables: config
            .visible_environment_variables()
            .into_iter()
            .map(|(name, value)| RuntimeEnvironmentVariableResponse { name, value })
            .collect(),
        profiles_file: config.profiles_file.display().to_string(),
        credentials_file: config.credentials_file.display().to_string(),
        credentials_file_exists: config.credentials_file.exists(),
        storage_backend: value_response(
            config.storage_backend.as_str().to_string(),
            &config.sources.storage_backend,
        ),
        database_target: value_response(
            config.database_target(),
            &config.sources.database_target,
        ),
        backup_dir: value_response(
            config.backup_dir.display().to_string(),
            &config.sources.backup_dir,
        ),
        bind_addr: value_response(config.bind_addr.clone(), &config.sources.bind_addr),
    })
}

async fn validate_profile_route(
    State(state): State<AppState>,
    Path(profile_id): Path<String>,
) -> Result<Json<ProfileValidationResponse>, (StatusCode, Json<ApiError>)> {
    let profiles = load_profiles(&state.config.profiles_file)
        .await
        .map_err(|error| internal_error("PROFILE_CONFIG_READ_FAILED", error.to_string()))?;

    let profile = profiles
        .api_profiles
        .iter()
        .find(|profile| profile.id == profile_id)
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ApiError {
                    code: "PROFILE_NOT_FOUND",
                    message: format!("Profile '{profile_id}' was not found."),
                }),
            )
        })?;

    let resolver = CredentialsResolver::new(&state.config.credentials_file);
    let checks = validate_profile(profile, &resolver).await;
    let ok = checks.iter().all(|check| check.status != "error");

    Ok(Json(ProfileValidationResponse {
        profile_id,
        profile_file: state.config.profiles_file.display().to_string(),
        credentials_file: state.config.credentials_file.display().to_string(),
        ok,
        checks: checks
            .into_iter()
            .map(|check| ProfileValidationCheckResponse {
                key: check.key.to_string(),
                status: check.status.to_string(),
                message: check.message,
            })
            .collect(),
    }))
}

fn value_response(value: String, source: &ConfigValueSource) -> RuntimeConfigValueResponse {
    RuntimeConfigValueResponse {
        value,
        source: source.kind.to_string(),
        variable: source.variable.map(str::to_string),
    }
}

fn internal_error(
    code: &'static str,
    message: String,
) -> (StatusCode, Json<ApiError>) {
    tracing::warn!(code, message, "Profile admin request failed");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiError { code, message }),
    )
}
