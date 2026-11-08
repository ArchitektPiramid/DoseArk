//! Compile-time API and dependency versions published in system health.

pub const APP_NAME: &str = "DoseArk";
pub const BACKEND_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CHANNEL: &str = env!("DOSEARK_BUILD_CHANNEL");
pub const DOMAIN_VERSION: &str = env!("DOSEARK_DOMAIN_VERSION");
pub const APPLICATION_VERSION: &str = env!("DOSEARK_APPLICATION_VERSION");
pub const CONTRACTS_VERSION: &str = env!("DOSEARK_CONTRACTS_VERSION");
pub const DB_POSTGRES_VERSION: &str = env!("DOSEARK_DB_POSTGRES_VERSION");
pub const DB_SQLITE_VERSION: &str = env!("DOSEARK_DB_SQLITE_VERSION");
