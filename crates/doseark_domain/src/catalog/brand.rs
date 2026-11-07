//! Product brands and the commands used to maintain the brand catalog.
//!
//! These records carry optional origin and contact details. Name validation
//! belongs to application services rather than these shared data structures.

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
