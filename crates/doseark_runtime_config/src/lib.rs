//! Runtime profile metadata and credential lookup for Rust-only DoseArk hosts.
//!
//! Profiles describe API connections and storage locations; credential references
//! resolve separately from a local JSON file. Browser clients must never read that
//! file or receive its secret values through transport responses.

mod credentials;
mod profiles;

pub use credentials::{CredentialsResolver, ProfileCredentials};
pub use profiles::{load_profiles, ProfileConfig, ProfileDatabaseConfig, ProfilesFile};
