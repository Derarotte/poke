# Data Externalization - Detailed Design

**Change ID**: 20251022-data-externalization

## Architecture Overview

### Current Architecture (Monolithic)
```
game.rs
  └─ Hardcoded Pokémon data
  └─ Hardcoded location data
  └─ Hardcoded move data
```

### Proposed Architecture (Data-Driven)
```
game.rs (unchanged)
  └─ Uses game structures (Pokemon, Location, etc.)

data/
  ├── loader.rs          (NEW) - Load JSON files
  ├── validator.rs       (NEW) - Validate JSON schema
  └── pokemon_data.rs    (MODIFIED) - Functions instead of data

assets/
  ├── pokemon/species.json
  ├── pokemon/moves.json
  ├── locations/world.json
  └── npcs/trainers.json
```

## Data File Schemas

### 1. Pokemon Species (`assets/pokemon/species.json`)

```json
{
  "species": [
    {
      "id": 1,
      "name": "妙蛙种子",
      "english_name": "Bulbasaur",
      "primary_type": "Grass",
      "secondary_type": "Poison",
      "base_stats": {
        "hp": 45,
        "attack": 49,
        "defense": 49,
        "sp_attack": 65,
        "sp_defense": 65,
        "speed": 45
      },
      "catch_rate": 45,
      "experience_yield": 64,
      "evolution": {
        "method": "level",
        "trigger": 16,
        "to": 2
      }
    }
  ]
}
```

### 2. Moves (`assets/pokemon/moves.json`)

```json
{
  "moves": [
    {
      "id": 1,
      "name": "念力",
      "type": "Psychic",
      "category": "Special",
      "power": 50,
      "accuracy": 100,
      "pp": 25,
      "effect": "None"
    }
  ]
}
```

### 3. Locations (`assets/locations/world.json`)

```json
{
  "locations": [
    {
      "id": 101,
      "name": "常青小镇",
      "description": "宁静祥和的小镇",
      "environment": "Grassland",
      "encounter_rate": 0.3,
      "is_starting_location": true,
      "connections": [102],
      "wild_pokemon": [
        {
          "pokemon_id": 25,
          "spawn_rate": 50.0,
          "level_min": 2,
          "level_max": 4
        }
      ],
      "npcs": [1]
    }
  ]
}
```

### 4. NPC Trainers (`assets/npcs/trainers.json`)

```json
{
  "trainers": [
    {
      "id": 1,
      "name": "青绿",
      "title": "Champion",
      "location_id": 101,
      "team": [
        {
          "pokemon_id": 25,
          "level": 5
        }
      ]
    }
  ]
}
```

## Implementation Strategy

### Phase 1: Data Loading Infrastructure

**Create `src/data/loader.rs`**:
- `load_pokemon_data() -> Result<Vec<PokemonSpecies>, Error>`
- `load_move_data() -> Result<Vec<MoveData>, Error>`
- `load_location_data() -> Result<Vec<LocationData>, Error>`

**Create `src/data/validator.rs`**:
- `validate_pokemon_json(json: &str) -> Result<(), Error>`
- `validate_location_json(json: &str) -> Result<(), Error>`
- Catch missing IDs, invalid references, schema violations

### Phase 2: Data Structures

Define intermediate JSON-mapped structures in `src/data/`:
```rust
struct PokemonSpeciesJSON {
    id: u32,
    name: String,
    base_stats: StatsJSON,
    // ... other fields
}

// Convert JSON to game structures
impl From<PokemonSpeciesJSON> for Pokemon {
    fn from(json: PokemonSpeciesJSON) -> Self { ... }
}
```

### Phase 3: Integration

Modify existing data access:
- `src/data/pokemon_data.rs::get_pokemon_by_id()` now:
  1. Loads from memory cache (loaded at startup)
  2. Returns reference to cached data
- Same for locations, moves, trainers

### Phase 4: Asset Directory

Create `assets/` directory with JSON files:
- Place in project root or configurable path
- Load at game startup in `main.rs`

## Data Flow

```
main.rs startup
  └─ loader::load_all_data()
     ├─ loads assets/pokemon/species.json
     ├─ loads assets/pokemon/moves.json
     ├─ loads assets/locations/world.json
     └─ loads assets/npcs/trainers.json
  └─ validator::validate_all()
  └─ Store in static/lazy_static cache
  └─ Game code queries cache via existing API
```

## Backward Compatibility

✅ **Game structures unchanged** - `Pokemon`, `Location`, etc. structs remain the same
✅ **API unchanged** - `get_pokemon_by_id()`, `get_location_by_id()` work the same
✅ **Save files compatible** - Player save data structure unchanged
✅ **No performance impact** - All data loaded once at startup

## Error Handling

- **Missing JSON file**: Game fails to start with clear error message
- **Invalid JSON syntax**: Serde error with file path and line number
- **Schema mismatch**: Custom validation error with detailed message
- **Invalid references**: Warn about missing Pokémon IDs in encounter pools

Example error:
```
Error loading game data:
  File: assets/pokemon/species.json
  Error: Pokemon #999 referenced in locations but not defined
  Fix: Add pokemon #999 to species.json or remove from location pools
```

## Future Enhancements (Not in Scope)

1. **Hot-reloading** - Reload JSON without restarting game
2. **Content editing tools** - Web/GUI editor for content
3. **Content versioning** - Track changes to JSON over time
4. **Mod support** - Allow community-created content packs

## Testing Strategy

1. **Unit tests** - JSON parsing and validation
2. **Integration tests** - Load all content, verify game starts
3. **Content tests** - All Pokémon have valid stats, locations have valid connections

```rust
#[test]
fn test_load_pokemon_data() {
    let pokemon = loader::load_pokemon_data().unwrap();
    assert!(pokemon.len() > 0);
}

#[test]
fn test_pokemon_data_integrity() {
    let pokemon = loader::load_pokemon_data().unwrap();
    for p in pokemon {
        assert!(p.id > 0);
        assert!(!p.name.is_empty());
    }
}
```

## Migration Path

1. **Keep existing code** - Keep hardcoded data functions temporarily
2. **Add JSON loading** - Load JSON in parallel
3. **Switch to JSON** - Gradually migrate to use JSON data
4. **Remove hardcoded data** - Delete old hardcoded data functions

This allows rollback if issues arise.

## Dependencies

- `serde_json` - Already in Cargo.toml
- `serde` with derive - Already in Cargo.toml
- No new external dependencies needed!

## Performance Considerations

| Aspect | Impact | Notes |
|--------|--------|-------|
| Startup time | +50ms | One-time load of all JSON files |
| Memory usage | +2-5MB | Stores all game content in memory |
| Runtime queries | No change | Direct memory access, same as before |

## Success Metrics

- ✅ All game data loaded from JSON files
- ✅ Can add new Pokémon without editing Rust code
- ✅ Can add new locations without editing Rust code
- ✅ Zero test failures
- ✅ Zero performance regression
- ✅ All existing functionality works

## Risk Mitigation

**Risk**: JSON files missing at runtime
**Mitigation**: Clear error message, check file existence at startup

**Risk**: Invalid JSON structure
**Mitigation**: Validate schema, provide schema documentation

**Risk**: Game data mismatch (e.g., location references non-existent Pokémon)
**Mitigation**: Validation checks for referential integrity
