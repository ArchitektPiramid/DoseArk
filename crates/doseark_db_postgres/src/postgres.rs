use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn create_pg_pool(
    database_url: &str,
    max_connections: u32,
) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(database_url)
        .await
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    tracing::info!("Running database migrations");

    match sqlx::migrate!("./migrations").run(pool).await {
        Ok(()) => {
            tracing::info!("Database migrations finished");
            Ok(())
        }
        Err(err) => {
            tracing::error!(
                error = %err,
                "Database migration failed. If this says a migration was modified, do not edit already-applied migrations. Add a new migration or reset the dev DB migration entry."
            );

            Err(err)
        }
    }
}
