use std::path::PathBuf;

use doseark_application::StorageBackupManager;
use doseark_db_sqlite::{
    create_sqlite_pool, run_migrations, SqliteBackupManager, SqliteStorageConfig,
};
use uuid::Uuid;

#[tokio::test]
async fn sqlite_backup_contains_committed_data() {
    let root = std::env::temp_dir().join(format!("doseark-sqlite-backup-{}", Uuid::new_v4()));
    let database_path = root.join("doseark.sqlite");
    let backup_path = root.join("backups").join("doseark-backup.sqlite");

    let config = SqliteStorageConfig::new(&database_path);
    let pool = create_sqlite_pool(&config)
        .await
        .expect("open SQLite database");
    run_migrations(&pool).await.expect("run migrations");

    let now = chrono::Utc::now();

    // Insert a complete brand record before creating the snapshot
    sqlx::query(
        "INSERT INTO brands (id, name, created_at, updated_at)
     VALUES (?1, ?2, ?3, ?4)",
    )
    .bind(Uuid::new_v4().to_string())
    .bind("Backup test")
    .bind(now)
    .bind(now)
    .execute(&pool)
    .await
    .expect("insert test row");

    let backup_manager = SqliteBackupManager::new(pool.clone());
    backup_manager
        .create_backup(&backup_path)
        .await
        .expect("create backup");

    assert!(backup_path.is_file());

    let backup_pool = create_sqlite_pool(&SqliteStorageConfig::new(&backup_path))
        .await
        .expect("open backup database");
    let brand_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM brands")
        .fetch_one(&backup_pool)
        .await
        .expect("query backup");
    assert_eq!(brand_count, 1);

    backup_pool.close().await;
    pool.close().await;
    cleanup(root);
}

fn cleanup(path: PathBuf) {
    let _ = std::fs::remove_dir_all(path);
}
