//! Profile validation results produced by the runtime administration API
//!
//! Each check carries a status and a human-readable explanation. The API
//! owns validation rules; these DTOs only define the serialized response.

use serde::Serialize;

/// Individual validation result for one profile requirement.
#[derive(Debug, Clone, Serialize)]
pub struct ProfileValidationCheckResponse {
    pub key: String,
    pub status: String,
    pub message: String,
}

/// Summary of the checks performed against a requested runtime profile.
#[derive(Debug, Clone, Serialize)]
pub struct ProfileValidationResponse {
    pub profile_id: String,
    pub profile_file: String,
    pub credentials_file: String,
    pub ok: bool,
    pub checks: Vec<ProfileValidationCheckResponse>,
}
