mod game;
mod cli;
mod data;
mod utils;
mod map;
mod npc;
mod pokemon_generator;

use cli::{Menu, RevivalMenu, MapMenu, LocationMenu};
use game::{Player, Pokemon, Battle, WildPokemonEncounter};
use data::{pokemon_data, locations_data};
use rand::Rng;
use map::{GameMap, create_locations};
use npc::{get_npc_by_id, Difficulty};
use pokemon_generator::generate_npc_team;

fn main() {
    loop {
        Menu::print_main_menu();
        let choice = Menu::get_input();

        match choice.as_str() {
            "1" => start_game(),
            "2" => {
                println!("\n再见! 感谢游玩!");
                break;
            }
            _ => println!("无效的选择，请重试"),
        }
    }
}

fn start_game() {
    let player_name = Menu::get_player_name();
    let mut player = Player::new(player_name.clone());

    // 给玩家一只初始宝可梦
    let starter_pokemon = pokemon_data::get_pokemon_by_id(25).unwrap(); // Pikachu
    player.add_pokemon(starter_pokemon);

    println!("\n═══════════════════════════════════════");
    println!("欢迎, {}!", player_name);
    println!("你获得了一只皮卡丘!");
    println!("═══════════════════════════════════════\n");

    game_loop(&mut player);
}

fn game_loop(player: &mut Player) {
    let all_locations = locations_data::get_all_locations();

    loop {
        // 检查新解锁的地点
        player.check_new_unlocks(&all_locations);

        // 检查是否全队昏迷，如果是则进入复活菜单
        if !player.has_active_pokemon() {
            if !handle_all_pokemon_fainted(player) {
                // 玩家选择返回主菜单或无法复活
                break;
            }
            continue;
        }

        // 获取当前位置信息
        let current_location_id = player.location_state.current_location_id;
        let visited_count = player.location_state.visited_count();
        let current_location = locations_data::get_location_by_id(current_location_id);

        if let Some(location) = &current_location {
            Menu::print_game_menu_with_location(&location.name, visited_count, all_locations.len());
        } else {
            Menu::print_game_menu();
        }

        let choice = Menu::get_input();

        match choice.as_str() {
            "1" => explore_location_encounter(player, current_location.as_ref()),
            "2" => {
                if let Some(_location) = &current_location {
                    handle_movement(player, &all_locations);
                } else {
                    println!("错误: 无法获取当前位置信息");
                }
            }
            "3" => {
                player.display_team();
            }
            "4" => {
                player.display_items();
            }
            "5" => {
                let mut game_map = GameMap::new();
                game_map.initialize(create_locations());
                explore_map(player, &mut game_map);
            }
            "6" => {
                println!("\n感谢游玩!");
                break;
            }
            _ => println!("无效的选择，请重试"),
        }
    }
}

/// 处理地点间的移动 (Task 6.1)
fn handle_movement(player: &mut Player, all_locations: &[game::Location]) {
    let current_location = locations_data::get_location_by_id(player.location_state.current_location_id);

    if let Some(location) = current_location {
        // 收集可达且已解锁的地点
        let mut reachable = Vec::new();
        for connected_id in &location.connected_locations {
            if let Some(connected) = locations_data::get_location_by_id(*connected_id) {
                let is_unlocked = player.can_unlock_location(&connected.unlock_requirement);
                reachable.push((*connected_id, connected.name, is_unlocked));
            }
        }

        if reachable.is_empty() {
            LocationMenu::show_no_reachable_locations();
            return;
        }

        LocationMenu::display_movement_menu(&location, &reachable);
        let choice = LocationMenu::get_input();

        if choice == "0" {
            return;
        }

        if let Ok(idx) = choice.parse::<usize>() {
            if idx > 0 && idx <= reachable.len() {
                let (target_id, target_name, is_unlocked) = &reachable[idx - 1];

                if !is_unlocked {
                    LocationMenu::show_movement_error("此地点未解锁");
                    return;
                }

                // 移动到目标地点
                player.location_state.current_location_id = *target_id;
                player.location_state.mark_visited(*target_id);
                LocationMenu::show_movement_success(target_name);
            }
        }
    }
}

