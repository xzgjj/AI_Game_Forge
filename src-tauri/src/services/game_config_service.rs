//! 游戏配置服务模块
//! 处理游戏配置的业务逻辑

use std::collections::HashMap;
use std::sync::RwLock;

use anyhow::{anyhow, Result};
use uuid::Uuid;

use crate::models::game_spec::{
    ArtStyle, CharacterSpec, GameSpec, GameType, LocationSpec, MechanicSpec, NarrativeStyle,
    TargetPlatform,
};

/// 游戏配置服务
pub struct GameConfigService {
    specs_by_project: RwLock<HashMap<Uuid, GameSpec>>,
}

impl GameConfigService {
    pub fn new() -> Self {
        Self {
            specs_by_project: RwLock::new(HashMap::new()),
        }
    }

    /// 创建基础配置草稿
    pub fn create_draft(
        &self,
        project_id: Uuid,
        game_type: GameType,
        art_style: ArtStyle,
        narrative_style: NarrativeStyle,
        target_platform: TargetPlatform,
    ) -> Result<GameSpec> {
        let mut specs = self
            .specs_by_project
            .write()
            .expect("spec lock poisoned");

        if specs.contains_key(&project_id) {
            return Err(anyhow!("项目配置已存在"));
        }

        let spec = GameSpec::new(
            project_id,
            game_type,
            art_style,
            narrative_style,
            target_platform,
        );

        specs.insert(project_id, spec.clone());
        Ok(spec)
    }

    /// 保存或覆盖配置
    pub fn save(&self, spec: GameSpec) -> Result<GameSpec> {
        if spec.theme.trim().is_empty() && spec.setting.trim().is_empty() {
            return Err(anyhow!("主题和设定不能同时为空"));
        }

        let mut specs = self
            .specs_by_project
            .write()
            .expect("spec lock poisoned");

        specs.insert(spec.project_id, spec.clone());
        Ok(spec)
    }

    /// 获取项目配置
    pub fn get_by_project(&self, project_id: Uuid) -> Option<GameSpec> {
        self.specs_by_project
            .read()
            .expect("spec lock poisoned")
            .get(&project_id)
            .cloned()
    }

    /// 更新文案字段
    pub fn update_story_fields(
        &self,
        project_id: Uuid,
        theme: String,
        setting: String,
        story_outline: String,
    ) -> Result<GameSpec> {
        if theme.trim().is_empty() {
            return Err(anyhow!("主题不能为空"));
        }

        let mut specs = self
            .specs_by_project
            .write()
            .expect("spec lock poisoned");

        let spec = specs
            .get_mut(&project_id)
            .ok_or_else(|| anyhow!("项目配置不存在"))?;

        spec.theme = theme;
        spec.setting = setting;
        spec.story_outline = story_outline;
        spec.increment_version();

        Ok(spec.clone())
    }

    pub fn add_character(&self, project_id: Uuid, character: CharacterSpec) -> Result<GameSpec> {
        let mut specs = self
            .specs_by_project
            .write()
            .expect("spec lock poisoned");

        let spec = specs
            .get_mut(&project_id)
            .ok_or_else(|| anyhow!("项目配置不存在"))?;

        spec.add_character(character);
        spec.increment_version();
        Ok(spec.clone())
    }

    pub fn add_location(&self, project_id: Uuid, location: LocationSpec) -> Result<GameSpec> {
        let mut specs = self
            .specs_by_project
            .write()
            .expect("spec lock poisoned");

        let spec = specs
            .get_mut(&project_id)
            .ok_or_else(|| anyhow!("项目配置不存在"))?;

        spec.add_location(location);
        spec.increment_version();
        Ok(spec.clone())
    }

    pub fn add_mechanic(&self, project_id: Uuid, mechanic: MechanicSpec) -> Result<GameSpec> {
        let mut specs = self
            .specs_by_project
            .write()
            .expect("spec lock poisoned");

        let spec = specs
            .get_mut(&project_id)
            .ok_or_else(|| anyhow!("项目配置不存在"))?;

        spec.add_mechanic(mechanic);
        spec.increment_version();
        Ok(spec.clone())
    }

    /// 列出配置
    pub fn list(&self, limit: Option<usize>, offset: Option<usize>) -> Vec<GameSpec> {
        let specs = self.specs_by_project.read().expect("spec lock poisoned");

        let mut values = specs.values().cloned().collect::<Vec<_>>();
        values.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

        let start = offset.unwrap_or(0);
        let end = limit.map(|l| start + l).unwrap_or(values.len());

        values
            .into_iter()
            .skip(start)
            .take(end.saturating_sub(start))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_update_spec_should_work() {
        let service = GameConfigService::new();
        let project_id = Uuid::new_v4();

        let created = service
            .create_draft(
                project_id,
                GameType::Rpg,
                ArtStyle::Pixel,
                NarrativeStyle::Epic,
                TargetPlatform::Pc,
            )
            .expect("draft should be created");

        assert_eq!(created.project_id, project_id);

        let updated = service
            .update_story_fields(
                project_id,
                "魔法大陆".to_string(),
                "王都与边境".to_string(),
                "主角寻找失落王冠".to_string(),
            )
            .expect("story should update");

        assert_eq!(updated.theme, "魔法大陆");
        assert!(updated.version >= 2);
    }

    #[test]
    fn add_domain_objects_should_increment_version() {
        let service = GameConfigService::new();
        let project_id = Uuid::new_v4();

        let _ = service
            .create_draft(
                project_id,
                GameType::Adventure,
                ArtStyle::HandDrawn,
                NarrativeStyle::Suspenseful,
                TargetPlatform::Web,
            )
            .expect("draft should be created");

        let character = CharacterSpec {
            name: "伊芙".to_string(),
            role: "主角".to_string(),
            age: Some(21),
            gender: Some("女".to_string()),
            appearance: "银发斗篷".to_string(),
            personality: "冷静".to_string(),
            background: "流亡贵族".to_string(),
            abilities: vec!["火焰术".to_string()],
            relationships: vec![],
        };

        let with_character = service
            .add_character(project_id, character)
            .expect("character should be added");

        assert_eq!(with_character.main_characters.len(), 1);

        let location = LocationSpec {
            name: "迷雾港".to_string(),
            location_type: "港口".to_string(),
            description: "常年浓雾".to_string(),
            key_features: vec!["灯塔".to_string()],
            atmosphere: "压抑".to_string(),
            significance: "主线起点".to_string(),
        };

        let with_location = service
            .add_location(project_id, location)
            .expect("location should be added");

        assert_eq!(with_location.key_locations.len(), 1);
    }
}
