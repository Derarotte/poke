//! BattleHandler - Battle execution and coordination
//!
//! Responsible for:
//! - Setting up and executing battles
//! - Turn coordination and move execution
//! - Opponent AI decision making
//! - Experience and reward calculation

use crate::game::{Player, Pokemon, Battle, BattleStatus};
use crate::cli::BattleMenu;
use rand::Rng;

/// Represents a player action during battle
#[derive(Debug, Clone)]
pub enum BattleAction {
    /// Use a move at the given index
    UseMove(usize),
    /// Use an item with name
    UseItem(String),
    /// Switch to Pokemon at given index
    Switch(usize),
    /// Attempt to escape from battle
    Escape,
}

/// Result of a completed battle
#[derive(Debug, Clone)]
pub struct BattleResult {
    /// Whether the player won
    pub won: bool,
    /// Experience gained by player's Pokemon
    pub exp_gained: u32,
    /// Money gained by player
    pub money_gained: u32,
}

/// Handles battle execution and coordination
pub struct BattleHandler;

impl BattleHandler {
    /// Execute a wild Pokemon battle
    pub fn execute_wild_battle(
        player: &mut Player,
        opponent_team: Vec<Pokemon>,
    ) -> Result<BattleResult, String> {
        if !player.has_active_pokemon() {
            return Err("没有可用的宝可梦！".to_string());
        }

        let mut battle = Battle::new_team_battle(player.pokemons.clone(), opponent_team, true);

        // Display battle start
        if let Some(opponent) = battle.get_opponent_pokemon() {
            BattleMenu::display_battle_start(&opponent.name, true);
        }

        // Main battle loop
        Self::battle_loop(&mut battle)?;

        // Calculate results
        let result = Self::calculate_battle_result(&battle);

        // Update player team
        player.pokemons = battle.player_team.clone();

        Ok(result)
    }

    /// Execute an NPC trainer battle
    pub fn execute_npc_battle(
        player: &mut Player,
        opponent_team: Vec<Pokemon>,
    ) -> Result<BattleResult, String> {
        if !player.has_active_pokemon() {
            return Err("没有可用的宝可梦！".to_string());
        }

        let mut battle = Battle::new_team_battle(player.pokemons.clone(), opponent_team, false);

        // Display battle start
        if let Some(opponent) = battle.get_opponent_pokemon() {
            BattleMenu::display_battle_start(&opponent.name, false);
        }

        // Main battle loop
        Self::battle_loop(&mut battle)?;

        // Calculate results
        let result = Self::calculate_battle_result(&battle);

        // Update player team
        player.pokemons = battle.player_team.clone();

        Ok(result)
    }

    /// Main battle loop
    fn battle_loop(battle: &mut Battle) -> Result<(), String> {
        loop {
            // Check if battle has ended
            if battle.check_battle_end() {
                BattleMenu::display_battle_result(battle, battle.calculate_reward_money());
                break;
            }

            // Display current status
            BattleMenu::display_battle_screen(battle);
            BattleMenu::display_battle_log(battle, 3);

            // Get player action
            BattleMenu::display_main_menu(battle);
            let choice = BattleMenu::read_input();

            // Handle player action
            match choice.as_str() {
                "1" => {
                    // Use move
                    BattleMenu::display_move_menu(battle);
                    let move_choice = BattleMenu::read_input();

                    if let Ok(move_idx) = move_choice.parse::<usize>() {
                        if move_idx > 0 {
                            if let Err(e) = battle.use_move(move_idx - 1, true) {
                                println!("错误: {}", e);
                                continue;
                            }

                            // Opponent acts
                            if let Err(e) = Self::handle_opponent_action(battle) {
                                println!("对手动作失败: {}", e);
                            }
                        }
                    }
                }
                "2" => {
                    // Use item (stub for now)
                    println!("道具功能正在开发中...");
                }
                "3" => {
                    // Switch Pokemon
                    BattleMenu::display_switch_menu(battle);
                    let switch_choice = BattleMenu::read_input();

                    if let Ok(switch_idx) = switch_choice.parse::<usize>() {
                        if switch_idx > 0 {
                            let mut valid_count = 0;
                            for (i, pokemon) in battle.player_team.iter().enumerate() {
                                if !pokemon.is_fainted() && i != battle.player_current_index {
                                    valid_count += 1;
                                    if valid_count == switch_idx {
                                        if let Err(e) = battle.switch_player_pokemon(i) {
                                            println!("切换失败: {}", e);
                                        } else {
                                            println!("切换成功!");
                                            if let Err(e) = Self::handle_opponent_action(battle) {
                                                println!("对手动作失败: {}", e);
                                            }
                                        }
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                "4" if battle.is_wild_battle => {
                    // Attempt escape
                    if battle.attempt_escape() {
                        println!("成功逃脱战斗！");
                        break;
                    } else {
                        println!("逃脱失败！对手进行了反击！");
                        if let Err(e) = Self::handle_opponent_action(battle) {
                            println!("对手动作失败: {}", e);
                        }
                    }
                }
                "0" => {
                    println!("战斗已取消");
                    break;
                }
                _ => println!("无效的选择，请重试"),
            }
        }

        Ok(())
    }

    /// Handle opponent's action (simple AI)
    fn handle_opponent_action(battle: &mut Battle) -> Result<(), String> {
        if let Some(opponent) = battle.get_opponent_pokemon() {
            if !opponent.moves.is_empty() {
                let mut rng = rand::thread_rng();
                let move_idx = rng.gen_range(0..opponent.moves.len());
                battle.use_move(move_idx, false)?;
            }
        }
        Ok(())
    }

    /// Calculate battle result and rewards
    fn calculate_battle_result(battle: &Battle) -> BattleResult {
        let won = battle.status == BattleStatus::PlayerWon;
        let exp_gained = if won { 50 } else { 0 };
        let money_gained = if won {
            battle.calculate_reward_money()
        } else {
            0
        };

        BattleResult {
            won,
            exp_gained,
            money_gained,
        }
    }
}
