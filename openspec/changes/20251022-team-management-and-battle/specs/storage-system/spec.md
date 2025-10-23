# 宠物仓库系统规范

## ADDED Requirements

### Requirement: 无限容量宝可梦存储

玩家应该能够在宝可梦仓库中存储无限数量的宝可梦，分为多个箱子。

#### Scenario: 创建和管理存储箱

```
Given: 系统提供 20 个存储箱，每个箱子最多 30 只宝可梦
When: 玩家捕捉的宝可梦超过 6 只队伍位置
Then: 额外的宝可梦自动进入仓库第 1 个箱子
And: 当箱子满 (30 只) 时，溢出宝可梦进入下一个箱子
```

#### Scenario: 浏览仓库箱子

```
Given: 玩家的仓库中有宝可梦
When: 玩家进入仓库管理菜单
Then: 应该显示:
  - 箱子列表 (1-20)
  - 每个箱子的使用情况 (e.g. "第 1 箱: 15/30")
  - 仓库总宝可梦数
```

### Requirement: 宝可梦放生系统

玩家应该能够永久删除（放生）不需要的宝可梦。

#### Scenario: 放生宝可梦

```
Given: 玩家在仓库中选择了一只宝可梦
When: 玩家选择 "放生"
Then: 系统应该提示确认
And: 确认后，该宝可梦被永久删除
And: 显示 "你放生了 XXX"
```

#### Scenario: 放生宝可梦获得感激度

```
Given: 玩家放生了一只宝可梦
When: 放生完成
Then: 可以显示宝可梦的感激信息（可选）
```

### Requirement: 宝可梦查询和统计

玩家应该能够快速查询和统计宝可梦信息。

#### Scenario: 按名字或属性查询

```
Given: 玩家的仓库中有多只宝可梦
When: 玩家使用搜索功能输入 "皮卡丘"
Then: 应该显示仓库中所有皮卡丘
And: 显示它们所在的箱子位置
```

#### Scenario: 仓库统计信息

```
Given: 玩家的仓库中有宝可梦
When: 玩家查看统计信息
Then: 应该显示:
  - 总宝可梦数
  - 已捕捉的不同物种数
  - 平均等级
  - 稀有宝可梦列表
```

### Requirement: 队伍与仓库交换

玩家应该能够快速在队伍和仓库之间移动宝可梦。

#### Scenario: 从仓库取出宝可梦到队伍

```
Given: 玩家的队伍未满 (< 6 只)
And: 仓库中有宝可梦
When: 玩家在仓库中选择一只宝可梦并选择 "加入队伍"
Then: 该宝可梦应该被添加到队伍
And: 从仓库中移除
```

#### Scenario: 从队伍放入宝可梦到仓库

```
Given: 玩家的队伍中有宝可梦
When: 玩家选择 "放入仓库"
Then: 该宝可梦应该被移入仓库
And: 从队伍中移除
And: 如果仓库也满了，显示错误提示
```

### Requirement: 箱子管理

玩家应该能够组织和管理存储箱。

#### Scenario: 重命名存储箱

```
Given: 玩家正在浏览某个箱子
When: 玩家选择 "重命名"
Then: 应该弹出输入框让玩家输入新名字
And: 输入 "火系宝可梦" 后箱子被重命名
```

#### Scenario: 查看箱子内容

```
Given: 玩家选择第 1 个箱子
When: 进入箱子详情
Then: 应该显示该箱子中的所有宝可梦
And: 每个宝可梦显示 (名字, Lv., ID)
```

---

## MODIFIED Requirements

### Requirement: Player 结构扩展

Player 结构应该包含存储系统。

#### Scenario: Player 含有 StorageSystem

```
Given: 系统存在 Player 结构体
When: 创建玩家
Then: Player 应该包含字段:
  - storage_system: StorageSystem
  - 在存档中保存和加载
```

---

## Related Capabilities

- **team-detail-system**: 队伍详细信息 - 联动从仓库取出宝可梦到队伍
- **battle-system**: 战斗系统 - 战斗中可以切换队伍中的宝可梦
