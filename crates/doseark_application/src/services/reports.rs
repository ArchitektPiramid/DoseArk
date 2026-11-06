//! Read-only daily intake and nutrient trend reports.
use crate::{ApplicationError, ApplicationResult, DailyIntakeReportRepository};
use doseark_domain::{DailyIntakeReport, DailyIntakeReportInput, NutrientTrendReport, NutrientTrendReportInput};
use std::sync::Arc;

/// Checks report ranges and delegates reporting queries.
#[derive(Clone)]
pub struct DailyIntakeReportService {
    repo: Arc<dyn DailyIntakeReportRepository>,
}

impl DailyIntakeReportService {
    pub fn new(repo: Arc<dyn DailyIntakeReportRepository>) -> Self {
        Self { repo }
    }

    pub async fn get_daily_intake_report(
        &self,
        input: DailyIntakeReportInput,
    ) -> ApplicationResult<DailyIntakeReport> {
        if input.end_at <= input.start_at {
            return Err(ApplicationError::Validation(
                "Report end time must be after start time".into(),
            ));
        }

        self.repo.get_daily_intake_report(input).await
    }

    /// Rejects invalid and excessively long report ranges.
    pub async fn get_nutrient_trend_report(
        &self,
        input: NutrientTrendReportInput,
    ) -> ApplicationResult<NutrientTrendReport> {
        if input.end_at <= input.start_at {
            return Err(ApplicationError::Validation(
                "Report end time must be after start time".into(),
            ));
        }

        if input.end_date < input.start_date {
            return Err(ApplicationError::Validation(
                "Report end date must not be before start date".into(),
            ));
        }

        let day_count = input
            .end_date
            .signed_duration_since(input.start_date)
            .num_days()
            + 1;

        if day_count > 732 {
            return Err(ApplicationError::Validation(
                "Report range cannot exceed two years".into(),
            ));
        }

        if input.nutrient_id.is_some() && input.product_id.is_some() {
            return Err(ApplicationError::Validation(
                "Select either a nutrient or a product report source".into(),
            ));
        }

        self.repo.get_nutrient_trend_report(input).await
    }
}
