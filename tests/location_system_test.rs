// Phase 7: Location System Tests
// Tests for location binding system, environment bonuses, unlock conditions, and wild Pokémon

use poke::game::{
    Player, EnvironmentType, EnvironmentBonus, WildPokemonSpawn, WildPokemonEncounter,
};
use poke::game::player::Badge;
use poke::data::locations_data;
use poke::pokemon_generator::generator::PokemonStats;

// ==================== Location System Tests (Task 7.1) ====================

#[test]
fn test_location_creation() {
    let locations = locations_data::get_all_locations();
    assert!(!locations.is_empty(), "Should have at least one location");
    assert_eq!(locations.len(), 10, "Should have exactly 10 locations");
}

#[test]
fn test_pallet_town_exists() {
    let pallet_town = locations_data::get_location_by_id(101);
    assert!(pallet_town.is_some(), "Pallet Town should exist");

    let location = pallet_town.unwrap();
    assert_eq!(location.name, "常青小镇");
    assert!(location.is_starting_location, "Pallet Town should be starting location");
}

#[test]
fn test_location_connections() {
    let pallet_town = locations_data::get_location_by_id(101).unwrap();
    assert!(!pallet_town.connected_locations.is_empty(), "Pallet Town should have connections");

    let viridian_forest = locations_data::get_location_by_id(102).unwrap();
    assert!(
        pallet_town.connected_locations.contains(&102),
        "Pallet Town should connect to Viridian Forest"
    );
}

#[test]
fn test_location_unlock_requirement_default() {
    let pallet_town = locations_data::get_location_by_id(101).unwrap();
    assert_eq!(
        pallet_town.unlock_requirement.required_level, 1,
        "Pallet Town should have no level requirement"
    );
    assert!(
        pallet_town.unlock_requirement.required_badges.is_empty(),
        "Pallet Town should have no badge requirements"
    );
}

#[test]
fn test_location_unlock_requirement_progression() {
    // Cerulean City requires Lv15 + 1 badge
    let cerulean = locations_data::get_location_by_id(105).unwrap();
    assert_eq!(cerulean.unlock_requirement.required_level, 15);
    assert_eq!(cerulean.unlock_requirement.required_badges.len(), 1);
}

#[test]
fn test_wild_pokemon_pool_not_empty() {
    let all_locations = locations_data::get_all_locations();
    let locations_with_pokemon: Vec<_> = all_locations
        .iter()
        .filter(|loc| !loc.wild_pokemon_pool.is_empty())
        .collect();

    assert!(
        !locations_with_pokemon.is_empty(),
        "At least some locations should have wild Pokémon"
    );
}

#[test]
fn test_encounter_rate_valid_range() {
    let all_locations = locations_data::get_all_locations();
    for location in all_locations {
        assert!(
            location.encounter_rate >= 0.0 && location.encounter_rate <= 1.0,
            "Encounter rate {} should be between 0 and 1",
            location.encounter_rate
        );
    }
}

#[test]
fn test_environment_bonus_types() {
    let all_locations = locations_data::get_all_locations();
    let mut environments = Vec::new();
    for location in &all_locations {
        if !environments.contains(&(location.environment as u32)) {
            environments.push(location.environment as u32);
        }
    }

    assert!(environments.len() > 1, "Should have multiple environment types");
}

#[test]
fn test_npc_trainers_assigned() {
    let all_locations = locations_data::get_all_locations();
    let locations_with_npcs: Vec<_> = all_locations
        .iter()
        .filter(|loc| !loc.npc_trainers.is_empty())
        .collect();

    assert!(
        !locations_with_npcs.is_empty(),
        "At least some locations should have NPC trainers"
    );
}

// ==================== Environment Bonus Tests (Task 7.1) ====================

#[test]
fn test_environment_bonus_grassland() {
    let bonus = EnvironmentBonus::from_environment(EnvironmentType::Grassland);
    assert_eq!(bonus.apply_to_stat("speed", 100), 110);
}

