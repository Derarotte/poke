use std::io::{self, Write};
use crate::game::Pokemon;
use crate::data::locations_data;

pub struct TeamDetailMenu;

impl TeamDetailMenu {
    /// 显示单只宝可梦的详细信息
    pub fn display_pokemon_detail(pokemon: &Pokemon) {
        println!("\n╔─────────────────────────────────────────╗");
        println!("║  {:<37} ║", format!("{} (Lv. {})", pokemon.name, pokemon.level));
        println!("╠─────────────────────────────────────────╣");

        // 基本信息
        println!("║ 宝可梦 ID: {:<31} ║", pokemon.id);

        // HP 进度条
        let hp_bar_width = 20;
        let filled = (pokemon.hp as f32 / pokemon.max_hp as f32 * hp_bar_width as f32) as usize;
        let empty = hp_bar_width - filled;
        let hp_bar = format!(
            "[{}{}]",
            "█".repeat(filled),
            "░".repeat(empty)
        );
        println!("║ HP: {}/{:<30} ║", pokemon.hp, pokemon.max_hp);
        println!("║ {:<39} ║", hp_bar);

        // 经验值进度
        let next_level_exp = pokemon.level as u32 * 100;
        let exp_percent = (pokemon.experience as f32 / next_level_exp as f32 * 100.0) as u32;
        println!("║ 经验值: {}/{} ({}%) {:<18} ║",
                 pokemon.experience, next_level_exp, exp_percent, "");

        println!("║                                         ║");

        // 属性信息
        println!("║ 属性信息:                               ║");
        let stats = &pokemon.stats;
        println!("║  HP:      {:<32} ║", stats.hp);
        println!("║  攻击:    {:<32} ║", stats.attack);
        println!("║  防守:    {:<32} ║", stats.defense);
        println!("║  特攻:    {:<32} ║", stats.sp_attack);
        println!("║  特防:    {:<32} ║", stats.sp_defense);
        println!("║  速度:    {:<32} ║", stats.speed);

        println!("║                                         ║");

        // 招式信息
        println!("║ 招式:                                   ║");
        if pokemon.moves.is_empty() {
            println!("║  (暂无招式)                             ║");
        } else {
            for (i, m) in pokemon.moves.iter().enumerate() {
                let move_info = format!(
                    "{}. {} ({:?})",
                    i + 1,
                    m.name,
                    m.pokemon_type
                );
                println!("║  {:<35} ║", move_info);
                let pp_info = format!("     Power: {}, Accuracy: {}%, PP: {}/{}",
                                     m.power, m.accuracy, m.pp, m.max_pp);
                println!("║  {:<35} ║", pp_info);
            }
        }

        println!("║                                         ║");

        // 捕捉信息
        println!("║ 捕捉信息:                               ║");
        println!("║  捕捉球:  {:<32} ║", pokemon.caught_with);

        // 获取捕捉地点名称
        let location_name = locations_data::get_location_by_id(pokemon.caught_location_id)
            .map(|loc| loc.name)
            .unwrap_or_else(|| format!("未知地点 (ID: {})", pokemon.caught_location_id));
        println!("║  地点:    {:<32} ║", location_name);

        // 捕捉日期
        if pokemon.caught_date > 0 {
            // 简化显示，实际可以转换为日期格式
            println!("║  日期:    {:<32} ║", format!("时间戳: {}", pokemon.caught_date));
        } else {
            println!("║  日期:    {:<32} ║", "未记录");
        }

        println!("╠─────────────────────────────────────────╣");
        println!("║ 选项: (R)返回  (M)更多操作              ║");
        println!("╚─────────────────────────────────────────╝");
    }

    /// 显示所有队伍宝可梦的简要列表
    pub fn display_team_list(team: &[Pokemon], current_index: usize) {
        println!("\n╔─────────────────────────────────────────╗");
        println!("║     宝可梦队伍 (总数: {}/6)           ║", team.len());
        println!("╠─────────────────────────────────────────╣");

        for (i, pokemon) in team.iter().enumerate() {
            let marker = if i == current_index { "→" } else { " " };
            let status = if pokemon.is_fainted() { "[昏迷]" } else { "[正常]" };
            let line = format!(
                "{}{}. {:<20} Lv.{:<3} {}",
                marker,
                i + 1,
                pokemon.name,
                pokemon.level,
                status
            );
            println!("║ {:<39} ║", line);
        }

        if team.is_empty() {
            println!("║ (队伍为空)                              ║");
        }

        println!("╠─────────────────────────────────────────╣");
        println!("║ 1-6: 查看详情  S: 存入仓库  R: 返回     ║");
        println!("╚─────────────────────────────────────────╝");
    }

    /// 获取用户输入
    pub fn get_input() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    /// 显示操作菜单
    pub fn display_options_menu() {
        println!("\n╔─────────────────────────────────────────╗");
        println!("║  宝可梦操作菜单                         ║");
        println!("╠─────────────────────────────────────────╣");
        println!("║ 1. 查看详细信息                         ║");
        println!("║ 2. 放入宠物仓库                         ║");
        println!("║ 3. 返回队伍列表                         ║");
        println!("║ 4. 返回主菜单                           ║");
        println!("╚─────────────────────────────────────────╝");
        print!("请选择: ");
        io::stdout().flush().unwrap();
    }

    /// 显示属性对比（用于仓库切换时）
    pub fn display_pokemon_comparison(current: &Pokemon, alternative: &Pokemon) {
        println!("\n╔─────────────────────────────────────────╗");
        println!("║     宝可梦对比                         ║");
        println!("╠──────────────────────┬──────────────────╣");
        println!("║ 属性         │ 当前  │ 对比             ║");
        println!("╠──────────────────────┼──────────────────╣");
        println!("║ 名字: {:<9} │ {:<5} │ {:<14} ║", "", current.name, alternative.name);
        println!("║ 等级: {:<9} │ {:<5} │ {:<14} ║", "", current.level, alternative.level);
        println!("║ HP:   {:<9} │ {:<5} │ {:<14} ║", "", current.hp, alternative.hp);
        println!("╚──────────────────────┴──────────────────╝");
    }
}
