# Complete Data Externalization - Implementation Tasks

**Change ID**: 20251022-complete-data-externalization
**Total Tasks**: 28
**Estimated Duration**: 6-9 days
**Priority**: HIGH

## Phase 1: Items & Type Effectiveness (Days 1-2)

### Task 1.1: Create items.json file
- **Description**: Extract all item data from `src/game/item.rs` (lines 22-71) to JSON format
- **Acceptance Criteria**:
  - ✅ `assets/items/items.json` created with all 6 items
  - ✅ All fields present: id, name, english_name, item_type, description, price, recovery_percent, is_revive, is_healing
  - ✅ Valid JSON syntax (no parsing errors)
  - ✅ Matches schema from design.md
  - ✅ Item IDs: 1-6 (PokeBall, Potion, SuperPotion, Revive, FullRestore, Antidote)
- **Estimated effort**: 1 hour
- **Current source**: src/game/item.rs lines 22-71

### Task 1.2: Create type_effectiveness.json file
- **Description**: Extract type matchup chart from `src/game/battle.rs` (lines 277-335) to JSON
- **Acceptance Criteria**:
  - ✅ `assets/battle/type_effectiveness.json` created
  - ✅ All type matchups included: Normal, Fire, Water, Grass, Electric, Psychic, etc.
  - ✅ All 30+ matchup rows with attacking, defending, multiplier fields
  - ✅ Valid JSON syntax
  - ✅ Multipliers: 0.0, 0.5, 1.0 (default), 2.0
  - ✅ Matches schema from design.md
- **Estimated effort**: 1.5 hours
- **Current source**: src/game/battle.rs lines 277-335

### Task 1.3: Update item.rs to load from JSON
- **Description**: Modify `src/game/item.rs` to use cached item data instead of hardcoded functions
- **Acceptance Criteria**:
  - ✅ `get_item_by_type()` function created to query JSON cache
  - ✅ All item creation functions (`create_poke_ball()`, etc.) replaced with cache lookups
  - ✅ API signature unchanged - external code calls unchanged
  - ✅ All 6 items accessible by type: "PokeBall", "Potion", "SuperPotion", "Revive", "FullRestore", "Antidote"
  - ✅ Existing tests pass
- **Estimated effort**: 1.5 hours
- **Dependencies**: Task 1.1

### Task 1.4: Update battle.rs to use type_effectiveness.json
- **Description**: Modify `src/game/battle.rs` type matching to query JSON instead of hardcoded chart
- **Acceptance Criteria**:
  - ✅ `get_type_effectiveness()` function created to lookup type matchup from JSON
  - ✅ Function signature: `fn(attacking_type: &str, defending_type: &str) -> f32`
  - ✅ Returns correct multipliers: 2.0 for super-effective, 0.5 for not-effective, 0.0 for immune, 1.0 default
  - ✅ Replaces hardcoded match statement (lines 277-335)
  - ✅ All battle damage calculations use new function
  - ✅ All existing tests pass
- **Estimated effort**: 1.5 hours
- **Dependencies**: Task 1.2

### Task 1.5: Wire items and type effectiveness to loader
- **Description**: Add item and type effectiveness loading to `src/data/loader.rs`
- **Acceptance Criteria**:
  - ✅ `load_items_data()` function added to loader.rs
  - ✅ `load_type_effectiveness_data()` function added to loader.rs
  - ✅ Both called in `load_all_data()`
  - ✅ Global cache includes `items` and `type_effectiveness` vectors
  - ✅ Getter functions: `get_item_by_type()`, `get_type_effectiveness()`
  - ✅ Validation added to validator.rs for items and types
- **Estimated effort**: 2 hours
- **Dependencies**: Task 1.1, 1.2, 1.3, 1.4

## Phase 2: Game Constants & Config (Days 2-4)

### Task 2.1: Create game_constants.json file
- **Description**: Extract game mechanics constants to JSON
- **Acceptance Criteria**:
  - ✅ `assets/config/game_constants.json` created
  - ✅ Battle section: damage_random_min_percent, damage_random_max_percent, escape_chance_percent
  - ✅ Experience section: formula_divisor, base_exp_per_level, money_per_level_multiplier
  - ✅ Pokemon section: max_moves, max_team_size
  - ✅ Stats section: hp_formula, stat_formula, nature_boost_multiplier, nature_penalty_multiplier
  - ✅ IV System section: min_value, max_value, perfect_value
  - ✅ Capture section: catch_rate_divisor, hp_penalty_multiplier, formula
  - ✅ NPC Generation section: default_iv_value
  - ✅ Valid JSON, matches design.md schema
