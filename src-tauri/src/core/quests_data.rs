// Copyright 2025 zl. All rights reserved.

//! 任务数据解析 - 位于偏移 0x14F (335字节处，共298字节)

use super::bit_reader::BitReader;
use super::bit_writer::BitWriter;
use super::error::{Error, Result};
use super::quests::{Difficulty, QuestId, QuestStatus};
use serde::{Deserialize, Serialize};

/// 任务状态标志
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestFlags {
    /// 0x01: 已介绍
    pub introduced: bool,
    /// 0x02: 已完成
    pub completed: bool,
    /// 0x04: 已奖励领取
    pub reward_claimed: bool,
    /// 0x08: 标志4 (未知)
    pub flag4: bool,
    /// 0x10: 标志5 (未知)
    pub flag5: bool,
    /// 0x20: 标志6 (未知)
    pub flag6: bool,
    /// 0x40: 标志7 (未知)
    pub flag7: bool,
    /// 0x80: 标志8 (未知)
    pub flag8: bool,
}

impl QuestFlags {
    pub fn from_byte(value: u8) -> Self {
        Self {
            introduced: (value & 0x01) != 0,
            completed: (value & 0x02) != 0,
            reward_claimed: (value & 0x04) != 0,
            flag4: (value & 0x08) != 0,
            flag5: (value & 0x10) != 0,
            flag6: (value & 0x20) != 0,
            flag7: (value & 0x40) != 0,
            flag8: (value & 0x80) != 0,
        }
    }

    pub fn to_byte(&self) -> u8 {
        let mut value = 0u8;
        if self.introduced { value |= 0x01; }
        if self.completed { value |= 0x02; }
        if self.reward_claimed { value |= 0x04; }
        if self.flag4 { value |= 0x08; }
        if self.flag5 { value |= 0x10; }
        if self.flag6 { value |= 0x20; }
        if self.flag7 { value |= 0x40; }
        if self.flag8 { value |= 0x80; }
        value
    }
}

/// 单个任务的数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestData {
    pub id: QuestId,
    pub flags: QuestFlags,
}

impl QuestData {
    pub fn new(id: QuestId) -> Self {
        Self {
            id,
            flags: QuestFlags {
                introduced: false,
                completed: false,
                reward_claimed: false,
                flag4: false,
                flag5: false,
                flag6: false,
                flag7: false,
                flag8: false,
            },
        }
    }

    /// 获取任务状态
    pub fn get_status(&self) -> QuestStatus {
        if self.flags.completed {
            QuestStatus::Completed
        } else if self.flags.introduced {
            QuestStatus::InProgress
        } else {
            QuestStatus::NotStarted
        }
    }

    /// 设置任务状态
    pub fn set_status(&mut self, status: QuestStatus) {
        match status {
            QuestStatus::NotStarted => {
                self.flags.introduced = false;
                self.flags.completed = false;
                self.flags.reward_claimed = false;
            }
            QuestStatus::InProgress => {
                self.flags.introduced = true;
                self.flags.completed = false;
                self.flags.reward_claimed = false;
            }
            QuestStatus::Completed => {
                self.flags.introduced = true;
                self.flags.completed = true;
                // 已完成不代表已领取奖励
            }
        }
    }
}

/// 任务数据列表 - 按难度组织
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestList {
    pub normal: Vec<QuestData>,
    pub nightmare: Vec<QuestData>,
    pub hell: Vec<QuestData>,
}

impl QuestList {
    /// 解析任务数据
    ///
    /// 从偏移 0x14F 开始，共298字节
    /// 结构:
    /// - 每个难度约 6 * 6 = 36 字节 (6个任务，每个任务6字节)
    /// - 第一幕: 7个任务
    /// - 第二幕: 6个任务
    /// - 第三幕: 6个任务
    /// - 第四幕: 3个任务 (arius等)
    /// - 第五幕: 7个任务 (D2R扩展)
    pub fn parse(reader: &mut BitReader, version: super::d2s::D2SVersion) -> Result<Self> {
        reader.align_to_byte();

        // 每个任务占用6字节
        // 第一幕: 7个任务 = 42字节
        let mut normal = Self::parse_act_quests(reader, true)?;

        // 检查是否有扩展任务 (第五幕)
        let has_expansion = version.is_d2r();

        // 第二幕和第三幕
        // 这里简化处理，读取所有任务数据
        // TODO: 完整实现任务解析

        let nightmare = Vec::new();
        let hell = Vec::new();

        Ok(Self {
            normal,
            nightmare,
            hell,
        })
    }

