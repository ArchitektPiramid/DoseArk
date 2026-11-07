//! Shared domain entities and input contracts for DoseArk.
//!
//! Models are organized by catalog, dosing, inventory and reporting concerns.
//! This crate defines data shapes, not database queries or transport-specific DTOs.
//! The feature modules and former top-level type/module paths remain public to
//! avoid changing imports in the API, application services or storage adapters.

pub mod catalog;
pub mod dosing;
pub mod inventory;
pub mod reports;

// Keep `doseark_domain::Product` and `doseark_domain::product::Product` working
pub use self::catalog::*;
pub use self::dosing::*;
pub use self::inventory::*;
pub use self::reports::*;
