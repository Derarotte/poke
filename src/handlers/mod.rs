//! Game handlers module - Separates concerns into focused, testable modules
//!
//! This module provides the core orchestration and specialized handlers for the game:
//! - GameController: Main game loop and state transitions
//! - EncounterManager: Wild Pokemon encounters and capture flow
//! - BattleHandler: Battle execution and turn management
//! - RevivalHandler: Pokemon faint detection and revival mechanics
//! - ExplorationHandler: Location movement and exploration

pub mod game_controller;
pub mod encounter_manager;
pub mod battle_handler;
pub mod revival_handler;
pub mod exploration_handler;

// Re-export public APIs
pub use game_controller::GameController;
pub use encounter_manager::{EncounterManager, EncounterResult};
pub use battle_handler::{BattleHandler, BattleResult, BattleAction};
pub use revival_handler::RevivalHandler;
pub use exploration_handler::{ExplorationHandler, ExplorationResult};
