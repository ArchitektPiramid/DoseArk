//! Managed backup and backup download HTTP routes.

use std::{path::Path, time::SystemTime};

use axum::{
    body::Body,
    extract::{Path as AxumPath, State},
    http::{header, HeaderValue, StatusCode},
    response::Response,
    routing::get,
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use tokio::fs;

use crate::state::AppState;

use crate::routes::error::ApiError;

#[derive(Debug, Serialize)]
struct BackupListResponse {
    status: &'static str,
    directory: String,
    files: Vec<BackupFileResponse>,
    can_create: bool,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct BackupFileResponse {
    name: String,
    size_bytes: u64,
    modified_at: DateTime<Utc>,
    format: String,
    download_path: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/admin/backups", get(list_backups).post(create_backup))
        .route("/api/admin/backups/{file_name}/download", get(download_backup))
}

async fn list_backups(State(state): State<AppState>) -> Json<BackupListResponse> {
    let directory = state.backup_dir.display().to_string();
    let capabilities = state.storage_diagnostics.capabilities();

    if !capabilities.backups {
        return Json(BackupListResponse {
            status: "unsupported",
            directory,
            files: Vec::new(),
            can_create: false,
            error: Some("Backups are not supported by the active storage adapter.".to_string()),
        });
    }

    if let Err(error) = fs::create_dir_all(&state.backup_dir).await {
        tracing::error!(
            backup_dir = %state.backup_dir.display(),
            error = %error,
            "Backup directory could not be prepared"
        );

        return Json(BackupListResponse {
            status: "unavailable",
            directory,
            files: Vec::new(),
            can_create: false,
            error: Some("Backup directory could not be created or accessed.".to_string()),
        });
    }

    match collect_backups(&state.backup_dir).await {
        Ok(files) => Json(BackupListResponse {
            status: "ok",
            directory,
            files,
            can_create: capabilities.managed_backups && state.storage_backup.is_some(),
            error: None,
        }),
        Err(error) => {
            tracing::error!(
                backup_dir = %state.backup_dir.display(),
                error = %error,
                "Backup directory could not be read"
            );

            Json(BackupListResponse {
                status: "unavailable",
                directory,
                files: Vec::new(),
                can_create: false,
                error: Some("Backup directory could not be read.".to_string()),
            })
        }
    }
}

async fn create_backup(
    State(state): State<AppState>,
) -> Result<Json<BackupFileResponse>, (StatusCode, Json<ApiError>)> {
    let capabilities = state.storage_diagnostics.capabilities();
    if !capabilities.managed_backups {
        return Err(api_error(
            StatusCode::NOT_IMPLEMENTED,
            "BACKUP_CREATE_UNSUPPORTED",
            "The active storage adapter cannot create managed backups.",
        ));
    }

    let backup_manager = state.storage_backup.as_deref().ok_or_else(|| {
        api_error(
            StatusCode::NOT_IMPLEMENTED,
            "BACKUP_CREATE_UNSUPPORTED",
            "The active storage adapter cannot create managed backups.",
        )
    })?;

    fs::create_dir_all(&state.backup_dir).await.map_err(|error| {
        tracing::error!(backup_dir = %state.backup_dir.display(), error = %error, "Could not create backup directory");
        api_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "BACKUP_DIRECTORY_FAILED",
            "Backup directory could not be prepared.",
        )
    })?;

    let now = Utc::now();
    let file_name = format!(
        "doseark-{}-{}-{:03}.sqlite",
        backup_manager.engine().as_str(),
        now.format("%Y%m%d-%H%M%S"),
        now.timestamp_subsec_millis(),
    );
    let path = state.backup_dir.join(&file_name);

    backup_manager.create_backup(&path).await.map_err(|error| {
        tracing::error!(file_name = %file_name, error = %error, "Managed storage backup failed");
        api_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "BACKUP_CREATE_FAILED",
            "Storage backup could not be created.",
        )
    })?;

    tracing::info!(
        storage = backup_manager.engine().as_str(),
        file_name = %file_name,
        "Managed storage backup created"
    );

    let backup = backup_file_response(&path, &file_name).await.map_err(|error| {
        tracing::error!(file_name = %file_name, error = %error, "Could not inspect created backup");
        api_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "BACKUP_METADATA_FAILED",
            "Backup was created but its metadata could not be read.",
        )
    })?;

    Ok(Json(backup))
}

