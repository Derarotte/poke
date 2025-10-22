use serde::{Deserialize, Serialize};

pub mod trainers;
pub use trainers::{create_all_npcs, get_npc_by_id, get_all_npcs, get_npcs_by_location};

/// 对战难度等级
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,      // 新手 (等级-5)
    Normal,    // 普通 (等级 ±0)
    Hard,      // 困难 (等级+5)
    Expert,    // 专家 (等级+10)
}

impl Difficulty {
    /// 获取难度对应的等级调整值
    pub fn level_adjustment(&self) -> i32 {
        match self {
            Difficulty::Easy => -5,
            Difficulty::Normal => 0,
            Difficulty::Hard => 5,
            Difficulty::Expert => 10,
        }
    }

    /// 获取难度的中文名称
    pub fn name(&self) -> &str {
        match self {
            Difficulty::Easy => "新手",
            Difficulty::Normal => "普通",
            Difficulty::Hard => "困难",
            Difficulty::Expert => "专家",
        }
    }
}

/// NPC 训练师
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPCTrainer {
    pub id: u32,
    pub name: String,
    pub title: String,                 // 如 "馆主", "天才少女" 等
    pub pokemon_ids: Vec<u32>,         // 宝可梦 ID
    pub reward_money: u32,
    pub reward_items: Vec<String>,
    pub defeated: bool,                // 是否已被击败
    pub difficulty: Difficulty,        // 难度等级
}

impl NPCTrainer {
    /// 创建新的 NPC 训练师
    pub fn new(
        id: u32,
        name: String,
        title: String,
        pokemon_ids: Vec<u32>,
        reward_money: u32,
        difficulty: Difficulty,
    ) -> Self {
        NPCTrainer {
            id,
            name,
            title,
            pokemon_ids,
            reward_money,
            reward_items: Vec::new(),
            defeated: false,
            difficulty,
        }
    }

    /// 添加奖励物品
    pub fn add_reward_item(&mut self, item: String) {
        self.reward_items.push(item);
    }

    /// 标记为已击败
    pub fn mark_defeated(&mut self) {
        self.defeated = true;
    }

    /// 获取全名 (包括称号)
    pub fn full_name(&self) -> String {
        format!("{} {}", self.title, self.name)
    }
}

/// 对战预览信息
#[derive(Debug, Clone)]
pub struct BattlePreview {
    pub trainer_name: String,
    pub trainer_title: String,
    pub pokemon_preview: Vec<PokemonPreview>,
    pub difficulty: Difficulty,
    pub suggested_level: u32,
}

/// 宝可梦预览信息
#[derive(Debug, Clone)]
pub struct PokemonPreview {
    pub name: String,
    pub level: u32,
    pub pokemon_type: String,
}

impl BattlePreview {
    /// 创建战斗预览
    pub fn new(
        trainer_name: String,
        trainer_title: String,
        pokemon_preview: Vec<PokemonPreview>,
        difficulty: Difficulty,
        suggested_level: u32,
    ) -> Self {
        BattlePreview {
            trainer_name,
            trainer_title,
            pokemon_preview,
            difficulty,
            suggested_level,
        }
    }

    /// 获取预览的宝可梦数量
    pub fn pokemon_count(&self) -> usize {
        self.pokemon_preview.len()
    }

    /// 获取最高等级的宝可梦
    pub fn max_level(&self) -> u32 {
        self.pokemon_preview
            .iter()
            .map(|p| p.level)
            .max()
            .unwrap_or(1)
    }

    /// 格式化显示
    pub fn display_info(&self) -> String {
        format!(
            "对手: {} {} (难度: {})\n建议等级: {}\n队伍: {} 只宝可梦",
            self.trainer_title,
            self.trainer_name,
            self.difficulty.name(),
            self.suggested_level,
            self.pokemon_count()
        )
    }
}
