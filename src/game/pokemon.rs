use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PokemonType {
    Normal,
    Fire,
    Water,
    Grass,
    Electric,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MoveType {
    Status,
    Physical,
    Special,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Move {
    pub id: u32,
    pub name: String,
    pub move_type: MoveType,
    pub pokemon_type: PokemonType,
    pub power: u32,
    pub accuracy: u32,
    pub pp: u32,
    pub max_pp: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stat {
    pub hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub sp_attack: u32,
    pub sp_defense: u32,
    pub speed: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub pokemon_type: (PokemonType, Option<PokemonType>),
    pub level: u32,
    pub experience: u32,
    pub hp: u32,
    pub max_hp: u32,
    pub stats: Stat,
    pub moves: Vec<Move>,
    pub catch_rate: u32,
    // 捕捉信息
    pub caught_with: String,          // 捕捉球类型
    pub caught_location_id: u32,      // 捕捉地点 ID
    pub caught_date: u64,             // 捕捉时间戳
}

impl Pokemon {
    pub fn new(
        id: u32,
        name: String,
        pokemon_type: (PokemonType, Option<PokemonType>),
        base_stats: Stat,
        catch_rate: u32,
    ) -> Self {
        let (hp, max_hp) = Pokemon::calculate_hp(base_stats.hp, 1);
        Pokemon {
            id,
            name,
            pokemon_type,
            level: 1,
            experience: 0,
            hp,
            max_hp,
            stats: Stat {
                hp: base_stats.hp,
                attack: base_stats.attack,
                defense: base_stats.defense,
                sp_attack: base_stats.sp_attack,
                sp_defense: base_stats.sp_defense,
                speed: base_stats.speed,
            },
            moves: vec![],
            catch_rate,
            caught_with: "Poké Ball".to_string(),
            caught_location_id: 101, // 默认常青小镇
            caught_date: 0,
        }
    }

    /// 设置宝可梦的捕捉信息
    pub fn set_catch_info(mut self, ball_type: String, location_id: u32, timestamp: u64) -> Self {
        self.caught_with = ball_type;
        self.caught_location_id = location_id;
        self.caught_date = timestamp;
        self
    }

    pub fn calculate_hp(base_hp: u32, level: u32) -> (u32, u32) {
        let max_hp = if base_hp == 1 {
            1
        } else {
            ((2 * base_hp as u32 + 31 + 0) * level) / 100 + level + 1
        };
        (max_hp, max_hp)
    }

    pub fn calculate_stat(base_stat: u32, level: u32) -> u32 {
        ((2 * base_stat as u32 + 31 + 0) * level) / 100 + 5
    }

    pub fn add_move(&mut self, move_data: Move) -> bool {
        if self.moves.len() < 4 {
            self.moves.push(move_data);
            true
        } else {
            false
        }
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        let (new_hp, new_max_hp) = Pokemon::calculate_hp(self.stats.hp, self.level);
        self.max_hp = new_max_hp;
        if self.hp < new_hp {
            self.hp = new_hp;
        }
    }

    pub fn take_damage(&mut self, damage: u32) {
        self.hp = if damage >= self.hp { 0 } else { self.hp - damage };
    }

    pub fn heal(&mut self, amount: u32) {
        self.hp = std::cmp::min(self.hp + amount, self.max_hp);
    }

    pub fn is_fainted(&self) -> bool {
        self.hp == 0
    }

    pub fn revive(&mut self, recovery_percent: f32) {
        let recovery_amount = (self.max_hp as f32 * recovery_percent) as u32;
        self.hp = std::cmp::min(recovery_amount, self.max_hp);
    }

    pub fn get_effective_stat(&self, stat: &str, level: u32) -> u32 {
        let base_stat = match stat {
            "hp" => self.stats.hp,
            "attack" => self.stats.attack,
            "defense" => self.stats.defense,
            "sp_attack" => self.stats.sp_attack,
            "sp_defense" => self.stats.sp_defense,
            "speed" => self.stats.speed,
            _ => 0,
        };
        Pokemon::calculate_stat(base_stat, level)
    }
}

impl std::fmt::Display for Pokemon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} (等级: {}) HP: {}/{}",
            self.name, self.level, self.hp, self.max_hp
        )
    }
}
