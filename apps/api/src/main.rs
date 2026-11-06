mod config;
mod database;
mod logging;
mod profile_validation;
mod routes;
mod state;
mod storage_runtime;
mod version;

use std::{net::SocketAddr, path::PathBuf, process::ExitCode};

use anyhow::Context;
use axum::Router;
use chrono::Utc;
use config::ApiConfig;
use state::AppState;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

/// Loads the optional .env file before logging and the API start.
fn load_environment() -> anyhow::Result<Option<PathBuf>> {
    if let Some(env_file) = config::env_or_legacy("DOSEARK_ENV_FILE", "MEDTAKER_ENV_FILE") {
        let path = PathBuf::from(env_file);

        dotenvy::from_path(&path)
            .map_err(|error| anyhow::anyhow!(
                "Could not load environment file '{}': {error}",
                path.display()
            ))?;

        return Ok(Some(path));
    }

    match dotenvy::dotenv() {
        Ok(path) => Ok(Some(path)),
        Err(error) if error.not_found() => Ok(None),
        Err(error) => Err(error.into()),
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    let env_file = match load_environment() {
        Ok(path) => path,

        Err(error) => {
            eprintln!("Failed to load environment configuration: {error}");
            return ExitCode::FAILURE;
        }
    };

    let _log_guard = match logging::init_logging() {
        Ok(guard) => guard,
        Err(error) => {
            eprintln!("Failed to initialize logging: {error:#}");
            return ExitCode::FAILURE;
        }
    };

    match &env_file {
        Some(path) => {
            tracing::debug!(
                path = %path.display(),
                "Environment file loaded"
            );
        }

        None => {
            tracing::debug!(
                "No .env file loaded; using process environment only"
            );
        }
    }

    match run(env_file).await {
        Ok(()) => ExitCode::SUCCESS,

        Err(error) => {
            tracing::error!(
                error = %error,
                "DoseArk API startup failed"
            );

            ExitCode::FAILURE
        }
    }
}

/// Initializes the selected database and starts the local HTTP API.
async fn run(env_file: Option<PathBuf>) -> anyhow::Result<()> {
    let config = ApiConfig::from_env(env_file)?;

    tracing::info!(
        backend_version = version::BACKEND_VERSION,
        bind = %config.bind_addr,
        "Starting DoseArk API"
    );
    tracing::debug!(
        app_name = version::APP_NAME,
        storage_backend = config.storage_backend.as_str(),
        database = %config.database_target(),
        backup_dir = %config.backup_dir.display(),
        "Runtime configuration loaded"
    );

    let storage = storage_runtime::initialize_storage(&config).await?;

    if storage.backup.is_some() {
        tokio::fs::create_dir_all(&config.backup_dir)
            .await
            .with_context(|| format!("could not create backup directory {}", config.backup_dir.display()))?;
        tracing::debug!(backup_dir = %config.backup_dir.display(), "Backup directory ready");
    }

    let state = AppState {
        services: storage.services,
        storage_diagnostics: storage.diagnostics,
        storage_backup: storage.backup,
        started_at: Utc::now(),
        backup_dir: config.backup_dir.clone(),
        max_backup_download_bytes: config.max_backup_download_bytes,
        config: config.clone(),
    };

    let health_routes = routes::health::router().layer(
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().level(Level::TRACE))
            .on_request(DefaultOnRequest::new().level(Level::TRACE))
            .on_response(DefaultOnResponse::new().level(Level::TRACE)),
    );

    let application_routes = Router::new()
        .merge(routes::brands::router())
        .merge(routes::products::router())
        .merge(routes::nutrients::router())
        .merge(routes::product_nutrients::router())
        .merge(routes::inventory_lots::router())
        .merge(routes::stock_movements::router())
        .merge(routes::dose_logs::router())
        .merge(routes::dose_log_nutrients::router())
        .merge(routes::reports::router())
        .merge(routes::storage_locations::router())
        .merge(routes::backups::router())
        .merge(routes::database_admin::router())
        .merge(routes::profile_admin::router())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        );

    let app = Router::new()
        .merge(health_routes)
        .merge(application_routes)
        .with_state(state)
        .layer(CorsLayer::permissive());

    let addr: SocketAddr = config
        .bind_addr
        .parse()
        .with_context(|| format!("invalid DOSEARK_API_BIND value: {}", config.bind_addr))?;
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("could not bind API listener to {addr}"))?;

    tracing::info!(address = %format!("http://{addr}"), "DoseArk API listening");

    axum::serve(listener, app)
        .await
        .context("HTTP server stopped unexpectedly")?;

    Ok(())
}
