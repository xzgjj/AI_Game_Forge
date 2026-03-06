//! AI引擎接口模块
//! 处理AI内容生成、重新生成、历史查询等操作

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::providers;
use crate::services::ServiceContainer;

/// 生成内容类型
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ContentType {
    Character,
    Scene,
    Dialogue,
    Item,
    Quest,
    Mechanism,
    Other(String),
}

/// AI生成请求
#[derive(Serialize, Deserialize, Debug, Clone)]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
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

/// 生成历史记录
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenerationHistory {
    pub id: Uuid,
    pub project_id: Uuid,
    pub content_type: ContentType,
    pub prompt: String,
    pub content: String,
    pub provider: String,
    pub tokens: u32,
    pub cost: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

fn to_provider_content_type(content_type: ContentType) -> providers::ContentType {
    match content_type {
        ContentType::Character => providers::ContentType::Character,
        ContentType::Scene => providers::ContentType::Scene,
        ContentType::Dialogue => providers::ContentType::Dialogue,
        ContentType::Item => providers::ContentType::Item,
        ContentType::Quest => providers::ContentType::Quest,
        ContentType::Mechanism => providers::ContentType::Mechanism,
        ContentType::Other(value) => providers::ContentType::Other(value),
    }
}

fn from_provider_content_type(content_type: providers::ContentType) -> ContentType {
    match content_type {
        providers::ContentType::Character => ContentType::Character,
        providers::ContentType::Scene => ContentType::Scene,
        providers::ContentType::Dialogue => ContentType::Dialogue,
        providers::ContentType::Item => ContentType::Item,
        providers::ContentType::Quest => ContentType::Quest,
        providers::ContentType::Mechanism => ContentType::Mechanism,
        providers::ContentType::Other(value) => ContentType::Other(value),
    }
}

fn to_provider_request(request: AIGenerationRequest) -> providers::AIGenerationRequest {
    providers::AIGenerationRequest {
        project_id: request.project_id,
        content_type: to_provider_content_type(request.content_type),
        prompt: request.prompt,
        context: request.context,
        provider_preference: request.provider_preference,
        max_tokens: request.max_tokens,
        temperature: request.temperature,
    }
}

fn from_provider_response(response: providers::AIGenerationResponse) -> AIGenerationResponse {
    AIGenerationResponse {
        id: response.id,
        request: AIGenerationRequest {
            project_id: response.request.project_id,
            content_type: from_provider_content_type(response.request.content_type),
            prompt: response.request.prompt,
            context: response.request.context,
            provider_preference: response.request.provider_preference,
            max_tokens: response.request.max_tokens,
            temperature: response.request.temperature,
        },
        content: response.content,
        provider_used: response.provider_used,
        tokens_used: response.tokens_used,
        cost: response.cost,
        generated_at: response.generated_at,
        metadata: response.metadata,
    }
}

#[tauri::command]
pub async fn generate_content(
    app_handle: AppHandle,
    request: AIGenerationRequest,
) -> Result<AIGenerationResponse, String> {
    let services = app_handle.state::<ServiceContainer>();

    services
        .ai_collab_service
        .generate(to_provider_request(request))
        .await
        .map(from_provider_response)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn regenerate_content(
    app_handle: AppHandle,
    generation_id: Uuid,
    modifications: serde_json::Value,
) -> Result<AIGenerationResponse, String> {
    let services = app_handle.state::<ServiceContainer>();

    services
        .ai_collab_service
        .regenerate(generation_id, modifications)
        .await
        .map(from_provider_response)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn get_generation_history(
    app_handle: AppHandle,
    project_id: Uuid,
    content_type: Option<ContentType>,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<Vec<GenerationHistory>, String> {
    let services = app_handle.state::<ServiceContainer>();

    let history = services.ai_collab_service.get_generation_history(
        project_id,
        content_type.map(to_provider_content_type),
        limit,
        offset,
    );

    Ok(history
        .into_iter()
        .map(|item| GenerationHistory {
            id: item.id,
            project_id: item.request.project_id,
            content_type: from_provider_content_type(item.request.content_type),
            prompt: item.request.prompt,
            content: item.content,
            provider: item.provider_used,
            tokens: item.tokens_used,
            cost: item.cost,
            created_at: item.generated_at,
        })
        .collect())
}

#[tauri::command]
pub async fn get_provider_status(app_handle: AppHandle) -> Result<serde_json::Value, String> {
    let services = app_handle.state::<ServiceContainer>();
    let status = services.api_mgmt_service.get_provider_status().await;
    Ok(serde_json::to_value(status).unwrap_or_else(|_| serde_json::json!({})))
}
