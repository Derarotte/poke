use serde::{Deserialize, Serialize};

pub mod generator;
pub use generator::{
    PokemonSpecies, BaseStats, PokemonStats, generate_pokemon, generate_perfect_pokemon,
    generate_pokemon_with_ivs, generate_npc_team, calculate_pokemon_stats, get_species,
};

/// 个体值 (Individual Values)，影响宝可梦的基础属性
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct IndividualValues {
    pub hp: u32,              // 0-31
    pub attack: u32,          // 0-31
    pub defense: u32,         // 0-31
    pub sp_attack: u32,       // 0-31
    pub sp_defense: u32,      // 0-31
    pub speed: u32,           // 0-31
}

impl IndividualValues {
    /// 生成随机个体值
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        IndividualValues {
            hp: rng.gen_range(0..=31),
            attack: rng.gen_range(0..=31),
            defense: rng.gen_range(0..=31),
            sp_attack: rng.gen_range(0..=31),
            sp_defense: rng.gen_range(0..=31),
            speed: rng.gen_range(0..=31),
        }
    }

    /// 生成完美个体值 (所有值都是 31)
    pub fn perfect() -> Self {
        IndividualValues {
            hp: 31,
            attack: 31,
            defense: 31,
            sp_attack: 31,
            sp_defense: 31,
            speed: 31,
        }
    }

    /// 计算总点数
    pub fn total(&self) -> u32 {
        self.hp + self.attack + self.defense + self.sp_attack + self.sp_defense + self.speed
    }
}

impl Default for IndividualValues {
    fn default() -> Self {
        IndividualValues {
            hp: 0,
            attack: 0,
            defense: 0,
            sp_attack: 0,
            sp_defense: 0,
            speed: 0,
        }
    }
}

/// 天赋系统，代表宝可梦的特殊能力
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Talent {
    // 普通天赋
    Normal,
    // 隐藏天赋
    Hidden,
}

impl Talent {
    /// 获取天赋的中文名称
    pub fn name(&self) -> &str {
        match self {
            Talent::Normal => "普通天赋",
            Talent::Hidden => "隐藏天赋",
        }
    }

    /// 获取随机天赋
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < 0.9 {
            Talent::Normal
        } else {
            Talent::Hidden
        }
    }
}

impl Default for Talent {
    fn default() -> Self {
        Talent::Normal
    }
}

/// 性格系统，影响宝可梦的属性倾向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Nature {
    // 中性
    Docile,
    Hardy,
    Serious,
    Bashful,
    Quirky,

    // 增强一个属性，降低另一个属性 (20 种)
    Lonely,      // +Attack -Defense
    Brave,       // +Attack -Speed
    Adamant,     // +Attack -SpA
    Naughty,     // +Attack -SpD

    Bold,        // +Defense -Attack
    Relaxed,     // +Defense -Speed
    Impish,      // +Defense -SpA
    Lax,         // +Defense -SpD

    Timid,       // +Speed -Attack
    Hasty,       // +Speed -Defense
    Jolly,       // +Speed -SpA
    Naive,       // +Speed -SpD

    Modest,      // +SpA -Attack
    Mild,        // +SpA -Defense
    Rash,        // +SpA -SpD
    Quiet,       // +SpA -Speed

    Calm,        // +SpD -Attack
    Gentle,      // +SpD -Defense
    Sassy,       // +SpD -Speed
    Careful,     // +SpD -SpA
}

impl Nature {
    /// 获取性格的中文名称
    pub fn name(&self) -> &str {
        match self {
            Nature::Docile => "和气",
            Nature::Hardy => "坚毅",
            Nature::Serious => "认真",
            Nature::Bashful => "害羞",
            Nature::Quirky => "淘气",

            Nature::Lonely => "孤独",
            Nature::Brave => "勇敢",
            Nature::Adamant => "固执",
            Nature::Naughty => "顽皮",

            Nature::Bold => "大胆",
            Nature::Relaxed => "悠闲",
            Nature::Impish => "淘气",
            Nature::Lax => "随意",

            Nature::Timid => "胆小",
            Nature::Hasty => "匆促",
            Nature::Jolly => "爽朗",
            Nature::Naive => "天真",

            Nature::Modest => "谦虚",
            Nature::Mild => "温和",
            Nature::Rash => "浮躁",
            Nature::Quiet => "沉着",

            Nature::Calm => "温顺",
            Nature::Gentle => "温柔",
            Nature::Sassy => "傲慢",
            Nature::Careful => "谨慎",
        }
    }

