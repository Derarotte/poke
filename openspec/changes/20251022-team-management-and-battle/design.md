# 队伍管理与战斗系统 - 详细设计文档

## 概述

本文档描述了队伍管理系统（详细信息展示+宠物仓库）和完整战斗系统的设计细节。

## 1. 队伍详细信息系统 (Team Detail System)

### 1.1 数据结构

```rust
// src/game/player.rs 中新增
pub struct TeamDetailView {
    pub pokemon: Pokemon,
    pub catch_date: String,        // 捕捉日期
    pub catch_location: String,    // 捕捉地点
    pub catch_method: String,      // 捕捉方式 (精灵球类型)
}

// Pokemon 结构中需要增加
pub struct Pokemon {
    // 既有字段...
    pub caught_with: String,       // 捕捉球类型
    pub caught_location_id: u32,   // 捕捉地点 ID
    pub caught_date: u64,          // 捕捉时间戳
}
```

### 1.2 显示内容

队伍详细信息菜单应显示以下内容：

```
┌─────────────────────────────────┐
│  皮卡丘 (Lv. 25)                │
├─────────────────────────────────┤
│ 宝可梦 ID: 25                    │
│ 经验值: 5000/10000              │
│                                 │
│ 属性信息:                       │
│  HP: 40/45       [████░░░░]     │
│  攻击: 55                        │
│  防守: 40                        │
│  特攻: 50                        │
│  特防: 50                        │
│  速度: 90                        │
│                                 │
│ 招式:                           │
│  1. 闪电 (Electric)             │
│  2. 十万伏特 (Electric)         │
│  3. 铁尾 (Steel)                │
│  4. 雷 (Electric)               │
│                                 │
│ 捕捉信息:                       │
│  捕捉球: 普通精灵球             │
│  地点: 常青森林                 │
│  日期: 2025-10-22              │
└─────────────────────────────────┘
```

## 2. 宠物仓库系统 (Storage System)

### 2.1 数据结构

```rust
// src/game/storage.rs (新建)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonBox {
    pub box_id: u32,              // 箱子编号 (1-20)
    pub name: String,              // 箱子名称
    pub pokemon: Vec<Pokemon>,      // 该箱中的宝可梦 (最多 30 只)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSystem {
    pub boxes: Vec<PokemonBox>,     // 20 个箱子
}

impl StorageSystem {
    pub fn new() -> Self {
        // 初始化 20 个箱子
    }

    pub fn add_pokemon(&mut self, pokemon: Pokemon) -> Result<(), String> {
        // 查找第一个有空位的箱子并放入
    }

    pub fn remove_pokemon(&mut self, box_id: u32, index: usize) -> Result<Pokemon, String> {
        // 从指定箱子移除宝可梦
    }

    pub fn list_box(&self, box_id: u32) -> Result<&[Pokemon], String> {
        // 列出指定箱子的内容
    }

    pub fn release_pokemon(&mut self, box_id: u32, index: usize) -> Result<(), String> {
        // 放生宝可梦（删除该宝可梦）
    }
}
```

### 2.2 仓库操作流程

```
主菜单
  └─ 宠物管理
      ├─ 查看队伍 (6 只)
      │   ├─ 查看详细信息
      │   ├─ 切换宝可梦 (从仓库)
      │   └─ 放入仓库
      │
      ├─ 查看仓库
      │   ├─ 浏览箱子 (1-20)
      │   │   ├─ 取出宝可梦 (到队伍)
      │   │   ├─ 放生宝可梦
      │   │   └─ 重命名箱子
      │   │
      │   └─ 统计信息
      │       ├─ 总宝可梦数
      │       ├─ 仓库使用情况
      │       └─ 稀有宝可梦提示
```

## 3. 战斗系统 (Battle System)

### 3.1 战斗状态机

