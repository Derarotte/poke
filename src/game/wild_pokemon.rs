use crate::game::EnvironmentBonus;
use crate::game::PokemonType;
use crate::pokemon_generator::generator::{get_species, calculate_pokemon_stats, PokemonStats};
use crate::pokemon_generator::PokemonInstance;
use rand::Rng;

/// 野生宝可梦遭遇信息 (包含环境加成)
#[derive(Debug, Clone)]
pub struct WildPokemonPreview {
    pub pokemon_id: u32,
    pub pokemon_name: String,
    pub level: u32,
    pub pokemon_type: (PokemonType, Option<PokemonType>),
    /// 原始属性 (不含环境加成)
    pub base_stats: PokemonStats,
    /// 当前环境加成后的属性
    pub boosted_stats: PokemonStats,
    /// 环境类型 (用于显示)
    pub environment: String,
    /// 环境加成百分比 (如 "攻击 +10%")
    pub bonus_description: String,
}

impl WildPokemonPreview {
    /// 显示预览信息
    pub fn display(&self) {
        println!("\n╭─────────────────────────────╮");
        println!("│     野生宝可梦出现!        │");
        println!("├─────────────────────────────┤");
        println!("│ 名字: {}", format!("{:<18}", self.pokemon_name));
        println!("│ 等级: {}", format!("{:<18}", self.level));

        let type_str = match self.pokemon_type {
            (t1, Some(t2)) => format!("{:?}/{:?}", t1, t2),
            (t1, None) => format!("{:?}", t1),
        };
        println!("│ 类型: {}", format!("{:<18}", type_str));
        println!("│ 当前环境: {}", format!("{:<14}", self.environment));

        if !self.bonus_description.is_empty() {
            println!("│ 属性加成: {}", format!("{:<14}", self.bonus_description));
        }

        println!("│");
        println!("│ 属性预览:");
        self.display_stats();

        println!("├─────────────────────────────┤");
        println!("│ 1. 捕捉  2. 战斗  3. 逃跑     │");
        println!("╰─────────────────────────────╯\n");
    }

    /// 显示属性对比 (原始 vs 加成)
    fn display_stats(&self) {
        let stats = [
            ("HP", self.base_stats.hp, self.boosted_stats.hp),
            ("攻击", self.base_stats.attack, self.boosted_stats.attack),
            ("防守", self.base_stats.defense, self.boosted_stats.defense),
            ("特攻", self.base_stats.sp_attack, self.boosted_stats.sp_attack),
            ("特防", self.base_stats.sp_defense, self.boosted_stats.sp_defense),
            ("速度", self.base_stats.speed, self.boosted_stats.speed),
        ];

        for (name, base, boosted) in &stats {
            if base == boosted {
                println!("│   {}: {}", format!("{:<4}", name), base);
            } else {
                println!("│   {}: {} → {}", format!("{:<4}", name), base, boosted);
            }
        }
    }
}

/// 野生宝可梦遭遇系统
pub struct WildPokemonEncounter;

impl WildPokemonEncounter {
    /// 从地点的宝可梦池中随机生成一只野生宝可梦
    pub fn generate_wild_pokemon(
        wild_pokemon_pool: &[crate::game::WildPokemonSpawn],
    ) -> Result<PokemonInstance, String> {
        if wild_pokemon_pool.is_empty() {
            return Err("地点没有野生宝可梦池".to_string());
        }

        // 使用加权随机选择
        let selected = Self::weighted_random_selection(wild_pokemon_pool)?;

        // 随机化等级
        let level = Self::random_level(selected.level_min, selected.level_max);

        // 生成宝可梦
        crate::pokemon_generator::generator::generate_pokemon(selected.pokemon_id, level)
    }

    /// 加权随机选择 (根据 spawn_rate)
    fn weighted_random_selection(
        pool: &[crate::game::WildPokemonSpawn],
    ) -> Result<crate::game::WildPokemonSpawn, String> {
        // 计算总权重
        let total_weight: f64 = pool.iter().map(|p| p.spawn_rate as f64).sum();

        if total_weight <= 0.0 {
            return Err("池的总权重为 0".to_string());
        }

        // 生成 0 到 total_weight 之间的随机数
        let mut rng = rand::thread_rng();
        let mut random_value: f64 = rng.gen::<f64>() * total_weight;

        // 根据权重选择
        for spawn in pool {
            random_value -= spawn.spawn_rate as f64;
            if random_value <= 0.0 {
                return Ok(spawn.clone());
            }
        }

        // 防御性编程: 如果没有选中，返回最后一个
        Ok(pool[pool.len() - 1].clone())
    }

