//! API统计接口模块
//! 处理AI API使用统计、成本分析、预算管理等

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

/// 统计时间段
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StatsPeriod {
    Day,       // 日统计
    Week,      // 周统计
    Month,     // 月统计
    Custom {   // 自定义时间段
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
    Info,     // 信息
    Warning,  // 警告
    Danger,   // 危险
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

/// 获取使用统计接口
///
/// # 参数
/// - `app_handle`: 应用句柄
/// - `period`: 统计时间段
///
/// # 返回
/// - 成功: `UsageStats` 使用统计数据
/// - 失败: 错误信息字符串
#[tauri::command]
pub async fn get_usage_stats(
    app_handle: AppHandle,
    period: StatsPeriod,
) -> Result<UsageStats, String> {
    log::info!("Getting usage stats for period: {:?}", period);

    // TODO: 实现统计查询逻辑

    // 返回示例数据
    Ok(UsageStats {
        period,
        total_requests: 0,
        total_tokens: 0,
        total_cost: 0.0,
        by_provider: Vec::new(),
        by_project: Vec::new(),
        cost_trend: Vec::new(),
    })
}

/// 设置预算限制接口
///
/// # 参数
/// - `app_handle`: 应用句柄
/// - `provider`: AI提供商名称（"all" 表示全部）
/// - `monthly_limit`: 月度预算限制
///
/// # 返回
/// - 成功: `true`
/// - 失败: 错误信息字符串
#[tauri::command]
pub async fn set_budget_limit(
    app_handle: AppHandle,
    provider: String,
    monthly_limit: f64,
) -> Result<bool, String> {
    log::info!("Setting budget limit for {}: ${}", provider, monthly_limit);

    // TODO: 实现预算设置逻辑

    Ok(true)
}

/// 获取预算告警接口
///
/// # 参数
/// - `app_handle`: 应用句柄
///
/// # 返回
/// - 成功: `Vec<BudgetAlert>` 预算告警列表
/// - 失败: 错误信息字符串
#[tauri::command]
pub async fn get_budget_alerts(
    app_handle: AppHandle,
) -> Result<Vec<BudgetAlert>, String> {
    log::debug!("Getting budget alerts");

    // TODO: 实现告警检查逻辑

    Ok(Vec::new())
}

/// 获取提供商列表接口
///
/// # 参数
/// - `app_handle`: 应用句柄
///
/// # 返回
/// - 成功: `Vec<String>` 提供商名称列表
/// - 失败: 错误信息字符串
#[tauri::command]
pub async fn get_provider_list(
    app_handle: AppHandle,
) -> Result<Vec<String>, String> {
    log::debug!("Getting provider list");

    // TODO: 从配置或数据库获取提供商列表

    Ok(vec![
        "openai".to_string(),
        "claude".to_string(),
        "zhipu".to_string(),
        "baidu".to_string(),
        "local".to_string(),
    ])
}

/// 重置统计接口
///
/// # 参数
/// - `app_handle`: 应用句柄
/// - `provider`: 提供商名称（可选，None表示全部）
/// - `start_date`: 开始日期（可选）
///
/// # 返回
/// - 成功: `true`
/// - 失败: 错误信息字符串
#[tauri::command]
pub async fn reset_stats(
    app_handle: AppHandle,
    provider: Option<String>,
    start_date: Option<chrono::NaiveDate>,
) -> Result<bool, String> {
    log::info!("Resetting stats for {:?} from {:?}", provider, start_date);

    // TODO: 实现统计重置逻辑

    Ok(true)
}
