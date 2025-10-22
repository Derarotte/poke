// 地图与 NPC 系统测试

#[cfg(test)]
mod map_system_tests {
    use poke::map::{GameMap, Location, Region, create_locations};

    #[test]
    fn test_game_map_creation() {
        let game_map = GameMap::new();
        assert_eq!(game_map.regions.len(), 0);
        assert_eq!(game_map.current_location, None);
        assert_eq!(game_map.visited_locations.len(), 0);
    }

    #[test]
    fn test_game_map_initialization() {
        let mut game_map = GameMap::new();
        let locations = create_locations();
        game_map.initialize(locations.clone());

        assert!(!game_map.regions.is_empty());
        assert_eq!(game_map.regions.len(), 1);
        assert_eq!(game_map.regions[0].name, "关都地区");
    }

    #[test]
    fn test_location_navigation() {
        let mut game_map = GameMap::new();
        game_map.initialize(create_locations());

        // 移动到第一个地点
        let result = game_map.move_to_location(101);
        assert!(result.is_ok());
        assert_eq!(game_map.current_location, Some(101));
        assert!(game_map.has_visited(101));
    }

    #[test]
    fn test_invalid_location_navigation() {
        let mut game_map = GameMap::new();
        game_map.initialize(create_locations());

        // 尝试移动到不存在的地点
        let result = game_map.move_to_location(999);
        assert!(result.is_err());
        assert_eq!(game_map.current_location, None);
    }

    #[test]
    fn test_visited_locations_tracking() {
        let mut game_map = GameMap::new();
        game_map.initialize(create_locations());

        // 访问多个地点
        let _ = game_map.move_to_location(101);
        let _ = game_map.move_to_location(102);
        let _ = game_map.move_to_location(103);

        assert!(game_map.has_visited(101));
        assert!(game_map.has_visited(102));
        assert!(game_map.has_visited(103));
        assert!(!game_map.has_visited(999));
    }

    #[test]
    fn test_get_current_location() {
        let mut game_map = GameMap::new();
        game_map.initialize(create_locations());

        let _ = game_map.move_to_location(101);
        let current = game_map.get_current_location();

        assert!(current.is_some());
        assert_eq!(current.unwrap().name, "常青小镇");
    }

    #[test]
    fn test_get_accessible_locations() {
        let mut game_map = GameMap::new();
        game_map.initialize(create_locations());

        let locations = game_map.get_accessible_locations();
        assert!(locations.len() > 0);
        assert_eq!(locations.len(), 11); // 应该有11个地点
    }

    #[test]
    fn test_location_has_npc() {
        let mut game_map = GameMap::new();
        game_map.initialize(create_locations());

        // 常青小镇应该有青绿 (ID: 1)
        let npcs = game_map.get_npcs_at_location(101);
        assert_eq!(npcs.len(), 1);
        assert_eq!(npcs[0].id, 1);
    }

    #[test]
    fn test_location_with_npcs() {
        let mut game_map = GameMap::new();
        game_map.initialize(create_locations());

        let location_info = game_map.get_location_with_npcs(101);
        assert!(location_info.is_some());

        let info = location_info.unwrap();
        assert_eq!(info.location.name, "常青小镇");
        assert_eq!(info.npcs.len(), 1);
    }
}

#[cfg(test)]
mod npc_system_tests {
    use poke::npc::{
        NPCTrainer, Difficulty, get_npc_by_id, get_all_npcs, get_npcs_by_location,
    };

    #[test]
    fn test_difficulty_level_adjustment() {
        assert_eq!(Difficulty::Easy.level_adjustment(), -5);
        assert_eq!(Difficulty::Normal.level_adjustment(), 0);
        assert_eq!(Difficulty::Hard.level_adjustment(), 5);
        assert_eq!(Difficulty::Expert.level_adjustment(), 10);
    }

    #[test]
    fn test_difficulty_name() {
        assert_eq!(Difficulty::Easy.name(), "新手");
        assert_eq!(Difficulty::Normal.name(), "普通");
        assert_eq!(Difficulty::Hard.name(), "困难");
        assert_eq!(Difficulty::Expert.name(), "专家");
    }

    #[test]
    fn test_npc_trainer_creation() {
        let npc = NPCTrainer::new(
            1,
            "青绿".to_string(),
            "劲敌".to_string(),
            vec![25],
            100,
            Difficulty::Easy,
        );

        assert_eq!(npc.id, 1);
        assert_eq!(npc.name, "青绿");
        assert_eq!(npc.title, "劲敌");
        assert_eq!(npc.defeated, false);
        assert_eq!(npc.reward_money, 100);
    }

    #[test]
    fn test_npc_full_name() {
        let npc = NPCTrainer::new(
            1,
            "青绿".to_string(),
            "劲敌".to_string(),
            vec![25],
            100,
            Difficulty::Easy,
        );

        assert_eq!(npc.full_name(), "劲敌 青绿");
    }

    #[test]
    fn test_npc_mark_defeated() {
        let mut npc = NPCTrainer::new(
            1,
            "青绿".to_string(),
            "劲敌".to_string(),
            vec![25],
            100,
            Difficulty::Easy,
        );

        assert_eq!(npc.defeated, false);
        npc.mark_defeated();
        assert_eq!(npc.defeated, true);
    }

    #[test]
    fn test_get_npc_by_id() {
        let npc = get_npc_by_id(1);
        assert!(npc.is_some());
        assert_eq!(npc.unwrap().name, "青绿");
    }

    #[test]
    fn test_get_npc_invalid_id() {
        let npc = get_npc_by_id(999);
        assert!(npc.is_none());
    }

