# 实现任务列表

## Phase 1: 数据准备（优先级：最高）

### Task 1.1: 更新 species.json 添加 initial_moves 字段
- [ ] 为所有现有 Pokemon 物种添加 `initial_moves` 数组
- [ ] 确保每个 Pokemon 至少有 1 个招式
- [ ] 优先选择与 Pokemon 类型匹配的招式
- [ ] 文件路径: `/Users/datagrand/Desktop/poke/assets/pokemon/species.json`

**参考数据：**
```json
{
  "id": 1,
  "name": "妙蛙种子",
  "initial_moves": [1, 74]  // 撞击 + 叶片
},
{
  "id": 4,
  "name": "小火龙",
  "initial_moves": [1, 33]  // 撞击 + 火焰
},
{
  "id": 7,
  "name": "杰尼龟",
  "initial_moves": [1, 55]  // 撞击 + 水枪
}
```

### Task 1.2: 验证 moves.json 包含所有必需的招式
- [ ] 确认 moves.json 包含所有 initial_moves 中引用的招式 ID
- [ ] 验证招式数据的完整性（power, accuracy, pp 等字段）
- [ ] 文件路径: `/Users/datagrand/Desktop/poke/assets/pokemon/moves.json`

## Phase 2: 数据结构更新（优先级：高）

### Task 2.1: 更新 PokemonSpeciesJSON 结构体
- [ ] 在 `json_schemas.rs` 中添加 `initial_moves: Option<Vec<u32>>` 字段
- [ ] 文件路径: `/Users/datagrand/Desktop/poke/src/data/json_schemas.rs`
- [ ] 代码位置: 第 8-19 行，`PokemonSpeciesJSON` 结构体定义

### Task 2.2: 更新 Move 结构体的字段映射
- [ ] 确保 Move 结构体字段与 JSON 数据匹配
- [ ] 处理 `move_type` vs `type` 字段的兼容性
- [ ] 文件路径: `/Users/datagrand/Desktop/poke/src/game/pokemon.rs`
- [ ] 代码位置: 第 32-42 行，`Move` 结构体定义

## Phase 3: 招式加载功能（优先级：高）

### Task 3.1: 实现 get_move_by_id() 函数
- [ ] 在 `pokemon_data.rs` 中添加 `get_move_by_id(id: u32) -> Option<Move>` 函数
- [ ] 从 game_data 缓存中查找招式
- [ ] 解析 JSON 数据为 Move 结构体
- [ ] 文件路径: `/Users/datagrand/Desktop/poke/src/data/pokemon_data.rs`

**函数签名：**
```rust
pub fn get_move_by_id(id: u32) -> Option<Move> {
    let game_data = loader::get_game_data()?;

    let move_data = game_data.moves.iter().find(|m| {
        m.get("id").and_then(|v| v.as_u64()).map(|v| v as u32) == Some(id)
    })?;

    parse_move_from_json(move_data)
}
```

### Task 3.2: 实现 parse_move_from_json() 辅助函数
- [ ] 创建 JSON 到 Move 结构体的转换函数
- [ ] 处理可选字段（如 power, accuracy）
- [ ] 正确映射 move_type（Physical/Special/Status）
- [ ] 正确映射 pokemon_type（Fire/Water/Grass 等）
- [ ] 文件路径: `/Users/datagrand/Desktop/poke/src/data/pokemon_data.rs`

**实现要点：**
```rust
fn parse_move_from_json(data: &serde_json::Value) -> Option<Move> {
    let id = data.get("id")?.as_u64()? as u32;
    let name = data.get("name")?.as_str()?.to_string();

    // 处理 "type" vs "move_type" 字段
    let type_str = data.get("type")
        .or_else(|| data.get("move_type"))
        .and_then(|v| v.as_str())?;

    let category = data.get("category")?.as_str()?;

    // 解析 MoveType: Physical, Special, Status
    let move_type = match category {
        "Physical" => MoveType::Physical,
        "Special" => MoveType::Special,
        "Status" => MoveType::Status,
        _ => return None,
    };

    // 解析 PokemonType
    let pokemon_type = parse_pokemon_type(type_str)?;

    // 处理可选的 power 和 accuracy
    let power = data.get("power")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;

    let accuracy = data.get("accuracy")
        .and_then(|v| v.as_u64())
        .unwrap_or(100) as u32;

    let pp = data.get("pp")?.as_u64()? as u32;

    Some(Move {
        id,
        name,
        move_type,
        pokemon_type,
        power,
        accuracy,
        pp,
        max_pp: pp,
    })
}
```

