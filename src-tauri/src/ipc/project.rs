//! 项目管理接口模块
//! 处理项目的创建、保存、加载、导出等操作

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::models::project as model;
use crate::services::ServiceContainer;

/// 项目状态
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ProjectStatus {
    Draft,      // 草稿
    Active,     // 活跃
    Archived,   // 已归档
    Exported,   // 已导出
    Deleted,    // 已删除
}

/// 项目信息
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectInfo {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: ProjectStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    pub version_count: u32,
    pub total_cost: f64,
    pub tags: Vec<String>,
}

/// 项目版本
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectVersion {
    pub version: u32,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub author: String,
    pub changes: serde_json::Value,
}

/// 导出格式
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ExportFormat {
    Unity,      // Unity项目
    Unreal,     // Unreal Engine项目
    Godot,      // Godot项目
    Json,       // 原始JSON数据
    Html,       // Web可玩版本
    Standalone, // 独立可执行文件
}

fn to_model_status(status: ProjectStatus) -> model::ProjectStatus {
    match status {
        ProjectStatus::Draft => model::ProjectStatus::Draft,
        ProjectStatus::Active => model::ProjectStatus::Active,
        ProjectStatus::Archived => model::ProjectStatus::Archived,
        ProjectStatus::Exported => model::ProjectStatus::Exported,
        ProjectStatus::Deleted => model::ProjectStatus::Deleted,
    }
}

fn from_model_status(status: model::ProjectStatus) -> ProjectStatus {
    match status {
        model::ProjectStatus::Draft => ProjectStatus::Draft,
        model::ProjectStatus::Active => ProjectStatus::Active,
        model::ProjectStatus::Archived => ProjectStatus::Archived,
        model::ProjectStatus::Exported => ProjectStatus::Exported,
        model::ProjectStatus::Deleted => ProjectStatus::Deleted,
    }
}

fn to_project_info(project: model::Project) -> ProjectInfo {
    ProjectInfo {
        id: project.id,
        name: project.name,
        description: project.description,
        status: from_model_status(project.status),
        created_at: project.created_at,
        updated_at: project.updated_at,
        last_accessed: project.last_accessed,
        version_count: project.version,
        total_cost: project.total_cost,
        tags: project.tags,
    }
}

fn export_format_name(format: &ExportFormat) -> &'static str {
    match format {
        ExportFormat::Unity => "unity",
        ExportFormat::Unreal => "unreal",
        ExportFormat::Godot => "godot",
        ExportFormat::Json => "json",
        ExportFormat::Html => "html",
        ExportFormat::Standalone => "standalone",
    }
}

fn export_extension(format: &ExportFormat) -> &'static str {
    match format {
        ExportFormat::Json => "json",
        ExportFormat::Html => "html",
        ExportFormat::Unity | ExportFormat::Unreal | ExportFormat::Godot | ExportFormat::Standalone => "zip",
    }
}

/// 创建项目接口
#[tauri::command]
pub async fn create_project(
    app_handle: AppHandle,
    name: String,
    description: Option<String>,
    template_id: Option<Uuid>,
) -> Result<ProjectInfo, String> {
    log::info!("Creating project: {}", name);
    let services = app_handle.state::<ServiceContainer>();

    services
        .project_service
        .create_project(name, description, template_id)
        .map(to_project_info)
        .map_err(|error| error.to_string())
}

/// 保存项目接口
#[tauri::command]
pub async fn save_project(
    app_handle: AppHandle,
    project_id: Uuid,
    description: Option<String>,
    changes: serde_json::Value,
) -> Result<ProjectVersion, String> {
    log::info!("Saving project: {}", project_id);
    let services = app_handle.state::<ServiceContainer>();

    services
        .project_service
        .save_project(project_id, description, changes)
        .map(|version| ProjectVersion {
            version: version.version,
            description: version.description,
            created_at: version.created_at,
            author: version.author,
            changes: version.changes,
        })
        .map_err(|error| error.to_string())
}

/// 加载项目接口
#[tauri::command]
pub async fn load_project(
    app_handle: AppHandle,
    project_id: Uuid,
    version: Option<u32>,
) -> Result<serde_json::Value, String> {
    log::info!("Loading project: {} version: {:?}", project_id, version);
    let services = app_handle.state::<ServiceContainer>();

    services
        .project_service
        .load_project(project_id, version)
        .map_err(|error| error.to_string())
}

/// 导出项目接口
#[tauri::command]
pub async fn export_project(
    app_handle: AppHandle,
    project_id: Uuid,
    format: ExportFormat,
    options: serde_json::Value,
) -> Result<String, String> {
    log::info!("Exporting project {} to format: {:?}", project_id, format);
    let services = app_handle.state::<ServiceContainer>();

    let payload = services
        .project_service
        .export_payload(project_id, export_format_name(&format), options)
        .map_err(|error| error.to_string())?;

    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;
    let export_dir = app_data_dir.join("exports");

    std::fs::create_dir_all(&export_dir).map_err(|error| error.to_string())?;

    let file_name = format!("{}_export.{}", project_id, export_extension(&format));
    let output_path = export_dir.join(file_name);

    match format {
        ExportFormat::Html => {
            let html = format!(
                "<!doctype html><html><head><meta charset=\"utf-8\"><title>GameCraft Export</title></head><body><pre>{}</pre></body></html>",
                serde_json::to_string_pretty(&payload).map_err(|error| error.to_string())?
            );
            std::fs::write(&output_path, html).map_err(|error| error.to_string())?;
        }
        _ => {
            let bytes = serde_json::to_vec_pretty(&payload).map_err(|error| error.to_string())?;
            std::fs::write(&output_path, bytes).map_err(|error| error.to_string())?;
        }
    }

    Ok(output_path.to_string_lossy().to_string())
}

/// 获取项目列表接口
#[tauri::command]
pub async fn get_project_list(
    app_handle: AppHandle,
    status_filter: Option<ProjectStatus>,
    tag_filter: Option<Vec<String>>,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<Vec<ProjectInfo>, String> {
    log::debug!("Getting project list with filters");
    let services = app_handle.state::<ServiceContainer>();

    let projects = services.project_service.list_projects(
        status_filter.map(to_model_status),
        tag_filter,
        limit,
        offset,
    );

    Ok(projects.into_iter().map(to_project_info).collect())
}

/// 删除项目接口
#[tauri::command]
pub async fn delete_project(
    app_handle: AppHandle,
    project_id: Uuid,
    permanent: bool,
) -> Result<bool, String> {
    log::info!("Deleting project: {} (permanent: {})", project_id, permanent);
    let services = app_handle.state::<ServiceContainer>();

    services
        .project_service
        .delete_project(project_id, permanent)
        .map_err(|error| error.to_string())
}

/// 恢复项目接口
#[tauri::command]
pub async fn restore_project(
    app_handle: AppHandle,
    project_id: Uuid,
) -> Result<ProjectInfo, String> {
    log::info!("Restoring project: {}", project_id);
    let services = app_handle.state::<ServiceContainer>();

    services
        .project_service
        .restore_project(project_id)
        .map(to_project_info)
        .map_err(|error| error.to_string())
}