    /// 解析单幕任务
    fn parse_act_quests(reader: &mut BitReader, _is_normal: bool) -> Result<Vec<QuestData>> {
        let mut quests = Vec::new();

        // 第一幕任务
        let quest_ids = [
            QuestId::DenOfEvil,
            QuestId::BloodRaven,
            QuestId::Smith,
            QuestId::Countess,
            QuestId::Cemetery,
            QuestId::ToolsOfTheTrade,
            QuestId::Imbue,
        ];

        for quest_id in quest_ids {
            let flags = Self::parse_quest_flags(reader)?;
            quests.push(QuestData {
                id: quest_id,
                flags,
            });
        }

        Ok(quests)
    }

    /// 解析单个任务的标志
    fn parse_quest_flags(reader: &mut BitReader) -> Result<QuestFlags> {
        // 每个任务6字节
        let _unknown1 = reader.read_u16_le()?;
        let _unknown2 = reader.read_u8()?;
        let _unknown3 = reader.read_u8()?;
        let _unknown4 = reader.read_u8()?;
        let flags_byte = reader.read_u8()?;

        Ok(QuestFlags::from_byte(flags_byte))
    }

    /// 写入任务数据
    pub fn write(&self, writer: &mut BitWriter) -> Result<()> {
        // TODO: 实现写入逻辑
        Ok(())
    }

    /// 获取指定难度的任务列表
    pub fn get_difficulty_quests(&self, difficulty: Difficulty) -> &[QuestData] {
        match difficulty {
            Difficulty::Normal => &self.normal,
            Difficulty::Nightmare => &self.nightmare,
            Difficulty::Hell => &self.hell,
        }
    }

    /// 获取指定难度的任务列表（可变）
    pub fn get_difficulty_quests_mut(&mut self, difficulty: Difficulty) -> &mut [QuestData] {
        match difficulty {
            Difficulty::Normal => &mut self.normal,
            Difficulty::Nightmare => &mut self.nightmare,
            Difficulty::Hell => &mut self.hell,
        }
    }
}

impl Default for QuestList {
    fn default() -> Self {
        Self {
            normal: Self::create_normal_quests(),
            nightmare: Vec::new(),
            hell: Vec::new(),
        }
    }
}

impl QuestList {
    fn create_normal_quests() -> Vec<QuestData> {
        vec![
            QuestData::new(QuestId::DenOfEvil),
            QuestData::new(QuestId::BloodRaven),
            QuestData::new(QuestId::Smith),
            QuestData::new(QuestId::Countess),
            QuestData::new(QuestId::Cemetery),
            QuestData::new(QuestId::ToolsOfTheTrade),
            QuestData::new(QuestId::Imbue),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quest_flags() {
        let flags = QuestFlags {
            introduced: true,
            completed: false,
            reward_claimed: false,
            flag4: false,
            flag5: false,
            flag6: false,
            flag7: false,
            flag8: false,
        };

        let byte = flags.to_byte();
        let restored = QuestFlags::from_byte(byte);

        assert_eq!(restored.introduced, flags.introduced);
        assert_eq!(restored.completed, flags.completed);
    }

    #[test]
    fn test_quest_status() {
        let mut quest = QuestData::new(QuestId::DenOfEvil);
        assert_eq!(quest.get_status(), QuestStatus::NotStarted);

        quest.set_status(QuestStatus::InProgress);
        assert_eq!(quest.get_status(), QuestStatus::InProgress);

        quest.set_status(QuestStatus::Completed);
        assert_eq!(quest.get_status(), QuestStatus::Completed);
    }
}