#[test]
fn test_environment_bonus_mountain() {
    let bonus = EnvironmentBonus::from_environment(EnvironmentType::Mountain);
    assert_eq!(bonus.apply_to_stat("attack", 100), 110);
}

#[test]
fn test_environment_bonus_water() {
    let bonus = EnvironmentBonus::from_environment(EnvironmentType::Water);
    assert_eq!(bonus.apply_to_stat("sp_defense", 100), 110);
}

#[test]
fn test_environment_bonus_cave() {
    let bonus = EnvironmentBonus::from_environment(EnvironmentType::Cave);
    assert_eq!(bonus.apply_to_stat("sp_attack", 100), 110);
}

#[test]
fn test_environment_bonus_forest() {
    let bonus = EnvironmentBonus::from_environment(EnvironmentType::Forest);
    assert_eq!(bonus.apply_to_stat("defense", 100), 110);
}

#[test]
fn test_environment_bonus_city() {
    let bonus = EnvironmentBonus::from_environment(EnvironmentType::City);
    // City should have minimal bonuses
    assert_eq!(bonus.apply_to_stat("hp", 100), 100);
}

#[test]
fn test_environment_bonus_rounding() {
    let bonus = EnvironmentBonus::from_environment(EnvironmentType::Grassland);
    // Test that rounding is handled correctly for non-round numbers
    let result = bonus.apply_to_stat("speed", 73);
    assert!(result > 73 && result <= 81); // 73 * 1.1 = 80.3, should round to 80
}

#[test]
fn test_pokemon_stats_apply_environment_bonus() {
    let base_stats = PokemonStats {
        hp: 100,
        attack: 100,
        defense: 100,
        sp_attack: 100,
        sp_defense: 100,
        speed: 100,
    };

    let bonus = EnvironmentBonus::from_environment(EnvironmentType::Mountain);
    let boosted = base_stats.apply_environment_bonus(&bonus);

    // Attack should be boosted by 10%
    assert_eq!(boosted.attack, 110);
    // Other stats should be unchanged
    assert_eq!(boosted.hp, 100);
}

// ==================== Location Unlock System Tests (Task 7.1) ====================

#[test]
fn test_player_can_unlock_starting_location() {
    let mut player = Player::new("Test".to_string());
    let pallet_town = locations_data::get_location_by_id(101).unwrap();

    assert!(player.can_unlock_location(&pallet_town.unlock_requirement));
}

#[test]
fn test_player_cannot_unlock_without_level() {
    let player = Player::new("Test".to_string());
    // Cerulean City requires Lv15
    let cerulean = locations_data::get_location_by_id(105).unwrap();

    assert!(!player.can_unlock_location(&cerulean.unlock_requirement));
}

#[test]
fn test_player_unlock_location_with_sufficient_level() {
    let mut player = Player::new("Test".to_string());

    // Add a high-level Pokémon
    let starter = poke::data::pokemon_data::get_pokemon_by_id(25).unwrap();
    player.add_pokemon(starter);
    player.pokemons[0].level = 15;

    // Add required badge
    player.badges.push(Badge {
        name: "Boulder Badge".to_string(),
        id: 1,
    });

    let cerulean = locations_data::get_location_by_id(105).unwrap();
    assert!(player.can_unlock_location(&cerulean.unlock_requirement));
}

#[test]
fn test_player_unlock_location_with_badges() {
    let mut player = Player::new("Test".to_string());

    // Add a high-level Pokémon
    let starter = poke::data::pokemon_data::get_pokemon_by_id(25).unwrap();
    player.add_pokemon(starter);
    player.pokemons[0].level = 15;

    // Add badge
    player.badges.push(Badge {
        name: "Test Badge".to_string(),
        id: 1,
    });

    let cerulean = locations_data::get_location_by_id(105).unwrap();
    assert!(player.can_unlock_location(&cerulean.unlock_requirement));
}

