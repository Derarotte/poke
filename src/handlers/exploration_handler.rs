//! ExplorationHandler - Location movement and exploration
//!
//! Responsible for:
//! - Location movement logic
//! - Movement validation
//! - Location unlock checking
//! - Exploration menu handling
//! - Location encounter triggering

use crate::game::{Player, Location};
use crate::cli::{LocationMenu, MapMenu};
use crate::map::GameMap;

/// Result of an exploration action
#[derive(Debug, Clone)]
pub enum ExplorationResult {
    /// Encounter was triggered
    EncounterTriggered,
    /// No encounter this time
    NoEncounter,
    /// Player moved to a new location
    LocationChanged(u32),
    /// Exploration was cancelled
    Cancelled,
}

/// Handles exploration and location movement
pub struct ExplorationHandler;

impl ExplorationHandler {
    /// Handle movement between locations
    pub fn handle_movement(player: &mut Player, _all_locations: &[Location]) -> Result<(), String> {
        let current_location = crate::data::locations_data::get_location_by_id(player.location_state.current_location_id);

        if let Some(location) = current_location {
            // Collect reachable locations
            let mut reachable = Vec::new();
            for connected_id in &location.connected_locations {
                if let Some(connected) = crate::data::locations_data::get_location_by_id(*connected_id) {
                    let is_unlocked = player.can_unlock_location(&connected.unlock_requirement);
                    reachable.push((*connected_id, connected.name.clone(), is_unlocked));
                }
            }

            if reachable.is_empty() {
                LocationMenu::show_no_reachable_locations();
                return Ok(());
            }

            LocationMenu::display_movement_menu(&location, &reachable);
            let choice = LocationMenu::get_input();

            if choice == "0" {
                return Ok(());
            }

            if let Ok(idx) = choice.parse::<usize>() {
                if idx > 0 && idx <= reachable.len() {
                    let (target_id, target_name, is_unlocked) = &reachable[idx - 1];

                    if !is_unlocked {
                        LocationMenu::show_movement_error("此地点未解锁");
                        return Ok(());
                    }

                    // Move to target location
                    player.location_state.current_location_id = *target_id;
                    player.location_state.mark_visited(*target_id);
                    LocationMenu::show_movement_success(target_name);
                }
            }
        }

        Ok(())
    }

    /// Handle exploration at current location
    pub fn handle_exploration(player: &mut Player, location: &Location) -> Result<ExplorationResult, String> {
        if !player.has_active_pokemon() {
            crate::cli::display::print_game_over();
            return Ok(ExplorationResult::Cancelled);
        }

        // Try to generate encounter
        match crate::handlers::EncounterManager::handle_encounter(player, location)? {
            crate::handlers::EncounterResult::Captured(_pokemon) => {
                // Pokemon captured - would be added to team/storage
                Ok(ExplorationResult::EncounterTriggered)
            }
            crate::handlers::EncounterResult::BattleInitiated => {
                // Battle initiated through encounter system
                Ok(ExplorationResult::EncounterTriggered)
            }
            crate::handlers::EncounterResult::Escaped => {
                Ok(ExplorationResult::NoEncounter)
            }
            crate::handlers::EncounterResult::Failed => {
                Ok(ExplorationResult::NoEncounter)
            }
        }
    }

    /// Handle map exploration
    pub fn explore_map(_player: &mut Player, game_map: &mut GameMap) {
        loop {
            MapMenu::print_map_navigation_menu();
            let choice = crate::cli::Menu::get_input();

            if choice == "0" {
                break;
            }

            if game_map.regions.iter().any(|r| r.id == choice.parse().ok().unwrap_or(0)) {
                // TODO: Implement region exploration
                println!("地区探索功能正在开发中...");
            } else {
                println!("无效的选择，请重试");
            }
        }
    }
}