```
          战斗开始
             ↓
        ┌────────────┐
        │  回合开始  │
        └─────┬──────┘
              ↓
        ┌─────────────────────────────┐
        │  显示战斗状态 & 菜单        │
        │  玩家选择: 招式/道具/交换   │
        └─────────┬───────────────────┘
              ↓
        ┌────────────────────────┐
        │ 计算速度，确定顺序    │
        │ （玩家先或敌方先）    │
        └─────────┬──────────────┘
              ↓
        ┌──────────────────────────┐
        │ 执行玩家行动             │
        │ (使用招式/道具/切换)     │
        └─────────┬────────────────┘
              ↓
        ┌──────────────────────────┐
        │ 检查敌方是否昏迷         │
        └──┬─────────────────────┬──┘
           │                     │
        (是)                    (否)
           │                     │
           ↓                     ↓
        ┌─────────┐  ┌──────────────────────┐
        │ 玩家胜  │  │ 执行敌方行动         │
        │ 结束    │  │ (使用招式/道具/切换)│
        └─────────┘  └─────────┬────────────┘
                              ↓
                    ┌──────────────────────┐
                    │ 检查玩家是否昏迷     │
                    └──┬─────────────────┬──┘
                       │                 │
                    (是)               (否)
                       │                 │
                       ↓                 ↓
                    ┌─────────┐  ┌──────────────┐
                    │ 玩家败  │  │ 回到回合开始 │
                    │ 结束    │  └──────────────┘
                    └─────────┘
```

### 3.2 核心战斗逻辑

```rust
pub struct BattleState {
    pub player_pokemon: Pokemon,
    pub opponent_pokemon: Pokemon,
    pub current_turn: u32,
    pub battle_log: Vec<String>,
}

impl BattleState {
    // 计算招式伤害
    pub fn calculate_damage(
        attacker: &Pokemon,
        defender: &Pokemon,
        move: &Move,
    ) -> u32 { ... }

    // 检查命中
    pub fn check_hit(accuracy: u32) -> bool { ... }

    // 处理宝可梦昏迷
    pub fn is_pokemon_fainted(&self) -> (bool, bool) {
        // 返回 (玩家昏迷, 对手昏迷)
    }

    // 执行招式
    pub fn execute_move(
        &mut self,
        move_idx: usize,
        is_player: bool,
    ) -> Result<(), String> { ... }

    // 使用道具
    pub fn use_item(
        &mut self,
        item_name: &str,
        target_idx: usize,
    ) -> Result<(), String> { ... }

    // 交换宝可梦
    pub fn switch_pokemon(
        &mut self,
        new_pokemon_idx: usize,
        is_player: bool,
    ) -> Result<(), String> { ... }
}
```

### 3.3 战斗选项

1. **使用招式** - 选择一个招式攻击对手
   - 显示招式名、类型、威力、命中率
   - 显示招式PP

2. **使用道具** - 在战斗中使用回复药等
   - 显示可用的战斗道具
   - 支持目标选择（自己或队友）

3. **切换宝可梦** - 替换当前战斗的宝可梦
   - 只显示未昏迷的宝可梦
   - 切换有1回合延迟

4. **逃跑** - 尝试逃离战斗
   - 有60%成功率
   - 失败则对手可以进行一次攻击

## 4. 集成点

### 4.1 与位置系统的集成

- 战斗发生时记录位置信息
- 野生宝可梦战斗与 WildPokemonEncounter 系统集成
- NPC 训练师战斗使用相同的战斗系统

### 4.2 与复活系统的集成

- 战斗结束后检查宝可梦状态
- 集成既有的复活菜单系统

### 4.3 与存档系统的集成

- 宠物仓库数据在存档中保存
- Player 结构扩展存储框结构

## 5. 实现策略

### 分阶段实现

**第1阶段**: 队伍详细信息系统（3 天）
- 增强 Pokemon 数据结构
- 创建队伍详细菜单
- 显示完整信息

**第2阶段**: 宠物仓库系统（4 天）
- 实现存储结构
- 实现仓库管理菜单
- 集成取出/存入功能

**第3阶段**: 战斗系统完整化（5 天）
- 完成战斗状态机
- 实现所有战斗选项
- 添加战斗日志

**第4阶段**: 测试与集成（2 天）
- 单元测试
- 集成测试
- 系统测试

## 6. 风险与缓解

| 风险 | 缓解措施 |
|------|----------|
| 仓库数据过多影响性能 | 使用分页加载，限制显示的宝可梦数量 |
| 战斗系统复杂导致 bug | 详细的战斗日志便于调试 |
| 存档兼容性问题 | 实现向后兼容的迁移机制 |

## 7. 测试计划

- **单元测试**: 各功能模块独立测试
- **集成测试**: 队伍-仓库-战斗流程测试
- **系统测试**: 完整游戏流程测试
- **目标覆盖**: > 80% 代码覆盖率
