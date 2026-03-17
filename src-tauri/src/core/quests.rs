// Copyright 2025 zl. All rights reserved.

//! 任务系统 - 支持术士君临版本

use serde::{Deserialize, Serialize};

/// 难度等级
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Normal,
    Nightmare,
    Hell,
}

/// 幕
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Act {
    Act1,   // 罗格营地
    Act2,   // 鲁高因
    Act3,   // 库拉斯特
    Act4,   // 混沌避难所
    Act5,   // 哈洛加斯 (D2R扩展)
}

/// 任务状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestStatus {
    NotStarted,
    InProgress,
    Completed,
}

/// 任务ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestId {
    // 第一幕
    DenOfEvil,        // 邪恶洞穴
    BloodRaven,       // 血鸟
    Smith,            // 铁匠
    Countess,         // 女伯爵
    Cemetery,         // 墓地
    ToolsOfTheTrade,  // 届原之锤
    Imbue,            // 凯恩回归

    // 第二幕
    Radament,         // 拉卡尼休
    HoradricStaff,    // 赫拉迪克杖
    Summoner,         // 召唤者
    Duriel,           // 都瑞尔

    // 第三幕
    GoldenBird,       // 黄金鸟
    JadeFigurine,     // 翡翠小像
    Blade,            // 吉宾宝刀
    Khalim,           // 卡吉姆
    Travincal,        // 议会
    Mephisto,         // 墨菲斯托

    // 第四幕
    Izual,            // 依卒尔
    Diablo,           // 迪亚波罗

    // 第五幕 (D2R扩展)
    SiegeOnHarrogath, // 哈洛加斯之围
    RescueOnMountArreat, // 拯救
    PrisonOfIce,      // 冰牢
    Betrayal,         // 背叛
    RiteOfPassage,    // 通行仪式
    Nihlathak,        // 尼拉塞克
    Baal,             // 巴尔
}

impl QuestId {
    /// 获取任务中文名称
    pub fn zh_name(&self) -> &'static str {
        match self {
            // 第一幕
            QuestId::DenOfEvil => "邪恶洞穴",
            QuestId::BloodRaven => "血鸟",
            QuestId::Smith => "冰冷之原",
            QuestId::Countess => "遗忘高塔",
            QuestId::Cemetery => "墓地",
            QuestId::ToolsOfTheTrade => "届原之锤",
            QuestId::Imbue => "凯恩",

            // 第二幕
            QuestId::Radament => "拉卡尼休",
            QuestId::HoradricStaff => "赫拉迪克杖",
            QuestId::Summoner => "召唤者",
            QuestId::Duriel => "都瑞尔",

            // 第三幕
            QuestId::GoldenBird => "黄金鸟",
            QuestId::JadeFigurine => "翡翠小像",
            QuestId::Blade => "吉宾宝刀",
            QuestId::Khalim => "卡吉姆",
            QuestId::Travincal => "崔凡克",
            QuestId::Mephisto => "墨菲斯托",

            // 第四幕
            QuestId::Izual => "依卒尔",
            QuestId::Diablo => "迪亚波罗",

            // 第五幕
            QuestId::SiegeOnHarrogath => "哈洛加斯之围",
            QuestId::RescueOnMountArreat => "亚瑞特山的救援",
            QuestId::PrisonOfIce => "冰牢",
            QuestId::Betrayal => "背叛",
            QuestId::RiteOfPassage => "通行仪式",
            QuestId::Nihlathak => "尼拉塞克",
            QuestId::Baal => "巴尔",
        }
    }

    /// 获取任务所属幕
    pub fn act(&self) -> Act {
        match self {
            QuestId::DenOfEvil | QuestId::BloodRaven | QuestId::Smith | QuestId::Countess
            | QuestId::Cemetery | QuestId::ToolsOfTheTrade | QuestId::Imbue => Act::Act1,

            QuestId::Radament | QuestId::HoradricStaff | QuestId::Summoner | QuestId::Duriel => Act::Act2,

            QuestId::GoldenBird | QuestId::JadeFigurine | QuestId::Blade | QuestId::Khalim
            | QuestId::Travincal | QuestId::Mephisto => Act::Act3,

            QuestId::Izual | QuestId::Diablo => Act::Act4,

            QuestId::SiegeOnHarrogath | QuestId::RescueOnMountArreat | QuestId::PrisonOfIce
            | QuestId::Betrayal | QuestId::RiteOfPassage | QuestId::Nihlathak | QuestId::Baal => Act::Act5,
        }
    }
}

