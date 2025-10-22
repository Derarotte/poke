use crate::game::Player;
use std::fs;
use std::path::Path;

pub struct SaveManager;

impl SaveManager {
    pub fn save_game(player: &Player, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(player)?;
        fs::write(filename, json)?;
        println!("游戏已保存!");
        Ok(())
    }

    pub fn load_game(filename: &str) -> Result<Player, Box<dyn std::error::Error>> {
        if !Path::new(filename).exists() {
            return Err("存档文件不存在".into());
        }

        let json = fs::read_to_string(filename)?;
        let player = serde_json::from_str(&json)?;
        println!("游戏已加载!");
        Ok(player)
    }

    pub fn delete_save(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        fs::remove_file(filename)?;
        println!("存档已删除!");
        Ok(())
    }
}
