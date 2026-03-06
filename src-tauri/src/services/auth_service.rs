//! 认证服务模块
//! 处理多元认证系统的业务逻辑

use std::collections::HashMap;
use std::sync::RwLock;

use anyhow::{anyhow, Result};
use chrono::Utc;
use regex::Regex;
use uuid::Uuid;

use crate::models::auth_session::{AuthMethod, AuthSession, DeviceType, SessionStatus};
use crate::models::user::{User, UserStatus};

/// 认证提供商配置
#[derive(Debug, Clone)]
pub enum AuthProviderConfig {
    OAuth2 {
        client_id: String,
        client_secret: String,
        redirect_uri: String,
        auth_url: String,
        token_url: String,
    },
    Phone {
        provider: String,
        access_key_id: String,
        access_key_secret: String,
        sign_name: String,
        template_code: String,
    },
    Email {
        smtp_host: String,
        smtp_port: u16,
        smtp_username: String,
        smtp_password: String,
        from_address: String,
    },
    Local {
        password_hash_algorithm: String,
    },
}

/// 认证提供商
pub trait AuthProvider: Send + Sync {
    fn name(&self) -> &str;
    fn authenticate(&self, credentials: &HashMap<String, String>) -> Result<AuthenticatedUser>;
    fn validate_session(&self, session_id: &Uuid) -> Result<bool>;
    fn logout(&self, session_id: &Uuid) -> Result<()>;
}

/// 认证用户
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
    pub preferences: HashMap<String, serde_json::Value>,
    pub session_id: Uuid,
    pub expires_at: chrono::DateTime<Utc>,
}

/// 认证服务
pub struct AuthService {
    providers: HashMap<String, Box<dyn AuthProvider>>,
    users: RwLock<HashMap<Uuid, User>>,
    email_index: RwLock<HashMap<String, Uuid>>,
    phone_index: RwLock<HashMap<String, Uuid>>,
    password_hashes: RwLock<HashMap<Uuid, String>>,
    sessions: RwLock<HashMap<Uuid, AuthSession>>,
    failed_attempts: RwLock<HashMap<String, u32>>,
    config: AuthServiceConfig,
}

/// 认证服务配置
#[derive(Debug, Clone)]
pub struct AuthServiceConfig {
    pub session_expiry_hours: u32,
    pub max_login_attempts: u32,
    pub lockout_minutes: u32,
    pub require_email_verification: bool,
    pub require_phone_verification: bool,
}

impl AuthService {
    /// 创建新的认证服务
    pub fn new(config: AuthServiceConfig) -> Self {
        Self {
            providers: HashMap::new(),
            users: RwLock::new(HashMap::new()),
            email_index: RwLock::new(HashMap::new()),
            phone_index: RwLock::new(HashMap::new()),
            password_hashes: RwLock::new(HashMap::new()),
            sessions: RwLock::new(HashMap::new()),
            failed_attempts: RwLock::new(HashMap::new()),
            config,
        }
    }

    /// 注册认证提供商
    pub fn register_provider(&mut self, name: String, provider: Box<dyn AuthProvider>) {
        self.providers.insert(name, provider);
    }

    /// 微信扫码登录
    pub async fn wechat_login(&self, auth_code: String) -> Result<AuthenticatedUser> {
        if auth_code.trim().len() < 4 {
            return Err(anyhow!("微信授权码无效"));
        }

        let external_id = format!("wechat:{}", auth_code.trim());
        self.ensure_not_locked(&external_id)?;

        let user_id = {
            let index = self.email_index.read().expect("email index lock poisoned");
            index.get(&external_id).copied()
        };

        let uid = if let Some(existing) = user_id {
            existing
        } else {
            let mut user = User::new(
                format!("wechat_{}", short_code(auth_code.trim())),
                Some(external_id.clone()),
                None,
            );
            user.status = UserStatus::Active;
            self.insert_user(user)
        };

        self.reset_failed_attempts(&external_id);
        self.create_authenticated_session(uid, AuthMethod::Wechat)
    }

