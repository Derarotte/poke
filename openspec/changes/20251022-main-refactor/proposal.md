# main.rs Refactoring Proposal

**Change ID**: 20251022-main-refactor
**Status**: PROPOSAL
**Author**: Claude
**Date**: 2025-10-22

## Problem Statement

The `src/main.rs` file has grown to 843 lines with 18 functions, violating the single responsibility principle. Functions are organized by feature (movement, exploration, battles) but lack clear modularity and separation of concerns. This makes the code harder to maintain, test, and extend.

### Current Issues

1. **Mixed Concerns**: Game loop logic, menu handling, and game state management are intertwined
2. **No Abstraction**: Combat logic, encounter mechanics, and menu interactions lack abstraction layers
3. **Difficult Testing**: Functions have multiple responsibilities making unit tests cumbersome
4. **Poor Reusability**: Duplicated logic across different game modes (wild encounters, NPC battles, etc.)
5. **Hard to Navigate**: 843 lines require frequent scrolling and context switching

## Solution Overview

Refactor `main.rs` into a modular architecture with clear separation of concerns:

1. **GameController** - Central game orchestration (game loop, state transitions)
2. **EncounterManager** - Wild Pokemon encounters and capture flow
3. **BattleHandler** - Battle execution and NPC battle coordination
4. **RevivalHandler** - Pokemon revival mechanics (items, Pokemon center)
5. **ExplorationHandler** - Location movement and environment exploration

Each module will:
- Have a single, well-defined responsibility
- Export a clean public API
- Have minimal external dependencies
- Be testable in isolation

## Goals

- ✅ Reduce main.rs to <150 lines (83% reduction)
- ✅ Create 5 handler modules with clear responsibilities
- ✅ Maintain 100% backward compatibility
- ✅ Improve code maintainability and testability
- ✅ Enable future feature additions without modifying main logic

## Impact

- **Files Created**: 5 new handler modules (`src/handlers/` directory)
- **Files Modified**: `src/main.rs` (cleanup), `src/lib.rs` (module exports)
- **Backward Compatibility**: ✅ 100% (no public API changes)
- **Testing**: ✅ All handlers independently testable

## Success Criteria

1. `src/main.rs` reduced to <150 lines
2. All 18 functions distributed to appropriate handler modules
3. No duplicated code
4. All existing tests still pass
5. Handler modules have <200 lines each
6. Clear module responsibility boundaries

## Next Steps

1. Review and validate this proposal
2. Create `design.md` with architectural details
3. Create `tasks.md` with implementation plan
4. Begin implementation phase

---

**Notes for Review**:
- This refactoring is primarily organizational - no behavioral changes
- The refactor will improve code quality without affecting user experience
- Future battle system enhancements will be much easier to implement
