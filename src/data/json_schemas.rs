//! JSON data schema definitions - Structures for deserializing game data from JSON
//!
//! These structures map to the JSON file formats and can be converted to game structures

use serde::{Deserialize, Serialize};

/// Pokémon species data from JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonSpeciesJSON {
    pub id: u32,
    pub name: String,
    pub english_name: Option<String>,
    pub primary_type: String,
    pub secondary_type: Option<String>,
    pub base_stats: BaseStatsJSON,
    pub catch_rate: u32,
    pub experience_yield: u32,
    pub evolution: Option<EvolutionJSON>,
    #[serde(default)]
    pub initial_moves: Option<Vec<u32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseStatsJSON {
    pub hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub sp_attack: u32,
    pub sp_defense: u32,
    pub speed: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionJSON {
    pub method: String,
    pub trigger: u32,
    pub to: u32,
}

/// Move data from JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveJSON {
    pub id: u32,
    pub name: String,
    pub english_name: Option<String>,
    pub move_type: String,
    #[serde(rename = "type")]
    pub type_field: Option<String>, // Alternative name
    pub category: String,
    pub power: Option<u32>,
    pub accuracy: Option<u32>,
    pub pp: u32,
    pub effect: Option<String>,
}

impl MoveJSON {
    /// Get the move type, preferring move_type over type field
    pub fn get_type(&self) -> String {
        self.move_type.clone()
    }
}

/// Location data from JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationJSON {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub environment: String,
    pub encounter_rate: f32,
    pub is_starting_location: Option<bool>,
    pub connections: Vec<u32>,
    pub wild_pokemon: Vec<WildPokemonSpawnJSON>,
    pub npcs: Option<Vec<u32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WildPokemonSpawnJSON {
    pub pokemon_id: u32,
    pub spawn_rate: f32,
    pub level_min: u32,
    pub level_max: u32,
}

/// NPC Trainer data from JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainerJSON {
    pub id: u32,
    pub name: String,
    pub title: Option<String>,
    pub location_id: Option<u32>,
    pub team: Vec<TrainerPokemonJSON>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainerPokemonJSON {
    pub pokemon_id: u32,
    pub level: u32,
    pub item: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pokemon_species_json_structure() {
        // Verify structure can be created
        let stats = BaseStatsJSON {
            hp: 45,
            attack: 49,
            defense: 49,
            sp_attack: 65,
            sp_defense: 65,
            speed: 45,
        };

        let pokemon = PokemonSpeciesJSON {
            id: 1,
            name: "妙蛙种子".to_string(),
            english_name: Some("Bulbasaur".to_string()),
            primary_type: "Grass".to_string(),
            secondary_type: Some("Poison".to_string()),
            base_stats: stats,
            catch_rate: 45,
            experience_yield: 64,
            evolution: None,
        };

        assert_eq!(pokemon.id, 1);
        assert_eq!(pokemon.name, "妙蛙种子");
    }

    #[test]
    fn test_move_json_get_type() {
        let move_data = MoveJSON {
            id: 1,
            name: "念力".to_string(),
            english_name: Some("Psychic".to_string()),
            move_type: "Psychic".to_string(),
            type_field: None,
            category: "Special".to_string(),
            power: Some(50),
            accuracy: Some(100),
            pp: 25,
            effect: None,
        };

        assert_eq!(move_data.get_type(), "Psychic");
    }
}
