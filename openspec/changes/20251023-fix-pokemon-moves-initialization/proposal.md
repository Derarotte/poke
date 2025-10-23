# 修复宝可梦招式初始化 Bug

## 问题描述

当前系统存在一个严重的 Bug：当 Pokemon 被添加到玩家队伍时，它们的 `moves` 数组为空，导致战斗中的"选择招式"菜单显示"选择 (0-0)"，玩家无法使用任何招式进行战斗。

### 问题根源

在 `/Users/datagrand/Desktop/poke/src/game/pokemon.rs` 第 97 行，`Pokemon::new()` 构造函数中，`moves` 字段被初始化为空数组：

```rust
moves: vec![],
```

这导致：
1. 所有新创建的 Pokemon 都没有招式
2. 战斗系统中无法选择任何招式
3. 玩家无法进行正常的战斗

### 影响范围

- **捕获系统**：捕获的野生 Pokemon 没有招式
- **战斗系统**：无法使用招式进行战斗
- **游戏可玩性**：严重影响核心玩法

## 目标

1. 为新创建的 Pokemon 自动初始化默认招式
2. 确保 Pokemon 至少拥有一个可用的招式
3. 招式应该与 Pokemon 的属性类型相匹配
4. 从 JSON 数据文件中加载招式信息（而不是硬编码）

## 成功标准

- [ ] 所有新创建的 Pokemon 都有至少 1 个招式
- [ ] 战斗中可以正常选择和使用招式
- [ ] 招式从 JSON 数据文件中加载
- [ ] 招式与 Pokemon 类型匹配（优先）
- [ ] 现有测试通过
- [ ] 添加新的单元测试验证招式初始化

## 相关文件

- `/Users/datagrand/Desktop/poke/src/game/pokemon.rs` - Pokemon 数据结构
- `/Users/datagrand/Desktop/poke/src/data/pokemon_data.rs` - Pokemon 数据加载
- `/Users/datagrand/Desktop/poke/src/data/loader.rs` - 数据加载器
- `/Users/datagrand/Desktop/poke/assets/pokemon/species.json` - Pokemon 物种数据
- `/Users/datagrand/Desktop/poke/assets/pokemon/moves.json` - 招式数据
