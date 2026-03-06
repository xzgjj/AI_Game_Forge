//! 游戏配置模型
//! 定义游戏规格数据结构和相关功能

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// 游戏类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GameType {
    Rpg,        // RPG游戏
    Adventure,  // 冒险游戏
    Puzzle,     // 解谜游戏
    Simulation, // 模拟游戏
    Strategy,   // 策略游戏
    Action,     // 动作游戏
    Sports,     // 体育游戏
    Racing,     // 竞速游戏
    Fighting,   // 格斗游戏
    Horror,     // 恐怖游戏
    Other(String), // 其他类型
}

/// 美术风格
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ArtStyle {
    Pixel,      // 像素风格
    HandDrawn,  // 手绘风格
    Cartoon3D,  // 3D卡通风格
    Realistic,  // 写实风格
    Anime,      // 动漫风格
    LowPoly,    // 低多边形风格
    Stylized,   // 风格化
    Retro,      // 复古风格
    Other(String), // 其他风格
}

/// 叙事风格
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NarrativeStyle {
    Lighthearted, // 轻松幽默
    Epic,         // 史诗壮丽
    Suspenseful,  // 悬疑紧张
    Romantic,     // 浪漫爱情
    Dark,         // 黑暗风格
    Comedic,      // 喜剧风格
    Dramatic,     // 戏剧性
    Educational,  // 教育性
    Other(String), // 其他风格
}

/// 目标平台
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TargetPlatform {
    Pc,         // PC平台
    Mobile,     // 移动平台
    Console,    // 主机平台
    Web,        // Web平台
    MultiPlatform, // 多平台
    VrAr,       // VR/AR平台
}

/// 年龄分级
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgeRating {
    Everyone,   // 全年龄段
    Teen,       // 青少年
    Mature,     // 成熟
    AdultsOnly, // 仅成人
}

/// 游戏配置模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSpec {
    pub id: Uuid,
    pub project_id: Uuid,
    pub game_type: GameType,
    pub art_style: ArtStyle,
    pub narrative_style: NarrativeStyle,
    pub target_platform: TargetPlatform,
    pub age_rating: AgeRating,
    pub theme: String,
    pub setting: String,
    pub main_characters: Vec<CharacterSpec>,
    pub key_locations: Vec<LocationSpec>,
    pub core_mechanics: Vec<MechanicSpec>,
    pub story_outline: String,
    pub visual_references: Vec<String>, // 图片URL或路径
    pub audio_style: String,
    pub ui_style: String,
    pub advanced_settings: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: u32,
}

