# 队伍管理与战斗系统 - OpenSpec Change Proposal

**ID**: `20251022-team-management-and-battle`
**Status**: Proposed
**Created**: 2025-10-22
**Scope**: Team Management UI, Pokémon Storage System, Complete Battle System

## 问题陈述 (Problem Statement)

当前游戏存在以下限制：

1. **队伍信息展示不完整** - 查看队伍时只显示名字和状态，无法看到具体属性
2. **缺少宠物管理系统** - 仅有6只队伍位置，无法管理多余的宠物
3. **战斗系统不完整** - 战斗选项不完整，缺少完整的战斗流程

## 目标 (Goals)

1. ✅ 实现完整的队伍详细信息展示（等级、属性、招式、捕捉球信息）
2. ✅ 建立宠物仓库系统，支持无限存储宠物
3. ✅ 实现宠物放生、查询、存储/取出功能
4. ✅ 完整实现战斗系统，包括所有战斗选项和胜败判定

## 范围 (Scope)

### Included
- 队伍详细信息菜单（stats, moves, catch info）
- 宠物仓库数据结构和管理方法
- 放生、查询、存储取出功能
- 完整战斗系统（目前部分实现）
- 相关CLI菜单更新

### Not Included (Future)
- 宠物交换/交易系统
- 宠物繁殖系统
- 竞技场特殊规则

## 架构概览 (Architecture Overview)

### 核心设计决策

1. **仓库存储**
   - 在 Player 结构中添加 `storage_box: Vec<Pokemon>`
   - 仓库容量无限制，便于玩家保存任意数量的宠物
   - 存档时随之保存

2. **队伍详细信息**
   - 创建新菜单结构 `TeamDetailMenu` 用于展示详细信息
   - 支持单个宝可梦详情查看和编辑

3. **战斗系统**
   - 扩展既有 `Battle` 结构体
   - 支持多轮战斗和完整的胜负判定
   - 集成宝可梦交换、道具使用等完整流程

## 相关规范 (Related Specs)

- `pokemon-core` - 宝可梦基础数据结构
- `location-binding-system` - 位置系统（战斗发生在特定位置）
- `pokemon-revival-system` - 复活系统（战斗后宝可梦可能昏迷）

## 下一步 (Next Steps)

1. 查看 `design.md` 了解详细的架构设计
2. 查看 `tasks.md` 了解实现任务分解
3. 查看 `specs/` 目录下的具体功能规范
4. 运行 `openspec apply 20251022-team-management-and-battle` 开始实现

---

**Specification Files**:
- `specs/team-detail-system/spec.md` - 队伍详细信息系统规范
- `specs/storage-system/spec.md` - 宠物仓库系统规范
- `specs/battle-system/spec.md` - 战斗系统规范

**Design Document**:
- `design.md` - 架构设计和实现细节

**Implementation Plan**:
- `tasks.md` - 19 个实现任务，分为 4 个阶段