/// 单个任务数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quest {
    pub id: QuestId,
    pub status: QuestStatus,
    pub rewards_claimed: bool,
}

/// 任务数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestData {
    pub normal: Vec<Quest>,
    pub nightmare: Vec<Quest>,
    pub hell: Vec<Quest>,
}

impl QuestData {
    pub fn new() -> Self {
        Self {
            normal: Self::create_quest_list(Difficulty::Normal),
            nightmare: Self::create_quest_list(Difficulty::Nightmare),
            hell: Self::create_quest_list(Difficulty::Hell),
        }
    }

    fn create_quest_list(_difficulty: Difficulty) -> Vec<Quest> {
        vec![
            // 第一幕
            Quest { id: QuestId::DenOfEvil, status: QuestStatus::NotStarted, rewards_claimed: false },
            Quest { id: QuestId::BloodRaven, status: QuestStatus::NotStarted, rewards_claimed: false },
            Quest { id: QuestId::Smith, status: QuestStatus::NotStarted, rewards_claimed: false },
            Quest { id: QuestId::Countess, status: QuestStatus::NotStarted, rewards_claimed: false },
            Quest { id: QuestId::Cemetery, status: QuestStatus::NotStarted, rewards_claimed: false },
            Quest { id: QuestId::ToolsOfTheTrade, status: QuestStatus::NotStarted, rewards_claimed: false },
            Quest { id: QuestId::Imbue, status: QuestStatus::NotStarted, rewards_claimed: false },
            // 第二幕
            Quest { id: QuestId::Radament, status: QuestStatus::NotStarted, rewards_claimed: false },
            Quest { id: QuestId::HoradricStaff, status: QuestStatus::NotStarted, rewards_claimed: false },
            Quest { id: QuestId::Summoner, status: QuestStatus::NotStarted, rewards_claimed: false },
            Quest { id: QuestId::Duriel, status: QuestStatus::NotStarted, rewards_claimed: false },
            // 第三幕
            Quest { id: QuestId::GoldenBird, status: QuestStatus::NotStarted, rewards_claimed: false },
            Quest { id: QuestId::JadeFigurine, status: QuestStatus::NotStarted, rewards_claimed: false },
            Quest { id: QuestId::Blade, status: QuestStatus::NotStarted, rewards_claimed: false },
            Quest { id: QuestId::Khalim, status: QuestStatus::NotStarted, rewards_claimed: false },
            Quest { id: QuestId::Travincal, status: QuestStatus::NotStarted, rewards_claimed: false },
            Quest { id: QuestId::Mephisto, status: QuestStatus::NotStarted, rewards_claimed: false },
            // 第四幕
            Quest { id: QuestId::Izual, status: QuestStatus::NotStarted, rewards_claimed: false },
            Quest { id: QuestId::Diablo, status: QuestStatus::NotStarted, rewards_claimed: false },
            // 第五幕
            Quest { id: QuestId::SiegeOnHarrogath, status: QuestStatus::NotStarted, rewards_claimed: false },
            Quest { id: QuestId::RescueOnMountArreat, status: QuestStatus::NotStarted, rewards_claimed: false },
            Quest { id: QuestId::PrisonOfIce, status: QuestStatus::NotStarted, rewards_claimed: false },
            Quest { id: QuestId::Betrayal, status: QuestStatus::NotStarted, rewards_claimed: false },
            Quest { id: QuestId::RiteOfPassage, status: QuestStatus::NotStarted, rewards_claimed: false },
            Quest { id: QuestId::Nihlathak, status: QuestStatus::NotStarted, rewards_claimed: false },
            Quest { id: QuestId::Baal, status: QuestStatus::NotStarted, rewards_claimed: false },
        ]
    }

    /// 获取指定难度的任务
    pub fn get_difficulty_quests(&self, difficulty: Difficulty) -> &[Quest] {
        match difficulty {
            Difficulty::Normal => &self.normal,
            Difficulty::Nightmare => &self.nightmare,
            Difficulty::Hell => &self.hell,
        }
    }

    /// 获取指定难度的任务（可变）
    pub fn get_difficulty_quests_mut(&mut self, difficulty: Difficulty) -> &mut [Quest] {
        match difficulty {
            Difficulty::Normal => &mut self.normal,
            Difficulty::Nightmare => &mut self.nightmare,
            Difficulty::Hell => &mut self.hell,
        }
    }
}

impl Default for QuestData {
    fn default() -> Self {
        Self::new()
    }
}
