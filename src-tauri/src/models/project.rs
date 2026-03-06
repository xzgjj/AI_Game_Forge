//! 项目模型
//! 定义项目数据结构和相关功能

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// 项目状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectStatus {
    Draft,      // 草稿
    Active,     // 活跃
    Archived,   // 已归档
    Exported,   // 已导出
    Deleted,    // 已删除
}

/// 项目模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: ProjectStatus,
    pub tags: Vec<String>,
    pub config_id: Option<Uuid>, // 关联的游戏配置ID
    pub version: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub total_cost: f64,
    pub is_template: bool,
    pub template_source: Option<Uuid>, // 如果是从模板创建，记录源模板ID
    pub metadata: serde_json::Value,
}

impl Project {
    /// 创建新项目
    pub fn new(
        user_id: Uuid,
        name: String,
        description: Option<String>,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            user_id,
            name,
            description,
            status: ProjectStatus::Draft,
            tags: Vec::new(),
            config_id: None,
            version: 1,
            created_at: now,
            updated_at: now,
            last_accessed: now,
            total_cost: 0.0,
            is_template: false,
            template_source: None,
            metadata: serde_json::json!({}),
        }
    }

    /// 更新最后访问时间
    pub fn update_access(&mut self) {
        self.last_accessed = Utc::now();
    }

    /// 增加版本号
    pub fn increment_version(&mut self, description: Option<String>) {
        self.version += 1;
        self.updated_at = Utc::now();

        // 在元数据中记录版本历史
        let mut history = self.metadata["version_history"]
            .as_array()
            .cloned()
            .unwrap_or_else(Vec::new);

        history.push(serde_json::json!({
            "version": self.version,
            "description": description,
            "timestamp": self.updated_at.to_rfc3339(),
        }));

        self.metadata["version_history"] = serde_json::Value::Array(history);
    }

    /// 添加标签
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.updated_at = Utc::now();
        }
    }

    /// 移除标签
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
        self.updated_at = Utc::now();
    }

    /// 添加成本记录
    pub fn add_cost(&mut self, amount: f64) {
        self.total_cost += amount;
        self.updated_at = Utc::now();
    }

    /// 检查项目是否可编辑
    pub fn is_editable(&self) -> bool {
        matches!(self.status, ProjectStatus::Draft | ProjectStatus::Active)
    }

    /// 检查项目是否可导出
    pub fn is_exportable(&self) -> bool {
        matches!(self.status, ProjectStatus::Active | ProjectStatus::Exported)
    }
}

/// 项目统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStats {
    pub project_id: Uuid,
    pub generation_count: u32,
    pub total_tokens: u64,
    pub total_cost: f64,
    pub avg_generation_time: f64, // 秒
    pub provider_usage: Vec<ProviderUsage>,
    pub last_generation: Option<DateTime<Utc>>,
}

/// 提供商使用情况
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderUsage {
    pub provider_name: String,
    pub request_count: u32,
    pub token_count: u64,
    pub cost: f64,
    pub success_rate: f64,
}
