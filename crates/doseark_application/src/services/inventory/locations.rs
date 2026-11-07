//! Zarządzanie lokalizacjami przechowywania produktów

use crate::{ApplicationError, ApplicationResult, StorageLocationRepository};
use doseark_domain::{CreateStorageLocationInput, StorageLocation, UpdateStorageLocationInput};
use std::sync::Arc;
use uuid::Uuid;

/// Manages and archives locations used by inventory
#[derive(Clone)]
pub struct StorageLocationService {
    repo: Arc<dyn StorageLocationRepository>,
}

impl StorageLocationService {
    pub fn new(repo: Arc<dyn StorageLocationRepository>) -> Self {
        Self { repo }
    }

    pub async fn list_storage_locations(
        &self,
        include_archived: bool,
    ) -> ApplicationResult<Vec<StorageLocation>> {
        self.repo.list_storage_locations(include_archived).await
    }

    pub async fn create_storage_location(
        &self,
        input: CreateStorageLocationInput,
    ) -> ApplicationResult<StorageLocation> {
        validate_storage_location_name(&input.name)?;
        self.repo.create_storage_location(input).await
    }

    pub async fn update_storage_location(
        &self,
        id: Uuid,
        input: UpdateStorageLocationInput,
    ) -> ApplicationResult<StorageLocation> {
        validate_storage_location_name(&input.name)?;
        self.repo.update_storage_location(id, input).await
    }

    pub async fn archive_storage_location(&self, id: Uuid) -> ApplicationResult<StorageLocation> {
        self.repo.archive_storage_location(id).await
    }
}

/// Requires a non-empty location name.
fn validate_storage_location_name(name: &str) -> ApplicationResult<()> {
    if name.trim().is_empty() {
        return Err(ApplicationError::Validation(
            "Storage location name is required".into(),
        ));
    }

    Ok(())
}
