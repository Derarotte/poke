# Implementation Tasks - Main.rs Refactoring

**Total Effort**: ~4-5 hours
**Dependencies**: None (refactoring only, no new features)
**Priority**: HIGH (code quality and maintainability)

## Phase 1: Module Structure Setup

### Task 1.1: Create handlers directory and module structure
- **Effort**: 30 minutes
- **Acceptance**:
  - ✅ `src/handlers/mod.rs` created
  - ✅ All 5 handler modules stubbed
  - ✅ Basic public API structure defined
  - ✅ Code compiles without errors
- **Details**:
  - Create `src/handlers/` directory
  - Create `src/handlers/mod.rs` with module declarations
  - Create empty implementations for all 5 handlers
  - Export public APIs

## Phase 2: Move Encounter Logic

### Task 2.1: Create EncounterManager
- **Effort**: 45 minutes
- **Acceptance**:
  - ✅ All encounter-related functions moved
  - ✅ `EncounterManager` public API complete
  - ✅ `EncounterResult` enum defined
  - ✅ No code duplication with old implementation
  - ✅ Tests pass (old + new paths work identically)
- **Details**:
  - Move `explore_location_encounter()` logic
  - Move `WildPokemonEncounter` handling
  - Create result enum for encounter outcomes
  - Refactor duplicate encounter display code
  - Add unit tests for encounter generation

### Task 2.2: Create legacy encounter compatibility layer
- **Effort**: 15 minutes
- **Acceptance**:
  - ✅ Old encounter code calls new handler
  - ✅ Main.rs still compiles and runs
  - ✅ No behavior changes
- **Details**:
  - Keep old `explore_location_encounter()` as wrapper
  - Delegate to `EncounterManager`
  - This allows gradual transition

## Phase 3: Move Battle Logic

### Task 3.1: Create BattleHandler
- **Effort**: 1 hour
- **Acceptance**:
  - ✅ All battle-related functions moved
  - ✅ `BattleHandler` public API complete
  - ✅ `BattleResult` enum defined
  - ✅ `BattleAction` enum defined
  - ✅ Wild battle logic extracted
  - ✅ NPC battle logic extracted
  - ✅ Tests pass for both battle types
- **Details**:
  - Move `handle_team_battle()` logic
  - Move `battle_wild()` logic (old single-Pokemon version)
  - Move `battle_npc_menu()` logic
  - Move `handle_opponent_action()` logic
  - Unify battle loop logic
  - Create `BattleAction` enum for player actions
  - Add comprehensive unit tests

### Task 3.2: Refactor battle coordination
- **Effort**: 30 minutes
- **Acceptance**:
  - ✅ Battle loop refactored with clean turn structure
  - ✅ Opponent AI extracted to separate method
  - ✅ Battle result calculation centralized
  - ✅ Experience and money distribution in handler
- **Details**:
  - Extract turn execution logic
  - Create `execute_battle_turn()` method
  - Implement simple opponent AI
  - Centralize reward calculation

## Phase 4: Move Revival Logic

### Task 4.1: Create RevivalHandler
- **Effort**: 45 minutes
- **Acceptance**:
  - ✅ All revival-related functions moved
  - ✅ `RevivalHandler` public API complete
  - ✅ Pokemon faint detection logic extracted
  - ✅ Item revival logic extracted
  - ✅ Pokemon center logic extracted
  - ✅ Tests pass for all revival types
- **Details**:
  - Move `handle_all_pokemon_fainted()` logic
  - Move `handle_item_revival()` logic
  - Move `handle_pokemon_center_revival()` logic
  - Create `check_and_handle_faint()` method
  - Extract VIP discount calculation
  - Add unit tests for cost calculation

## Phase 5: Move Exploration Logic

### Task 5.1: Create ExplorationHandler
- **Effort**: 45 minutes
- **Acceptance**:
  - ✅ All exploration-related functions moved
  - ✅ `ExplorationHandler` public API complete
  - ✅ `ExplorationResult` enum defined
  - ✅ Movement validation logic extracted
  - ✅ Location unlock checking extracted
  - ✅ Tests pass for movement and location logic
- **Details**:
  - Move `handle_movement()` logic
  - Move `explore_location_encounter()` coordination
  - Move `explore_region()` and `explore_location()` logic
  - Move `explore_map()` logic
  - Extract location availability checking
  - Add unit tests for movement validation

