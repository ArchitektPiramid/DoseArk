//! Transport contracts shared by the DoseArk HTTP API and future Tauri IPC endpoints
//!
//! This crate defines serialization-only response shapes. Domain rules, storage
//! models, and security-sensitive configuration resolution belong elsewhere.

pub mod system;

// Preserve root-level imports used by the API, e.g. doseark_contracts::HealthResponse
pub use system::*;
