use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{anyhow, Result};
use regex::Regex;

pub const DEFAULT_PACKAGE_NAME: &str = "com.aigameforge.core";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnityInitRequest {
    pub project_root: String,
    pub unity_version: String,
    pub template_preset: String,
    pub scene_name: String,
    pub use_urp: bool,
    pub use_input_system: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnityUpmRequest {
    pub project_root: String,
    pub package_name: Option<String>,
    pub display_name: Option<String>,
    pub version: Option<String>,
    pub unity: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnityValidationRequest {
    pub project_root: String,
    pub require_package: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnityBatchValidateRequest {
    pub project_root: String,
    pub editor_path: String,
    pub log_file: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnityBridgeResult {
    pub project_root: String,
    pub created: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnityValidationReport {
    pub project_root: String,
    pub ok: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub checked_files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnityBatchValidateReport {
    pub project_root: String,
    pub ok: bool,
    pub exit_code: Option<i32>,
    pub log_file: Option<String>,
    pub log_tail: Option<String>,
    pub warnings: Vec<String>,
}

pub struct UnityBridgeService;

impl UnityBridgeService {
    pub fn new() -> Self {
        Self
    }

    pub fn init_project(&self, request: UnityInitRequest) -> Result<UnityBridgeResult> {
        let project_root = PathBuf::from(request.project_root.trim());
        if project_root.as_os_str().is_empty() {
            return Err(anyhow!("项目路径不能为空"));
        }

        let mut created = Vec::new();
        let mut warnings = Vec::new();

        create_dir_if_missing(&project_root, &mut created)?;
        let assets_dir = project_root.join("Assets");
        let scenes_dir = assets_dir.join("Scenes");
        let packages_dir = project_root.join("Packages");
        let settings_dir = project_root.join("ProjectSettings");

        for dir in [&assets_dir, &scenes_dir, &packages_dir, &settings_dir] {
            create_dir_if_missing(dir, &mut created)?;
        }

        let project_version_path = settings_dir.join("ProjectVersion.txt");
        if !project_version_path.exists() {
            let version_line = format!("m_EditorVersion: {}\n", request.unity_version.trim());
            fs::write(&project_version_path, version_line)?;
            created.push(project_version_path.display().to_string());
        }

        let scene_file = scenes_dir.join(format!("{}.unity", sanitize_scene_name(&request.scene_name)));
        if !scene_file.exists() {
            fs::write(&scene_file, default_scene_content(&request.scene_name))?;
            created.push(scene_file.display().to_string());
        }

        let manifest_path = packages_dir.join("manifest.json");
        if !manifest_path.exists() {
            let manifest = default_manifest();
            fs::write(&manifest_path, serde_json::to_string_pretty(&manifest)?)?;
            created.push(manifest_path.display().to_string());
        }

        if request.template_preset.to_lowercase().contains("urp") && !request.use_urp {
            warnings.push("模板为 URP，但 use_urp 为 false。请确认设置。".to_string());
        }

        Ok(UnityBridgeResult {
            project_root: project_root.display().to_string(),
            created,
            warnings,
        })
    }

    pub fn inject_upm(&self, request: UnityUpmRequest) -> Result<UnityBridgeResult> {
        let project_root = PathBuf::from(request.project_root.trim());
        ensure_project_root(&project_root)?;

        let packages_dir = project_root.join("Packages");
        let mut created = Vec::new();
        let mut warnings = Vec::new();
        create_dir_if_missing(&packages_dir, &mut created)?;

        let package_name = request
            .package_name
            .unwrap_or_else(|| DEFAULT_PACKAGE_NAME.to_string());
        let package_folder = packages_dir.join(&package_name);
        create_dir_if_missing(&package_folder, &mut created)?;

        let runtime_dir = package_folder.join("Runtime");
        create_dir_if_missing(&runtime_dir, &mut created)?;

        let editor_dir = package_folder.join("Editor");
        create_dir_if_missing(&editor_dir, &mut created)?;

        let package_json = package_folder.join("package.json");
        if !package_json.exists() {
            let package_meta = serde_json::json!({
                "name": package_name,
                "displayName": request.display_name.unwrap_or_else(|| "AI Game Forge Core".to_string()),
                "version": request.version.unwrap_or_else(|| "0.1.0".to_string()),
                "unity": request.unity.unwrap_or_else(|| "2022.3".to_string()),
                "description": request.description.unwrap_or_else(|| "AI Game Forge runtime package".to_string()),
                "author": {
                    "name": "AI Game Forge",
                    "url": "https://example.invalid"
                }
            });
            fs::write(&package_json, serde_json::to_string_pretty(&package_meta)?)?;
            created.push(package_json.display().to_string());
        }

        let bootstrap_script = runtime_dir.join("AIGameForgeBootstrap.cs");
        if !bootstrap_script.exists() {
            fs::write(&bootstrap_script, default_bootstrap_script())?;
            created.push(bootstrap_script.display().to_string());
        }

        let validator_script = editor_dir.join("BuildValidator.cs");
        if !validator_script.exists() {
            fs::write(&validator_script, default_validator_script())?;
            created.push(validator_script.display().to_string());
        }

        let manifest_path = packages_dir.join("manifest.json");
        let manifest_created = !manifest_path.exists();
        let mut manifest = if !manifest_created {
            serde_json::from_slice::<serde_json::Value>(&fs::read(&manifest_path)?)
                .unwrap_or_else(|_| default_manifest())
        } else {
            default_manifest()
        };

        let dependencies = manifest
            .as_object_mut()
            .and_then(|obj| obj.get_mut("dependencies"))
            .and_then(|deps| deps.as_object_mut())
            .ok_or_else(|| anyhow!("manifest.json 格式错误，缺少 dependencies"))?;

        let package_entry = format!("file:Packages/{}", package_name);
        let previous = dependencies.insert(package_name.clone(), serde_json::Value::String(package_entry));
        if previous.is_some() {
            warnings.push("UPM 包已存在，已更新为本地路径。".to_string());
        }

        fs::write(&manifest_path, serde_json::to_string_pretty(&manifest)?)?;
        if manifest_created {
            created.push(manifest_path.display().to_string());
        }

        Ok(UnityBridgeResult {
            project_root: project_root.display().to_string(),
            created,
            warnings,
        })
    }

    pub fn validate_project(&self, request: UnityValidationRequest) -> Result<UnityValidationReport> {
        let project_root = PathBuf::from(request.project_root.trim());
        ensure_project_root(&project_root)?;

        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut checked_files = Vec::new();

        let settings = project_root.join("ProjectSettings").join("ProjectVersion.txt");
        if !settings.exists() {
            errors.push("缺少 ProjectSettings/ProjectVersion.txt".to_string());
        } else {
            checked_files.push(settings.display().to_string());
        }

        let manifest_path = project_root.join("Packages").join("manifest.json");
        if !manifest_path.exists() {
            errors.push("缺少 Packages/manifest.json".to_string());
        } else {
            checked_files.push(manifest_path.display().to_string());
            if request.require_package {
                if let Ok(manifest) = serde_json::from_slice::<serde_json::Value>(&fs::read(&manifest_path)?) {
                    if let Some(deps) = manifest.get("dependencies") {
                        let has_pkg = deps
                            .as_object()
                            .and_then(|obj| obj.get(DEFAULT_PACKAGE_NAME))
                            .is_some();
                        if !has_pkg {
                            errors.push("manifest.json 未包含 com.aigameforge.core".to_string());
                        }
                    }
                }
            }
        }

        let package_dir = project_root.join("Packages").join(DEFAULT_PACKAGE_NAME);
        if request.require_package {
            if !package_dir.exists() {
                errors.push("缺少 Packages/com.aigameforge.core".to_string());
            }
        }

        if package_dir.exists() {
            let runtime_dir = package_dir.join("Runtime");
            if runtime_dir.exists() {
                for entry in fs::read_dir(&runtime_dir)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("cs") {
                        let content = fs::read_to_string(&path)?;
                        checked_files.push(path.display().to_string());
                        if let Some(warn) = validate_csharp(&content, &path) {
                            warnings.push(warn);
                        }
                    }
                }
            } else {
                warnings.push("UPM 包 Runtime 目录缺失".to_string());
            }
        }

        let ok = errors.is_empty();
        Ok(UnityValidationReport {
            project_root: project_root.display().to_string(),
            ok,
            errors,
            warnings,
            checked_files,
        })
    }

    pub fn batch_validate_project(&self, request: UnityBatchValidateRequest) -> Result<UnityBatchValidateReport> {
        let project_root = PathBuf::from(request.project_root.trim());
        ensure_project_root(&project_root)?;

        let editor_path = PathBuf::from(request.editor_path.trim());
        if editor_path.as_os_str().is_empty() {
            return Err(anyhow!("Unity Editor 路径不能为空"));
        }
        if !editor_path.exists() {
            return Err(anyhow!("Unity Editor 路径不存在: {}", editor_path.display()));
        }

        let logs_dir = project_root.join("Logs");
        create_dir_if_missing(&logs_dir, &mut Vec::new())?;
        let log_file = request
            .log_file
            .map(PathBuf::from)
            .unwrap_or_else(|| logs_dir.join("unity_batch_validate.log"));

        let status = Command::new(&editor_path)
            .arg("-batchmode")
            .arg("-quit")
            .arg("-projectPath")
            .arg(&project_root)
            .arg("-logFile")
            .arg(&log_file)
            .arg("-executeMethod")
            .arg("AIGameForge.Editor.BuildValidator.ValidateProject")
            .status()?;

        let ok = status.success();
        let exit_code = status.code();
        let log_tail = read_log_tail(&log_file, 6000);

        Ok(UnityBatchValidateReport {
            project_root: project_root.display().to_string(),
            ok,
            exit_code,
            log_file: Some(log_file.display().to_string()),
            log_tail,
            warnings: Vec::new(),
        })
    }
}

fn ensure_project_root(project_root: &Path) -> Result<()> {
    if !project_root.exists() {
        return Err(anyhow!("项目路径不存在: {}", project_root.display()));
    }
    Ok(())
}

fn create_dir_if_missing(path: &Path, created: &mut Vec<String>) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
        created.push(path.display().to_string());
    }
    Ok(())
}

fn sanitize_scene_name(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return "Main".to_string();
    }
    trimmed.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_")
}

fn default_manifest() -> serde_json::Value {
    serde_json::json!({
        "dependencies": {}
    })
}

fn default_scene_content(scene_name: &str) -> String {
    let sanitized = sanitize_scene_name(scene_name);
    format!(
        "%YAML 1.1\n%TAG !u! tag:unity3d.com,2011:\n--- !u!1 &1\nGameObject:\n  m_ObjectHideFlags: 0\n  m_CorrespondingSourceObject: {{fileID: 0}}\n  m_PrefabInstance: {{fileID: 0}}\n  m_PrefabAsset: {{fileID: 0}}\n  serializedVersion: 6\n  m_Name: \"{}\"\n  m_TagString: \"Untagged\"\n  m_Icon: {{fileID: 0}}\n  m_NavMeshLayer: 0\n  m_StaticEditorFlags: 0\n  m_IsActive: 1\n  m_Layer: 0\n  m_Component:\n  - component: {{fileID: 4}}\n  m_Children: []\n  m_Father: {{fileID: 0}}\n  m_RootOrder: 0\n  m_LocalEulerAnglesHint: {{x: 0, y: 0, z: 0}}\n--- !u!4 &4\nTransform:\n  m_ObjectHideFlags: 0\n  m_CorrespondingSourceObject: {{fileID: 0}}\n  m_PrefabInstance: {{fileID: 0}}\n  m_PrefabAsset: {{fileID: 0}}\n  m_GameObject: {{fileID: 1}}\n  m_LocalRotation: {{x: 0, y: 0, z: 0, w: 1}}\n  m_LocalPosition: {{x: 0, y: 0, z: 0}}\n  m_LocalScale: {{x: 1, y: 1, z: 1}}\n  m_Children: []\n  m_Father: {{fileID: 0}}\n  m_RootOrder: 0\n  m_LocalEulerAnglesHint: {{x: 0, y: 0, z: 0}}\n",
        sanitized
    )
}

fn default_bootstrap_script() -> String {
    r#"using UnityEngine;

namespace AIGameForge.Core
{
    public class AIGameForgeBootstrap : MonoBehaviour
    {
        [SerializeField] private string buildTag = "v0.1.0";

        private void Awake()
        {
            Debug.Log($"AIGameForge bootstrap active: {buildTag}");
        }
    }
}
"#
    .to_string()
}

fn default_validator_script() -> String {
    r#"using UnityEditor;
using UnityEngine;

namespace AIGameForge.Editor
{
    public static class BuildValidator
    {
        public static void ValidateProject()
        {
            Debug.Log("AIGameForge Batch validation started");
            AssetDatabase.Refresh();
            Debug.Log("AIGameForge Batch validation completed");
        }
    }
}
"#
    .to_string()
}

fn validate_csharp(content: &str, path: &Path) -> Option<String> {
    let mut balance = 0i32;
    for ch in content.chars() {
        if ch == '{' {
            balance += 1;
        } else if ch == '}' {
            balance -= 1;
        }
        if balance < 0 {
            return Some(format!("{} 可能存在花括号不匹配", path.display()));
        }
    }

    if balance != 0 {
        return Some(format!("{} 花括号数量不平衡", path.display()));
    }

    let class_regex = Regex::new(r"class\s+(\w+)").ok();
    if let Some(regex) = class_regex {
        if let Some(captures) = regex.captures(content) {
            if let Some(class_name) = captures.get(1) {
                let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
                if !file_stem.contains(class_name.as_str()) {
                    return Some(format!(
                        "{} 类名与文件名不一致：{}",
                        path.display(),
                        class_name.as_str()
                    ));
                }
            }
        }
    }

    None
}

fn read_log_tail(path: &Path, max_bytes: usize) -> Option<String> {
    if !path.exists() {
        return None;
    }
    let bytes = fs::read(path).ok()?;
    if bytes.is_empty() {
        return None;
    }
    let start = if bytes.len() > max_bytes {
        bytes.len() - max_bytes
    } else {
        0
    };
    String::from_utf8(bytes[start..].to_vec()).ok()
}
