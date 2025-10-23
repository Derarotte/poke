use crate::game::{Location, EnvironmentType, WildPokemonSpawn};
use crate::data::loader;

// /// 创建所有游戏地点
// /// TODO: Remove in Task 4.4 - replaced by JSON cache
// pub fn create_all_locations() -> Vec<Location> {
//     vec![
//         create_pallet_town(),
//         create_viridian_forest(),
//         create_pewter_city(),
//         create_mt_moon(),
//         create_cerulean_city(),
//         create_vermilion_city(),
//         create_lavender_town(),
//         create_celadon_city(),
//         create_cinnabar_island(),
//         create_pokemon_league(),
//     ]
// }

// /// 常青小镇 - 游戏起始地点
// /// TODO: Remove in Task 4.4 - replaced by JSON cache
// fn create_pallet_town() -> Location {
//     let mut loc = Location::new(
//         101,
//         "常青小镇".to_string(),
//         "宁静祥和的小镇，你的冒险从这里开始".to_string(),
//         EnvironmentType::Grassland,
//     );
//     loc.is_starting_location = true;
//     loc.encounter_rate = 0.3;
//     loc.add_connection(102); // 常青森林
//
//     // 野生宝可梦池
//     loc.add_wild_pokemon(WildPokemonSpawn {
//         pokemon_id: 25, // 皮卡丘
//         spawn_rate: 50.0,
//         level_min: 2,
//         level_max: 4,
//     });
//     loc.add_wild_pokemon(WildPokemonSpawn {
//         pokemon_id: 10, // 毛毛虫
//         spawn_rate: 50.0,
//         level_min: 2,
//         level_max: 4,
//     });
//
//     loc.unlock_requirement = LocationRequirement::default();
//     loc.npc_trainers.push(1); // 青绿
//     loc
// }

// /// 常青森林
// /// TODO: Remove in Task 4.4 - replaced by JSON cache
// fn create_viridian_forest() -> Location {
//     let mut loc = Location::new(
//         102,
//         "常青森林".to_string(),
//         "茂密的森林，小宝可梦的家园".to_string(),
//         EnvironmentType::Forest,
//     );
//     loc.encounter_rate = 0.8;
//     loc.add_connection(101); // 常青小镇
//     loc.add_connection(103); // 华石镇
//
//     // 野生宝可梦池
//     loc.add_wild_pokemon(WildPokemonSpawn {
//         pokemon_id: 1,  // 妙蛙种子
//         spawn_rate: 40.0,
//         level_min: 2,
//         level_max: 5,
//     });
//     loc.add_wild_pokemon(WildPokemonSpawn {
//         pokemon_id: 16, // 小虫
//         spawn_rate: 40.0,
//         level_min: 3,
//         level_max: 6,
//     });
//     loc.add_wild_pokemon(WildPokemonSpawn {
//         pokemon_id: 21, // 烈雀
//         spawn_rate: 20.0,
//         level_min: 3,
//         level_max: 5,
//     });
//
//     loc.unlock_requirement = LocationRequirement {
//         required_level: 3,
//         required_badges: vec![],
//         required_pokemon_count: None,
//     };
//     loc
// }

