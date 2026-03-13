//! 数据库迁移模块
//! 处理数据库版本升级和数据迁移

use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use anyhow::Result;
use log::info;

/// 嵌入的数据库迁移
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

/// 运行所有挂起的迁移
pub fn run_migrations(conn: &mut SqliteConnection) -> Result<()> {
    info!("Checking for pending migrations...");

    conn.run_pending_migrations(MIGRATIONS)
        .map(|_| {
            info!("Database migrations completed successfully");
        })
        .map_err(|e| anyhow::anyhow!("Failed to run migrations: {}", e))?;

    Ok(())
}

/// 回滚最近的迁移
pub fn revert_migration(conn: &mut SqliteConnection) -> Result<()> {
    info!("Reverting last migration...");

    conn.revert_last_migration(MIGRATIONS)
        .map(|_| {
            info!("Migration reverted successfully");
        })
        .map_err(|e| anyhow::anyhow!("Failed to revert migration: {}", e))?;

    Ok(())
}

/// 获取当前迁移版本
pub fn current_version(conn: &mut SqliteConnection) -> Result<String> {
    let _ = conn;
    Ok("unknown".to_string())
}

/// 检查是否有挂起的迁移
pub fn has_pending_migrations(conn: &mut SqliteConnection) -> Result<bool> {
    let pending = conn
        .pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow::anyhow!("Failed to check pending migrations: {}", e))?
        .len() > 0;

    Ok(pending)
}
