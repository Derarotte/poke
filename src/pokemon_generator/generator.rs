use super::{PokemonInstance, IndividualValues, Talent, Nature};
use std::collections::HashMap;

/// Pokémon 物种信息
#[derive(Debug, Clone)]
pub struct PokemonSpecies {
    pub id: u32,
    pub name: String,
    pub base_stats: BaseStats,
    pub egg_group: String,
}

/// 基础属性 (不含 IV 和性格修正)
#[derive(Debug, Clone, Copy)]
pub struct BaseStats {
    pub hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub sp_attack: u32,
    pub sp_defense: u32,
    pub speed: u32,
}

impl BaseStats {
    /// 计算最终属性值 (包含 IV 和性格)
    pub fn calculate_stat(
        &self,
        base_value: u32,
        iv_value: u32,
        level: u32,
        multiplier: f32,
    ) -> u32 {
        let base = base_value as f32;
        let iv = iv_value as f32;
        let lvl = level as f32;

        // 标准公式: ((2 * Base + IV) * Level / 100 + 5) * Nature_Multiplier
        let raw_stat = ((2.0 * base + iv) * lvl / 100.0 + 5.0) * multiplier;
        raw_stat.max(1.0) as u32
    }

    /// 计算 HP (特殊公式)
    pub fn calculate_hp(&self, iv_hp: u32, level: u32) -> u32 {
        let base = self.hp as f32;
        let iv = iv_hp as f32;
        let lvl = level as f32;

        // HP 公式: (2 * Base + IV) * Level / 100 + Level + 5
        ((2.0 * base + iv) * lvl / 100.0 + lvl + 5.0) as u32
    }
}

/// 创建 Pokémon 物种数据库
pub fn get_species(species_id: u32) -> Option<PokemonSpecies> {
    let mut species_map = create_species_map();
    species_map.remove(&species_id)
}

/// 创建物种数据库
fn create_species_map() -> HashMap<u32, PokemonSpecies> {
    let mut map = HashMap::new();

    // 初始宝可梦
    map.insert(
        25,
        PokemonSpecies {
            id: 25,
            name: "皮卡丘".to_string(),
            base_stats: BaseStats {
                hp: 35,
                attack: 55,
                defense: 40,
                sp_attack: 50,
                sp_defense: 50,
                speed: 90,
            },
            egg_group: "陆地".to_string(),
        },
    );

    // 妙蛙家族
    map.insert(
        1,
        PokemonSpecies {
            id: 1,
            name: "妙蛙种子".to_string(),
            base_stats: BaseStats {
                hp: 45,
                attack: 49,
                defense: 49,
                sp_attack: 65,
                sp_defense: 65,
                speed: 45,
            },
            egg_group: "植物".to_string(),
        },
    );

    // 小火龙家族
    map.insert(
        4,
        PokemonSpecies {
            id: 4,
            name: "小火龙".to_string(),
            base_stats: BaseStats {
                hp: 39,
                attack: 52,
                defense: 43,
                sp_attack: 60,
                sp_defense: 50,
                speed: 65,
            },
            egg_group: "怪兽".to_string(),
        },
    );

    // 杰尼龟家族
    map.insert(
        7,
        PokemonSpecies {
            id: 7,
            name: "杰尼龟".to_string(),
            base_stats: BaseStats {
                hp: 44,
                attack: 48,
                defense: 65,
                sp_attack: 50,
                sp_defense: 64,
                speed: 43,
            },
            egg_group: "水1".to_string(),
        },
    );

    // 其他常见宝可梦
    map.insert(
        27,
        PokemonSpecies {
            id: 27,
            name: "小拳石".to_string(),
            base_stats: BaseStats {
                hp: 40,
                attack: 80,
                defense: 100,
                sp_attack: 30,
                sp_defense: 30,
                speed: 20,
            },
            egg_group: "矿物".to_string(),
        },
    );

    map.insert(
        54,
        PokemonSpecies {
            id: 54,
            name: "可达鸭".to_string(),
            base_stats: BaseStats {
                hp: 50,
                attack: 52,
                defense: 48,
                sp_attack: 66,
                sp_defense: 56,
                speed: 55,
            },
            egg_group: "水1".to_string(),
        },
    );

    map.insert(
        104,
        PokemonSpecies {
            id: 104,
            name: "卡拉卡拉".to_string(),
            base_stats: BaseStats {
                hp: 50,
                attack: 75,
                defense: 40,
                sp_attack: 40,
                sp_defense: 50,
                speed: 40,
            },
            egg_group: "陆地".to_string(),
        },
    );

    map.insert(
        147,
        PokemonSpecies {
            id: 147,
            name: "迷你龙".to_string(),
            base_stats: BaseStats {
                hp: 41,
                attack: 64,
                defense: 45,
                sp_attack: 72,
                sp_defense: 55,
                speed: 76,
            },
            egg_group: "龙".to_string(),
        },
    );

    map.insert(
        149,
        PokemonSpecies {
            id: 149,
            name: "快龙".to_string(),
            base_stats: BaseStats {
                hp: 91,
                attack: 134,
                defense: 95,
                sp_attack: 100,
                sp_defense: 100,
                speed: 80,
            },
            egg_group: "龙".to_string(),
        },
    );

    map
}

