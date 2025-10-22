use super::{NPCTrainer, Difficulty};

/// 创建所有 NPC 训练师
pub fn create_all_npcs() -> Vec<NPCTrainer> {
    vec![
        // 早期对手
        NPCTrainer::new(
            1,
            "青绿".to_string(),
            "劲敌".to_string(),
            vec![25],  // 皮卡丘
            100,
            Difficulty::Easy,
        ),

        // 馆主们
        NPCTrainer::new(
            2,
            "小刚".to_string(),
            "馆主".to_string(),
            vec![74, 75],  // 小岩石、小岩石
            500,
            Difficulty::Normal,
        ),

        NPCTrainer::new(
            3,
            "美娜".to_string(),
            "馆主".to_string(),
            vec![54, 60, 90],  // 可达鸭、蚊香蛙、壳兜
            500,
            Difficulty::Normal,
        ),

        NPCTrainer::new(
            4,
            "蕾欧娜".to_string(),
            "馆主".to_string(),
            vec![100, 101, 102],  // 鬼斯、鬼斯通、具甲怪
            750,
            Difficulty::Hard,
        ),

        NPCTrainer::new(
            5,
            "娜姿".to_string(),
            "馆主".to_string(),
            vec![92, 93, 94],  // 鬼斯、鬼斯通、具甲怪
            1000,
            Difficulty::Hard,
        ),

        NPCTrainer::new(
            6,
            "王牌".to_string(),
            "馆主".to_string(),
            vec![109, 110, 111],  // 双弹瓦斯、双弹瓦斯、铁甲贝
            750,
            Difficulty::Hard,
        ),

        // 中期挑战者
        NPCTrainer::new(
            7,
            "坂木".to_string(),
            "火箭队头领".to_string(),
            vec![23, 24, 25],  // 阿柏、阿柏怪、皮卡丘
            1500,
            Difficulty::Expert,
        ),

        NPCTrainer::new(
            8,
            "青绿".to_string(),
            "宿敌".to_string(),
            vec![3, 6, 9],  // 妙蛙草、喷火龙、水箭龟
            2000,
            Difficulty::Expert,
        ),

        // 最终BOSS
        NPCTrainer::new(
            9,
            "超梦".to_string(),
            "冠军".to_string(),
            vec![149, 150, 151],  // 快龙、超梦、梦幻
            3000,
            Difficulty::Expert,
        ),

        // 额外挑战者
        NPCTrainer::new(
            10,
            "莉莎".to_string(),
            "天才少女".to_string(),
            vec![27, 28, 31],  // 小拳石、隆隆石、尼多王
            800,
            Difficulty::Hard,
        ),

        NPCTrainer::new(
            11,
            "克劳德".to_string(),
            "武术家".to_string(),
            vec![104, 105, 106],  // 卡拉卡拉、骨骨龙、沙瓦
            600,
            Difficulty::Normal,
        ),

        NPCTrainer::new(
            12,
            "翠".to_string(),
            "草系使用者".to_string(),
            vec![69, 70, 71],  // 蛋蛋、蛋蛋、椰蛋树
            900,
            Difficulty::Hard,
        ),
    ]
}

/// 通过 ID 获取 NPC
pub fn get_npc_by_id(npc_id: u32) -> Option<NPCTrainer> {
    create_all_npcs()
        .into_iter()
        .find(|npc| npc.id == npc_id)
}

/// 获取所有 NPC
pub fn get_all_npcs() -> Vec<NPCTrainer> {
    create_all_npcs()
}

/// 通过位置 ID 获取该位置的所有 NPC
pub fn get_npcs_by_location(location_npcs: &[u32]) -> Vec<NPCTrainer> {
    let all_npcs = create_all_npcs();
    location_npcs
        .iter()
        .filter_map(|npc_id| all_npcs.iter().find(|npc| &npc.id == npc_id).cloned())
        .collect()
}
