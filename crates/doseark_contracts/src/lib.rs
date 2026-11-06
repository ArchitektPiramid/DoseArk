//! Shared transport contracts for DoseArk HTTP and Tauri IPC boundaries.
//!
//! Keep domain rules and persistence models out of this crate. Types here are
//! intentionally serialization-friendly request/response shapes.

pub mod system;

pub use system::*;