/// 探索当前地点，寻找野生宝可梦 (Task 6.2)
fn explore_location_encounter(player: &mut Player, location: Option<&game::Location>) {
    if !player.has_active_pokemon() {
        cli::display::print_game_over();
        return;
    }

    let location = match location {
        Some(loc) => loc,
        None => {
            println!("错误: 无法获取当前地点信息");
            return;
        }
    };

    // 检查遭遇率
    let mut rng = rand::thread_rng();
    let encounter_chance: f32 = rng.gen();

    if encounter_chance > location.encounter_rate {
        println!("\n你探索了 {}，但没有遇到任何宝可梦。", location.name);
        return;
    }

    // 从该地点的野生宝可梦池中生成一只宝可梦
    if location.wild_pokemon_pool.is_empty() {
        println!("\n这个地点没有野生宝可梦。");
        return;
    }

    match WildPokemonEncounter::generate_wild_pokemon(&location.wild_pokemon_pool) {
        Ok(wild_pokemon_instance) => {
            // 获取环境加成
            let environment_bonus = game::EnvironmentBonus::from_environment(location.environment);

            // 生成预览
            match WildPokemonEncounter::generate_preview(
                &wild_pokemon_instance,
                &environment_bonus,
                location.environment_name(),
            ) {
                Ok(preview) => {
                    // 显示遭遇预览
                    preview.display();

                    // 处理玩家选择
                    loop {
                        let choice = LocationMenu::get_input();

                        match choice.as_str() {
                            "1" => {
                                // 尝试捕捉
                                // 注意: 这里需要使用原始Pokémon，不能使用加成后的属性
                                println!("捕捉功能即将实现");
                                break;
                            }
                            "2" => {
                                // 进行战斗
                                println!("战斗功能即将实现");
                                break;
                            }
                            "3" => {
                                // 逃跑
                                println!("你成功逃跑了!");
                                break;
                            }
                            _ => println!("无效的选择，请重试"),
                        }
                    }
                }
                Err(e) => println!("生成预览失败: {}", e),
            }
        }
        Err(e) => println!("生成野生宝可梦失败: {}", e),
    }
}

fn explore(player: &mut Player) {
    if !player.has_active_pokemon() {
        cli::display::print_game_over();
        return;
    }

    let wild_pokemon = pokemon_data::get_wild_pokemon();
    println!("\n═══════════════════════════════════════");
    println!("你来到了野外!");
    println!("突然一只 {} 出现了!", wild_pokemon.name);
    println!("═══════════════════════════════════════");

    loop {
        Menu::print_exploration_menu();
        let choice = Menu::get_input();

        match choice.as_str() {
            "1" => {
                // 捕捉
                if player.use_item("Poké Ball", 1) {
                    println!("\n你投掷了一枚精灵球!");
                    let success = attempt_capture(&wild_pokemon);
                    cli::display::print_capture_result(success, &wild_pokemon.name);

                    if success {
                        if player.add_pokemon(wild_pokemon.clone()) {
                            println!("✓ {} 加入了你的队伍!", wild_pokemon.name);
                        } else {
                            cli::display::print_team_full();
                            // 保存到 pc
                        }
                        break;
                    }
                } else {
                    cli::display::print_no_balls();
                }
            }
            "2" => {
                // 战斗
                let player_active = player.get_active_pokemon().unwrap().clone();
                battle_wild(player, player_active.clone(), wild_pokemon.clone());
                break;
            }
            "3" => {
                // 逃跑
                if attempt_escape() {
                    cli::display::print_escape_result(true);
                    break;
                } else {
                    cli::display::print_escape_result(false);
                    // 对手发动攻击
                }
            }
            _ => println!("无效的选择"),
        }
    }
}

