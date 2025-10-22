# 地图与NPC对战系统

## 状态
- 创建时间: 2025-10-21
- 状态: 提案阶段

## 目标

实现一个**完整的地图和 NPC 对战系统**，包含以下核心功能：

1. **地图系统** - 多个探索区域，每个区域有独特的对手
2. **NPC 对战** - 预设的 NPC 对手，玩家可提前了解对方宝可梦
3. **宝可梦生成系统** - 系统化的宝可梦生成，防止重复，支持种族值和天赋
4. **难度梯度** - 从新手到困难的多个难度等级

## 设计

### 系统架构

```
游戏系统
├── 地图模块 (map)
│   ├── Region (区域)
│   │   └── Location (地点)
│   └── NavigationMenu (导航菜单)
├── NPC 系统 (npc)
│   ├── NPC (对手信息)
│   ├── NPCTeam (NPC队伍)
│   └── BattlePreview (对战预览)
├── 宝可梦生成系统 (pokemon_generator)
│   ├── PokemonSpecies (物种信息)
│   ├── IndividualValue (个体值)
│   ├── Talent (天赋系统)
│   └── TraitsGenerator (特征生成)
└── 对战系统增强
    ├── NPC对战处理
    └── 奖励系统
```

### 核心数据结构

#### 1. 地图系统

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub npc_trainers: Vec<u32>,  // NPC ID 列表
    pub wild_pokemon_ids: Vec<u32>,  // 可遭遇的宝可梦
    pub reward_money: u32,  // 该地点对战奖励
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub id: u32,
    pub name: String,
    pub locations: Vec<Location>,
    pub level_range: (u32, u32),  // 推荐等级范围
}

pub struct GameMap {
    pub regions: Vec<Region>,
    pub current_location: Option<u32>,
    pub visited_locations: HashSet<u32>,
}
```

#### 2. NPC 系统

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPCTrainer {
    pub id: u32,
    pub name: String,
    pub title: String,  // 如 "馆主", "天才少女" 等
    pub team: Vec<u32>,  // 宝可梦 ID
    pub reward_money: u32,
    pub reward_items: Vec<String>,
    pub defeated: bool,  // 是否已被击败
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPCTeam {
    pub npc_id: u32,
    pub pokemon_team: Vec<PokemonInstance>,  // 实例宝可梦
    pub difficulty: Difficulty,  // 难度等级
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Difficulty {
    Easy,      // 新手 (等级-5)
    Normal,    // 普通 (等级 ±0)
    Hard,      // 困难 (等级+5)
    Expert,    // 专家 (等级+10)
}
```

#### 3. 宝可梦生成系统

```rust
// 个体值 (IV: Individual Values)
// 范围 0-31，影响宝可梦属性的 12.5%
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndividualValues {
    pub hp: u32,           // 0-31
    pub attack: u32,       // 0-31
    pub defense: u32,      // 0-31
    pub sp_attack: u32,    // 0-31
    pub sp_defense: u32,   // 0-31
    pub speed: u32,        // 0-31
}

// 天赋 (Talent/Ability)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Talent {
    // 火属性
    Blaze,              // 火焰系 - 低血时火攻+50%
    FlashFire,          // 抢火 - 吸收火属性招式

    // 水属性
    Torrent,            // 激流 - 低血时水攻+50%
    WaterAbsorb,        // 亲水 - 吸收水属性招式

    // 草属性
    Overgrow,           // 茂盛 - 低血时草攻+50%

    // 电属性
    StaticElectricity,  // 静电 - 接触时可麻痹对手

    // 一般
    Synchronize,        // 同步 - 将异常状态反射给对手
    Intimidate,         // 威吓 - 出场时降低对手攻击
    DragonDance,        // 龙舞 - 回合结束后提升速度
}

// 性格 (Nature) - 影响属性增长
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Nature {
    Hardy,        // 无影响
    Lonely,       // 攻+, 防-
    Brave,        // 攻+, 速-
    Adamant,      // 攻+, 特攻-
    Naughty,      // 攻+, 特防-

    Bold,         // 防+, 攻-
    Docile,       // 无影响
    Relaxed,      // 防+, 速-
    Impish,       // 防+, 特攻-
    Lax,          // 防+, 特防-

    Timid,        // 速+, 攻-
    Hasty,        // 速+, 防-
    Serious,      // 无影响
    Jolly,        // 速+, 特攻-
    Naive,        // 速+, 特防-

    Modest,       // 特攻+, 攻-
    Mild,         // 特攻+, 防-
    Quiet,        // 特攻+, 速-
    Bashful,      // 无影响
    Rash,         // 特攻+, 特防-

    Calm,         // 特防+, 攻-
    Gentle,       // 特防+, 防-
    Sassy,        // 特防+, 速-
    Careful,      // 特防+, 特攻-
    Quirky,       // 无影响
}

// 宝可梦实例 (包含个体属性)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonInstance {
    pub pokemon: Pokemon,           // 基础宝可梦
    pub level: u32,
    pub iv: IndividualValues,       // 个体值
    pub talent: Talent,             // 天赋
    pub nature: Nature,             // 性格
    pub experience: u32,
    pub unique_id: u64,             // 唯一ID，防止重复
}

impl PokemonInstance {
    // 根据 IV 和 Level 计算最终属性
    pub fn calculate_stat(&self, stat_type: StatType) -> u32 {
        // 公式: ((2 * base_stat + IV + (EV / 4)) * level / 100) + level + 5
        // 当前版本简化为: ((2 * base_stat + IV) * level / 100) + level + 5
    }
}
```

