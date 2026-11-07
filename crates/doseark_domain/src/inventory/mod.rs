//! Inventory lots, stock movements and named storage locations.
//!
//! Types remain re-exported by `doseark_domain` for existing consumers.

pub mod inventory_lot;
pub mod stock_movement;
pub mod storage_location;

pub use self::inventory_lot::*;
pub use self::stock_movement::*;
pub use self::storage_location::*;
