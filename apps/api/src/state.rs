use std::{path::PathBuf, sync::Arc};

use chrono::{DateTime, Utc};
use doseark_application::{AppServices, StorageBackupManager, StorageDiagnostics};

use crate::config::ApiConfig;

#[derive(Clone)]
pub struct AppState {
    pub services: AppServices,
    pub storage_diagnostics: Arc<dyn StorageDiagnostics>,
    pub storage_backup: Option<Arc<dyn StorageBackupManager>>,
    pub started_at: DateTime<Utc>,
    pub backup_dir: PathBuf,
    pub max_backup_download_bytes: u64,
    pub config: ApiConfig,
}