### 宝可梦生成系统设计

#### 生成流程

```
创建 NPC 队伍
  ↓
根据难度选择宝可梦种族
  ↓
生成个体值 (IV)
  ↓
分配天赋 (Talent)
  ↓
分配性格 (Nature)
  ↓
生成招式组合
  ↓
生成唯一 ID
  ↓
完成宝可梦实例
```

#### 唯一性保证

```rust
pub struct PokemonRegistry {
    existing_pokemon: HashSet<u64>,  // 已生成的 ID
}

impl PokemonRegistry {
    pub fn generate_unique_id(&mut self, pokemon_data: &PokemonInstance) -> u64 {
        loop {
            let id = Self::hash_pokemon(pokemon_data);
            if !self.existing_pokemon.contains(&id) {
                self.existing_pokemon.insert(id);
                return id;
            }
            // 如果冲突，修改部分数据重试
        }
    }
}
```

### 地点示例

```rust
// 常青小镇 (Pallet Town)
pub fn create_pallet_town() -> Location {
    Location {
        id: 1,
        name: "常青小镇".to_string(),
        description: "你出生的小镇，有新手训练师。".to_string(),
        npc_trainers: vec![1, 2],  // 青绿、小蓝
        wild_pokemon_ids: vec![10, 25, 54],  // 绿毛虫、皮卡丘、嘟嘟
        reward_money: 50,
    }
}

// 金黄市 (Cerulean City)
pub fn create_cerulean_city() -> Location {
    Location {
        id: 2,
        name: "金黄市".to_string(),
        description: "有水系馆主。".to_string(),
        npc_trainers: vec![3],  // 水系馆主
        wild_pokemon_ids: vec![7, 25, 54, 129],  // 杰尼龟等
        reward_money: 200,
    }
}
```

### NPC 示例

```rust
// 青绿 (Gary)
pub fn create_npc_gary() -> NPCTrainer {
    NPCTrainer {
        id: 1,
        name: "青绿".to_string(),
        title: "劲敌".to_string(),
        team: vec![1, 4, 7],  // 妙蛙种子、小火龙、杰尼龟
        reward_money: 150,
        reward_items: vec!["经验药".to_string()],
        defeated: false,
    }
}

// 馆主 - 水系
pub fn create_gym_leader_misty() -> NPCTrainer {
    NPCTrainer {
        id: 3,
        name: "美娜".to_string(),
        title: "馆主 (水系)".to_string(),
        team: vec![120, 121, 122],  // 海星星等
        reward_money: 500,
        reward_items: vec!["海之徽章".to_string()],
        defeated: false,
    }
}
```

### 对战预览系统

```rust
pub struct BattlePreview {
    pub npc_name: String,
    pub npc_title: String,
    pub team_pokemon: Vec<PokemonPreview>,
    pub reward_money: u32,
    pub reward_items: Vec<String>,
}

pub struct PokemonPreview {
    pub name: String,
    pub level: u32,
    pub pokemon_type: (PokemonType, Option<PokemonType>),
    pub moves: Vec<String>,  // 招式列表
}
```

## 实现步骤

### 第一阶段：地图基础设施
- [ ] 创建地图模块 (map/)
- [ ] 实现 Location 和 Region 结构体
- [ ] 创建地点数据库
- [ ] 实现地图导航菜单
- [ ] 实现当前位置追踪

### 第二阶段：NPC 系统
- [ ] 创建 NPC 模块 (npc/)
- [ ] 实现 NPCTrainer 结构体
- [ ] 创建 NPC 数据库 (至少 10 个 NPC)
- [ ] 实现 NPC 队伍加载
- [ ] 实现对战前预览功能

