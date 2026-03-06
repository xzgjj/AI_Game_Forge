use gamecraft_ai_studio::database::repository::{Repository, RepositoryManager, UserRepository};
use gamecraft_ai_studio::models::{AuthMethod, AuthSession, DeviceType, User};

#[test]
fn data_layer_user_repository_smoke() {
    let repo = UserRepository::new_in_memory();
    let user = User::new("dl_user".to_string(), Some("dl@example.com".to_string()), None);

    let created = repo.create(user.clone()).expect("create should succeed");
    let found = repo
        .find_by_id(created.id)
        .expect("find should succeed")
        .expect("user should exist");

    assert_eq!(found.username, "dl_user");
}

#[test]
fn data_layer_manager_smoke() {
    let manager = RepositoryManager::new_in_memory();
    let user = User::new("manager_user".to_string(), None, Some("+8613800000000".to_string()));

    manager
        .users
        .create(user.clone())
        .expect("create user should succeed");

    let session = AuthSession::new(
        user.id,
        AuthMethod::Phone,
        "device-1".to_string(),
        DeviceType::Desktop,
        "ua".to_string(),
        None,
        "token".to_string(),
        None,
        24,
    );

    manager
        .auth_sessions
        .create(session)
        .expect("create session should succeed");

    let active = manager
        .auth_sessions
        .find_active_by_user_id(user.id)
        .expect("find active sessions should succeed");

    assert_eq!(active.len(), 1);
}