// /// 华石镇
// /// TODO: Remove in Task 4.4 - replaced by JSON cache
// fn create_pewter_city() -> Location {
//     let mut loc = Location::new(
//         103,
//         "华石镇".to_string(),
//         "靠近山脉的小镇，有许多稀有宝可梦".to_string(),
//         EnvironmentType::Mountain,
//     );
//     loc.encounter_rate = 0.6;
//     loc.add_connection(102); // 常青森林
//     loc.add_connection(104); // 月见山
//
//     loc.add_wild_pokemon(WildPokemonSpawn {
//         pokemon_id: 27, // 小拳石
//         spawn_rate: 50.0,
//         level_min: 5,
//         level_max: 8,
//     });
//     loc.add_wild_pokemon(WildPokemonSpawn {
//         pokemon_id: 29, // 地鼠
//         spawn_rate: 50.0,
//         level_min: 5,
//         level_max: 8,
//     });
//
//     loc.unlock_requirement = LocationRequirement {
//         required_level: 5,
//         required_badges: vec![],
//         required_pokemon_count: None,
//     };
//     loc
// }
//
// /// 月见山
// /// TODO: Remove in Task 4.4 - replaced by JSON cache
// fn create_mt_moon() -> Location {
//     let mut loc = Location::new(
//         104,
//         "月见山".to_string(),
//         "高耸的山脉，环境恶劣但资源丰富".to_string(),
//         EnvironmentType::Cave,
//     );
//     loc.encounter_rate = 0.7;
//     loc.add_connection(103); // 华石镇
//     loc.add_connection(105); // 华蓝市
//
//     loc.add_wild_pokemon(WildPokemonSpawn {
//         pokemon_id: 27, // 小拳石
//         spawn_rate: 40.0,
//         level_min: 7,
//         level_max: 10,
//     });
//     loc.add_wild_pokemon(WildPokemonSpawn {
//         pokemon_id: 41, // 超音蝠
//         spawn_rate: 40.0,
//         level_min: 7,
//         level_max: 10,
//     });
//     loc.add_wild_pokemon(WildPokemonSpawn {
//         pokemon_id: 74, // 小岩石
//         spawn_rate: 20.0,
//         level_min: 8,
//         level_max: 11,
//     });
//
//     loc.unlock_requirement = LocationRequirement {
//         required_level: 7,
//         required_badges: vec![],
//         required_pokemon_count: None,
//     };
//     loc.npc_trainers.push(2); // 小刚馆主
//     loc
// }
//
// /// 华蓝市
// /// TODO: Remove in Task 4.4 - replaced by JSON cache
// fn create_cerulean_city() -> Location {
//     let mut loc = Location::new(
//         105,
//         "华蓝市".to_string(),
//         "靠近水边的美丽城市，有许多水系宝可梦".to_string(),
//         EnvironmentType::Water,
//     );
//     loc.encounter_rate = 0.5;
//     loc.add_connection(104); // 月见山
//     loc.add_connection(106); // 海滨镇
//     loc.add_connection(107); // 金黄市
//
//     loc.add_wild_pokemon(WildPokemonSpawn {
//         pokemon_id: 54, // 可达鸭
//         spawn_rate: 50.0,
//         level_min: 15,
//         level_max: 18,
//     });
//     loc.add_wild_pokemon(WildPokemonSpawn {
//         pokemon_id: 60, // 蚊香蛙
//         spawn_rate: 50.0,
//         level_min: 15,
//         level_max: 18,
//     });
//
//     loc.unlock_requirement = LocationRequirement {
//         required_level: 15,
//         required_badges: vec![1],
//         required_pokemon_count: None,
//     };
//     loc.npc_trainers.push(3); // 美娜馆主
//     loc
// }
//
// /// 海滨镇
// /// TODO: Remove in Task 4.4 - replaced by JSON cache
// fn create_vermilion_city() -> Location {
//     let mut loc = Location::new(
//         106,
//         "海滨镇".to_string(),
//         "繁忙的港口城市，有许多商人".to_string(),
//         EnvironmentType::City,
//     );
//     loc.encounter_rate = 0.3;
//     loc.add_connection(105); // 华蓝市
//     loc.add_connection(108); // 紫苑镇
//
//     loc.add_wild_pokemon(WildPokemonSpawn {
//         pokemon_id: 152, // 菊草叶
//         spawn_rate: 100.0,
//         level_min: 18,
//         level_max: 22,
//     });
//
//     loc.unlock_requirement = LocationRequirement {
//         required_level: 16,
//         required_badges: vec![1],
//         required_pokemon_count: None,
//     };
//     loc.npc_trainers.push(4); // 波鬼馆主
//     loc
// }
//
// /// 金黄市
// /// TODO: Remove in Task 4.4 - replaced by JSON cache
// fn create_celadon_city() -> Location {
//     let mut loc = Location::new(
//         107,
//         "金黄市".to_string(),
//         "繁华的大都市，馆主是职业选手".to_string(),
//         EnvironmentType::City,
//     );
//     loc.encounter_rate = 0.2;
//     loc.add_connection(105); // 华蓝市
//     loc.add_connection(109); // 常磐市
//
//     loc.add_wild_pokemon(WildPokemonSpawn {
//         pokemon_id: 100, // 鬼斯
//         spawn_rate: 50.0,
//         level_min: 20,
//         level_max: 24,
//     });
//     loc.add_wild_pokemon(WildPokemonSpawn {
//         pokemon_id: 116, // 蛋蛋
//         spawn_rate: 50.0,
//         level_min: 20,
//         level_max: 24,
//     });
//
//     loc.unlock_requirement = LocationRequirement {
//         required_level: 20,
//         required_badges: vec![1, 2],
//         required_pokemon_count: None,
//     };
//     loc.npc_trainers.push(5); // 蕾欧娜馆主
//     loc
// }
//
// /// 紫苑镇
// /// TODO: Remove in Task 4.4 - replaced by JSON cache
// fn create_lavender_town() -> Location {
//     let mut loc = Location::new(
//         108,
//         "紫苑镇".to_string(),
//         "古老而神秘的城镇，传说这里有超古代遗迹".to_string(),
//         EnvironmentType::Cave,
//     );
//     loc.encounter_rate = 0.6;
//     loc.add_connection(106); // 海滨镇
//     loc.add_connection(109); // 常磐市
//
//     loc.add_wild_pokemon(WildPokemonSpawn {
//         pokemon_id: 92, // 鬼斯通
//         spawn_rate: 50.0,
//         level_min: 22,
//         level_max: 26,
//     });
//     loc.add_wild_pokemon(WildPokemonSpawn {
//         pokemon_id: 138, // 贝壳兽
//         spawn_rate: 50.0,
//         level_min: 23,
//         level_max: 27,
//     });
//
//     loc.unlock_requirement = LocationRequirement {
//         required_level: 21,
//         required_badges: vec![1, 2],
//         required_pokemon_count: None,
//     };
//     loc.npc_trainers.push(6); // 娜姿馆主
//     loc
// }
//
// /// 常磐市
// /// TODO: Remove in Task 4.4 - replaced by JSON cache
// fn create_cinnabar_island() -> Location {
//     let mut loc = Location::new(
//         109,
//         "常磐市".to_string(),
//         "最后的挑战地点，黑暗势力的总部".to_string(),
//         EnvironmentType::Mountain,
//     );
//     loc.encounter_rate = 0.4;
//     loc.add_connection(107); // 金黄市
//     loc.add_connection(108); // 紫苑镇
//     loc.add_connection(110); // 红莲岛
//
//     loc.add_wild_pokemon(WildPokemonSpawn {
//         pokemon_id: 147, // 迷你龙
//         spawn_rate: 100.0,
//         level_min: 30,
//         level_max: 35,
//     });
//
//     loc.unlock_requirement = LocationRequirement {
//         required_level: 35,
//         required_badges: vec![1, 2, 3],
//         required_pokemon_count: None,
//     };
//     loc.npc_trainers.push(7); // 坂木
//     loc.npc_trainers.push(8); // 劲敌
//     loc
// }
//
// /// 红莲岛
// /// TODO: Remove in Task 4.4 - replaced by JSON cache
// fn create_pokemon_league() -> Location {
//     let mut loc = Location::new(
//         110,
//         "宝可梦联盟".to_string(),
//         "强者的殿堂，最终的冠军挑战".to_string(),
//         EnvironmentType::City,
//     );
//     loc.encounter_rate = 0.0;
//     loc.add_connection(109); // 常磐市
//
//     loc.unlock_requirement = LocationRequirement {
//         required_level: 45,
//         required_badges: vec![1, 2, 3, 4],
//         required_pokemon_count: Some(6),
//     };
//     loc.npc_trainers.push(9); // 冠军
//     loc
// }

