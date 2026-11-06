//! Shared DoseArk domain models.
//!
//! This crate is the single source of truth for Rust-side entities and domain inputs.
//! Keep transport DTOs in `doseark_contracts` and persistence-specific SQL rows in database adapters.

pub mod brand;
pub mod daily_intake_report;
pub mod dose_log;
pub mod dose_log_nutrient;
pub mod inventory_lot;
pub mod nutrient;
pub mod product;
pub mod product_nutrient;
pub mod stock_movement;
pub mod storage_location;

pub use brand::*;
pub use daily_intake_report::*;
pub use dose_log::*;
pub use dose_log_nutrient::*;
pub use inventory_lot::*;
pub use nutrient::*;
pub use product::*;
pub use product_nutrient::*;
pub use stock_movement::*;
pub use storage_location::*;
