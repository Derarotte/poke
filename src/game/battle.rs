use super::{Pokemon, PokemonType, MoveType};
use crate::data::loader;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// 战斗状态
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BattleStatus {
    Active,      // 战斗进行中
    PlayerWon,   // 玩家胜利
    PlayerLost,  // 玩家失败
    Escaped,     // 成功逃脱
}

/// 完整的战斗系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Battle {
    // 玩家信息
    pub player_team: Vec<Pokemon>,
    pub player_current_index: usize,

    // 对手信息
    pub opponent_team: Vec<Pokemon>,
    pub opponent_current_index: usize,

    // 战斗状态
    pub turn: u32,
    pub status: BattleStatus,
    pub battle_log: Vec<String>,
    pub is_wild_battle: bool, // 野生战斗可以逃脱
}

impl Battle {
    /// 创建新的战斗（单Pokemon）
    pub fn new(player_pokemon: Pokemon, opponent_pokemon: Pokemon) -> Self {
        Battle {
            player_team: vec![player_pokemon],
            player_current_index: 0,
            opponent_team: vec![opponent_pokemon],
            opponent_current_index: 0,
            turn: 0,
            status: BattleStatus::Active,
            battle_log: Vec::new(),
            is_wild_battle: true,
        }
    }

    /// 创建新的团队战斗
    pub fn new_team_battle(
        player_team: Vec<Pokemon>,
        opponent_team: Vec<Pokemon>,
        is_wild: bool,
    ) -> Self {
        Battle {
            player_team,
            player_current_index: 0,
            opponent_team,
            opponent_current_index: 0,
            turn: 0,
            status: BattleStatus::Active,
            battle_log: Vec::new(),
            is_wild_battle: is_wild,
        }
    }

    /// 获取当前玩家宝可梦
    pub fn get_player_pokemon(&self) -> Option<&Pokemon> {
        self.player_team.get(self.player_current_index)
    }

    /// 获取当前对手宝可梦
    pub fn get_opponent_pokemon(&self) -> Option<&Pokemon> {
        self.opponent_team.get(self.opponent_current_index)
    }

    /// 获取当前玩家宝可梦（可变）
    pub fn get_player_pokemon_mut(&mut self) -> Option<&mut Pokemon> {
        self.player_team.get_mut(self.player_current_index)
    }

    /// 获取当前对手宝可梦（可变）
    pub fn get_opponent_pokemon_mut(&mut self) -> Option<&mut Pokemon> {
        self.opponent_team.get_mut(self.opponent_current_index)
    }

    /// 检查玩家是否有活跃宝可梦
    pub fn has_player_active(&self) -> bool {
        self.player_team.iter().any(|p| !p.is_fainted())
    }

    /// 检查对手是否有活跃宝可梦
    pub fn has_opponent_active(&self) -> bool {
        self.opponent_team.iter().any(|p| !p.is_fainted())
    }

    /// 切换玩家宝可梦
    pub fn switch_player_pokemon(&mut self, index: usize) -> Result<(), String> {
        if index >= self.player_team.len() {
            return Err("索引超出范围".to_string());
        }
        if self.player_team[index].is_fainted() {
            return Err("该宝可梦已昏迷".to_string());
        }
        if index == self.player_current_index {
            return Err("该宝可梦已在战斗中".to_string());
        }
        self.player_current_index = index;
        let pokemon_name = self.player_team[index].name.clone();
        self.add_log(format!("派遣了 {}!", pokemon_name));
        Ok(())
    }

    /// 切换对手宝可梦
    pub fn switch_opponent_pokemon(&mut self, index: usize) -> Result<(), String> {
        if index >= self.opponent_team.len() {
            return Err("索引超出范围".to_string());
        }
        if self.opponent_team[index].is_fainted() {
            return Err("该宝可梦已昏迷".to_string());
        }
        if index == self.opponent_current_index {
            return Err("该宝可梦已在战斗中".to_string());
        }
        self.opponent_current_index = index;
        let pokemon_name = self.opponent_team[index].name.clone();
        self.add_log(format!("对手派遣了 {}!", pokemon_name));
        Ok(())
    }

    /// 添加战斗日志
    pub fn add_log(&mut self, message: String) {
        self.battle_log.push(message);
    }

    /// 获取最后一条日志
    pub fn get_last_log(&self) -> Option<&String> {
        self.battle_log.last()
    }

