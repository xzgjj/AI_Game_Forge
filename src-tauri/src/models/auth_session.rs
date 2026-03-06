//! 认证会话模型
//! 定义用户会话数据结构和相关功能

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// 认证方法
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuthMethod {
    Wechat,     // 微信扫码
    Phone,      // 手机验证
    Email,      // 邮箱密码
    Github,     // GitHub OAuth
    Google,     // Google OAuth
    Local,      // 本地账户
}

/// 会话状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SessionStatus {
    Active,     // 活跃
    Expired,    // 已过期
    Revoked,    // 已撤销
    Invalid,    // 无效
}

/// 设备类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeviceType {
    Desktop,    // 桌面设备
    Mobile,     // 移动设备
    Tablet,     // 平板设备
    Unknown,    // 未知设备
}

/// 认证会话模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub auth_method: AuthMethod,
    pub device_id: String,
    pub device_type: DeviceType,
    pub user_agent: String,
    pub ip_address: Option<String>,
    pub token: String,
    pub refresh_token: Option<String>,
    pub status: SessionStatus,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub revocation_reason: Option<String>,
    pub metadata: serde_json::Value,
}

impl AuthSession {
    /// 创建新会话
    pub fn new(
        user_id: Uuid,
        auth_method: AuthMethod,
        device_id: String,
        device_type: DeviceType,
        user_agent: String,
        ip_address: Option<String>,
        token: String,
        refresh_token: Option<String>,
        expires_in_hours: u32,
    ) -> Self {
        let now = Utc::now();
        let expires_at = now + chrono::Duration::hours(expires_in_hours as i64);

        Self {
            id: Uuid::new_v4(),
            user_id,
            auth_method,
            device_id,
            device_type,
            user_agent,
            ip_address,
            token,
            refresh_token,
            status: SessionStatus::Active,
            created_at: now,
            expires_at,
            last_accessed: now,
            revoked_at: None,
            revocation_reason: None,
            metadata: serde_json::json!({}),
        }
    }

    /// 检查会话是否有效
    pub fn is_valid(&self) -> bool {
        self.status == SessionStatus::Active && Utc::now() < self.expires_at
    }

    /// 检查会话是否过期
    pub fn is_expired(&self) -> bool {
        Utc::now() >= self.expires_at
    }

    /// 更新最后访问时间
    pub fn update_access(&mut self) {
        self.last_accessed = Utc::now();
    }

    /// 刷新会话
    pub fn refresh(&mut self, new_token: String, new_refresh_token: Option<String>, expires_in_hours: u32) {
        let now = Utc::now();
        let new_expires_at = now + chrono::Duration::hours(expires_in_hours as i64);

        self.token = new_token;
        self.refresh_token = new_refresh_token;
        self.expires_at = new_expires_at;
        self.last_accessed = now;

        // 如果之前被标记为过期，重新激活
        if self.status == SessionStatus::Expired {
            self.status = SessionStatus::Active;
        }
    }

    /// 撤销会话
    pub fn revoke(&mut self, reason: Option<String>) {
        self.status = SessionStatus::Revoked;
        self.revoked_at = Some(Utc::now());
        self.revocation_reason = reason;
    }

    /// 标记为过期
    pub fn mark_expired(&mut self) {
        self.status = SessionStatus::Expired;
    }

    /// 获取剩余有效期（小时）
    pub fn remaining_hours(&self) -> f64 {
        if self.is_expired() {
            return 0.0;
        }

        let remaining = self.expires_at - Utc::now();
        remaining.num_seconds() as f64 / 3600.0
    }

    /// 获取会话持续时间（小时）
    pub fn duration_hours(&self) -> f64 {
        let duration = if self.status == SessionStatus::Active {
            Utc::now() - self.created_at
        } else if let Some(revoked_at) = self.revoked_at {
            revoked_at - self.created_at
        } else {
            self.last_accessed - self.created_at
        };

        duration.num_seconds() as f64 / 3600.0
    }

    /// 添加会话元数据
    pub fn add_metadata(&mut self, key: String, value: serde_json::Value) {
        self.metadata[key] = value;
    }

    /// 获取会话元数据
    pub fn get_metadata(&self, key: &str) -> Option<&serde_json::Value> {
        self.metadata.get(key)
    }

    /// 验证令牌
    pub fn verify_token(&self, token: &str) -> bool {
        self.token == token && self.is_valid()
    }
}

