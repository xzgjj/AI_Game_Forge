#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod database;
mod ipc;
mod models;
mod providers;
mod services;
mod utils;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .setup(|app| {
            database::init(&app.handle())?;
            services::init(&app.handle())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ipc::auth::login,
            ipc::auth::logout,
            ipc::auth::register_email,
            ipc::auth::validate_session,
            ipc::auth::get_current_user,
            ipc::api_stats::get_usage_stats,
            ipc::api_stats::set_budget_limit,
            ipc::api_stats::get_budget_alerts,
            ipc::api_stats::get_provider_list,
            ipc::api_stats::reset_stats,
            ipc::game_config::create_game_config,
            ipc::game_config::update_game_config,
            ipc::game_config::get_game_config,
            ipc::ai_engine::generate_content,
            ipc::ai_engine::regenerate_content,
            ipc::ai_engine::get_generation_history,
            ipc::ai_engine::get_provider_status,
            ipc::project::create_project,
            ipc::project::save_project,
            ipc::project::load_project,
            ipc::project::export_project,
            ipc::project::get_project_list,
            ipc::project::delete_project,
            ipc::project::restore_project,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
