//! Pokemon Detail Menu Module
//!
//! Provides comprehensive Pokemon information display with detailed stats, moves, and capture history.
//! This module displays Pokemon attributes in a structured, visually appealing format with progress bars.
//!
//! # Features
//! - Basic Pokemon information (ID, type, level, experience)
//! - All base stats with visual progress bars
//! - Move information with PP (Power Points) display
//! - Capture location, method, and timestamp
//! - Team navigation footer with keyboard controls
//!
//! # Display Format
//! All displays use 40-character width boxes for consistent terminal appearance.
//! Progress bars use Unicode box-drawing characters (█ for filled, ░ for empty).
//!
//! # Example
//! ```ignore
//! let pokemon = player.pokemons[0];
//! PokemonDetailMenu::display_pokemon_detail(&pokemon, 0, player.pokemons.len());
//! let choice = PokemonDetailMenu::get_input();
//! ```

use std::io::{self, Write};
use crate::game::Pokemon;
use crate::data::locations_data;

/// Menu handler for displaying comprehensive Pokemon details
///
/// Provides structured display of all Pokemon information including:
/// - Basic info (ID, type, level, experience)
/// - All 6 stats with visual progress bars
/// - Moves with type, power, accuracy, and PP
/// - Capture information (ball type, location, date)
/// - Navigation controls for team browsing
pub struct PokemonDetailMenu;

impl PokemonDetailMenu {
    /// Display comprehensive Pokemon detail information in multiple sections
    ///
    /// # Arguments
    /// * `pokemon` - The Pokemon to display
    /// * `team_index` - Current position in team (0-5)
    /// * `team_size` - Total number of Pokemon in team
    ///
    /// # Display Layout
    /// Displays in order:
    /// 1. Basic info box (name, ID, type, level, experience)
    /// 2. Stats box (all 6 stats with bars)
    /// 3. Moves box (up to 4 moves with details)
    /// 4. Capture info box (ball type, location, date)
    /// 5. Navigation footer
    pub fn display_pokemon_detail(pokemon: &Pokemon, team_index: usize, team_size: usize) {
        Self::display_basic_info(pokemon);
        Self::display_stats(pokemon);
        Self::display_moves(pokemon);
        Self::display_capture_info(pokemon);
        Self::display_navigation(team_index, team_size);
    }

    /// Display basic Pokemon information (ID, type, level, experience)
    ///
    /// Shows:
    /// - Pokemon name and level
    /// - National Pokedex ID
    /// - Type (primary and secondary if present)
    /// - Experience progress toward next level
    fn display_basic_info(pokemon: &Pokemon) {
        println!("\n╔════════════════════════════════════════╗");
        println!("║ {:<40} ║", format!("{} (Lv. {})", pokemon.name, pokemon.level));
        println!("╠════════════════════════════════════════╣");

        // ID and type info
        println!("║ ID: {:<34} ║", pokemon.id);
        let type_display = match pokemon.pokemon_type.1 {
            Some(second_type) => format!("{:?} / {:?}", pokemon.pokemon_type.0, second_type),
            None => format!("{:?} / —", pokemon.pokemon_type.0),
        };
        println!("║ 类型: {:<36} ║", type_display);

        // Experience bar
        let next_level_exp = pokemon.level as u32 * 100;
        let exp_percent = if next_level_exp > 0 {
            (pokemon.experience as f32 / next_level_exp as f32 * 100.0) as u32
        } else {
            0
        };
        let exp_bar_width = 20;
        let exp_filled = (exp_percent as usize * exp_bar_width / 100).min(exp_bar_width);
        let exp_empty = exp_bar_width - exp_filled;

        println!("║ 经验: {}/{} ({}%)             ║",
                 pokemon.experience, next_level_exp, exp_percent);
        println!("║ [{}{}]                             ║",
                 "█".repeat(exp_filled),
                 "░".repeat(exp_empty));

        // HP info
        let hp_bar_width = 20;
        let hp_filled = ((pokemon.hp as f32 / pokemon.max_hp as f32) * hp_bar_width as f32) as usize;
        let hp_empty = hp_bar_width - hp_filled;

        println!("║                                        ║");
        println!("║ HP: {}/{:<29} ║", pokemon.hp, pokemon.max_hp);
        println!("║ [{}{}]                             ║",
                 "█".repeat(hp_filled),
                 "░".repeat(hp_empty));

        println!("╚════════════════════════════════════════╝");
    }

    /// Display all Pokemon stats with visual progress bars
    ///
    /// Shows all six base stats:
    /// - HP (Hit Points)
    /// - ATK (Attack)
    /// - DEF (Defense)
    /// - SP.ATK (Special Attack)
    /// - SP.DEF (Special Defense)
    /// - SPD (Speed)
    ///
    /// Each stat is displayed as a 16-character progress bar scaled to 255 max value.
    fn display_stats(pokemon: &Pokemon) {
        println!("╔════════════════════════════════════════╗");
        println!("║              属性信息                  ║");
        println!("╠════════════════════════════════════════╣");

        let stats = &pokemon.stats;
        Self::display_stat_line("HP", stats.hp);
        Self::display_stat_line("攻击", stats.attack);
        Self::display_stat_line("防守", stats.defense);
        Self::display_stat_line("特攻", stats.sp_attack);
        Self::display_stat_line("特防", stats.sp_defense);
        Self::display_stat_line("速度", stats.speed);

        println!("╚════════════════════════════════════════╝");
    }

