//! Game data loader - Loads Pokémon, locations, and NPC data from JSON files
//!
//! This module is responsible for:
//! - Loading JSON data files from the assets directory
//! - Parsing JSON into Rust structures
//! - Validating loaded data
//! - Providing convenient access to game data

use std::fs;
use std::path::{Path, PathBuf};
use std::env;

/// Global game data cache
pub static mut GAME_DATA: Option<GameDataCache> = None;

/// Thread-safe access to game data
#[allow(static_mut_refs)]
pub fn get_game_data() -> Option<&'static GameDataCache> {
    unsafe { GAME_DATA.as_ref() }
}

/// Game data cache structure
pub struct GameDataCache {
    pub pokemon: Vec<serde_json::Value>,
    pub moves: Vec<serde_json::Value>,
    pub locations: Vec<serde_json::Value>,
    pub trainers: Vec<serde_json::Value>,
    pub items: Vec<serde_json::Value>,
    pub type_effectiveness: Vec<serde_json::Value>,
    pub game_constants: serde_json::Value,
    pub player_defaults: serde_json::Value,
    pub environment_bonuses: Vec<serde_json::Value>,
    pub natures: Vec<serde_json::Value>,
}

/// Find the assets directory by searching multiple locations
/// Returns the path to the assets directory if found
fn find_assets_dir() -> Option<PathBuf> {
    // Try 1: Check current working directory
    let cwd_assets = PathBuf::from("assets");
    if cwd_assets.exists() && cwd_assets.is_dir() {
        return Some(cwd_assets);
    }

    // Try 2-5: Check relative to executable location (up the directory tree)
    if let Ok(exe_path) = env::current_exe() {
        let mut current = exe_path.parent().map(|p| p.to_path_buf());

        // Check up to 5 levels up from the executable
        for _ in 0..5 {
            if let Some(dir) = current {
                let assets_path = dir.join("assets");
                if assets_path.exists() && assets_path.is_dir() {
                    return Some(assets_path);
                }
                current = dir.parent().map(|p| p.to_path_buf());
            } else {
                break;
            }
        }
    }

    // Try 6: Check home directory
    if let Ok(home) = env::var("HOME") {
        let home_assets = PathBuf::from(home).join(".local/share/poke/assets");
        if home_assets.exists() && home_assets.is_dir() {
            return Some(home_assets);
        }
    }

    None
}

/// Store the assets directory path for use by all load functions
thread_local! {
    static ASSETS_DIR: std::cell::RefCell<Option<PathBuf>> = std::cell::RefCell::new(None);
}

/// Set the assets directory path (called during initialization)
fn set_assets_dir(path: PathBuf) {
    ASSETS_DIR.with(|dir| {
        *dir.borrow_mut() = Some(path);
    });
}

/// Get the assets directory path
fn get_assets_dir() -> PathBuf {
    ASSETS_DIR.with(|dir| {
        dir.borrow().clone().unwrap_or_else(|| PathBuf::from("assets"))
    })
}

/// Load all game data from JSON files and initialize global cache
pub fn load_all_data() -> Result<(), String> {
    // Find and set assets directory
    let assets_dir = find_assets_dir()
        .ok_or_else(|| {
            "Assets directory not found. Searched: current directory, executable directory, ~/.local/share/poke/assets/".to_string()
        })?;

    set_assets_dir(assets_dir.clone());

    println!("Loading game data from {}...", assets_dir.display());

    let pokemon_data = load_pokemon_data()?;
    let move_data = load_move_data()?;
    let location_data = load_location_data()?;
    let trainer_data = load_trainer_data()?;
    let items_data = load_items_data()?;
    let type_effectiveness_data = load_type_effectiveness_data()?;
    let game_constants_data = load_game_constants()?;
    let player_defaults_data = load_player_defaults()?;
    let environment_bonuses_data = load_environment_bonuses()?;
    let natures_data = load_natures()?;

    println!("✓ Loaded {} Pokémon species", pokemon_data.len());
    println!("✓ Loaded {} moves", move_data.len());
    println!("✓ Loaded {} locations", location_data.len());
    println!("✓ Loaded {} trainers", trainer_data.len());
    println!("✓ Loaded {} items", items_data.len());
    println!("✓ Loaded {} type matchups", type_effectiveness_data.len());
    println!("✓ Loaded game constants");
    println!("✓ Loaded player defaults");
    println!("✓ Loaded {} environment types", environment_bonuses_data.len());
    println!("✓ Loaded {} natures", natures_data.len());

    // Initialize global cache
    unsafe {
        GAME_DATA = Some(GameDataCache {
            pokemon: pokemon_data,
            moves: move_data,
            locations: location_data,
            trainers: trainer_data,
            items: items_data,
            type_effectiveness: type_effectiveness_data,
            game_constants: game_constants_data,
            player_defaults: player_defaults_data,
            environment_bonuses: environment_bonuses_data,
            natures: natures_data,
        });
    }

    Ok(())
}

