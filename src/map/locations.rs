use super::{Location, Region};

/// 创建游戏中的所有位置
pub fn create_locations() -> Vec<Region> {
    vec![
        create_kanto_region(),
    ]
}

/// 创建关都地区
fn create_kanto_region() -> Region {
    Region {
        id: 1,
        name: "关都地区".to_string(),
        locations: vec![
            // 新手区
            Location {
                id: 101,
                name: "常青小镇".to_string(),
                description: "旅程的起点，宁静祥和的小镇".to_string(),
                npc_trainers: vec![1],  // 青绿
                wild_pokemon_ids: vec![],
                reward_money: 0,
                level_range: (1, 5),
            },

            // 新手森林
            Location {
                id: 102,
                name: "常青森林".to_string(),
                description: "一片茂密的森林，适合新手探险".to_string(),
                npc_trainers: vec![],
                wild_pokemon_ids: vec![1, 4, 7, 16, 19],  // 妙蛙种子、小火龙、杰尼龟、小鸟、小虫
                reward_money: 0,
                level_range: (2, 6),
            },

            // 华石镇
            Location {
                id: 103,
                name: "华石镇".to_string(),
                description: "一个小村庄，有许多稀有宝可梦".to_string(),
                npc_trainers: vec![],
                wild_pokemon_ids: vec![21, 23, 25],  // 烈雀、阿柏、皮卡丘
                reward_money: 0,
                level_range: (5, 10),
            },

            // 月见山
            Location {
                id: 104,
                name: "月见山".to_string(),
                description: "一座高耸的山脉，环境恶劣但资源丰富".to_string(),
                npc_trainers: vec![2],  // 馆主小刚
                wild_pokemon_ids: vec![27, 29, 41],  // 小拳石、地鼠、超音蝠
                reward_money: 500,
                level_range: (7, 12),
            },

            // 华蓝市
            Location {
                id: 105,
                name: "华蓝市".to_string(),
                description: "靠近水边的美丽城市，有许多水系宝可梦".to_string(),
                npc_trainers: vec![3],  // 馆主美娜
                wild_pokemon_ids: vec![54, 60, 90],  // 可达鸭、蚊香蛙、壳兜
                reward_money: 500,
                level_range: (15, 20),
            },

            // 红莲镇
            Location {
                id: 106,
                name: "红莲镇".to_string(),
                description: "建立在火山上的小镇，到处都是火焰".to_string(),
                npc_trainers: vec![],
                wild_pokemon_ids: vec![74, 104, 133],  // 小岩石、卡拉卡拉、伊布
                reward_money: 0,
                level_range: (18, 24),
            },

            // 金黄市
            Location {
                id: 107,
                name: "金黄市".to_string(),
                description: "繁华的大城市，馆主是职业电竞选手".to_string(),
                npc_trainers: vec![4],  // 馆主蕾欧娜
                wild_pokemon_ids: vec![100, 109, 116],  // 鬼斯、六尾、蛋蛋
                reward_money: 750,
                level_range: (20, 25),
            },

            // 浅红市
            Location {
                id: 108,
                name: "浅红市".to_string(),
                description: "古老而神秘的城市，传说这里有超古代遗迹".to_string(),
                npc_trainers: vec![5],  // 馆主娜姿
                wild_pokemon_ids: vec![138, 140, 92],  // 贝壳兽、化石翼龙、鬼斯通
                reward_money: 1000,
                level_range: (25, 35),
            },

            // 双子镇
            Location {
                id: 109,
                name: "双子镇".to_string(),
                description: "繁忙的港口城市，有许多商人".to_string(),
                npc_trainers: vec![6],  // 馆主王牌
                wild_pokemon_ids: vec![152, 155, 158],  // 菊草叶、小火球、小锯鳄
                reward_money: 750,
                level_range: (22, 28),
            },

            // 常磐市
            Location {
                id: 110,
                name: "常磐市".to_string(),
                description: "最后的挑战地点，黑暗势力的总部".to_string(),
                npc_trainers: vec![7, 8],  // 坂木、劲敌
                wild_pokemon_ids: vec![147, 149],  // 迷你龙、快龙
                reward_money: 1500,
                level_range: (40, 50),
            },

            // 精灵联盟
            Location {
                id: 111,
                name: "宝可梦联盟".to_string(),
                description: "强者的殿堂，最终的冠军挑战".to_string(),
                npc_trainers: vec![9],  // 冠军
                wild_pokemon_ids: vec![],
                reward_money: 2000,
                level_range: (45, 55),
            },
        ],
        level_range: (1, 55),
    }
}

/// 获取指定ID的位置
pub fn get_location_by_id(location_id: u32) -> Option<Location> {
    let regions = create_locations();
    for region in regions {
        for location in region.locations {
            if location.id == location_id {
                return Some(location);
            }
        }
    }
    None
}

/// 获取指定区域的所有位置
pub fn get_locations_by_region(region_id: u32) -> Vec<Location> {
    let regions = create_locations();
    regions
        .iter()
        .find(|r| r.id == region_id)
        .map(|r| r.locations.clone())
        .unwrap_or_default()
}

/// 获取所有位置
pub fn get_all_locations() -> Vec<Location> {
    let regions = create_locations();
    regions
        .iter()
        .flat_map(|r| r.locations.clone())
        .collect()
}
