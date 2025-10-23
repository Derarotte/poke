use crate::game::Player;
use std::io::Write;

pub struct RevivalMenu;

impl RevivalMenu {
    pub fn print_all_pokemon_fainted_menu() {
        println!("\n╔═════════════════════════════════════╗");
        println!("║    你的队伍全部昏迷了!             ║");
        println!("║    选择复活方式:                    ║");
        println!("╠═════════════════════════════════════╣");
        println!("║ 1. 使用恢复道具                    ║");
        println!("║ 2. 去精灵中心 (200 金币)            ║");
        println!("║ 3. 查看队伍状态                    ║");
        println!("║ 4. 返回主菜单                      ║");
        println!("╚═════════════════════════════════════╝");
        print!("请选择: ");
        std::io::stdout().flush().unwrap();
    }

    pub fn print_recovery_item_menu(player: &Player) {
        println!("\n╔═════════════════════════════════════╗");
        println!("║      选择恢复道具                  ║");
        println!("╠═════════════════════════════════════╣");

        let recovery_items = vec![
            ("恢复药", "恢复单只宝可梦 50% HP"),
            ("超级恢复药", "恢复单只宝可梦 100% HP"),
            ("全复活", "复活昏迷宝可梦，恢复 50% HP"),
            ("完全恢复", "复活昏迷宝可梦，恢复 100% HP"),
        ];

        let mut index = 1;
        for (item_name, effect) in recovery_items {
            if let Some(count) = player.items.get(item_name) {
                println!("║ {}. {} x{}                   ║", index, item_name, count);
                println!("║    {}", effect);
                index += 1;
            }
        }

        println!("║ 0. 返回                            ║");
        println!("╚═════════════════════════════════════╝");
        print!("请选择: ");
        std::io::stdout().flush().unwrap();
    }

    pub fn print_pokemon_center_menu(player: &Player) {
        println!("\n╔═════════════════════════════════════╗");
        println!("║      欢迎来到精灵中心!            ║");
        println!("║      你的金币: ¥{}                ║", player.money);
        println!("╠═════════════════════════════════════╣");

        let cost_per_pokemon = if player.visited_pokemon_center { 100 } else { 200 };
        let fainted_count = player.get_fainted_pokemon_count();

        println!("║ 昏迷宝可梦: {} 只               ║", fainted_count);
        println!("║ 单只复活费用: ¥{}                  ║", cost_per_pokemon);
        println!("║ 全队复活费用: ¥{}                  ║", cost_per_pokemon * fainted_count as u32);
        println!("║                                     ║");
        println!("║ 1. 复活全队                        ║");
        println!("║ 0. 离开                            ║");
        println!("╚═════════════════════════════════════╝");
        print!("请选择: ");
        std::io::stdout().flush().unwrap();
    }

    pub fn print_select_pokemon_to_revive_menu(player: &Player) {
        println!("\n╔═════════════════════════════════════╗");
        println!("║    选择要复活的宝可梦             ║");
        println!("╠═════════════════════════════════════╣");

        let mut index = 1;
        for (_i, pokemon) in player.pokemons.iter().enumerate() {
            if pokemon.is_fainted() {
                println!("║ {}. {} (等级 {})              ║", index, pokemon.name, pokemon.level);
                index += 1;
            }
        }

        println!("║ 0. 返回                            ║");
        println!("╚═════════════════════════════════════╝");
        print!("请选择: ");
        std::io::stdout().flush().unwrap();
    }

    pub fn print_insufficient_items() {
        println!("\n✗ 道具不足，无法使用!");
    }

    pub fn print_insufficient_money(needed: u32, have: u32) {
        println!("\n✗ 金币不足!");
        println!("  需要: ¥{}，拥有: ¥{}", needed, have);
    }

    pub fn print_revival_success(message: &str) {
        println!("\n{}", message);
    }

    pub fn print_revival_failed(message: &str) {
        println!("\n✗ 复活失败: {}", message);
    }

    pub fn print_first_visit_bonus() {
        println!("\n═══════════════════════════════════");
        println!("✓ 第一次拜访精灵中心!");
        println!("✓ 后续复活费用将获得 50% 折扣!");
        println!("═══════════════════════════════════");
    }
}
