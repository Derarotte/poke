# 快速开始指南

## 30 秒概览

**问题：** Pokemon 没有招式，无法战斗
**解决：** 从 JSON 加载初始招式
**修改：** 4 个文件，约 200 行代码

## 5 分钟实施方案

### 最小可行修复（MVP）

如果时间紧迫，只需完成以下 3 个任务即可修复 Bug：

#### 1. 更新 species.json（2 分钟）

在 `/Users/datagrand/Desktop/poke/assets/pokemon/species.json` 中为每个 Pokemon 添加 `initial_moves` 字段：

```json
{
  "id": 1,
  "name": "妙蛙种子",
  ...
  "initial_moves": [1, 74]  // 添加这一行
}
```

**推荐招式配置：**
```
妙蛙种子 (1): [1, 74]  // 撞击 + 叶片
小火龙 (4): [1, 33]    // 撞击 + 火焰
杰尼龟 (7): [1, 55]    // 撞击 + 水枪
绿毛虫 (10): [1]       // 撞击
皮卡丘 (25): [1, 97]   // 撞击 + 电击
胖可丁 (39): [1]       // 撞击
可达鸭 (54): [1, 55]   // 撞击 + 水枪
卡拉卡拉 (58): [1, 33] // 撞击 + 火焰
超梦 (63): [1]         // 撞击
鲤鱼王 (129): [1]      // 撞击
```

#### 2. 实现招式加载函数（2 分钟）

在 `/Users/datagrand/Desktop/poke/src/data/pokemon_data.rs` 末尾添加：

```rust
use crate::game::{Move, MoveType, PokemonType};

pub fn get_move_by_id(id: u32) -> Option<Move> {
    let game_data = loader::get_game_data()?;

    let move_data = game_data.moves.iter().find(|m| {
        m.get("id").and_then(|v| v.as_u64()).map(|v| v as u32) == Some(id)
    })?;

    parse_move_from_json(move_data)
}

fn parse_move_from_json(data: &serde_json::Value) -> Option<Move> {
    let id = data.get("id")?.as_u64()? as u32;
    let name = data.get("name")?.as_str()?.to_string();
    let type_str = data.get("type").and_then(|v| v.as_str())?;
    let category = data.get("category")?.as_str()?;

    let move_type = match category {
        "Physical" => MoveType::Physical,
        "Special" => MoveType::Special,
        "Status" => MoveType::Status,
        _ => return None,
    };

    let pokemon_type = parse_pokemon_type(type_str)?;
    let power = data.get("power").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
    let accuracy = data.get("accuracy").and_then(|v| v.as_u64()).unwrap_or(100) as u32;
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

#### 3. 更新 get_pokemon_by_id() 函数（1 分钟）

在同一文件中，找到 `get_pokemon_by_id()` 函数（第 79-88 行），在返回前添加：

```rust
pub fn get_pokemon_by_id(id: u32) -> Option<Pokemon> {
    // ... 现有代码 ...

    let mut pokemon = Pokemon::new(
        id,
        name,
        (primary_type, secondary_type),
        stats,
        catch_rate,
    );

    // === 新增代码开始 ===
    // 加载初始招式
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
        if let Some(tackle) = get_move_by_id(1) {
            pokemon.add_move(tackle);
        }
    }
    // === 新增代码结束 ===

    Some(pokemon)
}
```

### 验证修复

```bash
# 编译
cargo build

# 运行游戏
cargo run

# 测试：
# 1. 捕获一个 Pokemon
# 2. 进入战斗
# 3. 选择"使用招式"
# 4. 应该看到招式列表（不再是 0-0）
```

## 完整实施方案

如果有充足时间，按照以下顺序完成所有任务：

### Phase 1: 准备工作（30 分钟）

```bash
# 1. 更新所有 Pokemon 的 initial_moves
vim /Users/datagrand/Desktop/poke/assets/pokemon/species.json

# 2. 验证 moves.json 数据
cat /Users/datagrand/Desktop/poke/assets/pokemon/moves.json | jq '.moves[] | {id, name, type}'
```

### Phase 2-4: 核心实现（2 小时）

按照 `tasks.md` 中的 Phase 2-4 完成：
- 更新数据结构
- 实现招式加载
- 更新 Pokemon 初始化

### Phase 5: 兼容性（1 小时）

添加存档迁移逻辑

### Phase 6: 测试（2 小时）

编写和运行测试

### Phase 7: 文档（30 分钟）

更新注释和 CHANGELOG

## 常见问题

### Q: 如果 initial_moves 字段缺失怎么办？

A: 后备机制会自动添加"撞击"招式（ID: 1），确保 Pokemon 可以战斗。

### Q: 需要修改多少个 Pokemon？

A: species.json 中有多少个物种就需要添加多少个 initial_moves 字段。目前约 10-20 个。

### Q: 旧存档中的 Pokemon 会怎样？

A: 需要实现 Phase 5 的存档迁移逻辑，或者玩家重新捕获 Pokemon。

### Q: 招式 ID 从哪里来？

A: 参考 `/Users/datagrand/Desktop/poke/assets/pokemon/moves.json` 文件中的 id 字段。

### Q: 如果编译错误？

A: 检查：
1. 是否导入了 `Move`, `MoveType`, `PokemonType`
2. 函数签名是否正确
3. JSON 格式是否有效

## 检查清单

**最小修复（5 分钟）：**
- [ ] species.json 添加 initial_moves
- [ ] 实现 get_move_by_id()
- [ ] 更新 get_pokemon_by_id()
- [ ] 编译通过
- [ ] 手动测试通过

**完整修复（8 小时）：**
- [ ] 所有数据文件更新
- [ ] 所有代码实现
- [ ] 单元测试通过
- [ ] 集成测试通过
- [ ] 存档兼容性处理
- [ ] 文档更新

## 参考资源

- **详细设计：** `design.md`
- **任务清单：** `tasks.md`
- **实现流程：** `IMPLEMENTATION_FLOW.md`
- **问题背景：** `proposal.md`

## 获得帮助

如果遇到问题：
1. 查看 `IMPLEMENTATION_FLOW.md` 了解数据流
2. 查看 `design.md` 了解设计决策
3. 查看 `tasks.md` 寻找具体实现代码
4. 检查 JSON 文件格式是否正确

## 下一步

完成修复后：
1. 提交代码
2. 更新 CHANGELOG
3. 标记提案为"已完成"
4. 考虑后续增强（见 README.md）
