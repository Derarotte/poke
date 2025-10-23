# Design Document: Pokemon Detail View

## Architecture

### UI Flow

```
Main Menu (Option 3: 查看队伍)
    ↓
Team List View (Lists all 6 Pokemon)
    ↓ (Select Pokemon)
Pokemon Detail View (Comprehensive info)
    ↓ (Navigation arrows or menu)
    ├→ Previous/Next Pokemon in team
    ├→ Return to Team List
    └→ Back to Main Menu
```

### Data Model

The feature builds on existing structures:

```rust
// From pokemon_generator/mod.rs
PokemonInstance {
    species_id: u32,
    level: u32,
    experience: u32,
    individual_values: IndividualValues,
    talent: Talent,
    nature: Nature,
    unique_id: String,
}

IndividualValues {
    hp: u32,      // 0-31
    attack: u32,
    defense: u32,
    sp_attack: u32,
    sp_defense: u32,
    speed: u32,
}

Nature {
    // Neutral: Hardy, Docile, Serious, Bashful, Quirky
    // With modifiers: Lonely(+ATK,-DEF), Brave(+ATK,-SPD), ...
}

Talent {
    Normal,
    Hidden,
}
```

### UI Components

#### 1. Team List View Component
**Location**: `src/cli/team_list_menu.rs` (new file)

Displays:
- Pokemon position (1-6)
- Name and species
- Level and experience progress
- Current HP / Max HP
- Status (normal, fainted, etc.)
- Quick visual indicator (emoji or symbol)

```
╔════════════════════════════════════════╗
║           你的队伍 (1-6)              ║
╠════════════════════════════════════════╣
║ 1. 皮卡丘 Lv.25 (⚡ 电)               ║
║    HP: ███████░░ 95/120              ║
║ 2. 妙蛙花 Lv.28 (🌿 草/毒)            ║
║    HP: ████████░ 105/130             ║
║ 3. 喷火龙 Lv.30 (🔥 火/飞)            ║
║    HP: ███░░░░░░ 80/140 (昏迷)       ║
│...
╚════════════════════════════════════════╝
```

#### 2. Pokemon Detail View Component
**Location**: `src/cli/pokemon_detail_menu.rs` (new file)

Displays multiple sections:

##### Section A: Basic Info
```
╔════════════════════════════════════════╗
║        皮卡丘 (种族: 皮卡丘)          ║
╠════════════════════════════════════════╣
║ ID: 025                                ║
║ 类型: 电 / —                           ║
║ 等级: 25                               ║
║ 经验: 2500 / 4200 (59%)               ║
╚════════════════════════════════════════╝
```

##### Section B: Stats (种族值 vs 实际属性)
```
╔════════════════════════════════════════╗
║              属性对比                  ║
╠════════════════════════════════════════╣
║ 种族值 / 实际属性 (IV / 性格修正)     ║
║ HP:      35 / 95   (IV:20 / 1.0×)     ║
║ 攻击:    55 / 62   (IV:18 / 1.0×)     ║
║ 防守:    40 / 48   (IV:12 / 1.0×)     ║
║ 特攻:    50 / 65   (IV:25 / 1.1×)     ║
║ 特防:    50 / 58   (IV:15 / 1.0×)     ║
║ 速度:    90 / 105  (IV:28 / 1.0×)     ║
╚════════════════════════════════════════╝
```

##### Section C: Nature & IV
```
╔════════════════════════════════════════╗
║            性格 / 天赋 / IV           ║
╠════════════════════════════════════════╣
║ 性格: 温顺 (提升特攻 1.1× / 降低防守)║
║ 天赋: 普通天赋 (静电)                 ║
║ 个体值总和: 138/186 (74%)              ║
║   - HP: 20      - 攻击: 18            ║
║   - 防守: 12    - 特攻: 25            ║
║   - 特防: 15    - 速度: 28            ║
╚════════════════════════════════════════╝
```

##### Section D: Moves
```
╔════════════════════════════════════════╗
║              现有招式                  ║
╠════════════════════════════════════════╣
║ 1. 十万伏特 (电/特殊) - 威力:90        ║
║    PP: ██████░░░░ 8/15                 ║
║ 2. 电光一闪 (电/物理) - 威力:40        ║
║    PP: ██████████ 30/30                ║
║ 3. 电击 (电/特殊) - 威力:40            ║
║    PP: ████░░░░░░ 15/25                ║
╚════════════════════════════════════════╝
```

##### Section E: Capture Info
```
╔════════════════════════════════════════╗
║              捕捉信息                  ║
╠════════════════════════════════════════╣
║ 捕捉方式: 精灵球                       ║
║ 捕捉地点: 常青森林                     ║
║ 捕捉日期: 2025-10-22 14:30:45         ║
╚════════════════════════════════════════╝
```

##### Section F: Navigation
```
╔════════════════════════════════════════╗
║ ← 上一个 | 队伍 (1/6) | 下一个 → | 返回║
╚════════════════════════════════════════╝
```

### Interaction Model

**Team List Menu:**
- Input: Selection (1-6) to view details
- Input: 0 to return to main menu
- Display: Quick stat overview

**Detail View Menu:**
- Input: < > or A/D to navigate between team members
- Input: 0 or ESC to return to team list
- Scroll: Page Up/Down for longer displays
- Display: All detailed information about selected Pokemon

### Data Flow

```
Player::pokemons (Vec<PokemonInstance>)
    ↓
Team List View renders 6 Pokemon
    ↓
User selects Pokemon (1-6)
    ↓
Detail View renders:
    1. Get PokemonSpecies from generator
    2. Calculate PokemonStats using individual_values + nature
    3. Format all information for display
    4. Show navigation options
    ↓
User navigates (arrows) or returns
```

### Implementation Details

1. **Reuse existing components**:
   - `PokemonStats` struct for calculated stats
   - `calculate_pokemon_stats()` for stat calculations
   - Existing display utilities

2. **New implementations**:
   - Team list rendering (new)
   - Pokemon detail rendering (new)
   - Navigation between detail views (new)
   - Menu input handling for detail view (new)

3. **Data transformations**:
   - Convert PokemonInstance → PokemonStats (using calculate_pokemon_stats)
   - Format IV/Nature display
   - Calculate experience percentage
   - Convert timestamps to dates

## Testing Strategy

- Unit tests for stat calculations with IVs and Nature
- Integration tests for menu flow
- Visual inspection of output formatting
