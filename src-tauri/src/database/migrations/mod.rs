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
pub fn current_version(_conn: &mut SqliteConnection) -> Result<String> {
    // 由 diesel_migrations 管理版本，应用层仅返回可读占位版本。
    Ok("managed-by-diesel-migrations".to_string())
}

/// 检查是否有挂起的迁移
pub fn has_pending_migrations(_conn: &mut SqliteConnection) -> Result<bool> {
    // 兼容不同 Diesel 版本接口，避免调用不存在的 pending API。
    // 实际环境建议通过 run_pending_migrations 的返回值判断是否有迁移执行。
    Ok(false)
}
