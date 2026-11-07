//! Wspólny przedział UTC dla agregacji i zdarzeń raportu

use chrono::{DateTime, Duration, Utc};
use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::NutrientTrendReportInput;

/// Shift local-midnight bounds to the configured start of a reporting day
pub(super) fn query_bounds(
    input: &NutrientTrendReportInput,
) -> ApplicationResult<(DateTime<Utc>, DateTime<Utc>)> {
    let shift = Duration::hours(i64::from(input.day_start_hour));
    let start = input
        .start_at
        .checked_add_signed(shift)
        .ok_or_else(|| ApplicationError::Validation("Invalid report start date".into()))?;
    let end = input
        .end_at
        .checked_add_signed(shift)
        .ok_or_else(|| ApplicationError::Validation("Invalid report end date".into()))?;
    Ok((start, end))
}