    /// 手机验证登录
    pub async fn phone_login(&self, phone: String, code: String) -> Result<AuthenticatedUser> {
        let normalized_phone = normalize_phone(&phone)?;

        self.ensure_not_locked(&normalized_phone)?;

        if self.config.require_phone_verification && code.trim().len() < 4 {
            self.record_failed_attempt(&normalized_phone);
            return Err(anyhow!("手机验证码无效"));
        }

        let user_id = {
            let index = self.phone_index.read().expect("phone index lock poisoned");
            index.get(&normalized_phone).copied()
        };

        let uid = if let Some(existing) = user_id {
            existing
        } else {
            let mut user = User::new(format!("phone_{}", short_code(&normalized_phone)), None, Some(normalized_phone.clone()));
            user.status = UserStatus::Active;
            self.insert_user(user)
        };

        self.reset_failed_attempts(&normalized_phone);
        self.create_authenticated_session(uid, AuthMethod::Phone)
    }

    /// 邮箱注册
    pub async fn email_register(
        &self,
        email: String,
        password: String,
        verification_code: String,
    ) -> Result<AuthenticatedUser> {
        let normalized_email = normalize_email(&email)?;

        self.ensure_not_locked(&normalized_email)?;

        if self.config.require_email_verification && verification_code.trim().len() < 4 {
            self.record_failed_attempt(&normalized_email);
            return Err(anyhow!("邮箱验证码无效"));
        }

        if password.trim().len() < 8 {
            self.record_failed_attempt(&normalized_email);
            return Err(anyhow!("密码长度至少8位"));
        }

        {
            let index = self.email_index.read().expect("email index lock poisoned");
            if index.contains_key(&normalized_email) {
                self.record_failed_attempt(&normalized_email);
                return Err(anyhow!("邮箱已注册"));
            }
        }

        let mut user = User::new(
            normalized_email.split('@').next().unwrap_or("user").to_string(),
            Some(normalized_email.clone()),
            None,
        );
        user.status = UserStatus::Active;
        let user_id = self.insert_user(user);

        self.password_hashes
            .write()
            .expect("password hash lock poisoned")
            .insert(user_id, hash_password(&password));

        self.reset_failed_attempts(&normalized_email);
        self.create_authenticated_session(user_id, AuthMethod::Email)
    }

    /// 邮箱密码登录
    pub async fn email_login(&self, email: String, password: String) -> Result<AuthenticatedUser> {
        let normalized_email = normalize_email(&email)?;

        self.ensure_not_locked(&normalized_email)?;

        let user_id = {
            let index = self.email_index.read().expect("email index lock poisoned");
            index.get(&normalized_email).copied()
        }
        .ok_or_else(|| {
            self.record_failed_attempt(&normalized_email);
            anyhow!("用户不存在")
        })?;

        let expected = self
            .password_hashes
            .read()
            .expect("password hash lock poisoned")
            .get(&user_id)
            .cloned()
            .ok_or_else(|| {
                self.record_failed_attempt(&normalized_email);
                anyhow!("用户凭据异常")
            })?;

        if hash_password(&password) != expected {
            self.record_failed_attempt(&normalized_email);
            return Err(anyhow!("邮箱或密码错误"));
        }

        self.reset_failed_attempts(&normalized_email);
        self.create_authenticated_session(user_id, AuthMethod::Email)
    }

    /// 第三方OAuth登录
    pub async fn oauth_login(&self, provider: &str, auth_code: String) -> Result<AuthenticatedUser> {
        log::info!("OAuth login attempt with provider: {}", provider);

        if let Some(provider_impl) = self.providers.get(provider) {
            let mut credentials = HashMap::new();
            credentials.insert("code".to_string(), auth_code);
            return provider_impl.authenticate(&credentials);
        }

        Err(anyhow!("不支持的认证提供商: {}", provider))
    }

    /// 登出
    pub async fn logout(&self, session_id: Uuid) -> Result<()> {
        let mut sessions = self.sessions.write().expect("session lock poisoned");
        let session = sessions
            .get_mut(&session_id)
            .ok_or_else(|| anyhow!("会话不存在"))?;
        session.revoke(Some("user_logout".to_string()));
        Ok(())
    }

