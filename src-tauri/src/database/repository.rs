//! 仓储模块
//! 定义数据访问层接口和最小可用实现

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use anyhow::{anyhow, Result};
use uuid::Uuid;

use crate::database::ConnectionPool;
use crate::models::{AILog, APIStatRecord, AuthSession, GameSpec, Project, SessionStatus, User};

/// 基础仓储接口
pub trait Repository<T: Clone + Send + Sync + 'static> {
    fn create(&self, item: T) -> Result<T>;
    fn find_by_id(&self, id: Uuid) -> Result<Option<T>>;
    fn find_all(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<T>>;
    fn update(&self, id: Uuid, updates: T) -> Result<T>;
    fn delete(&self, id: Uuid) -> Result<bool>;
    fn count(&self) -> Result<i64>;
}

#[derive(Default)]
struct InMemoryTable<T: Clone> {
    items: RwLock<HashMap<Uuid, T>>,
}

impl<T: Clone> InMemoryTable<T> {
    fn insert(&self, id: Uuid, item: T) {
        self.items
            .write()
            .expect("table lock poisoned")
            .insert(id, item);
    }

    fn get(&self, id: &Uuid) -> Option<T> {
        self.items
            .read()
            .expect("table lock poisoned")
            .get(id)
            .cloned()
    }

    fn remove(&self, id: &Uuid) -> bool {
        self.items
            .write()
            .expect("table lock poisoned")
            .remove(id)
            .is_some()
    }

    fn values(&self) -> Vec<T> {
        self.items
            .read()
            .expect("table lock poisoned")
            .values()
            .cloned()
            .collect()
    }

    fn len(&self) -> usize {
        self.items.read().expect("table lock poisoned").len()
    }
}

fn paginate<T>(mut data: Vec<T>, limit: Option<i64>, offset: Option<i64>) -> Vec<T> {
    let start = offset.unwrap_or(0).max(0) as usize;
    let take = limit.unwrap_or(data.len() as i64).max(0) as usize;

    if start >= data.len() {
        return Vec::new();
    }

    data.drain(0..start);
    data.into_iter().take(take).collect()
}

/// 用户仓储
pub struct UserRepository {
    #[allow(dead_code)]
    pool: Option<Arc<ConnectionPool>>,
    table: InMemoryTable<User>,
}

impl UserRepository {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        Self {
            pool: Some(pool),
            table: InMemoryTable::default(),
        }
    }

    pub fn new_in_memory() -> Self {
        Self {
            pool: None,
            table: InMemoryTable::default(),
        }
    }

    pub fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        Ok(self
            .table
            .values()
            .into_iter()
            .find(|user| user.email.as_deref() == Some(email)))
    }

    pub fn find_by_phone(&self, phone: &str) -> Result<Option<User>> {
        Ok(self
            .table
            .values()
            .into_iter()
            .find(|user| user.phone.as_deref() == Some(phone)))
    }
}

impl Repository<User> for UserRepository {
    fn create(&self, item: User) -> Result<User> {
        self.table.insert(item.id, item.clone());
        Ok(item)
    }

    fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        Ok(self.table.get(&id))
    }

    fn find_all(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<User>> {
        let mut users = self.table.values();
        users.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        Ok(paginate(users, limit, offset))
    }

    fn update(&self, id: Uuid, updates: User) -> Result<User> {
        if self.table.get(&id).is_none() {
            return Err(anyhow!("用户不存在"));
        }

        self.table.insert(id, updates.clone());
        Ok(updates)
    }

    fn delete(&self, id: Uuid) -> Result<bool> {
        Ok(self.table.remove(&id))
    }

    fn count(&self) -> Result<i64> {
        Ok(i64::try_from(self.table.len()).unwrap_or(i64::MAX))
    }
}

/// 项目仓储
pub struct ProjectRepository {
    #[allow(dead_code)]
    pool: Option<Arc<ConnectionPool>>,
    table: InMemoryTable<Project>,
}

