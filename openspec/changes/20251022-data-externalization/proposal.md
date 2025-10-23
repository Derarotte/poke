# Data Externalization Proposal
**Change ID**: 20251022-data-externalization
**Status**: PROPOSED
**Priority**: HIGH
**Scope**: Game Content System Architecture

## Problem Statement

Currently, all game content is hardcoded in Rust files:
- Pokémon species data (stats, moves, types) in `src/data/pokemon_data.rs`
- Locations and connections in `src/data/locations_data.rs`
- Wild Pokémon encounter pools defined per-location
- NPC trainer data and teams hardcoded in various files

**Issues**:
1. **Content updates require code recompilation** - Adding new Pokémon, locations, or items requires Rust knowledge and rebuilding the entire project
2. **Hard to maintain** - Game designers can't update content without developer intervention
3. **Not scalable** - As content grows, Rust files become unmaintainable
4. **No version control for content** - Can't track content changes separately from code

## Solution Overview

Move game content to external JSON data files with a data loading layer that:
- Loads content from JSON files at startup
- Validates data integrity
- Provides Rust structures that game logic can use
- Allows hot-reloading during development (optional future feature)

### Benefits
✅ Game designers can update content without touching Rust code
✅ Easier to add new Pokémon, locations, items, moves
✅ Content updates don't require recompilation
✅ Clear separation between game logic and game content
✅ Version control friendly - easier to track content changes

## Proposed Structure

```
assets/
├── pokemon/
│   ├── species.json          # All Pokémon species definitions
│   └── moves.json            # All move definitions
├── locations/
│   └── world.json            # All locations and connections
├── npcs/
│   └── trainers.json         # NPC trainer definitions
└── items/
    └── items.json            # Item definitions and properties
```

## Implementation Phases

**Phase 1**: Pokémon species and moves data externalization
**Phase 2**: Location and map data externalization
**Phase 3**: NPC trainer data externalization
**Phase 4**: Create content editing guidelines and documentation

## Success Criteria

- ✅ All Pokémon data loaded from JSON (not hardcoded in Rust)
- ✅ All location data loaded from JSON
- ✅ Game compiles and all tests pass
- ✅ Data validation catches malformed JSON
- ✅ Game content can be updated without recompiling Rust code
- ✅ 100% backward compatibility with existing save files

## Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|-----------|
| JSON parsing errors crash game | HIGH | Validate all JSON at startup, provide clear error messages |
| Data format mismatch | MEDIUM | Use schema validation, version JSON format |
| Performance impact from file I/O | LOW | Load all content once at startup |

## Timeline

- **Phase 1-2**: 2-3 days
- **Phase 3**: 1 day
- **Phase 4**: 1 day
- **Total**: ~1 week

## Next Steps

1. Review and approve this proposal
2. Run `/openspec:apply 20251022-data-externalization` to begin implementation
3. Follow the detailed design and tasks
