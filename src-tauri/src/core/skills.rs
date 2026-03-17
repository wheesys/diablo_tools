// Copyright 2025 zl. All rights reserved.

//! 技能系统 - 支持术士君临版本

use serde::{Deserialize, Serialize};

/// D2R 职业枚举 (支持术士君临)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum CharacterClass {
    Amazon = 0,
    Sorceress = 1,
    Necromancer = 2,
    Paladin = 3,
    Barbarian = 4,
    Druid = 5,
    Assassin = 6,
    Warlock = 7,  // 术士君临新增
}

impl CharacterClass {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Amazon),
            1 => Some(Self::Sorceress),
            2 => Some(Self::Necromancer),
            3 => Some(Self::Paladin),
            4 => Some(Self::Barbarian),
            5 => Some(Self::Druid),
            6 => Some(Self::Assassin),
            7 => Some(Self::Warlock),
            _ => None,
        }
    }

    /// 职业中文名称
    pub fn zh_name(&self) -> &'static str {
        match self {
            Self::Amazon => "亚马逊",
            Self::Sorceress => "法师",
            Self::Necromancer => "死灵法师",
            Self::Paladin => "圣骑士",
            Self::Barbarian => "野蛮人",
            Self::Druid => "德鲁伊",
            Self::Assassin => "刺客",
            Self::Warlock => "术士",
        }
    }

    /// 职业英文名称
    pub fn en_name(&self) -> &'static str {
        match self {
            Self::Amazon => "Amazon",
            Self::Sorceress => "Sorceress",
            Self::Necromancer => "Necromancer",
            Self::Paladin => "Paladin",
            Self::Barbarian => "Barbarian",
            Self::Druid => "Druid",
            Self::Assassin => "Assassin",
            Self::Warlock => "Warlock",
        }
    }

    /// 获取技能数量 (术士君临扩展后)
    pub fn skill_count(&self) -> usize {
        match self {
            Self::Amazon => 20,      // 弓箭、标枪技能树
            Self::Sorceress => 30,   // 火、冰、雷三系
            Self::Necromancer => 30, // 白骨、亡灵、毒系
            Self::Paladin => 20,     // 战斗、进攻、防御
            Self::Barbarian => 30,   // 战斗、呐喊、战斗专家
            Self::Druid => 30,       // 元素、变形、召唤
            Self::Assassin => 30,    // 武学、影子、陷阱
            Self::Warlock => 35,     // 术士君临新增技能
        }
    }
}

/// 技能数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: u16,
    pub level: u8,
}

/// 角色技能数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSkills {
    pub class: CharacterClass,
    pub skills: Vec<Skill>,
    pub available_skill_points: u16,
}

impl CharacterSkills {
    pub fn new(class: CharacterClass) -> Self {
        let skill_count = class.skill_count();
        Self {
            class,
            skills: Vec::with_capacity(skill_count),
            available_skill_points: 0,
        }
    }

    /// 获取技能等级
    pub fn get_skill_level(&self, skill_id: u16) -> u8 {
        self.skills
            .iter()
            .find(|s| s.id == skill_id)
            .map(|s| s.level)
            .unwrap_or(0)
    }

    /// 设置技能等级
    pub fn set_skill_level(&mut self, skill_id: u16, level: u8) {
        if let Some(skill) = self.skills.iter_mut().find(|s| s.id == skill_id) {
            skill.level = level;
        } else {
            self.skills.push(Skill { id: skill_id, level });
        }
    }
}