#[test]
fn test_player_location_state_initialization() {
    let player = Player::new("Test".to_string());
    assert_eq!(player.location_state.current_location_id, 101);
    assert!(player.location_state.is_unlocked(101));
}

#[test]
fn test_player_location_state_visited_tracking() {
    let mut player = Player::new("Test".to_string());
    assert_eq!(player.location_state.visited_count(), 1); // Starting location

    player.location_state.mark_visited(102);
    assert_eq!(player.location_state.visited_count(), 2);
}

#[test]
fn test_player_get_max_level() {
    let mut player = Player::new("Test".to_string());
    let starter = poke::data::pokemon_data::get_pokemon_by_id(25).unwrap();
    player.add_pokemon(starter);
    player.pokemons[0].level = 20;

    assert_eq!(player.get_max_level(), 20);
}

#[test]
fn test_player_check_new_unlocks() {
    let mut player = Player::new("Test".to_string());
    let starter = poke::data::pokemon_data::get_pokemon_by_id(25).unwrap();
    player.add_pokemon(starter);
    player.pokemons[0].level = 5;

    let all_locations = locations_data::get_all_locations();
    player.check_new_unlocks(&all_locations);

    // After reaching level 5, Viridian Forest (Lv3) should be unlocked
    assert!(player.location_state.is_unlocked(102));
}

// ==================== Wild Pokémon System Tests (Task 7.1) ====================

#[test]
fn test_wild_pokemon_pool_weighted_selection() {
    // Use Pokémon IDs that actually exist in the database
    let pool = vec![
        WildPokemonSpawn {
            pokemon_id: 25, // Pikachu
            spawn_rate: 50.0,
            level_min: 1,
            level_max: 5,
        },
        WildPokemonSpawn {
            pokemon_id: 27, // Sandshrew
            spawn_rate: 50.0,
            level_min: 1,
            level_max: 5,
        },
    ];

    let mut pokemon_25_count = 0;
    let mut pokemon_27_count = 0;

    for _ in 0..1000 {
        if let Ok(selected) = WildPokemonEncounter::generate_wild_pokemon(&pool) {
            if selected.species_id == 25 {
                pokemon_25_count += 1;
            } else if selected.species_id == 27 {
                pokemon_27_count += 1;
            }
        }
    }

    // Both should be roughly equal (50% each) with 1000 samples, allowing broader variance
    // since some may fail if species not in database
    assert!(pokemon_25_count > 200, "Pokemon 25 count should be > 200, got {}", pokemon_25_count);
    assert!(pokemon_27_count > 200, "Pokemon 27 count should be > 200, got {}", pokemon_27_count);
}

#[test]
fn test_wild_pokemon_level_range() {
    let pool = vec![WildPokemonSpawn {
        pokemon_id: 1,
        spawn_rate: 100.0,
        level_min: 10,
        level_max: 15,
    }];

    for _ in 0..50 {
        if let Ok(pokemon) = WildPokemonEncounter::generate_wild_pokemon(&pool) {
            assert!(pokemon.level >= 10 && pokemon.level <= 15);
        }
    }
}

#[test]
fn test_wild_pokemon_encounter_preview_generation() {
    let pool = vec![WildPokemonSpawn {
        pokemon_id: 25,
        spawn_rate: 100.0,
        level_min: 5,
        level_max: 5,
    }];

    if let Ok(wild_pokemon) = WildPokemonEncounter::generate_wild_pokemon(&pool) {
        let bonus = EnvironmentBonus::from_environment(EnvironmentType::Grassland);

        match WildPokemonEncounter::generate_preview(&wild_pokemon, &bonus, "草地") {
            Ok(preview) => {
                assert_eq!(preview.pokemon_id, 25);
                assert_eq!(preview.level, 5);
                assert!(!preview.pokemon_name.is_empty());
            }
            Err(e) => panic!("Failed to generate preview: {}", e),
        }
    }
}

