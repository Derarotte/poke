use crate::game::{Pokemon, PokemonType, Stat, Move, MoveType};
use crate::data::loader;

pub fn get_pokemon_by_id(id: u32) -> Option<Pokemon> {
    // Get game data from the JSON cache
    let game_data = loader::get_game_data()?;

    // Search for the Pokemon with matching ID
    let pokemon_data = game_data.pokemon.iter().find(|p| {
        p.get("id").and_then(|v| v.as_u64()).map(|v| v as u32) == Some(id)
    })?;

    // Extract fields from JSON
    let name = pokemon_data.get("name")?.as_str()?.to_string();
    let primary_type_str = pokemon_data.get("primary_type")?.as_str()?;
    let secondary_type_str = pokemon_data.get("secondary_type").and_then(|v| v.as_str());
    let catch_rate = pokemon_data.get("catch_rate")?.as_u64()? as u32;

    // Parse base stats
    let base_stats = pokemon_data.get("base_stats")?;
    let stats = Stat {
        hp: base_stats.get("hp")?.as_u64()? as u32,
        attack: base_stats.get("attack")?.as_u64()? as u32,
        defense: base_stats.get("defense")?.as_u64()? as u32,
        sp_attack: base_stats.get("sp_attack")?.as_u64()? as u32,
        sp_defense: base_stats.get("sp_defense")?.as_u64()? as u32,
        speed: base_stats.get("speed")?.as_u64()? as u32,
    };

    // Parse primary type
    let primary_type = match primary_type_str {
        "Normal" => PokemonType::Normal,
        "Fire" => PokemonType::Fire,
        "Water" => PokemonType::Water,
        "Electric" => PokemonType::Electric,
        "Grass" => PokemonType::Grass,
        "Ice" => PokemonType::Ice,
        "Fighting" => PokemonType::Fighting,
        "Poison" => PokemonType::Poison,
        "Ground" => PokemonType::Ground,
        "Flying" => PokemonType::Flying,
        "Psychic" => PokemonType::Psychic,
        "Bug" => PokemonType::Bug,
        "Rock" => PokemonType::Rock,
        "Ghost" => PokemonType::Ghost,
        "Dragon" => PokemonType::Dragon,
        "Dark" => PokemonType::Dark,
        "Steel" => PokemonType::Steel,
        "Fairy" => PokemonType::Fairy,
        _ => return None,
    };

    // Parse secondary type if present
    let secondary_type = secondary_type_str.and_then(|t| {
        match t {
            "Normal" => Some(PokemonType::Normal),
            "Fire" => Some(PokemonType::Fire),
            "Water" => Some(PokemonType::Water),
            "Electric" => Some(PokemonType::Electric),
            "Grass" => Some(PokemonType::Grass),
            "Ice" => Some(PokemonType::Ice),
            "Fighting" => Some(PokemonType::Fighting),
            "Poison" => Some(PokemonType::Poison),
            "Ground" => Some(PokemonType::Ground),
            "Flying" => Some(PokemonType::Flying),
            "Psychic" => Some(PokemonType::Psychic),
            "Bug" => Some(PokemonType::Bug),
            "Rock" => Some(PokemonType::Rock),
            "Ghost" => Some(PokemonType::Ghost),
            "Dragon" => Some(PokemonType::Dragon),
            "Dark" => Some(PokemonType::Dark),
            "Steel" => Some(PokemonType::Steel),
            "Fairy" => Some(PokemonType::Fairy),
            _ => None,
        }
    });

    // Create the Pokemon
    let mut pokemon = Pokemon::new(
        id,
        name,
        (primary_type, secondary_type),
        stats,
        catch_rate,
    );

    // === Phase 4: Load initial moves from JSON ===
    if let Some(initial_moves) = pokemon_data.get("initial_moves")
        .and_then(|v| v.as_array()) {
        for move_id_value in initial_moves {
            if let Some(move_id) = move_id_value.as_u64() {
                if let Some(move_data) = get_move_by_id(move_id as u32) {
                    pokemon.add_move(move_data);
                }
            }
        }
    }

    // Fallback mechanism: Ensure Pokemon has at least one move
    if pokemon.moves.is_empty() {
        if let Some(tackle) = get_move_by_id(1) {
            pokemon.add_move(tackle);
        } else {
            // Last resort: use hardcoded default tackle move
            pokemon.add_move(create_default_tackle());
        }
    }

    Some(pokemon)
}