/// 生成具有完整属性的 Pokémon 实例
pub fn generate_pokemon(species_id: u32, level: u32) -> Result<PokemonInstance, String> {
    let _species =
        get_species(species_id).ok_or(format!("物种 {} 不存在", species_id))?;

    let pokemon = PokemonInstance::new(species_id, level);
    Ok(pokemon)
}

/// 生成完美个体值的 Pokémon
pub fn generate_perfect_pokemon(species_id: u32, level: u32) -> Result<PokemonInstance, String> {
    let species =
        get_species(species_id).ok_or(format!("物种 {} 不存在", species_id))?;

    let pokemon = PokemonInstance::perfect(species_id, level);
    Ok(pokemon)
}

/// 生成指定个体值的 Pokémon
pub fn generate_pokemon_with_ivs(
    species_id: u32,
    level: u32,
    ivs: IndividualValues,
) -> Result<PokemonInstance, String> {
    use uuid::Uuid;

    let species =
        get_species(species_id).ok_or(format!("物种 {} 不存在", species_id))?;

    Ok(PokemonInstance {
        species_id,
        level,
        experience: level as u32 * 100,
        individual_values: ivs,
        talent: Talent::random(),
        nature: Nature::random(),
        unique_id: Uuid::new_v4().to_string(),
    })
}

/// 生成 NPC 队伍 (固定个体值，确保质量)
pub fn generate_npc_team(
    pokemon_ids: &[u32],
    base_level: u32,
    difficulty_adjustment: i32,
) -> Result<Vec<PokemonInstance>, String> {
    let mut team = Vec::new();

    for &pokemon_id in pokemon_ids {
        let level = std::cmp::max(1, (base_level as i32 + difficulty_adjustment) as u32);

        // NPC 宝可梦有更好的个体值
        let ivs = IndividualValues {
            hp: 25,
            attack: 25,
            defense: 25,
            sp_attack: 25,
            sp_defense: 25,
            speed: 25,
        };

        let pokemon = generate_pokemon_with_ivs(pokemon_id, level, ivs)?;
        team.push(pokemon);
    }

    Ok(team)
}

/// 计算宝可梦的完整属性
pub fn calculate_pokemon_stats(
    species: &PokemonSpecies,
    pokemon: &PokemonInstance,
) -> PokemonStats {
    let nature_mults = pokemon.get_nature_multipliers();
    let level = pokemon.level;
    let ivs = &pokemon.individual_values;

    let hp = species.base_stats.calculate_hp(ivs.hp, level);
    let attack = species
        .base_stats
        .calculate_stat(species.base_stats.attack, ivs.attack, level, nature_mults.attack);
    let defense = species
        .base_stats
        .calculate_stat(species.base_stats.defense, ivs.defense, level, nature_mults.defense);
    let sp_attack = species.base_stats.calculate_stat(
        species.base_stats.sp_attack,
        ivs.sp_attack,
        level,
        nature_mults.sp_attack,
    );
    let sp_defense = species.base_stats.calculate_stat(
        species.base_stats.sp_defense,
        ivs.sp_defense,
        level,
        nature_mults.sp_defense,
    );
    let speed = species
        .base_stats
        .calculate_stat(species.base_stats.speed, ivs.speed, level, nature_mults.speed);

    PokemonStats {
        hp,
        attack,
        defense,
        sp_attack,
        sp_defense,
        speed,
    }
}

/// 宝可梦完整属性
#[derive(Debug, Clone, Copy)]
pub struct PokemonStats {
    pub hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub sp_attack: u32,
    pub sp_defense: u32,
    pub speed: u32,
}

impl PokemonStats {
    /// 显示属性信息
    pub fn display(&self) {
        println!("属性信息:");
        println!("  HP:      {}", self.hp);
        println!("  攻击:    {}", self.attack);
        println!("  防守:    {}", self.defense);
        println!("  特攻:    {}", self.sp_attack);
        println!("  特防:    {}", self.sp_defense);
        println!("  速度:    {}", self.speed);
    }

    /// 应用环境加成
    pub fn apply_environment_bonus(&self, bonus: &crate::game::EnvironmentBonus) -> PokemonStats {
        PokemonStats {
            hp: bonus.apply_to_stat("hp", self.hp),
            attack: bonus.apply_to_stat("attack", self.attack),
            defense: bonus.apply_to_stat("defense", self.defense),
            sp_attack: bonus.apply_to_stat("sp_attack", self.sp_attack),
            sp_defense: bonus.apply_to_stat("sp_defense", self.sp_defense),
            speed: bonus.apply_to_stat("speed", self.speed),
        }
    }
}
