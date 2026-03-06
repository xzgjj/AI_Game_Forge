//! AI日志模型
//! 定义AI调用日志数据结构和相关功能

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// 生成状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GenerationStatus {
    Pending,    // 等待中
    Processing, // 处理中
    Success,    // 成功
    Failed,     // 失败
    Cancelled,  // 已取消
}

/// AI日志模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AILog {
    pub id: Uuid,
    pub project_id: Uuid,
    pub user_id: Uuid,
    pub provider_name: String,
    pub model_name: String,
    pub prompt: String,
    pub response: String,
    pub status: GenerationStatus,
    pub tokens_used: u32,
    pub cost: f64,
    pub response_time_ms: u64,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub metadata: serde_json::Value,
}

impl AILog {
    /// 创建新的AI日志
    pub fn new(
        project_id: Uuid,
        user_id: Uuid,
        provider_name: String,
        model_name: String,
        prompt: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            project_id,
            user_id,
            provider_name,
            model_name,
            prompt,
            response: String::new(),
            status: GenerationStatus::Pending,
            tokens_used: 0,
            cost: 0.0,
            response_time_ms: 0,
            created_at: Utc::now(),
            completed_at: None,
            error_message: None,
            metadata: serde_json::json!({}),
        }
    }

    /// 标记为处理中
    pub fn mark_processing(&mut self) {
        self.status = GenerationStatus::Processing;
    }

    /// 标记为成功
    pub fn mark_success(
        &mut self,
        response: String,
        tokens_used: u32,
        cost: f64,
        response_time_ms: u64,
        metadata: serde_json::Value,
    ) {
        self.status = GenerationStatus::Success;
        self.response = response;
        self.tokens_used = tokens_used;
        self.cost = cost;
        self.response_time_ms = response_time_ms;
        self.metadata = metadata;
        self.completed_at = Some(Utc::now());
    }

    /// 标记为失败
    pub fn mark_failed(&mut self, error_message: String) {
        self.status = GenerationStatus::Failed;
        self.error_message = Some(error_message);
        self.completed_at = Some(Utc::now());
    }

    /// 计算每token成本
    pub fn cost_per_token(&self) -> f64 {
        if self.tokens_used > 0 {
            self.cost / self.tokens_used as f64
        } else {
            0.0
        }
    }

    /// 获取生成耗时（秒）
    pub fn duration_seconds(&self) -> f64 {
        match self.completed_at {
            Some(completed) => {
                let duration = completed - self.created_at;
                duration.num_milliseconds() as f64 / 1000.0
            }
            None => 0.0,
        }
    }

    /// 检查是否成功
    pub fn is_successful(&self) -> bool {
        self.status == GenerationStatus::Success
    }
}

/// AI使用统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIUsageStats {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub total_tokens: u64,
    pub total_cost: f64,
    pub avg_response_time_ms: f64,
    pub by_provider: Vec<ProviderStats>,
    pub by_hour: Vec<HourlyStats>,
}

/// 提供商统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderStats {
    pub provider_name: String,
    pub request_count: u64,
    pub success_rate: f64,
    pub avg_cost: f64,
    pub avg_response_time_ms: f64,
}

/// 小时统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourlyStats {
    pub hour: u32, // 0-23
    pub request_count: u64,
    pub avg_response_time_ms: f64,
}
