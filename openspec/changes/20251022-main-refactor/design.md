# Architecture Design - Main.rs Refactoring

## Current Architecture Problem

```
main.rs (843 lines)
├── Game loop coordination
├── Movement handling (108-151 lines)
├── Location encounters (152-229 lines)
├── Old exploration (230-287 lines)
├── Capture logic (288-295 lines)
├── Escape logic (296-300 lines)
├── Wild battle system (301-430 lines)
├── Pokemon revival (431-544 lines)
├── Map exploration (546-635 lines)
├── NPC battles (636-684 lines)
└── Team battles (685-843 lines)
```

**Issues**: Mixed concerns, duplicated patterns, hard to extend

## Target Architecture

```
src/
├── main.rs (100 lines)
│   └── Initialize & run GameController
│
└── handlers/
    ├── mod.rs (module exports)
    ├── game_controller.rs (150 lines)
    │   └── Game loop, state transitions, menu routing
    ├── encounter_manager.rs (120 lines)
    │   └── Wild Pokemon encounters, capture flow
    ├── battle_handler.rs (140 lines)
    │   └── Battle execution, turn coordination
    ├── revival_handler.rs (100 lines)
    │   └── Item revival, Pokemon center
    └── exploration_handler.rs (110 lines)
        └── Movement, location logic
```

## Module Responsibilities

### GameController (150 lines)
- Main game loop orchestration
- State transitions (explore → battle → revival)
- Menu routing and user input delegation
- Player state management
- Game exit handling

**Public API**:
```rust
pub struct GameController;
impl GameController {
    pub fn run() -> Result<(), String>
    pub fn new_game(player_name: String) -> Result<Self, String>
    pub fn handle_game_loop(&mut self, player: &mut Player) -> Result<bool, String>
}
```

### EncounterManager (120 lines)
- Wild Pokemon encounter generation
- Encounter preview display
- User action handling (fight/capture/flee)
- Capture rate calculation
- Encounter state machine

**Public API**:
```rust
pub struct EncounterManager;
impl EncounterManager {
    pub fn generate_encounter(location: &Location) -> Result<WildPokemonEncounter, String>
    pub fn handle_encounter_choice(
        choice: &str,
        player: &mut Player,
        opponent: Pokemon,
    ) -> Result<EncounterResult, String>
}

pub enum EncounterResult {
    Captured,
    Battled(BattleOutcome),
    Escaped,
    Failed,
}
```

### BattleHandler (140 lines)
- Battle setup and initialization
- Turn execution loop
- Move validation and execution
- Opponent AI decision making
- Battle result calculation and rewards

**Public API**:
```rust
pub struct BattleHandler;
impl BattleHandler {
    pub fn execute_wild_battle(
        player: &mut Player,
        opponent_team: Vec<Pokemon>,
    ) -> Result<BattleResult, String>

    pub fn execute_npc_battle(
        player: &mut Player,
        npc_team: Vec<Pokemon>,
    ) -> Result<BattleResult, String>

    pub fn handle_battle_turn(
        battle: &mut Battle,
        player_action: BattleAction,
    ) -> Result<(), String>
}

pub enum BattleAction {
    UseMove(usize),
    UseItem(String),
    Switch(usize),
    Escape,
}

pub struct BattleResult {
    pub won: bool,
    pub exp_gained: u32,
    pub money_gained: u32,
}
```

### RevivalHandler (100 lines)
- Pokemon faint detection
- Item-based revival
- Pokemon center mechanics
- Cost calculation and payment
- Team healing logic

**Public API**:
```rust
pub struct RevivalHandler;
impl RevivalHandler {
    pub fn check_and_handle_faint(&mut self, player: &mut Player) -> Result<bool, String>

    pub fn attempt_item_revival(
        player: &mut Player,
        pokemon_index: usize,
        item_name: &str,
    ) -> Result<String, String>

    pub fn attempt_center_revival(
        player: &mut Player,
        pokemon_index: Option<usize>,
    ) -> Result<String, String>
}
```

### ExplorationHandler (110 lines)
- Location movement logic
- Movement validation
- Location unlock checking
- Exploration menu handling
- Location encounter triggering

