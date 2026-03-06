//! API统计模型
//! 定义API使用统计和成本分析数据结构

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};

/// 统计时间段（用于IPC接口）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatsPeriod {
    Day,       // 日统计
    Week,      // 周统计
    Month,     // 月统计
    Custom {   // 自定义时间段
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    },
}

/// 告警级别（用于IPC接口）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertLevel {
    Info,     // 信息
    Warning,  // 警告
    Danger,   // 危险
}

/// 提供商统计（用于IPC接口）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderStats {
    pub name: String,
    pub requests: i64,
    pub tokens: i64,
    pub cost: f64,
    pub success_rate: f64,
    pub avg_response_time: f64,
}

/// 项目统计（用于IPC接口）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStats {
    pub project_id: String,
    pub project_name: String,
    pub requests: i64,
    pub tokens: i64,
    pub cost: f64,
    pub main_provider: String,
}

/// 每日成本（用于IPC接口）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyCost {
    pub date: NaiveDate,
    pub cost: f64,
    pub tokens: i64,
}

/// 使用统计（用于IPC接口）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStats {
    pub period: StatsPeriod,
    pub total_requests: i64,
    pub total_tokens: i64,
    pub total_cost: f64,
    pub by_provider: Vec<ProviderStats>,
    pub by_project: Vec<ProjectStats>,
    pub cost_trend: Vec<DailyCost>,
}

/// 预算告警（用于IPC接口）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetAlert {
    pub level: AlertLevel,
    pub message: String,
    pub threshold: f64,
    pub current: f64,
    pub suggested_action: String,
}

/// API统计记录模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIStatRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub project_id: Option<Uuid>,
    pub provider_name: String,
    pub endpoint: String,
    pub request_count: u32,
    pub token_count: u32,
    pub cost: f64,
    pub success_count: u32,
    pub error_count: u32,
    pub total_response_time_ms: u64,
    pub date: NaiveDate,
    pub hour: Option<u32>, // 0-23，None表示全天统计
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl APIStatRecord {
    /// 创建新的统计记录
    pub fn new(
        user_id: Uuid,
        provider_name: String,
        endpoint: String,
        date: NaiveDate,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            user_id,
            project_id: None,
            provider_name,
            endpoint,
            request_count: 0,
            token_count: 0,
            cost: 0.0,
            success_count: 0,
            error_count: 0,
            total_response_time_ms: 0,
            date,
            hour: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// 创建小时统计记录
    pub fn new_hourly(
        user_id: Uuid,
        provider_name: String,
        endpoint: String,
        date: NaiveDate,
        hour: u32,
    ) -> Self {
        let mut record = Self::new(user_id, provider_name, endpoint, date);
        record.hour = Some(hour);
        record
    }

    /// 添加请求记录
    pub fn add_request(
        &mut self,
        tokens: u32,
        cost: f64,
        response_time_ms: u64,
        success: bool,
        project_id: Option<Uuid>,
    ) {
        self.request_count += 1;
        self.token_count += tokens;
        self.cost += cost;
        self.total_response_time_ms += response_time_ms;

        if success {
            self.success_count += 1;
        } else {
            self.error_count += 1;
        }

        if project_id.is_some() {
            self.project_id = project_id;
        }

        self.updated_at = Utc::now();
    }

    /// 获取平均响应时间
    pub fn avg_response_time_ms(&self) -> f64 {
        if self.request_count > 0 {
            self.total_response_time_ms as f64 / self.request_count as f64
        } else {
            0.0
        }
    }

    /// 获取成功率
    pub fn success_rate(&self) -> f64 {
        if self.request_count > 0 {
            self.success_count as f64 / self.request_count as f64
        } else {
            0.0
        }
    }

    /// 获取错误率
    pub fn error_rate(&self) -> f64 {
        if self.request_count > 0 {
            self.error_count as f64 / self.request_count as f64
        } else {
            0.0
        }
    }

    /// 获取每请求平均成本
    pub fn avg_cost_per_request(&self) -> f64 {
        if self.request_count > 0 {
            self.cost / self.request_count as f64
        } else {
            0.0
        }
    }

    /// 获取每token平均成本
    pub fn avg_cost_per_token(&self) -> f64 {
        if self.token_count > 0 {
            self.cost / self.token_count as f64
        } else {
            0.0
        }
    }
}

/// 预算配置模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetConfig {
    pub id: Uuid,
    pub user_id: Uuid,
    pub scope: BudgetScope,
    pub target_id: Option<Uuid>, // 根据scope可能是provider_id或project_id
    pub monthly_limit: f64,
    pub currency: String,
    pub alert_thresholds: Vec<f64>, // 例如[0.5, 0.8, 0.95]
    pub reset_day: u32, // 每月几号重置（1-31）
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 预算范围
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BudgetScope {
    Global,     // 全局预算
    Provider,   // 按提供商
    Project,    // 按项目
    Category,   // 按类别
}

impl BudgetConfig {
    /// 创建新预算配置
    pub fn new(
        user_id: Uuid,
        scope: BudgetScope,
        target_id: Option<Uuid>,
        monthly_limit: f64,
        currency: String,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            user_id,
            scope,
            target_id,
            monthly_limit,
            currency,
            alert_thresholds: vec![0.5, 0.8, 0.95],
            reset_day: 1,
            created_at: now,
            updated_at: now,
        }
    }

    /// 检查是否触发告警
    pub fn check_alerts(&self, current_spent: f64) -> Vec<(f64, AlertLevel)> {
        let usage_ratio = current_spent / self.monthly_limit;
        let mut alerts = Vec::new();

        for &threshold in &self.alert_thresholds {
            if usage_ratio >= threshold {
                let level = if threshold >= 0.9 {
                    AlertLevel::Danger
                } else if threshold >= 0.7 {
                    AlertLevel::Warning
                } else {
                    AlertLevel::Info
                };
                alerts.push((threshold, level));
            }
        }

        alerts
    }

    /// 生成告警消息
    pub fn generate_alert_message(&self, threshold: f64, current_spent: f64) -> String {
        let usage_percent = (current_spent / self.monthly_limit * 100.0) as u32;
        let threshold_percent = (threshold * 100.0) as u32;

        match self.scope {
            BudgetScope::Global => format!(
                "全局预算使用已超过{}%（当前：{}%），本月已消费{} {}",
                threshold_percent, usage_percent, current_spent, self.currency
            ),
            BudgetScope::Provider => format!(
                "提供商预算使用已超过{}%（当前：{}%）",
                threshold_percent, usage_percent
            ),
            BudgetScope::Project => format!(
                "项目预算使用已超过{}%（当前：{}%）",
                threshold_percent, usage_percent
            ),
            BudgetScope::Category => format!(
                "类别预算使用已超过{}%（当前：{}%）",
                threshold_percent, usage_percent
            ),
        }
    }
}

