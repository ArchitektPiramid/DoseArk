//! Backward compatibility for persisted history and older report payloads.

use chrono::{DateTime, Utc};
use doseark_domain::{DoseLog, DoseLogStatus, NutrientTrendReport};
use serde_json::json;
use uuid::Uuid;

fn fixed_time() -> DateTime<Utc> {
    DateTime::parse_from_rfc3339("2026-11-07T10:00:00Z")
        .unwrap()
        .with_timezone(&Utc)
}

#[test]
fn older_dose_history_omitting_stock_projection_deserializes() {
    let log = DoseLog {
        id: Uuid::nil(),
        product_id: Uuid::nil(),
        inventory_lot_id: None,
        taken_at: fixed_time(),
        units_taken: 2.0,
        unit: "capsule".into(),
        serving_multiplier: 1.0,
        inventory_quantity_before: Some(12.0),
        inventory_quantity_after: Some(10.0),
        inventory_units_initial: Some(30.0),
        reason: None,
        notes: None,
        status: DoseLogStatus::Active,
        cancelled_at: None,
        cancellation_reason: None,
        created_at: fixed_time(),
        updated_at: fixed_time(),
    };

    let mut value = serde_json::to_value(&log).unwrap();
    assert_eq!(value["status"], "active");
    for name in [
        "inventory_quantity_before",
        "inventory_quantity_after",
        "inventory_units_initial",
    ] {
        value.as_object_mut().unwrap().remove(name);
    }

    let restored: DoseLog = serde_json::from_value(value).unwrap();
    assert!(restored.inventory_quantity_before.is_none());
    assert!(restored.inventory_quantity_after.is_none());
    assert!(restored.inventory_units_initial.is_none());
    assert_eq!(restored.units_taken, 2.0);
}

#[test]
fn older_trend_report_omitting_events_deserializes() {
    let value = json!({
        "start_date": "2026-11-01",
        "end_date": "2026-11-07",
        "start_at": "2026-11-01T00:00:00Z",
        "end_at": "2026-11-08T00:00:00Z",
        "selected_nutrient_id": null,
        "selected_product_id": null,
        "selected_unit": null,
        "day_start_hour": 4,
        "nutrients": [],
        "days": []
    });

    let report: NutrientTrendReport = serde_json::from_value(value).unwrap();
    assert_eq!(report.day_start_hour, 4);
    assert!(report.events.is_empty());
    assert!(report.days.is_empty());
}
