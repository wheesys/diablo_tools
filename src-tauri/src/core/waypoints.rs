// Copyright 2025 zl. All rights reserved.

//! 传送点系统

use super::Difficulty;
use serde::{Deserialize, Serialize};

/// 传送点ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WaypointId {
    // 第一幕
    RogueEncampment,    // 罗格营地
    BloodMoor,          // 血腥荒地
    ColdPlains,         // 冰冷之原
    StonyField,         // 石块旷野
    DarkWood,           // 黑暗森林
    BlackMarsh,         // 黑沼泽
    OuterCloister,      // 外修道院
    JailLevel1,         // 地牢一层
    InnerCloister,      // 内修道院
    Cathedral,          // 大教堂
    CatacombsLevel2,    // 坟穴二层

    // 第二幕
    LutGholein,         // 鲁高因
    SewersLevel2,       // 下水道二层
    DryHills,           // 干旱高地
    HallsOfTheDeadLevel2, // 死亡殿堂二层
    FarOasis,           // 遥远绿洲
    LostCity,           // 失落城市
    PalaceCellarLevel2, // 宫殿地窖二层
    ArcaneSanctuary,    // 奥术圣殿
    CanyonOfTheMagi,    // 法师峡谷
    DurielsLair,        // 都瑞尔巢穴

    // 第三幕
    KurastDocktown,     // 库拉斯特码头
    SpiderForest,       // 蜘蛛森林
    GreatMarsh,         // 巨大沼泽
    FlayerJungle,       // 裂地丛林
    LowerKurast,        // 下库拉斯特
    KurastBazaar,       // 库拉斯特集市
    UpperKurast,        // 上库拉斯特
    Travincal,          // 议会
    DuranceOfHateLevel2, // 憎恨囚牢二层
    DuranceOfHateLevel3, // 憎恨囚牢三层

    // 第四幕
    PandemoniumFortress, // 混沌避难所
    CityOfTheDamned,     // 被诅咒的城市
    RiverOfFlame,        // 火焰之河

    // 第五幕 (D2R扩展)
    Harrogath,          // 哈洛加斯
    FrigidHighlands,    // 极地高地
    Abaddon,            // 亚巴顿
    PitOfAcheron,       // 阿彻伦
    InfernalPit,        // 炼狱
    FrozenRiver,        // 冰河
    CrystalizedPassage, // 水晶通道
    GlacialTrail,       // 冰河小径
    HallsOfPain,        // 痛苦大厅
    HallsOfAnguish,     // 苦难大厅
    WorldstoneKeepLevel1, // 世界之石一层
    WorldstoneKeepLevel2, // 世界之石二层
    WorldstoneKeepLevel3, // 世界之石三层
    ThroneOfDestruction, // 毁灭王座
    WorldstoneChamber,  // 世界之石大殿
}