- **Estimated effort**: 2 hours
- **Current source**: src/game/battle.rs, src/game/pokemon.rs, src/pokemon_generator/generator.rs, src/handlers/encounter_manager.rs

### Task 2.2: Create player_defaults.json file
- **Description**: Extract player starting configuration to JSON
- **Acceptance Criteria**:
  - ✅ `assets/config/player_defaults.json` created
  - ✅ Starting section: location_id, money, inventory with item_type and quantity
  - ✅ Pokemon Center section: base_revival_cost, vip_discount_percent
  - ✅ Team section: max_size
  - ✅ Starting inventory: 5 PokeBalls, 3 Potions, 1 SuperPotion, 1 Revive
  - ✅ Starting location: 101 (home)
  - ✅ Starting money: 0
  - ✅ Valid JSON, matches design.md schema
- **Estimated effort**: 1 hour
- **Current source**: src/game/player.rs

### Task 2.3: Create environment_bonuses.json file
- **Description**: Extract location environment stat bonuses to JSON
- **Acceptance Criteria**:
  - ✅ `assets/config/environment_bonuses.json` created
  - ✅ All 6 environment types: Grassland, Forest, Cave, Water, Mountain, City
  - ✅ Each with type, name_zh, stat_bonuses (speed, defense, sp_attack, sp_defense, attack)
  - ✅ Grassland: speed +10%, Forest: defense +10%, Cave: sp_attack +10%, Water: sp_defense +10%
  - ✅ Mountain: attack +10%, City: all stats +5%
  - ✅ Valid JSON, matches design.md schema
- **Estimated effort**: 1 hour
- **Current source**: src/game/location.rs lines 53-83

### Task 2.4: Create natures.json file
- **Description**: Extract all 25 Pokémon natures to JSON
- **Acceptance Criteria**:
  - ✅ `assets/pokemon/natures.json` created
  - ✅ All 25 natures: Hardy, Lonely, Adamant, Naughty, Brave, Bold, Docile, Impish, Lax, Relaxed, Modest, Mild, Quiet, Rash, Calm, Gentle, Sassy, Careful, Quirky, Sly, Timid, Hasty, Jolly, Naive, (plus 1 additional)
  - ✅ Each nature: id, name_en, name_zh, stat_modifiers
  - ✅ Stat modifiers: boost stat to 1.1x, penalty stat to 0.9x
  - ✅ Hardy, Docile, Quirky have empty modifiers {}
  - ✅ Talent probability section: normal_percent (90), hidden_percent (10)
  - ✅ Valid JSON, matches design.md schema
- **Estimated effort**: 1.5 hours
- **Current source**: src/pokemon_generator/mod.rs lines 104-268

### Task 2.5: Update game modules to load constants from JSON
- **Description**: Modify `src/game/battle.rs`, `src/game/pokemon.rs`, `src/game/player.rs` to use config JSON
- **Acceptance Criteria**:
  - ✅ Battle damage formula uses constants from JSON
  - ✅ Experience calculation uses base_exp_per_level from JSON
  - ✅ Stat calculations use formulas from JSON
  - ✅ IV system uses min/max values from JSON
  - ✅ Capture calculation uses catch_rate_divisor from JSON
  - ✅ Player starting position/money/inventory loaded from JSON
  - ✅ All function signatures unchanged
  - ✅ All tests pass
- **Estimated effort**: 3 hours
- **Dependencies**: Task 2.1, 2.2, 2.3

### Task 2.6: Update pokemon_generator/mod.rs for natures
- **Description**: Modify nature system to load from JSON
- **Acceptance Criteria**:
  - ✅ `get_nature_by_id()` function created to query JSON cache
  - ✅ All 25 natures accessible by ID (Hardy, Lonely, etc.)
  - ✅ Nature stat modifiers applied correctly
  - ✅ Talent probability loaded from JSON (90% normal, 10% hidden)
  - ✅ All tests pass
- **Estimated effort**: 2 hours
- **Dependencies**: Task 2.4

### Task 2.7: Update location.rs for environment bonuses
- **Description**: Modify environment bonus system to load from JSON
- **Acceptance Criteria**:
  - ✅ `get_environment_bonus()` function created to query JSON cache
  - ✅ All 6 environment types accessible by type name
  - ✅ Stat bonuses applied correctly in battle/stat calculations
  - ✅ All tests pass
- **Estimated effort**: 1.5 hours
- **Dependencies**: Task 2.3

### Task 2.8: Wire all config data to loader
- **Description**: Add all new config loaders to `src/data/loader.rs`
- **Acceptance Criteria**:
  - ✅ `load_game_constants()`, `load_player_defaults()`, `load_environment_bonuses()` functions added
  - ✅ All called in `load_all_data()`
  - ✅ Global cache includes all new data vectors
  - ✅ Getter functions for each config type
  - ✅ Validation added to validator.rs for all config data
  - ✅ No compilation errors
