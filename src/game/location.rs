use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// 环境类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnvironmentType {
    Grassland,   // 草地 - 速度+10%
    Forest,      // 森林 - 防守+10%
    Cave,        // 洞穴 - 特攻+10%
    Water,       // 水域 - 特防+10%
    Mountain,    // 山地 - 攻击+10%
    City,        // 城市 - 全属性+5%
}

impl EnvironmentType {
    /// 获取环境的中文名称
    pub fn name(&self) -> &str {
        match self {
            EnvironmentType::Grassland => "草地",
            EnvironmentType::Forest => "森林",
            EnvironmentType::Cave => "洞穴",
            EnvironmentType::Water => "水域",
            EnvironmentType::Mountain => "山地",
            EnvironmentType::City => "城市",
        }
    }
}

/// 环境属性加成
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EnvironmentBonus {
    pub attack_bonus: f32,      // 默认 1.0
    pub defense_bonus: f32,
    pub sp_attack_bonus: f32,
    pub sp_defense_bonus: f32,
    pub speed_bonus: f32,
}

impl Default for EnvironmentBonus {
    fn default() -> Self {
        EnvironmentBonus {
            attack_bonus: 1.0,
            defense_bonus: 1.0,
            sp_attack_bonus: 1.0,
            sp_defense_bonus: 1.0,
            speed_bonus: 1.0,
        }
    }
}

impl EnvironmentBonus {
    /// 从环境类型创建加成
    pub fn from_environment(env: EnvironmentType) -> Self {
        match env {
            EnvironmentType::Grassland => EnvironmentBonus {
                speed_bonus: 1.1,
                ..Default::default()
            },
            EnvironmentType::Forest => EnvironmentBonus {
                defense_bonus: 1.1,
                ..Default::default()
            },
            EnvironmentType::Cave => EnvironmentBonus {
                sp_attack_bonus: 1.1,
                ..Default::default()
            },
            EnvironmentType::Water => EnvironmentBonus {
                sp_defense_bonus: 1.1,
                ..Default::default()
            },
            EnvironmentType::Mountain => EnvironmentBonus {
                attack_bonus: 1.1,
                ..Default::default()
            },
            EnvironmentType::City => EnvironmentBonus {
                attack_bonus: 1.05,
                defense_bonus: 1.05,
                sp_attack_bonus: 1.05,
                sp_defense_bonus: 1.05,
                speed_bonus: 1.05,
            },
        }
    }

    /// 对属性应用加成
    pub fn apply_to_stat(&self, stat_name: &str, base_value: u32) -> u32 {
        let multiplier = match stat_name {
            "attack" => self.attack_bonus,
            "defense" => self.defense_bonus,
            "sp_attack" => self.sp_attack_bonus,
            "sp_defense" => self.sp_defense_bonus,
            "speed" => self.speed_bonus,
            _ => 1.0,
        };
        (base_value as f32 * multiplier).round() as u32
    }
}

/// 地点解锁条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationRequirement {
    pub required_level: u32,
    pub required_badges: Vec<u32>,
    pub required_pokemon_count: Option<u32>,
}

impl Default for LocationRequirement {
    fn default() -> Self {
        LocationRequirement {
            required_level: 1,
            required_badges: Vec::new(),
            required_pokemon_count: None,
        }
    }
}

impl LocationRequirement {
    /// 生成友好的条件文本
    pub fn get_condition_text(&self) -> String {
        let mut conditions = Vec::new();

        if self.required_level > 1 {
            conditions.push(format!("等级 ≥ {}", self.required_level));
        }

        if !self.required_badges.is_empty() {
            conditions.push(format!("徽章 ≥ {}", self.required_badges.len()));
        }

        if let Some(count) = self.required_pokemon_count {
            conditions.push(format!("宝可梦 ≥ {}", count));
        }

        if conditions.is_empty() {
            "无条件".to_string()
        } else {
            conditions.join(" + ")
        }
    }
}

/// 野生宝可梦生成信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WildPokemonSpawn {
    pub pokemon_id: u32,
    pub spawn_rate: f32,     // 相对权重
    pub level_min: u32,
    pub level_max: u32,
}

