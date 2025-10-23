//! EncounterManager - Wild Pokemon encounter handling
//!
//! Responsible for:
//! - Generating wild Pokemon encounters
//! - Encounter preview display
//! - User action handling (fight/capture/flee)
//! - Capture rate calculation

use crate::game::{Player, Pokemon, WildPokemonEncounter, Location};
use crate::cli::LocationMenu;
use crate::handlers::BattleHandler;
use crate::pokemon_generator::PokemonInstance;
use rand::Rng;

/// Result of a wild Pokemon encounter
#[derive(Debug, Clone)]
pub enum EncounterResult {
    /// Pokemon was successfully captured
    Captured(Pokemon),
    /// Battle was initiated
    BattleInitiated,
    /// Player successfully escaped
    Escaped,
    /// Encounter failed or was cancelled
    Failed,
}

/// Manages wild Pokemon encounters
pub struct EncounterManager;

impl EncounterManager {
    /// Handle an encounter in a location
    pub fn handle_encounter(player: &mut Player, location: &Location) -> Result<EncounterResult, String> {
        // Check encounter chance
        let mut rng = rand::thread_rng();
        let encounter_chance: f32 = rng.gen();

        if encounter_chance > location.encounter_rate {
            println!("\n你探索了 {}，但没有遇到任何宝可梦。", location.name);
            return Ok(EncounterResult::Failed);
        }

        // Generate wild Pokemon
        if location.wild_pokemon_pool.is_empty() {
            println!("\n这个地点没有野生宝可梦。");
            return Ok(EncounterResult::Failed);
        }

        match WildPokemonEncounter::generate_wild_pokemon(&location.wild_pokemon_pool) {
            Ok(wild_pokemon_instance) => {
                let environment_bonus = crate::game::EnvironmentBonus::from_environment(location.environment);

                match WildPokemonEncounter::generate_preview(
                    &wild_pokemon_instance,
                    &environment_bonus,
                    location.environment_name(),
                ) {
                    Ok(preview) => {
                        preview.display();
                        Self::handle_encounter_choice(player, wild_pokemon_instance)
                    }
                    Err(e) => {
                        println!("生成预览失败: {}", e);
                        Ok(EncounterResult::Failed)
                    }
                }
            }
            Err(e) => {
                println!("生成野生宝可梦失败: {}", e);
                Ok(EncounterResult::Failed)
            }
        }
    }

    /// Handle player's choice during encounter
    fn handle_encounter_choice(player: &mut Player, wild_pokemon_instance: PokemonInstance) -> Result<EncounterResult, String> {
        loop {
            let choice = LocationMenu::get_input();

            match choice.as_str() {
                "1" => {
                    // Attempt capture
                    println!("捕捉功能即将实现");
                    return Ok(EncounterResult::Failed);
                }
                "2" => {
                    // Initiate battle - convert PokemonInstance to Pokemon first
                    if let Some(mut wild_pokemon) = crate::data::pokemon_data::get_pokemon_by_id(wild_pokemon_instance.species_id) {
                        // Update level to match generated instance
                        wild_pokemon.level = wild_pokemon_instance.level;
                        wild_pokemon.experience = wild_pokemon_instance.experience;

                        let opponent_team = vec![wild_pokemon];
                        match BattleHandler::execute_wild_battle(player, opponent_team) {
                            Ok(result) => {
                                if result.won {
                                    println!("\n你赢了！");
                                } else {
                                    println!("\n你输了！");
                                }
                                return Ok(EncounterResult::BattleInitiated);
                            }
                            Err(e) => {
                                println!("战斗错误: {}", e);
                                return Ok(EncounterResult::Failed);
                            }
                        }
                    } else {
                        println!("无法创建对手宝可梦");
                        return Ok(EncounterResult::Failed);
                    }
                }
                "3" => {
                    // Attempt escape
                    if Self::attempt_escape() {
                        println!("你成功逃跑了!");
                        return Ok(EncounterResult::Escaped);
                    } else {
                        println!("逃脱失败！");
                        // Could trigger opponent attack here
                    }
                }
                _ => println!("无效的选择，请重试"),
            }
        }
    }

    /// Attempt to escape from an encounter
    fn attempt_escape() -> bool {
        let mut rng = rand::thread_rng();
        rng.gen::<f32>() > 0.4 // 60% success rate
    }

    /// Calculate capture success probability
    pub fn attempt_capture(pokemon: &Pokemon) -> bool {
        let mut rng = rand::thread_rng();
        let capture_rate = pokemon.catch_rate as f32 / 255.0;
        let hp_ratio = pokemon.hp as f32 / pokemon.max_hp as f32;
        let success_rate = capture_rate * (1.0 - hp_ratio * 0.5);
        rng.gen::<f32>() < success_rate
    }
}
