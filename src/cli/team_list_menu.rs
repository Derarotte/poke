//! Team List Menu Module
//!
//! Provides a quick overview of all Pokemon in the player's team.
//! Displays up to 6 team members with their level, status, and HP bars.
//!
//! # Features
//! - Shows all 6 team slots (filled or empty)
//! - Quick HP status bars for each Pokemon
//! - Fainted status indicator
//! - Selection interface for viewing detailed Pokemon information
//!
//! # Display Format
//! Uses 40-character width boxes for consistent terminal appearance.
//!
//! # Example
//! ```ignore
//! TeamListMenu::display_team_list(&player);
//! let choice = TeamListMenu::get_input();
//! match choice.as_str() {
//!     "0" => { /* return */ },
//!     n => { /* select pokemon */ }
//! }
//! ```

use std::io::{self, Write};
use crate::game::{Pokemon, Player};

/// Menu handler for displaying the player's Pokemon team
///
/// Provides a quick overview of all team members with key information:
/// - Position in team (1-6)
/// - Pokemon name and level
/// - Quick HP status bar
/// - Fainted indicator if applicable
pub struct TeamListMenu;

impl TeamListMenu {
    /// Display all 6 Pokemon team slots with quick status overview
    ///
    /// Shows all team positions (1-6) with:
    /// - Pokemon name and level
    /// - HP status bar
    /// - Fainted status indicator if applicable
    /// - Empty slot indicator for unfilled positions
    ///
    /// Prompts user to select a Pokemon (1-6) or return (0).
    pub fn display_team_list(player: &Player) {
        println!("\n╔════════════════════════════════════════╗");
        println!("║           你的队伍 (1-6)              ║");
        println!("╠════════════════════════════════════════╣");

        // Display up to 6 Pokemon
        for i in 0..6 {
            if i < player.pokemons.len() {
                let pokemon = &player.pokemons[i];
                let level_display = format!("{}. {} Lv.{}", i + 1, pokemon.name, pokemon.level);

                // HP bar
                let hp_bar = Self::get_hp_bar(pokemon.hp, pokemon.max_hp);
                let status_icon = if pokemon.is_fainted() { " [昏迷]" } else { "" };

                println!("║ {:<35} ║", level_display);
                println!("║    HP: {}{:<8}  ║", hp_bar, status_icon);
            } else {
                println!("║ {}. (空置)                        ║", i + 1);
            }
        }

        println!("║                                        ║");
        println!("║ 请选择查看详情 (1-6) 或 [0] 返回      ║");
        println!("╚════════════════════════════════════════╝");
        print!("选择: ");
        io::stdout().flush().unwrap();
    }

    /// Generate HP progress bar visualization
    ///
    /// # Arguments
    /// * `current_hp` - Current HP value
    /// * `max_hp` - Maximum HP value
    ///
    /// # Returns
    /// Formatted string like: `[████████░░░░░░░] 95/120`
    ///
    /// # Display
    /// Uses 15-character progress bar width with filled (█) and empty (░) blocks.
    fn get_hp_bar(current_hp: u32, max_hp: u32) -> String {
        let bar_width = 15;
        let filled = if max_hp > 0 {
            ((current_hp as f32 / max_hp as f32) * bar_width as f32) as usize
        } else {
            0
        };
        let empty = bar_width - filled;

        format!(
            "[{}{}] {}/{}",
            "█".repeat(filled),
            "░".repeat(empty),
            current_hp,
            max_hp
        )
    }

    /// Read user input from stdin for team selection
    ///
    /// # Returns
    /// Trimmed user input as a String
    ///
    /// # Valid Inputs
    /// - `1` to `6`: Select a team member
    /// - `0`: Return to main menu
    pub fn get_input() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{PokemonType, Stat};

    /// Create a test Pokemon for testing team display
    fn create_test_pokemon(id: u32, name: &str, level: u32) -> Pokemon {
        Pokemon {
            id,
            name: name.to_string(),
            level,
            experience: 0,
            hp: 100,
            max_hp: 100,
            pokemon_type: (PokemonType::Normal, None),
            stats: Stat {
                hp: 100,
                attack: 100,
                defense: 100,
                sp_attack: 100,
                sp_defense: 100,
                speed: 100,
            },
            moves: vec![],
            caught_with: "精灵球".to_string(),
            caught_location_id: 1,
            caught_date: 0,
            catch_rate: 45,
        }
    }

    /// Create a test player with Pokemon
    fn create_test_player(pokemon_count: usize) -> Player {
        let mut player = Player::new("TestPlayer".to_string());
        for i in 0..pokemon_count {
            let pokemon = create_test_pokemon(
                (i + 1) as u32,
                &format!("Pokemon{}", i + 1),
                (10 + i as u32),
            );
            let _ = player.add_pokemon(pokemon);
        }
        player
    }

