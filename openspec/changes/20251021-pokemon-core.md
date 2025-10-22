# Pokemon 核心游戏系统

## 状态
- 创建时间: 2025-10-21
- 状态: 进行中

## 目标

实现一个功能完整的命令行宝可梦游戏，包括：
- 宝可梦捕捉和培养系统
- 回合制战斗系统
- 玩家背包和数据管理
- 交互式 CLI 菜单界面
- 游戏存档系统

## 设计

### 数据结构

#### Pokemon (宝可梦)
```rust
struct Pokemon {
    id: u32,
    name: String,
    level: u32,
    experience: u32,
    hp: u32,
    max_hp: u32,
    attack: u32,
    defense: u32,
    sp_attack: u32,
    sp_defense: u32,
    speed: u32,
    moves: Vec<Move>,  // 最多4个招式
    pokemon_type: PokemonType,
}
```

#### Move (招式)
```rust
struct Move {
    id: u32,
    name: String,
    move_type: MoveType,
    power: u32,
    accuracy: u32,
    pp: u32,      // 剩余使用次数
    max_pp: u32,
}
```

#### Player (玩家)
```rust
struct Player {
    name: String,
    level: u32,
    pokemons: Vec<Pokemon>,  // 最多6只
    badges: Vec<Badge>,
    items: HashMap<String, u32>,  // 道具数量
    money: u32,
}
```

### 核心模块

1. **game/pokemon.rs** - 宝可梦数据结构和方法
2. **game/player.rs** - 玩家数据结构
3. **game/battle.rs** - 战斗逻辑引擎
4. **data/pokemon_data.rs** - 151只基础宝可梦数据
5. **cli/menu.rs** - 主菜单和交互
6. **utils/save.rs** - 存档加载

### 战斗系统

- 回合制，每回合可选择：招式、道具、切换宝可梦、逃跑
- 伤害计算: `(((2 * level / 5 + 2) * power * attack / defense) / 50 + 2) * random(0.85-1.00)`
- 命中率检测使用随机数
- 属性相克系统（弱点、抵抗）
- 经验值获得: `base_exp * opponent_level / 7`

### 捕捉系统

- 捕捉率 = 基础捕捉率 * (max_hp - current_hp) / max_hp * 球系数
- 随机数 <= 捕捉率 则成功

## 实现步骤

### 第一阶段：数据结构
- [ ] 创建 game/mod.rs 模块结构
- [ ] 实现 Pokemon 结构体和方法 (game/pokemon.rs)
- [ ] 实现 Player 结构体 (game/player.rs)
- [ ] 创建宝可梦数据库 (data/pokemon_data.rs)

### 第二阶段：战斗系统
- [ ] 实现战斗逻辑 (game/battle.rs)
- [ ] 实现伤害计算
- [ ] 实现属性相克系统
- [ ] 实现捕捉和经验系统

### 第三阶段：CLI 界面
- [ ] 创建主菜单结构 (cli/menu.rs)
- [ ] 实现野外遭遇界面
- [ ] 实现战斗界面
- [ ] 实现背包界面
- [ ] 实现宝可梦队伍管理

### 第四阶段：游戏循环
- [ ] 实现主游戏循环 (main.rs)
- [ ] 集成所有模块
- [ ] 添加存档系统 (utils/save.rs)
- [ ] 测试整体流程

## 测试计划

### 单元测试
- [ ] 伤害计算正确性
- [ ] 捕捉率算法
- [ ] 升级和经验系统
- [ ] 属性相克判断

### 集成测试
- [ ] 完整战斗流程
- [ ] 宝可梦捕捉流程
- [ ] 存档和加载
- [ ] 菜单导航

### 手动测试
- [ ] 游戏从开始到完成的流程
- [ ] 各种边界情况
- [ ] 用户交互响应

## 依赖项

```toml
[dependencies]
rand = "0.8"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## 检查清单

- [ ] 宝可梦数据完整
- [ ] 战斗系统完整测试
- [ ] CLI 菜单完整实现
- [ ] 存档系统正常工作
- [ ] 代码文档完整
- [ ] 所有测试通过
- [ ] 游戏可流畅进行

## 交付物

- 完整的 Rust 项目
- 可执行的命令行宝可梦游戏
- 使用文档 (README.md)
- 测试覆盖率 > 80%
