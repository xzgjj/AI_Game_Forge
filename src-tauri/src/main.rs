#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// IPC模块
mod ipc;
mod services;
mod models;
mod database;
mod providers;
mod utils;

use ipc::*;
use tauri::Manager;

fn main() {
    // 初始化日志
    env_logger::init();
    log::info!("Starting GameCraft AI Studio...");

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            app.get_window("main").unwrap().show().unwrap();
        }))
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            // 初始化数据库
            database::init(&app.handle())?;

            // 初始化服务
            services::init(&app.handle())?;

            // 创建主窗口
            let window = tauri::WindowBuilder::new(
                app,
                "main",
                tauri::WindowUrl::App("index.html".into())
            )
            .title("GameCraft AI Studio")
            .inner_size(1280.0, 800.0)
            .min_inner_size(1024.0, 768.0)
            .build()?;

            // 设置系统托盘
            #[cfg(any(target_os = "windows", target_os = "macos"))]
            {
                use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem};

                let quit = CustomMenuItem::new("quit".to_string(), "退出");
                let show = CustomMenuItem::new("show".to_string(), "显示窗口");
                let hide = CustomMenuItem::new("hide".to_string(), "隐藏窗口");

                let tray_menu = SystemTrayMenu::new()
                    .add_item(show)
                    .add_item(hide)
                    .add_native_item(SystemTrayMenuItem::Separator)
                    .add_item(quit);

                let system_tray = SystemTray::new().with_menu(tray_menu);
                app.set_system_tray(system_tray)?;
            }

            Ok(())
        })
        // 注册IPC命令
        .invoke_handler(tauri::generate_handler![
            // 认证相关
            ipc::auth::login,
            ipc::auth::logout,
            ipc::auth::register_email,
            // API统计相关
            ipc::api_stats::get_usage_stats,
            ipc::api_stats::set_budget_limit,
            ipc::api_stats::get_budget_alerts,
            // 游戏配置相关
            ipc::game_config::create_game_config,
            ipc::game_config::update_game_config,
            ipc::game_config::get_game_config,
            // AI引擎相关
            ipc::ai_engine::generate_content,
            ipc::ai_engine::regenerate_content,
            ipc::ai_engine::get_generation_history,
            // 项目相关
            ipc::project::create_project,
            ipc::project::save_project,
            ipc::project::load_project,
            ipc::project::export_project,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
