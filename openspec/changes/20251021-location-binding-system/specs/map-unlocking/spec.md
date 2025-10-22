# 地图解锁规范

## ADDED Requirements

### Requirement: 渐进地点解锁
地点应该根据玩家进度逐步解锁，而非一次性开放。

#### Scenario: 初始解锁状态
```
Given: 新游戏开始
When: 游戏初始化
Then: 只有常青小镇应该是解锁状态
And: 其他所有地点应该为锁定状态
```

#### Scenario: 等级解锁
```
Given: 玩家队伍当前等级为 3
And: 常青森林需要等级 3 解锁
When: 玩家查看移动菜单
Then: 常青森林应该显示为 "已解锁"
And: 华石镇 (需要等级 5) 应该仍为 "锁定"
```

#### Scenario: 徽章解锁
```
Given: 玩家拥有 1 个徽章
And: 华蓝市需要 1 个徽章解锁
When: 玩家查看移动菜单
Then: 华蓝市应该显示为 "已解锁"
And: 显示条件: "徽章 1/1 ✓"
```

#### Scenario: 多条件解锁
```
Given: 玩家等级为 15，拥有 2 个徽章
And: 浅红市需要 (等级 ≥ 25 或 徽章 ≥ 2)
When: 玩家查看移动菜单
Then: 浅红市应该显示为 "已解锁"
And: 显示满足的条件
```

---

### Requirement: 解锁条件显示
系统应该清晰显示锁定地点的解锁条件。

#### Scenario: 显示缺失条件
```
Given: 玩家等级为 5，无徽章
And: 月见山需要 (等级 ≥ 7 + 击败小刚 的徽章)
When: 玩家查看移动菜单
Then: 应该显示:
  🔒 月见山
    条件: 等级 ≥ 7 (当前 5) ❌
         获得小刚徽章 ❌
```

#### Scenario: 显示部分满足条件
```
Given: 玩家等级为 20，无徽章
And: 华蓝市需要 (等级 ≥ 15 + 徽章 ≥ 1)
When: 玩家查看移动菜单
Then: 应该显示:
  🔒 华蓝市
    条件: 等级 ≥ 15 (当前 20) ✓
         获得徽章 1 个 (当前 0) ❌
```

---

### Requirement: 解锁通知
玩家解锁新地点时应该得到通知。

#### Scenario: 升级后解锁通知
```
Given: 玩家队伍升到 3 级
And: 常青森林需要等级 3
When: 解锁检查触发
Then: 应该显示通知:
  ✓ 新地点已解锁: 常青森林!
```

#### Scenario: 获得徽章后解锁通知
```
Given: 玩家击败馆主美娜获得徽章
And: 华蓝市需要 1 个徽章
When: 解锁检查触发
Then: 应该显示通知:
  ✓ 新地点已解锁: 华蓝市!
```

---

## MODIFIED Requirements

### Requirement: 移动菜单更新
移动菜单应该区分已解锁和未解锁地点。

#### Scenario: 移动菜单内容
```
Given: 玩家位于常青小镇，等级 5，无徽章
When: 玩家打开移动菜单
Then: 应该显示:
  可达地点:
    ✓ 常青森林 (已解锁) → 2级推荐
    🔒 华石镇 (需要等级5) - 满足条件，可解锁

  无法到达:
    ❌ 月见山 (不相邻)
```

### Requirement: 解锁条件数据驱动
所有解锁条件应该定义在数据中，而非硬编码。

#### Scenario: 条件数据结构
```
LocationRequirement {
  required_level: u32,
  required_badges: Vec<BadgeId>,
  required_pokemon_count: Option<u32>,
}
```

