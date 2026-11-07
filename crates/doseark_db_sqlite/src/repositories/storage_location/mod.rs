//! Repozytorium lokalizacji magazynowych wraz z ich archiwizacją

mod create;
mod mapping;
mod read;
mod update;

use async_trait::async_trait;
use doseark_application::{ApplicationResult, StorageLocationRepository};
use doseark_domain::{CreateStorageLocationInput, StorageLocation, UpdateStorageLocationInput};
use sqlx::SqlitePool;
use uuid::Uuid;

/// SQLite-backed storage location catalog
#[derive(Clone)]
pub struct SqliteStorageLocationRepository {
    pool: SqlitePool,
}

impl SqliteStorageLocationRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StorageLocationRepository for SqliteStorageLocationRepository {
    async fn list_storage_locations(
        &self,
        include_archived: bool,
    ) -> ApplicationResult<Vec<StorageLocation>> {
        read::list_storage_locations(&self.pool, include_archived).await
    }

    async fn create_storage_location(
        &self,
        input: CreateStorageLocationInput,
    ) -> ApplicationResult<StorageLocation> {
        create::create_storage_location(&self.pool, input).await
    }

    async fn update_storage_location(
        &self,
        id: Uuid,
        input: UpdateStorageLocationInput,
    ) -> ApplicationResult<StorageLocation> {
        update::update_storage_location(&self.pool, id, input).await
    }

    async fn archive_storage_location(&self, id: Uuid) -> ApplicationResult<StorageLocation> {
        update::archive_storage_location(&self.pool, id).await
    }
}