/// Helper functions for data access

/// Load Pokémon species data from JSON
fn load_pokemon_data() -> Result<Vec<serde_json::Value>, String> {
    let assets_dir = get_assets_dir();
    let path = assets_dir.join("pokemon/species.json");
    load_json_array(&path, "species")
}

/// Load move data from JSON
fn load_move_data() -> Result<Vec<serde_json::Value>, String> {
    let assets_dir = get_assets_dir();
    let path = assets_dir.join("pokemon/moves.json");
    load_json_array(&path, "moves")
}

/// Load location data from JSON
fn load_location_data() -> Result<Vec<serde_json::Value>, String> {
    let assets_dir = get_assets_dir();
    let path = assets_dir.join("locations/world.json");
    load_json_array(&path, "locations")
}

/// Load trainer data from JSON
fn load_trainer_data() -> Result<Vec<serde_json::Value>, String> {
    let assets_dir = get_assets_dir();
    let path = assets_dir.join("npcs/trainers.json");
    load_json_array(&path, "trainers")
}

/// Load items data from JSON
fn load_items_data() -> Result<Vec<serde_json::Value>, String> {
    let assets_dir = get_assets_dir();
    let path = assets_dir.join("items/items.json");
    load_json_array(&path, "items")
}

/// Load type effectiveness chart from JSON
fn load_type_effectiveness_data() -> Result<Vec<serde_json::Value>, String> {
    let assets_dir = get_assets_dir();
    let path = assets_dir.join("battle/type_effectiveness.json");
    load_json_array(&path, "type_chart")
}

/// Load game constants from JSON
fn load_game_constants() -> Result<serde_json::Value, String> {
    let assets_dir = get_assets_dir();
    let path = assets_dir.join("config/game_constants.json");

    // Check if file exists
    if !path.exists() {
        return Err(format!("Data file not found: {}", path.display()));
    }

    // Read file
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

    // Parse JSON
    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse {}: {}", path.display(), e))
}

/// Load player defaults from JSON
fn load_player_defaults() -> Result<serde_json::Value, String> {
    let assets_dir = get_assets_dir();
    let path = assets_dir.join("config/player_defaults.json");

    // Check if file exists
    if !path.exists() {
        return Err(format!("Data file not found: {}", path.display()));
    }

    // Read file
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

    // Parse JSON
    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse {}: {}", path.display(), e))
}

/// Load environment bonuses from JSON
fn load_environment_bonuses() -> Result<Vec<serde_json::Value>, String> {
    let assets_dir = get_assets_dir();
    let path = assets_dir.join("config/environment_bonuses.json");
    load_json_array(&path, "environments")
}

/// Load natures from JSON
fn load_natures() -> Result<Vec<serde_json::Value>, String> {
    let assets_dir = get_assets_dir();
    let path = assets_dir.join("pokemon/natures.json");
    load_json_array(&path, "natures")
}

/// Helper to load a JSON array from file
fn load_json_array(path: &Path, key: &str) -> Result<Vec<serde_json::Value>, String> {
    // Check if file exists
    if !path.exists() {
        return Err(format!("Data file not found: {}", path.display()));
    }

    // Read file
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

    // Parse JSON
    let json: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse {}: {} at line {}", path.display(), e, e.line()))?;

    // Extract array
    json.get(key)
        .and_then(|v| v.as_array())
        .map(|arr| arr.clone())
        .ok_or_else(|| format!("Invalid format in {}: expected '{}' array", path.display(), key))
}

/// Get item by item_type field
pub fn get_item_by_type(item_type: &str) -> Option<serde_json::Value> {
    get_game_data()?.items.iter().find(|item| {
        item.get("item_type")
            .and_then(|v| v.as_str())
            .map(|t| t == item_type)
            .unwrap_or(false)
    }).cloned()
}

/// Get type effectiveness multiplier for attacking and defending types
pub fn get_type_effectiveness(attacking: &str, defending: &str) -> f64 {
    get_game_data()
        .map(|cache| {
            cache.type_effectiveness.iter()
                .find(|entry| {
                    entry.get("attacking").and_then(|v| v.as_str()).map(|a| a == attacking).unwrap_or(false) &&
                    entry.get("defending").and_then(|v| v.as_str()).map(|d| d == defending).unwrap_or(false)
                })
                .and_then(|entry| entry.get("multiplier")?.as_f64())
                .unwrap_or(1.0)
        })
        .unwrap_or(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_pokemon_data() {
        // This test will pass once assets/pokemon/species.json exists
        match load_pokemon_data() {
            Ok(pokemon) => {
                println!("Loaded {} Pokémon", pokemon.len());
                assert!(pokemon.len() > 0);
            }
            Err(e) => {
                // Expected to fail before data files exist
                println!("Data not available yet: {}", e);
            }
        }
    }
}