impl GameSpec {
    /// 创建新游戏配置
    pub fn new(
        project_id: Uuid,
        game_type: GameType,
        art_style: ArtStyle,
        narrative_style: NarrativeStyle,
        target_platform: TargetPlatform,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            project_id,
            game_type,
            art_style,
            narrative_style,
            target_platform,
            age_rating: AgeRating::Everyone,
            theme: String::new(),
            setting: String::new(),
            main_characters: Vec::new(),
            key_locations: Vec::new(),
            core_mechanics: Vec::new(),
            story_outline: String::new(),
            visual_references: Vec::new(),
            audio_style: String::new(),
            ui_style: String::new(),
            advanced_settings: serde_json::json!({}),
            created_at: now,
            updated_at: now,
            version: 1,
        }
    }

    /// 增加版本号
    pub fn increment_version(&mut self) {
        self.version += 1;
        self.updated_at = Utc::now();
    }

    /// 添加主要角色
    pub fn add_character(&mut self, character: CharacterSpec) {
        self.main_characters.push(character);
        self.updated_at = Utc::now();
    }

    /// 添加关键地点
    pub fn add_location(&mut self, location: LocationSpec) {
        self.key_locations.push(location);
        self.updated_at = Utc::now();
    }

    /// 添加核心机制
    pub fn add_mechanic(&mut self, mechanic: MechanicSpec) {
        self.core_mechanics.push(mechanic);
        self.updated_at = Utc::now();
    }

    /// 添加视觉参考
    pub fn add_visual_reference(&mut self, reference: String) {
        self.visual_references.push(reference);
        self.updated_at = Utc::now();
    }

    /// 生成配置摘要
    pub fn generate_summary(&self) -> String {
        format!(
            "{}游戏，采用{}美术风格和{}叙事风格，面向{}平台",
            self.game_type_to_string(),
            self.art_style_to_string(),
            self.narrative_style_to_string(),
            self.target_platform_to_string()
        )
    }

    /// 转换为提示词上下文
    pub fn to_prompt_context(&self) -> serde_json::Value {
        serde_json::json!({
            "game_type": self.game_type_to_string(),
            "art_style": self.art_style_to_string(),
            "narrative_style": self.narrative_style_to_string(),
            "target_platform": self.target_platform_to_string(),
            "age_rating": self.age_rating_to_string(),
            "theme": self.theme,
            "setting": self.setting,
            "main_characters": self.main_characters.iter().map(|c| c.to_prompt_context()).collect::<Vec<_>>(),
            "key_locations": self.key_locations.iter().map(|l| l.to_prompt_context()).collect::<Vec<_>>(),
            "core_mechanics": self.core_mechanics.iter().map(|m| m.to_prompt_context()).collect::<Vec<_>>(),
            "story_outline": self.story_outline,
            "audio_style": self.audio_style,
            "ui_style": self.ui_style,
        })
    }

    fn game_type_to_string(&self) -> String {
        match &self.game_type {
            GameType::Rpg => "RPG".to_string(),
            GameType::Adventure => "冒险".to_string(),
            GameType::Puzzle => "解谜".to_string(),
            GameType::Simulation => "模拟".to_string(),
            GameType::Strategy => "策略".to_string(),
            GameType::Action => "动作".to_string(),
            GameType::Sports => "体育".to_string(),
            GameType::Racing => "竞速".to_string(),
            GameType::Fighting => "格斗".to_string(),
            GameType::Horror => "恐怖".to_string(),
            GameType::Other(s) => s.clone(),
        }
    }

    fn art_style_to_string(&self) -> String {
        match &self.art_style {
            ArtStyle::Pixel => "像素".to_string(),
            ArtStyle::HandDrawn => "手绘".to_string(),
            ArtStyle::Cartoon3D => "3D卡通".to_string(),
            ArtStyle::Realistic => "写实".to_string(),
            ArtStyle::Anime => "动漫".to_string(),
            ArtStyle::LowPoly => "低多边形".to_string(),
            ArtStyle::Stylized => "风格化".to_string(),
            ArtStyle::Retro => "复古".to_string(),
            ArtStyle::Other(s) => s.clone(),
        }
    }

    fn narrative_style_to_string(&self) -> String {
        match &self.narrative_style {
            NarrativeStyle::Lighthearted => "轻松幽默".to_string(),
            NarrativeStyle::Epic => "史诗壮丽".to_string(),
            NarrativeStyle::Suspenseful => "悬疑紧张".to_string(),
            NarrativeStyle::Romantic => "浪漫爱情".to_string(),
            NarrativeStyle::Dark => "黑暗风格".to_string(),
            NarrativeStyle::Comedic => "喜剧风格".to_string(),
            NarrativeStyle::Dramatic => "戏剧性".to_string(),
            NarrativeStyle::Educational => "教育性".to_string(),
            NarrativeStyle::Other(s) => s.clone(),
        }
    }

    fn target_platform_to_string(&self) -> String {
        match &self.target_platform {
            TargetPlatform::Pc => "PC".to_string(),
            TargetPlatform::Mobile => "移动设备".to_string(),
            TargetPlatform::Console => "游戏主机".to_string(),
            TargetPlatform::Web => "网页".to_string(),
            TargetPlatform::MultiPlatform => "多平台".to_string(),
            TargetPlatform::VrAr => "VR/AR".to_string(),
        }
    }

    fn age_rating_to_string(&self) -> String {
        match self.age_rating {
            AgeRating::Everyone => "全年龄段".to_string(),
            AgeRating::Teen => "青少年".to_string(),
            AgeRating::Mature => "成熟".to_string(),
            AgeRating::AdultsOnly => "仅成人".to_string(),
        }
    }
}

/// 角色规格
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSpec {
    pub name: String,
    pub role: String, // 主角、反派、伙伴等
    pub age: Option<u32>,
    pub gender: Option<String>,
    pub appearance: String,
    pub personality: String,
    pub background: String,
    pub abilities: Vec<String>,
    pub relationships: Vec<CharacterRelationship>,
}

impl CharacterSpec {
    pub fn to_prompt_context(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "role": self.role,
            "age": self.age,
            "gender": self.gender,
            "appearance": self.appearance,
            "personality": self.personality,
            "background": self.background,
            "abilities": self.abilities,
        })
    }
}

/// 角色关系
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterRelationship {
    pub target_character: String,
    pub relationship_type: String, // 朋友、敌人、家人等
    pub description: String,
}

/// 地点规格
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationSpec {
    pub name: String,
    pub location_type: String, // 城市、森林、城堡等
    pub description: String,
    pub key_features: Vec<String>,
    pub atmosphere: String,
    pub significance: String,
}

impl LocationSpec {
    pub fn to_prompt_context(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "type": self.location_type,
            "description": self.description,
            "key_features": self.key_features,
            "atmosphere": self.atmosphere,
            "significance": self.significance,
        })
    }
}

/// 机制规格
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MechanicSpec {
    pub name: String,
    pub description: String,
    pub complexity: u32, // 1-5
    pub implementation_hints: Vec<String>,
}

impl MechanicSpec {
    pub fn to_prompt_context(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "description": self.description,
            "complexity": self.complexity,
            "implementation_hints": self.implementation_hints,
        })
    }
}
