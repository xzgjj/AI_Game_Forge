//! Wizard 持久化 IPC 接口
//! 将 WizardState 写入 SQLite 并支持加载最近记录

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use diesel::sql_types::Text;
use diesel::{RunQueryDsl, QueryableByName};

use crate::database;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveWizardStatePayload {
    pub project_root: String,
    pub wizard_state: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveWizardStateResult {
    pub project_root: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WizardStateRecord {
    pub project_root: String,
    pub wizard_state: serde_json::Value,
    pub updated_at: String,
}

#[derive(Debug, QueryableByName)]
struct WizardRow {
    #[diesel(sql_type = Text)]
    project_root: String,
    #[diesel(sql_type = Text)]
    wizard_state: String,
    #[diesel(sql_type = Text)]
    updated_at: String,
}

#[tauri::command]
pub async fn save_wizard_state(
    app_handle: AppHandle,
    payload: SaveWizardStatePayload,
) -> Result<SaveWizardStateResult, String> {
    let project_root = payload.project_root.trim();
    if project_root.is_empty() {
        return Err("项目路径不能为空".to_string());
    }

    let updated_at = chrono::Utc::now().to_rfc3339();
    let wizard_state = serde_json::to_string(&payload.wizard_state)
        .map_err(|err| format!("序列化失败: {}", err))?;

    let mut conn = database::get_connection(&app_handle)
        .map_err(|err| format!("数据库连接失败: {}", err))?;

    let sql = r#"
        INSERT INTO wizard_states (project_root, wizard_state, updated_at)
        VALUES (?1, ?2, ?3)
        ON CONFLICT(project_root) DO UPDATE SET
            wizard_state = excluded.wizard_state,
            updated_at = excluded.updated_at
    "#;

    diesel::sql_query(sql)
        .bind::<Text, _>(project_root)
        .bind::<Text, _>(wizard_state)
        .bind::<Text, _>(&updated_at)
        .execute(&mut conn)
        .map_err(|err| format!("写入失败: {}", err))?;

    Ok(SaveWizardStateResult {
        project_root: project_root.to_string(),
        updated_at,
    })
}

#[tauri::command]
pub async fn load_latest_wizard_state(
    app_handle: AppHandle,
) -> Result<Option<WizardStateRecord>, String> {
    let mut conn = database::get_connection(&app_handle)
        .map_err(|err| format!("数据库连接失败: {}", err))?;

    let sql = r#"
        SELECT project_root, wizard_state, updated_at
        FROM wizard_states
        ORDER BY updated_at DESC
        LIMIT 1
    "#;

    let rows: Vec<WizardRow> = diesel::sql_query(sql)
        .load(&mut conn)
        .map_err(|err| format!("查询失败: {}", err))?;

    if rows.is_empty() {
        return Ok(None);
    }

    let row = &rows[0];
    let wizard_state: serde_json::Value = serde_json::from_str(&row.wizard_state)
        .map_err(|err| format!("解析失败: {}", err))?;

    Ok(Some(WizardStateRecord {
        project_root: row.project_root.clone(),
        wizard_state,
        updated_at: row.updated_at.clone(),
    }))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadWizardStatePayload {
    pub project_root: String,
}

#[tauri::command]
pub async fn load_wizard_state_by_project(
    app_handle: AppHandle,
    payload: LoadWizardStatePayload,
) -> Result<Option<WizardStateRecord>, String> {
    let project_root = payload.project_root.trim();
    if project_root.is_empty() {
        return Err("项目路径不能为空".to_string());
    }

    let mut conn = database::get_connection(&app_handle)
        .map_err(|err| format!("数据库连接失败: {}", err))?;

    let sql = r#"
        SELECT project_root, wizard_state, updated_at
        FROM wizard_states
        WHERE project_root = ?1
        LIMIT 1
    "#;

    let rows: Vec<WizardRow> = diesel::sql_query(sql)
        .bind::<Text, _>(project_root)
        .load(&mut conn)
        .map_err(|err| format!("查询失败: {}", err))?;

    if rows.is_empty() {
        return Ok(None);
    }

    let row = &rows[0];
    let wizard_state: serde_json::Value = serde_json::from_str(&row.wizard_state)
        .map_err(|err| format!("解析失败: {}", err))?;

    Ok(Some(WizardStateRecord {
        project_root: row.project_root.clone(),
        wizard_state,
        updated_at: row.updated_at.clone(),
    }))
}
