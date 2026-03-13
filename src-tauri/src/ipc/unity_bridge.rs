//! Unity Bridge IPC 接口
//! 初始化项目、注入 UPM 包、进行基础校验

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::services::unity_bridge_service::{
    UnityBatchValidateReport, UnityBatchValidateRequest, UnityBridgeResult, UnityBridgeService,
    UnityInitRequest, UnityUpmRequest, UnityValidationReport, UnityValidationRequest,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnityInitPayload {
    pub project_root: String,
    pub unity_version: String,
    pub template_preset: String,
    pub scene_name: String,
    pub use_urp: bool,
    pub use_input_system: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnityUpmPayload {
    pub project_root: String,
    pub package_name: Option<String>,
    pub display_name: Option<String>,
    pub version: Option<String>,
    pub unity: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnityValidatePayload {
    pub project_root: String,
    pub require_package: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnityBatchValidatePayload {
    pub project_root: String,
    pub editor_path: String,
    pub log_file: Option<String>,
}

#[tauri::command]
pub async fn unity_init_project(
    _app_handle: AppHandle,
    payload: UnityInitPayload,
) -> Result<UnityBridgeResult, String> {
    let service = UnityBridgeService::new();
    service
        .init_project(UnityInitRequest {
            project_root: payload.project_root,
            unity_version: payload.unity_version,
            template_preset: payload.template_preset,
            scene_name: payload.scene_name,
            use_urp: payload.use_urp,
            use_input_system: payload.use_input_system,
        })
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn unity_inject_upm(
    _app_handle: AppHandle,
    payload: UnityUpmPayload,
) -> Result<UnityBridgeResult, String> {
    let service = UnityBridgeService::new();
    service
        .inject_upm(UnityUpmRequest {
            project_root: payload.project_root,
            package_name: payload.package_name,
            display_name: payload.display_name,
            version: payload.version,
            unity: payload.unity,
            description: payload.description,
        })
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn unity_validate_project(
    _app_handle: AppHandle,
    payload: UnityValidatePayload,
) -> Result<UnityValidationReport, String> {
    let service = UnityBridgeService::new();
    service
        .validate_project(UnityValidationRequest {
            project_root: payload.project_root,
            require_package: payload.require_package,
        })
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn unity_batch_validate(
    _app_handle: AppHandle,
    payload: UnityBatchValidatePayload,
) -> Result<UnityBatchValidateReport, String> {
    let service = UnityBridgeService::new();
    service
        .batch_validate_project(UnityBatchValidateRequest {
            project_root: payload.project_root,
            editor_path: payload.editor_path,
            log_file: payload.log_file,
        })
        .map_err(|err| err.to_string())
}