fn attempt_capture(pokemon: &Pokemon) -> bool {
    let mut rng = rand::thread_rng();
    let capture_rate = pokemon.catch_rate as f32 / 255.0;
    let hp_ratio = pokemon.hp as f32 / pokemon.max_hp as f32;
    let success_rate = capture_rate * (1.0 - hp_ratio * 0.5);
    rng.gen::<f32>() < success_rate
}

fn attempt_escape() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen::<f32>() > 0.5 // 50% 逃跑成功率
}

fn battle_wild(player: &mut Player, mut player_pokemon: Pokemon, mut wild_pokemon: Pokemon) {
    println!("\n═══════════════════════════════════════");
    println!("开始战斗!");
    println!("═══════════════════════════════════════");

    let mut battle = Battle::new(player_pokemon.clone(), wild_pokemon.clone());

    loop {
        cli::display::print_battle_status(&player_pokemon, &wild_pokemon);

        if wild_pokemon.is_fainted() {
            cli::display::print_victory();
            // 获得经验
            battle.award_experience(50);
            player_pokemon.experience += 50;

            // 检查升级
            let old_level = player_pokemon.level;
            if player_pokemon.experience > player_pokemon.level as u32 * 100 {
                player_pokemon.level_up();
                if player_pokemon.level > old_level {
                    cli::display::print_level_up(&player_pokemon);
                }
            }

            // 更新玩家的宝可梦
            if let Some(active) = player.get_active_pokemon() {
                *active = player_pokemon;
            }
            break;
        }

        if player_pokemon.is_fainted() {
            cli::display::print_defeat();
            if let Some(active) = player.get_active_pokemon() {
                *active = player_pokemon;
            }
            break;
        }

        Menu::print_battle_menu();
        let choice = Menu::get_input();

        match choice.as_str() {
            "1" => {
                // 选择招式
                let moves_options: Vec<_> = player_pokemon
                    .moves
                    .iter()
                    .enumerate()
                    .map(|(i, m)| (i + 1, m.name.clone()))
                    .collect();

                cli::display::print_battle_status(&player_pokemon, &wild_pokemon);
                Menu::print_move_menu(&moves_options);
                let move_choice = Menu::get_input();

                if let Ok(idx) = move_choice.parse::<usize>() {
                    if idx > 0 && idx <= player_pokemon.moves.len() {
                        let player_move = &player_pokemon.moves[idx - 1];
                        if Battle::check_hit(player_move.accuracy) {
                            let damage = Battle::calculate_damage(&player_pokemon, &wild_pokemon, player_move);
                            wild_pokemon.take_damage(damage);
                            cli::display::print_battle_message(&format!(
                                "你的 {} 使用了 {}，造成 {} 伤害!",
                                player_pokemon.name, player_move.name, damage
                            ));
                        } else {
                            cli::display::print_battle_message("攻击没有命中!");
                        }

                        // 对手反击
                        if !wild_pokemon.is_fainted() && !wild_pokemon.moves.is_empty() {
                            let mut rng = rand::thread_rng();
                            let wild_move = &wild_pokemon.moves[rng.gen_range(0..wild_pokemon.moves.len())];
                            if Battle::check_hit(wild_move.accuracy) {
                                let damage = Battle::calculate_damage(&wild_pokemon, &player_pokemon, wild_move);
                                player_pokemon.take_damage(damage);
                                cli::display::print_battle_message(&format!(
                                    "对手的 {} 使用了 {}，造成 {} 伤害!",
                                    wild_pokemon.name, wild_move.name, damage
                                ));
                            } else {
                                cli::display::print_battle_message("对手的攻击没有命中!");
                            }
                        }
                    }
                }
            }
            "2" => {
                // 使用道具
                player.display_items();
            }
            "3" => {
                // 切换宝可梦
                println!("\n选择要切换的宝可梦:");
                for (i, poke) in player.pokemons.iter().enumerate() {
                    if !poke.is_fainted() {
                        println!("{}. {}", i + 1, poke);
                    }
                }
                print!("选择: ");
                std::io::Write::flush(&mut std::io::stdout()).unwrap();

                let switch_choice = Menu::get_input();
                if let Ok(idx) = switch_choice.parse::<usize>() {
                    if idx > 0 && idx <= player.pokemons.len() && !player.pokemons[idx - 1].is_fainted() {
                        player_pokemon = player.pokemons[idx - 1].clone();
                        cli::display::print_battle_message(&format!("切换为 {}!", player_pokemon.name));
                    }
                }
            }
            "4" => {
                // 逃跑
                if attempt_escape() {
                    cli::display::print_escape_result(true);
                    if let Some(active) = player.get_active_pokemon() {
                        *active = player_pokemon;
                    }
                    return;
                } else {
                    cli::display::print_escape_result(false);
                }
            }
            _ => println!("无效的选择"),
        }
    }
}

