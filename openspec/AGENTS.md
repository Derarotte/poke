# OpenSpec Agent 指导

## 工作流程

当处理此项目的任务时：

### 1. 提案阶段 (Proposal)
- 使用 `/openspec:proposal` 为新功能创建变更提案
- 在 `openspec/changes/` 目录中创建 markdown 文件
- 包含: 目标、设计、实现步骤、测试计划

### 2. 审批阶段 (Review)
- 提案应该清晰可理解，包含足够细节
- 验证与现有架构的兼容性

### 3. 实施阶段 (Apply)
- 使用 `/openspec:apply <proposal-id>` 执行提案
- 在实施时更新相关的 todo 列表
- 按提案中的步骤逐步实现功能

### 4. 存档阶段 (Archive)
- 功能完成后使用 `/openspec:archive <proposal-id>`
- 更新 `openspec/completed.md`

## 变更提案格式

```markdown
# [功能名称]

## 目标
- 简明扼要的目标描述

## 设计
- 架构决策
- 模块设计
- 数据结构

## 实现
- 具体步骤
- 涉及的文件
- 代码位置

## 测试
- 测试计划
- 验证步骤

## 检查清单
- [ ] 代码完成
- [ ] 测试通过
- [ ] 文档更新
```

## 文件命名规范

- 变更提案: `openspec/changes/YYYYMMDD-feature-name.md`
- 已完成: `openspec/completed.md`
- 项目配置: `openspec/project.md`
