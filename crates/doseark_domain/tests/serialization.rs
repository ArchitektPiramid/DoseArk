//! JSON compatibility tests for public domain models.
//!
//! Each module covers an independently useful part of the external data shape.

#[path = "serialization/enums.rs"]
mod enums;
#[path = "serialization/history.rs"]
mod history;
#[path = "serialization/inputs.rs"]
mod inputs;

#[path = "serialization/records.rs"]
mod records;
