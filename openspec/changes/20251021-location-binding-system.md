# 位置强绑定与地图解锁系统

## 状态
- 创建时间: 2025-10-21
- 状态: 提案阶段

## 目标

实现一个**位置强绑定**的游戏系统，将玩家锚定在特定地点，通过任务完成和条件满足来解锁新地图，并引入**环境属性加成**系统。核心改进包括：

1. **位置强绑定** - 玩家总是在一个具体地点，而非自由漂浮
2. **地图解锁系统** - 通过完成条件解锁新地点和地图
3. **环境加成系统** - 每个地点基于环境对宝可梦属性有加成
4. **随机野生宝可梦遭遇** - 地点特定的宝可梦池，带预览

## 核心设计

### 系统架构

```
游戏系统
├── 位置系统 (location)
│   ├── CurrentLocation (当前位置结构)
│   ├── LocationEnvironment (环境加成)
│   ├── LocationRequirement (解锁条件)
│   └── LocationUnlockState (解锁状态跟踪)
│
├── 野生宝可梦系统 (wild_pokemon)
│   ├── WildPokemonPool (地点宝可梦池)
│   ├── EncounterChance (遭遇概率)
│   └── EncounterPreview (遭遇预览)
│
├── 条件系统 (requirements)
│   ├── LevelRequirement (等级要求)
│   ├── BadgeRequirement (徽章要求)
│   ├── PokemonRequirement (宝可梦种类)
│   └── TaskRequirement (任务完成)
│
└── 环境加成系统 (environment_bonus)
    ├── EnvironmentType (环境类型)
    ├── BonusMultiplier (加成倍数)
    └── StatBonus (属性加成)
```

### 数据结构

