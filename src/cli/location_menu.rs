use std::io::{self, Write};
use crate::game::{Location, Player, WildPokemonPreview};

pub struct LocationMenu;

impl LocationMenu {
    /// 显示当前地点信息
    pub fn display_location_info(
        location: &Location,
        player: &Player,
    ) {
        println!("\n╔─────────────────────────────────────╗");
        println!("║  地点: {:<28} ║", location.name);
        println!("╠─────────────────────────────────────╣");
        println!("║ 描述: {:<28} ║", location.description);
        println!("║ 环境: {:<28} ║", location.environment_name());
        println!("║ 遭遇率: {:<25} ║", format!("{}%", (location.encounter_rate * 100.0) as u32));

        // 显示连接的地点
        if !location.connected_locations.is_empty() {
            println!("║                                     ║");
            println!("║ 可达地点:                          ║");
            for conn_id in &location.connected_locations {
                println!("║   - 地点 ID: {:<22} ║", conn_id);
            }
        }

        // 显示解锁条件
        println!("║                                     ║");
        println!("║ 解锁条件: {:<22} ║", player.get_unlock_status(&location.unlock_requirement));
        println!("╚─────────────────────────────────────╝");
    }

    /// 显示可移动到的地点菜单
    pub fn display_movement_menu(
        _current_location: &Location,
        reachable_locations: &[(u32, String, bool)], // (id, name, is_unlocked)
    ) {
        println!("\n╔─────────────────────────────────────╗");
        println!("║     选择要前往的地点               ║");
        println!("╠─────────────────────────────────────╣");

        for (idx, (_id, name, is_unlocked)) in reachable_locations.iter().enumerate() {
            let status = if *is_unlocked {
                format!("✓ {}", name)
            } else {
                format!("✗ {} (未解锁)", name)
            };
            println!("║ {}. {:<32} ║", idx + 1, status);
        }

        println!("║ 0. 返回                            ║");
        println!("╚─────────────────────────────────────╝");
        print!("请选择: ");
        io::stdout().flush().unwrap();
    }

    /// 显示地点解锁条件详情
    pub fn display_unlock_requirements(
        location: &Location,
        player: &Player,
    ) {
        println!("\n╔─────────────────────────────────────╗");
        println!("║  地点解锁条件: {:<21} ║", location.name);
        println!("╠─────────────────────────────────────╣");

        let requirement = &location.unlock_requirement;
        let mut has_requirement = false;

        if requirement.required_level > 1 {
            has_requirement = true;
            let max_level = player.get_max_level();
            let status = if max_level >= requirement.required_level { "✓" } else { "✗" };
            println!("║ {} 等级: {}/{:<25} ║", status, max_level, requirement.required_level);
        }

        if !requirement.required_badges.is_empty() {
            has_requirement = true;
            let status = if player.badges.len() >= requirement.required_badges.len() { "✓" } else { "✗" };
            println!("║ {} 徽章: {}/{:<24} ║", status, player.badges.len(), requirement.required_badges.len());
        }

        if let Some(required_count) = requirement.required_pokemon_count {
            has_requirement = true;
            let status = if player.pokemons.len() >= required_count as usize { "✓" } else { "✗" };
            println!("║ {} 宝可梦: {}/{:<22} ║", status, player.pokemons.len(), required_count);
        }

        if !has_requirement {
            println!("║ 无条件 (已可访问)                  ║");
        }

        println!("╚─────────────────────────────────────╝");
    }

    /// 显示遭遇预览菜单
    pub fn display_encounter_preview(preview: &WildPokemonPreview) {
        preview.display();
    }

    /// 显示位置信息 (简短版本，用于主菜单)
    pub fn display_location_status(
        location: &Location,
        visited_count: usize,
        total_locations: usize,
    ) {
        println!("\n当前位置: {} | 已访问: {}/{}", location.name, visited_count, total_locations);
        println!("环境加成: {} (+10% 某项属性)", location.environment_name());
    }

    /// 获取用户输入
    pub fn get_input() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    /// 显示移动失败信息
    pub fn show_movement_error(reason: &str) {
        println!("\n❌ 无法移动: {}", reason);
    }

    /// 显示成功移动信息
    pub fn show_movement_success(location_name: &str) {
        println!("\n✓ 成功前往 {}!", location_name);
    }

    /// 显示地点探索菜单
    pub fn display_exploration_menu(location_name: &str) {
        println!("\n╔─────────────────────────────────────╗");
        println!("║  你来到了 {:<23} ║", location_name);
        println!("╠─────────────────────────────────────╣");
        println!("║ 1. 探索 (寻找宝可梦)               ║");
        println!("║ 2. 查看地点信息                    ║");
        println!("║ 3. 查看解锁条件                    ║");
        println!("║ 4. 移动到其他地点                  ║");
        println!("║ 5. 返回主菜单                      ║");
        println!("╚─────────────────────────────────────╝");
        print!("请选择: ");
        io::stdout().flush().unwrap();
    }

    /// 显示没有可达地点信息
    pub fn show_no_reachable_locations() {
        println!("\n⚠ 当前地点没有相邻的地点可以前往。");
    }
}