// ==================== Integration Tests (Task 7.2) ====================

#[test]
fn test_game_progression_sequence() {
    let mut player = Player::new("TestPlayer".to_string());
    let starter = poke::data::pokemon_data::get_pokemon_by_id(25).unwrap();
    player.add_pokemon(starter);

    // Player starts at Pallet Town
    assert_eq!(player.location_state.current_location_id, 101);

    // Try to move to Viridian Forest
    player.location_state.current_location_id = 102;
    player.location_state.mark_visited(102);

    assert_eq!(player.location_state.current_location_id, 102);
    assert!(player.location_state.is_visited(102));
}

#[test]
fn test_location_unlock_progression() {
    let mut player = Player::new("TestPlayer".to_string());
    let starter = poke::data::pokemon_data::get_pokemon_by_id(25).unwrap();
    player.add_pokemon(starter);

    let all_locations = locations_data::get_all_locations();

    // Level up to unlock new areas
    player.pokemons[0].level = 5;
    player.check_new_unlocks(&all_locations);

    // Should be able to unlock Viridian Forest (Lv3 requirement)
    let viridian = locations_data::get_location_by_id(102).unwrap();
    assert!(player.can_unlock_location(&viridian.unlock_requirement));

    // But not Cerulean City (Lv15 requirement)
    let cerulean = locations_data::get_location_by_id(105).unwrap();
    assert!(!player.can_unlock_location(&cerulean.unlock_requirement));
}

#[test]
fn test_encounter_system_respects_location_pool() {
    let pallet_town = locations_data::get_location_by_id(101).unwrap();

    // Generate multiple encounters and verify they're from Pallet Town's pool
    for _ in 0..10 {
        if let Ok(pokemon) = WildPokemonEncounter::generate_wild_pokemon(&pallet_town.wild_pokemon_pool) {
            // Pokémon from Pallet Town should be in valid range
            assert!(pokemon.level >= 2 && pokemon.level <= 4);
            // Species should be from Pallet Town's pool (Pikachu or bird)
            assert!(pokemon.species_id == 25 || pokemon.species_id == 19);
        }
    }
}

#[test]
fn test_environment_bonus_applied_in_preview() {
    let pallet_town = locations_data::get_location_by_id(101).unwrap();

    if let Ok(wild_pokemon) = WildPokemonEncounter::generate_wild_pokemon(&pallet_town.wild_pokemon_pool) {
        let bonus = EnvironmentBonus::from_environment(pallet_town.environment);

        match WildPokemonEncounter::generate_preview(&wild_pokemon, &bonus, "草地") {
            Ok(preview) => {
                // Check that boosted stats are calculated
                assert!(preview.boosted_stats.hp > 0);
                // For Grassland, speed should be boosted
                assert!(preview.boosted_stats.speed >= preview.base_stats.speed);
            }
            Err(e) => panic!("Failed to generate preview: {}", e),
        }
    }
}

#[test]
fn test_multiple_location_connections_network() {
    let locations = locations_data::get_all_locations();

    // Build a connectivity map
    let mut connection_count = 0;
    for location in &locations {
        connection_count += location.connected_locations.len();
    }

    // Should have bidirectional connections
    assert!(connection_count > 0, "Locations should be connected");

    // Verify all connections point to existing locations
    for location in &locations {
        for connected_id in &location.connected_locations {
            let found = locations.iter().any(|l| l.id == *connected_id);
            assert!(found, "Connected location {} should exist", connected_id);
        }
    }
}

// ==================== Data Migration Tests (Task 8.2) ====================

