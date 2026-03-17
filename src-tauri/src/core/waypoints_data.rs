// Copyright 2025 zl. All rights reserved.

//! 传送点数据解析 - 位于偏移 0x279

use super::bit_reader::BitReader;
use super::bit_writer::BitWriter;
use super::error::{Error, Result};
use super::waypoints::WaypointId;
use super::quests::Difficulty;
use serde::{Deserialize, Serialize};

/// 传送点数据结构
///
/// 格式说明:
/// - WS头: 2字节 ("WS")
/// - 未知: 6字节 (固定值)
/// - 每个难度24字节:
///   - 2字节: {0x02, 0x01}
///   - 5字节: 传送点位字段 (最多40个传送点，用5字节表示)
///   - 17字节: 未知
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaypointDataRaw {
    /// 正常难度传送点位字段 (40位 = 5字节)
    pub normal: [u8; 5],
    /// 噩梦难度传送点位字段
    pub nightmare: [u8; 5],
    /// 地狱难度传送点位字段
    pub hell: [u8; 5],
}

impl WaypointDataRaw {
    /// 解析传送点数据
    ///
    /// 从偏移 0x279 开始，共81字节
    pub fn parse(reader: &mut BitReader) -> Result<Self> {
        reader.align_to_byte();

        // 读取WS头
        let ws = reader.read_bytes(2)?;
        if ws != b"WS" {
            return Err(Error::ParseError("无效的传送点数据头 (应为WS)".to_string()));
        }

        // 读取6字节未知数据
        let _unknown = reader.read_bytes(6)?;

        // 读取正常难度传送点 (24字节)
        let _header1 = reader.read_bytes(2)?; // {0x02, 0x01}
        let mut normal = [0u8; 5];
        for i in 0..5 {
            normal[i] = reader.read_u8()?;
        }
        let _padding1 = reader.read_bytes(17)?;

        // 读取噩梦难度传送点 (24字节)
        let _header2 = reader.read_bytes(2)?;
        let mut nightmare = [0u8; 5];
        for i in 0..5 {
            nightmare[i] = reader.read_u8()?;
        }
        let _padding2 = reader.read_bytes(17)?;

        // 读取地狱难度传送点 (24字节)
        let _header3 = reader.read_bytes(2)?;
        let mut hell = [0u8; 5];
        for i in 0..5 {
            hell[i] = reader.read_u8()?;
        }
        let _padding3 = reader.read_bytes(17)?;

        Ok(Self {
            normal,
            nightmare,
            hell,
        })
    }

    /// 写入传送点数据
    pub fn write(&self, writer: &mut BitWriter) -> Result<()> {
        writer.align_to_byte();

        // 写入WS头
        writer.write_bytes(b"WS")?;

        // 写入6字节固定值
        writer.write_bytes(&[0x01, 0x00, 0x00, 0x00, 0x50, 0x00])?;

        // 写入正常难度
        writer.write_bytes(&[0x02, 0x01])?;
        writer.write_bytes(&self.normal)?;
        writer.write_bytes(&[0u8; 17])?;

        // 写入噩梦难度
        writer.write_bytes(&[0x02, 0x01])?;
        writer.write_bytes(&self.nightmare)?;
        writer.write_bytes(&[0u8; 17])?;

        // 写入地狱难度
        writer.write_bytes(&[0x02, 0x01])?;
        writer.write_bytes(&self.hell)?;
        writer.write_bytes(&[0u8; 17])?;

        Ok(())
    }

    /// 检查指定难度的传送点是否激活
    pub fn is_activated(&self, difficulty: Difficulty, waypoint_id: WaypointId) -> bool {
        let bytes = match difficulty {
            Difficulty::Normal => &self.normal,
            Difficulty::Nightmare => &self.nightmare,
            Difficulty::Hell => &self.hell,
        };

        // 传送点位字段使用小端序，检查对应位
        let bit_index = Self::waypoint_to_bit_index(waypoint_id);
        let byte_index = bit_index / 8;
        let bit_offset = bit_index % 8;

        if byte_index < 5 {
            (bytes[byte_index] & (1 << bit_offset)) != 0
        } else {
            false
        }
    }

