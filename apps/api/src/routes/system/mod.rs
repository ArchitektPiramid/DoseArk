//! Operational HTTP endpoints for health, backup, database and profile administration
//!
//! Handlers delegate domain operations to application services and preserve the
//! existing HTTP paths and response formats.

pub mod health;
pub mod backups;
pub mod database_admin;
pub mod profile_admin;
