//! 百度文心提供商实现

use async_trait::async_trait;
use anyhow::Result;

use super::{AIProvider, AIGenerationRequest, AIGenerationResponse, ProviderConfig, ProviderStats};

/// 百度文心提供商
pub struct BaiduAIProvider {
    config: ProviderConfig,
    stats: ProviderStats,
}

impl BaiduAIProvider {
    pub fn new(config: ProviderConfig) -> Self {
        Self {
            config,
            stats: ProviderStats {
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                total_tokens: 0,
                total_cost: 0.0,
                avg_response_time_ms: 0.0,
                last_used: None,
            },
        }
    }
}

#[async_trait]
impl AIProvider for BaiduAIProvider {
    fn name(&self) -> &str {
        "baidu"
    }

    fn display_name(&self) -> &str {
        "百度文心"
    }

    async fn is_available(&self) -> bool {
        // TODO: 实现可用性检查
        self.config.enabled && self.config.api_key.is_some()
    }

    async fn generate(&self, request: AIGenerationRequest) -> Result<AIGenerationResponse> {
        // TODO: 实现百度文心API调用
        Err(anyhow::anyhow!("Baidu AI provider not implemented"))
    }

    fn get_config(&self) -> ProviderConfig {
        self.config.clone()
    }

    async fn get_stats(&self) -> ProviderStats {
        self.stats.clone()
    }

    async fn test_connection(&self) -> Result<()> {
        // TODO: 实现连接测试
        Err(anyhow::anyhow!("Baidu AI connection test not implemented"))
    }
}