    /// 随机化等级
    fn random_level(min: u32, max: u32) -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(min..=max)
    }

    /// 生成带有环境加成的预览
    pub fn generate_preview(
        wild_pokemon: &PokemonInstance,
        environment_bonus: &EnvironmentBonus,
        environment_name: &str,
    ) -> Result<WildPokemonPreview, String> {
        // 获取物种信息
        let species = get_species(wild_pokemon.species_id)
            .ok_or(format!("物种 {} 不存在", wild_pokemon.species_id))?;

        // 计算基础属性
        let base_stats = calculate_pokemon_stats(&species, wild_pokemon);

        // 应用环境加成
        let boosted_stats = base_stats.apply_environment_bonus(environment_bonus);

        // 生成加成描述
        let bonus_description = Self::generate_bonus_description(&base_stats, &boosted_stats);

        Ok(WildPokemonPreview {
            pokemon_id: wild_pokemon.species_id,
            pokemon_name: species.name,
            level: wild_pokemon.level,
            pokemon_type: (PokemonType::Normal, None), // TODO: 从物种获取类型
            base_stats,
            boosted_stats,
            environment: environment_name.to_string(),
            bonus_description,
        })
    }

    /// 生成加成描述文本
    fn generate_bonus_description(base: &PokemonStats, boosted: &PokemonStats) -> String {
        let mut bonuses = Vec::new();

        if base.hp != boosted.hp {
            let percent = ((boosted.hp as f32 / base.hp as f32 - 1.0) * 100.0) as i32;
            bonuses.push(format!("HP {}{:+}%", if percent > 0 { "+" } else { "" }, percent));
        }
        if base.attack != boosted.attack {
            let percent = ((boosted.attack as f32 / base.attack as f32 - 1.0) * 100.0) as i32;
            bonuses.push(format!("攻击 {}{:+}%", if percent > 0 { "+" } else { "" }, percent));
        }
        if base.defense != boosted.defense {
            let percent = ((boosted.defense as f32 / base.defense as f32 - 1.0) * 100.0) as i32;
            bonuses.push(format!("防守 {}{:+}%", if percent > 0 { "+" } else { "" }, percent));
        }
        if base.sp_attack != boosted.sp_attack {
            let percent = ((boosted.sp_attack as f32 / base.sp_attack as f32 - 1.0) * 100.0) as i32;
            bonuses.push(format!("特攻 {}{:+}%", if percent > 0 { "+" } else { "" }, percent));
        }
        if base.sp_defense != boosted.sp_defense {
            let percent = ((boosted.sp_defense as f32 / base.sp_defense as f32 - 1.0) * 100.0) as i32;
            bonuses.push(format!("特防 {}{:+}%", if percent > 0 { "+" } else { "" }, percent));
        }
        if base.speed != boosted.speed {
            let percent = ((boosted.speed as f32 / base.speed as f32 - 1.0) * 100.0) as i32;
            bonuses.push(format!("速度 {}{:+}%", if percent > 0 { "+" } else { "" }, percent));
        }

        bonuses.join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::WildPokemonSpawn;

    #[test]
    fn test_weighted_selection_distribution() {
        let pool = vec![
            WildPokemonSpawn {
                pokemon_id: 1,
                spawn_rate: 50.0,
                level_min: 1,
                level_max: 5,
            },
            WildPokemonSpawn {
                pokemon_id: 2,
                spawn_rate: 30.0,
                level_min: 1,
                level_max: 5,
            },
            WildPokemonSpawn {
                pokemon_id: 3,
                spawn_rate: 20.0,
                level_min: 1,
                level_max: 5,
            },
        ];

        let mut counts = [0u32; 3];
        const TRIALS: u32 = 10000;

        for _ in 0..TRIALS {
            let selected = WildPokemonEncounter::weighted_random_selection(&pool).unwrap();
            if selected.pokemon_id == 1 {
                counts[0] += 1;
            } else if selected.pokemon_id == 2 {
                counts[1] += 1;
            } else if selected.pokemon_id == 3 {
                counts[2] += 1;
            }
        }

        // 允许 5% 的误差范围
        let expected_1 = (TRIALS as f32 * 0.5) as u32;
        let expected_2 = (TRIALS as f32 * 0.3) as u32;
        let expected_3 = (TRIALS as f32 * 0.2) as u32;

        let margin = TRIALS / 20; // 5% 误差

        assert!(
            (counts[0] as i32 - expected_1 as i32).abs() < margin as i32,
            "Pokemon 1: expected ~{}, got {}",
            expected_1,
            counts[0]
        );
        assert!(
            (counts[1] as i32 - expected_2 as i32).abs() < margin as i32,
            "Pokemon 2: expected ~{}, got {}",
            expected_2,
            counts[1]
        );
        assert!(
            (counts[2] as i32 - expected_3 as i32).abs() < margin as i32,
            "Pokemon 3: expected ~{}, got {}",
            expected_3,
            counts[2]
        );
    }

    #[test]
    fn test_level_range() {
        let pool = vec![WildPokemonSpawn {
            pokemon_id: 1,
            spawn_rate: 100.0,
            level_min: 5,
            level_max: 10,
        }];

        const TRIALS: u32 = 1000;
        let mut min_level = u32::MAX;
        let mut max_level = u32::MIN;

        for _ in 0..TRIALS {
            let level = WildPokemonEncounter::random_level(5, 10);
            min_level = min_level.min(level);
            max_level = max_level.max(level);
        }

        assert!(
            min_level >= 5 && max_level <= 10,
            "Level out of range: {} to {}",
            min_level,
            max_level
        );
    }

    #[test]
    fn test_empty_pool_error() {
        let pool: Vec<WildPokemonSpawn> = vec![];
        let result = WildPokemonEncounter::weighted_random_selection(&pool);
        assert!(result.is_err());
    }
}
