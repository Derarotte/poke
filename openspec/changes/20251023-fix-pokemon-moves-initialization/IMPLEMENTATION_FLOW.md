# 实现流程图

## 当前状态（有 Bug）

```
用户捕获 Pokemon
    ↓
调用 Pokemon::new()
    ↓
创建 Pokemon { moves: vec![] }  ← 问题所在
    ↓
添加到玩家队伍
    ↓
进入战斗
    ↓
选择"使用招式"
    ↓
显示 "选择 (0-0)" ← 无法使用招式
```

## 修复后的流程

```
用户捕获 Pokemon
    ↓
调用 get_pokemon_by_id(id)
    ↓
从 species.json 读取物种数据
    ↓
解析 initial_moves 字段 [1, 74]
    ↓
遍历每个 move_id
    │
    ├─→ 调用 get_move_by_id(1)
    │       ↓
    │   从 moves.json 查找招式数据
    │       ↓
    │   解析为 Move 结构体
    │       ↓
    │   返回 Move { id: 1, name: "撞击", ... }
    │       ↓
    ├─→ 调用 pokemon.add_move(move)
    │       ↓
    └─→ 添加到 pokemon.moves 数组
    ↓
检查 moves 是否为空
    ↓
如果为空 → 添加默认"撞击"招式（后备机制）
    ↓
返回 Pokemon { moves: [Move, Move, ...] }
    ↓
添加到玩家队伍
    ↓
进入战斗
    ↓
选择"使用招式"
    ↓
显示 "选择 (0-2)" ← 显示 2 个招式
    ↓
玩家可以选择招式战斗 ✓
```

## 数据流

```
assets/pokemon/species.json
    ↓
    {
      "id": 1,
      "name": "妙蛙种子",
      "initial_moves": [1, 74]  ← 配置初始招式
    }
    ↓
loader::get_game_data()
    ↓
GameDataCache { pokemon: [...], moves: [...] }
    ↓
get_pokemon_by_id(1) ────┐
    ↓                    │
    ├─→ 读取物种数据      │
    ├─→ 读取 initial_moves │
    └─→ 遍历 [1, 74]     │
        ↓                │
        ├─→ get_move_by_id(1) ──→ moves.json
        │       ↓
        │   Move { id: 1, name: "撞击", power: 40, ... }
        │       ↓
        ├─→ pokemon.add_move(move)
        │       ↓
        └─→ pokemon.moves.push(move)
            ↓
Pokemon {
    id: 1,
    name: "妙蛙种子",
    moves: [
        Move { id: 1, name: "撞击", ... },
        Move { id: 74, name: "叶片", ... }
    ]
}
```

## 后备机制流程

```
get_pokemon_by_id(id)
    ↓
尝试从 JSON 加载 initial_moves
    ↓
┌──────────────────┐
│ 加载成功？       │
└──────────────────┘
    ↓           ↓
   是          否
    ↓           ↓
正常添加招式   ↓
    ↓       ┌────────────────┐
    ↓       │ moves 为空？   │
    ↓       └────────────────┘
    ↓           ↓        ↓
    ↓          是       否
    ↓           ↓        ↓
    └─→ 尝试 get_move_by_id(1)
            ↓        ↓
        成功    失败
            ↓        ↓
    添加"撞击"  使用硬编码
    招式        create_default_tackle()
            ↓
        Pokemon 保证至少有 1 个招式
```

## Phase 实施顺序

```
Phase 1: 数据准备
    ├─→ Task 1.1: 更新 species.json
    └─→ Task 1.2: 验证 moves.json
        ↓
Phase 2: 数据结构
    ├─→ Task 2.1: 更新 PokemonSpeciesJSON
    └─→ Task 2.2: 更新 Move 结构体
        ↓
Phase 3: 招式加载
    ├─→ Task 3.1: get_move_by_id()
    ├─→ Task 3.2: parse_move_from_json()
    └─→ Task 3.3: parse_pokemon_type()
        ↓
Phase 4: Pokemon 初始化 ← 核心修复
    ├─→ Task 4.1: 更新 get_pokemon_by_id()
    └─→ Task 4.2: create_default_tackle()
        ↓
Phase 5: 存档兼容性
    └─→ Task 5.1: 存档迁移逻辑
        ↓
Phase 6: 测试
    ├─→ Task 6.1: 单元测试
    ├─→ Task 6.2: 集成测试
    └─→ Task 6.3: 手动测试
        ↓
Phase 7: 文档
    ├─→ Task 7.1: 代码注释
    └─→ Task 7.2: CHANGELOG
```

## 关键决策点

### 决策 1: 数据存储位置

**选项：**
- A. 在 Pokemon 结构体中硬编码
- B. 在 species.json 中配置（选择）
- C. 单独的 initial_moves.json 文件

**选择 B 的原因：**
- 符合现有架构
- 易于维护和修改
- 集中管理物种数据

### 决策 2: 后备机制

**选项：**
- A. 不加载就报错
- B. 使用默认招式（选择）
- C. 创建空 Pokemon

**选择 B 的原因：**
- 提高鲁棒性
- 保证游戏可玩性
- 便于调试

### 决策 3: 实现方式

**选项：**
- A. 在 Pokemon::new() 中实现
- B. 在 get_pokemon_by_id() 中实现（选择）
- C. 在 Player::add_pokemon() 中实现

**选择 B 的原因：**
- 职责清晰
- 可以访问 JSON 数据
- 不影响 Pokemon 结构体

## 测试策略

```
单元测试
    ├─→ test_get_move_by_id()
    ├─→ test_parse_move_from_json()
    ├─→ test_parse_pokemon_type()
    └─→ test_pokemon_has_initial_moves()
        ↓
集成测试
    ├─→ test_battle_with_moves()
    ├─→ test_catch_pokemon_has_moves()
    └─→ test_move_menu_display()
        ↓
手动测试
    ├─→ 启动游戏
    ├─→ 捕获 Pokemon
    ├─→ 进入战斗
    └─→ 验证招式菜单
```

## 成功指标

```
✅ Pokemon.moves.len() >= 1
✅ 战斗菜单显示 "选择 (0-N)" 其中 N > 0
✅ 可以选择和使用招式
✅ 招式消耗 PP
✅ 所有测试通过
✅ 旧存档兼容
```