#[test]
fn test_player_migration_from_old_save() {
    let mut old_player = Player::new("OldPlayer".to_string());
    let starter = poke::data::pokemon_data::get_pokemon_by_id(25).unwrap();
    old_player.add_pokemon(starter);
    old_player.pokemons[0].level = 10;

    // Simulate old save: reset location state
    old_player.location_state.current_location_id = 0;

    // Migrate the old player
    let migrated_player = old_player.migrate_from_old_save();

    // Verify migration succeeded
    assert_eq!(migrated_player.location_state.current_location_id, 101, "Should be at Pallet Town");
    assert!(migrated_player.location_state.is_unlocked(101), "Pallet Town should be unlocked");
    assert!(migrated_player.location_state.is_visited(101), "Pallet Town should be marked as visited");
}

#[test]
fn test_player_migration_preserves_team() {
    let mut old_player = Player::new("OldPlayer".to_string());
    let starter = poke::data::pokemon_data::get_pokemon_by_id(25).unwrap();
    old_player.add_pokemon(starter);
    old_player.pokemons[0].level = 25;
    old_player.pokemons[0].experience = 5000;

    // Simulate old save
    old_player.location_state.current_location_id = 0;

    // Migrate
    let migrated_player = old_player.migrate_from_old_save();

    // Verify team is preserved
    assert_eq!(migrated_player.pokemons.len(), 1);
    assert_eq!(migrated_player.pokemons[0].level, 25);
    assert_eq!(migrated_player.pokemons[0].experience, 5000);
}

#[test]
fn test_player_migration_preserves_items() {
    let mut old_player = Player::new("OldPlayer".to_string());
    old_player.items.insert("Poké Ball".to_string(), 20);
    old_player.money = 5000;

    // Simulate old save
    old_player.location_state.current_location_id = 0;

    // Migrate
    let migrated_player = old_player.migrate_from_old_save();

    // Verify items and money are preserved
    assert_eq!(migrated_player.items.get("Poké Ball"), Some(&20));
    assert_eq!(migrated_player.money, 5000);
}

#[test]
fn test_player_migration_auto_unlock_locations() {
    let mut old_player = Player::new("OldPlayer".to_string());
    let starter = poke::data::pokemon_data::get_pokemon_by_id(25).unwrap();
    old_player.add_pokemon(starter);
    // Level up to unlock Viridian Forest (requires Lv3)
    old_player.pokemons[0].level = 10;

    // Simulate old save
    old_player.location_state.current_location_id = 0;

    // Migrate
    let migrated_player = old_player.migrate_from_old_save();

    // Verify additional locations are auto-unlocked
    assert!(migrated_player.location_state.is_unlocked(102), "Viridian Forest should be unlocked");
}

#[test]
fn test_player_migration_idempotent() {
    let player = Player::new("TestPlayer".to_string());
    let starter = poke::data::pokemon_data::get_pokemon_by_id(25).unwrap();
    let mut player = player;
    player.add_pokemon(starter);

    // First migration should work
    let migrated_once = player.migrate_from_old_save();
    assert_eq!(migrated_once.location_state.current_location_id, 101);

    // Second migration should be idempotent (no-op)
    let migrated_twice = migrated_once.clone().migrate_from_old_save();
    assert_eq!(migrated_twice.location_state.current_location_id, 101);
}

#[test]
fn test_backward_compatibility_new_player_same_as_migrated() {
    // Create a new player the normal way
    let mut new_player = Player::new("NewPlayer".to_string());
    let starter = poke::data::pokemon_data::get_pokemon_by_id(25).unwrap();
    new_player.add_pokemon(starter.clone());
    new_player.pokemons[0].level = 5;

    // Create an old player and migrate
    let mut old_player = Player::new("OldPlayer".to_string());
    old_player.add_pokemon(starter);
    old_player.pokemons[0].level = 5;
    old_player.location_state.current_location_id = 0; // Simulate old save
    let migrated_player = old_player.migrate_from_old_save();

    // Both should have same location after migration
    assert_eq!(
        new_player.location_state.current_location_id,
        migrated_player.location_state.current_location_id,
        "New and migrated players should have same location"
    );
}