async fn download_backup(
    State(state): State<AppState>,
    AxumPath(file_name): AxumPath<String>,
) -> Result<Response<Body>, (StatusCode, Json<ApiError>)> {
    if !state.storage_diagnostics.capabilities().backups {
        return Err(api_error(
            StatusCode::NOT_IMPLEMENTED,
            "BACKUPS_UNSUPPORTED",
            "Backups are not supported by the active storage adapter.",
        ));
    }

    if !is_safe_file_name(&file_name) || !is_supported_backup_name(&file_name) {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "INVALID_BACKUP_FILE",
            "Invalid backup filename.",
        ));
    }

    let path = state.backup_dir.join(&file_name);
    let metadata = fs::metadata(&path).await.map_err(|_| {
        api_error(
            StatusCode::NOT_FOUND,
            "BACKUP_NOT_FOUND",
            "Backup file was not found.",
        )
    })?;

    if !metadata.is_file() {
        return Err(api_error(
            StatusCode::NOT_FOUND,
            "BACKUP_NOT_FOUND",
            "Backup file was not found.",
        ));
    }

    if metadata.len() > state.max_backup_download_bytes {
        return Err(api_error(
            StatusCode::PAYLOAD_TOO_LARGE,
            "BACKUP_TOO_LARGE",
            "Backup is larger than the configured in-memory download limit.",
        ));
    }

    let bytes = fs::read(&path).await.map_err(|error| {
        tracing::error!(file_name = %file_name, error = %error, "Could not read backup file");
        api_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "BACKUP_READ_FAILED",
            "Backup file could not be read.",
        )
    })?;

    let content_disposition = HeaderValue::try_from(format!(
        "attachment; filename=\"{}\"",
        file_name.replace('"', "")
    ))
    .map_err(|_| {
        api_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INVALID_DOWNLOAD_HEADER",
            "Could not prepare backup download.",
        )
    })?;

    let mut response = Response::new(Body::from(bytes));
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );
    response
        .headers_mut()
        .insert(header::CONTENT_DISPOSITION, content_disposition);
    response.headers_mut().insert(
        header::CONTENT_LENGTH,
        HeaderValue::try_from(metadata.len().to_string()).unwrap_or(HeaderValue::from_static("0")),
    );

    Ok(response)
}

async fn collect_backups(directory: &Path) -> std::io::Result<Vec<BackupFileResponse>> {
    let mut entries = fs::read_dir(directory).await?;
    let mut files = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        let file_name = entry.file_name().to_string_lossy().to_string();
        if !is_supported_backup_name(&file_name) {
            continue;
        }

        let metadata = match entry.metadata().await {
            Ok(metadata) if metadata.is_file() => metadata,
            Ok(_) => continue,
            Err(error) => {
                tracing::warn!(file_name = %file_name, error = %error, "Could not read backup metadata");
                continue;
            }
        };

        files.push(BackupFileResponse {
            format: backup_format(&file_name).to_string(),
            download_path: format!("/api/admin/backups/{file_name}/download"),
            name: file_name,
            size_bytes: metadata.len(),
            modified_at: metadata
                .modified()
                .unwrap_or(SystemTime::UNIX_EPOCH)
                .into(),
        });
    }

    files.sort_by(|left, right| right.modified_at.cmp(&left.modified_at));
    Ok(files)
}

async fn backup_file_response(path: &Path, file_name: &str) -> std::io::Result<BackupFileResponse> {
    let metadata = fs::metadata(path).await?;

    Ok(BackupFileResponse {
        name: file_name.to_string(),
        size_bytes: metadata.len(),
        modified_at: metadata
            .modified()
            .unwrap_or(SystemTime::UNIX_EPOCH)
            .into(),
        format: backup_format(file_name).to_string(),
        download_path: format!("/api/admin/backups/{file_name}/download"),
    })
}

fn is_safe_file_name(file_name: &str) -> bool {
    !file_name.is_empty()
        && Path::new(file_name).file_name().and_then(|name| name.to_str()) == Some(file_name)
        && !file_name.contains('/')
        && !file_name.contains('\\')
}

fn is_supported_backup_name(file_name: &str) -> bool {
    let lower = file_name.to_ascii_lowercase();
    lower.ends_with(".dump")
        || lower.ends_with(".dump.gz")
        || lower.ends_with(".sql")
        || lower.ends_with(".sql.gz")
        || lower.ends_with(".backup")
        || lower.ends_with(".sqlite")
        || lower.ends_with(".sqlite3")
        || lower.ends_with(".db")
}

fn backup_format(file_name: &str) -> &'static str {
    let lower = file_name.to_ascii_lowercase();
    if lower.ends_with(".sqlite") || lower.ends_with(".sqlite3") || lower.ends_with(".db") {
        "sqlite"
    } else if lower.ends_with(".dump.gz") || lower.ends_with(".sql.gz") {
        "gzip"
    } else if lower.ends_with(".dump") || lower.ends_with(".backup") {
        "postgres-custom"
    } else {
        "sql"
    }
}

fn api_error(
    status: StatusCode,
    code: &'static str,
    message: &str,
) -> (StatusCode, Json<ApiError>) {
    (
        status,
        Json(ApiError {
            code,
            message: message.to_string(),
        }),
    )
}
