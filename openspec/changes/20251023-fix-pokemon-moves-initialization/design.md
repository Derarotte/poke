# 设计方案：Pokemon 招式初始化系统

## 架构概述

本方案旨在建立一个完整的招式初始化系统，使每个 Pokemon 在创建时都能自动获得合适的初始招式。

## 设计方案对比

### 方案 1：在 Pokemon::new() 中添加默认招式（简单方案）

**优点：**
- 实现简单快速
- 最小化代码修改
- 容易理解和维护

**缺点：**
- 需要硬编码默认招式 ID
- 不够灵活，无法根据 Pokemon 类型自动选择招式
- 未来扩展性较差

**实现：**
```rust
impl Pokemon {
    pub fn new(...) -> Self {
        let mut pokemon = Pokemon { ... };

        // 添加默认"撞击"招式 (ID: 1)
        pokemon.add_move(create_tackle_move());

        pokemon
    }
}
```

### 方案 2：从 JSON 数据中加载 Pokemon 的初始招式集（推荐方案）

**优点：**
- 数据驱动，易于配置和修改
- 可以为每个 Pokemon 物种定义不同的初始招式
- 符合现有的数据外部化架构
- 易于扩展和维护

**缺点：**
- 需要修改 JSON schema
- 实现复杂度略高

**实现：**
需要在 `species.json` 中添加 `initial_moves` 字段：

```json
{
  "id": 1,
  "name": "妙蛙种子",
  "primary_type": "Grass",
  "secondary_type": "Poison",
  "base_stats": { ... },
  "initial_moves": [1, 74],  // 撞击 + 叶片
  ...
}
```

### 方案 3：基于等级和类型动态生成招式

**优点：**
- 最灵活和真实
- 符合原版 Pokemon 游戏机制

**缺点：**
- 实现复杂度最高
- 需要完整的招式学习表
- 超出当前 Bug 修复范围

## 最终选择：方案 2（数据驱动）

我们选择方案 2 作为实现方案，原因如下：

1. **符合项目架构**：项目已经采用 JSON 数据外部化，这个方案延续了这一架构
2. **平衡性好**：在实现复杂度和功能完整性之间取得良好平衡
3. **易于扩展**：未来可以轻松添加更多招式或修改初始招式集
4. **维护性强**：非程序员也能通过修改 JSON 文件来调整招式配置

## 数据结构设计

### 1. JSON Schema 扩展

在 `species.json` 中为每个 Pokemon 添加 `initial_moves` 字段：

```json
{
  "species": [
    {
      "id": 1,
      "name": "妙蛙种子",
      "english_name": "Bulbasaur",
      "primary_type": "Grass",
      "secondary_type": "Poison",
      "base_stats": { ... },
      "catch_rate": 45,
      "experience_yield": 64,
      "initial_moves": [1, 74]  // 新增字段
    },
    ...
  ]
}
```

### 2. Rust 数据结构更新

更新 `PokemonSpeciesJSON` 结构体：

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonSpeciesJSON {
    pub id: u32,
    pub name: String,
    pub english_name: Option<String>,
    pub primary_type: String,
    pub secondary_type: Option<String>,
    pub base_stats: BaseStatsJSON,
    pub catch_rate: u32,
    pub experience_yield: u32,
    pub evolution: Option<EvolutionJSON>,
    pub initial_moves: Option<Vec<u32>>,  // 新增字段
}
```

### 3. 招式加载函数

新增招式加载辅助函数：

```rust
// 在 pokemon_data.rs 中
pub fn get_move_by_id(id: u32) -> Option<Move> {
    let game_data = loader::get_game_data()?;
    let move_data = game_data.moves.iter().find(|m| {
        m.get("id").and_then(|v| v.as_u64()).map(|v| v as u32) == Some(id)
    })?;

    // 解析 JSON 为 Move 结构
    parse_move_from_json(move_data)
}

fn parse_move_from_json(data: &serde_json::Value) -> Option<Move> {
    // 解析逻辑
}
```

## 实现流程

```
1. 用户捕获/创建 Pokemon
   ↓
2. 调用 Pokemon::new()
   ↓
3. 查询 species.json 获取 initial_moves
   ↓
4. 从 moves.json 加载每个招式的详细信息
   ↓
5. 调用 pokemon.add_move() 添加招式
   ↓
6. 返回拥有招式的 Pokemon
```

## 回退策略

如果无法从 JSON 加载招式（数据文件损坏、格式错误等），系统应该：

1. 记录错误日志
2. 添加默认的"撞击"招式（ID: 1）作为后备
3. 确保 Pokemon 至少有一个可用招式

```rust
// 后备机制
if pokemon.moves.is_empty() {
    // 添加默认撞击招式
    pokemon.add_move(create_default_tackle());
}
```

## 性能考虑

- **数据缓存**：招式数据已经通过 `loader::get_game_data()` 全局缓存，无需重复加载
- **初始化时机**：只在 Pokemon 创建时加载一次招式
- **内存占用**：每个 Pokemon 最多 4 个招式，内存占用可忽略

## 兼容性

### 向后兼容

- 现有的存档文件（已有 Pokemon 没有招式）需要迁移
- 建议：在加载存档时检查 Pokemon 招式数组，如果为空则自动初始化

### 数据兼容

- `initial_moves` 字段设为 `Option<Vec<u32>>`，允许为空
- 如果 JSON 中没有该字段，使用默认招式（撞击）

## 测试策略

### 单元测试

1. 测试从 JSON 加载招式
2. 测试 Pokemon 创建后有招式
3. 测试后备机制（数据缺失时）
4. 测试招式类型匹配

### 集成测试

1. 测试战斗中招式选择功能
2. 测试捕获 Pokemon 后的招式可用性
3. 测试存档加载后的招式数据