/*
// OLD HARDCODED FUNCTIONS - TO BE REMOVED IN TASK 4.4
// These functions are kept temporarily for reference but are no longer used

// 妙蛙种子
fn create_bulbasaur() -> Pokemon {
    let stats = Stat {
        hp: 45,
        attack: 49,
        defense: 49,
        sp_attack: 65,
        sp_defense: 65,
        speed: 45,
    };

    let mut pokemon = Pokemon::new(
        1,
        "妙蛙种子".to_string(),
        (PokemonType::Grass, Some(PokemonType::Poison)),
        stats,
        45,
    );

    pokemon.add_move(Move {
        id: 1,
        name: "撞击".to_string(),
        move_type: MoveType::Physical,
        pokemon_type: PokemonType::Normal,
        power: 40,
        accuracy: 100,
        pp: 35,
        max_pp: 35,
    });

    pokemon.add_move(Move {
        id: 74,
        name: "叶片".to_string(),
        move_type: MoveType::Physical,
        pokemon_type: PokemonType::Grass,
        power: 55,
        accuracy: 100,
        pp: 25,
        max_pp: 25,
    });

    pokemon
}

// 妙蛙草
fn create_ivysaur() -> Pokemon {
    let stats = Stat {
        hp: 60,
        attack: 62,
        defense: 63,
        sp_attack: 80,
        sp_defense: 80,
        speed: 60,
    };

    let mut pokemon = Pokemon::new(
        2,
        "妙蛙草".to_string(),
        (PokemonType::Grass, Some(PokemonType::Poison)),
        stats,
        45,
    );

    pokemon.add_move(Move {
        id: 1,
        name: "撞击".to_string(),
        move_type: MoveType::Physical,
        pokemon_type: PokemonType::Normal,
        power: 40,
        accuracy: 100,
        pp: 35,
        max_pp: 35,
    });

    pokemon.add_move(Move {
        id: 74,
        name: "叶片".to_string(),
        move_type: MoveType::Physical,
        pokemon_type: PokemonType::Grass,
        power: 55,
        accuracy: 100,
        pp: 25,
        max_pp: 25,
    });

    pokemon
}

// 妙蛙花
fn create_venusaur() -> Pokemon {
    let stats = Stat {
        hp: 80,
        attack: 82,
        defense: 83,
        sp_attack: 100,
        sp_defense: 100,
        speed: 80,
    };

    let mut pokemon = Pokemon::new(
        3,
        "妙蛙花".to_string(),
        (PokemonType::Grass, Some(PokemonType::Poison)),
        stats,
        45,
    );

    pokemon.add_move(Move {
        id: 1,
        name: "撞击".to_string(),
        move_type: MoveType::Physical,
        pokemon_type: PokemonType::Normal,
        power: 40,
        accuracy: 100,
        pp: 35,
        max_pp: 35,
    });

    pokemon.add_move(Move {
        id: 74,
        name: "叶片".to_string(),
        move_type: MoveType::Physical,
        pokemon_type: PokemonType::Grass,
        power: 55,
        accuracy: 100,
        pp: 25,
        max_pp: 25,
    });

    pokemon
}

// 小火龙
fn create_charmander() -> Pokemon {
    let stats = Stat {
        hp: 39,
        attack: 52,
        defense: 43,
        sp_attack: 60,
        sp_defense: 50,
        speed: 65,
    };

    let mut pokemon = Pokemon::new(
        4,
        "小火龙".to_string(),
        (PokemonType::Fire, None),
        stats,
        45,
    );

    pokemon.add_move(Move {
        id: 1,
        name: "撞击".to_string(),
        move_type: MoveType::Physical,
        pokemon_type: PokemonType::Normal,
        power: 40,
        accuracy: 100,
        pp: 35,
        max_pp: 35,
    });

    pokemon.add_move(Move {
        id: 33,
        name: "火焰".to_string(),
        move_type: MoveType::Special,
        pokemon_type: PokemonType::Fire,
        power: 30,
        accuracy: 100,
        pp: 25,
        max_pp: 25,
    });

    pokemon
}

// 火恐龙
fn create_charmeleon() -> Pokemon {
    let stats = Stat {
        hp: 58,
        attack: 64,
        defense: 58,
        sp_attack: 80,
        sp_defense: 65,
        speed: 80,
    };

    let mut pokemon = Pokemon::new(
        5,
        "火恐龙".to_string(),
        (PokemonType::Fire, None),
        stats,
        45,
    );

    pokemon.add_move(Move {
        id: 1,
        name: "撞击".to_string(),
        move_type: MoveType::Physical,
        pokemon_type: PokemonType::Normal,
        power: 40,
        accuracy: 100,
        pp: 35,
        max_pp: 35,
    });

    pokemon.add_move(Move {
        id: 34,
        name: "火焰喷射".to_string(),
        move_type: MoveType::Special,
        pokemon_type: PokemonType::Fire,
        power: 90,
        accuracy: 100,
        pp: 15,
        max_pp: 15,
    });

    pokemon
}

// 喷火龙
fn create_charizard() -> Pokemon {
    let stats = Stat {
        hp: 78,
        attack: 84,
        defense: 78,
        sp_attack: 109,
        sp_defense: 85,
        speed: 100,
    };

    let mut pokemon = Pokemon::new(
        6,
        "喷火龙".to_string(),
        (PokemonType::Fire, Some(PokemonType::Flying)),
        stats,
        45,
    );

    pokemon.add_move(Move {
        id: 1,
        name: "撞击".to_string(),
        move_type: MoveType::Physical,
        pokemon_type: PokemonType::Normal,
        power: 40,
        accuracy: 100,
        pp: 35,
        max_pp: 35,
    });

    pokemon.add_move(Move {
        id: 34,
        name: "火焰喷射".to_string(),
        move_type: MoveType::Special,
        pokemon_type: PokemonType::Fire,
        power: 90,
        accuracy: 100,
        pp: 15,
        max_pp: 15,
    });

    pokemon
}

// 杰尼龟
fn create_squirtle() -> Pokemon {
    let stats = Stat {
        hp: 44,
        attack: 48,
        defense: 65,
        sp_attack: 50,
        sp_defense: 64,
        speed: 43,
    };

    let mut pokemon = Pokemon::new(
        7,
        "杰尼龟".to_string(),
        (PokemonType::Water, None),
        stats,
        45,
    );

    pokemon.add_move(Move {
        id: 1,
        name: "撞击".to_string(),
        move_type: MoveType::Physical,
        pokemon_type: PokemonType::Normal,
        power: 40,
        accuracy: 100,
        pp: 35,
        max_pp: 35,
    });

    pokemon.add_move(Move {
        id: 55,
        name: "水枪".to_string(),
        move_type: MoveType::Special,
        pokemon_type: PokemonType::Water,
        power: 40,
        accuracy: 100,
        pp: 25,
        max_pp: 25,
    });

    pokemon
}

// 卡咪龟
fn create_wartortle() -> Pokemon {
    let stats = Stat {
        hp: 59,
        attack: 63,
        defense: 80,
        sp_attack: 65,
        sp_defense: 80,
        speed: 58,
    };

    let mut pokemon = Pokemon::new(
        8,
        "卡咪龟".to_string(),
        (PokemonType::Water, None),
        stats,
        45,
    );

    pokemon.add_move(Move {
        id: 1,
        name: "撞击".to_string(),
        move_type: MoveType::Physical,
        pokemon_type: PokemonType::Normal,
        power: 40,
        accuracy: 100,
        pp: 35,
        max_pp: 35,
    });

    pokemon.add_move(Move {
        id: 55,
        name: "水枪".to_string(),
        move_type: MoveType::Special,
        pokemon_type: PokemonType::Water,
        power: 40,
        accuracy: 100,
        pp: 25,
        max_pp: 25,
    });

    pokemon
}

// 水龟
fn create_blastoise() -> Pokemon {
    let stats = Stat {
        hp: 79,
        attack: 83,
        defense: 100,
        sp_attack: 85,
        sp_defense: 105,
        speed: 78,
    };

    let mut pokemon = Pokemon::new(
        9,
        "水龟".to_string(),
        (PokemonType::Water, None),
        stats,
        45,
    );

    pokemon.add_move(Move {
        id: 1,
        name: "撞击".to_string(),
        move_type: MoveType::Physical,
        pokemon_type: PokemonType::Normal,
        power: 40,
        accuracy: 100,
        pp: 35,
        max_pp: 35,
    });

    pokemon.add_move(Move {
        id: 56,
        name: "泡沫".to_string(),
        move_type: MoveType::Special,
        pokemon_type: PokemonType::Water,
        power: 65,
        accuracy: 100,
        pp: 20,
        max_pp: 20,
    });

    pokemon
}

// 绿毛虫
fn create_caterpie() -> Pokemon {
    let stats = Stat {
        hp: 45,
        attack: 52,
        defense: 43,
        sp_attack: 60,
        sp_defense: 50,
        speed: 65,
    };

    let mut pokemon = Pokemon::new(
        10,
        "绿毛虫".to_string(),
        (PokemonType::Bug, None),
        stats,
        255,
    );

    pokemon.add_move(Move {
        id: 1,
        name: "撞击".to_string(),
        move_type: MoveType::Physical,
        pokemon_type: PokemonType::Normal,
        power: 40,
        accuracy: 100,
        pp: 35,
        max_pp: 35,
    });

    pokemon
}

// 皮卡丘
fn create_pikachu() -> Pokemon {
    let stats = Stat {
        hp: 35,
        attack: 55,
        defense: 40,
        sp_attack: 50,
        sp_defense: 50,
        speed: 90,
    };

    let mut pokemon = Pokemon::new(
        25,
        "皮卡丘".to_string(),
        (PokemonType::Electric, None),
        stats,
        190,
    );

    pokemon.add_move(Move {
        id: 1,
        name: "撞击".to_string(),
        move_type: MoveType::Physical,
        pokemon_type: PokemonType::Normal,
        power: 40,
        accuracy: 100,
        pp: 35,
        max_pp: 35,
    });

    pokemon.add_move(Move {
        id: 97,
        name: "电击".to_string(),
        move_type: MoveType::Special,
        pokemon_type: PokemonType::Electric,
        power: 40,
        accuracy: 100,
        pp: 30,
        max_pp: 30,
    });

    pokemon
}

// 胖可丁
fn create_jigglypuff() -> Pokemon {
    let stats = Stat {
        hp: 115,
        attack: 40,
        defense: 20,
        sp_attack: 45,
        sp_defense: 25,
        speed: 20,
    };

    let mut pokemon = Pokemon::new(
        39,
        "胖可丁".to_string(),
        (PokemonType::Normal, Some(PokemonType::Fairy)),
        stats,
        170,
    );

    pokemon.add_move(Move {
        id: 1,
        name: "撞击".to_string(),
        move_type: MoveType::Physical,
        pokemon_type: PokemonType::Normal,
        power: 40,
        accuracy: 100,
        pp: 35,
        max_pp: 35,
    });

    pokemon
}

// 嘟嘟
fn create_psyduck() -> Pokemon {
    let stats = Stat {
        hp: 50,
        attack: 52,
        defense: 48,
        sp_attack: 65,
        sp_defense: 50,
        speed: 55,
    };

    let mut pokemon = Pokemon::new(
        54,
        "嘟嘟".to_string(),
        (PokemonType::Water, None),
        stats,
        190,
    );

    pokemon.add_move(Move {
        id: 1,
        name: "撞击".to_string(),
        move_type: MoveType::Physical,
        pokemon_type: PokemonType::Normal,
        power: 40,
        accuracy: 100,
        pp: 35,
        max_pp: 35,
    });

    pokemon.add_move(Move {
        id: 55,
        name: "水枪".to_string(),
        move_type: MoveType::Special,
        pokemon_type: PokemonType::Water,
        power: 40,
        accuracy: 100,
        pp: 25,
        max_pp: 25,
    });

    pokemon
}

// 卡拉卡拉
fn create_growlithe() -> Pokemon {
    let stats = Stat {
        hp: 55,
        attack: 70,
        defense: 40,
        sp_attack: 70,
        sp_defense: 40,
        speed: 60,
    };

    let mut pokemon = Pokemon::new(
        58,
        "卡拉卡拉".to_string(),
        (PokemonType::Fire, None),
        stats,
        190,
    );

    pokemon.add_move(Move {
        id: 1,
        name: "撞击".to_string(),
        move_type: MoveType::Physical,
        pokemon_type: PokemonType::Normal,
        power: 40,
        accuracy: 100,
        pp: 35,
        max_pp: 35,
    });

    pokemon.add_move(Move {
        id: 33,
        name: "火焰".to_string(),
        move_type: MoveType::Special,
        pokemon_type: PokemonType::Fire,
        power: 30,
        accuracy: 100,
        pp: 25,
        max_pp: 25,
    });

    pokemon
}

// 超梦
fn create_abra() -> Pokemon {
    let stats = Stat {
        hp: 25,
        attack: 20,
        defense: 15,
        sp_attack: 105,
        sp_defense: 55,
        speed: 90,
    };

    let mut pokemon = Pokemon::new(
        63,
        "超梦".to_string(),
        (PokemonType::Psychic, None),
        stats,
        200,
    );

    pokemon.add_move(Move {
        id: 1,
        name: "撞击".to_string(),
        move_type: MoveType::Physical,
        pokemon_type: PokemonType::Normal,
        power: 40,
        accuracy: 100,
        pp: 35,
        max_pp: 35,
    });

    pokemon
}

// 鲤鱼王
fn create_magikarp() -> Pokemon {
    let stats = Stat {
        hp: 20,
        attack: 10,
        defense: 55,
        sp_attack: 15,
        sp_defense: 20,
        speed: 80,
    };

    let mut pokemon = Pokemon::new(
        129,
        "鲤鱼王".to_string(),
        (PokemonType::Water, None),
        stats,
        255,
    );

    pokemon.add_move(Move {
        id: 1,
        name: "撞击".to_string(),
        move_type: MoveType::Physical,
        pokemon_type: PokemonType::Normal,
        power: 40,
        accuracy: 100,
        pp: 35,
        max_pp: 35,
    });

    pokemon
}
*/

