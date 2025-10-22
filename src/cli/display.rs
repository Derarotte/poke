use crate::game::Pokemon;

pub fn clear_screen() {
    println!("{}", "\n".repeat(1));
}

pub fn print_separator() {
    println!("═══════════════════════════════════════");
}

pub fn print_pokemon_info(pokemon: &Pokemon) {
    println!("\n┌─────────────────────────────────┐");
    println!("│ 宝可梦信息:                     │");
    println!("├─────────────────────────────────┤");
    println!("│ 名字: {:<22} │", pokemon.name);
    println!("│ 等级: {:<22} │", pokemon.level);
    println!("│ HP: {}/{:<20} │", pokemon.hp, pokemon.max_hp);
    println!("│ 类型: {:?}/{:?}", pokemon.pokemon_type.0, pokemon.pokemon_type.1);
    println!("│ 攻击: {:<22} │", pokemon.get_effective_stat("attack", pokemon.level));
    println!("│ 防御: {:<22} │", pokemon.get_effective_stat("defense", pokemon.level));
    println!("│ 速度: {:<22} │", pokemon.get_effective_stat("speed", pokemon.level));
    println!("└─────────────────────────────────┘");
}

pub fn print_battle_status(player_pokemon: &Pokemon, opponent_pokemon: &Pokemon) {
    println!("\n╔═════════════════════════════════════╗");
    println!("║           对战状态                  ║");
    println!("╠═════════════════════════════════════╣");
    println!("║ 你的宝可梦: {:<20} ║", player_pokemon.name);
    println!("║ HP: {}/{:<27} ║", player_pokemon.hp, player_pokemon.max_hp);

    // HP 条
    let hp_percentage = (player_pokemon.hp as f32 / player_pokemon.max_hp as f32) * 20.0;
    let hp_bar = "█".repeat(hp_percentage as usize);
    let empty_bar = "░".repeat(20 - hp_percentage as usize);
    println!("║ [{}{}] ║", hp_bar, empty_bar);

    println!("║                                     ║");
    println!("║ 对手宝可梦: {:<18} ║", opponent_pokemon.name);
    println!("║ HP: {}/{:<27} ║", opponent_pokemon.hp, opponent_pokemon.max_hp);

    let opp_hp_percentage = (opponent_pokemon.hp as f32 / opponent_pokemon.max_hp as f32) * 20.0;
    let opp_hp_bar = "█".repeat(opp_hp_percentage as usize);
    let opp_empty_bar = "░".repeat(20 - opp_hp_percentage as usize);
    println!("║ [{}{}] ║", opp_hp_bar, opp_empty_bar);

    println!("╚═════════════════════════════════════╝");
}

pub fn print_battle_message(message: &str) {
    println!("\n[对战消息]: {}", message);
}

pub fn print_capture_result(success: bool, pokemon_name: &str) {
    if success {
        println!("\n✓ 太好了! 成功捕捉到了 {}!", pokemon_name);
    } else {
        println!("\n✗ 哎呀! {} 挣脱了精灵球!", pokemon_name);
    }
}

pub fn print_escape_result(success: bool) {
    if success {
        println!("\n✓ 成功逃离了战斗!");
    } else {
        println!("\n✗ 逃跑失败了!");
    }
}

pub fn print_level_up(pokemon: &Pokemon) {
    println!("\n╔═════════════════════════════════════╗");
    println!("║       {}升级了！       ║", pokemon.name);
    println!("╠═════════════════════════════════════╣");
    println!("║ 现在等级: {:<22} ║", pokemon.level);
    println!("║ 当前HP: {}/{:<21} ║", pokemon.hp, pokemon.max_hp);
    println!("╚═════════════════════════════════════╝");
}

pub fn print_team_full() {
    println!("\n✗ 队伍已满！无法捕捉更多宝可梦!");
}

pub fn print_no_balls() {
    println!("\n✗ 精灵球不足！");
}

pub fn print_game_over() {
    println!("\n╔═════════════════════════════════════╗");
    println!("║         你的队伍全部昏迷了！       ║");
    println!("║           游戏结束                  ║");
    println!("╚═════════════════════════════════════╝");
}

pub fn print_victory() {
    println!("\n╔═════════════════════════════════════╗");
    println!("║        你赢得了这场战斗!           ║");
    println!("╚═════════════════════════════════════╝");
}

pub fn print_defeat() {
    println!("\n╔═════════════════════════════════════╗");
    println!("║        你输掉了这场战斗!           ║");
    println!("╚═════════════════════════════════════╝");
}
