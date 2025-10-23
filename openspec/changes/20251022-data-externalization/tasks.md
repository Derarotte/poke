# Data Externalization - Implementation Tasks

**Change ID**: 20251022-data-externalization
**Total Tasks**: 24
**Estimated Duration**: 5-7 days

## Phase 1: Infrastructure & Data Structures (Days 1-2)

### Task 1.1: Create data loader module
- **Description**: Create `src/data/loader.rs` with JSON loading functions
- **Acceptance Criteria**:
  - ✅ `load_pokemon_data() -> Result<Vec<...>, Error>` compiles
  - ✅ `load_location_data() -> Result<Vec<...>, Error>` compiles
  - ✅ Error handling for missing files
  - ✅ Logging for loaded data count
- **Estimated effort**: 2 hours

### Task 1.2: Create data validator module
- **Description**: Create `src/data/validator.rs` for schema validation
- **Acceptance Criteria**:
  - ✅ `validate_pokemon_json()` checks required fields
  - ✅ `validate_location_json()` checks valid connections
  - ✅ `validate_referential_integrity()` ensures Pokémon IDs exist
  - ✅ Clear error messages with file path and line numbers
- **Estimated effort**: 2 hours

### Task 1.3: Create Pokémon JSON structures
- **Description**: Define serde structs for Pokémon JSON schema
- **Acceptance Criteria**:
  - ✅ `PokemonSpeciesJSON` struct matches schema
  - ✅ `MoveJSON` struct matches schema
  - ✅ Serde derives for serialization
  - ✅ Conversion from JSON to game `Pokemon` struct
- **Estimated effort**: 1.5 hours
- **Dependencies**: Task 1.1

### Task 1.4: Create Location JSON structures
- **Description**: Define serde structs for Location JSON schema
- **Acceptance Criteria**:
  - ✅ `LocationJSON` struct defined
  - ✅ `WildPokemonSpawnJSON` for encounter pools
  - ✅ Serde derives configured
  - ✅ Conversion to game `Location` struct
- **Estimated effort**: 1.5 hours
- **Dependencies**: Task 1.1

### Task 1.5: Create NPC JSON structures
- **Description**: Define serde structs for NPC trainer JSON schema
- **Acceptance Criteria**:
  - ✅ `TrainerJSON` struct defined
  - ✅ `TrainerPokemonJSON` for team composition
  - ✅ Conversion to game structures
- **Estimated effort**: 1 hour
- **Dependencies**: Task 1.1

## Phase 2: Data File Creation (Days 2-3)

### Task 2.1: Create assets directory structure
- **Description**: Create `assets/` directories and README
- **Acceptance Criteria**:
  - ✅ `assets/pokemon/` directory exists
  - ✅ `assets/locations/` directory exists
  - ✅ `assets/npcs/` directory exists
  - ✅ `assets/items/` directory exists (placeholder)
  - ✅ README.md in assets/ with format documentation
- **Estimated effort**: 0.5 hours

### Task 2.2: Create Pokémon species.json
- **Description**: Export all Pokémon data to JSON
- **Acceptance Criteria**:
  - ✅ All 15 current Pokémon species in JSON format
  - ✅ All base stats included
  - ✅ All types and abilities included
  - ✅ Valid JSON syntax (no parsing errors)
  - ✅ Matches schema from design.md
- **Estimated effort**: 2 hours
- **Details**: Convert from `src/data/pokemon_data.rs::create_*` functions

### Task 2.3: Create moves.json
- **Description**: Export all move data to JSON
- **Acceptance Criteria**:
  - ✅ All moves referenced by Pokémon in JSON
  - ✅ Move properties: type, power, accuracy, pp
  - ✅ Valid JSON syntax
  - ✅ No orphaned move IDs
- **Estimated effort**: 1.5 hours

### Task 2.4: Create world locations.json
- **Description**: Export all location data to JSON
- **Acceptance Criteria**:
  - ✅ All 10 locations exported
  - ✅ Connections verified (location IDs exist)
  - ✅ Wild Pokémon pools reference valid species
  - ✅ NPC IDs reference valid trainers
  - ✅ Valid JSON syntax
