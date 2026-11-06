use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageLocation {
    pub id: Uuid,

    pub name: String,
    pub description: Option<String>,

    pub color: Option<String>,
    pub icon: Option<String>,

    pub sort_order: i32,
    pub is_archived: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStorageLocationInput {
    pub name: String,
    pub description: Option<String>,

    pub color: Option<String>,
    pub icon: Option<String>,

    pub sort_order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateStorageLocationInput {
    pub name: String,
    pub description: Option<String>,

    pub color: Option<String>,
    pub icon: Option<String>,

    pub sort_order: Option<i32>,
    pub is_archived: bool,
}