// 处理全队昏迷的菜单
fn handle_all_pokemon_fainted(player: &mut Player) -> bool {
    loop {
        RevivalMenu::print_all_pokemon_fainted_menu();
        let choice = Menu::get_input();

        match choice.as_str() {
            "1" => {
                // 使用恢复道具
                if !handle_item_revival(player) {
                    // 如果没有恢复道具，继续菜单
                }
            }
            "2" => {
                // 去精灵中心
                handle_pokemon_center_revival(player);
            }
            "3" => {
                // 查看队伍
                player.display_team();
            }
            "4" => {
                // 返回主菜单
                return false;
            }
            _ => println!("无效的选择，请重试"),
        }

        // 检查是否有活跃宝可梦，如果有则返回 true 继续游戏
        if player.has_active_pokemon() {
            if !player.visited_pokemon_center {
                RevivalMenu::print_first_visit_bonus();
            }
            return true;
        }
    }
}

// 处理使用道具复活
fn handle_item_revival(player: &mut Player) -> bool {
    loop {
        RevivalMenu::print_recovery_item_menu(player);
        let choice = Menu::get_input();

        if choice == "0" {
            break;
        }

        // 获取恢复道具列表
        let recovery_items = vec!["恢复药", "超级恢复药", "全复活", "完全恢复"];
        let mut available_items = Vec::new();

        for item_name in recovery_items {
            if player.items.get(item_name).unwrap_or(&0) > &0 {
                available_items.push(item_name);
            }
        }

        if let Ok(idx) = choice.parse::<usize>() {
            if idx > 0 && idx <= available_items.len() {
                let item_name = available_items[idx - 1];

                // 显示昏迷的宝可梦
                RevivalMenu::print_select_pokemon_to_revive_menu(player);
                let poke_choice = Menu::get_input();

                if poke_choice == "0" {
                    continue;
                }

                if let Ok(poke_idx) = poke_choice.parse::<usize>() {
                    let mut fainted_indices = Vec::new();
                    for (i, poke) in player.pokemons.iter().enumerate() {
                        if poke.is_fainted() {
                            fainted_indices.push(i);
                        }
                    }

                    if poke_idx > 0 && poke_idx <= fainted_indices.len() {
                        let pokemon_index = fainted_indices[poke_idx - 1];
                        match player.revive_pokemon_with_item(pokemon_index, item_name) {
                            Ok(msg) => RevivalMenu::print_revival_success(&msg),
                            Err(msg) => RevivalMenu::print_revival_failed(&msg),
                        }
                        return true;
                    }
                }
            }
        }
    }

    false
}

// 处理精灵中心复活
fn handle_pokemon_center_revival(player: &mut Player) {
    loop {
        RevivalMenu::print_pokemon_center_menu(player);
        let choice = Menu::get_input();

        if choice == "0" {
            break;
        }

        if choice == "1" {
            // 复活全队
            match player.revive_full_team_at_center() {
                Ok(msg) => RevivalMenu::print_revival_success(&msg),
                Err(msg) => RevivalMenu::print_revival_failed(&msg),
            }
            break;
        }
    }
}

