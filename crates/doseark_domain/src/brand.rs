//! Marki produktów i dane wejściowe do zarządzania katalogiem

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A catalog brand shared by product records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brand {
    pub id: Uuid,
    pub name: String,
    pub country: Option<String>,
    pub website: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Data required to register a brand
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBrandInput {
    pub name: String,
    pub country: Option<String>,
    pub website: Option<String>,
    pub notes: Option<String>,
}

/// Editable brand attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBrandInput {
    pub name: String,
    pub country: Option<String>,
    pub website: Option<String>,
    pub notes: Option<String>,
}
