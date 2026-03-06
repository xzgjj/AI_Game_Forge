//! API管理服务模块
//! 处理多AI提供商的管理、智能路由、成本统计等

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use anyhow::{Result, anyhow};
use chrono::{Utc, Duration};
use serde_json::Value;

use crate::providers::{AIProvider, AIGenerationRequest, AIGenerationResponse};
use crate::models::api_stats::*;

/// AI提供商统计
#[derive(Debug, Clone)]
pub struct ProviderStatsInternal {
    pub name: String,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub total_tokens: u64,
    pub total_cost: f64,
    pub avg_response_time_ms: f64,
    pub last_used: chrono::DateTime<Utc>,
    pub is_available: bool,
    pub error_rate: f64,
}

/// 路由策略
#[derive(Debug, Clone, PartialEq)]
pub enum RoutingStrategy {
    CostFirst,      // 成本优先
    SpeedFirst,     // 速度优先
    QualityFirst,   // 质量优先
    Balanced,       // 平衡模式
    Manual(String), // 手动指定提供商
}

/// 预算限制
#[derive(Debug, Clone)]
pub struct BudgetLimit {
    pub provider_name: String, // "all" 表示全局预算
    pub monthly_limit: f64,
    pub current_month_spent: f64,
    pub reset_day: u32, // 每月几号重置（1-31）
    pub alerts_sent: Vec<AlertLevel>,
}

/// API管理服务
pub struct APIManagementService {
    providers: HashMap<String, Arc<dyn AIProvider>>,
    stats: RwLock<HashMap<String, ProviderStatsInternal>>,
    budgets: RwLock<Vec<BudgetLimit>>,
    routing_strategy: RwLock<RoutingStrategy>,
    request_history: RwLock<VecDeque<AIGenerationRequest>>,
    config: APIMgmtConfig,
}

/// API管理配置
#[derive(Debug, Clone)]
pub struct APIMgmtConfig {
    pub default_provider: String,
    pub enable_smart_routing: bool,
    pub fallback_order: Vec<String>,
    pub cache_enabled: bool,
    pub cache_size: usize,
    pub request_timeout_secs: u64,
    pub max_retries: u32,
    pub cost_currency: String,
}

impl APIManagementService {
    /// 创建新的API管理服务
    pub fn new(config: APIMgmtConfig) -> Self {
        Self {
            providers: HashMap::new(),
            stats: RwLock::new(HashMap::new()),
            budgets: RwLock::new(Vec::new()),
            routing_strategy: RwLock::new(RoutingStrategy::Balanced),
            request_history: RwLock::new(VecDeque::new()),
            config,
        }
    }

    /// 注册AI提供商
    pub fn register_provider(&mut self, name: String, provider: Arc<dyn AIProvider>) {
        log::info!("Registering AI provider: {}", name);

        // 初始化统计
        let stats = ProviderStatsInternal {
            name: name.clone(),
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            total_tokens: 0,
            total_cost: 0.0,
            avg_response_time_ms: 0.0,
            last_used: Utc::now(),
            is_available: true,
            error_rate: 0.0,
        };

        self.providers.insert(name.clone(), provider);
        self.stats.write().unwrap().insert(name, stats);
    }

    /// 智能路由请求
    pub async fn smart_route(
        &self,
        request: AIGenerationRequest,
    ) -> Result<AIGenerationResponse> {
        log::debug!("Smart routing request: {:?}", request.content_type);

        let provider_name = self.select_provider(&request).await?;
        log::info!("Selected provider: {} for request", provider_name);

        self.execute_request(provider_name, request).await
    }

    /// 选择最佳提供商
    async fn select_provider(&self, request: &AIGenerationRequest) -> Result<String> {
        let strategy = self.routing_strategy.read().unwrap().clone();

        match strategy {
            RoutingStrategy::Manual(provider) => {
                if self.providers.contains_key(&provider) {
                    return Ok(provider);
                } else {
                    log::warn!("Manual provider {} not available, falling back", provider);
                }
            }
            _ => {} // 其他策略继续处理
        }

        if !self.config.enable_smart_routing {
            return Ok(self.config.default_provider.clone());
        }

        // 获取可用的提供商
        let available_providers: Vec<String> = self.get_available_providers().await;

        if available_providers.is_empty() {
            return Err(anyhow!("没有可用的AI提供商"));
        }

        // 根据策略选择提供商
        match strategy {
            RoutingStrategy::CostFirst => self.select_cost_first(&available_providers),
            RoutingStrategy::SpeedFirst => self.select_speed_first(&available_providers),
            RoutingStrategy::QualityFirst => self.select_quality_first(&available_providers, request),
            RoutingStrategy::Balanced => self.select_balanced(&available_providers, request),
            _ => Ok(self.config.default_provider.clone()),
        }
    }