pub fn get_wild_pokemon() -> Pokemon {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let pokemon_ids = vec![1, 4, 7, 10, 25, 39, 54, 58, 63, 129];
    let id = pokemon_ids[rng.gen_range(0..pokemon_ids.len())];
    get_pokemon_by_id(id).unwrap()
}

// ============================================================================
// Phase 3: Move Loading Functions
// ============================================================================

/// Get a move by its ID from the game data
pub fn get_move_by_id(id: u32) -> Option<Move> {
    let game_data = loader::get_game_data()?;

    let move_data = game_data.moves.iter().find(|m| {
        m.get("id").and_then(|v| v.as_u64()).map(|v| v as u32) == Some(id)
    })?;

    parse_move_from_json(move_data)
}

/// Parse a Move from JSON data
fn parse_move_from_json(data: &serde_json::Value) -> Option<Move> {
    let id = data.get("id")?.as_u64()? as u32;
    let name = data.get("name")?.as_str()?.to_string();

    // Handle both "type" and "move_type" field names
    let type_str = data.get("type")
        .or_else(|| data.get("move_type"))
        .and_then(|v| v.as_str())?;

    let category = data.get("category")?.as_str()?;

    // Parse MoveType: Physical, Special, Status
    let move_type = match category {
        "Physical" => MoveType::Physical,
        "Special" => MoveType::Special,
        "Status" => MoveType::Status,
        _ => return None,
    };

    // Parse PokemonType
    let pokemon_type = parse_pokemon_type(type_str)?;

    // Handle optional power and accuracy
    let power = data.get("power")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;

    let accuracy = data.get("accuracy")
        .and_then(|v| v.as_u64())
        .unwrap_or(100) as u32;

    let pp = data.get("pp")?.as_u64()? as u32;

    Some(Move {
        id,
        name,
        move_type,
        pokemon_type,
        power,
        accuracy,
        pp,
        max_pp: pp,
    })
}

