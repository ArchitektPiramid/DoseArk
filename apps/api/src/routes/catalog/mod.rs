//! Catalog HTTP endpoints for brands, products, and nutrient definitions
//!
//! Handlers delegate domain operations to application services and preserve the
//! existing HTTP paths and response formats.

pub mod brands;
pub mod nutrients;
pub mod product_nutrients;
pub mod products;
