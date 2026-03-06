//! 服务模块
//! 包含所有业务逻辑服务

pub mod auth_service;
pub mod api_mgmt_service;
pub mod game_config_service;
pub mod ai_collab_service;
pub mod user_service;
pub mod audit_service;
pub mod provider_manager;

use tauri::{AppHandle, Manager};
use anyhow::Result;
use std::sync::Arc;

use self::ai_collab_service::AICollabService;
use self::api_mgmt_service::{APIMgmtConfig, APIManagementService};
use self::audit_service::AuditService;
use self::auth_service::{AuthService, AuthServiceConfig};
use self::game_config_service::GameConfigService;
use self::user_service::UserService;

/// 服务容器
pub struct ServiceContainer {
    pub auth_service: Arc<AuthService>,
    pub api_mgmt_service: Arc<APIManagementService>,
    pub game_config_service: Arc<GameConfigService>,
    pub ai_collab_service: Arc<AICollabService>,
    pub user_service: Arc<UserService>,
    pub audit_service: Arc<AuditService>,
}

impl ServiceContainer {
    pub fn new() -> Self {
        Self {
            auth_service: Arc::new(AuthService::new(AuthServiceConfig::default())),
            api_mgmt_service: Arc::new(APIManagementService::new(APIMgmtConfig::default())),
            game_config_service: Arc::new(GameConfigService::new()),
            ai_collab_service: Arc::new(AICollabService::new()),
            user_service: Arc::new(UserService::new()),
            audit_service: Arc::new(AuditService::new()),
        }
    }
}

/// 初始化所有服务
pub fn init(app: &AppHandle) -> Result<()> {
    log::info!("Initializing services...");

    app.manage(ServiceContainer::new());
    log::info!("Services initialized");

    Ok(())
}
