use gamecraft_ai_studio::models::game_spec::{ArtStyle, GameType, NarrativeStyle, TargetPlatform};
use gamecraft_ai_studio::services::api_mgmt_service::{APIMgmtConfig, APIManagementService};
use gamecraft_ai_studio::services::audit_service::{AuditEvent, AuditSeverity, AuditService};
use gamecraft_ai_studio::services::auth_service::{AuthService, AuthServiceConfig};
use gamecraft_ai_studio::services::game_config_service::GameConfigService;
use gamecraft_ai_studio::services::user_service::UserService;

#[test]
fn business_services_smoke_test() {
    let _auth = AuthService::new(AuthServiceConfig::default());
    let _api = APIManagementService::new(APIMgmtConfig::default());
    let _game = GameConfigService::new();
    let _user = UserService::new();
    let _audit = AuditService::new();
}

#[tokio::test]
async fn auth_register_login_and_session_flow() {
    let auth = AuthService::new(AuthServiceConfig::default());

    let registered = auth
        .email_register(
            "flow@example.com".to_string(),
            "12345678".to_string(),
            "1234".to_string(),
        )
        .await
        .expect("register should succeed");

    let validated = auth
        .validate_session(registered.session_id)
        .await
        .expect("session should be valid");

    assert_eq!(validated.email, Some("flow@example.com".to_string()));
}

#[test]
fn game_config_and_audit_flow() {
    let game = GameConfigService::new();
    let audit = AuditService::new();
    let project_id = uuid::Uuid::new_v4();

    let created = game
        .create_draft(
            project_id,
            GameType::Rpg,
            ArtStyle::Pixel,
            NarrativeStyle::Epic,
            TargetPlatform::Pc,
        )
        .expect("draft should be created");

    assert_eq!(created.project_id, project_id);

    let event_id = audit.record(AuditEvent::new(
        None,
        Some(project_id),
        "game.spec.created".to_string(),
        "草稿配置已创建".to_string(),
        AuditSeverity::Info,
        serde_json::json!({ "project_id": project_id }),
    ));

    assert_ne!(event_id, uuid::Uuid::nil());
}