    /// 验证会话
    pub async fn validate_session(&self, session_id: Uuid) -> Result<AuthenticatedUser> {
        let mut sessions = self.sessions.write().expect("session lock poisoned");
        let session = sessions
            .get_mut(&session_id)
            .ok_or_else(|| anyhow!("会话不存在"))?;

        if session.status == SessionStatus::Revoked {
            return Err(anyhow!("会话已被撤销"));
        }

        if session.is_expired() {
            session.mark_expired();
            return Err(anyhow!("会话已过期"));
        }

        session.update_access();
        self.authenticated_user_from_session(session)
    }

    /// 刷新会话
    pub async fn refresh_session(&self, session_id: Uuid) -> Result<AuthenticatedUser> {
        let mut sessions = self.sessions.write().expect("session lock poisoned");
        let session = sessions
            .get_mut(&session_id)
            .ok_or_else(|| anyhow!("会话不存在"))?;

        if !session.is_valid() {
            return Err(anyhow!("会话不可刷新"));
        }

        session.refresh(
            new_token(),
            Some(new_token()),
            self.config.session_expiry_hours,
        );

        self.authenticated_user_from_session(session)
    }

    /// 获取当前登录的用户列表
    pub async fn get_active_sessions(&self, user_id: Option<Uuid>) -> Result<Vec<AuthSession>> {
        let sessions = self.sessions.read().expect("session lock poisoned");

        let mut result = sessions
            .values()
            .filter(|session| session.status == SessionStatus::Active && !session.is_expired())
            .filter(|session| user_id.map(|uid| session.user_id == uid).unwrap_or(true))
            .cloned()
            .collect::<Vec<_>>();

        result.sort_by(|a, b| b.last_accessed.cmp(&a.last_accessed));
        Ok(result)
    }

    fn authenticated_user_from_session(&self, session: &AuthSession) -> Result<AuthenticatedUser> {
        let user = self
            .users
            .read()
            .expect("user lock poisoned")
            .get(&session.user_id)
            .cloned()
            .ok_or_else(|| anyhow!("用户不存在"))?;

        Ok(AuthenticatedUser {
            user_id: user.id,
            username: user.username,
            email: user.email,
            phone: user.phone,
            avatar_url: user.avatar_url,
            preferences: user
                .preferences
                .as_object()
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .collect(),
            session_id: session.id,
            expires_at: session.expires_at,
        })
    }

    fn create_authenticated_session(&self, user_id: Uuid, method: AuthMethod) -> Result<AuthenticatedUser> {
        {
            let mut users = self.users.write().expect("user lock poisoned");
            let user = users
                .get_mut(&user_id)
                .ok_or_else(|| anyhow!("用户不存在"))?;
            user.status = UserStatus::Active;
            user.update_login();
        }

        let session = AuthSession::new(
            user_id,
            method,
            "desktop-device".to_string(),
            DeviceType::Desktop,
            "GameCraftDesktop/0.1".to_string(),
            None,
            new_token(),
            Some(new_token()),
            self.config.session_expiry_hours,
        );

        let session_id = session.id;

        self.sessions
            .write()
            .expect("session lock poisoned")
            .insert(session.id, session.clone());

        self.authenticated_user_from_session(&session)
            .map(|mut authenticated| {
                authenticated.session_id = session_id;
                authenticated
            })
    }

    fn insert_user(&self, user: User) -> Uuid {
        let user_id = user.id;

        if let Some(email) = user.email.clone() {
            self.email_index
                .write()
                .expect("email index lock poisoned")
                .insert(email, user_id);
        }

        if let Some(phone) = user.phone.clone() {
            self.phone_index
                .write()
                .expect("phone index lock poisoned")
                .insert(phone, user_id);
        }

        self.users
            .write()
            .expect("user lock poisoned")
            .insert(user_id, user);

        user_id
    }

    fn ensure_not_locked(&self, identifier: &str) -> Result<()> {
        let attempts = self
            .failed_attempts
            .read()
            .expect("failed attempts lock poisoned")
            .get(identifier)
            .copied()
            .unwrap_or(0);

        if attempts >= self.config.max_login_attempts {
            return Err(anyhow!(
                "登录失败次数过多，请在{}分钟后重试",
                self.config.lockout_minutes
            ));
        }

        Ok(())
    }

    fn record_failed_attempt(&self, identifier: &str) {
        let mut attempts = self
            .failed_attempts
            .write()
            .expect("failed attempts lock poisoned");
        let entry = attempts.entry(identifier.to_string()).or_insert(0);
        *entry += 1;
    }

