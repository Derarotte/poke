# Complete Data Externalization - Remaining Data Files
**Change ID**: 20251022-complete-data-externalization
**Status**: PROPOSED
**Priority**: HIGH
**Scope**: Externalize ~688 lines of remaining hardcoded game data

## Problem Statement

While Pokémon species, moves, locations, and trainers have been successfully externalized to JSON, there are still **~688 lines of hardcoded game data** scattered across the codebase:

- **Item data** - Properties, prices, effects (95 lines in `item.rs`)
- **Type effectiveness chart** - All type matchups (60 lines in `battle.rs`)
- **Game constants** - Damage formulas, escape rates, exp curves (50 lines)
- **Player defaults** - Starting items, money, team size (30 lines)
- **Environment bonuses** - Location stat multipliers (35 lines)
- **Nature system** - All 25 natures and their stat modifications (135 lines)
- **Game mechanics** - IV system, capture formulas, level curves (15 lines)
- **UI/Menu text** - All hardcoded menu strings (100 lines)
- **Fallback data** - Backup species and encounter data (168 lines)

**Issue**: Game designers still cannot modify:
- Item properties without editing Rust
- Type matchups without recompiling
- Game balance constants
- Starting player inventory
- Pokemon nature effects
- Menu text/localization

## Solution Overview

Externalize all remaining hardcoded game data to JSON files:
```
assets/
├── items/
│   └── items.json           (NEW) - All item definitions
├── battle/
│   └── type_effectiveness.json (NEW) - Type matchup chart
├── config/
│   ├── game_constants.json  (NEW) - Game mechanics constants
│   ├── player_defaults.json (NEW) - Starting player setup
│   └── environment_bonuses.json (NEW) - Location bonuses
├── pokemon/
│   └── natures.json         (NEW) - Nature definitions
└── localization/
    └── zh_CN.json          (NEW) - Menu text (future i18n)
```

## Implementation Approach

**Phase 1 (High Priority):**
1. Extract item data → `items.json`
2. Extract type effectiveness → `type_effectiveness.json`
3. Update `item.rs` to load from JSON
4. Update `battle.rs` type matching to use JSON

**Phase 2 (Medium Priority):**
5. Extract game constants → `game_constants.json`
6. Extract player defaults → `player_defaults.json`
7. Extract environment bonuses → `environment_bonuses.json`
8. Extract nature system → `natures.json`
9. Update all affected modules

**Phase 3 (Optional - Future):**
10. Menu text localization → `zh_CN.json`
11. Remove hardcoded fallback data
12. Implement i18n system

## Benefits

✅ **Complete data-driven design** - All game data in JSON
✅ **Game balance tuning** - Adjust constants without recompiling
✅ **Item editing** - Add/modify items via JSON
✅ **Type system flexibility** - Change type matchups dynamically
✅ **Nature customization** - Modify nature effects easily
✅ **Localization ready** - Prepare for multiple languages
✅ **Content modding** - Players can create custom content
✅ **Version control** - Track data changes separately

## Success Criteria

- ✅ All 688 lines of hardcoded data moved to JSON
- ✅ ~7 new JSON files created with proper schemas
- ✅ All existing tests still pass
- ✅ Zero breaking changes to game logic
- ✅ 100% backward compatibility
- ✅ Clear documentation for each new data format
- ✅ Game can be modified without touching Rust code

## Timeline

- **Phase 1 (High)**: 2-3 days
- **Phase 2 (Medium)**: 3-4 days
- **Phase 3 (Optional)**: 1-2 days
- **Total**: 6-9 days

## Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|-----------|
| Increased startup load time | LOW | Cache all JSON at startup, profile if needed |
| Complex JSON structures | MEDIUM | Use clear schemas, provide examples |
| Backward compatibility | MEDIUM | Extensive testing, gradual rollout |
| Player save corruption | LOW | Save format unchanged, only runtime data |

## Next Steps

1. User reviews and approves this proposal
2. Run `/openspec:apply 20251022-complete-data-externalization`
3. Follow implementation phases sequentially
4. Validate with comprehensive testing

## Reference: Current Externalization Progress

**Already Complete (✓):**
- ✓ Pokémon species → `assets/pokemon/species.json` (16 species)
- ✓ Moves → `assets/pokemon/moves.json` (7 moves)
- ✓ Locations → `assets/locations/world.json` (10 locations)
- ✓ Trainers → `assets/npcs/trainers.json` (9 trainers)

**This Proposal Will Complete:**
- Items, type matchups, game constants, natures, etc.

**Result**: 100% data-driven Pokémon game 🎉
