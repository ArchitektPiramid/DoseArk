//! Zgodność historycznych żądań z obsługą archiwizacji produktów

use doseark_domain::{CreateProductInput, UpdateProductInput};
use serde_json::json;

fn product_payload() -> serde_json::Value {
    json!({
        "name": "Test product",
        "category": "supplement",
        "dosage_form": "tablet",
        "serving_unit": "tablet",
        "serving_size_units": 1.0
    })
}

#[test]
fn omitted_archived_flag_keeps_legacy_requests_valid() {
    let create: CreateProductInput = serde_json::from_value(product_payload()).unwrap();
    let update: UpdateProductInput = serde_json::from_value(product_payload()).unwrap();
    assert_eq!(create.archived, None);
    assert_eq!(update.archived, None);
    assert!(serde_json::to_value(create)
        .unwrap()
        .get("archived")
        .is_none());
    assert!(serde_json::to_value(update)
        .unwrap()
        .get("archived")
        .is_none());
}

#[test]
fn archive_state_accepts_explicit_true_and_false() {
    for archived in [true, false] {
        let mut payload = product_payload();
        payload["archived"] = json!(archived);
        let create: CreateProductInput = serde_json::from_value(payload.clone()).unwrap();
        let update: UpdateProductInput = serde_json::from_value(payload).unwrap();
        assert_eq!(create.archived, Some(archived));
        assert_eq!(update.archived, Some(archived));
    }
}