### 第三阶段：宝可梦生成系统（核心）
- [ ] 创建 pokemon_generator 模块
- [ ] 实现个体值 (IV) 生成系统
- [ ] 实现天赋 (Talent) 系统
- [ ] 实现性格 (Nature) 系统
- [ ] 实现唯一性检查系统
- [ ] 实现 PokemonInstance 结构体
- [ ] 添加属性计算公式

### 第四阶段：宝可梦数据库扩展
- [ ] 将 pokemon_data.rs 转为种族基础数据
- [ ] 添加每个宝可梦的多个天赋选项
- [ ] 添加招式学习池
- [ ] 为每个宝可梦定义难度等级

### 第五阶段：对战系统集成
- [ ] 修改战斗系统以支持 NPC 对战
- [ ] 实现对战前预览菜单
- [ ] 实现奖励系统 (金币、经验)
- [ ] 记录 NPC 击败状态
- [ ] 实现无法再战已击败 NPC 的逻辑

### 第六阶段：游戏循环修改
- [ ] 修改主菜单添加"地图"选项
- [ ] 实现地图导航系统
- [ ] 实现地点选择菜单
- [ ] 集成所有系统到游戏循环

### 第七阶段：UI/UX 完善
- [ ] 地图显示界面
- [ ] NPC 信息显示
- [ ] 对战预览界面
- [ ] 属性详细显示

### 第八阶段：测试
- [ ] 宝可梦生成唯一性测试
- [ ] 属性计算正确性测试
- [ ] 天赋系统测试
- [ ] 完整的对战流程测试

## 关键特性详解

### 1. 个体值系统 (IV System)

**作用**: 使同一物种的宝可梦具有不同的属性

```
最终属性 = 基础属性 + (个体值 / 32) * 基础属性 * 12.5%

例如: 皮卡丘 (攻击基础值 55)
- IV = 0:  攻击 = 55
- IV = 15: 攻击 = 55 + 8.6 ≈ 64
- IV = 31: 攻击 = 55 + 17.2 ≈ 72
```

**收集 IV**:
- 敌方宝可梦的 IV 会随机生成
- 玩家可以通过对战发现高 IV 宝可梦
- 收集高 IV 宝可梦来增强队伍

### 2. 天赋系统 (Talent/Ability)

**作用**: 为宝可梦提供独特能力

```
示例天赋效果:
- 火焰系: 血量低于 1/3 时，火属性招式威力×1.5
- 抢火: 被火属性招式命中时吸收伤害并提升火属性招式威力
- 威吓: 进场时自动降低对手宝可梦的攻击
```

**策略性**:
- 某些宝可梦有多个天赋选择
- 不同天赋适应不同战术
- 天赋选择影响宝可梦的角色定位

### 3. 性格系统 (Nature)

**作用**: 影响宝可梦属性增长

```
25 种性格矩阵:
- 5 行：攻击、防御、速度、特攻、特防
- 5 列：对应的加成属性
- 对角线：无影响

示例:
- Adamant (倔强): 攻击 +10%, 特攻 -10%
- Timid (胆小): 速度 +10%, 攻击 -10%
```

**收集性格**:
- 同种宝可梦可能有不同性格
- 某些性格更适合某种玩法
- 收集"最优性格"的宝可梦成为目标

### 4. 唯一性防止系统

**防止重复的方法**:

```rust
struct PokemonInstance {
    pokemon_id: u32,
    level: u32,
    iv: [u32; 6],
    talent: Talent,
    nature: Nature,
}

// 生成唯一哈希
fn hash_pokemon(instance: &PokemonInstance) -> u64 {
    let mut hasher = DefaultHasher::new();
    instance.pokemon_id.hash(&mut hasher);
    instance.level.hash(&mut hasher);
    instance.iv.iter().for_each(|iv| iv.hash(&mut hasher));
    instance.talent.hash(&mut hasher);
    instance.nature.hash(&mut hasher);
    hasher.finish()
}
```

**冲突处理**:
- 记录已生成的 ID
- 冲突时调整 IV 或天赋重新生成
- 确保每只宝可梦是独特的

## CLI 菜单示例

### 地图菜单
```
╔═════════════════════════════════════╗
║           宝可梦地图                ║
╠═════════════════════════════════════╣
║ 【关都地区】                        ║
║ 1. 常青小镇 (等级: 1-5)            ║
║ 2. 常青森林 (等级: 3-7)            ║
║ 3. 金黄市 (等级: 15-20) [馆主]    ║
║                                     ║
║ 已到访: 3/8  已击败馆主: 1/3      ║
║ 0. 返回菜单                        ║
╚═════════════════════════════════════╝
请选择:
```

