//! Udostępnianie raportów i trendów dziennych

mod daily;
mod mapping;
mod trend;

use async_trait::async_trait;
use doseark_application::{ApplicationError, ApplicationResult, DailyIntakeReportRepository};
use doseark_domain::{
    DailyIntakeReport, DailyIntakeReportInput, NutrientTrendReport, NutrientTrendReportInput,
};
use sqlx::SqlitePool;

/// SQLite implementation of the daily intake and trend reports
#[derive(Clone)]
pub struct SqliteDailyIntakeReportRepository {
    pool: SqlitePool,
}

impl SqliteDailyIntakeReportRepository {
    /// Share the existing connection pool with report queries
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DailyIntakeReportRepository for SqliteDailyIntakeReportRepository {
    /// Load nutrient totals and dose details for a single day
    async fn get_daily_intake_report(
        &self,
        input: DailyIntakeReportInput,
    ) -> ApplicationResult<DailyIntakeReport> {
        daily::load(&self.pool, input).await
    }

    /// Build a trend for the requested product or nutrient
    async fn get_nutrient_trend_report(
        &self,
        input: NutrientTrendReportInput,
    ) -> ApplicationResult<NutrientTrendReport> {
        trend::load(&self.pool, input).await
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ApplicationError {
    ApplicationError::Repository(error.to_string())
}
