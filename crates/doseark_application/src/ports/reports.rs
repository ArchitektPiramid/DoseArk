//! Port raportów dziennych i trendów składników

use crate::ApplicationResult;
use async_trait::async_trait;
use doseark_domain::{
    DailyIntakeReport, DailyIntakeReportInput, NutrientTrendReport, NutrientTrendReportInput,
};

/// Builds daily intake and trend reports from stored doses
#[async_trait]
pub trait DailyIntakeReportRepository: Send + Sync {
    async fn get_daily_intake_report(
        &self,
        input: DailyIntakeReportInput,
    ) -> ApplicationResult<DailyIntakeReport>;

    async fn get_nutrient_trend_report(
        &self,
        input: NutrientTrendReportInput,
    ) -> ApplicationResult<NutrientTrendReport>;
}