### NPC 对战预览
```
╔═════════════════════════════════════╗
║       对战预览 - 馆主美娜           ║
║         (水系馆主)                  ║
╠═════════════════════════════════════╣
║ 奖励: 500 金币 + 海之徽章           ║
║                                     ║
║ 对手队伍:                           ║
║ 1. 星星 (等级 18)                  ║
║    类型: 水                        ║
║    招式: 水枪、急冻光线、保护       ║
║                                     ║
║ 2. 海星星 (等级 20)                ║
║    类型: 水、超能                  ║
║    招式: 精神强念、泡沫束、痛苦    ║
║                                     ║
║ 难度: 困难 (推荐等级 20-25)        ║
║                                     ║
║ 1. 开始对战                         ║
║ 2. 返回                             ║
╚═════════════════════════════════════╝
```

### 宝可梦详细信息
```
╔═════════════════════════════════════╗
║      宝可梦详细信息 - 皮卡丘        ║
╠═════════════════════════════════════╣
║ 等级: 25                            ║
║ 经验: 1500/2500                    ║
║ 性格: 勇敢 (倔强) [攻+10% 特攻-10%]║
║ 天赋: 静电                          ║
║                                     ║
║ 【属性值】                          ║
║ HP:       45  (基础 35, IV 15)    ║
║ 攻击:     68  (基础 55, IV 20)    ║
║ 防御:     52  (基础 40, IV 18)    ║
║ 特攻:     65  (基础 50, IV 25)    ║
║ 特防:     62  (基础 50, IV 22)    ║
║ 速度:     87  (基础 90, IV 31)    ║
║                                     ║
║ 【招式】                            ║
║ 1. 撞击 (威力 40, 精度 100%)       ║
║ 2. 电击 (威力 40, 精度 100%)       ║
║ 3. 10万伏特 (威力 90, 精度 100%)  ║
║ 4. 铁头 (威力 80, 精度 100%)       ║
║                                     ║
║ 唯一 ID: 0x7F2E1A9B4C8D6E5F       ║
╚═════════════════════════════════════╝
```

## 向后兼容性

- 现有的野生宝可梦遭遇不受影响
- 现有的玩家队伍可迁移到新系统
- 现有的存档可以保留但需要更新

## 游戏流程整合

```
主菜单
├─ 探索 (原有系统)
│  └─ 野生宝可梦遭遇
├─ 地图 (新系统) ← 新增
│  ├─ 选择地点
│  ├─ 查看 NPC 列表
│  ├─ 预览对战
│  └─ 开始 NPC 对战
├─ 队伍
├─ 背包
└─ 存档/加载
```

## 优势

### 游戏深度
- ✅ 系统化的对手难度梯度
- ✅ 更多的内容和可玩性
- ✅ 收集高 IV/好性格宝可梦的目标
- ✅ 多周目挑战

### 战术性
- ✅ 了解对手队伍，提前准备
- ✅ 天赋和性格增加策略变化
- ✅ 队伍搭配变得更重要

### 内容量
- ✅ 从"简单探索"升级到"有剧情的冒险"
- ✅ 馆主对战增加目标感
- ✅ 收集系统增加重玩价值

## 测试计划

### 单元测试
- [ ] IV 计算正确性
- [ ] 性格属性加成正确
- [ ] 唯一 ID 生成无重复
- [ ] 天赋效果正确应用

### 集成测试
- [ ] 完整的地图导航
- [ ] NPC 对战全流程
- [ ] 奖励正确发放
- [ ] 宝可梦生成系统

### 平衡测试
- [ ] 对战难度梯度合理
- [ ] 奖励金币数量平衡
- [ ] IV 分布合理
- [ ] 天赋能力平衡

## 检查清单

- [ ] 地图模块完成
- [ ] NPC 系统完成
- [ ] 宝可梦生成系统完成
- [ ] 对战系统集成
- [ ] 所有测试通过
- [ ] UI 完整
- [ ] 文档更新
- [ ] 游戏平衡测试

## 交付物

- 完整的地图和 NPC 系统
- 宝可梦生成系统 (IV、天赋、性格)
- 对战预览系统
- 10+ 个精心设计的 NPC
- 至少 8 个地点
- 完整的测试套件
- 更新的游戏文档

## 优先级建议

**高优先级** (核心功能):
1. 宝可梦生成系统 (基础)
2. NPC 系统
3. 地图系统
4. 对战集成

**中优先级** (增强):
1. 对战预览系统
2. 天赋系统
3. 性格系统

**低优先级** (美化):
1. UI 优化
2. 更多地点和 NPC
3. 特殊事件

## 相关功能 (未来扩展)

- [ ] 进化系统与地图联动
- [ ] 任务系统
- [ ] 道具商店与经济系统
- [ ] 宝可梦训练所
- [ ] 联盟挑战赛
- [ ] 多人对战
- [ ] 宝可梦交易
