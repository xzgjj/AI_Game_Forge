//! 仓储模块
//! 定义数据访问层接口和实现

use std::sync::Arc;
use anyhow::Result;
use diesel::prelude::*;
use uuid::Uuid;

use crate::database::{ConnectionPool, PooledConnection};
use crate::models::*;

/// 基础仓储接口
pub trait Repository<T> {
    fn create(&self, item: T) -> Result<T>;
    fn find_by_id(&self, id: Uuid) -> Result<Option<T>>;
    fn find_all(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<T>>;
    fn update(&self, id: Uuid, updates: T) -> Result<T>;
    fn delete(&self, id: Uuid) -> Result<bool>;
    fn count(&self) -> Result<i64>;
}

/// 用户仓储
pub struct UserRepository {
    pool: Arc<ConnectionPool>,
}

impl UserRepository {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        Self { pool }
    }

    pub fn get_connection(&self) -> Result<PooledConnection> {
        self.pool.get().map_err(|e| anyhow::anyhow!("Failed to get connection: {}", e))
    }
}

// TODO: 实现具体的仓储方法
// 这里需要根据实际的数据库schema来实现

/// 项目仓储
pub struct ProjectRepository {
    pool: Arc<ConnectionPool>,
}

impl ProjectRepository {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        Self { pool }
    }
}

/// AI日志仓储
pub struct AILogRepository {
    pool: Arc<ConnectionPool>,
}

impl AILogRepository {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        Self { pool }
    }
}

/// 游戏配置仓储
pub struct GameSpecRepository {
    pool: Arc<ConnectionPool>,
}

impl GameSpecRepository {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        Self { pool }
    }
}

/// API统计仓储
pub struct APIStatsRepository {
    pool: Arc<ConnectionPool>,
}

impl APIStatsRepository {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        Self { pool }
    }
}

/// 认证会话仓储
pub struct AuthSessionRepository {
    pool: Arc<ConnectionPool>,
}

impl AuthSessionRepository {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        Self { pool }
    }
}

/// 仓储管理器
pub struct RepositoryManager {
    pub users: Arc<UserRepository>,
    pub projects: Arc<ProjectRepository>,
    pub ai_logs: Arc<AILogRepository>,
    pub game_specs: Arc<GameSpecRepository>,
    pub api_stats: Arc<APIStatsRepository>,
    pub auth_sessions: Arc<AuthSessionRepository>,
}

impl RepositoryManager {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        Self {
            users: Arc::new(UserRepository::new(pool.clone())),
            projects: Arc::new(ProjectRepository::new(pool.clone())),
            ai_logs: Arc::new(AILogRepository::new(pool.clone())),
            game_specs: Arc::new(GameSpecRepository::new(pool.clone())),
            api_stats: Arc::new(APIStatsRepository::new(pool.clone())),
            auth_sessions: Arc::new(AuthSessionRepository::new(pool.clone())),
        }
    }
}