    /// 获取随机性格
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let natures = [
            Nature::Docile, Nature::Hardy, Nature::Serious, Nature::Bashful, Nature::Quirky,
            Nature::Lonely, Nature::Brave, Nature::Adamant, Nature::Naughty,
            Nature::Bold, Nature::Relaxed, Nature::Impish, Nature::Lax,
            Nature::Timid, Nature::Hasty, Nature::Jolly, Nature::Naive,
            Nature::Modest, Nature::Mild, Nature::Rash, Nature::Quiet,
            Nature::Calm, Nature::Gentle, Nature::Sassy, Nature::Careful,
        ];
        natures[rng.gen_range(0..natures.len())]
    }
}

impl Default for Nature {
    fn default() -> Self {
        Nature::Hardy
    }
}

/// 宝可梦实例，代表一个具体的宝可梦
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonInstance {
    pub species_id: u32,               // 宝可梦物种 ID
    pub level: u32,
    pub experience: u32,
    pub individual_values: IndividualValues,
    pub talent: Talent,
    pub nature: Nature,
    pub unique_id: String,             // 唯一 ID，用于防止重复
}

impl PokemonInstance {
    /// 创建新的宝可梦实例
    pub fn new(species_id: u32, level: u32) -> Self {
        use uuid::Uuid;
        PokemonInstance {
            species_id,
            level,
            experience: level as u32 * 100,
            individual_values: IndividualValues::random(),
            talent: Talent::random(),
            nature: Nature::random(),
            unique_id: Uuid::new_v4().to_string(),
        }
    }

    /// 生成完美个体值的宝可梦
    pub fn perfect(species_id: u32, level: u32) -> Self {
        use uuid::Uuid;
        PokemonInstance {
            species_id,
            level,
            experience: level as u32 * 100,
            individual_values: IndividualValues::perfect(),
            talent: Talent::Hidden,
            nature: Nature::random(),
            unique_id: Uuid::new_v4().to_string(),
        }
    }

    /// 获取属性加成倍数 (基于性格)
    pub fn get_nature_multipliers(&self) -> NatureMultipliers {
        match self.nature {
            Nature::Lonely => NatureMultipliers { attack: 1.1, defense: 0.9, ..Default::default() },
            Nature::Brave => NatureMultipliers { attack: 1.1, speed: 0.9, ..Default::default() },
            Nature::Adamant => NatureMultipliers { attack: 1.1, sp_attack: 0.9, ..Default::default() },
            Nature::Naughty => NatureMultipliers { attack: 1.1, sp_defense: 0.9, ..Default::default() },

            Nature::Bold => NatureMultipliers { defense: 1.1, attack: 0.9, ..Default::default() },
            Nature::Relaxed => NatureMultipliers { defense: 1.1, speed: 0.9, ..Default::default() },
            Nature::Impish => NatureMultipliers { defense: 1.1, sp_attack: 0.9, ..Default::default() },
            Nature::Lax => NatureMultipliers { defense: 1.1, sp_defense: 0.9, ..Default::default() },

            Nature::Timid => NatureMultipliers { speed: 1.1, attack: 0.9, ..Default::default() },
            Nature::Hasty => NatureMultipliers { speed: 1.1, defense: 0.9, ..Default::default() },
            Nature::Jolly => NatureMultipliers { speed: 1.1, sp_attack: 0.9, ..Default::default() },
            Nature::Naive => NatureMultipliers { speed: 1.1, sp_defense: 0.9, ..Default::default() },

            Nature::Modest => NatureMultipliers { sp_attack: 1.1, attack: 0.9, ..Default::default() },
            Nature::Mild => NatureMultipliers { sp_attack: 1.1, defense: 0.9, ..Default::default() },
            Nature::Rash => NatureMultipliers { sp_attack: 1.1, sp_defense: 0.9, ..Default::default() },
            Nature::Quiet => NatureMultipliers { sp_attack: 1.1, speed: 0.9, ..Default::default() },

            Nature::Calm => NatureMultipliers { sp_defense: 1.1, attack: 0.9, ..Default::default() },
            Nature::Gentle => NatureMultipliers { sp_defense: 1.1, defense: 0.9, ..Default::default() },
            Nature::Sassy => NatureMultipliers { sp_defense: 1.1, speed: 0.9, ..Default::default() },
            Nature::Careful => NatureMultipliers { sp_defense: 1.1, sp_attack: 0.9, ..Default::default() },

            _ => NatureMultipliers::default(),
        }
    }
}

/// 性格属性加成倍数
#[derive(Debug, Clone, Copy)]
pub struct NatureMultipliers {
    pub hp: f32,
    pub attack: f32,
    pub defense: f32,
    pub sp_attack: f32,
    pub sp_defense: f32,
    pub speed: f32,
}

impl Default for NatureMultipliers {
    fn default() -> Self {
        NatureMultipliers {
            hp: 1.0,
            attack: 1.0,
            defense: 1.0,
            sp_attack: 1.0,
            sp_defense: 1.0,
            speed: 1.0,
        }
    }
}
