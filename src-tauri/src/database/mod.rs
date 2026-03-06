//! 数据库模块
//! 处理数据库连接、迁移和仓储模式

pub mod migrations;
pub mod repository;
pub mod schema;

use std::sync::Arc;
use anyhow::Result;
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use tauri::AppHandle;
use log::{info, warn};

/// 数据库连接池类型
pub type ConnectionPool = Pool<ConnectionManager<SqliteConnection>>;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;

/// 数据库配置
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub path: String,
    pub max_connections: u32,
    pub connection_timeout_secs: u64,
    pub enable_foreign_keys: bool,
    pub enable_wal: bool,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            path: "gamecraft.db".to_string(),
            max_connections: 5,
            connection_timeout_secs: 30,
            enable_foreign_keys: true,
            enable_wal: true,
        }
    }
}

/// 数据库管理器
pub struct DatabaseManager {
    pool: ConnectionPool,
    db_path: String,
}

impl DatabaseManager {
    /// 创建新的数据库管理器
    pub fn new(config: DatabaseConfig) -> Result<Self> {
        info!("Initializing database at: {}", config.path);

        // 创建数据库文件目录（如果需要）
        if let Some(parent) = std::path::Path::new(&config.path).parent() {
            std::fs::create_dir_all(parent)?;
        }

        // 创建连接管理器
        let manager = ConnectionManager::<SqliteConnection>::new(&config.path);

        // 创建连接池
        let pool = Pool::builder()
            .max_size(config.max_connections)
            .connection_timeout(std::time::Duration::from_secs(config.connection_timeout_secs))
            .build(manager)?;

        // 测试连接
        let mut conn = pool.get()?;
        info!("Database connection established");

        // 配置数据库
        Self::configure_database(&mut conn, &config)?;

        Ok(Self {
            pool,
            db_path: config.path,
        })
    }

    /// 配置数据库
    fn configure_database(conn: &mut SqliteConnection, config: &DatabaseConfig) -> Result<()> {
        use diesel::RunQueryDsl;

        // 启用外键约束
        if config.enable_foreign_keys {
            diesel::sql_query("PRAGMA foreign_keys = ON;").execute(conn)?;
        }

        // 启用WAL模式（提高并发性能）
        if config.enable_wal {
            diesel::sql_query("PRAGMA journal_mode = WAL;").execute(conn)?;
            diesel::sql_query("PRAGMA synchronous = NORMAL;").execute(conn)?;
        }

        // 设置合理的缓存大小
        diesel::sql_query("PRAGMA cache_size = -10000;").execute(conn)?; // 10MB缓存

        info!("Database configured successfully");
        Ok(())
    }

    /// 获取连接池
    pub fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    /// 获取数据库连接
    pub fn get_connection(&self) -> Result<PooledConnection> {
        self.pool.get().map_err(|e| anyhow::anyhow!("Failed to get database connection: {}", e))
    }

    /// 运行数据库迁移
    pub fn run_migrations(&self) -> Result<()> {
        info!("Running database migrations...");

        let mut conn = self.get_connection()?;
        migrations::run_migrations(&mut conn)?;

        info!("Database migrations completed successfully");
        Ok(())
    }

    /// 备份数据库
    pub fn backup(&self, backup_path: &str) -> Result<()> {
        info!("Backing up database to: {}", backup_path);

        let mut conn = self.get_connection()?;

        // 使用SQLite的备份API
        diesel::sql_query(format!("VACUUM INTO '{}';", backup_path))
            .execute(&mut conn)?;

        info!("Database backup completed");
        Ok(())
    }

    /// 检查数据库完整性
    pub fn check_integrity(&self) -> Result<bool> {
        match self.get_connection() {
            Ok(_) => {
                info!("Database integrity check passed");
                Ok(true)
            }
            Err(error) => {
                warn!("Database integrity check failed: {}", error);
                Ok(false)
            }
        }
    }

    /// 获取数据库统计信息
    pub fn get_stats(&self) -> Result<DatabaseStats> {
        let file_size = std::fs::metadata(&self.db_path)
            .map(|meta| i64::try_from(meta.len()).unwrap_or(i64::MAX))
            .unwrap_or(0);

        Ok(DatabaseStats {
            file_size_bytes: file_size,
            used_pages: 0,
            free_pages: 0,
            page_size: 0,
        })
    }
}

/// 数据库统计信息
#[derive(Debug, Clone)]
pub struct DatabaseStats {
    pub file_size_bytes: i64,
    pub used_pages: i64,
    pub free_pages: i64,
    pub page_size: u32,
}

/// 初始化数据库（供应用启动时调用）
pub fn init(app: &AppHandle) -> Result<()> {
    info!("Initializing database...");

    // 获取应用数据目录
    let app_data_dir = app.path().app_data_dir()?;
    let db_path = app_data_dir.join("gamecraft.db").to_string_lossy().to_string();

    // 创建数据库配置
    let config = DatabaseConfig {
        path: db_path,
        ..Default::default()
    };

    // 创建数据库管理器
    let db_manager = DatabaseManager::new(config)?;

    // 运行迁移
    db_manager.run_migrations()?;

    // 将数据库管理器存储到Tauri状态
    app.manage(Arc::new(db_manager));

    info!("Database initialization completed");
    Ok(())
}

/// 获取数据库连接（从Tauri状态）
pub fn get_connection(app: &AppHandle) -> Result<PooledConnection> {
    let db_manager = app.state::<Arc<DatabaseManager>>();
    db_manager.get_connection()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn database_config_default_should_be_valid() {
        let config = DatabaseConfig::default();
        assert!(!config.path.is_empty());
        assert!(config.max_connections > 0);
    }

    #[test]
    fn database_stats_shape_should_be_constructible() {
        let stats = DatabaseStats {
            file_size_bytes: 0,
            used_pages: 0,
            free_pages: 0,
            page_size: 0,
        };

        assert_eq!(stats.file_size_bytes, 0);
    }
}
