//! System endpoint contracts for health, storage diagnostics, runtime configuration,
//! and profile validation
//!
//! The named modules organize related responses while the re-exports preserve
//! existing `doseark_contracts::system::*` and crate-root imports.

pub mod database;
pub mod health;
pub mod profiles;
pub mod runtime;

pub use self::database::{
    DatabaseOverviewResponse, DatabaseTableStatsResponse, StorageCapabilitiesResponse,
};
pub use self::health::{DatabaseHealthResponse, HealthResponse};
pub use self::profiles::{ProfileValidationCheckResponse, ProfileValidationResponse};
pub use self::runtime::{
    RuntimeConfigValueResponse, RuntimeConfigurationResponse, RuntimeEnvironmentVariableResponse,
};