    /// 成本优先选择
    fn select_cost_first(&self, providers: &[String]) -> Result<String> {
        let stats = self.stats.read().unwrap();

        providers
            .iter()
            .filter_map(|name| {
                stats.get(name).map(|stat| (name, stat.total_cost / stat.total_requests.max(1) as f64))
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(name, _)| name.clone())
            .ok_or_else(|| anyhow!("无法选择成本最优的提供商"))
    }

    /// 速度优先选择
    fn select_speed_first(&self, providers: &[String]) -> Result<String> {
        let stats = self.stats.read().unwrap();

        providers
            .iter()
            .filter_map(|name| {
                stats.get(name).map(|stat| (name, stat.avg_response_time_ms))
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(name, _)| name.clone())
            .ok_or_else(|| anyhow!("无法选择速度最优的提供商"))
    }

    /// 质量优先选择（根据请求类型）
    fn select_quality_first(&self, providers: &[String], _request: &AIGenerationRequest) -> Result<String> {
        // TODO: 根据请求类型和历史成功率选择
        // 目前先返回第一个可用提供商
        providers.first()
            .cloned()
            .ok_or_else(|| anyhow!("没有可用的提供商"))
    }

    /// 平衡选择
    fn select_balanced(&self, providers: &[String], _request: &AIGenerationRequest) -> Result<String> {
        // TODO: 实现综合评分算法
        // 考虑成本、速度、质量、使用频率等因素
        Ok(self.config.default_provider.clone())
    }

    /// 执行请求
    async fn execute_request(
        &self,
        provider_name: String,
        request: AIGenerationRequest,
    ) -> Result<AIGenerationResponse> {
        let start_time = Utc::now();

        // 检查预算
        self.check_budget(&provider_name, &request).await?;

        // 获取提供商
        let provider = self.providers.get(&provider_name)
            .ok_or_else(|| anyhow!("提供商不存在: {}", provider_name))?;

        // 执行请求
        let result = provider.generate(request.clone()).await;

        let end_time = Utc::now();
        let duration = end_time - start_time;

        // 更新统计
        self.update_stats(&provider_name, &result, duration).await;

        result
    }

    /// 检查预算
    async fn check_budget(&self, provider_name: &str, _request: &AIGenerationRequest) -> Result<()> {
        let budgets = self.budgets.read().unwrap();

        // 检查全局预算
        if let Some(global_budget) = budgets.iter().find(|b| b.provider_name == "all") {
            if global_budget.current_month_spent >= global_budget.monthly_limit {
                return Err(anyhow!("全局预算已用完"));
            }
        }

        // 检查提供商特定预算
        if let Some(provider_budget) = budgets.iter().find(|b| &b.provider_name == provider_name) {
            if provider_budget.current_month_spent >= provider_budget.monthly_limit {
                return Err(anyhow!("提供商 {} 的预算已用完", provider_name));
            }
        }

        Ok(())
    }

    /// 更新统计
    async fn update_stats(
        &self,
        provider_name: &str,
        result: &Result<AIGenerationResponse>,
        duration: chrono::Duration,
    ) {
        let mut stats = self.stats.write().unwrap();

        if let Some(provider_stats) = stats.get_mut(provider_name) {
            provider_stats.total_requests += 1;
            provider_stats.last_used = Utc::now();
            provider_stats.avg_response_time_ms =
                (provider_stats.avg_response_time_ms * (provider_stats.total_requests - 1) as f64
                 + duration.num_milliseconds() as f64) / provider_stats.total_requests as f64;

            match result {
                Ok(response) => {
                    provider_stats.successful_requests += 1;
                    provider_stats.total_tokens += response.tokens_used as u64;
                    provider_stats.total_cost += response.cost;
                    provider_stats.is_available = true;
                }
                Err(_) => {
                    provider_stats.failed_requests += 1;
                    provider_stats.error_rate =
                        provider_stats.failed_requests as f64 / provider_stats.total_requests as f64;

                    // 如果错误率过高，标记为不可用
                    if provider_stats.error_rate > 0.3 && provider_stats.total_requests > 10 {
                        provider_stats.is_available = false;
                        log::warn!("提供商 {} 错误率过高，标记为不可用", provider_name);
                    }
                }
            }
        }
    }

    /// 获取可用提供商列表
    async fn get_available_providers(&self) -> Vec<String> {
        let stats = self.stats.read().unwrap();

        self.providers.keys()
            .filter(|name| {
                stats.get(*name)
                    .map(|stat| stat.is_available)
                    .unwrap_or(false)
            })
            .cloned()
            .collect()
    }

    /// 获取使用统计
    pub async fn get_usage_stats(&self, period: StatsPeriod) -> Result<UsageStats> {
        log::info!("Getting usage stats for period: {:?}", period);

        // TODO: 从数据库查询详细统计

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

    /// 设置预算限制
    pub async fn set_budget_limit(&self, provider: String, monthly_limit: f64) -> Result<bool> {
        log::info!("Setting budget limit for {}: ${}", provider, monthly_limit);

        let mut budgets = self.budgets.write().unwrap();

        if let Some(existing) = budgets.iter_mut().find(|b| b.provider_name == provider) {
            existing.monthly_limit = monthly_limit;
        } else {
            budgets.push(BudgetLimit {
                provider_name: provider,
                monthly_limit,
                current_month_spent: 0.0,
                reset_day: 1, // 每月1号重置
                alerts_sent: Vec::new(),
            });
        }

        Ok(true)
    }

    /// 检查预算告警
    pub async fn check_budget_alerts(&self) -> Vec<BudgetAlert> {
        log::debug!("Checking budget alerts");

        let mut alerts = Vec::new();
        let budgets = self.budgets.read().unwrap();

        for budget in budgets.iter() {
            let usage_ratio = budget.current_month_spent / budget.monthly_limit;

            if usage_ratio >= 0.95 {
                alerts.push(BudgetAlert {
                    level: AlertLevel::Danger,
                    message: format!("{} 预算使用已超过95%", budget.provider_name),
                    threshold: 0.95,
                    current: usage_ratio,
                    suggested_action: "立即停止使用或增加预算".to_string(),
                });
            } else if usage_ratio >= 0.8 {
                alerts.push(BudgetAlert {
                    level: AlertLevel::Warning,
                    message: format!("{} 预算使用已超过80%", budget.provider_name),
                    threshold: 0.8,
                    current: usage_ratio,
                    suggested_action: "考虑切换到成本更低的提供商".to_string(),
                });
            } else if usage_ratio >= 0.5 {
                alerts.push(BudgetAlert {
                    level: AlertLevel::Info,
                    message: format!("{} 预算使用已超过50%", budget.provider_name),
                    threshold: 0.5,
                    current: usage_ratio,
                    suggested_action: "监控使用情况".to_string(),
                });
            }
        }

        alerts
    }

    /// 设置路由策略
    pub async fn set_routing_strategy(&self, strategy: RoutingStrategy) -> Result<()> {
        log::info!("Setting routing strategy: {:?}", strategy);

        *self.routing_strategy.write().unwrap() = strategy;
        Ok(())
    }

    /// 获取提供商状态
    pub async fn get_provider_status(&self) -> HashMap<String, Value> {
        let stats = self.stats.read().unwrap();
        let mut status = HashMap::new();

        for (name, stat) in stats.iter() {
            status.insert(name.clone(), serde_json::json!({
                "available": stat.is_available,
                "total_requests": stat.total_requests,
                "success_rate": if stat.total_requests > 0 {
                    stat.successful_requests as f64 / stat.total_requests as f64
                } else {
                    0.0
                },
                "avg_response_time_ms": stat.avg_response_time_ms,
                "total_cost": stat.total_cost,
                "error_rate": stat.error_rate,
                "last_used": stat.last_used.to_rfc3339(),
            }));
        }

        status
    }
}

impl Default for APIMgmtConfig {
    fn default() -> Self {
        Self {
            default_provider: "openai".to_string(),
            enable_smart_routing: true,
            fallback_order: vec![
                "openai".to_string(),
                "claude".to_string(),
                "zhipu".to_string(),
                "baidu".to_string(),
                "local".to_string(),
            ],
            cache_enabled: true,
            cache_size: 1000,
            request_timeout_secs: 30,
            max_retries: 3,
            cost_currency: "USD".to_string(),
        }
    }
}
