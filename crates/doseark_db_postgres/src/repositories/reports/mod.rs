//! Read-only intake reports and trend aggregation for the PostgreSQL backend.

mod daily;
mod mapping;
mod trend;

use async_trait::async_trait;
use doseark_application::{ApplicationResult, DailyIntakeReportRepository};
use doseark_domain::{
    DailyIntakeReport, DailyIntakeReportInput, NutrientTrendReport, NutrientTrendReportInput,
};
use sqlx::PgPool;

#[derive(Clone)]
pub struct PgDailyIntakeReportRepository {
    pool: PgPool,
}

impl PgDailyIntakeReportRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DailyIntakeReportRepository for PgDailyIntakeReportRepository {
    async fn get_daily_intake_report(
        &self,
        input: DailyIntakeReportInput,
    ) -> ApplicationResult<DailyIntakeReport> {
        daily::get_daily_intake_report(&self.pool, input).await
    }

    async fn get_nutrient_trend_report(
        &self,
        input: NutrientTrendReportInput,
    ) -> ApplicationResult<NutrientTrendReport> {
        trend::get_nutrient_trend_report(&self.pool, input).await
    }
}
