use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub mod locations;
pub use locations::{create_locations, get_location_by_id, get_locations_by_region, get_all_locations};

use crate::npc::{NPCTrainer, get_npcs_by_location};

/// 代表游戏中的一个地点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub npc_trainers: Vec<u32>,        // NPC ID 列表
    pub wild_pokemon_ids: Vec<u32>,    // 可遭遇的宝可梦 ID
    pub reward_money: u32,             // 该地点对战奖励
    pub level_range: (u32, u32),       // 推荐等级范围
}

/// 代表游戏中的一个区域，包含多个地点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub id: u32,
    pub name: String,
    pub locations: Vec<Location>,
    pub level_range: (u32, u32),       // 推荐等级范围
}

/// 游戏地图系统
pub struct GameMap {
    pub regions: Vec<Region>,
    pub current_location: Option<u32>,
    pub visited_locations: HashSet<u32>,
}

impl GameMap {
    /// 创建新的游戏地图
    pub fn new() -> Self {
        GameMap {
            regions: Vec::new(),
            current_location: None,
            visited_locations: HashSet::new(),
        }
    }

    /// 初始化地图 (从数据源加载)
    pub fn initialize(&mut self, regions: Vec<Region>) {
        self.regions = regions;
    }

    /// 获取指定地点
    pub fn get_location(&self, location_id: u32) -> Option<&Location> {
        for region in &self.regions {
            for location in &region.locations {
                if location.id == location_id {
                    return Some(location);
                }
            }
        }
        None
    }

    /// 移动到新地点
    pub fn move_to_location(&mut self, location_id: u32) -> Result<(), String> {
        if self.get_location(location_id).is_some() {
            self.current_location = Some(location_id);
            self.visited_locations.insert(location_id);
            Ok(())
        } else {
            Err(format!("地点 {} 不存在", location_id))
        }
    }

    /// 获取当前地点
    pub fn get_current_location(&self) -> Option<&Location> {
        self.current_location.and_then(|id| self.get_location(id))
    }

    /// 获取所有区域
    pub fn get_all_regions(&self) -> &[Region] {
        &self.regions
    }

    /// 获取指定区域的所有地点
    pub fn get_locations_in_region(&self, region_id: u32) -> Vec<&Location> {
        self.regions
            .iter()
            .find(|r| r.id == region_id)
            .map(|r| r.locations.iter().collect())
            .unwrap_or_default()
    }

    /// 检查是否已访问某个地点
    pub fn has_visited(&self, location_id: u32) -> bool {
        self.visited_locations.contains(&location_id)
    }

    /// 获取指定地点的所有 NPC
    pub fn get_npcs_at_location(&self, location_id: u32) -> Vec<NPCTrainer> {
        if let Some(location) = self.get_location(location_id) {
            get_npcs_by_location(&location.npc_trainers)
        } else {
            Vec::new()
        }
    }

    /// 获取地点及其关联的 NPC 信息
    pub fn get_location_with_npcs(&self, location_id: u32) -> Option<LocationWithNPCs> {
        if let Some(location) = self.get_location(location_id) {
            let npcs = self.get_npcs_at_location(location_id);
            Some(LocationWithNPCs {
                location: location.clone(),
                npcs,
            })
        } else {
            None
        }
    }

    /// 获取所有可访问的地点 (不同区域)
    pub fn get_accessible_locations(&self) -> Vec<Location> {
        self.get_all_regions()
            .iter()
            .flat_map(|r| r.locations.iter().cloned())
            .collect()
    }

    /// 显示地图信息
    pub fn display_map_info(&self) {
        println!("\n╔═════════════════════════════════════╗");
        println!("║         地图信息                    ║");
        println!("╠═════════════════════════════════════╣");
        println!("║ 区域数: {}                          ║", self.regions.len());

        let total_locations: usize = self.regions.iter().map(|r| r.locations.len()).sum();
        println!("║ 总地点数: {}                        ║", total_locations);
        println!("║ 已访问地点: {}/{}                 ║", self.visited_locations.len(), total_locations);

        if let Some(current_id) = self.current_location {
            if let Some(current) = self.get_location(current_id) {
                println!("║ 当前位置: {}                    ║", current.name);
            }
        }
        println!("╚═════════════════════════════════════╝");
    }
}

/// 包含 NPC 信息的地点
#[derive(Debug, Clone)]
pub struct LocationWithNPCs {
    pub location: Location,
    pub npcs: Vec<NPCTrainer>,
}

impl LocationWithNPCs {
    /// 显示地点详细信息
    pub fn display_info(&self) {
        println!("\n╔═════════════════════════════════════╗");
        println!("║ 地点: {}                        ║", self.location.name);
        println!("╠═════════════════════════════════════╣");
        println!("║ 描述: {}                 ║", self.location.description);
        println!("║ 等级范围: {}-{}                      ║",
                 self.location.level_range.0, self.location.level_range.1);
        println!("║ 对战奖励: ¥{}                        ║", self.location.reward_money);

        if self.npcs.is_empty() {
            println!("║ NPC: 无                             ║");
        } else {
            println!("║ NPC:                                 ║");
            for (i, npc) in self.npcs.iter().enumerate() {
                println!("║   {}. {} {}           ║",
                        i + 1, npc.title, npc.name);
            }
        }
        println!("╚═════════════════════════════════════╝");
    }
}

impl Default for GameMap {
    fn default() -> Self {
        Self::new()
    }
}
