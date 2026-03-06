//! 用户模型
//! 定义用户数据结构和相关功能

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// 用户角色
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    Guest,      // 访客
    User,       // 普通用户
    Pro,        // 专业用户
    Admin,      // 管理员
}

/// 用户状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserStatus {
    Pending,    // 待验证
    Active,     // 活跃
    Suspended,  // 已暂停
    Banned,     // 已封禁
}

/// 用户模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub role: UserRole,
    pub status: UserStatus,
    pub preferences: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub login_count: u32,
    pub total_spent: f64,
}

impl User {
    /// 创建新用户
    pub fn new(
        username: String,
        email: Option<String>,
        phone: Option<String>,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            email,
            phone,
            username,
            display_name: None,
            avatar_url: None,
            role: UserRole::User,
            status: UserStatus::Pending,
            preferences: serde_json::json!({}),
            created_at: now,
            updated_at: now,
            last_login_at: None,
            login_count: 0,
            total_spent: 0.0,
        }
    }

    /// 检查用户是否已验证
    pub fn is_verified(&self) -> bool {
        self.status == UserStatus::Active
    }

    /// 检查用户是否有管理员权限
    pub fn is_admin(&self) -> bool {
        self.role == UserRole::Admin
    }

    /// 更新最后登录时间
    pub fn update_login(&mut self) {
        let now = Utc::now();
        self.last_login_at = Some(now);
        self.login_count += 1;
        self.updated_at = now;
    }

    /// 更新用户偏好
    pub fn update_preferences(&mut self, preferences: serde_json::Value) {
        self.preferences = preferences;
        self.updated_at = Utc::now();
    }

    /// 添加消费记录
    pub fn add_spending(&mut self, amount: f64) {
        self.total_spent += amount;
        self.updated_at = Utc::now();
    }
}

/// 用户统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStats {
    pub user_id: Uuid,
    pub project_count: u32,
    pub generation_count: u32,
    pub total_tokens: u64,
    pub total_cost: f64,
    pub favorite_providers: Vec<String>,
    pub avg_session_duration: f64, // 分钟
    pub last_active: DateTime<Utc>,
}