// 地图探索系统
fn explore_map(player: &mut Player, game_map: &mut GameMap) {
    loop {
        MapMenu::print_map_navigation_menu();
        let choice = Menu::get_input();

        match choice.as_str() {
            "1" => {
                // 选择地区
                MapMenu::print_region_menu();
                let region_choice = Menu::get_input();

                if region_choice == "1" {
                    // 关都地区
                    explore_region(player, game_map, 1);
                }
            }
            "2" => {
                // 查看已访问位置
                let visited: Vec<_> = game_map
                    .get_accessible_locations()
                    .into_iter()
                    .filter(|loc| game_map.has_visited(loc.id))
                    .collect();

                MapMenu::print_visited_locations(&visited);
                let _ = Menu::get_input();
            }
            "3" => {
                break;
            }
            _ => println!("无效的选择"),
        }
    }
}

// 探索地区
fn explore_region(player: &mut Player, game_map: &mut GameMap, region_id: u32) {
    let locations = map::get_locations_by_region(region_id);

    loop {
        MapMenu::print_location_menu(&locations);
        let choice = Menu::get_input();

        if choice == "0" {
            break;
        }

        if let Ok(idx) = choice.parse::<usize>() {
            if idx > 0 && idx <= locations.len() {
                let location = &locations[idx - 1];
                let location_id = location.id;

                if let Err(e) = game_map.move_to_location(location_id) {
                    println!("错误: {}", e);
                    continue;
                }

                explore_location(player, game_map, location_id);
            }
        }
    }
}

// 探索位置
fn explore_location(player: &mut Player, game_map: &GameMap, location_id: u32) {
    loop {
        if let Some(location_with_npcs) = game_map.get_location_with_npcs(location_id) {
            MapMenu::print_location_detail_menu(&location_with_npcs);
            let choice = Menu::get_input();

            match choice.as_str() {
                "1" => {
                    // 对战 NPC
                    if !location_with_npcs.npcs.is_empty() {
                        battle_npc_menu(player, &location_with_npcs.npcs);
                    }
                }
                "2" => {
                    break;
                }
                _ => println!("无效的选择"),
            }
        } else {
            println!("位置未找到");
            break;
        }
    }
}

// NPC 对战菜单
fn battle_npc_menu(player: &mut Player, npcs: &[crate::npc::NPCTrainer]) {
    loop {
        MapMenu::print_npc_select_menu(npcs);
        let choice = Menu::get_input();

        if choice == "0" {
            break;
        }

        if let Ok(idx) = choice.parse::<usize>() {
            if idx > 0 && idx <= npcs.len() {
                let npc = &npcs[idx - 1];
                MapMenu::print_battle_preview_menu(npc);
                let battle_choice = Menu::get_input();

                match battle_choice.as_str() {
                    "1" => {
                        // 开始对战
                        if !player.has_active_pokemon() {
                            println!("\n你没有可用的宝可梦!");
                            continue;
                        }

                        MapMenu::print_battle_start(&npc.name, &npc.title);

                        // 生成 NPC 队伍 (包含难度调整)
                        let difficulty_adj = npc.difficulty.level_adjustment();
                        let npc_pokemon_ids = npc.pokemon_ids.clone();

                        if let Ok(npc_team) = generate_npc_team(&npc_pokemon_ids, 5, difficulty_adj) {
                            // 这里需要实现实际的对战逻辑
                            // 现在我们只是简单地显示"对战开始"
                            println!("对战已启动! (完整对战逻辑待实现)");
                            println!("NPC 队伍中有 {} 只宝可梦", npc_team.len());
                        }
                        break;
                    }
                    "2" => {
                        break;
                    }
                    _ => println!("无效的选择"),
                }
            }
        }
    }
}
