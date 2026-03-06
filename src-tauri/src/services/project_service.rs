//! 项目管理服务模块
//! 提供项目创建、版本保存、列表查询、删除恢复和导出数据装配。

use std::collections::HashMap;
use std::sync::RwLock;

use anyhow::{anyhow, Result};
use chrono::Utc;
use serde_json::Value;
use uuid::Uuid;

use crate::models::project::{Project, ProjectStatus};

/// 项目版本记录
#[derive(Debug, Clone)]
pub struct ProjectVersionRecord {
    pub version: u32,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub author: String,
    pub changes: Value,
}

/// 项目服务
pub struct ProjectService {
    projects: RwLock<HashMap<Uuid, Project>>,
    versions: RwLock<HashMap<Uuid, Vec<ProjectVersionRecord>>>,
    demo_user_id: Uuid,
}

impl ProjectService {
    pub fn new() -> Self {
        Self {
            projects: RwLock::new(HashMap::new()),
            versions: RwLock::new(HashMap::new()),
            demo_user_id: Uuid::new_v4(),
        }
    }

    pub fn create_project(
        &self,
        name: String,
        description: Option<String>,
        template_id: Option<Uuid>,
    ) -> Result<Project> {
        if name.trim().is_empty() {
            return Err(anyhow!("项目名称不能为空"));
        }

        let mut project = Project::new(self.demo_user_id, name.trim().to_string(), description);
        project.status = ProjectStatus::Active;
        project.template_source = template_id;

        let project_id = project.id;

        self.projects
            .write()
            .expect("project lock poisoned")
            .insert(project_id, project.clone());

        self.versions
            .write()
            .expect("version lock poisoned")
            .insert(
                project_id,
                vec![ProjectVersionRecord {
                    version: project.version,
                    description: "初始化项目".to_string(),
                    created_at: project.created_at,
                    author: "system".to_string(),
                    changes: serde_json::json!({
                        "event": "project_created",
                        "template_id": template_id
                    }),
                }],
            );

        Ok(project)
    }

    pub fn save_project(
        &self,
        project_id: Uuid,
        description: Option<String>,
        changes: Value,
    ) -> Result<ProjectVersionRecord> {
        let mut projects = self.projects.write().expect("project lock poisoned");
        let project = projects
            .get_mut(&project_id)
            .ok_or_else(|| anyhow!("项目不存在"))?;

        if !project.is_editable() {
            return Err(anyhow!("当前项目状态不可编辑"));
        }

        let version_description = description
            .unwrap_or_else(|| format!("更新到版本 {}", project.version + 1))
            .trim()
            .to_string();

        project.increment_version(Some(version_description.clone()));
        project.metadata["last_changes"] = changes.clone();
        project.metadata["last_saved_at"] = serde_json::json!(Utc::now().to_rfc3339());

        let record = ProjectVersionRecord {
            version: project.version,
            description: version_description,
            created_at: project.updated_at,
            author: "system".to_string(),
            changes,
        };

        self.versions
            .write()
            .expect("version lock poisoned")
            .entry(project_id)
            .or_default()
            .push(record.clone());

        Ok(record)
    }

    pub fn load_project(&self, project_id: Uuid, version: Option<u32>) -> Result<Value> {
        let mut projects = self.projects.write().expect("project lock poisoned");
        let project = projects
            .get_mut(&project_id)
            .ok_or_else(|| anyhow!("项目不存在"))?;

        project.update_access();
        let current = project.clone();
        drop(projects);

        let versions = self
            .versions
            .read()
            .expect("version lock poisoned")
            .get(&project_id)
            .cloned()
            .unwrap_or_default();

        let selected_version = if let Some(target_version) = version {
            versions
                .iter()
                .find(|item| item.version == target_version)
                .cloned()
                .ok_or_else(|| anyhow!("目标版本不存在"))?
        } else {
            versions
                .last()
                .cloned()
                .ok_or_else(|| anyhow!("项目版本记录为空"))?
        };

        Ok(serde_json::json!({
            "project": current,
            "selected_version": {
                "version": selected_version.version,
                "description": selected_version.description,
                "created_at": selected_version.created_at,
                "author": selected_version.author,
                "changes": selected_version.changes,
            },
            "history_count": versions.len(),
        }))
    }