- **Estimated effort**: 2 hours
- **Dependencies**: Task 2.2 (need valid Pokémon IDs)

### Task 2.5: Create trainers.json
- **Description**: Export NPC trainer data to JSON
- **Acceptance Criteria**:
  - ✅ All 5+ trainers exported
  - ✅ Team composition specified
  - ✅ Pokémon IDs and levels valid
  - ✅ Locations reference valid location IDs
  - ✅ Valid JSON syntax
- **Estimated effort**: 1 hour
- **Dependencies**: Task 2.2, 2.4

## Phase 3: Integration & Loading (Days 3-4)

### Task 3.1: Implement data loading at startup
- **Description**: Call loader in `main.rs` to load all data
- **Acceptance Criteria**:
  - ✅ `loader::load_all_data()` called in main()
  - ✅ Errors prevent game from starting
  - ✅ Success message shows loaded item counts
  - ✅ Data available globally or via getter functions
  - ✅ Game compiles and runs
- **Estimated effort**: 2 hours
- **Dependencies**: Task 1.1-1.5, 2.1-2.5

### Task 3.2: Create global data cache
- **Description**: Store loaded data in static/lazy_static
- **Acceptance Criteria**:
  - ✅ `POKEMON_CACHE` static with all species
  - ✅ `LOCATION_CACHE` static with all locations
  - ✅ `TRAINER_CACHE` static with all trainers
  - ✅ Safe access via getter functions
  - ✅ Thread-safe (using Mutex or RwLock if needed)
- **Estimated effort**: 1.5 hours
- **Dependencies**: Task 3.1

### Task 3.3: Update pokemon_data.rs functions
- **Description**: Modify `get_pokemon_by_id()` etc. to use JSON data
- **Acceptance Criteria**:
  - ✅ `get_pokemon_by_id()` queries cache instead of match statement
  - ✅ All existing game code calls unchanged
  - ✅ API signature identical
  - ✅ Behavior identical to before
- **Estimated effort**: 1 hour
- **Dependencies**: Task 3.2

### Task 3.4: Update locations_data.rs functions
- **Description**: Modify location loading to use JSON data
- **Acceptance Criteria**:
  - ✅ `get_all_locations()` returns locations from JSON
  - ✅ `get_location_by_id()` queries cache
  - ✅ All existing code works unchanged
  - ✅ Connections validated
- **Estimated effort**: 1 hour
- **Dependencies**: Task 3.2

### Task 3.5: Update NPCs/trainer data access
- **Description**: Wire trainer data to use JSON
- **Acceptance Criteria**:
  - ✅ Trainer access uses JSON data
  - ✅ Team composition loaded from JSON
  - ✅ All existing code works
- **Estimated effort**: 1 hour
- **Dependencies**: Task 3.2

## Phase 4: Validation & Testing (Days 4-5)

### Task 4.1: Write data loading unit tests
- **Description**: Test JSON loading and parsing
- **Acceptance Criteria**:
  - ✅ `test_load_pokemon_data()` verifies file loads
  - ✅ `test_load_location_data()` verifies locations load
  - ✅ `test_pokemon_count()` verifies expected quantity
  - ✅ All tests pass
- **Estimated effort**: 1.5 hours
- **Dependencies**: Task 3.1

### Task 4.2: Write data validation tests
- **Description**: Test validator catches errors
- **Acceptance Criteria**:
  - ✅ `test_validate_missing_pokemon()` detects missing Pokémon
  - ✅ `test_validate_invalid_location_connection()` detects bad connections
  - ✅ `test_validate_referential_integrity()` ensures cross-references valid
  - ✅ Tests pass
- **Estimated effort**: 2 hours
- **Dependencies**: Task 1.2, 4.1

### Task 4.3: Integration test - full game startup
- **Description**: Test game loads with external data
- **Acceptance Criteria**:
  - ✅ Game starts without crashes
  - ✅ `cargo test --all` passes (all 29+ tests)
  - ✅ No warnings related to data loading
  - ✅ Data accessible from game code
