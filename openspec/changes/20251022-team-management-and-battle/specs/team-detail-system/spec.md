# 队伍详细信息系统规范

## ADDED Requirements

### Requirement: 队伍宝可梦详细信息展示

玩家应该能够查看队伍中每只宝可梦的完整详细信息，包括属性、招式和捕捉信息。

#### Scenario: 查看单只宝可梦详情

```
Given: 玩家的队伍中有一只 Lv.25 的皮卡丘
And: 该皮卡丘在常青森林用普通精灵球捕捉
When: 玩家选择查看该宝可梦详情
Then: 应该显示:
  - 名字: 皮卡丘
  - 等级: 25
  - 宝可梦 ID: 25
  - 类型: 电气
  - 经验值: 5000/10000 (进度条)
  - HP: 40/45 (当前/最大，进度条)
  - 属性 (攻击/防守/特攻/特防/速度)
  - 4 个招式及其类型、威力、命中率、PP
  - 捕捉球类型: 普通精灵球
  - 捕捉地点: 常青森林
  - 捕捉日期: 2025-10-22
```

#### Scenario: 浏览队伍中的多只宝可梦

```
Given: 玩家队伍中有 3 只宝可梦
When: 玩家在队伍详情菜单中按上/下键浏览
Then: 应该逐一显示每只宝可梦的详细信息
And: 显示 "宝可梦 1/3" 等分页提示
```

### Requirement: 增强 Pokemon 数据结构

宝可梦应该记录捕捉信息和其他元数据。

#### Scenario: Pokemon 结构扩展

```
Given: 系统中存在 Pokemon 结构体
When: 创建新的宝可梦
Then: 应该自动记录:
  - caught_with: 捕捉球类型 (String)
  - caught_location_id: 捕捉地点 ID (u32)
  - caught_date: 捕捉时间 (u64 时间戳)
```

### Requirement: 队伍管理选项

玩家在查看队伍详情时应该能够执行管理操作。

#### Scenario: 从队伍放入宠物仓库

```
Given: 玩家正在查看一只宝可梦的详情
And: 该宝可梦不是最后活跃的宝可梦
When: 玩家选择 "放入仓库"
Then: 该宝可梦应该被移动到仓库
And: 显示 "已将 XXX 放入仓库"
```

#### Scenario: 切换队伍宝可梦

```
Given: 玩家的仓库中有宝可梦
And: 玩家的队伍中有空位或想替换某只
When: 玩家选择 "从仓库取出"
Then: 应该显示可用的宝可梦列表
And: 玩家选择后该宝可梦加入队伍
```

---

## MODIFIED Requirements

无修改需求

---

## Related Capabilities

- **storage-system**: 宠物仓库系统 - 存储和管理队伍外的宝可梦
- **pokemon-core**: 宝可梦基础数据 - Pokemon 结构的扩展