    #[test]
    fn test_get_all_npcs() {
        let npcs = get_all_npcs();
        assert!(npcs.len() >= 10);
    }

    #[test]
    fn test_get_npcs_by_location() {
        let location_npcs = vec![1]; // 常青小镇的 NPC
        let npcs = get_npcs_by_location(&location_npcs);

        assert_eq!(npcs.len(), 1);
        assert_eq!(npcs[0].id, 1);
    }
}

#[cfg(test)]
mod pokemon_generator_tests {
    use poke::pokemon_generator::{
        IndividualValues, Talent, Nature, PokemonInstance, get_species,
        generate_pokemon, generate_perfect_pokemon, generate_npc_team,
    };

    #[test]
    fn test_individual_values_random() {
        let ivs = IndividualValues::random();
        assert!(ivs.hp <= 31);
        assert!(ivs.attack <= 31);
        assert!(ivs.defense <= 31);
        assert!(ivs.sp_attack <= 31);
        assert!(ivs.sp_defense <= 31);
        assert!(ivs.speed <= 31);
    }

    #[test]
    fn test_individual_values_perfect() {
        let ivs = IndividualValues::perfect();
        assert_eq!(ivs.hp, 31);
        assert_eq!(ivs.attack, 31);
        assert_eq!(ivs.defense, 31);
        assert_eq!(ivs.sp_attack, 31);
        assert_eq!(ivs.sp_defense, 31);
        assert_eq!(ivs.speed, 31);
    }

    #[test]
    fn test_individual_values_total() {
        let ivs = IndividualValues {
            hp: 31,
            attack: 31,
            defense: 31,
            sp_attack: 31,
            sp_defense: 31,
            speed: 31,
        };
        assert_eq!(ivs.total(), 186);
    }

    #[test]
    fn test_talent_random() {
        let talent = Talent::random();
        assert!(talent == Talent::Normal || talent == Talent::Hidden);
    }

    #[test]
    fn test_nature_random() {
        let nature = Nature::random();
        assert!(!nature.name().is_empty());
    }

    #[test]
    fn test_pokemon_instance_creation() {
        let pokemon = PokemonInstance::new(25, 5);
        assert_eq!(pokemon.species_id, 25);
        assert_eq!(pokemon.level, 5);
        assert_eq!(pokemon.experience, 500);
    }

    #[test]
    fn test_pokemon_instance_perfect() {
        let pokemon = PokemonInstance::perfect(25, 5);
        assert_eq!(pokemon.individual_values.total(), 186);
        assert_eq!(pokemon.talent, Talent::Hidden);
    }

    #[test]
    fn test_get_species() {
        let species = get_species(25);
        assert!(species.is_some());
        assert_eq!(species.unwrap().name, "皮卡丘");
    }

    #[test]
    fn test_get_invalid_species() {
        let species = get_species(999);
        assert!(species.is_none());
    }

    #[test]
    fn test_generate_pokemon() {
        let result = generate_pokemon(25, 5);
        assert!(result.is_ok());
        let pokemon = result.unwrap();
        assert_eq!(pokemon.species_id, 25);
        assert_eq!(pokemon.level, 5);
    }

    #[test]
    fn test_generate_perfect_pokemon() {
        let result = generate_perfect_pokemon(25, 5);
        assert!(result.is_ok());
        let pokemon = result.unwrap();
        assert_eq!(pokemon.individual_values.total(), 186);
    }

    #[test]
    fn test_generate_npc_team() {
        let pokemon_ids = vec![25, 27, 54];
        let result = generate_npc_team(&pokemon_ids, 10, 5);

        assert!(result.is_ok());
        let team = result.unwrap();
        assert_eq!(team.len(), 3);
        assert_eq!(team[0].level, 15); // 10 + 5 (难度调整)
    }

    #[test]
    fn test_generate_npc_team_with_easy_difficulty() {
        let pokemon_ids = vec![25];
        let result = generate_npc_team(&pokemon_ids, 10, -5);

        assert!(result.is_ok());
        let team = result.unwrap();
        assert_eq!(team[0].level, 5); // 10 + (-5)
    }

    #[test]
    fn test_pokemon_nature_multipliers() {
        let pokemon = PokemonInstance::new(25, 5);
        let mults = pokemon.get_nature_multipliers();

        // 所有倍数应该在 0.9 到 1.1 之间
        assert!(mults.hp >= 0.9 && mults.hp <= 1.1);
        assert!(mults.attack >= 0.9 && mults.attack <= 1.1);
    }
}

#[cfg(test)]
mod integration_tests {
    use poke::map::GameMap;
    use poke::map::create_locations;

    #[test]
    fn test_map_with_all_locations_and_npcs() {
        let mut game_map = GameMap::new();
        game_map.initialize(create_locations());

        // 验证所有地点都能访问并有 NPC 信息
        for location in game_map.get_accessible_locations() {
            let info = game_map.get_location_with_npcs(location.id);
            assert!(info.is_some(), "地点 {} 应该有信息", location.name);
        }
    }

    #[test]
    fn test_full_map_exploration_flow() {
        let mut game_map = GameMap::new();
        game_map.initialize(create_locations());

        // 模拟完整探索流程
        let locations = game_map.get_accessible_locations();

        for location in locations.iter().take(5) {
            // 访问每个地点
            let result = game_map.move_to_location(location.id);
            assert!(result.is_ok());

            // 验证已访问
            assert!(game_map.has_visited(location.id));

            // 获取位置信息
            let info = game_map.get_location_with_npcs(location.id);
            assert!(info.is_some());
        }

        // 验证访问计数
        assert!(game_map.visited_locations.len() >= 5);
    }
}
