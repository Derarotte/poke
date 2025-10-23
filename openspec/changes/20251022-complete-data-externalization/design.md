# Complete Data Externalization - Detailed Design

**Change ID**: 20251022-complete-data-externalization

## Overview

This document details the design for externalizing all remaining hardcoded game data (~688 lines) to JSON format.

## Data Schemas

### 1. Items (`assets/items/items.json`)

Current location: `src/game/item.rs` (lines 22-71)

```json
{
  "items": [
    {
      "id": 1,
      "name": "精灵球",
      "english_name": "Poké Ball",
      "item_type": "PokeBall",
      "description": "用来捕捉宝可梦的基础工具",
      "price": 100,
      "recovery_percent": null,
      "is_revive": false,
      "is_healing": false
    },
    {
      "id": 2,
      "name": "恢复药",
      "english_name": "Potion",
      "item_type": "Potion",
      "description": "恢复宝可梦50点HP",
      "price": 150,
      "recovery_percent": 50,
      "is_revive": false,
      "is_healing": true
    },
    {
      "id": 3,
      "name": "超级恢复药",
      "english_name": "Super Potion",
      "item_type": "SuperPotion",
      "description": "恢复宝可梦100点HP",
      "price": 250,
      "recovery_percent": 100,
      "is_revive": false,
      "is_healing": true
    },
    {
      "id": 4,
      "name": "全复活",
      "english_name": "Revive",
      "item_type": "Revive",
      "description": "复活1只昏迷的宝可梦，恢复50%HP",
      "price": 200,
      "recovery_percent": 50,
      "is_revive": true,
      "is_healing": false
    },
    {
      "id": 5,
      "name": "全復恢复",
      "english_name": "Full Restore",
      "item_type": "FullRestore",
      "description": "完全恢复宝可梦的全部HP",
      "price": 400,
      "recovery_percent": 100,
      "is_revive": false,
      "is_healing": true
    },
    {
      "id": 6,
      "name": "解毒剂",
      "english_name": "Antidote",
      "item_type": "Antidote",
      "description": "治疗中毒状态",
      "price": 100,
      "recovery_percent": null,
      "is_revive": false,
      "is_healing": false
    }
  ]
}
```

### 2. Type Effectiveness (`assets/battle/type_effectiveness.json`)

Current location: `src/game/battle.rs` (lines 277-335)

```json
{
  "type_chart": [
    {"attacking": "Normal", "defending": "Rock", "multiplier": 0.5},
    {"attacking": "Normal", "defending": "Ghost", "multiplier": 0.0},
    {"attacking": "Fire", "defending": "Grass", "multiplier": 2.0},
    {"attacking": "Fire", "defending": "Ice", "multiplier": 2.0},
    {"attacking": "Fire", "defending": "Bug", "multiplier": 2.0},
    {"attacking": "Fire", "defending": "Steel", "multiplier": 2.0},
    {"attacking": "Fire", "defending": "Water", "multiplier": 0.5},
    {"attacking": "Fire", "defending": "Ground", "multiplier": 0.5},
    {"attacking": "Fire", "defending": "Rock", "multiplier": 0.5},
    {"attacking": "Water", "defending": "Fire", "multiplier": 2.0},
    {"attacking": "Water", "defending": "Ground", "multiplier": 2.0},
    {"attacking": "Water", "defending": "Rock", "multiplier": 2.0},
    {"attacking": "Water", "defending": "Water", "multiplier": 0.5},
    {"attacking": "Water", "defending": "Grass", "multiplier": 0.5},
    {"attacking": "Water", "defending": "Electric", "multiplier": 0.5},
    {"attacking": "Grass", "defending": "Water", "multiplier": 2.0},
    {"attacking": "Grass", "defending": "Ground", "multiplier": 2.0},
    {"attacking": "Grass", "defending": "Rock", "multiplier": 2.0},
    {"attacking": "Grass", "defending": "Fire", "multiplier": 0.5},
    {"attacking": "Grass", "defending": "Grass", "multiplier": 0.5},
    {"attacking": "Grass", "defending": "Poison", "multiplier": 0.5},
    {"attacking": "Electric", "defending": "Water", "multiplier": 2.0},
    {"attacking": "Electric", "defending": "Flying", "multiplier": 2.0},
    {"attacking": "Electric", "defending": "Electric", "multiplier": 0.5},
    {"attacking": "Electric", "defending": "Grass", "multiplier": 0.5},
    {"attacking": "Electric", "defending": "Ground", "multiplier": 0.0},
    {"attacking": "Psychic", "defending": "Fighting", "multiplier": 2.0},
    {"attacking": "Psychic", "defending": "Poison", "multiplier": 2.0},
    {"attacking": "Psychic", "defending": "Psychic", "multiplier": 0.5},
    {"attacking": "Psychic", "defending": "Dark", "multiplier": 0.0}
  ]
}
```

### 3. Game Constants (`assets/config/game_constants.json`)

