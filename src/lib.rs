// 宝可梦游戏库

pub mod game;
pub mod cli;
pub mod data;
pub mod utils;
pub mod map;
pub mod npc;
pub mod pokemon_generator;

// 重新导出主要类型供测试使用
pub use game::{Player, Pokemon, Battle};
pub use map::{GameMap, Location, Region};
pub use npc::{NPCTrainer, Difficulty};
pub use pokemon_generator::{IndividualValues, Talent, Nature, PokemonInstance};
