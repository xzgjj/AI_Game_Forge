//! 服务模块
//! 包含所有业务逻辑服务

pub mod auth_service;
pub mod api_mgmt_service;
pub mod game_config_service;
pub mod ai_collab_service;
pub mod user_service;
pub mod audit_service;
pub mod provider_manager;

use tauri::AppHandle;
use anyhow::Result;

/// 初始化所有服务
pub fn init(_app: &AppHandle) -> Result<()> {
    log::info!("Initializing services...");

    // TODO: 初始化各个服务

    Ok(())
}