Current locations: Multiple files
- Battle formulas: `src/game/battle.rs`
- Stat calculation: `src/game/pokemon.rs`
- Pokemon generation: `src/pokemon_generator/generator.rs`
- Capture system: `src/handlers/encounter_manager.rs`

```json
{
  "battle": {
    "damage_random_min_percent": 85,
    "damage_random_max_percent": 100,
    "escape_chance_percent": 60
  },
  "experience": {
    "formula_divisor": 7,
    "base_exp_per_level": 100,
    "money_per_level_multiplier": 10
  },
  "pokemon": {
    "max_moves": 4,
    "max_team_size": 6
  },
  "stats": {
    "hp_formula": "(2 * base + iv) * level / 100 + level + 5",
    "stat_formula": "((2 * base + iv) * level / 100 + 5) * nature_multiplier",
    "nature_boost_multiplier": 1.1,
    "nature_penalty_multiplier": 0.9
  },
  "iv_system": {
    "min_value": 0,
    "max_value": 31,
    "perfect_value": 31
  },
  "capture": {
    "catch_rate_divisor": 255,
    "hp_penalty_multiplier": 0.5,
    "formula": "capture_rate / 255.0 * (1.0 - hp_ratio * 0.5)"
  },
  "npc_generation": {
    "default_iv_value": 25
  }
}
```

### 4. Player Defaults (`assets/config/player_defaults.json`)

Current location: `src/game/player.rs`

```json
{
  "starting": {
    "location_id": 101,
    "money": 0,
    "inventory": [
      {"item_type": "PokeBall", "quantity": 5},
      {"item_type": "Potion", "quantity": 3},
      {"item_type": "SuperPotion", "quantity": 1},
      {"item_type": "Revive", "quantity": 1}
    ]
  },
  "pokemon_center": {
    "base_revival_cost": 200,
    "vip_discount_percent": 50
  },
  "team": {
    "max_size": 6
  }
}
```

### 5. Environment Bonuses (`assets/config/environment_bonuses.json`)

Current location: `src/game/location.rs` (lines 53-83)

```json
{
  "environments": [
    {
      "type": "Grassland",
      "name_zh": "草地",
      "stat_bonuses": {
        "speed": 1.1
      }
    },
    {
      "type": "Forest",
      "name_zh": "森林",
      "stat_bonuses": {
        "defense": 1.1
      }
    },
    {
      "type": "Cave",
      "name_zh": "山洞",
      "stat_bonuses": {
        "sp_attack": 1.1
      }
    },
    {
      "type": "Water",
      "name_zh": "水域",
      "stat_bonuses": {
        "sp_defense": 1.1
      }
    },
    {
      "type": "Mountain",
      "name_zh": "山脉",
      "stat_bonuses": {
        "attack": 1.1
      }
    },
    {
      "type": "City",
      "name_zh": "城市",
      "stat_bonuses": {
        "attack": 1.05,
        "defense": 1.05,
        "sp_attack": 1.05,
        "sp_defense": 1.05,
        "speed": 1.05
      }
    }
  ]
}
```

### 6. Natures (`assets/pokemon/natures.json`)

Current location: `src/pokemon_generator/mod.rs` (lines 104-268)