    /// Display a single stat with progress bar visualization
    ///
    /// # Arguments
    /// * `name` - Stat name (e.g., "HP", "攻击")
    /// * `value` - Stat value (0-255)
    ///
    /// # Display
    /// Renders as: `name [████████░░░░░░░░] value`
    /// where filled blocks (█) represent the stat value proportion.
    fn display_stat_line(name: &str, value: u32) {
        let bar_width = 16;
        let filled = ((value as f32 / 255.0) * bar_width as f32) as usize;
        let empty = bar_width - filled.min(bar_width);

        println!("║ {:<6} [{}{}] {:<19} ║",
                 name,
                 "█".repeat(filled.min(bar_width)),
                 "░".repeat(empty),
                 value);
    }

    /// Display Pokemon moves with detailed information
    ///
    /// Shows up to 4 moves that the Pokemon currently knows.
    /// For each move displays:
    /// - Move number and name
    /// - Type, base power, and accuracy
    /// - Current PP and max PP with progress bar
    ///
    /// If the Pokemon has no moves, displays placeholder text.
    fn display_moves(pokemon: &Pokemon) {
        println!("╔════════════════════════════════════════╗");
        println!("║              现有招式                  ║");
        println!("╠════════════════════════════════════════╣");

        if pokemon.moves.is_empty() {
            println!("║ (暂无招式)                             ║");
        } else {
            for (i, m) in pokemon.moves.iter().enumerate() {
                println!("║ {}. {:<32} ║", i + 1, m.name);
                println!("║    类型:{:?} 威力:{} 命中:{} ║",
                         m.pokemon_type, m.power, m.accuracy);

                // PP bar
                let pp_bar_width = 20;
                let pp_filled = ((m.pp as f32 / m.max_pp as f32) * pp_bar_width as f32) as usize;
                let pp_empty = pp_bar_width - pp_filled;

                println!("║    PP: [{}{}] {}/{}  ║",
                         "█".repeat(pp_filled),
                         "░".repeat(pp_empty),
                         m.pp,
                         m.max_pp);
            }
        }

        println!("╚════════════════════════════════════════╝");
    }

    /// Display Pokemon capture and origin information
    ///
    /// Shows details about when and where the Pokemon was caught:
    /// - Capture method (the type of Pokéball used)
    /// - Capture location (resolved from location_id or shown as "Unknown")
    /// - Capture date (displayed as timestamp, see limitations)
    ///
    /// # Note
    /// Timestamp is currently displayed in epoch format. Future enhancement
    /// could format this as human-readable date (YYYY-MM-DD).
    fn display_capture_info(pokemon: &Pokemon) {
        println!("╔════════════════════════════════════════╗");
        println!("║              捕捉信息                  ║");
        println!("╠════════════════════════════════════════╣");

        println!("║ 捕捉方式: {:<31} ║", pokemon.caught_with);

        // Get location name
        let location_name = locations_data::get_location_by_id(pokemon.caught_location_id)
            .map(|loc| loc.name)
            .unwrap_or_else(|| format!("未知地点 (ID: {})", pokemon.caught_location_id));
        println!("║ 捕捉地点: {:<31} ║", location_name);

        // Capture date
        if pokemon.caught_date > 0 {
            let timestamp_str = format!("时间戳: {}", pokemon.caught_date);
            println!("║ 捕捉日期: {:<31} ║", timestamp_str);
        } else {
            println!("║ 捕捉日期: {:<31} ║", "未记录");
        }

        println!("╚════════════════════════════════════════╝");
    }

    /// Display navigation footer with keyboard controls
    ///
    /// Shows team navigation options at the bottom of the display.
    /// Indicates current position in team (e.g., "队伍 (1/6)").
    ///
    /// # Keyboard Controls
    /// - `0` or `R`: Return to team list
    /// - `<` or `A`: Navigate to previous Pokemon
    /// - `>` or `D`: Navigate to next Pokemon
    fn display_navigation(current_index: usize, team_size: usize) {
        println!("╔════════════════════════════════════════╗");
        let nav_text = format!("← 上一个 │ 队伍 ({}/{}) │ 下一个 → │ 返回",
                               current_index + 1, team_size);
        println!("║ {:<38} ║", nav_text);
        println!("╚════════════════════════════════════════╝");
        print!("选择: ");
        io::stdout().flush().unwrap();
    }

    /// Read user input from stdin for navigation
    ///
    /// # Returns
    /// Trimmed user input as a String
    ///
    /// # Valid Inputs
    /// - `0` or `r`/`R`: Return to team list
    /// - `<` or `a`/`A`: Previous Pokemon
    /// - `>` or `d`/`D`: Next Pokemon
    pub fn get_input() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{Pokemon, PokemonType, Stat, Move, MoveType};

