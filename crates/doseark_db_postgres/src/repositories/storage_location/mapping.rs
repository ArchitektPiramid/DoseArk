use chrono::{DateTime, Utc};
use doseark_domain::StorageLocation;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub(super) struct StorageLocationRow {
    pub(super) id: Uuid,

    pub(super) name: String,
    pub(super) description: Option<String>,

    pub(super) color: Option<String>,
    pub(super) icon: Option<String>,

    pub(super) sort_order: i32,
    pub(super) is_archived: bool,

    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
}

impl From<StorageLocationRow> for StorageLocation {
    fn from(row: StorageLocationRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            description: row.description,
            color: row.color,
            icon: row.icon,
            sort_order: row.sort_order,
            is_archived: row.is_archived,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