impl ProjectRepository {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        Self {
            pool: Some(pool),
            table: InMemoryTable::default(),
        }
    }

    pub fn new_in_memory() -> Self {
        Self {
            pool: None,
            table: InMemoryTable::default(),
        }
    }

    pub fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Project>> {
        let mut projects = self
            .table
            .values()
            .into_iter()
            .filter(|project| project.user_id == user_id)
            .collect::<Vec<_>>();
        projects.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        Ok(projects)
    }
}

impl Repository<Project> for ProjectRepository {
    fn create(&self, item: Project) -> Result<Project> {
        self.table.insert(item.id, item.clone());
        Ok(item)
    }

    fn find_by_id(&self, id: Uuid) -> Result<Option<Project>> {
        Ok(self.table.get(&id))
    }

    fn find_all(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<Project>> {
        let mut projects = self.table.values();
        projects.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        Ok(paginate(projects, limit, offset))
    }

    fn update(&self, id: Uuid, updates: Project) -> Result<Project> {
        if self.table.get(&id).is_none() {
            return Err(anyhow!("项目不存在"));
        }

        self.table.insert(id, updates.clone());
        Ok(updates)
    }

    fn delete(&self, id: Uuid) -> Result<bool> {
        Ok(self.table.remove(&id))
    }

    fn count(&self) -> Result<i64> {
        Ok(i64::try_from(self.table.len()).unwrap_or(i64::MAX))
    }
}

/// AI日志仓储
pub struct AILogRepository {
    #[allow(dead_code)]
    pool: Option<Arc<ConnectionPool>>,
    table: InMemoryTable<AILog>,
}

impl AILogRepository {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        Self {
            pool: Some(pool),
            table: InMemoryTable::default(),
        }
    }

    pub fn new_in_memory() -> Self {
        Self {
            pool: None,
            table: InMemoryTable::default(),
        }
    }

    pub fn find_by_project_id(&self, project_id: Uuid) -> Result<Vec<AILog>> {
        let mut logs = self
            .table
            .values()
            .into_iter()
            .filter(|log| log.project_id == project_id)
            .collect::<Vec<_>>();
        logs.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(logs)
    }
}

impl Repository<AILog> for AILogRepository {
    fn create(&self, item: AILog) -> Result<AILog> {
        self.table.insert(item.id, item.clone());
        Ok(item)
    }

    fn find_by_id(&self, id: Uuid) -> Result<Option<AILog>> {
        Ok(self.table.get(&id))
    }

    fn find_all(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<AILog>> {
        let mut logs = self.table.values();
        logs.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(paginate(logs, limit, offset))
    }

    fn update(&self, id: Uuid, updates: AILog) -> Result<AILog> {
        if self.table.get(&id).is_none() {
            return Err(anyhow!("AI日志不存在"));
        }

        self.table.insert(id, updates.clone());
        Ok(updates)
    }

    fn delete(&self, id: Uuid) -> Result<bool> {
        Ok(self.table.remove(&id))
    }

    fn count(&self) -> Result<i64> {
        Ok(i64::try_from(self.table.len()).unwrap_or(i64::MAX))
    }
}

/// API统计仓储
pub struct APIStatsRepository {
    #[allow(dead_code)]
    pool: Option<Arc<ConnectionPool>>,
    table: InMemoryTable<APIStatRecord>,
}

impl APIStatsRepository {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        Self {
            pool: Some(pool),
            table: InMemoryTable::default(),
        }
    }

    pub fn new_in_memory() -> Self {
        Self {
            pool: None,
            table: InMemoryTable::default(),
        }
    }

    pub fn find_by_provider(&self, provider_name: &str) -> Result<Vec<APIStatRecord>> {
        let mut data = self
            .table
            .values()
            .into_iter()
            .filter(|record| record.provider_name == provider_name)
            .collect::<Vec<_>>();

        data.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        Ok(data)
    }
}

impl Repository<APIStatRecord> for APIStatsRepository {
    fn create(&self, item: APIStatRecord) -> Result<APIStatRecord> {
        self.table.insert(item.id, item.clone());
        Ok(item)
    }

    fn find_by_id(&self, id: Uuid) -> Result<Option<APIStatRecord>> {
        Ok(self.table.get(&id))
    }