**Public API**:
```rust
pub struct ExplorationHandler;
impl ExplorationHandler {
    pub fn handle_movement(
        player: &mut Player,
        all_locations: &[Location],
    ) -> Result<(), String>

    pub fn handle_exploration(
        player: &mut Player,
        location: &Location,
    ) -> Result<ExplorationResult, String>
}

pub enum ExplorationResult {
    EncounterTriggered(WildPokemonEncounter),
    NoEncounter,
    LocationChanged(u32),
}
```

## Data Flow

```
main.rs
  ↓
GameController::run()
  ├─→ Display main menu
  ├─→ new_game() → start player
  ├─→ handle_game_loop()
  │    ├─→ ExplorationHandler::handle_exploration()
  │    │    └─→ EncounterManager::generate_encounter()
  │    │         └─→ EncounterManager::handle_encounter_choice()
  │    │              ├─→ BattleHandler::execute_wild_battle()
  │    │              └─→ EncounterManager::attempt_capture()
  │    ├─→ ExplorationHandler::handle_movement()
  │    ├─→ RevivalHandler::check_and_handle_faint()
  │    │    └─→ RevivalHandler::attempt_*_revival()
  │    └─→ repeat until exit
  └─→ exit
```

## Implementation Strategy

### Phase 1: Module Structure
1. Create `src/handlers/` directory
2. Create module skeleton for each handler
3. Export from `src/handlers/mod.rs`
4. Create stub public APIs

### Phase 2: Move Functions
1. Move encounter logic → `EncounterManager`
2. Move battle logic → `BattleHandler`
3. Move revival logic → `RevivalHandler`
4. Move exploration logic → `ExplorationHandler`
5. Move game loop → `GameController`

### Phase 3: Refine APIs
1. Refactor functions into clean methods
2. Create enum types for results
3. Reduce function parameters
4. Add error handling

### Phase 4: Clean main.rs
1. Remove all game logic
2. Keep only initialization and controller call
3. Verify compilation
4. Run all tests

## Key Design Decisions

### 1. Handler Pattern (vs Trait-based)
- **Chosen**: Standalone handler structs with static/impl methods
- **Reason**: Simpler to use, faster compilation, clearer responsibility boundaries
- **Trade-off**: Less flexible for future extension, but clear and maintainable

### 2. Result Types (vs Options)
- **Chosen**: Specific result enums (EncounterResult, BattleResult, etc.)
- **Reason**: Clearer semantics, better error messages
- **Trade-off**: More boilerplate, but better user feedback

### 3. Function Organization
- **Chosen**: Group related functions in impl blocks
- **Reason**: Natural Rust idioms, clear responsibility
- **Trade-off**: Requires careful module boundary definition

### 4. Error Handling
- **Chosen**: Result<T, String> with descriptive messages
- **Reason**: Consistent with existing codebase
- **Trade-off**: Could use custom error types for better type safety

## Testing Strategy

Each handler will be independently testable:

```rust
// Example: BattleHandler tests
#[test]
fn test_execute_wild_battle_victory() { }

#[test]
fn test_execute_npc_battle_defeat() { }

#[test]
fn test_battle_result_experience_calculation() { }

// Example: EncounterManager tests
#[test]
fn test_encounter_generation_from_location() { }

#[test]
fn test_capture_attempt_with_high_rate_pokemon() { }

// Example: RevivalHandler tests
#[test]
fn test_pokemon_center_cost_calculation() { }

#[test]
fn test_item_revival_vip_discount() { }
```

## Migration Path

1. **Step 1**: Create handlers with all functions
2. **Step 2**: main.rs delegates to handlers
3. **Step 3**: Test both old and new paths work identically
4. **Step 4**: Remove old main.rs functions once verified
5. **Step 5**: Deprecate old patterns gradually

## Future Extensibility

This refactoring enables:

1. **New game modes** (tournament, gym battles)
   - Add `TournamentHandler`
   - Extend `BattleHandler` with tournament logic

2. **Multiplayer** (future)
   - Add `NetworkHandler`
   - Extend `BattleHandler` with network I/O

3. **Replay system** (future)
   - Add `ReplayHandler`
   - Store `BattleAction` sequences

4. **Advanced AI** (future)
   - Add `AIHandler` with sophisticated opponent logic
   - Extend `BattleHandler` AI decision making

---

This architecture creates a clean, maintainable foundation for all future development.