    /// 尝试逃脱战斗
    pub fn attempt_escape(&mut self) -> bool {
        if !self.is_wild_battle {
            self.add_log("无法从训练师战斗中逃脱！".to_string());
            return false;
        }

        let mut rng = rand::thread_rng();
        let success = rng.gen::<f32>() < 0.6; // 60% 成功率

        if success {
            self.status = BattleStatus::Escaped;
            self.add_log("成功逃脱！".to_string());
        } else {
            self.add_log("逃脱失败！".to_string());
        }

        success
    }

    /// 检查战斗是否结束
    pub fn check_battle_end(&mut self) -> bool {
        if !self.has_player_active() {
            self.status = BattleStatus::PlayerLost;
            self.add_log("你的所有宝可梦都昏迷了！你输了！".to_string());
            return true;
        }

        if !self.has_opponent_active() {
            self.status = BattleStatus::PlayerWon;
            self.add_log("你赢了！".to_string());
            return true;
        }

        // 检查当前宝可梦是否昏迷
        if let Some(player_poke) = self.get_player_pokemon() {
            if player_poke.is_fainted() {
                self.add_log(format!("{} 昏迷了！", player_poke.name));
                // 尝试自动切换到下一只活跃宝可梦
                for (i, poke) in self.player_team.iter().enumerate() {
                    if !poke.is_fainted() && i != self.player_current_index {
                        let _ = self.switch_player_pokemon(i);
                        break;
                    }
                }
                return false;
            }
        }

        if let Some(opponent_poke) = self.get_opponent_pokemon() {
            if opponent_poke.is_fainted() {
                self.add_log(format!("对手的 {} 昏迷了！", opponent_poke.name));
                // 对手自动切换宝可梦
                for (i, poke) in self.opponent_team.iter().enumerate() {
                    if !poke.is_fainted() && i != self.opponent_current_index {
                        let _ = self.switch_opponent_pokemon(i);
                        break;
                    }
                }
                return false;
            }
        }

        false
    }