    fn find_all(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<APIStatRecord>> {
        let mut data = self.table.values();
        data.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        Ok(paginate(data, limit, offset))
    }

    fn update(&self, id: Uuid, updates: APIStatRecord) -> Result<APIStatRecord> {
        if self.table.get(&id).is_none() {
            return Err(anyhow!("API统计记录不存在"));
        }

        self.table.insert(id, updates.clone());
        Ok(updates)
    }

    fn delete(&self, id: Uuid) -> Result<bool> {
        Ok(self.table.remove(&id))
    }

    fn count(&self) -> Result<i64> {
        Ok(i64::try_from(self.table.len()).unwrap_or(i64::MAX))
    }
}

/// 游戏配置仓储
pub struct GameSpecRepository {
    #[allow(dead_code)]
    pool: Option<Arc<ConnectionPool>>,
    table: InMemoryTable<GameSpec>,
}

impl GameSpecRepository {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        Self {
            pool: Some(pool),
            table: InMemoryTable::default(),
        }
    }

    pub fn new_in_memory() -> Self {
        Self {
            pool: None,
            table: InMemoryTable::default(),
        }
    }

    pub fn find_by_project_id(&self, project_id: Uuid) -> Result<Vec<GameSpec>> {
        let mut specs = self
            .table
            .values()
            .into_iter()
            .filter(|spec| spec.project_id == project_id)
            .collect::<Vec<_>>();
        specs.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        Ok(specs)
    }
}

impl Repository<GameSpec> for GameSpecRepository {
    fn create(&self, item: GameSpec) -> Result<GameSpec> {
        self.table.insert(item.id, item.clone());
        Ok(item)
    }

    fn find_by_id(&self, id: Uuid) -> Result<Option<GameSpec>> {
        Ok(self.table.get(&id))
    }

    fn find_all(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<GameSpec>> {
        let mut data = self.table.values();
        data.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        Ok(paginate(data, limit, offset))
    }

    fn update(&self, id: Uuid, updates: GameSpec) -> Result<GameSpec> {
        if self.table.get(&id).is_none() {
            return Err(anyhow!("游戏配置不存在"));
        }

        self.table.insert(id, updates.clone());
        Ok(updates)
    }

    fn delete(&self, id: Uuid) -> Result<bool> {
        Ok(self.table.remove(&id))
    }

    fn count(&self) -> Result<i64> {
        Ok(i64::try_from(self.table.len()).unwrap_or(i64::MAX))
    }
}

/// 认证会话仓储
pub struct AuthSessionRepository {
    #[allow(dead_code)]
    pool: Option<Arc<ConnectionPool>>,
    table: InMemoryTable<AuthSession>,
}

impl AuthSessionRepository {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        Self {
            pool: Some(pool),
            table: InMemoryTable::default(),
        }
    }

    pub fn new_in_memory() -> Self {
        Self {
            pool: None,
            table: InMemoryTable::default(),
        }
    }

    pub fn find_active_by_user_id(&self, user_id: Uuid) -> Result<Vec<AuthSession>> {
        let mut sessions = self
            .table
            .values()
            .into_iter()
            .filter(|session| session.user_id == user_id && session.status == SessionStatus::Active)
            .collect::<Vec<_>>();

        sessions.sort_by(|a, b| b.last_accessed.cmp(&a.last_accessed));
        Ok(sessions)
    }

    pub fn revoke_session(&self, session_id: Uuid, reason: Option<String>) -> Result<()> {
        let mut session = self
            .table
            .get(&session_id)
            .ok_or_else(|| anyhow!("会话不存在"))?;

        session.revoke(reason);
        self.table.insert(session_id, session);
        Ok(())
    }
}

impl Repository<AuthSession> for AuthSessionRepository {
    fn create(&self, item: AuthSession) -> Result<AuthSession> {
        self.table.insert(item.id, item.clone());
        Ok(item)
    }

    fn find_by_id(&self, id: Uuid) -> Result<Option<AuthSession>> {
        Ok(self.table.get(&id))
    }

