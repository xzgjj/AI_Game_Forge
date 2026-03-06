//! AI提供商模块
//! 定义AI提供商的统一接口和具体实现

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use anyhow::Result;

/// AI生成请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIGenerationRequest {
    pub project_id: Uuid,
    pub content_type: ContentType,
    pub prompt: String,
    pub context: serde_json::Value,
    pub provider_preference: Option<String>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

/// AI生成响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIGenerationResponse {
    pub id: Uuid,
    pub request: AIGenerationRequest,
    pub content: String,
    pub provider_used: String,
    pub tokens_used: u32,
    pub cost: f64,
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub metadata: serde_json::Value,
}

/// 生成内容类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    Character,   // 角色设计
    Scene,       // 场景描述
    Dialogue,    // 对话台词
    Item,        // 物品描述
    Quest,       // 任务设计
    Mechanism,   // 机制设计
    Other(String), // 其他类型
}

/// AI提供商统一接口
#[async_trait]
pub trait AIProvider: Send + Sync {
    /// 提供商名称
    fn name(&self) -> &str;

    /// 提供商显示名称
    fn display_name(&self) -> &str;

    /// 检查提供商是否可用
    async fn is_available(&self) -> bool;

    /// 生成内容
    async fn generate(&self, request: AIGenerationRequest) -> Result<AIGenerationResponse>;

    /// 获取提供商配置
    fn get_config(&self) -> ProviderConfig;

    /// 获取提供商统计
    async fn get_stats(&self) -> ProviderStats;

    /// 测试连接
    async fn test_connection(&self) -> Result<()>;
}

/// 提供商配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub default_model: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub timeout_seconds: u64,
    pub cost_per_1k_input: f64,
    pub cost_per_1k_output: f64,
    pub enabled: bool,
}

/// 提供商统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderStats {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub total_tokens: u64,
    pub total_cost: f64,
    pub avg_response_time_ms: f64,
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
}

// 重新导出具体提供商实现
#[cfg(feature = "openai")]
pub mod openai;
#[cfg(feature = "claude")]
pub mod claude;
#[cfg(feature = "zhipu")]
pub mod zhipu;
#[cfg(feature = "baidu")]
pub mod baidu;
#[cfg(feature = "local")]
pub mod local;

/// 提供商工厂
pub struct ProviderFactory;

impl ProviderFactory {
    /// 创建提供商实例
    pub fn create_provider(name: &str, config: ProviderConfig) -> Result<Box<dyn AIProvider>> {
        match name.to_lowercase().as_str() {
            #[cfg(feature = "openai")]
            "openai" => Ok(Box::new(openai::OpenAIProvider::new(config))),
            #[cfg(feature = "claude")]
            "claude" => Ok(Box::new(claude::ClaudeProvider::new(config))),
            #[cfg(feature = "zhipu")]
            "zhipu" => Ok(Box::new(zhipu::ZhipuAIProvider::new(config))),
            #[cfg(feature = "baidu")]
            "baidu" => Ok(Box::new(baidu::BaiduAIProvider::new(config))),
            #[cfg(feature = "local")]
            "local" => Ok(Box::new(local::LocalAIProvider::new(config))),
            _ => Err(anyhow::anyhow!("Unsupported provider: {}", name)),
        }
    }
}