/// 设备信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub device_id: String,
    pub device_type: DeviceType,
    pub user_agent: String,
    pub os: String,
    pub browser: String,
    pub screen_resolution: Option<String>,
    pub language: String,
    pub timezone: String,
    pub last_login: DateTime<Utc>,
    pub session_count: u32,
}

impl DeviceInfo {
    /// 从User-Agent解析设备信息
    pub fn from_user_agent(device_id: String, user_agent: String) -> Self {
        let now = Utc::now();

        // 简单的User-Agent解析（实际应用中应该使用专门的库）
        let (os, browser) = Self::parse_user_agent(&user_agent);
        let device_type = Self::detect_device_type(&user_agent);

        Self {
            device_id,
            device_type,
            user_agent,
            os: os.to_string(),
            browser: browser.to_string(),
            screen_resolution: None,
            language: "zh-CN".to_string(), // 默认
            timezone: "UTC".to_string(),   // 默认
            last_login: now,
            session_count: 1,
        }
    }

    fn parse_user_agent(user_agent: &str) -> (&str, &str) {
        let ua = user_agent.to_lowercase();

        // 检测操作系统
        let os = if ua.contains("windows") {
            "Windows"
        } else if ua.contains("mac os") || ua.contains("macos") {
            "macOS"
        } else if ua.contains("linux") {
            "Linux"
        } else if ua.contains("android") {
            "Android"
        } else if ua.contains("iphone") || ua.contains("ipad") {
            "iOS"
        } else {
            "Unknown"
        };

        // 检测浏览器
        let browser = if ua.contains("chrome") && !ua.contains("edg") {
            "Chrome"
        } else if ua.contains("firefox") {
            "Firefox"
        } else if ua.contains("safari") && !ua.contains("chrome") {
            "Safari"
        } else if ua.contains("edge") {
            "Edge"
        } else if ua.contains("opera") {
            "Opera"
        } else {
            "Unknown"
        };

        (os, browser)
    }

    fn detect_device_type(user_agent: &str) -> DeviceType {
        let ua = user_agent.to_lowercase();

        if ua.contains("mobile") || ua.contains("iphone") || ua.contains("android") {
            DeviceType::Mobile
        } else if ua.contains("tablet") || ua.contains("ipad") {
            DeviceType::Tablet
        } else {
            DeviceType::Desktop
        }
    }

    /// 更新最后登录时间
    pub fn update_login(&mut self) {
        self.last_login = Utc::now();
        self.session_count += 1;
    }

    /// 设置屏幕分辨率
    pub fn set_screen_resolution(&mut self, resolution: String) {
        self.screen_resolution = Some(resolution);
    }

    /// 设置语言
    pub fn set_language(&mut self, language: String) {
        self.language = language;
    }

    /// 设置时区
    pub fn set_timezone(&mut self, timezone: String) {
        self.timezone = timezone;
    }
}

/// 登录尝试记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginAttempt {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub auth_method: AuthMethod,
    pub identifier: String, // 邮箱、手机号等
    pub ip_address: String,
    pub user_agent: String,
    pub success: bool,
    pub failure_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub metadata: serde_json::Value,
}

impl LoginAttempt {
    /// 创建新的登录尝试记录
    pub fn new(
        auth_method: AuthMethod,
        identifier: String,
        ip_address: String,
        user_agent: String,
        success: bool,
        failure_reason: Option<String>,
        user_id: Option<Uuid>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            auth_method,
            identifier,
            ip_address,
            user_agent,
            success,
            failure_reason,
            created_at: Utc::now(),
            metadata: serde_json::json!({}),
        }
    }

    /// 添加元数据
    pub fn add_metadata(&mut self, key: String, value: serde_json::Value) {
        self.metadata[key] = value;
    }

    /// 检查是否可疑（多次失败尝试）
    pub fn is_suspicious(failed_attempts: &[LoginAttempt], time_window_minutes: i64) -> bool {
        if failed_attempts.is_empty() {
            return false;
        }

        let now = Utc::now();
        let window_start = now - chrono::Duration::minutes(time_window_minutes);

        let recent_failures = failed_attempts
            .iter()
            .filter(|attempt| attempt.created_at >= window_start && !attempt.success)
            .count();

        // 例如：5分钟内5次失败尝试视为可疑
        recent_failures >= 5
    }
}
