//! Required fields and optional values in create/update request models.

use doseark_domain::{CreateInventoryLotInput, InventoryLotStatus, SaveProductNutrientInput};
use serde_json::json;
use uuid::Uuid;

#[test]
fn inventory_lot_input_round_trips_without_losing_optional_fields() {
    let product_id = Uuid::parse_str("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa").unwrap();
    let location_id = Uuid::parse_str("bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb").unwrap();
    let input = CreateInventoryLotInput {
        product_id,
        lot_number: Some("LOT-10".into()),
        storage_location: None,
        storage_location_id: Some(location_id),
        units_initial: 30.0,
        units_remaining: Some(22.0),
        purchase_date: None,
        opened_date: None,
        expires_at: None,
        price_amount: Some(25.5),
        price_currency: Some("PLN".into()),
        status: Some(InventoryLotStatus::OnHold),
        notes: None,
    };

    let value = serde_json::to_value(&input).unwrap();
    assert_eq!(value["product_id"], json!(product_id));
    assert_eq!(value["storage_location_id"], json!(location_id));
    assert_eq!(value["status"], "on_hold");
    assert_eq!(value["units_remaining"], 22.0);

    let restored: CreateInventoryLotInput = serde_json::from_value(value).unwrap();
    assert_eq!(restored.storage_location_id, Some(location_id));
    assert_eq!(restored.price_amount, Some(25.5));
}

#[test]
fn composition_input_requires_nutrient_identity() {
    let nutrient_id = Uuid::parse_str("cccccccc-cccc-cccc-cccc-cccccccccccc").unwrap();
    let input = SaveProductNutrientInput {
        nutrient_id,
        amount_per_serving: 150.0,
        unit: "mg".into(),
        label_daily_value_percent: Some(20.0),
        label_reference_amount: None,
        label_reference_unit: None,
        source: "label".into(),
        notes: None,
    };

    let mut value = serde_json::to_value(&input).unwrap();
    let restored: SaveProductNutrientInput = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(restored.nutrient_id, nutrient_id);
    assert_eq!(restored.amount_per_serving, 150.0);

    value.as_object_mut().unwrap().remove("nutrient_id");
    let error = serde_json::from_value::<SaveProductNutrientInput>(value).unwrap_err();
    assert!(error.to_string().contains("missing field `nutrient_id`"));
}