    pub fn export_payload(
        &self,
        project_id: Uuid,
        format: &str,
        options: Value,
    ) -> Result<Value> {
        let mut projects = self.projects.write().expect("project lock poisoned");
        let project = projects
            .get_mut(&project_id)
            .ok_or_else(|| anyhow!("项目不存在"))?;

        if !project.is_exportable() {
            return Err(anyhow!("项目当前状态不可导出"));
        }

        project.status = ProjectStatus::Exported;
        project.updated_at = Utc::now();

        let version_count = self
            .versions
            .read()
            .expect("version lock poisoned")
            .get(&project_id)
            .map(|items| items.len())
            .unwrap_or(0);

        Ok(serde_json::json!({
            "project": project.clone(),
            "format": format,
            "exported_at": Utc::now(),
            "version_count": version_count,
            "options": options,
        }))
    }

    pub fn list_projects(
        &self,
        status_filter: Option<ProjectStatus>,
        tag_filter: Option<Vec<String>>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Vec<Project> {
        let projects = self.projects.read().expect("project lock poisoned");
        let mut list = projects.values().cloned().collect::<Vec<_>>();

        if let Some(status) = status_filter {
            list.retain(|project| project.status == status);
        }

        if let Some(tags) = tag_filter {
            list.retain(|project| {
                tags.iter()
                    .all(|tag| project.tags.iter().any(|item| item == tag))
            });
        }

        list.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

        let start = offset.unwrap_or(0) as usize;
        let take = limit.unwrap_or(list.len() as u32) as usize;

        list.into_iter().skip(start).take(take).collect()
    }

    pub fn delete_project(&self, project_id: Uuid, permanent: bool) -> Result<bool> {
        if permanent {
            let removed = self
                .projects
                .write()
                .expect("project lock poisoned")
                .remove(&project_id)
                .is_some();

            if removed {
                self.versions
                    .write()
                    .expect("version lock poisoned")
                    .remove(&project_id);
            }

            return Ok(removed);
        }

        let mut projects = self.projects.write().expect("project lock poisoned");
        let project = projects
            .get_mut(&project_id)
            .ok_or_else(|| anyhow!("项目不存在"))?;

        project.status = ProjectStatus::Deleted;
        project.updated_at = Utc::now();
        Ok(true)
    }

    pub fn restore_project(&self, project_id: Uuid) -> Result<Project> {
        let mut projects = self.projects.write().expect("project lock poisoned");
        let project = projects
            .get_mut(&project_id)
            .ok_or_else(|| anyhow!("项目不存在"))?;

        if project.status != ProjectStatus::Deleted {
            return Err(anyhow!("仅支持恢复已删除项目"));
        }

        project.status = ProjectStatus::Active;
        project.updated_at = Utc::now();
        Ok(project.clone())
    }

    pub fn get_project(&self, project_id: Uuid) -> Option<Project> {
        self.projects
            .read()
            .expect("project lock poisoned")
            .get(&project_id)
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_save_load_should_work() {
        let service = ProjectService::new();
        let project = service
            .create_project("Demo".to_string(), Some("desc".to_string()), None)
            .expect("project should be created");

        let version = service
            .save_project(
                project.id,
                Some("first update".to_string()),
                serde_json::json!({ "scene_count": 2 }),
            )
            .expect("save should succeed");

        assert_eq!(version.version, 2);

        let loaded = service
            .load_project(project.id, Some(2))
            .expect("load should succeed");

        assert_eq!(loaded["selected_version"]["version"], 2);
    }
}
