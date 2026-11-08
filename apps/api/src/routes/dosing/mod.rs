//! Dosing HTTP endpoints for intake history and saved nutrient snapshots
//!
//! Handlers delegate domain operations to application services and preserve the
//! existing HTTP paths and response formats.

pub mod dose_log_nutrients;
pub mod dose_logs;