- **Estimated effort**: 1.5 hours
- **Dependencies**: Task 2.1-2.7

## Phase 3: Menu Text & Localization (Optional, Days 4-5)

### Task 3.1: Create zh_CN.json localization file
- **Description**: Extract hardcoded menu text to JSON (future i18n support)
- **Acceptance Criteria**:
  - ✅ `assets/localization/zh_CN.json` created
  - ✅ Menu strings: "主菜单", "探险", "查看队伍", "打开背包", "访问宝可梦中心", "保存游戏", "退出游戏"
  - ✅ Battle strings: "选择行动", "宝可梦战斗", "物品", "逃脑", "战斗开始", "战斗结束"
  - ✅ Encounter strings: "遇见了宝可梦", "逃脑成功", "逃脱失败"
  - ✅ All Chinese text organized by category
  - ✅ Valid JSON, proper UTF-8 encoding
- **Estimated effort**: 1.5 hours
- **Note**: Optional - provides foundation for future i18n system

### Task 3.2: Update menu system for localization
- **Description**: Wire menu text to use localized strings from JSON
- **Acceptance Criteria**:
  - ✅ GameController menu prints strings from cache instead of hardcoded literals
  - ✅ All menu options use localized text
  - ✅ Battle system uses localized battle strings
  - ✅ Encounter system uses localized encounter strings
  - ✅ All tests pass
  - ✅ Future languages can be added by adding new locale files
- **Estimated effort**: 2 hours
- **Dependencies**: Task 3.1

## Phase 4: Validation & Testing (Days 5-6)

### Task 4.1: Update validator.rs for all new data
- **Description**: Add comprehensive validation for all new config data
- **Acceptance Criteria**:
  - ✅ `validate_items_data()` checks item IDs, prices, recovery_percent
  - ✅ `validate_type_effectiveness()` checks type names exist, multipliers are valid (0.0, 0.5, 1.0, 2.0)
  - ✅ `validate_game_constants()` checks all required keys present, numeric ranges valid
  - ✅ `validate_player_defaults()` checks starting location exists, inventory items are valid
  - ✅ `validate_environment_bonuses()` checks environment types, stat names, bonus ranges (1.0-1.2)
  - ✅ `validate_natures()` checks all 25 natures present, stat modifiers are 0.9 or 1.1
  - ✅ Clear error messages for each validation failure
- **Estimated effort**: 2 hours

### Task 4.2: Write comprehensive unit tests
- **Description**: Test all new JSON loaders and converters
- **Acceptance Criteria**:
  - ✅ `test_load_items_data()` verifies items load, count is 6
  - ✅ `test_load_type_effectiveness()` verifies chart loads, basic matchups correct
  - ✅ `test_load_game_constants()` verifies all constants load
  - ✅ `test_load_player_defaults()` verifies starting config loads
  - ✅ `test_load_environment_bonuses()` verifies all 6 environments load
  - ✅ `test_load_natures()` verifies all 25 natures load
  - ✅ All tests pass, code coverage > 90%
- **Estimated effort**: 2.5 hours
- **Dependencies**: Task 4.1

### Task 4.3: Integration tests
- **Description**: Test game startup with all new data
- **Acceptance Criteria**:
  - ✅ Game starts without crashes
  - ✅ `cargo test --all` passes (all 34+ tests)
  - ✅ Items system working (can equip items in battle)
  - ✅ Type effectiveness working (damage calculations correct)
  - ✅ Game constants applied (exp gain, damage formulas correct)
  - ✅ Player starts with correct inventory
  - ✅ Natures applied correctly to Pokémon
  - ✅ Environment bonuses applied correctly
  - ✅ No performance regression
- **Estimated effort**: 2 hours
- **Dependencies**: Task 1.5, 2.8, 4.2

### Task 4.4: Remove hardcoded data code
- **Description**: Delete now-unused hardcoded data creation functions
- **Acceptance Criteria**:
  - ✅ All `create_*_item()` functions removed from src/game/item.rs
  - ✅ Hardcoded type chart removed from src/game/battle.rs
  - ✅ Hardcoded environment bonuses removed from src/game/location.rs
  - ✅ Hardcoded natures removed from src/pokemon_generator/mod.rs
  - ✅ Hardcoded constants removed from files
  - ✅ No compiler warnings about dead code
  - ✅ All tests still pass
  - ✅ Approximately 688 lines of Rust code eliminated
- **Estimated effort**: 1.5 hours
- **Dependencies**: Task 4.3

