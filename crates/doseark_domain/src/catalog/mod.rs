//! Catalog entities: brands, products, nutrients and per-serving composition.
//!
//! Types remain re-exported by `doseark_domain` for existing consumers.

pub mod brand;
pub mod nutrient;
pub mod product;
pub mod product_nutrient;

pub use self::brand::*;
pub use self::nutrient::*;
pub use self::product::*;
pub use self::product_nutrient::*;
