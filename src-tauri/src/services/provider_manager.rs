//! 提供商管理器模块
//! 管理AI提供商的注册、发现和健康检查

use std::collections::HashMap;
use std::sync::Arc;
use crate::providers::AIProvider;

/// 提供商管理器
pub struct ProviderManager {
    providers: HashMap<String, Arc<dyn AIProvider>>,
}

impl ProviderManager {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    pub fn register_provider(&mut self, name: String, provider: Arc<dyn AIProvider>) {
        self.providers.insert(name, provider);
    }

    pub fn get_provider(&self, name: &str) -> Option<Arc<dyn AIProvider>> {
        self.providers.get(name).cloned()
    }

    pub fn list_providers(&self) -> Vec<String> {
        self.providers.keys().cloned().collect()
    }

    pub async fn health_check(&self) -> HashMap<String, bool> {
        let mut results = HashMap::new();

        for (name, provider) in &self.providers {
            let available = provider.is_available().await;
            results.insert(name.clone(), available);
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use async_trait::async_trait;
    use chrono::Utc;
    use uuid::Uuid;

    struct MockProvider {
        available: bool,
    }

    #[async_trait]
    impl AIProvider for MockProvider {
        fn name(&self) -> &str {
            "mock"
        }

        fn display_name(&self) -> &str {
            "Mock"
        }

        async fn is_available(&self) -> bool {
            self.available
        }

        async fn generate(
            &self,
            request: crate::providers::AIGenerationRequest,
        ) -> Result<crate::providers::AIGenerationResponse> {
            Ok(crate::providers::AIGenerationResponse {
                id: Uuid::new_v4(),
                request,
                content: "ok".to_string(),
                provider_used: "mock".to_string(),
                tokens_used: 1,
                cost: 0.0,
                generated_at: Utc::now(),
                metadata: serde_json::json!({}),
            })
        }

        fn get_config(&self) -> crate::providers::ProviderConfig {
            crate::providers::ProviderConfig {
                api_key: None,
                base_url: None,
                default_model: "mock".to_string(),
                max_tokens: 128,
                temperature: 0.7,
                timeout_seconds: 5,
                cost_per_1k_input: 0.0,
                cost_per_1k_output: 0.0,
                enabled: true,
            }
        }

        async fn get_stats(&self) -> crate::providers::ProviderStats {
            crate::providers::ProviderStats {
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

    #[tokio::test]
    async fn health_check_should_reflect_provider_status() {
        let mut manager = ProviderManager::new();
        manager.register_provider(
            "mock".to_string(),
            Arc::new(MockProvider { available: true }),
        );

        let result = manager.health_check().await;
        assert_eq!(result.get("mock"), Some(&true));
    }
}
