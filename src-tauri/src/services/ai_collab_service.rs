//! AI协作服务模块
//! 处理AI生成和协作的业务逻辑

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use anyhow::{anyhow, Result};
use regex::Regex;
use serde_json::Value;
use uuid::Uuid;

use crate::providers::{AIGenerationRequest, AIGenerationResponse, AIProvider, ContentType};

/// AI协作服务
pub struct AICollabService {
    providers: RwLock<HashMap<String, Arc<dyn AIProvider>>>,
    history: RwLock<Vec<AIGenerationResponse>>,
    default_provider: RwLock<Option<String>>,
    max_history: usize,
}

impl AICollabService {
    pub fn new() -> Self {
        Self {
            providers: RwLock::new(HashMap::new()),
            history: RwLock::new(Vec::new()),
            default_provider: RwLock::new(None),
            max_history: 500,
        }
    }

    pub fn register_provider(&self, name: String, provider: Arc<dyn AIProvider>) {
        self.providers
            .write()
            .expect("provider lock poisoned")
            .insert(name.clone(), provider);

        let mut default = self
            .default_provider
            .write()
            .expect("default provider lock poisoned");

        if default.is_none() {
            *default = Some(name);
        }
    }

    pub fn set_default_provider(&self, provider: String) -> Result<()> {
        let providers = self.providers.read().expect("provider lock poisoned");

        if !providers.contains_key(&provider) {
            return Err(anyhow!("默认提供商不存在: {}", provider));
        }

        *self
            .default_provider
            .write()
            .expect("default provider lock poisoned") = Some(provider);

        Ok(())
    }

    pub async fn generate(&self, request: AIGenerationRequest) -> Result<AIGenerationResponse> {
        let provider_name = self.choose_provider(&request)?;

        let provider = self
            .providers
            .read()
            .expect("provider lock poisoned")
            .get(&provider_name)
            .cloned()
            .ok_or_else(|| anyhow!("提供商不存在: {}", provider_name))?;

        let mut response = provider.generate(request).await?;
        response.content = sanitize_response(&response.content);

        self.push_history(response.clone());
        Ok(response)
    }

    pub async fn regenerate(
        &self,
        generation_id: Uuid,
        modifications: Value,
    ) -> Result<AIGenerationResponse> {
        let original = self
            .history
            .read()
            .expect("history lock poisoned")
            .iter()
            .find(|item| item.id == generation_id)
            .cloned()
            .ok_or_else(|| anyhow!("历史记录不存在"))?;

        let extra_prompt = modifications
            .get("instruction")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();

        let mut new_request = original.request.clone();
        if !extra_prompt.trim().is_empty() {
            new_request.prompt = format!("{}\n\n[修改要求]\n{}", new_request.prompt, extra_prompt);
        }

        self.generate(new_request).await
    }

    pub fn get_generation_history(
        &self,
        project_id: Uuid,
        content_type: Option<ContentType>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Vec<AIGenerationResponse> {
        let history = self.history.read().expect("history lock poisoned");

        let mut data = history
            .iter()
            .filter(|item| item.request.project_id == project_id)
            .filter(|item| {
                content_type
                    .as_ref()
                    .map(|t| same_content_type(&item.request.content_type, t))
                    .unwrap_or(true)
            })
            .cloned()
            .collect::<Vec<_>>();

        data.sort_by(|a, b| b.generated_at.cmp(&a.generated_at));

        let start = offset.unwrap_or(0) as usize;
        let take = limit.unwrap_or(data.len() as u32) as usize;

        data.into_iter().skip(start).take(take).collect()
    }

    pub fn clear_project_history(&self, project_id: Uuid) {
        let mut history = self.history.write().expect("history lock poisoned");
        history.retain(|item| item.request.project_id != project_id);
    }

    fn choose_provider(&self, request: &AIGenerationRequest) -> Result<String> {
        if let Some(preferred) = request.provider_preference.as_ref() {
            if self
                .providers
                .read()
                .expect("provider lock poisoned")
                .contains_key(preferred)
            {
                return Ok(preferred.clone());
            }
        }

        self.default_provider
            .read()
            .expect("default provider lock poisoned")
            .clone()
            .ok_or_else(|| anyhow!("未配置默认提供商"))
    }

    fn push_history(&self, response: AIGenerationResponse) {
        let mut history = self.history.write().expect("history lock poisoned");
        history.push(response);

        if history.len() > self.max_history {
            let overflow = history.len() - self.max_history;
            history.drain(0..overflow);
        }
    }
}

fn sanitize_response(content: &str) -> String {
    let regex = Regex::new(r"(?s)<thinking>.*?</thinking>").expect("thinking regex invalid");
    regex.replace_all(content, "").trim().to_string()
}

fn same_content_type(a: &ContentType, b: &ContentType) -> bool {
    match (a, b) {
        (ContentType::Character, ContentType::Character)
        | (ContentType::Scene, ContentType::Scene)
        | (ContentType::Dialogue, ContentType::Dialogue)
        | (ContentType::Item, ContentType::Item)
        | (ContentType::Quest, ContentType::Quest)
        | (ContentType::Mechanism, ContentType::Mechanism) => true,
        (ContentType::Other(x), ContentType::Other(y)) => x == y,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use chrono::Utc;

    struct MockProvider;

    #[async_trait]
    impl AIProvider for MockProvider {
        fn name(&self) -> &str {
            "mock"
        }

        fn display_name(&self) -> &str {
            "Mock"
        }

        async fn is_available(&self) -> bool {
            true
        }

        async fn generate(&self, request: AIGenerationRequest) -> Result<AIGenerationResponse> {
            Ok(AIGenerationResponse {
                id: Uuid::new_v4(),
                request,
                content: "<thinking>internal chain</thinking>最终输出内容".to_string(),
                provider_used: "mock".to_string(),
                tokens_used: 100,
                cost: 0.02,
                generated_at: Utc::now(),
                metadata: serde_json::json!({}),
            })
        }

        fn get_config(&self) -> crate::providers::ProviderConfig {
            crate::providers::ProviderConfig {
                api_key: None,
                base_url: None,
                default_model: "mock".to_string(),
                max_tokens: 1024,
                temperature: 0.7,
                timeout_seconds: 30,
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

    fn sample_request() -> AIGenerationRequest {
        AIGenerationRequest {
            project_id: Uuid::new_v4(),
            content_type: ContentType::Dialogue,
            prompt: "生成一段对话".to_string(),
            context: serde_json::json!({}),
            provider_preference: Some("mock".to_string()),
            max_tokens: Some(512),
            temperature: Some(0.7),
        }
    }

    #[tokio::test]
    async fn generate_should_strip_thinking_content() {
        let service = AICollabService::new();
        service.register_provider("mock".to_string(), Arc::new(MockProvider));

        let response = service.generate(sample_request()).await.expect("generate should succeed");

        assert_eq!(response.content, "最终输出内容");
    }

    #[tokio::test]
    async fn regenerate_should_use_history() {
        let service = AICollabService::new();
        service.register_provider("mock".to_string(), Arc::new(MockProvider));

        let first = service.generate(sample_request()).await.expect("first generation should succeed");
        let second = service
            .regenerate(first.id, serde_json::json!({ "instruction": "更简洁" }))
            .await
            .expect("regeneration should succeed");

        assert_ne!(first.id, second.id);
    }
}
