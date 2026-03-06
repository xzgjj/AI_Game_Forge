//! API管理服务模块
//! 处理多AI提供商的管理、智能路由、成本统计等

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use anyhow::{Result, anyhow};
use chrono::Utc;
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
        let stats = self.stats.read().unwrap();

        providers
            .iter()
            .filter_map(|name| {
                stats.get(name).map(|stat| {
                    let success_rate = if stat.total_requests > 0 {
                        stat.successful_requests as f64 / stat.total_requests as f64
                    } else {
                        1.0
                    };

                    let avg_cost = stat.total_cost / stat.total_requests.max(1) as f64;
                    let speed_score = 1.0 / (1.0 + stat.avg_response_time_ms / 1000.0);
                    let cost_score = 1.0 / (1.0 + avg_cost);

                    // 平衡评分: 成功率 > 速度 > 成本
                    let score = success_rate * 0.45 + speed_score * 0.35 + cost_score * 0.20;
                    (name, score)
                })
            })
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(name, _)| name.clone())
            .ok_or_else(|| anyhow!("无法选择平衡策略提供商"))
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

        {
            let mut history = self.request_history.write().unwrap();
            history.push_back(request);
            if history.len() > self.config.cache_size {
                history.pop_front();
            }
        }

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

        let stats = self.stats.read().unwrap();
        let mut by_provider = Vec::new();
        let mut total_requests: i64 = 0;
        let mut total_tokens: i64 = 0;
        let mut total_cost: f64 = 0.0;

        for provider in stats.values() {
            total_requests += to_i64(provider.total_requests);
            total_tokens += to_i64(provider.total_tokens);
            total_cost += provider.total_cost;

            by_provider.push(ProviderStats {
                name: provider.name.clone(),
                requests: to_i64(provider.total_requests),
                tokens: to_i64(provider.total_tokens),
                cost: provider.total_cost,
                success_rate: if provider.total_requests > 0 {
                    provider.successful_requests as f64 / provider.total_requests as f64
                } else {
                    0.0
                },
                avg_response_time: provider.avg_response_time_ms,
            });
        }

        by_provider.sort_by(|a, b| b.cost.partial_cmp(&a.cost).unwrap_or(std::cmp::Ordering::Equal));

        Ok(UsageStats {
            period,
            total_requests,
            total_tokens,
            total_cost,
            by_provider,
            by_project: Vec::new(),
            cost_trend: vec![DailyCost {
                date: Utc::now().date_naive(),
                cost: total_cost,
                tokens: total_tokens,
            }],
        })
    }

    /// 设置预算限制
    pub async fn set_budget_limit(&self, provider: String, monthly_limit: f64) -> Result<bool> {
        log::info!("Setting budget limit for {}: ${}", provider, monthly_limit);

        if monthly_limit <= 0.0 {
            return Err(anyhow!("预算上限必须大于0"));
        }

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
            if budget.monthly_limit <= 0.0 {
                continue;
            }

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

    /// 更新预算消耗（用于测试和离线统计）
    pub fn update_budget_spent(&self, provider: &str, amount: f64) {
        if amount <= 0.0 {
            return;
        }

        let mut budgets = self.budgets.write().unwrap();
        for budget in budgets.iter_mut() {
            if budget.provider_name == provider || budget.provider_name == "all" {
                budget.current_month_spent += amount;
            }
        }
    }

    /// 获取已注册提供商数量
    pub fn provider_count(&self) -> usize {
        self.providers.len()
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

fn to_i64(value: u64) -> i64 {
    i64::try_from(value).unwrap_or(i64::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use uuid::Uuid;

    struct MockProvider {
        provider_name: String,
        succeed: bool,
        cost: f64,
    }

    #[async_trait]
    impl AIProvider for MockProvider {
        fn name(&self) -> &str {
            &self.provider_name
        }

        fn display_name(&self) -> &str {
            &self.provider_name
        }

        async fn is_available(&self) -> bool {
            true
        }

        async fn generate(&self, request: AIGenerationRequest) -> Result<AIGenerationResponse> {
            if !self.succeed {
                return Err(anyhow!("provider failure"));
            }

            Ok(AIGenerationResponse {
                id: Uuid::new_v4(),
                request,
                content: "ok".to_string(),
                provider_used: self.provider_name.clone(),
                tokens_used: 100,
                cost: self.cost,
                generated_at: Utc::now(),
                metadata: serde_json::json!({}),
            })
        }

        fn get_config(&self) -> crate::providers::ProviderConfig {
            crate::providers::ProviderConfig {
                api_key: None,
                base_url: None,
                default_model: "mock".to_string(),
                max_tokens: 1024,
                temperature: 0.7,
                timeout_seconds: 30,
                cost_per_1k_input: 0.0,
                cost_per_1k_output: 0.0,
                enabled: true,
            }
        }

        async fn get_stats(&self) -> crate::providers::ProviderStats {
            crate::providers::ProviderStats {
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                total_tokens: 0,
                total_cost: 0.0,
                avg_response_time_ms: 0.0,
                last_used: None,
            }
        }

        async fn test_connection(&self) -> Result<()> {
            Ok(())
        }
    }

    fn sample_request() -> AIGenerationRequest {
        AIGenerationRequest {
            project_id: Uuid::new_v4(),
            content_type: crate::providers::ContentType::Dialogue,
            prompt: "test".to_string(),
            context: serde_json::json!({}),
            provider_preference: None,
            max_tokens: Some(64),
            temperature: Some(0.5),
        }
    }

    #[tokio::test]
    async fn smart_route_should_select_and_execute_provider() {
        let mut service = APIManagementService::new(APIMgmtConfig::default());
        service.register_provider(
            "openai".to_string(),
            Arc::new(MockProvider {
                provider_name: "openai".to_string(),
                succeed: true,
                cost: 0.02,
            }),
        );

        let response = service
            .smart_route(sample_request())
            .await
            .expect("smart route should succeed");

        assert_eq!(response.provider_used, "openai");
        assert_eq!(service.provider_count(), 1);
    }

    #[tokio::test]
    async fn budget_alerts_should_trigger() {
        let service = APIManagementService::new(APIMgmtConfig::default());
        service
            .set_budget_limit("all".to_string(), 100.0)
            .await
            .expect("set budget should succeed");
        service.update_budget_spent("all", 85.0);

        let alerts = service.check_budget_alerts().await;
        assert!(!alerts.is_empty());
        assert!(matches!(alerts[0].level, AlertLevel::Warning | AlertLevel::Danger));
    }
}