### Task 3.3: 实现 parse_pokemon_type() 辅助函数
- [ ] 创建字符串到 PokemonType 枚举的转换函数
- [ ] 处理所有 18 种 Pokemon 类型
- [ ] 文件路径: `/Users/datagrand/Desktop/poke/src/data/pokemon_data.rs`

**实现代码：**
```rust
fn parse_pokemon_type(type_str: &str) -> Option<PokemonType> {
    match type_str {
        "Normal" => Some(PokemonType::Normal),
        "Fire" => Some(PokemonType::Fire),
        "Water" => Some(PokemonType::Water),
        "Grass" => Some(PokemonType::Grass),
        "Electric" => Some(PokemonType::Electric),
        "Ice" => Some(PokemonType::Ice),
        "Fighting" => Some(PokemonType::Fighting),
        "Poison" => Some(PokemonType::Poison),
        "Ground" => Some(PokemonType::Ground),
        "Flying" => Some(PokemonType::Flying),
        "Psychic" => Some(PokemonType::Psychic),
        "Bug" => Some(PokemonType::Bug),
        "Rock" => Some(PokemonType::Rock),
        "Ghost" => Some(PokemonType::Ghost),
        "Dragon" => Some(PokemonType::Dragon),
        "Dark" => Some(PokemonType::Dark),
        "Steel" => Some(PokemonType::Steel),
        "Fairy" => Some(PokemonType::Fairy),
        _ => None,
    }
}
```

## Phase 4: Pokemon 初始化更新（优先级：最高）

### Task 4.1: 更新 get_pokemon_by_id() 函数添加招式初始化
- [ ] 在创建 Pokemon 后，从 JSON 读取 initial_moves
- [ ] 调用 get_move_by_id() 加载每个招式
- [ ] 调用 pokemon.add_move() 添加招式到 Pokemon
- [ ] 实现后备机制：如果没有 initial_moves，添加默认"撞击"招式
- [ ] 文件路径: `/Users/datagrand/Desktop/poke/src/data/pokemon_data.rs`
- [ ] 代码位置: 第 4-88 行，`get_pokemon_by_id()` 函数

**修改方案：**
```rust
pub fn get_pokemon_by_id(id: u32) -> Option<Pokemon> {
    let game_data = loader::get_game_data()?;
    let pokemon_data = game_data.pokemon.iter().find(|p| {
        p.get("id").and_then(|v| v.as_u64()).map(|v| v as u32) == Some(id)
    })?;

    // ... 现有的解析逻辑 ...

    let mut pokemon = Pokemon::new(
        id,
        name,
        (primary_type, secondary_type),
        stats,
        catch_rate,
    );

    // === 新增：加载初始招式 ===
    if let Some(initial_moves) = pokemon_data.get("initial_moves")
        .and_then(|v| v.as_array()) {
        for move_id_value in initial_moves {
            if let Some(move_id) = move_id_value.as_u64() {
                if let Some(move_data) = get_move_by_id(move_id as u32) {
                    pokemon.add_move(move_data);
                }
            }
        }
    }

    // 后备机制：确保至少有一个招式
    if pokemon.moves.is_empty() {
        if let Some(tackle) = get_move_by_id(1) {  // ID 1 是"撞击"
            pokemon.add_move(tackle);
        }
    }

    Some(pokemon)
}
```

### Task 4.2: 创建默认招式工厂函数（可选）
- [ ] 创建 `create_default_tackle()` 函数作为最终后备
- [ ] 如果 JSON 加载完全失败，使用硬编码的"撞击"招式
- [ ] 文件路径: `/Users/datagrand/Desktop/poke/src/data/pokemon_data.rs`

**实现代码：**
```rust
fn create_default_tackle() -> Move {
    Move {
        id: 1,
        name: "撞击".to_string(),
        move_type: MoveType::Physical,
        pokemon_type: PokemonType::Normal,
        power: 40,
        accuracy: 100,
        pp: 35,
        max_pp: 35,
    }
}
```

## Phase 5: 存档兼容性（优先级：中）

### Task 5.1: 添加存档迁移逻辑
- [ ] 在加载存档时检查每个 Pokemon 的 moves 数组
- [ ] 如果 moves 为空，调用招式初始化逻辑
- [ ] 文件路径: `/Users/datagrand/Desktop/poke/src/utils/save.rs` 或 `/Users/datagrand/Desktop/poke/src/game/player.rs`