#### 1. 位置信息 (强绑定)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: u32,
    pub name: String,
    pub description: String,

    // 强绑定相关
    pub is_starting_location: bool,  // 是否是起始位置
    pub can_leave: bool,             // 是否可以离开
    pub connected_locations: Vec<u32>, // 可前往的地点

    // 环境相关
    pub environment: EnvironmentType,
    pub stat_bonus: EnvironmentBonus, // 环境加成

    // 宝可梦相关
    pub wild_pokemon_pool: Vec<WildPokemonSpawn>,
    pub encounter_rate: f32, // 遭遇率 (0.0 - 1.0)

    // 解锁相关
    pub unlock_requirement: LocationRequirement,
    pub npc_trainers: Vec<u32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EnvironmentType {
    Grassland,   // 草地 - 速度+10%
    Forest,      // 森林 - 防守+10%
    Cave,        // 洞穴 - 特攻+10%
    Water,       // 水域 - 特防+10%
    Mountain,    // 山地 - 攻击+10%
    City,        // 城市 - 全属性+5%
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EnvironmentBonus {
    pub attack_bonus: f32,      // 默认 1.0
    pub defense_bonus: f32,
    pub sp_attack_bonus: f32,
    pub sp_defense_bonus: f32,
    pub speed_bonus: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationRequirement {
    pub required_level: u32,
    pub required_badges: Vec<u32>,
    pub required_pokemon_count: Option<u32>,
    pub completed_tasks: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WildPokemonSpawn {
    pub pokemon_id: u32,
    pub spawn_rate: f32,     // 相对权重
    pub level_min: u32,
    pub level_max: u32,
}
```

#### 2. 玩家位置状态

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerLocationState {
    pub current_location_id: u32,
    pub unlocked_locations: HashSet<u32>,
    pub visited_locations: HashSet<u32>,
    pub location_completion_state: HashMap<u32, LocationState>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LocationState {
    Locked,
    Unlocked,
    Visited,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WildPokemonPreview {
    pub pokemon_name: String,
    pub pokemon_id: u32,
    pub level: u32,
    pub pokemon_type: String,
}
```

#### 3. 环境加成计算

```rust
impl EnvironmentBonus {
    pub fn apply_to_stat(&self, stat_name: &str, base_value: u32) -> u32 {
        let multiplier = match stat_name {
            "attack" => self.attack_bonus,
            "defense" => self.defense_bonus,
            "sp_attack" => self.sp_attack_bonus,
            "sp_defense" => self.sp_defense_bonus,
            "speed" => self.speed_bonus,
            _ => 1.0,
        };
        (base_value as f32 * multiplier).round() as u32
    }
}
```

## 游戏流程

### 起始流程
```
游戏开始
    ↓
创建玩家 (位置: 常青小镇)
    ↓
显示当前位置信息
    ↓
显示可用操作:
    1. 探索 (随机遭遇)
    2. 移动到其他地点
    3. 查看队伍
    4. 其他操作
```

### 地点移动流程
```
选择移动
    ↓
显示可达地点列表:
    - 已解锁地点 (可移动)
    - 已锁定地点 (显示解锁条件)
    ↓
选择目标地点
    ↓
检查是否满足条件:
    ✓ 满足 → 移动并显示地点信息
    ✗ 不满足 → 显示缺失条件
```

### 野生宝可梦遭遇流程
```
选择探索
    ↓
随机生成遭遇 (基于地点遭遇率)
    ↓
遭遇宝可梦:
    1. 显示宝可梦名字、等级、类型
    2. 应用环境加成显示预览
    ↓
玩家选择:
    - 捕捉
    - 战斗
    - 逃跑
```

## 关键特性

### 1. 位置强绑定
- 玩家总是位于某个地点
- 不能在虚空中移动
- 每个地点有明确的出口列表
- 移动需要检查连接性

### 2. 地图渐进解锁
| 地点 | 起始状态 | 解锁条件 |
|------|---------|---------|
| 常青小镇 | 已解锁 | - |
| 常青森林 | 锁定 | 队伍等级 ≥ 3 |
| 华石镇 | 锁定 | 队伍等级 ≥ 5 |
| 月见山 | 锁定 | 队伍等级 ≥ 7 + 击败小刚 |
| 华蓝市 | 锁定 | 队伍等级 ≥ 15 + 获得2个徽章 |
| ... | 锁定 | 逐步递进 |

### 3. 环境加成系统
```
草地 (Grassland):
  - 速度 +10%
  - 示例: 速度100的宝可梦 → 110

森林 (Forest):
  - 防守 +10%

洞穴 (Cave):
  - 特攻 +10%

水域 (Water):
  - 特防 +10%

山地 (Mountain):
  - 攻击 +10%

城市 (City):
  - 所有属性 +5%
```

### 4. 野生宝可梦遭遇
```
遭遇预览显示:
  ┌─────────────────────┐
  │  野生宝可梦出现!    │
  ├─────────────────────┤
  │ 宝可梦: 皮卡丘      │
  │ 等级: 5             │
  │ 类型: 电系          │
  │ 预计属性加成:       │
  │   速度: 110 (+10%)  │
  └─────────────────────┘
```

## 实现步骤

### Phase 1: 位置系统基础 (1-2天)
- [ ] 创建位置数据结构
- [ ] 实现位置状态管理
- [ ] 创建玩家位置状态追踪
- [ ] 测试位置导航逻辑

### Phase 2: 环境加成系统 (1天)
- [ ] 实现环境类型枚举
- [ ] 创建环境加成计算
- [ ] 集成到宝可梦属性系统
- [ ] 测试加成计算准确性

### Phase 3: 地图解锁系统 (1-2天)
- [ ] 创建解锁条件定义
- [ ] 实现条件检查逻辑
- [ ] 创建地点解锁追踪
- [ ] 添加地点解锁通知

### Phase 4: 野生宝可梦系统 (1-2天)
- [ ] 创建野生宝可梦池系统
- [ ] 实现遭遇生成逻辑
- [ ] 创建遭遇预览显示
- [ ] 集成到游戏循环

### Phase 5: UI/UX 改进 (1天)
- [ ] 创建位置菜单UI
- [ ] 实现位置信息显示
- [ ] 创建解锁条件显示
- [ ] 实现遭遇预览菜单

### Phase 6: 测试与集成 (1天)
- [ ] 编写位置系统测试
- [ ] 编写环境加成测试
- [ ] 编写解锁系统测试
- [ ] 集成所有系统

## 文件结构

```
src/
├── game/
│   ├── location.rs (新) - 位置系统
│   ├── environment.rs (新) - 环境加成
│   ├── wild_pokemon.rs (新) - 野生宝可梦
│   ├── requirements.rs (新) - 解锁条件
│   └── mod.rs (修改)
├── cli/
│   ├── location_menu.rs (新) - 位置菜单
│   └── mod.rs (修改)
├── data/
│   ├── locations_data.rs (新) - 位置数据
│   └── mod.rs (修改)
└── main.rs (修改) - 游戏循环更新
```

## 关键决策

### 1. 为什么强绑定位置?
- **优势**: 清晰的游戏进度、易于管理状态
- **劣势**: 需要更多的移动菜单
- **选择理由**: 提供更好的叙事性和进度感

### 2. 为什么是渐进解锁?
- **优势**: 鼓励玩家进步、避免过度挑战
- **劣势**: 限制自由度
- **选择理由**: 平衡难度和自由度

### 3. 为什么环境加成只有 ±10%?
- **优势**: 影响不过分、易于平衡
- **劣势**: 加成可能不明显
- **选择理由**: 战术相关但不主导胜负

## 向后兼容性

- ✅ 现有存档需要迁移 (添加默认位置状态)
- ✅ 现有地点信息需要扩展
- ✅ 现有宝可梦不受影响

## 验收标准

- [ ] 玩家总是位于一个有效地点
- [ ] 地点连接性正确
- [ ] 解锁条件正常运作
- [ ] 环境加成正确计算
- [ ] 野生遭遇展示加成后的属性
- [ ] 所有测试通过 (新增 20+ 个测试)
- [ ] 无内存泄漏

## 估算

- **开发时间**: 5-7 天
- **测试时间**: 1-2 天
- **代码行数**: ~800-1000 行
- **新测试数**: 20+

## 风险与缓解

| 风险 | 影响 | 缓解 |
|------|------|------|
| 位置系统过于复杂 | 代码维护难 | 模块化设计 |
| 解锁条件难以平衡 | 游戏体验差 | 数据驱动设计 |
| 环境加成计算错误 | 游戏不平衡 | 充分的单元测试 |
| 存档兼容性问题 | 玩家数据丢失 | 迁移脚本 |

---

**下一步**: 提案审核后，请使用 `/openspec:apply 20251021-location-binding-system` 开始实施。
