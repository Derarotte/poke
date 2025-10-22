use crate::game::{Location, EnvironmentType, LocationRequirement, WildPokemonSpawn};

/// 创建所有游戏地点
pub fn create_all_locations() -> Vec<Location> {
    vec![
        create_pallet_town(),
        create_viridian_forest(),
        create_pewter_city(),
        create_mt_moon(),
        create_cerulean_city(),
        create_vermilion_city(),
        create_lavender_town(),
        create_celadon_city(),
        create_cinnabar_island(),
        create_pokemon_league(),
    ]
}

/// 常青小镇 - 游戏起始地点
fn create_pallet_town() -> Location {
    let mut loc = Location::new(
        101,
        "常青小镇".to_string(),
        "宁静祥和的小镇，你的冒险从这里开始".to_string(),
        EnvironmentType::Grassland,
    );
    loc.is_starting_location = true;
    loc.encounter_rate = 0.3;
    loc.add_connection(102); // 常青森林

    // 野生宝可梦池
    loc.add_wild_pokemon(WildPokemonSpawn {
        pokemon_id: 25, // 皮卡丘
        spawn_rate: 50.0,
        level_min: 2,
        level_max: 4,
    });
    loc.add_wild_pokemon(WildPokemonSpawn {
        pokemon_id: 19, // 小鸟
        spawn_rate: 50.0,
        level_min: 2,
        level_max: 4,
    });

    loc.unlock_requirement = LocationRequirement::default();
    loc.npc_trainers.push(1); // 青绿
    loc
}

/// 常青森林
fn create_viridian_forest() -> Location {
    let mut loc = Location::new(
        102,
        "常青森林".to_string(),
        "茂密的森林，小宝可梦的家园".to_string(),
        EnvironmentType::Forest,
    );
    loc.encounter_rate = 0.8;
    loc.add_connection(101); // 常青小镇
    loc.add_connection(103); // 华石镇

    // 野生宝可梦池
    loc.add_wild_pokemon(WildPokemonSpawn {
        pokemon_id: 1,  // 妙蛙种子
        spawn_rate: 40.0,
        level_min: 2,
        level_max: 5,
    });
    loc.add_wild_pokemon(WildPokemonSpawn {
        pokemon_id: 16, // 小虫
        spawn_rate: 40.0,
        level_min: 3,
        level_max: 6,
    });
    loc.add_wild_pokemon(WildPokemonSpawn {
        pokemon_id: 21, // 烈雀
        spawn_rate: 20.0,
        level_min: 3,
        level_max: 5,
    });

    loc.unlock_requirement = LocationRequirement {
        required_level: 3,
        required_badges: vec![],
        required_pokemon_count: None,
    };
    loc
}

/// 华石镇
fn create_pewter_city() -> Location {
    let mut loc = Location::new(
        103,
        "华石镇".to_string(),
        "靠近山脉的小镇，有许多稀有宝可梦".to_string(),
        EnvironmentType::Mountain,
    );
    loc.encounter_rate = 0.6;
    loc.add_connection(102); // 常青森林
    loc.add_connection(104); // 月见山

    loc.add_wild_pokemon(WildPokemonSpawn {
        pokemon_id: 27, // 小拳石
        spawn_rate: 50.0,
        level_min: 5,
        level_max: 8,
    });
    loc.add_wild_pokemon(WildPokemonSpawn {
        pokemon_id: 29, // 地鼠
        spawn_rate: 50.0,
        level_min: 5,
        level_max: 8,
    });

    loc.unlock_requirement = LocationRequirement {
        required_level: 5,
        required_badges: vec![],
        required_pokemon_count: None,
    };
    loc
}

/// 月见山
fn create_mt_moon() -> Location {
    let mut loc = Location::new(
        104,
        "月见山".to_string(),
        "高耸的山脉，环境恶劣但资源丰富".to_string(),
        EnvironmentType::Cave,
    );
    loc.encounter_rate = 0.7;
    loc.add_connection(103); // 华石镇
    loc.add_connection(105); // 华蓝市

    loc.add_wild_pokemon(WildPokemonSpawn {
        pokemon_id: 27, // 小拳石
        spawn_rate: 40.0,
        level_min: 7,
        level_max: 10,
    });
    loc.add_wild_pokemon(WildPokemonSpawn {
        pokemon_id: 41, // 超音蝠
        spawn_rate: 40.0,
        level_min: 7,
        level_max: 10,
    });
    loc.add_wild_pokemon(WildPokemonSpawn {
        pokemon_id: 74, // 小岩石
        spawn_rate: 20.0,
        level_min: 8,
        level_max: 11,
    });

    loc.unlock_requirement = LocationRequirement {
        required_level: 7,
        required_badges: vec![],
        required_pokemon_count: None,
    };
    loc.npc_trainers.push(2); // 小刚馆主
    loc
}