**建议位置：Player 加载后**
```rust
impl Player {
    pub fn load_from_save() -> Result<Self, String> {
        let mut player = // ... 加载逻辑 ...

        // 迁移逻辑：为没有招式的 Pokemon 添加招式
        for pokemon in &mut player.pokemons {
            if pokemon.moves.is_empty() {
                // 重新初始化招式
                if let Some(species_data) = get_pokemon_species_data(pokemon.id) {
                    load_initial_moves_for_pokemon(pokemon, &species_data);
                }
            }
        }

        Ok(player)
    }
}
```

## Phase 6: 测试（优先级：高）

### Task 6.1: 编写单元测试
- [ ] 测试 `get_move_by_id()` 函数
- [ ] 测试 `parse_move_from_json()` 函数
- [ ] 测试 `parse_pokemon_type()` 函数
- [ ] 测试 `get_pokemon_by_id()` 返回的 Pokemon 有招式
- [ ] 文件路径: `/Users/datagrand/Desktop/poke/tests/pokemon_moves_test.rs`（新建）

**测试示例：**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pokemon_has_initial_moves() {
        // 加载游戏数据
        loader::load_all_data().ok();

        // 创建妙蛙种子
        let bulbasaur = get_pokemon_by_id(1).unwrap();

        // 验证有招式
        assert!(!bulbasaur.moves.is_empty(), "Pokemon should have moves");
        assert!(bulbasaur.moves.len() >= 1, "Pokemon should have at least 1 move");

        // 验证招式数据
        let first_move = &bulbasaur.moves[0];
        assert_eq!(first_move.id, 1);  // 撞击
        assert_eq!(first_move.name, "撞击");
    }

    #[test]
    fn test_get_move_by_id() {
        loader::load_all_data().ok();

        let tackle = get_move_by_id(1).unwrap();
        assert_eq!(tackle.id, 1);
        assert_eq!(tackle.name, "撞击");
        assert_eq!(tackle.power, 40);
        assert_eq!(tackle.pp, 35);
    }
}
```

### Task 6.2: 编写集成测试
- [ ] 测试战斗系统中招式选择功能
- [ ] 测试捕获 Pokemon 后可以使用招式
- [ ] 测试招式菜单显示正确
- [ ] 文件路径: `/Users/datagrand/Desktop/poke/tests/battle_moves_integration_test.rs`（新建）

**测试示例：**
```rust
#[test]
fn test_battle_with_moves() {
    loader::load_all_data().ok();

    let mut player = Player::new("Test Player".to_string());
    let pokemon = get_pokemon_by_id(1).unwrap();
    player.add_pokemon(pokemon);

    let wild_pokemon = get_pokemon_by_id(4).unwrap();
    let battle = Battle::new_wild_battle(&mut player, wild_pokemon);

    // 验证玩家 Pokemon 有招式
    let player_pokemon = battle.get_player_pokemon().unwrap();
    assert!(!player_pokemon.moves.is_empty(), "Player Pokemon should have moves in battle");
}
```

### Task 6.3: 手动测试
- [ ] 启动游戏，捕获一个 Pokemon
- [ ] 进入战斗，选择"使用招式"
- [ ] 验证招式菜单显示招式列表（不再是 0-0）
- [ ] 验证可以选择并使用招式
- [ ] 验证招式消耗 PP

## Phase 7: 文档和清理（优先级：低）

### Task 7.1: 更新代码注释
- [ ] 为新增的函数添加文档注释
- [ ] 解释招式初始化流程
- [ ] 说明后备机制

### Task 7.2: 更新 CHANGELOG
- [ ] 记录 Bug 修复
- [ ] 记录新增的招式初始化功能
- [ ] 说明数据格式变更

## 验收标准

完成所有任务后，系统应该满足以下条件：

1. ✅ 所有新创建的 Pokemon 都有至少 1 个招式
2. ✅ 战斗中"选择招式"菜单显示正确的招式列表（例如："选择 (0-2)"表示有 2 个招式）
3. ✅ 玩家可以正常选择和使用招式
4. ✅ 招式从 JSON 数据文件加载，不是硬编码
5. ✅ 存在后备机制，即使数据缺失也能保证 Pokemon 有招式
6. ✅ 所有测试通过
7. ✅ 现有功能不受影响（向后兼容）

## 优先级排序

**立即执行（阻塞性 Bug）：**
1. Phase 1: 数据准备
2. Phase 3: 招式加载功能
3. Phase 4: Pokemon 初始化更新

**高优先级（核心功能）：**
4. Phase 2: 数据结构更新
5. Phase 6: 测试

**中优先级（兼容性）：**
6. Phase 5: 存档兼容性

**低优先级（维护性）：**
7. Phase 7: 文档和清理
