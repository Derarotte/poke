use super::{Pokemon, ItemType, PlayerLocationState};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Badge {
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub level: u32,
    pub pokemons: Vec<Pokemon>,
    pub badges: Vec<Badge>,
    pub items: HashMap<String, u32>,
    pub money: u32,
    pub visited_pokemon_center: bool,
    pub location_state: PlayerLocationState,
}

impl Player {
    pub fn new(name: String) -> Self {
        let mut items = HashMap::new();
        items.insert("Poké Ball".to_string(), 5);
        items.insert("恢复药".to_string(), 3);          // 新增：3个恢复药
        items.insert("超级恢复药".to_string(), 1);      // 新增：1个超级恢复药
        items.insert("全复活".to_string(), 1);          // 新增：1个全复活

        Player {
            name,
            level: 1,
            pokemons: vec![],
            badges: vec![],
            items,
            money: 0,
            visited_pokemon_center: false,
            location_state: PlayerLocationState::default(),
        }
    }

    pub fn add_pokemon(&mut self, pokemon: Pokemon) -> bool {
        if self.pokemons.len() < 6 {
            self.pokemons.push(pokemon);
            true
        } else {
            false
        }
    }

    pub fn get_active_pokemon(&mut self) -> Option<&mut Pokemon> {
        self.pokemons.iter_mut().find(|p| !p.is_fainted())
    }

    pub fn add_item(&mut self, item: String, count: u32) {
        *self.items.entry(item).or_insert(0) += count;
    }

    pub fn use_item(&mut self, item: &str, count: u32) -> bool {
        if let Some(current) = self.items.get_mut(item) {
            if *current >= count {
                *current -= count;
                return true;
            }
        }
        false
    }

    pub fn add_money(&mut self, amount: u32) {
        self.money += amount;
    }

    pub fn has_active_pokemon(&self) -> bool {
        self.pokemons.iter().any(|p| !p.is_fainted())
    }

    pub fn display_team(&self) {
        println!("\n=== 宝可梦队伍 ===");
        for (i, pokemon) in self.pokemons.iter().enumerate() {
            let status = if pokemon.is_fainted() { "[昏迷]" } else { "" };
            println!("{}. {} {}", i + 1, pokemon, status);
        }
    }

    pub fn display_items(&self) {
        println!("\n=== 背包 ===");
        if self.items.is_empty() {
            println!("背包为空");
        } else {
            for (item, count) in self.items.iter() {
                println!("{}: {}", item, count);
            }
        }
        println!("\n金钱: ¥{}", self.money);
    }

    // 复活系统方法

    pub fn get_fainted_pokemon_count(&self) -> usize {
        self.pokemons.iter().filter(|p| p.is_fainted()).count()
    }

    pub fn get_active_pokemon_count(&self) -> usize {
        self.pokemons.iter().filter(|p| !p.is_fainted()).count()
    }

    pub fn all_pokemon_fainted(&self) -> bool {
        !self.has_active_pokemon()
    }

    pub fn revive_pokemon_with_item(&mut self, pokemon_index: usize, item_name: &str) -> Result<String, String> {
        if pokemon_index >= self.pokemons.len() {
            return Err("宝可梦序号无效".to_string());
        }

        // 先检查宝可梦状态，确定效果
        let is_fainted = self.pokemons[pokemon_index].is_fainted();
        let pokemon_name = self.pokemons[pokemon_index].name.clone();
        let max_hp = self.pokemons[pokemon_index].max_hp;

        // 根据道具名称确定复活效果
        let recovery_percent = match item_name {
            "恢复药" => {
                if is_fainted {
                    return Err("需要使用全复活或完全恢复来复活昏迷的宝可梦".to_string());
                }
                0.5  // 50% HP
            },
            "超级恢复药" => {
                if is_fainted {
                    return Err("需要使用全复活或完全恢复来复活昏迷的宝可梦".to_string());
                }
                1.0  // 100% HP
            },
            "全复活" => 0.5,  // 复活 + 50% HP
            "完全恢复" => 1.0,  // 复活 + 100% HP
            _ => return Err(format!("未知的恢复道具: {}", item_name)),
        };

        // 检查是否有该道具
        if !self.use_item(item_name, 1) {
            return Err(format!("{}不足", item_name));
        }

        // 执行复活/治疗
        if item_name == "全复活" || item_name == "完全恢复" {
            self.pokemons[pokemon_index].revive(recovery_percent);
            let hp = self.pokemons[pokemon_index].hp;
            Ok(format!("✓ {}复活了，HP 恢复至 {}/{}", pokemon_name, hp, max_hp))
        } else {
            self.pokemons[pokemon_index].heal((max_hp as f32 * recovery_percent) as u32);
            let hp = self.pokemons[pokemon_index].hp;
            Ok(format!("✓ {}恢复了，HP 现在为 {}/{}", pokemon_name, hp, max_hp))
        }
    }

    pub fn revive_pokemon_at_center(&mut self, pokemon_index: usize) -> Result<String, String> {
        if pokemon_index >= self.pokemons.len() {
            return Err("宝可梦序号无效".to_string());
        }

        let cost = 200;  // 基础复活费用

        // VIP 折扣：已拜访过精灵中心的玩家打 50% 折扣
        let final_cost = if self.visited_pokemon_center { cost / 2 } else { cost };

        if self.money < final_cost {
            return Err(format!("金币不足。需要 ¥{}，但只有 ¥{}", final_cost, self.money));
        }

        self.money -= final_cost;
        self.pokemons[pokemon_index].revive(1.0);  // 100% HP 恢复
        self.visited_pokemon_center = true;

        Ok(format!(
            "✓ 支付 ¥{} 后，{}已完全恢复！(剩余金币: ¥{})",
            final_cost, self.pokemons[pokemon_index].name, self.money
        ))
    }

