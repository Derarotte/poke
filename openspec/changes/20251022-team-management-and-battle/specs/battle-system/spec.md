# 战斗系统规范

## ADDED Requirements

### Requirement: 完整的战斗回合制系统

战斗应该遵循完整的回合制流程，支持所有常见的战斗行动。

#### Scenario: 基本战斗流程

```
Given: 玩家与野生宝可梦或 NPC 进入战斗
When: 战斗开始
Then: 应该遵循以下流程:
  1. 显示双方宝可梦的 HP 和状态
  2. 显示战斗菜单 (招式/道具/切换/逃跑)
  3. 玩家和对手轮流行动
  4. 直到一方全部宝可梦昏迷或逃跑成功
```

#### Scenario: 速度决定行动顺序

```
Given: 玩家宝可梦速度 50，对手宝可梦速度 80
When: 同一回合双方都选择攻击
Then: 对手应该先行动
And: 如果对手的攻击导致玩家宝可梦昏迷，玩家无法行动
```

### Requirement: 招式系统

玩家和对手应该能够使用招式进行攻击。

#### Scenario: 选择和使用招式

```
Given: 玩家的宝可梦有 4 个招式
When: 玩家选择 "使用招式"
Then: 应该显示:
  - 招式名字
  - 招式类型 (Physical/Special/Status)
  - 宝可梦类型加成
  - 威力 (Power)
  - 命中率 (Accuracy)
  - 当前 PP / 最大 PP
And: 玩家选择后计算伤害并执行
```

#### Scenario: 计算招式伤害

```
Given: 玩家宝可梦使用 "十万伏特" (威力 90, 电属性)
And: 对手是水系宝可梦 (双倍伤害)
When: 攻击命中
Then: 伤害计算应该包含:
  - 基础伤害 = (((2 * Attacker_Level / 5 + 2) * Power * Attacker_Attack / Defender_Defense) / 50 + 2) * Modifier
  - Modifier = 属性克制 * 随机因数 (0.85 ~ 1.0)
```

#### Scenario: 招式 PP 消耗

```
Given: 招式有 PP (Points)
When: 招式被使用
Then: PP 应该减少 1
And: 当 PP 为 0 时，该招式无法使用
And: 提示 "XXX 的 PP 用完了！"
```

### Requirement: 道具使用系统

玩家应该能够在战斗中使用道具。

#### Scenario: 使用恢复药恢复 HP

```
Given: 玩家拥有 3 个 "恢复药" (回复 20 HP)
And: 队伍中某只宝可梦 HP 为 10/40
When: 玩家在战斗中选择 "使用道具"
Then: 应该显示可用的战斗道具列表
And: 玩家选择 "恢复药" 后选择目标宝可梦
And: 该宝可梦 HP 恢复 20 (变为 30/40)
And: 道具数量减少 1 (2 个剩余)
```

#### Scenario: 战斗状态道具 (如果支持)

```
Given: 玩家拥有 "解毒药"
And: 队伍中某只宝可梦被中毒
When: 玩家使用 "解毒药"
Then: 中毒状态应该被清除
```

### Requirement: 宝可梦切换系统

玩家应该能够在战斗中切换活跃宝可梦。

#### Scenario: 切换活跃宝可梦

```
Given: 玩家当前宝可梦 HP 较低
And: 队伍中还有其他未昏迷的宝可梦
When: 玩家选择 "切换宝可梦"
Then: 应该显示可用的未昏迷宝可梦列表
And: 玩家选择一只后，该宝可梦替换当前宝可梦
And: 对手可以在玩家切换时进行一次无代价的攻击（可选）
```

#### Scenario: 活动宝可梦全部昏迷

```
Given: 玩家队伍中只有 1 只活跃宝可梦
When: 该宝可梦昏迷
Then: 游戏应该检查是否还有其他活跃宝可梦
And: 如果有，玩家必须选择切换
And: 如果没有，玩家战斗失败
```

### Requirement: 逃跑系统

玩家应该能够尝试逃离战斗。

#### Scenario: 逃跑成功和失败

```
Given: 玩家在野生宝可梦战斗中
When: 玩家选择 "逃跑"
Then: 应该有 60% 的成功率逃离
And: 失败时对手进行一次攻击，然后继续战斗
And: 成功时显示 "成功逃脱！" 并返回游戏
```

#### Scenario: NPC 战斗无法逃跑

```
Given: 玩家在与 NPC 训练师战斗
When: 玩家尝试选择 "逃跑"
Then: 应该显示 "无法从训练师战斗中逃脱！"
And: 逃跑选项应该被禁用或隐藏
```

### Requirement: 战斗结算系统

战斗结束时应该显示结果并分配奖励。

#### Scenario: 战斗胜利

```
Given: 对手宝可梦全部昏迷
When: 战斗结束
Then: 应该显示:
  - "你赢了！" 或类似消息
  - 玩家队伍获得的经验值
  - 获得的金钱奖励
  - 宝可梦升级信息（如果有）
```

#### Scenario: 战斗失败

```
Given: 玩家队伍全部昏迷或逃跑成功
When: 战斗结束
Then: 应该显示:
  - "你输了！" 或类似消息
  - 可选：扣除金钱（如果与 NPC 战斗）
  - 返回上一个游戏状态
```

### Requirement: 野生宝可梦战斗集成

野生宝可梦战斗应该使用完整的战斗系统。

#### Scenario: 野生宝可梦战斗

```
Given: 玩家在地点遭遇野生宝可梦
And: 系统显示了宝可梦预览
When: 玩家选择 "战斗"
Then: 应该进入完整的战斗系统
And: 战斗结束后返回地点探索菜单
```

### Requirement: NPC 训练师战斗集成

NPC 战斗应该支持多只宝可梦。

#### Scenario: NPC 队伍战斗

```
Given: 玩家遇到拥有 3 只宝可梦的 NPC 训练师
When: 开始战斗
Then: 应该遵循以下规则:
  - 玩家和 NPC 轮流选择宝可梦
  - 一只 NPC 宝可梦昏迷后，NPC 自动发送下一只
  - 玩家可以选择切换或继续
  - 所有 NPC 宝可梦昏迷时，玩家胜利
```

---

## MODIFIED Requirements

### Requirement: 扩展既有战斗系统

既有的 Battle 结构体应该扩展以支持完整功能。

#### Scenario: Battle 结构体支持完整流程

```
Given: 系统存在 Battle 结构体
When: 创建战斗实例
Then: Battle 应该支持:
  - 存储双方宝可梦和状态
  - 执行招式、道具、切换等操作
  - 维护战斗日志
  - 计算 turn 顺序
```

---

## Related Capabilities

- **team-detail-system**: 队伍信息 - 战斗中显示宝可梦详情
- **storage-system**: 宠物仓库 - 战斗中可以切换仓库中的宝可梦（前提：先加入队伍）
- **location-binding-system**: 位置系统 - 野生战斗与位置关联
