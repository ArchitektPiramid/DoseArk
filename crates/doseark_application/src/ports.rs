//! Persistence ports implemented by PostgreSQL and SQLite infrastructure adapters.

use async_trait::async_trait;
use doseark_domain::{
    Brand, CreateBrandInput, CreateDoseLogInput, CreateInventoryLotInput, CreateNutrientInput,
    CreateProductInput, CreateStorageLocationInput, DailyIntakeReport, DailyIntakeReportInput,
    DoseLog, DoseLogListInput, DoseLogNutrient, DoseLogPage, InventoryLot, Nutrient,
    NutrientTrendReport, NutrientTrendReportInput, Product, ProductNutrient, SaveProductNutrientInput, StockMovement,
    StorageLocation, UpdateBrandInput, UpdateDoseLogInput, UpdateInventoryLotInput,
    UpdateNutrientInput, UpdateProductInput, UpdateStorageLocationInput,
};
use uuid::Uuid;

use crate::ApplicationResult;

#[async_trait]
pub trait BrandRepository: Send + Sync {
    async fn list_brands(&self) -> ApplicationResult<Vec<Brand>>;
    async fn create_brand(&self, input: CreateBrandInput) -> ApplicationResult<Brand>;
    async fn update_brand(&self, id: Uuid, input: UpdateBrandInput) -> ApplicationResult<Brand>;
}

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn list_products(&self) -> ApplicationResult<Vec<Product>>;
    async fn create_product(&self, input: CreateProductInput) -> ApplicationResult<Product>;
    async fn update_product(&self, id: Uuid, input: UpdateProductInput) -> ApplicationResult<Product>;
}

#[async_trait]
pub trait NutrientRepository: Send + Sync {
    async fn list_nutrients(&self) -> ApplicationResult<Vec<Nutrient>>;
    async fn create_nutrient(&self, input: CreateNutrientInput) -> ApplicationResult<Nutrient>;
    async fn update_nutrient(&self, id: Uuid, input: UpdateNutrientInput) -> ApplicationResult<Nutrient>;
}

#[async_trait]
pub trait ProductNutrientRepository: Send + Sync {
    async fn list_product_nutrients(&self, product_id: Uuid) -> ApplicationResult<Vec<ProductNutrient>>;

    async fn replace_product_nutrients(
        &self,
        product_id: Uuid,
        items: Vec<SaveProductNutrientInput>,
    ) -> ApplicationResult<Vec<ProductNutrient>>;
}

#[async_trait]
pub trait InventoryLotRepository: Send + Sync {
    async fn list_inventory_lots(&self) -> ApplicationResult<Vec<InventoryLot>>;

    async fn create_inventory_lot(
        &self,
        input: CreateInventoryLotInput,
    ) -> ApplicationResult<InventoryLot>;

    async fn update_inventory_lot(
        &self,
        id: Uuid,
        input: UpdateInventoryLotInput,
    ) -> ApplicationResult<InventoryLot>;
}

#[async_trait]
pub trait StockMovementRepository: Send + Sync {
    async fn list_stock_movements(&self) -> ApplicationResult<Vec<StockMovement>>;

    async fn list_stock_movements_for_lot(
        &self,
        inventory_lot_id: Uuid,
    ) -> ApplicationResult<Vec<StockMovement>>;
}

#[async_trait]
pub trait DoseLogRepository: Send + Sync {
    async fn list_dose_logs(&self, input: DoseLogListInput) -> ApplicationResult<DoseLogPage>;

    async fn create_dose_log(
        &self,
        input: CreateDoseLogInput,
    ) -> ApplicationResult<DoseLog>;

    async fn update_dose_log(
        &self,
        id: Uuid,
        input: UpdateDoseLogInput,
    ) -> ApplicationResult<DoseLog>;

    async fn cancel_dose_log(&self, id: Uuid) -> ApplicationResult<DoseLog>;
}

#[async_trait]
pub trait DoseLogNutrientRepository: Send + Sync {
    async fn list_dose_log_nutrients(
        &self,
        dose_log_id: Uuid,
    ) -> ApplicationResult<Vec<DoseLogNutrient>>;
}

#[async_trait]
pub trait DailyIntakeReportRepository: Send + Sync {
    async fn get_daily_intake_report(
        &self,
        input: DailyIntakeReportInput,
    ) -> ApplicationResult<DailyIntakeReport>;

    async fn get_nutrient_trend_report(
        &self,
        input: NutrientTrendReportInput,
    ) -> ApplicationResult<NutrientTrendReport>;
}


#[async_trait]
pub trait StorageLocationRepository: Send + Sync {
    async fn list_storage_locations(
        &self,
        include_archived: bool,
    ) -> ApplicationResult<Vec<StorageLocation>>;

    async fn create_storage_location(
        &self,
        input: CreateStorageLocationInput,
    ) -> ApplicationResult<StorageLocation>;

    async fn update_storage_location(
        &self,
        id: Uuid,
        input: UpdateStorageLocationInput,
    ) -> ApplicationResult<StorageLocation>;

    async fn archive_storage_location(
        &self,
        id: Uuid,
    ) -> ApplicationResult<StorageLocation>;
}
