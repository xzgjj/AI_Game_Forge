use gamecraft_ai_studio::providers::{AIGenerationRequest, ContentType};
use gamecraft_ai_studio::services::api_mgmt_service::{APIMgmtConfig, APIManagementService};
use gamecraft_ai_studio::services::project_service::ProjectService;

#[test]
fn project_service_smoke_flow() {
    let service = ProjectService::new();

    let project = service
        .create_project("Infra Demo".to_string(), Some("test".to_string()), None)
        .expect("project should be created");

    let version = service
        .save_project(
            project.id,
            Some("save 1".to_string()),
            serde_json::json!({ "step": "infra-save" }),
        )
        .expect("project should be saved");

    let loaded = service
        .load_project(project.id, Some(version.version))
        .expect("project should load");

    assert_eq!(loaded["project"]["id"], project.id.to_string());
}

#[tokio::test]
async fn api_stats_provider_registry_smoke() {
    let service = APIManagementService::new(APIMgmtConfig::default());
    let usage = service
        .get_usage_stats(gamecraft_ai_studio::models::api_stats::StatsPeriod::Day)
        .await
        .expect("usage stats should be retrievable");

    assert_eq!(usage.total_requests, 0);
}

#[test]
fn generation_request_shape_smoke() {
    let request = AIGenerationRequest {
        project_id: uuid::Uuid::new_v4(),
        content_type: ContentType::Character,
        prompt: "生成角色".to_string(),
        context: serde_json::json!({}),
        provider_preference: Some("demo".to_string()),
        max_tokens: Some(256),
        temperature: Some(0.6),
    };

    assert_eq!(request.max_tokens, Some(256));
}
