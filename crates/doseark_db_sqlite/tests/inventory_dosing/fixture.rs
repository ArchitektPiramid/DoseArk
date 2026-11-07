//! Dane testowe SQLite dla scenariuszy dawek i magazynu

use std::path::PathBuf;

use chrono::Utc;
use doseark_application::InventoryDosingRepositoryBundle;
use doseark_db_sqlite::{create_sqlite_pool, run_migrations, SqliteStorage, SqliteStorageConfig};
use doseark_domain::{
    CreateBrandInput, CreateDoseLogInput, CreateInventoryLotInput, CreateNutrientInput,
    CreateProductInput, CreateStorageLocationInput, DosageForm, DoseLog, InventoryLotStatus,
    NutrientKind, ProductCategory, SaveProductNutrientInput, UpdateDoseLogInput,
    UpdateInventoryLotInput,
};
use sqlx::SqlitePool;
use uuid::Uuid;

/// A fresh SQLite database with one product, nutrient and stocked lot
pub(super) struct Fixture {
    path: PathBuf,
    pool: SqlitePool,
    pub repos: InventoryDosingRepositoryBundle,
    pub product_id: Uuid,
    pub lot_id: Uuid,
}

impl Fixture {
    /// Builds an isolated database; no user data is read or changed
    pub async fn new() -> Self {
        let path =
            std::env::temp_dir().join(format!("doseark-sqlite-dosing-{}.db", Uuid::new_v4()));
        let pool = create_sqlite_pool(&SqliteStorageConfig::new(&path))
            .await
            .expect("SQLite pool should open");
        run_migrations(&pool)
            .await
            .expect("SQLite migrations should apply");

        let storage = SqliteStorage::new(pool.clone(), path.display().to_string());
        let catalog = storage.catalog_repositories();
        let repos = storage.inventory_dosing_repositories();

        let brand = catalog
            .brands
            .create_brand(CreateBrandInput {
                name: "Dosing test brand".into(),
                country: Some("PL".into()),
                website: None,
                notes: None,
            })
            .await
            .expect("brand should be created");

        let nutrient = catalog
            .nutrients
            .create_nutrient(CreateNutrientInput {
                canonical_name: "sqlite_test_magnesium".into(),
                display_name: "SQLite Test Magnesium".into(),
                kind: NutrientKind::Mineral,
                default_unit: "mg".into(),
                notes: None,
                reference_daily_amount: Some(375.0),
                reference_daily_unit: Some("mg".into()),
                reference_source: Some("test".into()),
                reference_notes: None,
            })
            .await
            .expect("nutrient should be created");

        let product = catalog
            .products
            .create_product(CreateProductInput {
                brand_id: Some(brand.id),
                name: "DoseArk dosing test product".into(),
                category: ProductCategory::Supplement,
                dosage_form: DosageForm::Tablet,
                serving_unit: "tablet".into(),
                serving_size_units: 2.0,
                unit_weight_mg: Some(500.0),
                barcode: None,
                description: None,
                ui_color: None,
                ui_icon: None,
                user_importance: None,
            })
            .await
            .expect("product should be created");

        catalog
            .product_nutrients
            .replace_product_nutrients(
                product.id,
                vec![SaveProductNutrientInput {
                    nutrient_id: nutrient.id,
                    amount_per_serving: 100.0,
                    unit: "mg".into(),
                    label_daily_value_percent: None,
                    label_reference_amount: None,
                    label_reference_unit: None,
                    source: "manual".into(),
                    notes: None,
                }],
            )
            .await
            .expect("composition should be saved");

        let location = catalog
            .storage_locations
            .create_storage_location(CreateStorageLocationInput {
                name: "Kitchen".into(),
                description: None,
                color: None,
                icon: None,
                sort_order: None,
            })
            .await
            .expect("location should be created");

        let lot = repos
            .inventory_lots
            .create_inventory_lot(CreateInventoryLotInput {
                product_id: product.id,
                lot_number: Some("LOT-001".into()),
                storage_location: None,
                storage_location_id: Some(location.id),
                units_initial: 10.0,
                units_remaining: None,
                purchase_date: None,
                opened_date: None,
                expires_at: None,
                price_amount: Some(20.0),
                price_currency: Some("PLN".into()),
                status: Some(InventoryLotStatus::Active),
                notes: None,
            })
            .await
            .expect("lot should be created");
        assert_eq!(lot.storage_location.as_deref(), Some("Kitchen"));

        repos
            .inventory_lots
            .update_inventory_lot(
                lot.id,
                UpdateInventoryLotInput {
                    product_id: product.id,
                    lot_number: lot.lot_number,
                    storage_location: None,
                    storage_location_id: Some(location.id),
                    units_initial: 10.0,
                    units_remaining: 9.0,
                    purchase_date: None,
                    opened_date: None,
                    expires_at: None,
                    price_amount: Some(20.0),
                    price_currency: Some("PLN".into()),
                    status: InventoryLotStatus::Active,
                    notes: None,
                },
            )
            .await
            .expect("lot should be adjusted");

        Self {
            path,
            pool,
            repos,
            product_id: product.id,
            lot_id: lot.id,
        }
    }

    /// input contains tablet count and the expected fraction of a two-tablet serving
    pub fn dose(&self, units: f64, multiplier: f64, with_lot: bool) -> CreateDoseLogInput {
        CreateDoseLogInput {
            product_id: self.product_id,
            inventory_lot_id: with_lot.then_some(self.lot_id),
            taken_at: Utc::now(),
            units_taken: units,
            unit: "tablet".into(),
            serving_multiplier: multiplier,
            reason: None,
            notes: Some("test dose".into()),
        }
    }

    pub fn edit(&self, dose: &DoseLog, units: f64, multiplier: f64) -> UpdateDoseLogInput {
        UpdateDoseLogInput {
            product_id: self.product_id,
            inventory_lot_id: dose.inventory_lot_id,
            taken_at: dose.taken_at.clone(),
            units_taken: units,
            unit: "tablet".into(),
            serving_multiplier: multiplier,
            reason: None,
            notes: Some("edited dose".into()),
        }
    }

    pub async fn remaining(&self) -> f64 {
        let lots = self
            .repos
            .inventory_lots
            .list_inventory_lots()
            .await
            .unwrap();
        lots.iter()
            .find(|lot| lot.id == self.lot_id)
            .unwrap()
            .units_remaining
    }

    /// Closes SQLite before removing the temporary database and WAL files
    pub async fn close(self) {
        self.pool.close().await;
        for suffix in ["db", "db-shm", "db-wal"] {
            let path = if suffix == "db" {
                self.path.clone()
            } else {
                self.path.with_extension(suffix)
            };
            let _ = std::fs::remove_file(path);
        }
    }
}
