//! Game data validator - Validates integrity of loaded game data
//!
//! Ensures:
//! - All required fields are present
//! - References between data are valid
//! - Data types are correct
//! - Values are within expected ranges

use std::collections::HashSet;
use serde_json::Value;

#[cfg(test)]
use serde_json::json;

/// Validate all loaded game data for integrity
pub fn validate_all_data(
    pokemon: &[Value],
    locations: &[Value],
    trainers: &[Value],
) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    // Validate individual datasets
    if let Err(e) = validate_pokemon_data(pokemon) {
        errors.extend(e);
    }

    if let Err(e) = validate_location_data(locations, pokemon) {
        errors.extend(e);
    }

    if let Err(e) = validate_trainer_data(trainers, pokemon, locations) {
        errors.extend(e);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validate items data
pub fn validate_items_data(items: &[Value]) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    for (idx, item) in items.iter().enumerate() {
        // Check required fields
        if !item.get("id").and_then(|v| v.as_u64()).is_some() {
            errors.push(format!("Item at index {}: missing or invalid 'id'", idx));
        }

        if !item.get("name").and_then(|v| v.as_str()).is_some() {
            errors.push(format!("Item at index {}: missing or invalid 'name'", idx));
        }

        if !item.get("item_type").and_then(|v| v.as_str()).is_some() {
            errors.push(format!("Item at index {}: missing or invalid 'item_type'", idx));
        }

        // Validate price is non-negative
        if let Some(price) = item.get("price").and_then(|v| v.as_u64()) {
            if price == 0 {
                errors.push(format!("Item at index {}: price cannot be 0", idx));
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validate type effectiveness chart
pub fn validate_type_effectiveness(types: &[Value]) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    let valid_multipliers = vec![0.0, 0.5, 1.0, 2.0];

    for (idx, entry) in types.iter().enumerate() {
        // Check required fields
        if !entry.get("attacking").and_then(|v| v.as_str()).is_some() {
            errors.push(format!(
                "Type effectiveness at index {}: missing or invalid 'attacking'",
                idx
            ));
        }

        if !entry.get("defending").and_then(|v| v.as_str()).is_some() {
            errors.push(format!(
                "Type effectiveness at index {}: missing or invalid 'defending'",
                idx
            ));
        }

        // Validate multiplier
        if let Some(mult) = entry.get("multiplier").and_then(|v| v.as_f64()) {
            if !valid_multipliers.contains(&mult) {
                errors.push(format!(
                    "Type effectiveness at index {}: invalid multiplier {}",
                    idx, mult
                ));
            }
        } else {
            errors.push(format!(
                "Type effectiveness at index {}: missing or invalid 'multiplier'",
                idx
            ));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validate environment bonuses
pub fn validate_environment_bonuses(envs: &[Value]) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    for (idx, env) in envs.iter().enumerate() {
        // Check required fields
        if !env.get("type").and_then(|v| v.as_str()).is_some() {
            errors.push(format!(
                "Environment at index {}: missing or invalid 'type'",
                idx
            ));
        }

        // Validate stat bonuses are in valid range (1.0 - 1.2)
        if let Some(bonuses) = env.get("stat_bonuses").and_then(|v| v.as_object()) {
            for (stat, bonus) in bonuses {
                if let Some(multiplier) = bonus.as_f64() {
                    if multiplier < 1.0 || multiplier > 1.2 {
                        errors.push(format!(
                            "Environment at index {}: {} bonus {} outside valid range [1.0, 1.2]",
                            idx, stat, multiplier
                        ));
                    }
                }
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validate natures data
pub fn validate_natures(natures: &[Value]) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    let valid_modifiers = vec![0.9, 1.1];

    for (idx, nature) in natures.iter().enumerate() {
        // Check required fields
        if !nature.get("id").and_then(|v| v.as_str()).is_some() {
            errors.push(format!("Nature at index {}: missing or invalid 'id'", idx));
        }

        if !nature.get("name_en").and_then(|v| v.as_str()).is_some() {
            errors.push(format!(
                "Nature at index {}: missing or invalid 'name_en'",
                idx
            ));
        }

        // Validate stat modifiers are 0.9 or 1.1
        if let Some(modifiers) = nature.get("stat_modifiers").and_then(|v| v.as_object()) {
            for (_stat, modifier) in modifiers {
                if let Some(mult) = modifier.as_f64() {
                    if !valid_modifiers.contains(&mult) {
                        errors.push(format!(
                            "Nature at index {}: invalid stat modifier {}",
                            idx, mult
                        ));
                    }
                }
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validate Pokémon species data
fn validate_pokemon_data(pokemon: &[Value]) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    let mut seen_ids = HashSet::new();

    for (idx, poke) in pokemon.iter().enumerate() {
        // Check required fields
        if !poke.get("id").and_then(|v| v.as_u64()).is_some() {
            errors.push(format!("Pokémon at index {}: missing or invalid 'id'", idx));
        }

        if !poke.get("name").and_then(|v| v.as_str()).is_some() {
            errors.push(format!("Pokémon at index {}: missing or invalid 'name'", idx));
        }

        if !poke.get("primary_type").and_then(|v| v.as_str()).is_some() {
            errors.push(format!("Pokémon at index {}: missing or invalid 'primary_type'", idx));
        }

        if !poke.get("base_stats").is_some() {
            errors.push(format!("Pokémon at index {}: missing 'base_stats'", idx));
        }

        // Check for duplicate IDs
        if let Some(id) = poke.get("id").and_then(|v| v.as_u64()) {
            if seen_ids.contains(&id) {
                errors.push(format!("Duplicate Pokémon ID: {}", id));
            }
            seen_ids.insert(id);
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validate location data and connections
fn validate_location_data(locations: &[Value], pokemon: &[Value]) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    let mut location_ids = HashSet::new();
    let pokemon_ids: HashSet<u64> = pokemon
        .iter()
        .filter_map(|p| p.get("id").and_then(|v| v.as_u64()))
        .collect();

    // First pass: collect all location IDs
    for location in locations {
        if let Some(id) = location.get("id").and_then(|v| v.as_u64()) {
            location_ids.insert(id);
        }
    }

    // Second pass: validate references
    for (idx, location) in locations.iter().enumerate() {
        // Check required fields
        if !location.get("id").and_then(|v| v.as_u64()).is_some() {
            errors.push(format!("Location at index {}: missing or invalid 'id'", idx));
        }

        if !location.get("name").and_then(|v| v.as_str()).is_some() {
            errors.push(format!("Location at index {}: missing or invalid 'name'", idx));
        }

        // Check connections reference valid locations
        if let Some(connections) = location.get("connections").and_then(|v| v.as_array()) {
            for conn_id in connections {
                if let Some(id) = conn_id.as_u64() {
                    if !location_ids.contains(&id) && id != 0 {
                        errors.push(format!(
                            "Location {}: invalid connection to non-existent location {}",
                            location.get("id").and_then(|v| v.as_u64()).unwrap_or(0),
                            id
                        ));
                    }
                }
            }
        }

        // Check wild Pokémon reference valid species
        if let Some(wild_pool) = location.get("wild_pokemon").and_then(|v| v.as_array()) {
            for spawn in wild_pool {
                if let Some(poke_id) = spawn.get("pokemon_id").and_then(|v| v.as_u64()) {
                    if !pokemon_ids.contains(&poke_id) {
                        errors.push(format!(
                            "Location {}: references non-existent Pokémon #{}",
                            location.get("id").and_then(|v| v.as_u64()).unwrap_or(0),
                            poke_id
                        ));
                    }
                }
            }
        }

        // Check NPCs reference valid trainer IDs
        if let Some(npcs) = location.get("npcs").and_then(|v| v.as_array()) {
            for npc_id in npcs {
                if let Some(id) = npc_id.as_u64() {
                    if id == 0 {
                        errors.push(format!(
                            "Location {}: invalid NPC ID 0",
                            location.get("id").and_then(|v| v.as_u64()).unwrap_or(0)
                        ));
                    }
                }
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validate trainer data
fn validate_trainer_data(
    trainers: &[Value],
    pokemon: &[Value],
    locations: &[Value],
) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    let pokemon_ids: HashSet<u64> = pokemon
        .iter()
        .filter_map(|p| p.get("id").and_then(|v| v.as_u64()))
        .collect();

    let location_ids: HashSet<u64> = locations
        .iter()
        .filter_map(|l| l.get("id").and_then(|v| v.as_u64()))
        .collect();

    for (idx, trainer) in trainers.iter().enumerate() {
        // Check required fields
        if !trainer.get("id").and_then(|v| v.as_u64()).is_some() {
            errors.push(format!("Trainer at index {}: missing or invalid 'id'", idx));
        }

        if !trainer.get("name").and_then(|v| v.as_str()).is_some() {
            errors.push(format!("Trainer at index {}: missing or invalid 'name'", idx));
        }

        // Check location reference
        if let Some(loc_id) = trainer.get("location_id").and_then(|v| v.as_u64()) {
            if loc_id > 0 && !location_ids.contains(&loc_id) {
                errors.push(format!(
                    "Trainer {}: references non-existent location #{}",
                    trainer.get("id").and_then(|v| v.as_u64()).unwrap_or(0),
                    loc_id
                ));
            }
        }

        // Check team Pokémon
        if let Some(team) = trainer.get("team").and_then(|v| v.as_array()) {
            for (team_idx, member) in team.iter().enumerate() {
                if let Some(poke_id) = member.get("pokemon_id").and_then(|v| v.as_u64()) {
                    if !pokemon_ids.contains(&poke_id) {
                        errors.push(format!(
                            "Trainer {}: team member {} has invalid Pokémon ID {}",
                            trainer.get("id").and_then(|v| v.as_u64()).unwrap_or(0),
                            team_idx,
                            poke_id
                        ));
                    }
                }

                if !member.get("level").and_then(|v| v.as_u64()).is_some() {
                    errors.push(format!(
                        "Trainer {}: team member {} missing 'level'",
                        trainer.get("id").and_then(|v| v.as_u64()).unwrap_or(0),
                        team_idx
                    ));
                }
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_pokemon_data_valid() {
        let pokemon = vec![
            json!({
                "id": 1,
                "name": "Test",
                "primary_type": "Normal",
                "base_stats": {}
            }),
        ];

        assert!(validate_pokemon_data(&pokemon).is_ok());
    }

    #[test]
    fn test_validate_pokemon_data_missing_id() {
        let pokemon = vec![json!({
            "name": "Test",
            "primary_type": "Normal",
            "base_stats": {}
        })];

        assert!(validate_pokemon_data(&pokemon).is_err());
    }
}
