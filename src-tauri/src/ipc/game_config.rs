//! 游戏配置接口模块
//! 处理游戏配置的创建、更新、查询等操作

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::models::game_spec as model;
use crate::services::ServiceContainer;

/// 游戏类型枚举
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GameType {
    Rpg,
    Adventure,
    Puzzle,
    Simulation,
    Strategy,
    Action,
    Other(String),
}

/// 美术风格枚举
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ArtStyle {
    Pixel,
    HandDrawn,
    Cartoon3D,
    Realistic,
    Anime,
    Other(String),
}

/// 叙事风格枚举
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NarrativeStyle {
    Lighthearted,
    Epic,
    Suspenseful,
    Romantic,
    Dark,
    Other(String),
}

/// 目标平台枚举
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TargetPlatform {
    Pc,
    Mobile,
    Console,
    Web,
    MultiPlatform,
}

/// 游戏配置
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameConfig {
    pub id: Uuid,
    pub project_id: Uuid,
    pub game_type: GameType,
    pub art_style: ArtStyle,
    pub narrative_style: NarrativeStyle,
    pub target_platform: TargetPlatform,
    pub advanced_settings: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

fn to_model_game_type(value: GameType) -> model::GameType {
    match value {
        GameType::Rpg => model::GameType::Rpg,
        GameType::Adventure => model::GameType::Adventure,
        GameType::Puzzle => model::GameType::Puzzle,
        GameType::Simulation => model::GameType::Simulation,
        GameType::Strategy => model::GameType::Strategy,
        GameType::Action => model::GameType::Action,
        GameType::Other(v) => model::GameType::Other(v),
    }
}

fn to_model_art_style(value: ArtStyle) -> model::ArtStyle {
    match value {
        ArtStyle::Pixel => model::ArtStyle::Pixel,
        ArtStyle::HandDrawn => model::ArtStyle::HandDrawn,
        ArtStyle::Cartoon3D => model::ArtStyle::Cartoon3D,
        ArtStyle::Realistic => model::ArtStyle::Realistic,
        ArtStyle::Anime => model::ArtStyle::Anime,
        ArtStyle::Other(v) => model::ArtStyle::Other(v),
    }
}

fn to_model_narrative_style(value: NarrativeStyle) -> model::NarrativeStyle {
    match value {
        NarrativeStyle::Lighthearted => model::NarrativeStyle::Lighthearted,
        NarrativeStyle::Epic => model::NarrativeStyle::Epic,
        NarrativeStyle::Suspenseful => model::NarrativeStyle::Suspenseful,
        NarrativeStyle::Romantic => model::NarrativeStyle::Romantic,
        NarrativeStyle::Dark => model::NarrativeStyle::Dark,
        NarrativeStyle::Other(v) => model::NarrativeStyle::Other(v),
    }
}

fn to_model_target_platform(value: TargetPlatform) -> model::TargetPlatform {
    match value {
        TargetPlatform::Pc => model::TargetPlatform::Pc,
        TargetPlatform::Mobile => model::TargetPlatform::Mobile,
        TargetPlatform::Console => model::TargetPlatform::Console,
        TargetPlatform::Web => model::TargetPlatform::Web,
        TargetPlatform::MultiPlatform => model::TargetPlatform::MultiPlatform,
    }
}

fn from_model_game_type(value: model::GameType) -> GameType {
    match value {
        model::GameType::Rpg => GameType::Rpg,
        model::GameType::Adventure => GameType::Adventure,
        model::GameType::Puzzle => GameType::Puzzle,
        model::GameType::Simulation => GameType::Simulation,
        model::GameType::Strategy => GameType::Strategy,
        model::GameType::Action => GameType::Action,
        model::GameType::Other(v) => GameType::Other(v),
        _ => GameType::Other("Unsupported".to_string()),
    }
}

fn from_model_art_style(value: model::ArtStyle) -> ArtStyle {
    match value {
        model::ArtStyle::Pixel => ArtStyle::Pixel,
        model::ArtStyle::HandDrawn => ArtStyle::HandDrawn,
        model::ArtStyle::Cartoon3D => ArtStyle::Cartoon3D,
        model::ArtStyle::Realistic => ArtStyle::Realistic,
        model::ArtStyle::Anime => ArtStyle::Anime,
        model::ArtStyle::Other(v) => ArtStyle::Other(v),
        _ => ArtStyle::Other("Unsupported".to_string()),
    }
}

fn from_model_narrative_style(value: model::NarrativeStyle) -> NarrativeStyle {
    match value {
        model::NarrativeStyle::Lighthearted => NarrativeStyle::Lighthearted,
        model::NarrativeStyle::Epic => NarrativeStyle::Epic,
        model::NarrativeStyle::Suspenseful => NarrativeStyle::Suspenseful,
        model::NarrativeStyle::Romantic => NarrativeStyle::Romantic,
        model::NarrativeStyle::Dark => NarrativeStyle::Dark,
        model::NarrativeStyle::Other(v) => NarrativeStyle::Other(v),
        _ => NarrativeStyle::Other("Unsupported".to_string()),
    }
}

fn from_model_target_platform(value: model::TargetPlatform) -> TargetPlatform {
    match value {
        model::TargetPlatform::Pc => TargetPlatform::Pc,
        model::TargetPlatform::Mobile => TargetPlatform::Mobile,
        model::TargetPlatform::Console => TargetPlatform::Console,
        model::TargetPlatform::Web => TargetPlatform::Web,
        model::TargetPlatform::MultiPlatform => TargetPlatform::MultiPlatform,
        _ => TargetPlatform::Pc,
    }
}

fn from_model_config(spec: model::GameSpec) -> GameConfig {
    GameConfig {
        id: spec.id,
        project_id: spec.project_id,
        game_type: from_model_game_type(spec.game_type),
        art_style: from_model_art_style(spec.art_style),
        narrative_style: from_model_narrative_style(spec.narrative_style),
        target_platform: from_model_target_platform(spec.target_platform),
        advanced_settings: spec.advanced_settings,
        created_at: spec.created_at,
        updated_at: spec.updated_at,
    }
}

#[tauri::command]
pub async fn create_game_config(
    app_handle: AppHandle,
    project_id: Uuid,
    game_type: GameType,
    art_style: ArtStyle,
    narrative_style: NarrativeStyle,
    target_platform: TargetPlatform,
    advanced_settings: serde_json::Value,
) -> Result<GameConfig, String> {
    let services = app_handle.state::<ServiceContainer>();

    let mut spec = services
        .game_config_service
        .create_draft(
            project_id,
            to_model_game_type(game_type),
            to_model_art_style(art_style),
            to_model_narrative_style(narrative_style),
            to_model_target_platform(target_platform),
        )
        .map_err(|error| error.to_string())?;

    spec.advanced_settings = advanced_settings;

    Ok(from_model_config(spec))
}

#[tauri::command]
pub async fn update_game_config(
    app_handle: AppHandle,
    config_id: Uuid,
    updates: serde_json::Value,
) -> Result<GameConfig, String> {
    let services = app_handle.state::<ServiceContainer>();

    let mut spec = services
        .game_config_service
        .list(None, None)
        .into_iter()
        .find(|item| item.id == config_id)
        .ok_or_else(|| "游戏配置不存在".to_string())?;

    if let Some(theme) = updates.get("theme").and_then(serde_json::Value::as_str) {
        spec.theme = theme.to_string();
    }

    if let Some(setting) = updates.get("setting").and_then(serde_json::Value::as_str) {
        spec.setting = setting.to_string();
    }

    if let Some(story_outline) = updates
        .get("story_outline")
        .and_then(serde_json::Value::as_str)
    {
        spec.story_outline = story_outline.to_string();
    }

    if let Some(advanced) = updates.get("advanced_settings") {
        spec.advanced_settings = advanced.clone();
    }

    if spec.theme.trim().is_empty() && spec.setting.trim().is_empty() {
        spec.theme = "默认主题".to_string();
    }

    let saved = services
        .game_config_service
        .save(spec)
        .map_err(|error| error.to_string())?;

    Ok(from_model_config(saved))
}

#[tauri::command]
pub async fn get_game_config(app_handle: AppHandle, config_id: Uuid) -> Result<GameConfig, String> {
    let services = app_handle.state::<ServiceContainer>();

    services
        .game_config_service
        .list(None, None)
        .into_iter()
        .find(|item| item.id == config_id)
        .map(from_model_config)
        .ok_or_else(|| "游戏配置不存在".to_string())
}
