//! GameController - Central game orchestration
//!
//! Responsible for:
//! - Main game loop coordination
//! - State transitions (explore → battle → revival)
//! - Menu routing and user input delegation
//! - Player state management

use crate::game::Player;
use crate::cli::{Menu, TeamListMenu, PokemonDetailMenu};

/// Central game controller for orchestrating the main game flow
pub struct GameController;

impl GameController {
    /// Run the main game (entry point)
    pub fn run() {
        loop {
            Menu::print_main_menu();
            let choice = Menu::get_input();

            match choice.as_str() {
                "1" => Self::start_new_game(),
                "2" => {
                    println!("\n再见! 感谢游玩!");
                    break;
                }
                _ => println!("无效的选择，请重试"),
            }
        }
    }

    /// Initialize and start a new game
    fn start_new_game() {
        let player_name = Menu::get_player_name();
        let mut player = Player::new(player_name.clone());

        // Give player a starter Pokemon
        if let Some(starter_pokemon) = crate::data::pokemon_data::get_pokemon_by_id(25) {
            let _ = player.add_pokemon(starter_pokemon);
        }

        println!("\n═══════════════════════════════════════");
        println!("欢迎, {}!", player_name);
        println!("你获得了一只皮卡丘!");
        println!("═══════════════════════════════════════\n");

        Self::game_loop(&mut player);
    }

    /// Main game loop
    fn game_loop(player: &mut Player) {
        let all_locations = crate::data::locations_data::get_all_locations();

        loop {
            // Check for new unlocked locations
            player.check_new_unlocks(&all_locations);

            // Check if all Pokemon are fainted
            if !player.has_active_pokemon() {
                if !crate::handlers::RevivalHandler::check_and_handle_faint(player) {
                    break;
                }
                continue;
            }

            // Get current location
            let current_location_id = player.location_state.current_location_id;
            let visited_count = player.location_state.visited_count();
            let current_location = crate::data::locations_data::get_location_by_id(current_location_id);

            // Display menu
            if let Some(location) = &current_location {
                Menu::print_game_menu_with_location(&location.name, visited_count, all_locations.len());
            } else {
                Menu::print_game_menu();
            }

            let choice = Menu::get_input();

            // Route to appropriate handler
            match choice.as_str() {
                "1" => {
                    if let Some(location) = &current_location {
                        let _ = crate::handlers::ExplorationHandler::handle_exploration(player, location);
                    }
                }
                "2" => {
                    let _ = crate::handlers::ExplorationHandler::handle_movement(player, &all_locations);
                }
                "3" => {
                    // Display team with detail viewing capability
                    Self::view_team_details(player);
                }
                "4" => player.display_items(),
                "5" => {
                    use crate::map::{GameMap, create_locations};
                    let mut game_map = GameMap::new();
                    game_map.initialize(create_locations());
                    crate::handlers::ExplorationHandler::explore_map(player, &mut game_map);
                }
                "6" => {
                    println!("\n感谢游玩!");
                    break;
                }
                _ => println!("无效的选择，请重试"),
            }
        }
    }

    /// View team details with Pokemon selection and detailed information
    fn view_team_details(player: &Player) {
        loop {
            TeamListMenu::display_team_list(player);
            let choice = TeamListMenu::get_input();

            if choice == "0" {
                break; // Return to main menu
            }

            if let Ok(idx) = choice.parse::<usize>() {
                if idx > 0 && idx <= player.pokemons.len() {
                    // Show detail view for selected Pokemon
                    Self::show_pokemon_detail(player, idx - 1);
                } else if idx <= 6 {
                    // Selected slot is empty (between len() and 6)
                    println!("\n⚠ 此位置为空");
                }
            }
        }
    }

    /// Show detailed information for a specific Pokemon with navigation
    fn show_pokemon_detail(player: &Player, pokemon_index: usize) {
        loop {
            if pokemon_index < player.pokemons.len() {
                let pokemon = &player.pokemons[pokemon_index];
                PokemonDetailMenu::display_pokemon_detail(pokemon, pokemon_index, player.pokemons.len());
                let choice = PokemonDetailMenu::get_input();

                match choice.as_str() {
                    "0" | "r" | "R" => break, // Return to team list
                    "<" | "a" | "A" => {
                        // Previous Pokemon
                        if pokemon_index > 0 {
                            Self::show_pokemon_detail(player, pokemon_index - 1);
                            break;
                        }
                    }
                    ">" | "d" | "D" => {
                        // Next Pokemon
                        if pokemon_index < player.pokemons.len() - 1 {
                            Self::show_pokemon_detail(player, pokemon_index + 1);
                            break;
                        }
                    }
                    _ => {
                        // Do nothing for invalid input
                    }
                }
            } else {
                break;
            }
        }
    }
}
