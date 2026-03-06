//! 提供商管理器模块
//! 管理AI提供商的注册、发现和健康检查

use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;

use crate::providers::AIProvider;

/// 提供商管理器
pub struct ProviderManager {
    providers: HashMap<String, Arc<dyn AIProvider>>,
}

impl ProviderManager {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    pub fn register_provider(&mut self, name: String, provider: Arc<dyn AIProvider>) {
        self.providers.insert(name, provider);
    }

    pub fn get_provider(&self, name: &str) -> Option<Arc<dyn AIProvider>> {
        self.providers.get(name).cloned()
    }

    pub fn list_providers(&self) -> Vec<String> {
        self.providers.keys().cloned().collect()
    }

    pub async fn health_check(&self) -> HashMap<String, bool> {
        let mut results = HashMap::new();

        for (name, provider) in &self.providers {
            // TODO: 实现健康检查
            results.insert(name.clone(), true);
        }

        results
    }
}
