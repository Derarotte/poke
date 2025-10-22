use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ItemType {
    PokeBall,         // 精灵球
    Potion,           // 基础恢复药 (50% HP)
    SuperPotion,      // 超级恢复药 (100% HP)
    Revive,           // 全复活药 (复活 + 50% HP)
    FullRestore,      // 完全恢复 (复活 + 100% HP)
    Antidote,         // 解毒药 (未来用)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub item_type: ItemType,
    pub name: String,
    pub description: String,
    pub price: u32,
}

impl Item {
    pub fn new(item_type: ItemType) -> Self {
        match item_type {
            ItemType::PokeBall => Item {
                item_type,
                name: "Poké Ball".to_string(),
                description: "用来捕捉宝可梦的道具".to_string(),
                price: 100,
            },
            ItemType::Potion => Item {
                item_type,
                name: "恢复药".to_string(),
                description: "恢复宝可梦 50% 的 HP".to_string(),
                price: 150,
            },
            ItemType::SuperPotion => Item {
                item_type,
                name: "超级恢复药".to_string(),
                description: "完全恢复宝可梦的 HP".to_string(),
                price: 250,
            },
            ItemType::Revive => Item {
                item_type,
                name: "全复活".to_string(),
                description: "让昏迷的宝可梦复活，恢复 50% HP".to_string(),
                price: 200,
            },
            ItemType::FullRestore => Item {
                item_type,
                name: "完全恢复".to_string(),
                description: "让昏迷的宝可梦复活，完全恢复 HP".to_string(),
                price: 400,
            },
            ItemType::Antidote => Item {
                item_type,
                name: "解毒药".to_string(),
                description: "解除宝可梦的中毒状态".to_string(),
                price: 100,
            },
        }
    }

    pub fn get_hp_recovery(&self) -> Option<f32> {
        match self.item_type {
            ItemType::Potion => Some(0.5),          // 50% HP
            ItemType::SuperPotion => Some(1.0),     // 100% HP
            ItemType::Revive => Some(0.5),          // 复活 + 50% HP
            ItemType::FullRestore => Some(1.0),     // 复活 + 100% HP
            _ => None,
        }
    }

    pub fn is_revive_item(&self) -> bool {
        matches!(self.item_type, ItemType::Revive | ItemType::FullRestore)
    }

    pub fn is_recovery_item(&self) -> bool {
        matches!(
            self.item_type,
            ItemType::Potion | ItemType::SuperPotion | ItemType::Revive | ItemType::FullRestore
        )
    }
}

// 便捷函数：获取所有道具类型
pub fn get_all_item_types() -> Vec<ItemType> {
    vec![
        ItemType::PokeBall,
        ItemType::Potion,
        ItemType::SuperPotion,
        ItemType::Revive,
        ItemType::FullRestore,
        ItemType::Antidote,
    ]
}

// 获取道具名称
pub fn get_item_name(item_type: ItemType) -> String {
    Item::new(item_type).name
}