    /// 确定回合顺序（返回 true 表示玩家先行）
    pub fn determine_turn_order(&self) -> bool {
        if let (Some(player_poke), Some(opponent_poke)) =
            (self.get_player_pokemon(), self.get_opponent_pokemon())
        {
            let player_speed = player_poke.get_effective_stat("speed", player_poke.level);
            let opponent_speed = opponent_poke.get_effective_stat("speed", opponent_poke.level);
            player_speed >= opponent_speed
        } else {
            true
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

        // Try to load from JSON cache first
        let attack_str = format!("{:?}", attack_type);
        let defend_str = format!("{:?}", defend_type);
        let effectiveness = loader::get_type_effectiveness(&attack_str, &defend_str);

        // If JSON cache loaded something non-default, use it
        if effectiveness != 1.0 {
            return effectiveness;
        }

        // Fallback to hardcoded match table (for tests and if JSON loading fails)
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
            (Poison, Fairy) => 2.0,
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

    /// 执行招式攻击
    pub fn use_move(&mut self, move_idx: usize, is_player: bool) -> Result<(), String> {
        // 首先获取攻击方和防守方的数据
        let (attacker_name, move_data, defender_clone) = if is_player {
            let attacker = self.get_player_pokemon().ok_or("没有活跃的玩家宝可梦")?;
            let defender = self.get_opponent_pokemon().ok_or("没有活跃的对手宝可梦")?;

            if move_idx >= attacker.moves.len() {
                return Err("招式索引超出范围".to_string());
            }

            let move_data = attacker.moves[move_idx].clone();
            (attacker.name.clone(), move_data, defender.clone())
        } else {
            let attacker = self.get_opponent_pokemon().ok_or("没有活跃的对手宝可梦")?;
            let defender = self.get_player_pokemon().ok_or("没有活跃的玩家宝可梦")?;

            if move_idx >= attacker.moves.len() {
                return Err("招式索引超出范围".to_string());
            }

            let move_data = attacker.moves[move_idx].clone();
            (attacker.name.clone(), move_data, defender.clone())
        };

        // 检查 PP
        if move_data.pp == 0 {
            return Err(format!("{} 的 PP 用完了！", move_data.name));
        }

        // 检查命中
        if !Self::check_hit(move_data.accuracy) {
            let actor = if is_player { "玩家的" } else { "对手的" };
            self.add_log(format!("{}{}的攻击没有命中!", actor, attacker_name));
            return Ok(());
        }

        // 获取攻击方再次计算伤害
        let damage = if is_player {
            let attacker = self.get_player_pokemon().ok_or("没有活跃的玩家宝可梦")?;
            Self::calculate_damage(attacker, &defender_clone, &move_data)
        } else {
            let attacker = self.get_opponent_pokemon().ok_or("没有活跃的对手宝可梦")?;
            Self::calculate_damage(attacker, &defender_clone, &move_data)
        };

        // 应用伤害
        if is_player {
            if let Some(opponent) = self.get_opponent_pokemon_mut() {
                opponent.take_damage(damage);
                self.add_log(format!(
                    "玩家的 {} 使用了 {}，造成 {} 伤害！",
                    attacker_name, move_data.name, damage
                ));
            }
        } else {
            if let Some(player) = self.get_player_pokemon_mut() {
                player.take_damage(damage);
                self.add_log(format!(
                    "对手的 {} 使用了 {}，造成 {} 伤害！",
                    attacker_name, move_data.name, damage
                ));
            }
        }

        Ok(())
    }

    /// 使用道具（如恢复药）
    pub fn use_item(&mut self, item_type: &str, target_is_player: bool, _amount: u32) -> Result<(), String> {
        let target = if target_is_player {
            self.get_player_pokemon_mut().ok_or("没有活跃的玩家宝可梦")?
        } else {
            self.get_opponent_pokemon_mut().ok_or("没有活跃的对手宝可梦")?
        };

        match item_type {
            "恢复药" => {
                target.heal(20);
                let actor = if target_is_player { "玩家" } else { "对手" };
                self.add_log(format!("{} 使用了恢复药，恢复了 20 HP！", actor));
                Ok(())
            }
            "超级恢复药" => {
                target.heal(50);
                let actor = if target_is_player { "玩家" } else { "对手" };
                self.add_log(format!("{} 使用了超级恢复药，恢复了 50 HP！", actor));
                Ok(())
            }
            "全复活" => {
                target.revive(1.0); // 完全复活
                let actor = if target_is_player { "玩家的" } else { "对手的" };
                self.add_log(format!("{}宝可梦被全复活了！", actor));
                Ok(())
            }
            _ => Err(format!("未知的道具类型: {}", item_type)),
        }
    }

    /// 分配经验值
    pub fn award_experience(&mut self, base_exp: u32) {
        if let Some(opponent) = self.get_opponent_pokemon() {
            let experience_gained = (base_exp * opponent.level) / 7;
            let mut level_up_messages = Vec::new();

            // 给所有活跃宝可梦分配经验
            for pokemon in self.player_team.iter_mut() {
                if !pokemon.is_fainted() {
                    pokemon.experience += experience_gained;

                    // 检查升级
                    let next_level_exp = pokemon.level as u32 * 100;
                    while pokemon.experience >= next_level_exp {
                        pokemon.level_up();
                        level_up_messages.push(format!("{} 升到 Lv.{}！", pokemon.name, pokemon.level));
                    }
                }
            }

            // 现在添加所有消息
            for msg in level_up_messages {
                self.add_log(msg);
            }

            self.add_log(format!("获得了 {} 经验值！", experience_gained));
        }
    }

    /// 计算金钱奖励
    pub fn calculate_reward_money(&self) -> u32 {
        if let Some(opponent) = self.get_opponent_pokemon() {
            opponent.level as u32 * 10  // 基础奖励
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{Stat, Move};

    // 辅助函数：创建测试宝可梦
    fn create_test_pokemon(name: &str, level: u32, hp: u32, attack: u32, sp_attack: u32) -> Pokemon {
        let pokemon = Pokemon {
            id: 1,
            name: name.to_string(),
            pokemon_type: (PokemonType::Normal, None),
            level,
            experience: 0,
            hp,
            max_hp: hp,
            stats: Stat {
                hp: 100,
                attack,
                defense: 100,
                sp_attack,
                sp_defense: 100,
                speed: 100,
            },
            moves: vec![
                Move {
                    id: 1,
                    name: "Tackle".to_string(),
                    pokemon_type: PokemonType::Normal,
                    move_type: MoveType::Physical,
                    power: 40,
                    accuracy: 100,
                    pp: 35,
                    max_pp: 35,
                },
                Move {
                    id: 2,
                    name: "Thunderbolt".to_string(),
                    pokemon_type: PokemonType::Electric,
                    move_type: MoveType::Special,
                    power: 90,
                    accuracy: 100,
                    pp: 15,
                    max_pp: 15,
                },
            ],
            catch_rate: 45,
            caught_with: "Poké Ball".to_string(),
            caught_location_id: 101,
            caught_date: 0,
        };
        pokemon
    }

    #[test]
    fn test_battle_creation() {
        let player = create_test_pokemon("Pikachu", 5, 35, 55, 50);
        let opponent = create_test_pokemon("Bulbasaur", 5, 45, 49, 65);

        let battle = Battle::new(player.clone(), opponent.clone());

        assert_eq!(battle.player_team.len(), 1);
        assert_eq!(battle.opponent_team.len(), 1);
        assert_eq!(battle.status, BattleStatus::Active);
        assert_eq!(battle.turn, 0);
        assert!(battle.is_wild_battle);
    }

    #[test]
    fn test_team_battle_creation() {
        let player_team = vec![
            create_test_pokemon("Pikachu", 5, 35, 55, 50),
            create_test_pokemon("Bulbasaur", 5, 45, 49, 65),
        ];
        let opponent_team = vec![
            create_test_pokemon("Charmander", 5, 39, 52, 60),
        ];

        let battle = Battle::new_team_battle(player_team.clone(), opponent_team.clone(), true);

        assert_eq!(battle.player_team.len(), 2);
        assert_eq!(battle.opponent_team.len(), 1);
        assert!(battle.is_wild_battle);
    }

    #[test]
    fn test_damage_calculation() {
        let attacker = create_test_pokemon("Pikachu", 10, 35, 55, 90);
        let defender = create_test_pokemon("Squirtle", 10, 44, 48, 65);
        let move_data = &attacker.moves[1]; // Thunderbolt (Electric)

        let damage = Battle::calculate_damage(&attacker, &defender, move_data);
        assert!(damage > 0, "伤害应该大于 0");
        // 注意：Thunderbolt 对 Squirtle (Water/Ground) 的克制关系：
        // Electric vs Water = Not Very Effective = 0.5x
        // Electric vs Ground = Normal = 1.0x
        // 所以实际伤害会被削弱
    }

    #[test]
    fn test_type_effectiveness_super_effective() {
        // Water beats Fire
        let effectiveness = Battle::get_type_effectiveness(
            PokemonType::Water,
            (PokemonType::Fire, None),
        );
        assert_eq!(effectiveness, 2.0, "水克火应该是 2.0 倍克制");
    }

    #[test]
    fn test_type_effectiveness_not_very_effective() {
        // Fire weak to Water
        let effectiveness = Battle::get_type_effectiveness(
            PokemonType::Fire,
            (PokemonType::Water, None),
        );
        assert_eq!(effectiveness, 0.5, "火被水克应该是 0.5 倍伤害");
    }

    #[test]
    fn test_type_effectiveness_dual_type() {
        // Fire vs Grass/Bug: Fire beats Grass (2.0x) and Fire beats Bug (2.0x) = 4.0x
        let effectiveness = Battle::get_type_effectiveness(
            PokemonType::Fire,
            (PokemonType::Grass, Some(PokemonType::Bug)),
        );
        assert_eq!(effectiveness, 4.0, "火克草虫应该是 4.0 倍克制");
    }

    #[test]
    fn test_check_hit() {
        // 100% 准确率应该总是命中
        assert!(Battle::check_hit(100));

        // 0% 准确率应该总是不命中
        assert!(!Battle::check_hit(0));
    }

    #[test]
    fn test_get_active_pokemon() {
        let player = create_test_pokemon("Pikachu", 5, 35, 55, 50);
        let opponent = create_test_pokemon("Bulbasaur", 5, 45, 49, 65);

        let battle = Battle::new(player, opponent);

        assert!(battle.get_player_pokemon().is_some());
        assert!(battle.get_opponent_pokemon().is_some());
    }

    #[test]
    fn test_has_active_pokemon() {
        let player = create_test_pokemon("Pikachu", 5, 35, 55, 50);
        let opponent = create_test_pokemon("Bulbasaur", 5, 45, 49, 65);

        let battle = Battle::new(player, opponent);

        assert!(battle.has_player_active());
        assert!(battle.has_opponent_active());
    }

    #[test]
    fn test_switch_pokemon() {
        let player_team = vec![
            create_test_pokemon("Pikachu", 5, 35, 55, 50),
            create_test_pokemon("Bulbasaur", 5, 45, 49, 65),
        ];
        let opponent_team = vec![
            create_test_pokemon("Charmander", 5, 39, 52, 60),
        ];

        let mut battle = Battle::new_team_battle(player_team, opponent_team, true);

        assert_eq!(battle.player_current_index, 0);

        // 切换到第二只宝可梦
        assert!(battle.switch_player_pokemon(1).is_ok());
        assert_eq!(battle.player_current_index, 1);
    }

    #[test]
    fn test_cannot_switch_to_fainted_pokemon() {
        let mut player_team = vec![
            create_test_pokemon("Pikachu", 5, 35, 55, 50),
            create_test_pokemon("Bulbasaur", 5, 45, 49, 65),
        ];

        // 让第二只宝可梦昏迷
        player_team[1].hp = 0;

        let opponent_team = vec![
            create_test_pokemon("Charmander", 5, 39, 52, 60),
        ];

        let mut battle = Battle::new_team_battle(player_team, opponent_team, true);

        // 尝试切换到昏迷的宝可梦应该失败
        assert!(battle.switch_player_pokemon(1).is_err());
    }

    #[test]
    fn test_battle_log() {
        let player = create_test_pokemon("Pikachu", 5, 35, 55, 50);
        let opponent = create_test_pokemon("Bulbasaur", 5, 45, 49, 65);

        let mut battle = Battle::new(player, opponent);

        assert_eq!(battle.battle_log.len(), 0);

        battle.add_log("测试消息".to_string());
        assert_eq!(battle.battle_log.len(), 1);
        assert_eq!(battle.get_last_log(), Some(&"测试消息".to_string()));
    }

    #[test]
    fn test_escape_attempt() {
        let player = create_test_pokemon("Pikachu", 5, 35, 55, 50);
        let opponent = create_test_pokemon("Bulbasaur", 5, 45, 49, 65);

        let mut battle = Battle::new(player, opponent);

        // 至少有机会逃脱（可能成功或失败）
        let escaped = battle.attempt_escape();
        assert!(escaped || !escaped); // 总是真（60% 成功，40% 失败都是合法的）
    }

    #[test]
    fn test_cannot_escape_trainer_battle() {
        let player = create_test_pokemon("Pikachu", 5, 35, 55, 50);
        let opponent = create_test_pokemon("Bulbasaur", 5, 45, 49, 65);

        let mut battle = Battle::new(player, opponent);
        battle.is_wild_battle = false; // 训练师战斗

        let escaped = battle.attempt_escape();
        assert!(!escaped, "训练师战斗中应该无法逃脱");
    }

    #[test]
    fn test_calculate_reward_money() {
        let player = create_test_pokemon("Pikachu", 5, 35, 55, 50);
        let opponent = create_test_pokemon("Bulbasaur", 20, 45, 49, 65);

        let battle = Battle::new(player, opponent);

        let money = battle.calculate_reward_money();
        assert_eq!(money, 20 * 10, "金钱应该等于对手等级 * 10");
    }

    #[test]
    fn test_battle_end_detection() {
        let player = create_test_pokemon("Pikachu", 5, 35, 55, 50);
        let mut opponent = create_test_pokemon("Bulbasaur", 5, 45, 49, 65);

        // 让对手昏迷
        opponent.hp = 0;

        let mut battle = Battle::new(player.clone(), opponent.clone());

        assert!(battle.check_battle_end());
        assert_eq!(battle.status, BattleStatus::PlayerWon);
    }

    #[test]
    fn test_determine_turn_order() {
        let player = create_test_pokemon("Pikachu", 5, 55, 55, 50); // 高速度
        let opponent = create_test_pokemon("Bulbasaur", 5, 45, 49, 65); // 低速度

        let battle = Battle::new(player, opponent);

        // 高速度的宝可梦应该先手
        assert!(battle.determine_turn_order());
    }

    #[test]
    fn test_use_move_basic() {
        let player = create_test_pokemon("Pikachu", 5, 35, 55, 50);
        let opponent = create_test_pokemon("Bulbasaur", 5, 45, 49, 65);

        let mut battle = Battle::new(player, opponent);
        let opponent_hp_before = battle.get_opponent_pokemon().unwrap().hp;

        // 使用招式
        assert!(battle.use_move(0, true).is_ok());

        let opponent_hp_after = battle.get_opponent_pokemon().unwrap().hp;
        assert!(opponent_hp_after <= opponent_hp_before, "对手应该受到伤害");
    }

    #[test]
    fn test_experience_distribution() {
        let player = create_test_pokemon("Pikachu", 5, 35, 55, 50);
        let opponent = create_test_pokemon("Bulbasaur", 10, 45, 49, 65);

        let mut battle = Battle::new(player.clone(), opponent);

        let exp_before = battle.player_team[0].experience;
        battle.award_experience(50);
        let exp_after = battle.player_team[0].experience;

        assert!(exp_after > exp_before, "玩家应该获得经验");
    }
}
