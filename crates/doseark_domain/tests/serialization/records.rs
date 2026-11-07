//! Round-trip tests for full catalog, inventory and historical snapshot records.
//!
//! These fixtures intentionally include optional fields and timestamps to
//! detect accidental changes to public JSON keys when models move modules.

use doseark_domain::{DoseLogNutrient, Product, StockMovement};
use serde_json::{json, Value};

/// Requires a serialized model to reproduce the entire original JSON object.
fn assert_round_trip<T>(value: Value)
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    let decoded: T = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(serde_json::to_value(decoded).unwrap(), value);
}

#[test]
fn catalog_product_record_keeps_all_fields() {
    assert_round_trip::<Product>(json!({
        "id": "aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa",
        "brand_id": null,
        "name": "Vitamin C",
        "category": "supplement",
        "dosage_form": "capsule",
        "serving_unit": "capsule",
        "serving_size_units": 2.0,
        "unit_weight_mg": 500.0,
        "barcode": null,
        "description": "Standard serving: two capsules",
        "archived": false,
        "created_at": "2026-11-01T12:00:00Z",
        "updated_at": "2026-11-01T12:00:00Z",
        "ui_color": "#ffbb00",
        "ui_icon": null,
        "user_importance": "normal"
    }));
}

#[test]
fn inventory_movement_preserves_signed_quantities() {
    assert_round_trip::<StockMovement>(json!({
        "id": "aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa",
        "inventory_lot_id": "bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb",
        "product_id": "cccccccc-cccc-cccc-cccc-cccccccccccc",
        "dose_log_id": null,
        "occurred_at": "2026-11-07T09:30:00Z",
        "movement_type": "manual_remove",
        "quantity_delta": -3.5,
        "quantity_after": 14.0,
        "reason": "manual adjustment",
        "notes": null,
        "created_at": "2026-11-07T09:30:00Z"
    }));
}

#[test]
fn nutrient_snapshot_preserves_recorded_values() {
    assert_round_trip::<DoseLogNutrient>(json!({
        "id": "aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa",
        "dose_log_id": "bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb",
        "nutrient_id": "cccccccc-cccc-cccc-cccc-cccccccccccc",
        "nutrient_canonical_name": "vitamin_c",
        "nutrient_display_name": "Vitamin C",
        "nutrient_kind": "vitamin",
        "amount": 125.0,
        "unit": "mg",
        "reference_daily_amount": 80.0,
        "reference_daily_unit": "mg",
        "reference_source": null,
        "percent_of_reference": 156.25,
        "created_at": "2026-11-07T09:30:00Z"
    }));
}