    fn find_all(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<AuthSession>> {
        let mut sessions = self.table.values();
        sessions.sort_by(|a, b| b.last_accessed.cmp(&a.last_accessed));
        Ok(paginate(sessions, limit, offset))
    }

    fn update(&self, id: Uuid, updates: AuthSession) -> Result<AuthSession> {
        if self.table.get(&id).is_none() {
            return Err(anyhow!("会话不存在"));
        }

        self.table.insert(id, updates.clone());
        Ok(updates)
    }

    fn delete(&self, id: Uuid) -> Result<bool> {
        Ok(self.table.remove(&id))
    }

    fn count(&self) -> Result<i64> {
        Ok(i64::try_from(self.table.len()).unwrap_or(i64::MAX))
    }
}

/// 仓储管理器
pub struct RepositoryManager {
    pub users: Arc<UserRepository>,
    pub projects: Arc<ProjectRepository>,
    pub ai_logs: Arc<AILogRepository>,
    pub game_specs: Arc<GameSpecRepository>,
    pub api_stats: Arc<APIStatsRepository>,
    pub auth_sessions: Arc<AuthSessionRepository>,
}

impl RepositoryManager {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        Self {
            users: Arc::new(UserRepository::new(pool.clone())),
            projects: Arc::new(ProjectRepository::new(pool.clone())),
            ai_logs: Arc::new(AILogRepository::new(pool.clone())),
            game_specs: Arc::new(GameSpecRepository::new(pool.clone())),
            api_stats: Arc::new(APIStatsRepository::new(pool.clone())),
            auth_sessions: Arc::new(AuthSessionRepository::new(pool)),
        }
    }

    pub fn new_in_memory() -> Self {
        Self {
            users: Arc::new(UserRepository::new_in_memory()),
            projects: Arc::new(ProjectRepository::new_in_memory()),
            ai_logs: Arc::new(AILogRepository::new_in_memory()),
            game_specs: Arc::new(GameSpecRepository::new_in_memory()),
            api_stats: Arc::new(APIStatsRepository::new_in_memory()),
            auth_sessions: Arc::new(AuthSessionRepository::new_in_memory()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{AuthMethod, DeviceType, ProjectStatus, UserRole};

    #[test]
    fn user_repository_crud_should_work() {
        let repo = UserRepository::new_in_memory();

        let mut user = User::new("tester".to_string(), Some("test@example.com".to_string()), None);
        user.role = UserRole::User;

        let created = repo.create(user.clone()).expect("create user should succeed");
        assert_eq!(created.id, user.id);

        let found = repo
            .find_by_email("test@example.com")
            .expect("find by email should succeed")
            .expect("user should exist");
        assert_eq!(found.id, user.id);

        let count = repo.count().expect("count should succeed");
        assert_eq!(count, 1);

        let deleted = repo.delete(user.id).expect("delete should succeed");
        assert!(deleted);
    }

    #[test]
    fn session_repository_revoke_should_work() {
        let repo = AuthSessionRepository::new_in_memory();
        let session = AuthSession::new(
            Uuid::new_v4(),
            AuthMethod::Email,
            "device-1".to_string(),
            DeviceType::Desktop,
            "ua".to_string(),
            None,
            "token".to_string(),
            None,
            24,
        );

        let id = session.id;
        repo.create(session).expect("create session should succeed");
        repo.revoke_session(id, Some("test".to_string()))
            .expect("revoke should succeed");

        let found = repo
            .find_by_id(id)
            .expect("find should succeed")
            .expect("session should exist");

        assert_eq!(found.status, SessionStatus::Revoked);
    }

    #[test]
    fn project_repository_find_by_user_should_work() {
        let repo = ProjectRepository::new_in_memory();
        let user_id = Uuid::new_v4();

        let mut project = Project::new(user_id, "Demo".to_string(), None);
        project.status = ProjectStatus::Active;
        project.tags = vec!["rpg".to_string()];

        repo.create(project).expect("create project should succeed");

        let list = repo
            .find_by_user_id(user_id)
            .expect("find by user should succeed");

        assert_eq!(list.len(), 1);
    }
}
