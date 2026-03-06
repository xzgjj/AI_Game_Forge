//! 审计服务模块
//! 处理操作审计和日志记录的业务逻辑

use std::sync::RwLock;

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct AuditEvent {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub event_type: String,
    pub message: String,
    pub severity: AuditSeverity,
    pub metadata: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl AuditEvent {
    pub fn new(
        user_id: Option<Uuid>,
        project_id: Option<Uuid>,
        event_type: String,
        message: String,
        severity: AuditSeverity,
        metadata: serde_json::Value,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            project_id,
            event_type,
            message,
            severity,
            metadata,
            created_at: chrono::Utc::now(),
        }
    }
}

/// 审计服务
pub struct AuditService {
    events: RwLock<Vec<AuditEvent>>,
}

impl AuditService {
    pub fn new() -> Self {
        Self {
            events: RwLock::new(Vec::new()),
        }
    }

    pub fn record(&self, event: AuditEvent) -> Uuid {
        let event_id = event.id;
        self.events.write().expect("audit lock poisoned").push(event);
        event_id
    }

    pub fn list(
        &self,
        limit: Option<usize>,
        offset: Option<usize>,
        severity: Option<AuditSeverity>,
    ) -> Vec<AuditEvent> {
        let events = self.events.read().expect("audit lock poisoned");

        let mut data = events
            .iter()
            .filter(|event| severity.as_ref().map(|s| &event.severity == s).unwrap_or(true))
            .cloned()
            .collect::<Vec<_>>();

        data.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        let start = offset.unwrap_or(0);
        let take = limit.unwrap_or(data.len());

        data.into_iter().skip(start).take(take).collect()
    }

    pub fn list_by_user(&self, user_id: Uuid, limit: Option<usize>) -> Vec<AuditEvent> {
        self.list(limit, Some(0), None)
            .into_iter()
            .filter(|event| event.user_id.map(|uid| uid == user_id).unwrap_or(false))
            .collect()
    }

    pub fn list_by_project(&self, project_id: Uuid, limit: Option<usize>) -> Vec<AuditEvent> {
        self.list(limit, Some(0), None)
            .into_iter()
            .filter(|event| event.project_id.map(|pid| pid == project_id).unwrap_or(false))
            .collect()
    }

    pub fn purge_before(&self, before: chrono::DateTime<chrono::Utc>) -> usize {
        let mut events = self.events.write().expect("audit lock poisoned");
        let original = events.len();

        events.retain(|event| event.created_at >= before);
        original.saturating_sub(events.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_and_filter_events_should_work() {
        let service = AuditService::new();
        let user_id = Uuid::new_v4();
        let project_id = Uuid::new_v4();

        service.record(AuditEvent::new(
            Some(user_id),
            Some(project_id),
            "auth.login".to_string(),
            "用户登录成功".to_string(),
            AuditSeverity::Info,
            serde_json::json!({ "method": "email" }),
        ));

        service.record(AuditEvent::new(
            Some(user_id),
            Some(project_id),
            "ai.generate".to_string(),
            "模型调用失败".to_string(),
            AuditSeverity::Error,
            serde_json::json!({ "provider": "openai" }),
        ));

        let errors = service.list(Some(10), Some(0), Some(AuditSeverity::Error));
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].event_type, "ai.generate");

        let by_user = service.list_by_user(user_id, Some(10));
        assert_eq!(by_user.len(), 2);
    }
}
