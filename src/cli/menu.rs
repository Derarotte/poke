use std::io::{self, Write};

pub struct Menu;

impl Menu {
    pub fn print_main_menu() {
        println!("\n╔════════════════════════════════════╗");
        println!("║     欢迎来到宝可梦冒险世界        ║");
        println!("╠════════════════════════════════════╣");
        println!("║ 1. 开始新游戏                      ║");
        println!("║ 2. 退出游戏                        ║");
        println!("╚════════════════════════════════════╝");
        print!("请选择: ");
        io::stdout().flush().unwrap();
    }

    pub fn print_game_menu_with_location(location_name: &str, visited_count: usize, total_locations: usize) {
        println!("\n╔════════════════════════════════════╗");
        println!("║  📍 当前位置: {:<22} ║", location_name);
        println!("║  🗺 已访问: {}/{:<20} ║", visited_count, total_locations);
        println!("╠════════════════════════════════════╣");
        println!("║ 1. 探索 (寻找宝可梦)               ║");
        println!("║ 2. 移动到其他地点                  ║");
        println!("║ 3. 查看队伍                        ║");
        println!("║ 4. 查看背包                        ║");
        println!("║ 5. 地图 (地区和对战)               ║");
        println!("║ 6. 回到大厅                        ║");
        println!("╚════════════════════════════════════╝");
        print!("请选择: ");
        io::stdout().flush().unwrap();
    }

    pub fn print_game_menu() {
        println!("\n╔════════════════════════════════════╗");
        println!("║         主菜单                     ║");
        println!("╠════════════════════════════════════╣");
        println!("║ 1. 探索 (寻找宝可梦)               ║");
        println!("║ 2. 地图 (地区和对战)               ║");
        println!("║ 3. 查看队伍                        ║");
        println!("║ 4. 查看背包                        ║");
        println!("║ 5. 回到大厅                        ║");
        println!("╚════════════════════════════════════╝");
        print!("请选择: ");
        io::stdout().flush().unwrap();
    }

    pub fn print_exploration_menu() {
        println!("\n╔════════════════════════════════════╗");
        println!("║       你来到了野外...              ║");
        println!("║   突然一只宝可梦出现了!            ║");
        println!("╠════════════════════════════════════╣");
        println!("║ 1. 投掷精灵球捕捉                  ║");
        println!("║ 2. 与之战斗                        ║");
        println!("║ 3. 逃跑                            ║");
        println!("╚════════════════════════════════════╝");
        print!("请选择: ");
        io::stdout().flush().unwrap();
    }

    pub fn print_battle_menu() {
        println!("\n╔════════════════════════════════════╗");
        println!("║        ← 对战中 →                  ║");
        println!("╠════════════════════════════════════╣");
        println!("║ 1. 使用招式                        ║");
        println!("║ 2. 使用道具                        ║");
        println!("║ 3. 切换宝可梦                      ║");
        println!("║ 4. 逃跑                            ║");
        println!("╚════════════════════════════════════╝");
        print!("请选择: ");
        io::stdout().flush().unwrap();
    }

    pub fn print_move_menu(moves: &[(usize, String)]) {
        println!("\n╔════════════════════════════════════╗");
        println!("║        选择要使用的招式             ║");
        println!("╠════════════════════════════════════╣");
        for (idx, name) in moves {
            println!("║ {}. {:<29} ║", idx, name);
        }
        println!("║ 0. 返回                            ║");
        println!("╚════════════════════════════════════╝");
        print!("请选择: ");
        io::stdout().flush().unwrap();
    }

    pub fn get_input() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    pub fn get_player_name() -> String {
        println!("\n═══════════════════════════════════════");
        println!("     欢迎来到宝可梦世界!");
        println!("═══════════════════════════════════════");
        print!("\n请输入你的名字: ");
        io::stdout().flush().unwrap();
        Menu::get_input()
    }
}
