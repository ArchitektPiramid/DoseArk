//! Catalog use cases for brands and products.
use crate::{ApplicationError, ApplicationResult, BrandRepository, ProductRepository};
use doseark_domain::{Brand, CreateBrandInput, Product, CreateProductInput, UpdateBrandInput, UpdateProductInput};
use std::sync::Arc;
use uuid::Uuid;

/// Validates brand names and delegates catalog operations.
#[derive(Clone)]
pub struct BrandService {
    repo: Arc<dyn BrandRepository>,
}

impl BrandService {
    pub fn new(repo: Arc<dyn BrandRepository>) -> Self {
        Self { repo }
    }

    pub async fn list_brands(&self) -> ApplicationResult<Vec<Brand>> {
        self.repo.list_brands().await
    }

    pub async fn create_brand(&self, input: CreateBrandInput) -> ApplicationResult<Brand> {
        validate_brand_name(&input.name)?;
        self.repo.create_brand(input).await
    }

    pub async fn update_brand(&self, id: Uuid, input: UpdateBrandInput) -> ApplicationResult<Brand> {
        validate_brand_name(&input.name)?;
        self.repo.update_brand(id, input).await
    }
}

// == Product Service ==
/// Manages products and validates serving configuration.
#[derive(Clone)]
pub struct ProductService {
    repo: Arc<dyn ProductRepository>,
}

impl ProductService {
    pub fn new(repo: Arc<dyn ProductRepository>) -> Self {
        Self { repo }
    }

    pub async fn list_products(&self) -> ApplicationResult<Vec<Product>> {
        self.repo.list_products().await
    }

    pub async fn create_product(&self, input: CreateProductInput) -> ApplicationResult<Product> {
        validate_product_name(&input.name)?;
        validate_serving_size(input.serving_size_units)?;

        self.repo.create_product(input).await
    }

    pub async fn update_product(&self, id: Uuid, input: UpdateProductInput) -> ApplicationResult<Product> {
        validate_product_name(&input.name)?;
        validate_serving_size(input.serving_size_units)?;

        self.repo.update_product(id, input).await
    }
}

/// Rejects an empty brand name.
fn validate_brand_name(name: &str) -> ApplicationResult<()> {
    if name.trim().is_empty() {
        return Err(ApplicationError::Validation("Brand name is required".into()));
    }

    Ok(())
}

/// Rejects an empty product name.
fn validate_product_name(name: &str) -> ApplicationResult<()> {
    if name.trim().is_empty() {
        return Err(ApplicationError::Validation("Product name is required".into()));
    }

    Ok(())
}

/// Rejects zero or negative serving sizes.
fn validate_serving_size(value: f64) -> ApplicationResult<()> {
    if value <= 0.0 {
        return Err(ApplicationError::Validation(
            "Serving size must be greater than zero".into(),
        ));
    }

    Ok(())
}
