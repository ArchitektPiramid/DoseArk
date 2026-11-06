//! File-backed runtime profile metadata and server/native-only credential resolution.
//!
//! Browser JavaScript must never read credential files. This crate is intended for
//! Rust hosts such as the HTTP API and future Tauri runtimes.

mod credentials;
mod profiles;

pub use credentials::{CredentialsResolver, ProfileCredentials};
pub use profiles::{load_profiles, ProfileConfig, ProfileDatabaseConfig, ProfilesFile};