    fn reset_failed_attempts(&self, identifier: &str) {
        self.failed_attempts
            .write()
            .expect("failed attempts lock poisoned")
            .remove(identifier);
    }
}

impl Default for AuthServiceConfig {
    fn default() -> Self {
        Self {
            session_expiry_hours: 720,
            max_login_attempts: 5,
            lockout_minutes: 30,
            require_email_verification: true,
            require_phone_verification: true,
        }
    }
}

fn hash_password(password: &str) -> String {
    blake3::hash(password.as_bytes()).to_hex().to_string()
}

fn normalize_email(email: &str) -> Result<String> {
    let normalized = email.trim().to_lowercase();
    let re = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").expect("email regex invalid");

    if re.is_match(&normalized) {
        Ok(normalized)
    } else {
        Err(anyhow!("邮箱格式无效"))
    }
}

fn normalize_phone(phone: &str) -> Result<String> {
    let normalized = phone.trim().to_string();
    let re = Regex::new(r"^\+?[0-9]{10,16}$").expect("phone regex invalid");

    if re.is_match(&normalized) {
        Ok(normalized)
    } else {
        Err(anyhow!("手机号格式无效"))
    }
}

fn short_code(source: &str) -> String {
    source.chars().take(6).collect::<String>().replace(':', "_")
}

fn new_token() -> String {
    format!("token_{}", Uuid::new_v4())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockOAuthProvider;

    impl AuthProvider for MockOAuthProvider {
        fn name(&self) -> &str {
            "github"
        }

        fn authenticate(&self, credentials: &HashMap<String, String>) -> Result<AuthenticatedUser> {
            if credentials.get("code").is_none() {
                return Err(anyhow!("missing code"));
            }

            Ok(AuthenticatedUser {
                user_id: Uuid::new_v4(),
                username: "oauth_user".to_string(),
                email: Some("oauth@example.com".to_string()),
                phone: None,
                avatar_url: None,
                preferences: HashMap::new(),
                session_id: Uuid::new_v4(),
                expires_at: Utc::now() + chrono::Duration::hours(24),
            })
        }

        fn validate_session(&self, _session_id: &Uuid) -> Result<bool> {
            Ok(true)
        }

        fn logout(&self, _session_id: &Uuid) -> Result<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn email_register_and_login_should_work() {
        let service = AuthService::new(AuthServiceConfig::default());

        let registered = service
            .email_register(
                "tester@example.com".to_string(),
                "12345678".to_string(),
                "1234".to_string(),
            )
            .await
            .expect("register should succeed");

        let logged_in = service
            .email_login("tester@example.com".to_string(), "12345678".to_string())
            .await
            .expect("login should succeed");

        assert_eq!(registered.email, Some("tester@example.com".to_string()));
        assert_eq!(logged_in.email, Some("tester@example.com".to_string()));
    }

    #[tokio::test]
    async fn phone_login_and_session_validation_should_work() {
        let service = AuthService::new(AuthServiceConfig::default());

        let logged_in = service
            .phone_login("+8613800000000".to_string(), "5678".to_string())
            .await
            .expect("phone login should succeed");

        let validated = service
            .validate_session(logged_in.session_id)
            .await
            .expect("session validation should succeed");

        assert_eq!(validated.phone, Some("+8613800000000".to_string()));
    }

    #[tokio::test]
    async fn logout_should_revoke_session() {
        let service = AuthService::new(AuthServiceConfig::default());

        let logged_in = service
            .wechat_login("wechat-abc123".to_string())
            .await
            .expect("wechat login should succeed");

        service
            .logout(logged_in.session_id)
            .await
            .expect("logout should succeed");

        let err = service
            .validate_session(logged_in.session_id)
            .await
            .expect_err("revoked session should fail validation");

        assert!(err.to_string().contains("撤销"));
    }

    #[tokio::test]
    async fn oauth_login_should_delegate_to_provider() {
        let mut service = AuthService::new(AuthServiceConfig::default());
        service.register_provider("github".to_string(), Box::new(MockOAuthProvider));

        let logged_in = service
            .oauth_login("github", "oauth-code".to_string())
            .await
            .expect("oauth login should succeed");

        assert_eq!(logged_in.username, "oauth_user");
    }
}