/// 华蓝市
fn create_cerulean_city() -> Location {
    let mut loc = Location::new(
        105,
        "华蓝市".to_string(),
        "靠近水边的美丽城市，有许多水系宝可梦".to_string(),
        EnvironmentType::Water,
    );
    loc.encounter_rate = 0.5;
    loc.add_connection(104); // 月见山
    loc.add_connection(106); // 海滨镇
    loc.add_connection(107); // 金黄市

    loc.add_wild_pokemon(WildPokemonSpawn {
        pokemon_id: 54, // 可达鸭
        spawn_rate: 50.0,
        level_min: 15,
        level_max: 18,
    });
    loc.add_wild_pokemon(WildPokemonSpawn {
        pokemon_id: 60, // 蚊香蛙
        spawn_rate: 50.0,
        level_min: 15,
        level_max: 18,
    });

    loc.unlock_requirement = LocationRequirement {
        required_level: 15,
        required_badges: vec![1],
        required_pokemon_count: None,
    };
    loc.npc_trainers.push(3); // 美娜馆主
    loc
}

/// 海滨镇
fn create_vermilion_city() -> Location {
    let mut loc = Location::new(
        106,
        "海滨镇".to_string(),
        "繁忙的港口城市，有许多商人".to_string(),
        EnvironmentType::City,
    );
    loc.encounter_rate = 0.3;
    loc.add_connection(105); // 华蓝市
    loc.add_connection(108); // 紫苑镇

    loc.add_wild_pokemon(WildPokemonSpawn {
        pokemon_id: 152, // 菊草叶
        spawn_rate: 100.0,
        level_min: 18,
        level_max: 22,
    });

    loc.unlock_requirement = LocationRequirement {
        required_level: 16,
        required_badges: vec![1],
        required_pokemon_count: None,
    };
    loc.npc_trainers.push(4); // 波鬼馆主
    loc
}

/// 金黄市
fn create_celadon_city() -> Location {
    let mut loc = Location::new(
        107,
        "金黄市".to_string(),
        "繁华的大都市，馆主是职业选手".to_string(),
        EnvironmentType::City,
    );
    loc.encounter_rate = 0.2;
    loc.add_connection(105); // 华蓝市
    loc.add_connection(109); // 常磐市

    loc.add_wild_pokemon(WildPokemonSpawn {
        pokemon_id: 100, // 鬼斯
        spawn_rate: 50.0,
        level_min: 20,
        level_max: 24,
    });
    loc.add_wild_pokemon(WildPokemonSpawn {
        pokemon_id: 116, // 蛋蛋
        spawn_rate: 50.0,
        level_min: 20,
        level_max: 24,
    });

    loc.unlock_requirement = LocationRequirement {
        required_level: 20,
        required_badges: vec![1, 2],
        required_pokemon_count: None,
    };
    loc.npc_trainers.push(5); // 蕾欧娜馆主
    loc
}

/// 紫苑镇
fn create_lavender_town() -> Location {
    let mut loc = Location::new(
        108,
        "紫苑镇".to_string(),
        "古老而神秘的城镇，传说这里有超古代遗迹".to_string(),
        EnvironmentType::Cave,
    );
    loc.encounter_rate = 0.6;
    loc.add_connection(106); // 海滨镇
    loc.add_connection(109); // 常磐市

    loc.add_wild_pokemon(WildPokemonSpawn {
        pokemon_id: 92, // 鬼斯通
        spawn_rate: 50.0,
        level_min: 22,
        level_max: 26,
    });
    loc.add_wild_pokemon(WildPokemonSpawn {
        pokemon_id: 138, // 贝壳兽
        spawn_rate: 50.0,
        level_min: 23,
        level_max: 27,
    });

    loc.unlock_requirement = LocationRequirement {
        required_level: 21,
        required_badges: vec![1, 2],
        required_pokemon_count: None,
    };
    loc.npc_trainers.push(6); // 娜姿馆主
    loc
}

/// 常磐市
fn create_cinnabar_island() -> Location {
    let mut loc = Location::new(
        109,
        "常磐市".to_string(),
        "最后的挑战地点，黑暗势力的总部".to_string(),
        EnvironmentType::Mountain,
    );
    loc.encounter_rate = 0.4;
    loc.add_connection(107); // 金黄市
    loc.add_connection(108); // 紫苑镇
    loc.add_connection(110); // 红莲岛

    loc.add_wild_pokemon(WildPokemonSpawn {
        pokemon_id: 147, // 迷你龙
        spawn_rate: 100.0,
        level_min: 30,
        level_max: 35,
    });

    loc.unlock_requirement = LocationRequirement {
        required_level: 35,
        required_badges: vec![1, 2, 3],
        required_pokemon_count: None,
    };
    loc.npc_trainers.push(7); // 坂木
    loc.npc_trainers.push(8); // 劲敌
    loc
}

/// 红莲岛
fn create_pokemon_league() -> Location {
    let mut loc = Location::new(
        110,
        "宝可梦联盟".to_string(),
        "强者的殿堂，最终的冠军挑战".to_string(),
        EnvironmentType::City,
    );
    loc.encounter_rate = 0.0;
    loc.add_connection(109); // 常磐市

    loc.unlock_requirement = LocationRequirement {
        required_level: 45,
        required_badges: vec![1, 2, 3, 4],
        required_pokemon_count: Some(6),
    };
    loc.npc_trainers.push(9); // 冠军
    loc
}

/// 通过ID获取地点
pub fn get_location_by_id(location_id: u32) -> Option<Location> {
    create_all_locations()
        .into_iter()
        .find(|loc| loc.id == location_id)
}

/// 获取所有地点
pub fn get_all_locations() -> Vec<Location> {
    create_all_locations()
}