/// 地点信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: u32,
    pub name: String,
    pub description: String,

    // 连接关系
    pub connected_locations: Vec<u32>,

    // 环境相关
    pub environment: EnvironmentType,
    pub stat_bonus: EnvironmentBonus,

    // 野生宝可梦相关
    pub wild_pokemon_pool: Vec<WildPokemonSpawn>,
    pub encounter_rate: f32, // 遭遇率 (0.0 - 1.0)

    // 解锁相关
    pub unlock_requirement: LocationRequirement,
    pub npc_trainers: Vec<u32>,

    // 游戏开始相关
    pub is_starting_location: bool,
}

impl Location {
    /// 创建新地点
    pub fn new(
        id: u32,
        name: String,
        description: String,
        environment: EnvironmentType,
    ) -> Self {
        let stat_bonus = EnvironmentBonus::from_environment(environment);

        Location {
            id,
            name,
            description,
            connected_locations: Vec::new(),
            environment,
            stat_bonus,
            wild_pokemon_pool: Vec::new(),
            encounter_rate: 0.7,
            unlock_requirement: LocationRequirement::default(),
            npc_trainers: Vec::new(),
            is_starting_location: false,
        }
    }

    /// 添加连接地点
    pub fn add_connection(&mut self, location_id: u32) {
        if !self.connected_locations.contains(&location_id) {
            self.connected_locations.push(location_id);
        }
    }

    /// 添加野生宝可梦
    pub fn add_wild_pokemon(&mut self, spawn: WildPokemonSpawn) {
        self.wild_pokemon_pool.push(spawn);
    }

    /// 获取环境名称
    pub fn environment_name(&self) -> &str {
        self.environment.name()
    }
}

/// 玩家位置状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerLocationState {
    pub current_location_id: u32,
    pub unlocked_locations: HashSet<u32>,
    pub visited_locations: HashSet<u32>,
}

impl Default for PlayerLocationState {
    fn default() -> Self {
        let mut unlocked = HashSet::new();
        unlocked.insert(101); // 常青小镇 (起始)

        let mut visited = HashSet::new();
        visited.insert(101);

        PlayerLocationState {
            current_location_id: 101,
            unlocked_locations: unlocked,
            visited_locations: visited,
        }
    }
}

impl PlayerLocationState {
    /// 是否已解锁某个地点
    pub fn is_unlocked(&self, location_id: u32) -> bool {
        self.unlocked_locations.contains(&location_id)
    }

    /// 是否已访问某个地点
    pub fn is_visited(&self, location_id: u32) -> bool {
        self.visited_locations.contains(&location_id)
    }

    /// 解锁地点
    pub fn unlock_location(&mut self, location_id: u32) {
        self.unlocked_locations.insert(location_id);
    }

    /// 标记为已访问
    pub fn mark_visited(&mut self, location_id: u32) {
        self.visited_locations.insert(location_id);
    }

    /// 获取已访问地点数
    pub fn visited_count(&self) -> usize {
        self.visited_locations.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_bonus_creation() {
        let bonus = EnvironmentBonus::from_environment(EnvironmentType::Grassland);
        assert_eq!(bonus.speed_bonus, 1.1);
        assert_eq!(bonus.attack_bonus, 1.0);
    }

    #[test]
    fn test_apply_stat_bonus() {
        let bonus = EnvironmentBonus::from_environment(EnvironmentType::Grassland);
        let boosted = bonus.apply_to_stat("speed", 100);
        assert_eq!(boosted, 110);
    }

    #[test]
    fn test_location_requirement_text() {
        let req = LocationRequirement {
            required_level: 5,
            required_badges: vec![1, 2],
            required_pokemon_count: Some(3),
        };
        let text = req.get_condition_text();
        assert!(text.contains("等级"));
        assert!(text.contains("徽章"));
    }

    #[test]
    fn test_player_location_state_default() {
        let state = PlayerLocationState::default();
        assert_eq!(state.current_location_id, 101);
        assert!(state.is_unlocked(101));
        assert!(state.is_visited(101));
    }
}