## Phase 5: Documentation & Cleanup (Days 6-7)

### Task 5.1: Update content editing guidelines
- **Description**: Extend CONTENT_GUIDE.md with new data types
- **Acceptance Criteria**:
  - ✅ Documentation for adding new items to items.json
  - ✅ Documentation for modifying type effectiveness
  - ✅ Documentation for adjusting game constants
  - ✅ Documentation for changing player starting setup
  - ✅ Documentation for modifying environment bonuses
  - ✅ Documentation for creating new natures
  - ✅ Documentation for updating localization
  - ✅ Schema examples for each new file type
- **Estimated effort**: 2 hours

### Task 5.2: Create comprehensive README update
- **Description**: Update main README.md with data externalization
- **Acceptance Criteria**:
  - ✅ Explains all 6 new JSON files
  - ✅ Shows before/after statistics (688 lines moved to JSON)
  - ✅ Lists all file locations under assets/
  - ✅ Explains how to modify content without recompiling
  - ✅ Links to CONTENT_GUIDE.md
  - ✅ Mentions backward compatibility
- **Estimated effort**: 1 hour

### Task 5.3: Create implementation completion report
- **Description**: Document all changes and results
- **Acceptance Criteria**:
  - ✅ `IMPLEMENTATION_COMPLETE.md` created
  - ✅ All 28 tasks listed with completion status
  - ✅ Metrics: 688 lines moved to JSON, 6 new files created
  - ✅ Before/after code comparison
  - ✅ Test results: all tests passing
  - ✅ Performance metrics (startup time if changed)
  - ✅ List of all modified/created files
- **Estimated effort**: 1.5 hours

## Task Dependencies Map

```
Phase 1 (sequential):
  ├─ Task 1.1 ──→ Task 1.3 ──────────────┐
  ├─ Task 1.2 ──→ Task 1.4 ──────────────┼─→ Task 1.5
  └─────────────────────────────────────┘

Phase 2 (parallel 2.1-2.4, then 2.5-2.8):
  ├─ Task 2.1 ──┐
  ├─ Task 2.2 ──┼─→ Task 2.5 ────────┐
  ├─ Task 2.3 ──┼─→ Task 2.7 ────────┼─→ Task 2.8
  └─ Task 2.4 ──→ Task 2.6 ────────┘

Phase 3 (optional):
  ├─ Task 3.1 ──→ Task 3.2

Phase 4 (sequential):
  ├─ Task 4.1 ──→ Task 4.2 ──→ Task 4.3 ──→ Task 4.4

Phase 5 (parallel 5.1-5.2, then 5.3):
  ├─ Task 5.1 ──┐
  ├─ Task 5.2 ──┼─→ Task 5.3
  └─────────────┘
```

## Success Metrics

- ✅ All 688 lines of hardcoded data moved to JSON (6 new files)
- ✅ 8 game modules updated to use JSON data (item.rs, battle.rs, pokemon.rs, location.rs, player.rs, encounter_manager.rs, pokemon_generator/mod.rs, battle_handler.rs)
- ✅ Items system fully externalized (6 items)
- ✅ Type effectiveness chart fully externalized (30+ matchups)
- ✅ Game constants fully externalized (28+ constants)
- ✅ Player defaults fully externalized
- ✅ Environment bonuses fully externalized (6 types)
- ✅ Nature system fully externalized (25 natures)
- ✅ Menu text prepared for i18n (optional phase)
- ✅ All 34+ tests pass
- ✅ Zero compilation errors
- ✅ Zero hardcoded game data in Rust code
- ✅ Game content fully editable without recompiling
- ✅ 100% backward compatible with existing saves and gameplay
- ✅ Clear documentation for content designers

## Rollback Plan

If critical issues arise:
1. Revert to Git commit before Phase 1 Task 1.1
2. Hardcoded data functions remain in Git history
3. Game logic unchanged, data access pattern unchanged
4. Rollback should be transparent to players and saves

## Phase Completion Order

- **Phase 1**: Items & Type Effectiveness (2 days) - Foundation for damage calculations
- **Phase 2**: Game Constants & Config (3-4 days) - Core game mechanics and player setup
- **Phase 3**: Localization (1-2 days, Optional) - Future i18n support
- **Phase 4**: Validation & Testing (2 days) - Ensure all data integrity
- **Phase 5**: Documentation (1 day) - Complete the implementation

## Notes

- Phase 1 and Phase 2 can overlap (start Phase 2 after Phase 1 Task 1.5 completes)
- Phase 3 is optional but recommended for future multi-language support
- All phases maintain 100% backward compatibility
- Game designers will be able to modify all data without Rust knowledge after Phase 2 completion