```json
{
  "natures": [
    {
      "id": "Hardy",
      "name_en": "Hardy",
      "name_zh": "固执",
      "stat_modifiers": {}
    },
    {
      "id": "Lonely",
      "name_en": "Lonely",
      "name_zh": "孤独",
      "stat_modifiers": {
        "attack": 1.1,
        "defense": 0.9
      }
    },
    {
      "id": "Adamant",
      "name_en": "Adamant",
      "name_zh": "倔强",
      "stat_modifiers": {
        "attack": 1.1,
        "sp_attack": 0.9
      }
    },
    {
      "id": "Naughty",
      "name_en": "Naughty",
      "name_zh": "调皮",
      "stat_modifiers": {
        "attack": 1.1,
        "sp_defense": 0.9
      }
    },
    {
      "id": "Brave",
      "name_en": "Brave",
      "name_zh": "勇敢",
      "stat_modifiers": {
        "attack": 1.1,
        "speed": 0.9
      }
    },
    {
      "id": "Bold",
      "name_en": "Bold",
      "name_zh": "大胆",
      "stat_modifiers": {
        "defense": 1.1,
        "attack": 0.9
      }
    },
    {
      "id": "Docile",
      "name_en": "Docile",
      "name_zh": "和气",
      "stat_modifiers": {}
    },
    {
      "id": "Impish",
      "name_en": "Impish",
      "name_zh": "调皮",
      "stat_modifiers": {
        "defense": 1.1,
        "sp_attack": 0.9
      }
    },
    {
      "id": "Lax",
      "name_en": "Lax",
      "name_zh": "懒散",
      "stat_modifiers": {
        "defense": 1.1,
        "sp_defense": 0.9
      }
    },
    {
      "id": "Relaxed",
      "name_en": "Relaxed",
      "name_zh": "悠闲",
      "stat_modifiers": {
        "defense": 1.1,
        "speed": 0.9
      }
    },
    {
      "id": "Modest",
      "name_en": "Modest",
      "name_zh": "谦虚",
      "stat_modifiers": {
        "sp_attack": 1.1,
        "attack": 0.9
      }
    },
    {
      "id": "Mild",
      "name_en": "Mild",
      "name_zh": "温和",
      "stat_modifiers": {
        "sp_attack": 1.1,
        "defense": 0.9
      }
    },
    {
      "id": "Quiet",
      "name_en": "Quiet",
      "name_zh": "冷静",
      "stat_modifiers": {
        "sp_attack": 1.1,
        "speed": 0.9
      }
    },
    {
      "id": "Rash",
      "name_en": "Rash",
      "name_zh": "浮躁",
      "stat_modifiers": {
        "sp_attack": 1.1,
        "sp_defense": 0.9
      }
    },
    {
      "id": "Calm",
      "name_en": "Calm",
      "name_zh": "平和",
      "stat_modifiers": {
        "sp_defense": 1.1,
        "attack": 0.9
      }
    },
    {
      "id": "Gentle",
      "name_en": "Gentle",
      "name_zh": "温柔",
      "stat_modifiers": {
        "sp_defense": 1.1,
        "defense": 0.9
      }
    },
    {
      "id": "Sassy",
      "name_en": "Sassy",
      "name_zh": "自大",
      "stat_modifiers": {
        "sp_defense": 1.1,
        "speed": 0.9
      }
    },
    {
      "id": "Careful",
      "name_en": "Careful",
      "name_zh": "谨慎",
      "stat_modifiers": {
        "sp_defense": 1.1,
        "sp_attack": 0.9
      }
    },
    {
      "id": "Quirky",
      "name_en": "Quirky",
      "name_zh": "古怪",
      "stat_modifiers": {}
    },
    {
      "id": "Sly",
      "name_en": "Sly",
      "name_zh": "狡猾",
      "stat_modifiers": {
        "speed": 1.1,
        "attack": 0.9
      }
    },
    {
      "id": "Timid",
      "name_en": "Timid",
      "name_zh": "胆小",
      "stat_modifiers": {
        "speed": 1.1,
        "attack": 0.9
      }
    },
    {
      "id": "Hasty",
      "name_en": "Hasty",
      "name_zh": "急躁",
      "stat_modifiers": {
        "speed": 1.1,
        "defense": 0.9
      }
    },
    {
      "id": "Jolly",
      "name_en": "Jolly",
      "name_zh": "爽朗",
      "stat_modifiers": {
        "speed": 1.1,
        "sp_attack": 0.9
      }
    },
    {
      "id": "Naive",
      "name_en": "Naive",
      "name_zh": "天真",
      "stat_modifiers": {
        "speed": 1.1,
        "sp_defense": 0.9
      }
    }
  ],
  "talent_probability": {
    "normal_percent": 90,
    "hidden_percent": 10
  }
}
```

## Implementation Architecture

### Data Loading Strategy

```
Startup Sequence:
  ↓
loader::load_all_data() [Already Exists]
  ├─ Load pokemon/species.json
  ├─ Load pokemon/moves.json
  ├─ Load locations/world.json
  ├─ Load npcs/trainers.json
  ├─ Load items/items.json [NEW]
  ├─ Load battle/type_effectiveness.json [NEW]
  ├─ Load config/game_constants.json [NEW]
  ├─ Load config/player_defaults.json [NEW]
  ├─ Load config/environment_bonuses.json [NEW]
  └─ Load pokemon/natures.json [NEW]
  ↓
validator::validate_all_data() [Extend]
  ├─ Validate item references
  ├─ Validate type chart completeness
  └─ Validate constant ranges
  ↓
Initialize GameDataCache
  ↓
Game Code Queries Cache via helper functions
  ├─ get_item_by_type()
  ├─ get_type_effectiveness()
  ├─ get_game_constant()
  ├─ get_environment_bonus()
  ├─ get_nature_by_id()
  └─ get_player_defaults()
```

### Changes by Module

**loader.rs**: Add 5 new load functions
**validator.rs**: Add validation for new data
**item.rs**: Replace hardcoded data with cache queries
**battle.rs**: Replace type chart with JSON lookup
**pokemon.rs**: Remove hardcoded constants
**location.rs**: Remove hardcoded environment bonuses
**pokemon_generator/mod.rs**: Remove hardcoded natures
**game/player.rs**: Load defaults from JSON
**handlers/encounter_manager.rs**: Load capture constants from JSON

## Error Handling

All errors during JSON load/validation:
- Display clear error message
- Game exits gracefully
- User knows what file is missing/invalid

## Testing Strategy

1. Unit tests for each JSON loader function
2. Validation tests for schema correctness
3. Integration tests for game startup
4. Round-trip tests (load → use → verify)

## Backward Compatibility

✅ **100% Backward Compatible**
- Game logic unchanged
- API signatures unchanged
- Save files work the same
- Player experience identical

## Future Enhancements

1. **Hot-reloading** - Reload JSON without restart
2. **Balance tuning** - Adjust game constants via JSON
3. **Modding support** - Load custom content
4. **i18n system** - Multiple language support
