//! Stable wire names for enum variants shared with API clients and database adapters.

use doseark_domain::{
    DosageForm, DoseLogStatus, InventoryLotStatus, NutrientKind, ProductCategory,
    ProductUserImportance, StockMovementType,
};
use serde::{de::DeserializeOwned, Serialize};

/// Checks that an enum retains its public wire name in both directions.
fn assert_wire_name<T: Serialize + DeserializeOwned>(value: T, expected: &str) {
    let json = serde_json::to_string(&value).unwrap();
    assert_eq!(json, format!("\"{expected}\""));
    let restored: T = serde_json::from_str(&json).unwrap();
    assert_eq!(serde_json::to_string(&restored).unwrap(), json);
}

#[test]
fn inventory_and_dose_status_names_are_stable() {
    assert_wire_name(InventoryLotStatus::Active, "active");
    assert_wire_name(InventoryLotStatus::Reserve, "reserve");
    assert_wire_name(InventoryLotStatus::OnHold, "on_hold");
    assert_wire_name(InventoryLotStatus::Empty, "empty");
    assert_wire_name(InventoryLotStatus::Expired, "expired");
    assert_wire_name(InventoryLotStatus::Discarded, "discarded");
    assert_wire_name(InventoryLotStatus::Archived, "archived");
    assert_wire_name(DoseLogStatus::Active, "active");
    assert_wire_name(DoseLogStatus::Cancelled, "cancelled");
}

#[test]
fn product_and_stock_enum_names_are_stable() {
    for (value, name) in [
        (ProductUserImportance::Low, "low"),
        (ProductUserImportance::Normal, "normal"),
        (ProductUserImportance::High, "high"),
        (ProductUserImportance::Critical, "critical"),
    ] {
        assert_wire_name(value, name);
    }
    for (value, name) in [
        (ProductCategory::Supplement, "supplement"),
        (ProductCategory::Medication, "medication"),
        (ProductCategory::Food, "food"),
        (ProductCategory::Herb, "herb"),
        (ProductCategory::Other, "other"),
    ] {
        assert_wire_name(value, name);
    }
    for (value, name) in [
        (DosageForm::Tablet, "tablet"),
        (DosageForm::Capsule, "capsule"),
        (DosageForm::Softgel, "softgel"),
        (DosageForm::Powder, "powder"),
        (DosageForm::Liquid, "liquid"),
        (DosageForm::Drops, "drops"),
        (DosageForm::Spray, "spray"),
        (DosageForm::Gummy, "gummy"),
        (DosageForm::Other, "other"),
    ] {
        assert_wire_name(value, name);
    }
    for (value, name) in [
        (NutrientKind::Vitamin, "vitamin"),
        (NutrientKind::Mineral, "mineral"),
        (NutrientKind::AminoAcid, "amino_acid"),
        (NutrientKind::FattyAcid, "fatty_acid"),
        (NutrientKind::HerbExtract, "herb_extract"),
        (NutrientKind::DrugSubstance, "drug_substance"),
        (NutrientKind::Probiotic, "probiotic"),
        (NutrientKind::Other, "other"),
    ] {
        assert_wire_name(value, name);
    }
    for (value, name) in [
        (StockMovementType::Dose, "dose"),
        (StockMovementType::ManualAdd, "manual_add"),
        (StockMovementType::ManualRemove, "manual_remove"),
        (StockMovementType::Correction, "correction"),
        (StockMovementType::Discard, "discard"),
        (StockMovementType::OpenPackage, "open_package"),
        (StockMovementType::Reconcile, "reconcile"),
    ] {
        assert_wire_name(value, name);
    }
}