/// 成本分析报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostAnalysisReport {
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_cost: f64,
    pub cost_by_provider: Vec<CostByProvider>,
    pub cost_by_project: Vec<CostByProject>,
    pub cost_by_day: Vec<DailyCostBreakdown>,
    pub efficiency_metrics: EfficiencyMetrics,
    pub recommendations: Vec<CostOptimizationRecommendation>,
}

/// 按提供商成本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostByProvider {
    pub provider_name: String,
    pub cost: f64,
    pub percentage: f64,
    pub avg_cost_per_token: f64,
    pub efficiency_score: f64, // 0-1，越高越高效
}

/// 按项目成本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostByProject {
    pub project_id: Uuid,
    pub project_name: String,
    pub cost: f64,
    pub percentage: f64,
    pub value_score: f64, // 价值评分，基于生成内容质量和使用频率
}

/// 每日成本明细
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyCostBreakdown {
    pub date: NaiveDate,
    pub total_cost: f64,
    pub provider_breakdown: Vec<DailyProviderCost>,
}

/// 每日提供商成本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyProviderCost {
    pub provider_name: String,
    pub cost: f64,
}

/// 效率指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EfficiencyMetrics {
    pub avg_cost_per_request: f64,
    pub avg_cost_per_token: f64,
    pub tokens_per_dollar: f64,
    pub success_rate: f64,
    pub avg_response_time_ms: f64,
    pub cost_efficiency_trend: f64, // 与上期比较的变化率
}

/// 成本优化建议
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostOptimizationRecommendation {
    pub priority: RecommendationPriority,
    pub title: String,
    pub description: String,
    pub estimated_savings: f64,
    pub implementation_effort: ImplementationEffort,
    pub action_steps: Vec<String>,
}

/// 建议优先级
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationPriority {
    High,
    Medium,
    Low,
}

/// 实施难度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImplementationEffort {
    Low,     // 低，配置调整
    Medium,  // 中，代码修改
    High,    // 高，架构调整
}