/// Parse a Pokemon type string to PokemonType enum
fn parse_pokemon_type(type_str: &str) -> Option<PokemonType> {
    match type_str {
        "Normal" => Some(PokemonType::Normal),
        "Fire" => Some(PokemonType::Fire),
        "Water" => Some(PokemonType::Water),
        "Grass" => Some(PokemonType::Grass),
        "Electric" => Some(PokemonType::Electric),
        "Ice" => Some(PokemonType::Ice),
        "Fighting" => Some(PokemonType::Fighting),
        "Poison" => Some(PokemonType::Poison),
        "Ground" => Some(PokemonType::Ground),
        "Flying" => Some(PokemonType::Flying),
        "Psychic" => Some(PokemonType::Psychic),
        "Bug" => Some(PokemonType::Bug),
        "Rock" => Some(PokemonType::Rock),
        "Ghost" => Some(PokemonType::Ghost),
        "Dragon" => Some(PokemonType::Dragon),
        "Dark" => Some(PokemonType::Dark),
        "Steel" => Some(PokemonType::Steel),
        "Fairy" => Some(PokemonType::Fairy),
        _ => None,
    }
}

/// Create a default "Tackle" move as a fallback
fn create_default_tackle() -> Move {
    Move {
        id: 1,
        name: "撞击".to_string(),
        move_type: MoveType::Physical,
        pokemon_type: PokemonType::Normal,
        power: 40,
        accuracy: 100,
        pp: 35,
        max_pp: 35,
    }
}
