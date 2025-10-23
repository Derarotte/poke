//! Pokémon CLI Game - Main Entry Point
//!
//! This is the main entry point for the Pokémon CLI game.
//! All game logic is delegated to the handlers module for clean separation of concerns.

mod game;
mod cli;
mod data;
mod utils;
mod map;
mod npc;
mod pokemon_generator;
mod handlers;

use handlers::GameController;

fn main() {
    // Load game data from JSON files
    match data::loader::load_all_data() {
        Ok(()) => {
            println!("Game data loaded successfully!\n");
            GameController::run();
        }
        Err(e) => {
            eprintln!("Failed to load game data: {}", e);
            std::process::exit(1);
        }
    }
}
