//! HTTP endpoints grouped by application feature.
//!
//! Each group owns its Axum routers. Existing `routes::<endpoint>` paths remain
//! available through re-exports so route registration does not need to change.

mod error;

pub mod catalog;
pub mod dosing;
pub mod inventory;
pub mod reports;
pub mod system;

pub use self::catalog::{brands, nutrients, product_nutrients, products};
pub use self::dosing::{dose_log_nutrients, dose_logs};
pub use self::inventory::{inventory_lots, stock_movements, storage_locations};
pub use self::system::{backups, database_admin, health, profile_admin};