impl WaypointId {
    /// 获取传送点中文名称
    pub fn zh_name(&self) -> &'static str {
        match self {
            // 第一幕
            WaypointId::RogueEncampment => "罗格营地",
            WaypointId::BloodMoor => "血腥荒地",
            WaypointId::ColdPlains => "冰冷之原",
            WaypointId::StonyField => "石块旷野",
            WaypointId::DarkWood => "黑暗森林",
            WaypointId::BlackMarsh => "黑沼泽",
            WaypointId::OuterCloister => "外修道院",
            WaypointId::JailLevel1 => "地牢一层",
            WaypointId::InnerCloister => "内修道院",
            WaypointId::Cathedral => "大教堂",
            WaypointId::CatacombsLevel2 => "坟穴二层",

            // 第二幕
            WaypointId::LutGholein => "鲁高因",
            WaypointId::SewersLevel2 => "下水道二层",
            WaypointId::DryHills => "干旱高地",
            WaypointId::HallsOfTheDeadLevel2 => "死亡殿堂二层",
            WaypointId::FarOasis => "遥远绿洲",
            WaypointId::LostCity => "失落城市",
            WaypointId::PalaceCellarLevel2 => "宫殿地窖二层",
            WaypointId::ArcaneSanctuary => "奥术圣殿",
            WaypointId::CanyonOfTheMagi => "法师峡谷",
            WaypointId::DurielsLair => "都瑞尔巢穴",

            // 第三幕
            WaypointId::KurastDocktown => "库拉斯特码头",
            WaypointId::SpiderForest => "蜘蛛森林",
            WaypointId::GreatMarsh => "巨大沼泽",
            WaypointId::FlayerJungle => "裂地丛林",
            WaypointId::LowerKurast => "下库拉斯特",
            WaypointId::KurastBazaar => "库拉斯特集市",
            WaypointId::UpperKurast => "上库拉斯特",
            WaypointId::Travincal => "崔凡克",
            WaypointId::DuranceOfHateLevel2 => "憎恨囚牢二层",
            WaypointId::DuranceOfHateLevel3 => "憎恨囚牢三层",

            // 第四幕
            WaypointId::PandemoniumFortress => "混沌避难所",
            WaypointId::CityOfTheDamned => "被诅咒的城市",
            WaypointId::RiverOfFlame => "火焰之河",

            // 第五幕
            WaypointId::Harrogath => "哈洛加斯",
            WaypointId::FrigidHighlands => "极地高地",
            WaypointId::Abaddon => "亚巴顿",
            WaypointId::PitOfAcheron => "阿彻伦",
            WaypointId::InfernalPit => "炼狱",
            WaypointId::FrozenRiver => "冰河",
            WaypointId::CrystalizedPassage => "水晶通道",
            WaypointId::GlacialTrail => "冰河小径",
            WaypointId::HallsOfPain => "痛苦大厅",
            WaypointId::HallsOfAnguish => "苦难大厅",
            WaypointId::WorldstoneKeepLevel1 => "世界之石一层",
            WaypointId::WorldstoneKeepLevel2 => "世界之石二层",
            WaypointId::WorldstoneKeepLevel3 => "世界之石三层",
            WaypointId::ThroneOfDestruction => "毁灭王座",
            WaypointId::WorldstoneChamber => "世界之石大殿",
        }
    }

    /// 获取传送点所属幕
    pub fn act(&self) -> u8 {
        match self {
            // 第一幕
            WaypointId::RogueEncampment | WaypointId::BloodMoor | WaypointId::ColdPlains
            | WaypointId::StonyField | WaypointId::DarkWood | WaypointId::BlackMarsh
            | WaypointId::OuterCloister | WaypointId::JailLevel1 | WaypointId::InnerCloister
            | WaypointId::Cathedral | WaypointId::CatacombsLevel2 => 1,

            // 第二幕
            WaypointId::LutGholein | WaypointId::SewersLevel2 | WaypointId::DryHills
            | WaypointId::HallsOfTheDeadLevel2 | WaypointId::FarOasis | WaypointId::LostCity
            | WaypointId::PalaceCellarLevel2 | WaypointId::ArcaneSanctuary | WaypointId::CanyonOfTheMagi
            | WaypointId::DurielsLair => 2,

            // 第三幕
            WaypointId::KurastDocktown | WaypointId::SpiderForest | WaypointId::GreatMarsh
            | WaypointId::FlayerJungle | WaypointId::LowerKurast | WaypointId::KurastBazaar
            | WaypointId::UpperKurast | WaypointId::Travincal | WaypointId::DuranceOfHateLevel2
            | WaypointId::DuranceOfHateLevel3 => 3,

            // 第四幕
            WaypointId::PandemoniumFortress | WaypointId::CityOfTheDamned | WaypointId::RiverOfFlame => 4,

            // 第五幕
            WaypointId::Harrogath | WaypointId::FrigidHighlands | WaypointId::Abaddon
            | WaypointId::PitOfAcheron | WaypointId::InfernalPit | WaypointId::FrozenRiver
            | WaypointId::CrystalizedPassage | WaypointId::GlacialTrail | WaypointId::HallsOfPain
            | WaypointId::HallsOfAnguish | WaypointId::WorldstoneKeepLevel1 | WaypointId::WorldstoneKeepLevel2
            | WaypointId::WorldstoneKeepLevel3 | WaypointId::ThroneOfDestruction | WaypointId::WorldstoneChamber => 5,
        }
    }

    /// 获取所有第一幕传送点
    pub fn act1_waypoints() -> &'static [WaypointId] {
        &[
            WaypointId::RogueEncampment,
            WaypointId::BloodMoor,
            WaypointId::ColdPlains,
            WaypointId::StonyField,
            WaypointId::DarkWood,
            WaypointId::BlackMarsh,
            WaypointId::OuterCloister,
            WaypointId::JailLevel1,
            WaypointId::InnerCloister,
            WaypointId::Cathedral,
            WaypointId::CatacombsLevel2,
        ]
    }

    /// 获取所有第二幕传送点
    pub fn act2_waypoints() -> &'static [WaypointId] {
        &[
            WaypointId::LutGholein,
            WaypointId::SewersLevel2,
            WaypointId::DryHills,
            WaypointId::HallsOfTheDeadLevel2,
            WaypointId::FarOasis,
            WaypointId::LostCity,
            WaypointId::PalaceCellarLevel2,
            WaypointId::ArcaneSanctuary,
            WaypointId::CanyonOfTheMagi,
            WaypointId::DurielsLair,
        ]
    }

    /// 获取所有第三幕传送点
    pub fn act3_waypoints() -> &'static [WaypointId] {
        &[
            WaypointId::KurastDocktown,
            WaypointId::SpiderForest,
            WaypointId::GreatMarsh,
            WaypointId::FlayerJungle,
            WaypointId::LowerKurast,
            WaypointId::KurastBazaar,
            WaypointId::UpperKurast,
            WaypointId::Travincal,
            WaypointId::DuranceOfHateLevel2,
            WaypointId::DuranceOfHateLevel3,
        ]
    }

    /// 获取所有第四幕传送点
    pub fn act4_waypoints() -> &'static [WaypointId] {
        &[
            WaypointId::PandemoniumFortress,
            WaypointId::CityOfTheDamned,
            WaypointId::RiverOfFlame,
        ]
    }

    /// 获取所有第五幕传送点
    pub fn act5_waypoints() -> &'static [WaypointId] {
        &[
            WaypointId::Harrogath,
            WaypointId::FrigidHighlands,
            WaypointId::Abaddon,
            WaypointId::PitOfAcheron,
            WaypointId::InfernalPit,
            WaypointId::FrozenRiver,
            WaypointId::CrystalizedPassage,
            WaypointId::GlacialTrail,
            WaypointId::HallsOfPain,
            WaypointId::HallsOfAnguish,
            WaypointId::WorldstoneKeepLevel1,
            WaypointId::WorldstoneKeepLevel2,
            WaypointId::WorldstoneKeepLevel3,
            WaypointId::ThroneOfDestruction,
            WaypointId::WorldstoneChamber,
        ]
    }
}

