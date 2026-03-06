//! API统计接口模块
//! 处理AI API使用统计、成本分析、预算管理等

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::models::api_stats as model;
use crate::services::ServiceContainer;

/// 统计时间段
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StatsPeriod {
    Day,
    Week,
    Month,
    Custom {
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    },
}

/// 提供商统计
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProviderStats {
    pub name: String,
    pub requests: i64,
    pub tokens: i64,
    pub cost: f64,
    pub success_rate: f64,
    pub avg_response_time: f64,
}

/// 项目统计
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectStats {
    pub project_id: String,
    pub project_name: String,
    pub requests: i64,
    pub tokens: i64,
    pub cost: f64,
    pub main_provider: String,
}

/// 每日成本
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DailyCost {
    pub date: chrono::NaiveDate,
    pub cost: f64,
    pub tokens: i64,
}

/// 使用统计
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsageStats {
    pub period: StatsPeriod,
    pub total_requests: i64,
    pub total_tokens: i64,
    pub total_cost: f64,
    pub by_provider: Vec<ProviderStats>,
    pub by_project: Vec<ProjectStats>,
    pub cost_trend: Vec<DailyCost>,
}

/// 告警级别
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AlertLevel {
    Info,
    Warning,
    Danger,
}

/// 预算告警
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BudgetAlert {
    pub level: AlertLevel,
    pub message: String,
    pub threshold: f64,
    pub current: f64,
    pub suggested_action: String,
}

fn to_model_period(period: StatsPeriod) -> model::StatsPeriod {
    match period {
        StatsPeriod::Day => model::StatsPeriod::Day,
        StatsPeriod::Week => model::StatsPeriod::Week,
        StatsPeriod::Month => model::StatsPeriod::Month,
        StatsPeriod::Custom { start, end } => model::StatsPeriod::Custom { start, end },
    }
}

fn from_model_period(period: model::StatsPeriod) -> StatsPeriod {
    match period {
        model::StatsPeriod::Day => StatsPeriod::Day,
        model::StatsPeriod::Week => StatsPeriod::Week,
        model::StatsPeriod::Month => StatsPeriod::Month,
        model::StatsPeriod::Custom { start, end } => StatsPeriod::Custom { start, end },
    }
}

fn from_model_usage(stats: model::UsageStats) -> UsageStats {
    UsageStats {
        period: from_model_period(stats.period),
        total_requests: stats.total_requests,
        total_tokens: stats.total_tokens,
        total_cost: stats.total_cost,
        by_provider: stats
            .by_provider
            .into_iter()
            .map(|p| ProviderStats {
                name: p.name,
                requests: p.requests,
                tokens: p.tokens,
                cost: p.cost,
                success_rate: p.success_rate,
                avg_response_time: p.avg_response_time,
            })
            .collect(),
        by_project: stats
            .by_project
            .into_iter()
            .map(|p| ProjectStats {
                project_id: p.project_id,
                project_name: p.project_name,
                requests: p.requests,
                tokens: p.tokens,
                cost: p.cost,
                main_provider: p.main_provider,
            })
            .collect(),
        cost_trend: stats
            .cost_trend
            .into_iter()
            .map(|c| DailyCost {
                date: c.date,
                cost: c.cost,
                tokens: c.tokens,
            })
            .collect(),
    }
}

fn from_model_alert(alert: model::BudgetAlert) -> BudgetAlert {
    BudgetAlert {
        level: match alert.level {
            model::AlertLevel::Info => AlertLevel::Info,
            model::AlertLevel::Warning => AlertLevel::Warning,
            model::AlertLevel::Danger => AlertLevel::Danger,
        },
        message: alert.message,
        threshold: alert.threshold,
        current: alert.current,
        suggested_action: alert.suggested_action,
    }
}

#[tauri::command]
pub async fn get_usage_stats(app_handle: AppHandle, period: StatsPeriod) -> Result<UsageStats, String> {
    let services = app_handle.state::<ServiceContainer>();

    services
        .api_mgmt_service
        .get_usage_stats(to_model_period(period))
        .await
        .map(from_model_usage)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn set_budget_limit(
    app_handle: AppHandle,
    provider: String,
    monthly_limit: f64,
) -> Result<bool, String> {
    let services = app_handle.state::<ServiceContainer>();
    services
        .api_mgmt_service
        .set_budget_limit(provider, monthly_limit)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn get_budget_alerts(app_handle: AppHandle) -> Result<Vec<BudgetAlert>, String> {
    let services = app_handle.state::<ServiceContainer>();
    let alerts = services.api_mgmt_service.check_budget_alerts().await;
    Ok(alerts.into_iter().map(from_model_alert).collect())
}

#[tauri::command]
pub async fn get_provider_list(app_handle: AppHandle) -> Result<Vec<String>, String> {
    let services = app_handle.state::<ServiceContainer>();

    let mut providers = services
        .api_mgmt_service
        .get_provider_status()
        .await
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    providers.sort();

    Ok(providers)
}

#[tauri::command]
pub async fn reset_stats(
    _app_handle: AppHandle,
    _provider: Option<String>,
    _start_date: Option<chrono::NaiveDate>,
) -> Result<bool, String> {
    // 当前实现的统计是运行时聚合，重置逻辑将在持久化统计接入后实现。
    Ok(true)
}
