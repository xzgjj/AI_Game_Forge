//! IPC接口模块
//! 定义前端与后端通信的所有接口

pub mod auth;
pub mod api_stats;
pub mod game_config;
pub mod ai_engine;
pub mod project;
pub mod unity_bridge;
pub mod wizard;

// 重新导出常用类型
pub use auth::*;
pub use api_stats::*;
pub use game_config::*;
pub use ai_engine::*;
pub use project::*;
pub use unity_bridge::*;
pub use wizard::*;
