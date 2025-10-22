use crate::map::{Location, LocationWithNPCs};
use crate::npc::NPCTrainer;
use std::io::Write;

pub struct MapMenu;

impl MapMenu {
    /// 打印地区选择菜单
    pub fn print_region_menu() {
        println!("\n╔═════════════════════════════════════╗");
        println!("║      选择要探索的地区             ║");
        println!("╠═════════════════════════════════════╣");
        println!("║ 1. 关都地区                        ║");
        println!("║                                     ║");
        println!("║ 0. 返回                            ║");
        println!("╚═════════════════════════════════════╝");
        print!("请选择: ");
        std::io::stdout().flush().unwrap();
    }

    /// 打印位置选择菜单
    pub fn print_location_menu(locations: &[Location]) {
        println!("\n╔═════════════════════════════════════╗");
        println!("║      选择要访问的地点             ║");
        println!("╠═════════════════════════════════════╣");

        for (i, location) in locations.iter().enumerate() {
            println!("║ {}. {}                       ║", i + 1, location.name);
        }

        println!("║                                     ║");
        println!("║ 0. 返回                            ║");
        println!("╚═════════════════════════════════════╝");
        print!("请选择: ");
        std::io::stdout().flush().unwrap();
    }

    /// 打印位置详细信息菜单
    pub fn print_location_detail_menu(location_with_npcs: &LocationWithNPCs) {
        location_with_npcs.display_info();

        println!("\n╔═════════════════════════════════════╗");
        println!("║      选择行动                     ║");
        println!("╠═════════════════════════════════════╣");

        if !location_with_npcs.npcs.is_empty() {
            println!("║ 1. 对战 NPC                        ║");
        }

        println!("║ 2. 返回地点选择                    ║");
        println!("╚═════════════════════════════════════╝");
        print!("请选择: ");
        std::io::stdout().flush().unwrap();
    }

    /// 打印 NPC 选择菜单
    pub fn print_npc_select_menu(npcs: &[NPCTrainer]) {
        println!("\n╔═════════════════════════════════════╗");
        println!("║      选择对手                     ║");
        println!("╠═════════════════════════════════════╣");

        for (i, npc) in npcs.iter().enumerate() {
            let status = if npc.defeated { "✓ 已击败" } else { "[ ]" };
            println!("║ {}. {} {} {}                 ║",
                     i + 1, status, npc.title, npc.name);
        }

        println!("║                                     ║");
        println!("║ 0. 返回                            ║");
        println!("╚═════════════════════════════════════╝");
        print!("请选择: ");
        std::io::stdout().flush().unwrap();
    }

    /// 打印对战预览菜单
    pub fn print_battle_preview_menu(npc: &NPCTrainer) {
        println!("\n╔═════════════════════════════════════╗");
        println!("║        对手信息                    ║");
        println!("╠═════════════════════════════════════╣");
        println!("║ 对手: {} {}                  ║", npc.title, npc.name);
        println!("║ 队伍数: {} 只                      ║", npc.pokemon_ids.len());
        println!("║ 奖励: ¥{}                          ║", npc.reward_money);

        if npc.defeated {
            println!("║ 状态: ✓ 已击败                     ║");
        } else {
            println!("║ 状态: [ ] 未击败                   ║");
        }

        println!("╠═════════════════════════════════════╣");
        println!("║ 1. 开始对战                        ║");
        println!("║ 2. 返回                            ║");
        println!("╚═════════════════════════════════════╝");
        print!("请选择: ");
        std::io::stdout().flush().unwrap();
    }

    /// 打印地图导航菜单
    pub fn print_map_navigation_menu() {
        println!("\n╔═════════════════════════════════════╗");
        println!("║        地图系统                    ║");
        println!("╠═════════════════════════════════════╣");
        println!("║ 1. 选择地区                        ║");
        println!("║ 2. 查看已访问位置                  ║");
        println!("║ 3. 返回主菜单                      ║");
        println!("╚═════════════════════════════════════╝");
        print!("请选择: ");
        std::io::stdout().flush().unwrap();
    }

    /// 打印访问过的位置列表
    pub fn print_visited_locations(locations: &[Location]) {
        println!("\n╔═════════════════════════════════════╗");
        println!("║      已访问的位置                 ║");
        println!("╠═════════════════════════════════════╣");

        if locations.is_empty() {
            println!("║ 暂无访问过的位置                    ║");
        } else {
            for (i, location) in locations.iter().enumerate() {
                println!("║ {}. {}                       ║", i + 1, location.name);
            }
        }

        println!("╠═════════════════════════════════════╣");
        println!("║ 0. 返回                            ║");
        println!("╚═════════════════════════════════════╝");
        print!("请选择: ");
        std::io::stdout().flush().unwrap();
    }

    /// 打印对战结果
    pub fn print_battle_result(won: bool, reward_money: u32) {
        if won {
            println!("\n╔═════════════════════════════════════╗");
            println!("║      ✓ 你赢得了对战!             ║");
            println!("║      获得 ¥{}               ║", reward_money);
            println!("╚═════════════════════════════════════╝");
        } else {
            println!("\n╔═════════════════════════════════════╗");
            println!("║      ✗ 你输掉了对战!             ║");
            println!("╚═════════════════════════════════════╝");
        }
    }

    /// 打印对战开始信息
    pub fn print_battle_start(npc_name: &str, npc_title: &str) {
        println!("\n═══════════════════════════════════");
        println!("对战开始!");
        println!("对手: {} {}", npc_title, npc_name);
        println!("═══════════════════════════════════\n");
    }
}