## Phase 6: Move Game Loop Logic

### Task 6.1: Create GameController
- **Effort**: 1 hour
- **Acceptance**:
  - ✅ Game loop logic extracted
  - ✅ State transitions handled
  - ✅ Menu routing delegated to handlers
  - ✅ `GameController` public API complete
  - ✅ Main game loop <50 lines
  - ✅ All tests pass
- **Details**:
  - Move `game_loop()` logic
  - Move `start_game()` logic
  - Create `handle_game_loop()` method
  - Implement state machine for game flow
  - Delegate to appropriate handlers based on user choice
  - Extract menu display logic calls

## Phase 7: Refactor main.rs

### Task 7.1: Clean up main.rs
- **Effort**: 30 minutes
- **Acceptance**:
  - ✅ main.rs reduced to <150 lines
  - ✅ Only entry point logic remains
  - ✅ All imports from handlers module
  - ✅ No game logic in main.rs
  - ✅ Code compiles and runs
- **Details**:
  - Remove all handler functions
  - Remove all imports except handlers
  - Keep only `main()` and `start_game()` as wrappers
  - Update imports to use handlers module
  - Verify no code duplication

### Task 7.2: Update lib.rs exports
- **Effort**: 15 minutes
- **Acceptance**:
  - ✅ All handlers exported from lib.rs
  - ✅ Library API surface clear
  - ✅ Future testing easier
- **Details**:
  - Export handlers module
  - Export all public handler APIs
  - Create convenience re-exports if needed

## Phase 8: Testing & Validation

### Task 8.1: Run all tests
- **Effort**: 30 minutes
- **Acceptance**:
  - ✅ All 29 existing tests pass
  - ✅ No regressions
  - ✅ `cargo test` runs cleanly
  - ✅ `cargo build` succeeds
- **Details**:
  - Run full test suite
  - Verify no behavior changes
  - Check for compiler warnings
  - Verify example gameplay

### Task 8.2: Add handler integration tests
- **Effort**: 45 minutes
- **Acceptance**:
  - ✅ At least 10 new integration tests
  - ✅ Test battle → revival → exploration flow
  - ✅ Test NPC battles
  - ✅ Test encounter generation
  - ✅ All tests pass
- **Details**:
  - Create integration test for full game flow
  - Test encounter → battle → victory flow
  - Test encounter → battle → defeat → revival flow
  - Test movement between locations
  - Test revival at Pokemon center

## Phase 9: Documentation & Cleanup

### Task 9.1: Update code documentation
- **Effort**: 30 minutes
- **Acceptance**:
  - ✅ All handlers documented with doc comments
  - ✅ Public APIs clearly explained
  - ✅ Example usage provided
- **Details**:
  - Add doc comments to all public methods
  - Add module-level documentation
  - Provide usage examples

### Task 9.2: Remove old code
- **Effort**: 15 minutes
- **Acceptance**:
  - ✅ No dead code remains
  - ✅ No duplicate implementations
  - ✅ Clean git history
- **Details**:
  - Remove old wrapper functions (if any)
  - Remove any commented-out code
  - Verify imports are clean

## Task Dependencies

```
1.1 (Setup) → 2.1 (Encounters) → 3.1 (Battles) → 4.1 (Revival) → 5.1 (Exploration) → 6.1 (GameController)
                                                                                          ↓
                                                                                      7.1 (Cleanup)
                                                                                          ↓
                                                                                      8.1 (Testing)
                                                                                          ↓
                                                                                      9.1 (Docs)
```

**Note**: Tasks within each phase can be parallelized

## Success Metrics

- ✅ main.rs reduced from 843 to <150 lines (83% reduction)
- ✅ 5 handler modules created with clear responsibilities
- ✅ All 29 existing tests pass
- ✅ 10+ new integration tests added
- ✅ Code compiles without warnings
- ✅ No behavioral changes (backward compatible)
- ✅ Handler modules average <150 lines each
- ✅ Public APIs are clear and well-documented

## Rollback Plan

If any issues arise:

1. All changes are in new `src/handlers/` directory
2. Old main.rs functions can remain as wrappers
3. Can revert to previous state by removing handlers/ directory
4. No breaking changes to external interfaces

---

**Estimated Timeline**: 4-5 hours total
**Recommended**: Complete over 1-2 days with testing
