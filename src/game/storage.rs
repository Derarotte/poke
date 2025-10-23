use crate::game::Pokemon;
use serde::{Deserialize, Serialize};

/// 宝可梦存储箱
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonBox {
    pub box_id: u32,
    pub name: String,
    pub pokemon: Vec<Pokemon>,
}

impl PokemonBox {
    pub fn new(box_id: u32) -> Self {
        PokemonBox {
            box_id,
            name: format!("箱子 {}", box_id),
            pokemon: Vec::new(),
        }
    }

    pub fn is_full(&self) -> bool {
        self.pokemon.len() >= 30
    }

    pub fn is_empty(&self) -> bool {
        self.pokemon.is_empty()
    }

    pub fn add_pokemon(&mut self, pokemon: Pokemon) -> Result<(), String> {
        if self.is_full() {
            Err("箱子已满 (30/30)".to_string())
        } else {
            self.pokemon.push(pokemon);
            Ok(())
        }
    }

    pub fn remove_pokemon(&mut self, index: usize) -> Result<Pokemon, String> {
        if index >= self.pokemon.len() {
            Err("宝可梦索引超出范围".to_string())
        } else {
            Ok(self.pokemon.remove(index))
        }
    }

    pub fn get_pokemon(&self, index: usize) -> Option<&Pokemon> {
        self.pokemon.get(index)
    }

    pub fn rename(&mut self, new_name: String) {
        self.name = new_name;
    }
}

/// 宠物仓库系统 - 20 个箱子，每个最多 30 只宝可梦
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSystem {
    pub boxes: Vec<PokemonBox>,
}

impl StorageSystem {
    /// 创建新的存储系统（初始化 20 个箱子）
    pub fn new() -> Self {
        let mut boxes = Vec::new();
        for i in 1..=20 {
            boxes.push(PokemonBox::new(i));
        }
        StorageSystem { boxes }
    }

    /// 添加宝可梦到第一个有空位的箱子
    pub fn add_pokemon(&mut self, pokemon: Pokemon) -> Result<(u32, usize), String> {
        for box_index in 0..self.boxes.len() {
            if !self.boxes[box_index].is_full() {
                self.boxes[box_index].add_pokemon(pokemon)?;
                return Ok((self.boxes[box_index].box_id, self.boxes[box_index].pokemon.len() - 1));
            }
        }
        Err("所有箱子都已满！".to_string())
    }

    /// 从指定箱子移除宝可梦
    pub fn remove_pokemon(&mut self, box_id: u32, index: usize) -> Result<Pokemon, String> {
        let box_index = self.find_box_index(box_id)?;
        self.boxes[box_index].remove_pokemon(index)
    }

    /// 放生（删除）宝可梦
    pub fn release_pokemon(&mut self, box_id: u32, index: usize) -> Result<String, String> {
        let pokemon = self.remove_pokemon(box_id, index)?;
        Ok(format!("你放生了 {}!", pokemon.name))
    }

    /// 获取指定箱子的宝可梦列表
    pub fn get_box_pokemon(&self, box_id: u32) -> Result<&[Pokemon], String> {
        let box_index = self.find_box_index(box_id)?;
        Ok(&self.boxes[box_index].pokemon)
    }

    /// 获取指定箱子的可变引用
    pub fn get_box_mut(&mut self, box_id: u32) -> Result<&mut PokemonBox, String> {
        let box_index = self.find_box_index(box_id)?;
        Ok(&mut self.boxes[box_index])
    }

    /// 获取指定箱子的不可变引用
    pub fn get_box(&self, box_id: u32) -> Result<&PokemonBox, String> {
        let box_index = self.find_box_index(box_id)?;
        Ok(&self.boxes[box_index])
    }

    /// 重命名箱子
    pub fn rename_box(&mut self, box_id: u32, new_name: String) -> Result<(), String> {
        let box_index = self.find_box_index(box_id)?;
        self.boxes[box_index].rename(new_name);
        Ok(())
    }

    /// 获取仓库统计信息
    pub fn get_statistics(&self) -> StorageStats {
        let mut total_pokemon = 0;
        let mut used_boxes = 0;
        let mut empty_boxes = 0;

        for box_item in &self.boxes {
            total_pokemon += box_item.pokemon.len();
            if box_item.is_empty() {
                empty_boxes += 1;
            } else {
                used_boxes += 1;
            }
        }

        StorageStats {
            total_pokemon,
            used_boxes,
            empty_boxes,
            total_boxes: 20,
        }
    }

    /// 查询宝可梦（按名字）
    pub fn search_pokemon(&self, name: &str) -> Vec<(u32, usize, &Pokemon)> {
        let mut results = Vec::new();
        for box_item in &self.boxes {
            for (i, pokemon) in box_item.pokemon.iter().enumerate() {
                if pokemon.name.to_lowercase().contains(&name.to_lowercase()) {
                    results.push((box_item.box_id, i, pokemon));
                }
            }
        }
        results
    }

    /// 获取所有稀有宝可梦（等级 >= 50）
    pub fn get_rare_pokemon(&self) -> Vec<(u32, usize, &Pokemon)> {
        let mut results = Vec::new();
        for box_item in &self.boxes {
            for (i, pokemon) in box_item.pokemon.iter().enumerate() {
                if pokemon.level >= 50 {
                    results.push((box_item.box_id, i, pokemon));
                }
            }
        }
        results
    }

    // 私有辅助方法
    fn find_box_index(&self, box_id: u32) -> Result<usize, String> {
        if box_id < 1 || box_id > 20 {
            return Err(format!("箱子 ID {} 无效 (范围: 1-20)", box_id));
        }
        Ok((box_id - 1) as usize)
    }
}

impl Default for StorageSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// 仓库统计信息
#[derive(Debug, Clone)]
pub struct StorageStats {
    pub total_pokemon: usize,
    pub used_boxes: usize,
    pub empty_boxes: usize,
    pub total_boxes: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_system_creation() {
        let storage = StorageSystem::new();
        assert_eq!(storage.boxes.len(), 20);
    }

    #[test]
    fn test_pokemon_box_capacity() {
        let mut box_item = PokemonBox::new(1);
        assert!(!box_item.is_full());
        assert_eq!(box_item.pokemon.len(), 0);
    }

    #[test]
    fn test_get_statistics() {
        let storage = StorageSystem::new();
        let stats = storage.get_statistics();
        assert_eq!(stats.total_pokemon, 0);
        assert_eq!(stats.empty_boxes, 20);
        assert_eq!(stats.used_boxes, 0);
    }
}
