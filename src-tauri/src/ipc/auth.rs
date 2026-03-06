//! 认证接口模块
//! 处理用户登录、注册、会话管理等认证相关功能

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::services::ServiceContainer;

/// 登录方法枚举
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LoginMethod {
    WechatQr,
    PhoneCode,
    EmailPassword,
    OAuth(String),
}

/// 登录请求
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginRequest {
    pub method: LoginMethod,
    pub credentials: HashMap<String, String>,
}

/// 用户信息
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserInfo {
    pub id: Uuid,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub username: String,
    pub avatar_url: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub preferences: HashMap<String, serde_json::Value>,
}

/// 认证响应
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthResponse {
    pub success: bool,
    pub user: Option<UserInfo>,
    pub token: Option<String>,
    pub error: Option<String>,
}

fn ok_response(user: UserInfo, token: String) -> AuthResponse {
    AuthResponse {
        success: true,
        user: Some(user),
        token: Some(token),
        error: None,
    }
}

fn fail_response(message: String) -> AuthResponse {
    AuthResponse {
        success: false,
        user: None,
        token: None,
        error: Some(message),
    }
}

fn to_user_info(authenticated: &crate::services::auth_service::AuthenticatedUser) -> UserInfo {
    UserInfo {
        id: authenticated.user_id,
        email: authenticated.email.clone(),
        phone: authenticated.phone.clone(),
        username: authenticated.username.clone(),
        avatar_url: authenticated.avatar_url.clone(),
        created_at: chrono::Utc::now(),
        preferences: authenticated.preferences.clone(),
    }
}

#[tauri::command]
pub async fn login(app_handle: AppHandle, request: LoginRequest) -> Result<AuthResponse, String> {
    let services = app_handle.state::<ServiceContainer>();

    let result = match request.method {
        LoginMethod::WechatQr => {
            let code = request
                .credentials
                .get("auth_code")
                .or_else(|| request.credentials.get("code"))
                .cloned()
                .unwrap_or_default();
            services.auth_service.wechat_login(code).await
        }
        LoginMethod::PhoneCode => {
            let phone = request
                .credentials
                .get("phone")
                .cloned()
                .unwrap_or_default();
            let code = request
                .credentials
                .get("code")
                .cloned()
                .unwrap_or_default();
            services.auth_service.phone_login(phone, code).await
        }
        LoginMethod::EmailPassword => {
            let email = request
                .credentials
                .get("email")
                .cloned()
                .unwrap_or_default();
            let password = request
                .credentials
                .get("password")
                .cloned()
                .unwrap_or_default();
            services.auth_service.email_login(email, password).await
        }
        LoginMethod::OAuth(provider) => {
            let code = request
                .credentials
                .get("code")
                .cloned()
                .unwrap_or_default();
            services.auth_service.oauth_login(&provider, code).await
        }
    };

    match result {
        Ok(authenticated) => Ok(ok_response(
            to_user_info(&authenticated),
            authenticated.session_id.to_string(),
        )),
        Err(error) => Ok(fail_response(error.to_string())),
    }
}

#[tauri::command]
pub async fn logout(app_handle: AppHandle, session_id: Uuid) -> Result<bool, String> {
    let services = app_handle.state::<ServiceContainer>();
    services
        .auth_service
        .logout(session_id)
        .await
        .map(|_| true)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn register_email(
    app_handle: AppHandle,
    email: String,
    password: String,
    verification_code: String,
) -> Result<AuthResponse, String> {
    let services = app_handle.state::<ServiceContainer>();

    match services
        .auth_service
        .email_register(email, password, verification_code)
        .await
    {
        Ok(authenticated) => Ok(ok_response(
            to_user_info(&authenticated),
            authenticated.session_id.to_string(),
        )),
        Err(error) => Ok(fail_response(error.to_string())),
    }
}

#[tauri::command]
pub async fn validate_session(app_handle: AppHandle, token: String) -> Result<UserInfo, String> {
    let services = app_handle.state::<ServiceContainer>();

    let session_id = Uuid::parse_str(token.trim()).map_err(|_| "无效会话令牌".to_string())?;

    services
        .auth_service
        .validate_session(session_id)
        .await
        .map(|authenticated| to_user_info(&authenticated))
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn get_current_user(app_handle: AppHandle) -> Result<UserInfo, String> {
    let services = app_handle.state::<ServiceContainer>();

    let sessions = services
        .auth_service
        .get_active_sessions(None)
        .await
        .map_err(|error| error.to_string())?;

    let latest = sessions
        .first()
        .ok_or_else(|| "当前没有活跃会话".to_string())?;

    services
        .auth_service
        .validate_session(latest.id)
        .await
        .map(|authenticated| to_user_info(&authenticated))
        .map_err(|error| error.to_string())
}
