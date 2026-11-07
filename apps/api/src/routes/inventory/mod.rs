//! Inventory HTTP endpoints for lots, locations, and stock movements
//!
//! Handlers delegate domain operations to application services and preserve the
//! existing HTTP paths and response formats.

pub mod inventory_lots;
pub mod stock_movements;
pub mod storage_locations;
