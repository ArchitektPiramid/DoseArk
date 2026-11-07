//! Serwisy katalogu składników i składu produktów

mod catalog;
mod composition;

pub use self::catalog::NutrientService;
pub use self::composition::ProductCompositionService;