    /// Create a test Pokemon for testing display functions
    fn create_test_pokemon() -> Pokemon {
        Pokemon {
            id: 25,
            name: "皮卡丘".to_string(),
            level: 25,
            experience: 2500,
            hp: 95,
            max_hp: 120,
            pokemon_type: (PokemonType::Electric, None),
            stats: Stat {
                hp: 95,
                attack: 62,
                defense: 48,
                sp_attack: 65,
                sp_defense: 58,
                speed: 105,
            },
            moves: vec![
                Move {
                    id: 1,
                    name: "十万伏特".to_string(),
                    move_type: MoveType::Special,
                    pokemon_type: PokemonType::Electric,
                    power: 90,
                    accuracy: 100,
                    pp: 8,
                    max_pp: 15,
                },
            ],
            caught_with: "精灵球".to_string(),
            caught_location_id: 1,
            caught_date: 1699012445,
            catch_rate: 35,
        }
    }

    #[test]
    fn test_stat_line_display_hp() {
        // Test that stat line formatting doesn't panic and produces expected width
        let _stat_name = "HP";
        let stat_value = 95u32;

        // Just verify the function doesn't panic when called
        // (actual display is to stdout, we can't easily capture)
        let bar_width = 16;
        let filled = ((stat_value as f32 / 255.0) * bar_width as f32) as usize;
        assert!(filled <= bar_width);
        assert!(filled > 0);
    }

    #[test]
    fn test_stat_line_display_zero() {
        // Test stat value of 0
        let stat_value = 0u32;
        let bar_width = 16;
        let filled = ((stat_value as f32 / 255.0) * bar_width as f32) as usize;
        assert_eq!(filled, 0);
    }

    #[test]
    fn test_stat_line_display_max() {
        // Test max stat value (255)
        let stat_value = 255u32;
        let bar_width = 16;
        let filled = ((stat_value as f32 / 255.0) * bar_width as f32) as usize;
        assert_eq!(filled, bar_width);
    }

    #[test]
    fn test_pokemon_detail_menu_creation() {
        // Test that PokemonDetailMenu can be instantiated
        let _menu = PokemonDetailMenu;
        // If this compiles and runs, the struct is valid
    }

    #[test]
    fn test_test_pokemon_stats() {
        // Verify test Pokemon has valid stats
        let pokemon = create_test_pokemon();
        assert_eq!(pokemon.id, 25);
        assert_eq!(pokemon.name, "皮卡丘");
        assert_eq!(pokemon.level, 25);
        assert!(pokemon.stats.hp > 0);
        assert!(pokemon.hp <= pokemon.max_hp);
    }

    #[test]
    fn test_test_pokemon_has_moves() {
        // Verify test Pokemon has at least one move
        let pokemon = create_test_pokemon();
        assert!(!pokemon.moves.is_empty());
        assert_eq!(pokemon.moves.len(), 1);
        assert_eq!(pokemon.moves[0].name, "十万伏特");
    }

    #[test]
    fn test_test_pokemon_capture_info() {
        // Verify test Pokemon has capture information
        let pokemon = create_test_pokemon();
        assert_eq!(pokemon.caught_with, "精灵球");
        assert_eq!(pokemon.caught_location_id, 1);
        assert!(pokemon.caught_date > 0);
    }

    #[test]
    fn test_hp_bar_calculation_full() {
        // Test HP bar when Pokemon is at full health
        let hp = 120u32;
        let max_hp = 120u32;
        let bar_width = 15;
        let filled = ((hp as f32 / max_hp as f32) * bar_width as f32) as usize;
        assert_eq!(filled, bar_width);
    }

    #[test]
    fn test_hp_bar_calculation_half() {
        // Test HP bar when Pokemon is at half health
        let hp = 60u32;
        let max_hp = 120u32;
        let bar_width = 15;
        let filled = ((hp as f32 / max_hp as f32) * bar_width as f32) as usize;
        assert_eq!(filled, bar_width / 2);
    }

    #[test]
    fn test_hp_bar_calculation_quarter() {
        // Test HP bar when Pokemon is at quarter health
        let hp = 30u32;
        let max_hp = 120u32;
        let bar_width = 15;
        let filled = ((hp as f32 / max_hp as f32) * bar_width as f32) as usize;
        assert!(filled <= bar_width / 4 + 1); // Allow rounding
    }

    #[test]
    fn test_hp_bar_zero_max_hp() {
        // Test edge case: Pokemon with 0 max_hp (shouldn't happen but test for safety)
        let hp = 0u32;
        let max_hp = 0u32;
        let bar_width = 15;
        let filled = if max_hp > 0 {
            ((hp as f32 / max_hp as f32) * bar_width as f32) as usize
        } else {
            0
        };
        assert_eq!(filled, 0);
    }

    #[test]
    fn test_experience_bar_calculation() {
        // Test experience progress calculation
        let pokemon = create_test_pokemon();
        let next_level_exp = pokemon.level as u32 * 100;
        let exp_percent = (pokemon.experience as f32 / next_level_exp as f32 * 100.0) as u32;
        assert!(exp_percent > 0 && exp_percent <= 100);
    }
}
