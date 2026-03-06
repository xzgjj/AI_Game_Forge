//! 服务模块
//! 包含所有业务逻辑服务

pub mod auth_service;
pub mod api_mgmt_service;
pub mod game_config_service;
pub mod ai_collab_service;
pub mod user_service;
pub mod audit_service;
pub mod provider_manager;
pub mod project_service;

use tauri::{AppHandle, Manager};
use anyhow::Result;
use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

use self::ai_collab_service::AICollabService;
use self::api_mgmt_service::{APIMgmtConfig, APIManagementService};
use self::audit_service::AuditService;
use self::auth_service::{AuthService, AuthServiceConfig};
use self::game_config_service::GameConfigService;
use self::project_service::ProjectService;
use self::user_service::UserService;
use crate::providers::{AIGenerationRequest, AIGenerationResponse, AIProvider, ProviderConfig, ProviderStats};

/// 服务容器
pub struct ServiceContainer {
    pub auth_service: Arc<AuthService>,
    pub api_mgmt_service: Arc<APIManagementService>,
    pub game_config_service: Arc<GameConfigService>,
    pub project_service: Arc<ProjectService>,
    pub ai_collab_service: Arc<AICollabService>,
    pub user_service: Arc<UserService>,
    pub audit_service: Arc<AuditService>,
}

impl ServiceContainer {
    pub fn new() -> Self {
        let demo_provider = Arc::new(DemoAIProvider::new());

        let mut api_config = APIMgmtConfig::default();
        api_config.default_provider = "demo".to_string();
        api_config.fallback_order = vec!["demo".to_string()];

        let mut api_mgmt_service = APIManagementService::new(api_config);
        api_mgmt_service.register_provider("demo".to_string(), demo_provider.clone());

        let ai_collab_service = AICollabService::new();
        ai_collab_service.register_provider("demo".to_string(), demo_provider);

        Self {
            auth_service: Arc::new(AuthService::new(AuthServiceConfig::default())),
            api_mgmt_service: Arc::new(api_mgmt_service),
            game_config_service: Arc::new(GameConfigService::new()),
            project_service: Arc::new(ProjectService::new()),
            ai_collab_service: Arc::new(ai_collab_service),
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

struct DemoAIProvider;

impl DemoAIProvider {
    fn new() -> Self {
        Self
    }
}

#[async_trait]
impl AIProvider for DemoAIProvider {
    fn name(&self) -> &str {
        "demo"
    }

    fn display_name(&self) -> &str {
        "Demo Provider"
    }

    async fn is_available(&self) -> bool {
        true
    }

    async fn generate(&self, request: AIGenerationRequest) -> Result<AIGenerationResponse> {
        let preview = request.prompt.chars().take(48).collect::<String>();
        Ok(AIGenerationResponse {
            id: Uuid::new_v4(),
            request,
            content: format!("已生成演示内容：{}", preview),
            provider_used: "demo".to_string(),
            tokens_used: 128,
            cost: 0.0,
            generated_at: Utc::now(),
            metadata: serde_json::json!({
                "mode": "demo",
                "notes": "This provider is for local demo flow only."
            }),
        })
    }

    fn get_config(&self) -> ProviderConfig {
        ProviderConfig {
            api_key: None,
            base_url: None,
            default_model: "demo-model".to_string(),
            max_tokens: 2048,
            temperature: 0.7,
            timeout_seconds: 30,
            cost_per_1k_input: 0.0,
            cost_per_1k_output: 0.0,
            enabled: true,
        }
    }

    async fn get_stats(&self) -> ProviderStats {
        ProviderStats {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            total_tokens: 0,
            total_cost: 0.0,
            avg_response_time_ms: 0.0,
            last_used: None,
        }
    }

    async fn test_connection(&self) -> Result<()> {
        Ok(())
    }
}
