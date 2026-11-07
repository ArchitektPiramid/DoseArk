//! Repozytoria testowe zapisujące wywołania serwisów bez użycia bazy danych

use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use doseark_application::{
    ApplicationResult, BrandRepository, DailyIntakeReportRepository, DoseLogNutrientRepository,
    DoseLogRepository, InventoryLotRepository, NutrientRepository, ProductNutrientRepository,
    ProductRepository, RepositoryBundle, StockMovementRepository, StorageLocationRepository,
};
use doseark_domain::*;
use uuid::Uuid;

/// Captures service calls and the last requested dose page
#[derive(Default)]
pub struct RepositoryProbe {
    calls: Mutex<Vec<&'static str>>,
    page: Mutex<Option<(i64, i64, Option<Uuid>)>>,
}

impl RepositoryProbe {
    /// Supplies the same probe through all repository interfaces
    pub fn bundle(self: &Arc<Self>) -> RepositoryBundle {
        RepositoryBundle {
            brands: self.clone(),
            products: self.clone(),
            nutrients: self.clone(),
            product_nutrients: self.clone(),
            inventory_lots: self.clone(),
            stock_movements: self.clone(),
            dose_logs: self.clone(),
            dose_log_nutrients: self.clone(),
            reports: self.clone(),
            storage_locations: self.clone(),
        }
    }

    fn record(&self, name: &'static str) {
        self.calls.lock().unwrap().push(name);
    }

    pub fn calls(&self) -> Vec<&'static str> {
        self.calls.lock().unwrap().clone()
    }

    pub fn last_page(&self) -> Option<(i64, i64, Option<Uuid>)> {
        *self.page.lock().unwrap()
    }
}

#[async_trait]
impl BrandRepository for RepositoryProbe {
    async fn list_brands(&self) -> ApplicationResult<Vec<Brand>> {
        self.record("brands");
        Ok(Vec::new())
    }

    async fn create_brand(&self, _input: CreateBrandInput) -> ApplicationResult<Brand> {
        panic!("unexpected brand write")
    }

    async fn update_brand(&self, _id: Uuid, _input: UpdateBrandInput) -> ApplicationResult<Brand> {
        panic!("unexpected brand update")
    }
}

#[async_trait]
impl ProductRepository for RepositoryProbe {
    async fn list_products(&self) -> ApplicationResult<Vec<Product>> {
        self.record("products");
        Ok(Vec::new())
    }

    async fn create_product(&self, _input: CreateProductInput) -> ApplicationResult<Product> {
        panic!("unexpected product write")
    }

    async fn update_product(&self, _id: Uuid, _input: UpdateProductInput) -> ApplicationResult<Product> {
        panic!("unexpected product update")
    }
}

#[async_trait]
impl NutrientRepository for RepositoryProbe {
    async fn list_nutrients(&self) -> ApplicationResult<Vec<Nutrient>> {
        self.record("nutrients");
        Ok(Vec::new())
    }

    async fn create_nutrient(&self, _input: CreateNutrientInput) -> ApplicationResult<Nutrient> {
        panic!("unexpected nutrient write")
    }

    async fn update_nutrient(&self, _id: Uuid, _input: UpdateNutrientInput) -> ApplicationResult<Nutrient> {
        panic!("unexpected nutrient update")
    }
}

#[async_trait]
impl ProductNutrientRepository for RepositoryProbe {
    async fn list_product_nutrients(&self, _product_id: Uuid) -> ApplicationResult<Vec<ProductNutrient>> {
        self.record("composition");
        Ok(Vec::new())
    }

    async fn replace_product_nutrients(
        &self,
        _product_id: Uuid,
        _items: Vec<SaveProductNutrientInput>,
    ) -> ApplicationResult<Vec<ProductNutrient>> {
        panic!("unexpected composition write")
    }
}

#[async_trait]
impl InventoryLotRepository for RepositoryProbe {
    async fn list_inventory_lots(&self) -> ApplicationResult<Vec<InventoryLot>> {
        self.record("lots");
        Ok(Vec::new())
    }