/// 传送点数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaypointData {
    pub normal: Vec<bool>,
    pub nightmare: Vec<bool>,
    pub hell: Vec<bool>,
}

impl WaypointData {
    pub fn new() -> Self {
        // 第一幕11个，第二幕10个，第三幕10个，第四幕3个，第五幕15个
        // 总共49个传送点
        let total = 11 + 10 + 10 + 3 + 15;
        Self {
            normal: vec![false; total],
            nightmare: vec![false; total],
            hell: vec![false; total],
        }
    }

    /// 获取指定难度的传送点状态
    pub fn get_waypoints(&self, difficulty: Difficulty) -> &[bool] {
        match difficulty {
            Difficulty::Normal => &self.normal,
            Difficulty::Nightmare => &self.nightmare,
            Difficulty::Hell => &self.hell,
        }
    }

    /// 设置传送点状态
    pub fn set_waypoint(&mut self, difficulty: Difficulty, index: usize, activated: bool) {
        let waypoints = match difficulty {
            Difficulty::Normal => &mut self.normal,
            Difficulty::Nightmare => &mut self.nightmare,
            Difficulty::Hell => &mut self.hell,
        };
        if index < waypoints.len() {
            waypoints[index] = activated;
        }
    }

    /// 获取传送点状态
    pub fn is_activated(&self, difficulty: Difficulty, index: usize) -> bool {
        match difficulty {
            Difficulty::Normal => self.normal.get(index).copied().unwrap_or(false),
            Difficulty::Nightmare => self.nightmare.get(index).copied().unwrap_or(false),
            Difficulty::Hell => self.hell.get(index).copied().unwrap_or(false),
        }
    }
}

impl Default for WaypointData {
    fn default() -> Self {
        Self::new()
    }
}