/// 获取所有地点
pub fn get_all_locations() -> Vec<Location> {
    match loader::get_game_data() {
        Some(game_data) => {
            game_data.locations.iter().filter_map(|loc_json| {
                let id = loc_json.get("id")?.as_u64()? as u32;
                let name = loc_json.get("name")?.as_str()?.to_string();
                let description = loc_json.get("description")?.as_str()?.to_string();
                let environment_str = loc_json.get("environment")?.as_str()?;
                let environment = match environment_str {
                    "Grassland" => EnvironmentType::Grassland,
                    "Forest" => EnvironmentType::Forest,
                    "Mountain" => EnvironmentType::Mountain,
                    "Cave" => EnvironmentType::Cave,
                    "Water" => EnvironmentType::Water,
                    "City" => EnvironmentType::City,
                    _ => EnvironmentType::Grassland,
                };

                let mut location = Location::new(id, name, description, environment);
                location.encounter_rate = loc_json.get("encounter_rate")?.as_f64()? as f32;
                location.is_starting_location = loc_json.get("is_starting_location")?.as_bool()?;

                // Add connections
                if let Some(connections) = loc_json.get("connections")?.as_array() {
                    for conn_id in connections {
                        if let Some(id) = conn_id.as_u64() {
                            location.add_connection(id as u32);
                        }
                    }
                }

                // Add wild pokemon
                if let Some(wild_pokemon) = loc_json.get("wild_pokemon")?.as_array() {
                    for spawn in wild_pokemon {
                        if let (Some(poke_id), Some(spawn_rate), Some(min_lvl), Some(max_lvl)) = (
                            spawn.get("pokemon_id")?.as_u64(),
                            spawn.get("spawn_rate")?.as_f64(),
                            spawn.get("level_min")?.as_u64(),
                            spawn.get("level_max")?.as_u64(),
                        ) {
                            location.add_wild_pokemon(WildPokemonSpawn {
                                pokemon_id: poke_id as u32,
                                spawn_rate: spawn_rate as f32,
                                level_min: min_lvl as u32,
                                level_max: max_lvl as u32,
                            });
                        }
                    }
                }

                // Add NPCs
                if let Some(npcs) = loc_json.get("npcs")?.as_array() {
                    for npc_id in npcs {
                        if let Some(id) = npc_id.as_u64() {
                            location.npc_trainers.push(id as u32);
                        }
                    }
                }

                Some(location)
            }).collect()
        }
        None => Vec::new(),
    }
}