    /// 设置传送点状态
    pub fn set_waypoint(&mut self, difficulty: Difficulty, waypoint_id: WaypointId, activated: bool) {
        let bytes = match difficulty {
            Difficulty::Normal => &mut self.normal,
            Difficulty::Nightmare => &mut self.nightmare,
            Difficulty::Hell => &mut self.hell,
        };

        let bit_index = Self::waypoint_to_bit_index(waypoint_id);
        let byte_index = bit_index / 8;
        let bit_offset = bit_index % 8;

        if byte_index < 5 {
            if activated {
                bytes[byte_index] |= 1 << bit_offset;
            } else {
                bytes[byte_index] &= !(1 << bit_offset);
            }
        }
    }

    /// 将传送点ID转换为位索引
    fn waypoint_to_bit_index(waypoint_id: WaypointId) -> usize {
        // 第一幕传送点 (0-10)
        // 第二幕传送点 (0-9)
        // 第三幕传送点 (0-9)
        // 第四幕传送点 (0-2)
        // 第五幕传送点 (0-14)

        // 按难度顺序排列，每种难度内部按传送点顺序
        // D2R使用一个连续的位字段表示所有传送点

        let act = waypoint_id.act() as usize;

        // 计算当前幕之前的所有传送点数量
        let mut offset = 0;
        if act > 1 {
            offset += 11; // 第一幕
        }
        if act > 2 {
            offset += 10; // 第二幕
        }
        if act > 3 {
            offset += 10; // 第三幕
        }
        if act > 4 {
            offset += 3; // 第四幕
        }

        // 加上当前幕内的传送点索引
        let waypoint_in_act = Self::get_waypoint_index_in_act(waypoint_id);
        offset + waypoint_in_act
    }

    /// 获取传送点在当前幕内的索引
    fn get_waypoint_index_in_act(waypoint_id: WaypointId) -> usize {
        // 根据幕返回索引
        match waypoint_id {
            // 第一幕
            WaypointId::RogueEncampment => 0,
            WaypointId::BloodMoor => 1,
            WaypointId::ColdPlains => 2,
            WaypointId::StonyField => 3,
            WaypointId::DarkWood => 4,
            WaypointId::BlackMarsh => 5,
            WaypointId::OuterCloister => 6,
            WaypointId::JailLevel1 => 7,
            WaypointId::InnerCloister => 8,
            WaypointId::Cathedral => 9,
            WaypointId::CatacombsLevel2 => 10,

            // 第二幕
            WaypointId::LutGholein => 0,
            WaypointId::SewersLevel2 => 1,
            WaypointId::DryHills => 2,
            WaypointId::HallsOfTheDeadLevel2 => 3,
            WaypointId::FarOasis => 4,
            WaypointId::LostCity => 5,
            WaypointId::PalaceCellarLevel2 => 6,
            WaypointId::ArcaneSanctuary => 7,
            WaypointId::CanyonOfTheMagi => 8,
            WaypointId::DurielsLair => 9,

            // 第三幕
            WaypointId::KurastDocktown => 0,
            WaypointId::SpiderForest => 1,
            WaypointId::GreatMarsh => 2,
            WaypointId::FlayerJungle => 3,
            WaypointId::LowerKurast => 4,
            WaypointId::KurastBazaar => 5,
            WaypointId::UpperKurast => 6,
            WaypointId::Travincal => 7,
            WaypointId::DuranceOfHateLevel2 => 8,
            WaypointId::DuranceOfHateLevel3 => 9,

            // 第四幕
            WaypointId::PandemoniumFortress => 0,
            WaypointId::CityOfTheDamned => 1,
            WaypointId::RiverOfFlame => 2,

            // 第五幕
            WaypointId::Harrogath => 0,
            WaypointId::FrigidHighlands => 1,
            WaypointId::Abaddon => 2,
            WaypointId::PitOfAcheron => 3,
            WaypointId::InfernalPit => 4,
            WaypointId::FrozenRiver => 5,
            WaypointId::CrystalizedPassage => 6,
            WaypointId::GlacialTrail => 7,
            WaypointId::HallsOfPain => 8,
            WaypointId::HallsOfAnguish => 9,
            WaypointId::WorldstoneKeepLevel1 => 10,
            WaypointId::WorldstoneKeepLevel2 => 11,
            WaypointId::WorldstoneKeepLevel3 => 12,
            WaypointId::ThroneOfDestruction => 13,
            WaypointId::WorldstoneChamber => 14,
        }
    }

