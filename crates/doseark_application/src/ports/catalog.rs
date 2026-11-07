//! Porty katalogu produktów, marek i składników

use crate::ApplicationResult;
use async_trait::async_trait;
use doseark_domain::{
    Brand, CreateBrandInput, CreateNutrientInput, CreateProductInput, Nutrient, Product,
    ProductNutrient, SaveProductNutrientInput, UpdateBrandInput, UpdateNutrientInput,
    UpdateProductInput,
};
use uuid::Uuid;

/// Stores and retrieves product brands
#[async_trait]
pub trait BrandRepository: Send + Sync {
    async fn list_brands(&self) -> ApplicationResult<Vec<Brand>>;
    async fn create_brand(&self, input: CreateBrandInput) -> ApplicationResult<Brand>;
    async fn update_brand(&self, id: Uuid, input: UpdateBrandInput) -> ApplicationResult<Brand>;
}

/// Persists catalog product definitions
#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn list_products(&self) -> ApplicationResult<Vec<Product>>;
    async fn create_product(&self, input: CreateProductInput) -> ApplicationResult<Product>;
    async fn update_product(
        &self,
        id: Uuid,
        input: UpdateProductInput,
    ) -> ApplicationResult<Product>;
}

/// Manages the shared nutrient catalog
#[async_trait]
pub trait NutrientRepository: Send + Sync {
    async fn list_nutrients(&self) -> ApplicationResult<Vec<Nutrient>>;
    async fn create_nutrient(&self, input: CreateNutrientInput) -> ApplicationResult<Nutrient>;
    async fn update_nutrient(
        &self,
        id: Uuid,
        input: UpdateNutrientInput,
    ) -> ApplicationResult<Nutrient>;
}

/// Reads and replaces ingredient composition for a product
#[async_trait]
pub trait ProductNutrientRepository: Send + Sync {
    async fn list_product_nutrients(
        &self,
        product_id: Uuid,
    ) -> ApplicationResult<Vec<ProductNutrient>>;

    async fn replace_product_nutrients(
        &self,
        product_id: Uuid,
        items: Vec<SaveProductNutrientInput>,
    ) -> ApplicationResult<Vec<ProductNutrient>>;
}