    pub fn revive_full_team_at_center(&mut self) -> Result<String, String> {
        let fainted_count = self.get_fainted_pokemon_count();
        if fainted_count == 0 {
            return Err("没有昏迷的宝可梦，无需复活".to_string());
        }

        let cost_per_pokemon = 200;
        let total_cost = fainted_count as u32 * cost_per_pokemon;
        let final_cost = if self.visited_pokemon_center { total_cost / 2 } else { total_cost };

        if self.money < final_cost {
            return Err(format!("金币不足。需要 ¥{}，但只有 ¥{}", final_cost, self.money));
        }

        self.money -= final_cost;

        // 复活所有昏迷的宝可梦
        for pokemon in self.pokemons.iter_mut() {
            if pokemon.is_fainted() {
                pokemon.revive(1.0);  // 100% HP 恢复
            }
        }

        self.visited_pokemon_center = true;

        Ok(format!(
            "✓ 支付 ¥{} 后，你的队伍已完全恢复！(剩余金币: ¥{})",
            final_cost, self.money
        ))
    }

    pub fn display_recovery_items(&self) {
        println!("\n=== 恢复道具 ===");
        let recovery_items = vec!["恢复药", "超级恢复药", "全复活", "完全恢复"];
        for (i, item_name) in recovery_items.iter().enumerate() {
            if let Some(count) = self.items.get(*item_name) {
                println!("{}. {} x{}", i + 1, item_name, count);
            }
        }
        println!("0. 返回");
    }

    pub fn display_fainted_pokemon(&self) {
        println!("\n=== 昏迷的宝可梦 ===");
        let mut fainted_count = 0;
        for (_i, pokemon) in self.pokemons.iter().enumerate() {
            if pokemon.is_fainted() {
                fainted_count += 1;
                println!("{}. {} (等级 {})", fainted_count, pokemon.name, pokemon.level);
            }
        }
        if fainted_count == 0 {
            println!("没有昏迷的宝可梦");
        }
    }

    // ==================== 位置系统相关方法 ====================

    /// 检查位置是否满足解锁条件
    pub fn can_unlock_location(&self, requirement: &crate::game::LocationRequirement) -> bool {
        // 检查等级要求
        if !self.pokemons.is_empty() {
            let max_level = self.pokemons.iter().map(|p| p.level).max().unwrap_or(1);
            if max_level < requirement.required_level {
                return false;
            }
        }

        // 检查徽章要求
        if self.badges.len() < requirement.required_badges.len() {
            return false;
        }

        // 检查宝可梦数量要求
        if let Some(required_count) = requirement.required_pokemon_count {
            if self.pokemons.len() < required_count as usize {
                return false;
            }
        }

        true
    }

    /// 获取队伍最高等级
    pub fn get_max_level(&self) -> u32 {
        self.pokemons
            .iter()
            .map(|p| p.level)
            .max()
            .unwrap_or(1)
    }

    /// 检查所有新解锁的位置
    pub fn check_new_unlocks(&mut self, all_locations: &[crate::game::Location]) {
        let mut newly_unlocked = Vec::new();

        for location in all_locations {
            if !self.location_state.is_unlocked(location.id)
                && self.can_unlock_location(&location.unlock_requirement) {
                newly_unlocked.push((location.id, location.name.clone()));
            }
        }

        for (loc_id, loc_name) in newly_unlocked {
            self.location_state.unlock_location(loc_id);
            println!("\n✓ 新地点已解锁: {}!", loc_name);
        }
    }

    /// 获取位置解锁条件的显示文本
    pub fn get_unlock_status(&self, requirement: &crate::game::LocationRequirement) -> String {
        let mut status = Vec::new();

        let max_level = self.get_max_level();
        if requirement.required_level > 1 {
            if max_level >= requirement.required_level {
                status.push(format!("✓ 等级 {} (当前 {})", requirement.required_level, max_level));
            } else {
                status.push(format!("✗ 等级 {} (当前 {})", requirement.required_level, max_level));
            }
        }

        if !requirement.required_badges.is_empty() {
            let needed = requirement.required_badges.len();
            let have = self.badges.len();
            if have >= needed {
                status.push(format!("✓ 徽章 {}/{}", have, needed));
            } else {
                status.push(format!("✗ 徽章 {}/{}", have, needed));
            }
        }

        if let Some(required_count) = requirement.required_pokemon_count {
            if self.pokemons.len() >= required_count as usize {
                status.push(format!("✓ 宝可梦 {}/{}", self.pokemons.len(), required_count));
            } else {
                status.push(format!("✗ 宝可梦 {}/{}", self.pokemons.len(), required_count));
            }
        }

        if status.is_empty() {
            "无条件".to_string()
        } else {
            status.join(" + ")
        }
    }

    /// 从旧存档迁移数据 (Task 8.1)
    /// 为不含位置信息的旧存档添加位置系统数据
    pub fn migrate_from_old_save(mut self) -> Self {
        // 如果已经有位置信息，则无需迁移
        if self.location_state.current_location_id != 0 {
            return self;
        }

        // 初始化位置系统
        // 默认设置玩家在常青小镇 (ID 101)
        self.location_state.current_location_id = 101;
        self.location_state.unlock_location(101);
        self.location_state.mark_visited(101);

        // 根据当前队伍等级检查可以解锁的其他地点
        self.check_new_unlocks(&crate::data::locations_data::get_all_locations());

        self
    }
}