    /// 获取所有已激活的传送点列表
    pub fn get_active_waypoints(&self, difficulty: Difficulty) -> Vec<WaypointId> {
        let mut waypoints = Vec::new();

        // 检查所有传送点
        for wp in Self::all_waypoints() {
            if self.is_activated(difficulty, *wp) {
                waypoints.push(*wp);
            }
        }

        waypoints
    }

    /// 获取所有传送点
    fn all_waypoints() -> &'static [WaypointId] {
        // 第一幕
        const ACT1: &[WaypointId] = &[
            WaypointId::RogueEncampment, WaypointId::BloodMoor, WaypointId::ColdPlains,
            WaypointId::StonyField, WaypointId::DarkWood, WaypointId::BlackMarsh,
            WaypointId::OuterCloister, WaypointId::JailLevel1, WaypointId::InnerCloister,
            WaypointId::Cathedral, WaypointId::CatacombsLevel2,
        ];

        // 第二幕
        const ACT2: &[WaypointId] = &[
            WaypointId::LutGholein, WaypointId::SewersLevel2, WaypointId::DryHills,
            WaypointId::HallsOfTheDeadLevel2, WaypointId::FarOasis, WaypointId::LostCity,
            WaypointId::PalaceCellarLevel2, WaypointId::ArcaneSanctuary, WaypointId::CanyonOfTheMagi,
            WaypointId::DurielsLair,
        ];

        // 第三幕
        const ACT3: &[WaypointId] = &[
            WaypointId::KurastDocktown, WaypointId::SpiderForest, WaypointId::GreatMarsh,
            WaypointId::FlayerJungle, WaypointId::LowerKurast, WaypointId::KurastBazaar,
            WaypointId::UpperKurast, WaypointId::Travincal, WaypointId::DuranceOfHateLevel2,
            WaypointId::DuranceOfHateLevel3,
        ];

        // 第四幕
        const ACT4: &[WaypointId] = &[
            WaypointId::PandemoniumFortress, WaypointId::CityOfTheDamned, WaypointId::RiverOfFlame,
        ];

        // 第五幕
        const ACT5: &[WaypointId] = &[
            WaypointId::Harrogath, WaypointId::FrigidHighlands, WaypointId::Abaddon,
            WaypointId::PitOfAcheron, WaypointId::InfernalPit, WaypointId::FrozenRiver,
            WaypointId::CrystalizedPassage, WaypointId::GlacialTrail, WaypointId::HallsOfPain,
            WaypointId::HallsOfAnguish, WaypointId::WorldstoneKeepLevel1, WaypointId::WorldstoneKeepLevel2,
            WaypointId::WorldstoneKeepLevel3, WaypointId::ThroneOfDestruction, WaypointId::WorldstoneChamber,
        ];

        // 使用静态数组的方式返回所有传送点
        // 由于Rust的限制，我们使用迭代器的方式
        ACT1.iter()
            .chain(ACT2.iter())
            .chain(ACT3.iter())
            .chain(ACT4.iter())
            .chain(ACT5.iter())
            .copied()
            .collect::<Vec<_>>()
            .leak()
    }
}

impl Default for WaypointDataRaw {
    fn default() -> Self {
        Self {
            normal: [0; 5],
            nightmare: [0; 5],
            hell: [0; 5],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waypoint_bit_index() {
        // 罗格营地应该是第0个传送点
        let idx = WaypointDataRaw::waypoint_to_bit_index(WaypointId::RogueEncampment);
        assert_eq!(idx, 0);

        // 鲁高因是第11个 (第二幕第一个)
        let idx = WaypointDataRaw::waypoint_to_bit_index(WaypointId::LutGholein);
        assert_eq!(idx, 11);
    }

    #[test]
    fn test_waypoint_activation() {
        let mut data = WaypointDataRaw::default();

        // 激活罗格营地
        data.set_waypoint(Difficulty::Normal, WaypointId::RogueEncampment, true);
        assert!(data.is_activated(Difficulty::Normal, WaypointId::RogueEncampment));

        // 检查默认未激活
        assert!(!data.is_activated(Difficulty::Normal, WaypointId::BloodMoor));
    }
}
