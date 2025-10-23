use crate::game::{Battle, Pokemon, BattleStatus};

pub struct BattleMenu;

impl BattleMenu {
    /// 显示完整的战斗界面
    pub fn display_battle_screen(battle: &Battle) {
        println!("\n{}", "=".repeat(80));
        BattleMenu::display_opponent_pokemon(battle);
        println!();
        BattleMenu::display_player_pokemon(battle);
        println!("{}", "=".repeat(80));
    }

    /// 显示对手宝可梦信息
    pub fn display_opponent_pokemon(battle: &Battle) {
        if let Some(opponent) = battle.get_opponent_pokemon() {
            let status = if opponent.is_fainted() { "[昏迷]" } else { "" };
            println!("对手的宝可梦: {} (Lv.{}) {}", opponent.name, opponent.level, status);
            BattleMenu::display_hp_bar(&opponent, false);
        }
    }

    /// 显示玩家宝可梦信息
    pub fn display_player_pokemon(battle: &Battle) {
        if let Some(player) = battle.get_player_pokemon() {
            let status = if player.is_fainted() { "[昏迷]" } else { "" };
            println!("你的宝可梦: {} (Lv.{}) {}", player.name, player.level, status);
            BattleMenu::display_hp_bar(&player, true);
        }
    }

    /// 显示 HP 进度条
    fn display_hp_bar(pokemon: &Pokemon, _show_numbers: bool) {
        let max_hp = pokemon.max_hp;
        let current_hp = pokemon.hp;
        let bar_length = 20;
        let filled = (current_hp as f32 / max_hp as f32 * bar_length as f32) as usize;
        let empty = bar_length - filled;

        let bar = format!(
            "HP: [{}{}] {}/{}",
            "█".repeat(filled),
            "░".repeat(empty),
            current_hp,
            max_hp
        );

        println!("{}", bar);
    }

    /// 显示主菜单（选择动作）
    pub fn display_main_menu(battle: &Battle) {
        println!("\n--- 选择你的动作 ---");
        println!("1. 使用招式");
        println!("2. 使用道具");
        println!("3. 切换宝可梦");
        if battle.is_wild_battle {
            println!("4. 逃脱");
        }
        println!("0. 退出");
        print!("选择 (0-{}): ", if battle.is_wild_battle { 4 } else { 3 });
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }

    /// 显示招式菜单
    pub fn display_move_menu(battle: &Battle) {
        if let Some(player) = battle.get_player_pokemon() {
            println!("\n--- 选择招式 ---");
            for (i, mv) in player.moves.iter().enumerate() {
                let pp_color = if mv.pp == 0 { "❌" } else { "✓" };
                println!("{}: {} (Pow:{}, PP:{}/{}) {}", i + 1, mv.name, mv.power, mv.pp, mv.max_pp, pp_color);
            }
            println!("0. 返回");
            print!("选择 (0-{}): ", player.moves.len());
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }
    }

    /// 显示道具菜单
    pub fn display_item_menu(items: &std::collections::HashMap<String, u32>) {
        println!("\n--- 可用的道具 ---");

        let recovery_items = vec!["恢复药", "超级恢复药", "全复活", "完全恢复"];
        let mut item_count = 0;

        for (_i, item_name) in recovery_items.iter().enumerate() {
            if let Some(count) = items.get(*item_name) {
                if *count > 0 {
                    item_count += 1;
                    println!("{}: {} x{}", item_count, item_name, count);
                }
            }
        }

        if item_count == 0 {
            println!("没有可用的恢复道具");
        }

        println!("0. 返回");
        print!("选择 (0-{}): ", item_count);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }

    /// 显示切换宝可梦菜单
    pub fn display_switch_menu(battle: &Battle) {
        println!("\n--- 选择切换的宝可梦 ---");
        let mut valid_count = 0;

        for (i, pokemon) in battle.player_team.iter().enumerate() {
            if !pokemon.is_fainted() && i != battle.player_current_index {
                valid_count += 1;
                let status = if pokemon.is_fainted() { "[昏迷]" } else { "✓" };
                println!("{}: {} (Lv.{}) {}", valid_count, pokemon.name, pokemon.level, status);
            }
        }

        if valid_count == 0 {
            println!("没有可用的宝可梦进行切换");
        }

        println!("0. 返回");
        print!("选择 (0-{}): ", valid_count);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }

    /// 显示战斗日志
    pub fn display_battle_log(battle: &Battle, recent_count: usize) {
        println!("\n--- 战斗历史 ---");
        let start = if battle.battle_log.len() > recent_count {
            battle.battle_log.len() - recent_count
        } else {
            0
        };

        for log in &battle.battle_log[start..] {
            println!("  {}", log);
        }
    }

    /// 显示战斗结果
    pub fn display_battle_result(battle: &Battle, reward_money: u32) {
        println!("\n{}", "=".repeat(80));
        match battle.status {
            BattleStatus::PlayerWon => {
                println!("🎉 恭喜！你赢了战斗！");
                println!("获得金币: ¥{}", reward_money);
            }
            BattleStatus::PlayerLost => {
                println!("😢 你输了战斗...");
                println!("金币扣除: ¥{}", reward_money / 2);
            }
            BattleStatus::Escaped => {
                println!("✓ 成功逃脱战斗！");
            }
            BattleStatus::Active => {
                println!("战斗进行中...");
            }
        }
        println!("{}", "=".repeat(80));
    }

    /// 显示战斗摘要
    pub fn display_battle_summary(battle: &Battle) {
        println!("\n--- 战斗摘要 ---");
        println!("回合数: {}", battle.turn);
        println!("战斗状态: {:?}", battle.status);

        if let Some(player) = battle.get_player_pokemon() {
            println!("你的宝可梦: {} (HP: {}/{})", player.name, player.hp, player.max_hp);
        }

        if let Some(opponent) = battle.get_opponent_pokemon() {
            println!("对手宝可梦: {} (HP: {}/{})", opponent.name, opponent.hp, opponent.max_hp);
        }
    }

    /// 读取用户输入
    pub fn read_input() -> String {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        input.trim().to_string()
    }

    /// 显示准备战斗的界面
    pub fn display_battle_start(opponent_name: &str, is_wild: bool) {
        let opponent_type = if is_wild { "野生" } else { "训练师的" };
        println!("\n{}", "=".repeat(80));
        println!("⚔️  {}{}出现了！", opponent_type, opponent_name);
        println!("{}", "=".repeat(80));
    }

    /// 清屏
    pub fn clear_screen() {
        #[cfg(target_os = "windows")]
        {
            let _ = std::process::Command::new("cls").status();
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = std::process::Command::new("clear").status();
        }
    }
}
