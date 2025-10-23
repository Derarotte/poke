//! RevivalHandler - Pokemon faint detection and revival mechanics
//!
//! Responsible for:
//! - Pokemon faint detection
//! - Item-based revival
//! - Pokemon center mechanics
//! - Cost calculation and payment

use crate::game::Player;
use crate::cli::{RevivalMenu, Menu};

/// Handles Pokemon revival and faint detection
pub struct RevivalHandler;

impl RevivalHandler {
    /// Check if all Pokemon are fainted and handle revival if needed
    pub fn check_and_handle_faint(player: &mut Player) -> bool {
        if !player.all_pokemon_fainted() {
            return true;
        }

        // All Pokemon are fainted - need to revive
        loop {
            RevivalMenu::print_all_pokemon_fainted_menu();
            let choice = Menu::get_input();

            match choice.as_str() {
                "1" => {
                    if Self::handle_item_revival(player) {
                        return true;
                    }
                }
                "2" => {
                    if Self::handle_pokemon_center_revival(player) {
                        return true;
                    }
                }
                "3" => {
                    player.display_team();
                }
                "4" => {
                    println!("\n游戏结束！感谢游玩!");
                    return false;
                }
                _ => println!("无效的选择，请重试"),
            }
        }
    }

    /// Handle revival using items
    fn handle_item_revival(player: &mut Player) -> bool {
        RevivalMenu::print_select_pokemon_to_revive_menu(player);
        let input = Menu::get_input();

        if input == "0" {
            return false;
        }

        if let Ok(idx) = input.parse::<usize>() {
            if idx > 0 {
                // Find the idx-th fainted Pokemon
                let mut fainted_index = 0;
                let mut target_pokemon_idx = None;
                for (i, pokemon) in player.pokemons.iter().enumerate() {
                    if pokemon.is_fainted() {
                        fainted_index += 1;
                        if fainted_index == idx {
                            target_pokemon_idx = Some(i);
                            break;
                        }
                    }
                }

                if let Some(pokemon_index) = target_pokemon_idx {
                    RevivalMenu::print_recovery_item_menu(player);
                    let item_choice = Menu::get_input();

                    if item_choice == "0" {
                        return false;
                    }

                    let recovery_items = vec!["恢复药", "超级恢复药", "全复活", "完全恢复"];
                    let mut available_idx = 0;
                    for item_name in recovery_items {
                        if player.items.get(item_name).copied().unwrap_or(0) > 0 {
                            available_idx += 1;
                            if item_choice == available_idx.to_string() {
                                match player.revive_pokemon_with_item(pokemon_index, item_name) {
                                    Ok(msg) => {
                                        RevivalMenu::print_revival_success(&msg);
                                        return true;
                                    }
                                    Err(e) => {
                                        RevivalMenu::print_revival_failed(&e);
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        false
    }

    /// Handle revival at Pokemon center
    fn handle_pokemon_center_revival(player: &mut Player) -> bool {
        RevivalMenu::print_pokemon_center_menu(player);
        let choice = Menu::get_input();

        match choice.as_str() {
            "1" => {
                // Revive all
                match player.revive_full_team_at_center() {
                    Ok(msg) => {
                        RevivalMenu::print_revival_success(&msg);
                        RevivalMenu::print_first_visit_bonus();
                        return true;
                    }
                    Err(e) => {
                        RevivalMenu::print_revival_failed(&e);
                        return false;
                    }
                }
            }
            "0" => {
                return false;
            }
            _ => println!("无效的选择，请重试"),
        }

        false
    }
}
