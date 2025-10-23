# OpenSpec Proposal: 修复 Pokemon 招式初始化 Bug

**提案 ID:** 20251023-fix-pokemon-moves-initialization
**创建日期:** 2025-10-23
**状态:** 待审批
**优先级:** 紧急（阻塞性 Bug）

## 概述

修复当前系统中 Pokemon 创建时招式数组为空的严重 Bug。该 Bug 导致玩家在战斗中无法使用任何招式，严重影响游戏可玩性。

## 问题描述

当 Pokemon 被添加到玩家队伍时（通过捕获或其他方式），它们的 `moves` 数组为空。这导致：

- 战斗菜单显示"选择 (0-0)"
- 玩家无法选择任何招式
- 游戏核心玩法（战斗系统）无法正常工作

**截图证据：** 用户提供的战斗截图显示了这个问题。

## 解决方案

采用**数据驱动**的方法：

1. 在 `species.json` 中为每个 Pokemon 物种添加 `initial_moves` 字段
2. 实现从 `moves.json` 加载招式数据的功能
3. 在 `Pokemon::new()` 创建实例时自动加载初始招式
4. 提供后备机制：如果数据缺失，添加默认"撞击"招式

## 文档结构

```
20251023-fix-pokemon-moves-initialization/
├── README.md        # 本文件 - 提案概览
├── proposal.md      # 详细问题描述和目标
├── design.md        # 技术设计方案和架构决策
└── tasks.md         # 完整的实现任务清单（7个阶段，20+ 任务）
```

## 快速开始

### 审查提案

1. 阅读 `proposal.md` 了解问题背景和目标
2. 阅读 `design.md` 了解技术方案选择
3. 阅读 `tasks.md` 了解具体实现步骤

### 实施提案

使用 OpenSpec 命令：

```bash
/openspec:apply 20251023-fix-pokemon-moves-initialization
```

或者按照 `tasks.md` 中的任务列表逐步手动实施。

## 影响范围

### 修改的文件

**数据文件：**
- `/Users/datagrand/Desktop/poke/assets/pokemon/species.json` - 添加 initial_moves 字段
- `/Users/datagrand/Desktop/poke/assets/pokemon/moves.json` - 验证数据完整性

**源代码：**
- `/Users/datagrand/Desktop/poke/src/data/json_schemas.rs` - 更新数据结构
- `/Users/datagrand/Desktop/poke/src/data/pokemon_data.rs` - 实现招式加载和初始化
- `/Users/datagrand/Desktop/poke/src/utils/save.rs` - 存档兼容性（可选）

**测试：**
- `/Users/datagrand/Desktop/poke/tests/pokemon_moves_test.rs` - 新增单元测试
- `/Users/datagrand/Desktop/poke/tests/battle_moves_integration_test.rs` - 新增集成测试

### 受影响的系统

- ✅ 捕获系统
- ✅ 战斗系统
- ✅ Pokemon 初始化
- ⚠️ 存档系统（需要兼容性处理）

## 风险评估

**风险等级：** 低-中

**潜在风险：**
1. 旧存档中的 Pokemon 可能仍然没有招式（需要迁移逻辑）
2. JSON 数据格式错误可能导致加载失败（有后备机制）
3. 性能影响（已评估，可忽略）

**缓解措施：**
- 实现后备机制（默认招式）
- 添加存档迁移逻辑
- 充分的单元测试和集成测试

## 验收标准

- [ ] 所有新创建的 Pokemon 都有至少 1 个招式
- [ ] 战斗中可以正常选择和使用招式
- [ ] 招式从 JSON 数据文件中加载
- [ ] 招式与 Pokemon 类型匹配（优先）
- [ ] 现有测试通过
- [ ] 新增测试通过
- [ ] 旧存档兼容性处理

## 估计工作量

- **数据准备：** 1-2 小时
- **代码实现：** 3-4 小时
- **测试：** 2-3 小时
- **文档：** 1 小时

**总计：** 7-10 小时

## 依赖关系

**前置条件：**
- 游戏数据加载系统已实现（`loader.rs`）
- JSON 数据文件已存在（`species.json`, `moves.json`）

**阻塞问题：**
- 无

## 后续工作

完成此提案后，可以考虑的扩展：

1. 实现基于等级的招式学习系统
2. 实现 TM/HM 招式机器系统
3. 实现招式遗忘和重新学习功能
4. 添加招式效果系统（状态变化、附加伤害等）

## 联系人

**提案创建者：** AI Assistant (Claude)
**审批者：** 待定
**实施者：** 待定

## 变更历史

- **2025-10-23:** 初始提案创建