- **Estimated effort**: 1.5 hours
- **Dependencies**: Task 3.1-3.5

### Task 4.4: Remove hardcoded data functions
- **Description**: Delete now-unused hardcoded data creation functions
- **Acceptance Criteria**:
  - ✅ `create_bulbasaur()`, `create_pikachu()` etc. removed
  - ✅ All `create_*_location()` functions removed
  - ✅ Compiler warnings about dead code eliminated
  - ✅ Tests still pass
- **Estimated effort**: 1 hour
- **Dependencies**: Task 4.3

## Phase 5: Documentation & Cleanup (Day 5)

### Task 5.1: Document content editing guidelines
- **Description**: Create guide for adding new Pokémon/locations
- **Acceptance Criteria**:
  - ✅ `CONTENT_GUIDE.md` created
  - ✅ Instructions for adding new Pokémon species
  - ✅ Instructions for adding new locations
  - ✅ Instructions for adding new trainers
  - ✅ Schema examples provided
- **Estimated effort**: 1.5 hours
- **Dependencies**: None

### Task 5.2: Create JSON schema documentation
- **Description**: Document the JSON schema and field meanings
- **Acceptance Criteria**:
  - ✅ `assets/README.md` with schema documentation
  - ✅ Example entries for each file type
  - ✅ Field descriptions and constraints
  - ✅ Validation rules documented
- **Estimated effort**: 1 hour

### Task 5.3: Update project README
- **Description**: Document data externalization in main README
- **Acceptance Criteria**:
  - ✅ README mentions external data files
  - ✅ Explains how to modify game content
  - ✅ Links to CONTENT_GUIDE.md
  - ✅ Mentions no recompilation needed for content updates
- **Estimated effort**: 0.5 hours

### Task 5.4: Update openspec with results
- **Description**: Mark proposal as completed
- **Acceptance Criteria**:
  - ✅ `IMPLEMENTATION_COMPLETE.md` created with results
  - ✅ Metrics: lines of code moved to JSON, file sizes
  - ✅ Before/after comparison
  - ✅ List of all created files
- **Estimated effort**: 1 hour

## Task Dependencies Map

```
Phase 1 (parallel tasks 1.1-1.5)
  ├─ Task 1.1 ──┐
  ├─ Task 1.2 ──┤
  ├─ Task 1.3 ──┼─→ Phase 2 (parallel tasks 2.1-2.5)
  ├─ Task 1.4 ──┤         ├─ Task 2.1 (0.5h)
  └─ Task 1.5 ──┘         ├─ Task 2.2 (2h) ──→ Task 2.4
                          ├─ Task 2.3 (1.5h)
                          ├─ Task 2.4 (2h) ───→ Task 2.5
                          └─ Task 2.5 (1h)

Phase 3 (sequential 3.1 → 3.2 → 3.3, 3.4, 3.5)
  ├─ Task 3.1 ──→ Task 3.2 ──→ Task 3.3
                       └────→ Task 3.4
                       └────→ Task 3.5

Phase 4 (parallel 4.1, 4.2 after 4.1; then 4.3; then 4.4)
  ├─ Task 4.1 ──→ Task 4.2 ──→ Task 4.3 ──→ Task 4.4

Phase 5 (parallel 5.1-5.3, then 5.4)
  ├─ Task 5.1 ──┐
  ├─ Task 5.2 ──┼─→ Task 5.4
  └─ Task 5.3 ──┘
```

## Success Metrics

- ✅ All Pokémon data loaded from JSON, zero hardcoded species
- ✅ All location data loaded from JSON, zero hardcoded locations
- ✅ All trainer data loaded from JSON
- ✅ Game launches and loads content correctly
- ✅ All 29+ tests pass
- ✅ No performance regression
- ✅ Content can be updated without recompiling Rust code
- ✅ Clear documentation for content editors

## Rollback Plan

If critical issues arise:
1. Revert to previous Git commit
2. Hardcoded data functions are still in history
3. Game logic unchanged, data access unchanged
4. Rollback should be transparent to players