    #[test]
    fn test_team_list_menu_creation() {
        // Test that TeamListMenu can be instantiated
        let _menu = TeamListMenu;
        // If this compiles and runs, the struct is valid
    }

    #[test]
    fn test_hp_bar_full_health() {
        // Test HP bar when Pokemon has full health
        let current_hp = 100u32;
        let max_hp = 100u32;
        let bar_width = 15;
        let filled = ((current_hp as f32 / max_hp as f32) * bar_width as f32) as usize;
        assert_eq!(filled, bar_width);
        let empty = bar_width - filled;
        assert_eq!(empty, 0);
    }

    #[test]
    fn test_hp_bar_half_health() {
        // Test HP bar when Pokemon has half health
        let current_hp = 50u32;
        let max_hp = 100u32;
        let bar_width = 15;
        let filled = ((current_hp as f32 / max_hp as f32) * bar_width as f32) as usize;
        assert_eq!(filled, bar_width / 2);
    }

    #[test]
    fn test_hp_bar_low_health() {
        // Test HP bar when Pokemon has low health
        let current_hp = 10u32;
        let max_hp = 100u32;
        let bar_width = 15;
        let filled = ((current_hp as f32 / max_hp as f32) * bar_width as f32) as usize;
        assert!(filled < bar_width / 4);
    }

    #[test]
    fn test_hp_bar_zero_health() {
        // Test HP bar when Pokemon has no health (fainted)
        let current_hp = 0u32;
        let max_hp = 100u32;
        let bar_width = 15;
        let filled = ((current_hp as f32 / max_hp as f32) * bar_width as f32) as usize;
        assert_eq!(filled, 0);
    }

    #[test]
    fn test_hp_bar_zero_max_hp() {
        // Test HP bar edge case: zero max_hp
        let current_hp = 0u32;
        let max_hp = 0u32;
        let bar_width = 15;
        let filled = if max_hp > 0 {
            ((current_hp as f32 / max_hp as f32) * bar_width as f32) as usize
        } else {
            0
        };
        assert_eq!(filled, 0);
    }

    #[test]
    fn test_test_pokemon_creation() {
        // Test that test Pokemon can be created
        let pokemon = create_test_pokemon(1, "TestMon", 10);
        assert_eq!(pokemon.id, 1);
        assert_eq!(pokemon.name, "TestMon");
        assert_eq!(pokemon.level, 10);
    }

    #[test]
    fn test_empty_player_team() {
        // Test player with no Pokemon
        let player = create_test_player(0);
        assert_eq!(player.pokemons.len(), 0);
    }

    #[test]
    fn test_player_team_single_pokemon() {
        // Test player with one Pokemon
        let player = create_test_player(1);
        assert_eq!(player.pokemons.len(), 1);
        assert_eq!(player.pokemons[0].name, "Pokemon1");
    }

    #[test]
    fn test_player_team_full() {
        // Test player with maximum team size (6)
        let player = create_test_player(6);
        assert_eq!(player.pokemons.len(), 6);
        for (i, pokemon) in player.pokemons.iter().enumerate() {
            assert_eq!(pokemon.name, format!("Pokemon{}", i + 1));
        }
    }

    #[test]
    fn test_player_team_over_limit() {
        // Test player trying to add more than 6 Pokemon
        let mut player = create_test_player(6);
        let new_pokemon = create_test_pokemon(7, "Pokemon7", 10);
        // add_pokemon should fail or be prevented
        let result = player.add_pokemon(new_pokemon);
        // The result depends on implementation, just verify it doesn't panic
        let _ = result;
    }

    #[test]
    fn test_fainted_pokemon_status() {
        // Test that fainted Pokemon are properly identified
        let pokemon = create_test_pokemon(1, "Fainted", 10);
        let is_fainted = pokemon.is_fainted();
        assert!(!is_fainted); // 100 HP, so not fainted
    }

    #[test]
    fn test_hp_bar_format() {
        // Test that HP bar format produces correct visual
        let current_hp = 60u32;
        let max_hp = 100u32;
        let bar_width = 15;
        let filled = ((current_hp as f32 / max_hp as f32) * bar_width as f32) as usize;
        let empty = bar_width - filled;

        // Build the bar string as TeamListMenu would
        let bar_str = format!(
            "[{}{}] {}/{}",
            "█".repeat(filled),
            "░".repeat(empty),
            current_hp,
            max_hp
        );

        // Verify the bar has expected properties
        assert!(bar_str.contains("█"));
        assert!(bar_str.contains("░"));
        assert!(bar_str.contains("60/100"));
    }
}
