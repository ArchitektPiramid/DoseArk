//! Shared HTTP error shape and application error mapping.
//!
//! Domain validation and missing records retain their original HTTP statuses.
//! Specialized admin routes keep their own error codes and messages.

use axum::{http::StatusCode, Json};
use doseark_application::ApplicationError;
use serde::Serialize;

/// Stable JSON error payload used by the current API routes.
#[derive(Debug, Serialize)]
pub(super) struct ApiError {
    pub(super) code: &'static str,
    pub(super) message: String,
}

/// Converts application errors without changing existing response bodies.
pub(super) fn map_application_error(error: ApplicationError) -> (StatusCode, Json<ApiError>) {
    let status = match error {
        ApplicationError::Validation(_) => StatusCode::BAD_REQUEST,
        ApplicationError::NotFound(_) => StatusCode::NOT_FOUND,
        ApplicationError::Repository(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };

    (
        status,
        Json(ApiError {
            code: "API_ERROR",
            message: error.to_string(),
        }),
    )
}

#[cfg(test)]
#[path = "../../tests/unit/http_error.rs"]
mod tests;
