use super::{Pokemon, PokemonType, MoveType};
use rand::Rng;

pub struct Battle {
    pub player_pokemon: Pokemon,
    pub opponent_pokemon: Pokemon,
    pub turn: u32,
}

impl Battle {
    pub fn new(player_pokemon: Pokemon, opponent_pokemon: Pokemon) -> Self {
        Battle {
            player_pokemon,
            opponent_pokemon,
            turn: 0,
        }
    }

    pub fn calculate_damage(
        attacker: &Pokemon,
        defender: &Pokemon,
        move_data: &super::Move,
    ) -> u32 {
        let mut rng = rand::thread_rng();

        if move_data.move_type == MoveType::Status {
            return 0;
        }

        // 获取攻击方的攻击或特攻
        let attack = if move_data.move_type == MoveType::Physical {
            attacker.get_effective_stat("attack", attacker.level)
        } else {
            attacker.get_effective_stat("sp_attack", attacker.level)
        };

        // 获取防守方的防御或特防
        let defense = if move_data.move_type == MoveType::Physical {
            defender.get_effective_stat("defense", defender.level)
        } else {
            defender.get_effective_stat("sp_defense", defender.level)
        };

        let base_damage = (((2.0 * attacker.level as f64 / 5.0 + 2.0)
            * move_data.power as f64
            * attack as f64
            / defense as f64)
            / 50.0
            + 2.0) as u32;

        // 属性相克系数
        let effectiveness = Battle::get_type_effectiveness(move_data.pokemon_type, defender.pokemon_type);

        // 随机值 (85-100%)
        let random_factor: f64 = rng.gen_range(0.85..=1.00);

        let final_damage = (base_damage as f64 * effectiveness * random_factor) as u32;
        std::cmp::max(1, final_damage)
    }

    pub fn get_type_effectiveness(
        attack_type: PokemonType,
        defender_type: (PokemonType, Option<PokemonType>),
    ) -> f64 {
        let mut effectiveness = 1.0;

        // 检查第一类型
        effectiveness *= Battle::get_single_type_effectiveness(attack_type, defender_type.0);

        // 检查第二类型
        if let Some(second_type) = defender_type.1 {
            effectiveness *= Battle::get_single_type_effectiveness(attack_type, second_type);
        }

        effectiveness
    }

    fn get_single_type_effectiveness(attack_type: PokemonType, defend_type: PokemonType) -> f64 {
        use PokemonType::*;

        match (attack_type, defend_type) {
            // 火属性克制
            (Fire, Grass) | (Fire, Ice) | (Fire, Bug) | (Fire, Steel) => 2.0,
            // 水属性克制
            (Water, Fire) | (Water, Ground) | (Water, Rock) => 2.0,
            // 草属性克制
            (Grass, Water) | (Grass, Ground) | (Grass, Rock) => 2.0,
            // 电属性克制
            (Electric, Water) | (Electric, Flying) => 2.0,
            // 冰属性克制
            (Ice, Grass) | (Ice, Flying) | (Ice, Ground) | (Ice, Dragon) => 2.0,
            // 格斗属性克制
            (Fighting, Normal) | (Fighting, Ice) | (Fighting, Rock) | (Fighting, Dark) | (Fighting, Steel) => 2.0,
            // 毒属性克制
            (Poison, Grass) | (Poison, Fairy) => 2.0,
            // 地面属性克制
            (Ground, Fire) | (Ground, Electric) | (Ground, Poison) | (Ground, Rock) | (Ground, Steel) => 2.0,
            // 飞行属性克制
            (Flying, Grass) | (Flying, Fighting) | (Flying, Bug) => 2.0,
            // 超能力属性克制
            (Psychic, Fighting) | (Psychic, Poison) => 2.0,
            // 虫属性克制
            (Bug, Grass) | (Bug, Psychic) | (Bug, Dark) => 2.0,
            // 岩石属性克制
            (Rock, Fire) | (Rock, Ice) | (Rock, Flying) | (Rock, Bug) => 2.0,
            // 幽灵属性克制
            (Ghost, Ghost) | (Ghost, Psychic) => 2.0,
            // 龙属性克制
            (Dragon, Dragon) => 2.0,
            // 暗黑属性克制
            (Dark, Ghost) | (Dark, Psychic) => 2.0,
            // 钢属性克制
            (Steel, Ice) | (Steel, Rock) | (Steel, Fairy) => 2.0,

            // 属性不克制 (伤害减半)
            (Fire, Fire) | (Fire, Water) | (Fire, Rock) => 0.5,
            (Water, Water) | (Water, Grass) => 0.5,
            (Grass, Fire) | (Grass, Poison) | (Grass, Flying) => 0.5,
            (Electric, Grass) | (Electric, Electric) | (Electric, Dragon) => 0.5,
            (Ice, Fire) | (Ice, Water) | (Ice, Ice) => 0.5,
            (Fighting, Flying) | (Fighting, Poison) | (Fighting, Psychic) => 0.5,
            (Poison, Grass) | (Poison, Poison) | (Poison, Ground) | (Poison, Rock) => 0.5,
            (Ground, Grass) | (Ground, Bug) => 0.5,
            (Flying, Electric) | (Flying, Rock) | (Flying, Steel) => 0.5,
            (Psychic, Steel) | (Psychic, Psychic) | (Psychic, Dark) => 0.5,
            (Bug, Fire) | (Bug, Fighting) | (Bug, Flying) | (Bug, Poison) | (Bug, Ghost) | (Bug, Steel) => 0.5,
            (Rock, Fighting) | (Rock, Ground) | (Rock, Steel) => 0.5,
            (Ghost, Dark) => 0.5,
            (Dragon, Steel) => 0.5,
            (Dark, Dark) | (Dark, Steel) => 0.5,
            (Steel, Water) | (Steel, Electric) | (Steel, Steel) => 0.5,

            // 默认正常伤害
            _ => 1.0,
        }
    }

    pub fn check_hit(accuracy: u32) -> bool {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..=100) <= accuracy
    }

    pub fn award_experience(&mut self, base_exp: u32) {
        let experience_gained = (base_exp * self.opponent_pokemon.level) / 7;
        self.player_pokemon.experience += experience_gained;

        // 简单的升级系统：每100经验升1级
        let levels_gained = experience_gained / 100;
        for _ in 0..levels_gained {
            self.player_pokemon.level_up();
        }
    }
}