/// 通过ID获取地点
pub fn get_location_by_id(location_id: u32) -> Option<Location> {
    let game_data = loader::get_game_data()?;

    let loc_json = game_data.locations.iter().find(|loc| {
        loc.get("id").and_then(|v| v.as_u64()).map(|id| id as u32) == Some(location_id)
    })?;

    let id = location_id;
    let name = loc_json.get("name")?.as_str()?.to_string();
    let description = loc_json.get("description")?.as_str()?.to_string();
    let environment_str = loc_json.get("environment")?.as_str()?;
    let environment = match environment_str {
        "Grassland" => EnvironmentType::Grassland,
        "Forest" => EnvironmentType::Forest,
        "Mountain" => EnvironmentType::Mountain,
        "Cave" => EnvironmentType::Cave,
        "Water" => EnvironmentType::Water,
        "City" => EnvironmentType::City,
        _ => EnvironmentType::Grassland,
    };

    let mut location = Location::new(id, name, description, environment);
    location.encounter_rate = loc_json.get("encounter_rate")?.as_f64()? as f32;
    location.is_starting_location = loc_json.get("is_starting_location")?.as_bool()?;

    // Add connections
    if let Some(connections) = loc_json.get("connections")?.as_array() {
        for conn_id in connections {
            if let Some(cid) = conn_id.as_u64() {
                location.add_connection(cid as u32);
            }
        }
    }

    // Add wild pokemon
    if let Some(wild_pokemon) = loc_json.get("wild_pokemon")?.as_array() {
        for spawn in wild_pokemon {
            if let (Some(poke_id), Some(spawn_rate), Some(min_lvl), Some(max_lvl)) = (
                spawn.get("pokemon_id")?.as_u64(),
                spawn.get("spawn_rate")?.as_f64(),
                spawn.get("level_min")?.as_u64(),
                spawn.get("level_max")?.as_u64(),
            ) {
                location.add_wild_pokemon(WildPokemonSpawn {
                    pokemon_id: poke_id as u32,
                    spawn_rate: spawn_rate as f32,
                    level_min: min_lvl as u32,
                    level_max: max_lvl as u32,
                });
            }
        }
    }

    // Add NPCs
    if let Some(npcs) = loc_json.get("npcs")?.as_array() {
        for npc_id in npcs {
            if let Some(nid) = npc_id.as_u64() {
                location.npc_trainers.push(nid as u32);
            }
        }
    }

    Some(location)
}
