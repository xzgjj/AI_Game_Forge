//! 用户服务模块
//! 处理用户资料和偏好的业务逻辑

use std::collections::HashMap;
use std::sync::RwLock;

use anyhow::{anyhow, Result};
use uuid::Uuid;

use crate::models::user::{User, UserStatus};

/// 用户服务
pub struct UserService {
    users: RwLock<HashMap<Uuid, User>>,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            users: RwLock::new(HashMap::new()),
        }
    }

    pub fn create_user(
        &self,
        username: String,
        email: Option<String>,
        phone: Option<String>,
    ) -> Result<User> {
        if username.trim().is_empty() {
            return Err(anyhow!("用户名不能为空"));
        }

        let user = User::new(username, email, phone);
        self.users
            .write()
            .expect("user lock poisoned")
            .insert(user.id, user.clone());

        Ok(user)
    }

    pub fn get_user(&self, user_id: Uuid) -> Option<User> {
        self.users
            .read()
            .expect("user lock poisoned")
            .get(&user_id)
            .cloned()
    }

    pub fn activate_user(&self, user_id: Uuid) -> Result<User> {
        let mut users = self.users.write().expect("user lock poisoned");
        let user = users
            .get_mut(&user_id)
            .ok_or_else(|| anyhow!("用户不存在"))?;

        user.status = UserStatus::Active;
        user.updated_at = chrono::Utc::now();
        Ok(user.clone())
    }

    pub fn suspend_user(&self, user_id: Uuid, reason: Option<String>) -> Result<User> {
        let mut users = self.users.write().expect("user lock poisoned");
        let user = users
            .get_mut(&user_id)
            .ok_or_else(|| anyhow!("用户不存在"))?;

        user.status = UserStatus::Suspended;
        if let Some(r) = reason {
            user.preferences["suspend_reason"] = serde_json::Value::String(r);
        }
        user.updated_at = chrono::Utc::now();

        Ok(user.clone())
    }

    pub fn update_preferences(&self, user_id: Uuid, preferences: serde_json::Value) -> Result<User> {
        if !preferences.is_object() {
            return Err(anyhow!("偏好配置必须是对象"));
        }

        let mut users = self.users.write().expect("user lock poisoned");
        let user = users
            .get_mut(&user_id)
            .ok_or_else(|| anyhow!("用户不存在"))?;

        user.update_preferences(preferences);
        Ok(user.clone())
    }

    pub fn add_spending(&self, user_id: Uuid, amount: f64) -> Result<User> {
        if amount < 0.0 {
            return Err(anyhow!("消费金额不能为负数"));
        }

        let mut users = self.users.write().expect("user lock poisoned");
        let user = users
            .get_mut(&user_id)
            .ok_or_else(|| anyhow!("用户不存在"))?;

        user.add_spending(amount);
        Ok(user.clone())
    }

    pub fn list_users(&self, limit: Option<usize>, offset: Option<usize>) -> Vec<User> {
        let users = self.users.read().expect("user lock poisoned");

        let mut data = users.values().cloned().collect::<Vec<_>>();
        data.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

        let start = offset.unwrap_or(0);
        let take = limit.unwrap_or(data.len());

        data.into_iter().skip(start).take(take).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_activate_user_should_work() {
        let service = UserService::new();
        let created = service
            .create_user("dev_user".to_string(), Some("dev@example.com".to_string()), None)
            .expect("create should succeed");

        assert_eq!(created.status, UserStatus::Pending);

        let activated = service
            .activate_user(created.id)
            .expect("activate should succeed");

        assert_eq!(activated.status, UserStatus::Active);
    }

    #[test]
    fn update_preferences_and_spending_should_work() {
        let service = UserService::new();
        let created = service
            .create_user("budget_user".to_string(), None, Some("+8613800000000".to_string()))
            .expect("create should succeed");

        let updated = service
            .update_preferences(
                created.id,
                serde_json::json!({
                    "theme": "dark",
                    "preferred_provider": "openai"
                }),
            )
            .expect("update preferences should succeed");

        assert_eq!(updated.preferences["theme"], serde_json::json!("dark"));

        let with_spending = service
            .add_spending(created.id, 12.5)
            .expect("add spending should succeed");

        assert!(with_spending.total_spent >= 12.5);
    }
}
