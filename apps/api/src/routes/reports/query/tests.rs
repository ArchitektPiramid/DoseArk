//! Report boundary and validation regression tests (no database required).

use super::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[test]
fn daily_report_uses_half_open_local_day_window() {
    let input = build_daily_intake_input(DailyIntakeReportQuery {
        date: "2026-11-07".to_string(),
        timezone_offset_minutes: Some(-60),
    })
    .expect("valid date");
    assert_eq!(
        input.start_at,
        DateTime::parse_from_rfc3339("2026-11-06T23:00:00Z")
            .unwrap()
            .with_timezone(&Utc)
    );
    assert_eq!(
        input.end_at,
        DateTime::parse_from_rfc3339("2026-11-07T23:00:00Z")
            .unwrap()
            .with_timezone(&Utc)
    );
}

#[test]
fn trend_window_respects_day_start_hour_and_offset() {
    let input = build_nutrient_trend_input(NutrientTrendReportQuery {
        start_date: "2026-11-07".into(),
        end_date: "2026-11-08".into(),
        timezone_offset_minutes: Some(-60),
        day_start_hour: Some(4),
        nutrient_id: Some(Uuid::new_v4()),
        product_id: None,
        unit: None,
    })
    .expect("valid trend");
    assert_eq!(
        input.start_at,
        DateTime::parse_from_rfc3339("2026-11-07T03:00:00Z")
            .unwrap()
            .with_timezone(&Utc)
    );
    assert_eq!(
        input.end_at,
        DateTime::parse_from_rfc3339("2026-11-09T03:00:00Z")
            .unwrap()
            .with_timezone(&Utc)
    );
}

#[test]
fn trend_rejects_invalid_range_and_conflicting_filters() {
    let query = || NutrientTrendReportQuery {
        start_date: "2026-11-08".into(),
        end_date: "2026-11-07".into(),
        timezone_offset_minutes: None,
        day_start_hour: None,
        nutrient_id: None,
        product_id: None,
        unit: None,
    };
    assert!(build_nutrient_trend_input(query()).is_err());
    assert!(build_nutrient_trend_input(NutrientTrendReportQuery {
        start_date: "2026-11-07".into(),
        end_date: "2026-11-07".into(),
        day_start_hour: Some(24),
        ..query()
    })
    .is_err());
    assert!(build_nutrient_trend_input(NutrientTrendReportQuery {
        start_date: "2026-11-07".into(),
        end_date: "2026-11-07".into(),
        nutrient_id: Some(Uuid::new_v4()),
        product_id: Some(Uuid::new_v4()),
        ..query()
    })
    .is_err());
}