    async fn create_inventory_lot(&self, _input: CreateInventoryLotInput) -> ApplicationResult<InventoryLot> {
        panic!("unexpected lot write")
    }

    async fn update_inventory_lot(
        &self,
        _id: Uuid,
        _input: UpdateInventoryLotInput,
    ) -> ApplicationResult<InventoryLot> {
        panic!("unexpected lot update")
    }
}

#[async_trait]
impl StockMovementRepository for RepositoryProbe {
    async fn list_stock_movements(&self) -> ApplicationResult<Vec<StockMovement>> {
        self.record("movements");
        Ok(Vec::new())
    }

    async fn list_stock_movements_for_lot(&self, _inventory_lot_id: Uuid) -> ApplicationResult<Vec<StockMovement>> {
        self.record("movements_for_lot");
        Ok(Vec::new())
    }
}

#[async_trait]
impl DoseLogRepository for RepositoryProbe {
    async fn list_dose_logs(&self, input: DoseLogListInput) -> ApplicationResult<DoseLogPage> {
        self.record("doses");
        *self.page.lock().unwrap() = Some((input.limit, input.offset, input.product_id));
        Ok(DoseLogPage {
            items: Vec::new(),
            total: 0,
            limit: input.limit,
            offset: input.offset,
        })
    }

    async fn create_dose_log(&self, _input: CreateDoseLogInput) -> ApplicationResult<DoseLog> {
        panic!("unexpected dose write")
    }

    async fn update_dose_log(&self, _id: Uuid, _input: UpdateDoseLogInput) -> ApplicationResult<DoseLog> {
        panic!("unexpected dose update")
    }

    async fn cancel_dose_log(&self, _id: Uuid) -> ApplicationResult<DoseLog> {
        panic!("unexpected dose cancellation")
    }
}

#[async_trait]
impl DoseLogNutrientRepository for RepositoryProbe {
    async fn list_dose_log_nutrients(&self, _dose_log_id: Uuid) -> ApplicationResult<Vec<DoseLogNutrient>> {
        self.record("dose_nutrients");
        Ok(Vec::new())
    }
}

#[async_trait]
impl DailyIntakeReportRepository for RepositoryProbe {
    async fn get_daily_intake_report(&self, input: DailyIntakeReportInput) -> ApplicationResult<DailyIntakeReport> {
        self.record("reports");
        Ok(DailyIntakeReport {
            date: input.date,
            start_at: input.start_at,
            end_at: input.end_at,
            rows: Vec::new(),
            dose_logs: Vec::new(),
        })
    }

    async fn get_nutrient_trend_report(
        &self,
        input: NutrientTrendReportInput,
    ) -> ApplicationResult<NutrientTrendReport> {
        self.record("trends");
        Ok(NutrientTrendReport {
            start_date: input.start_date,
            end_date: input.end_date,
            start_at: input.start_at,
            end_at: input.end_at,
            selected_nutrient_id: input.nutrient_id,
            selected_product_id: input.product_id,
            selected_unit: input.unit,
            day_start_hour: input.day_start_hour,
            nutrients: Vec::new(),
            days: Vec::new(),
            events: Vec::new(),
        })
    }
}

#[async_trait]
impl StorageLocationRepository for RepositoryProbe {
    async fn list_storage_locations(&self, _include_archived: bool) -> ApplicationResult<Vec<StorageLocation>> {
        self.record("locations");
        Ok(Vec::new())
    }

    async fn create_storage_location(&self, _input: CreateStorageLocationInput) -> ApplicationResult<StorageLocation> {
        panic!("unexpected location write")
    }

    async fn update_storage_location(
        &self,
        _id: Uuid,
        _input: UpdateStorageLocationInput,
    ) -> ApplicationResult<StorageLocation> {
        panic!("unexpected location update")
    }

    async fn archive_storage_location(&self, _id: Uuid) -> ApplicationResult<StorageLocation> {
        panic!("unexpected location archive")
    }
}
