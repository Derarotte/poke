use crate::game::{Pokemon, PokemonType, Stat, Move, MoveType};

pub fn get_pokemon_by_id(id: u32) -> Option<Pokemon> {
    match id {
        1 => Some(create_bulbasaur()),
        2 => Some(create_ivysaur()),
        3 => Some(create_venusaur()),
        4 => Some(create_charmander()),
        5 => Some(create_charmeleon()),
        6 => Some(create_charizard()),
        7 => Some(create_squirtle()),
        8 => Some(create_wartortle()),
        9 => Some(create_blastoise()),
        10 => Some(create_caterpie()),
        25 => Some(create_pikachu()),
        39 => Some(create_jigglypuff()),
        54 => Some(create_psyduck()),
        58 => Some(create_growlithe()),
        63 => Some(create_abra()),
        129 => Some(create_magikarp()),
        _ => None,
    }
}

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

pub fn get_wild_pokemon() -> Pokemon {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let pokemon_ids = vec![1, 4, 7, 10, 25, 39, 54, 58, 63, 129];
    let id = pokemon_ids[rng.gen_range(0..pokemon_ids.len())];
    get_pokemon_by_id(id).unwrap()
}
