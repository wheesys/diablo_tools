// Copyright 2025 zl. All rights reserved.

//! 技能相关的 Tauri 命令

use crate::core::d2s::D2SFile;
use crate::core::skills::{CharacterClass};
use crate::core::skills_data::SkillList;
use serde::{Deserialize, Serialize};
use std::fs;

/// 技能显示信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDisplayInfo {
    pub id: u16,
    pub name: String,
    pub description: String,
    pub level: u8,
    pub max_level: u8,
    pub skill_tree: String,
    pub prerequisites: Vec<String>, // 前置技能名称
}

/// 难度技能信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultySkills {
    pub difficulty: String,
    pub skills: Vec<SkillDisplayInfo>,
    pub available_points: u32,
}

/// 技能响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillsResponse {
    pub class: String,
    pub class_en: String,
    pub normal: DifficultySkills,
    pub nightmare: DifficultySkills,
    pub hell: DifficultySkills,
    pub total_available_points: u32,
}

impl SkillDisplayInfo {
    pub fn from_skill_data(skill: &crate::core::skills_data::SkillData, class: CharacterClass) -> Self {
        let name = get_skill_name(class, skill.id);
        let description = get_skill_description(class, skill.id);
        let (skill_tree, max_level) = get_skill_info(class, skill.id);
        let prerequisites = get_prerequisites(class, skill.id);

        Self {
            id: skill.id,
            name,
            description,
            level: skill.level,
            max_level,
            skill_tree,
            prerequisites,
        }
    }
}

/// 获取技能名称 (简化版，实际需要完整数据库)
fn get_skill_name(class: CharacterClass, skill_id: u16) -> String {
    // TODO: 从技能数据库获取完整名称
    // 这里返回技能ID作为临时方案
    match class {
        CharacterClass::Amazon => format!("亚马逊技能{}", skill_id),
        CharacterClass::Sorceress => format!("法师技能{}", skill_id),
        CharacterClass::Necromancer => format!("死灵技能{}", skill_id),
        CharacterClass::Paladin => format!("圣骑技能{}", skill_id),
        CharacterClass::Barbarian => format!("野蛮技能{}", skill_id),
        CharacterClass::Druid => format!("德鲁伊技能{}", skill_id),
        CharacterClass::Assassin => format!("刺客技能{}", skill_id),
        CharacterClass::Warlock => get_warlock_skill_name(skill_id),
    }
}

/// 获取术士技能名称
fn get_warlock_skill_name(skill_id: u16) -> String {
    match skill_id {
        0 => "灵魂尖刺".to_string(),
        1 => "黑暗契约".to_string(),
        2 => "暗影束缚".to_string(),
        3 => "骨甲".to_string(),
        4 => "虚弱诅咒".to_string(),
        5 => "灵魂收割".to_string(),
        6 => "地狱火".to_string(),
        7 => "恶魔契约".to_string(),
        8 => "黑暗仪式".to_string(),
        9 => "灵魂守护".to_string(),
        10 => "混沌箭".to_string(),
        11 => "冥界保护".to_string(),
        12 => "天启".to_string(),
        13 => "恶魔召唤".to_string(),
        14 => "黑暗治疗".to_string(),
        15 => "灵魂燃烧".to_string(),
        16 => "妖术".to_string(),
        17 => "暗影精通".to_string(),
        18 => "地狱火协同".to_string(),
        19 => "天启协同".to_string(),
        20 => "恶魔契约协同".to_string(),
        21 => "混沌精通".to_string(),
        22 => "灵魂虹吸".to_string(),
        23 => "黑暗冲击".to_string(),
        24 => "冥界守护".to_string(),
        25 => "痛苦诅咒".to_string(),
        26 => "召唤恶魔".to_string(),
        27 => "吸血鬼之触".to_string(),
        28 => "末日".to_string(),
        29 => "黑暗治疗协同".to_string(),
        30 => "暗影冲击".to_string(),
        31 => "地狱风暴".to_string(),
        32 => "邪恶契约".to_string(),
        33 => "死亡大师".to_string(),
        34 => "灵魂交换".to_string(),
        _ => format!("术士技能{}", skill_id),
    }
}

/// 获取技能描述
fn get_skill_description(class: CharacterClass, skill_id: u16) -> String {
    // TODO: 从技能数据库获取完整描述
    format!("{}的技能 {}", class.zh_name(), skill_id)
}

/// 获取技能信息 (技能树和最大等级)
fn get_skill_info(class: CharacterClass, skill_id: u16) -> (String, u8) {
    // 简化版技能树分类
    let (tree, max_level) = match class {
        CharacterClass::Sorceress => {
            if skill_id < 10 { ("火系", 20) }
            else if skill_id < 20 { ("冰系", 20) }
            else { ("雷系", 20) }
        }
        CharacterClass::Necromancer => {
            if skill_id < 10 { ("白骨系", 20) }
            else if skill_id < 20 { ("亡灵系", 20) }
            else { ("毒系", 20) }
        }
        CharacterClass::Warlock => {
            if skill_id < 12 { ("暗影魔法", 20) }
            else if skill_id < 22 { ("诅咒系", 20) }
            else { ("召唤系", 20) }
        }
        _ => ("通用", 20),
    };
    (tree.to_string(), max_level)
}

/// 获取前置技能
fn get_prerequisites(class: CharacterClass, skill_id: u16) -> Vec<String> {
    // TODO: 从技能数据库获取完整前置技能
    Vec::new()
}

/// 获取存档中的技能数据
#[tauri::command]
pub async fn get_skills(file_path: String) -> Result<SkillsResponse, String> {
    let data = fs::read(&file_path)
        .map_err(|e| format!("读取文件失败: {}", e))?;

    let d2s = D2SFile::parse(&data)
        .map_err(|e| format!("解析存档失败: {}", e))?;

    let class = d2s.character.class;
    let class_name = class.zh_name().to_string();
    let class_name_en = class.en_name().to_string();

    // 转换普通难度技能
    let normal_skills: Vec<SkillDisplayInfo> = d2s.character.skills.normal
        .iter()
        .map(|s| SkillDisplayInfo::from_skill_data(s, class))
        .collect();

    // 转换噩梦难度技能
    let nightmare_skills: Vec<SkillDisplayInfo> = d2s.character.skills.nightmare
        .iter()
        .map(|s| SkillDisplayInfo::from_skill_data(s, class))
        .collect();

    // 转换地狱难度技能
    let hell_skills: Vec<SkillDisplayInfo> = d2s.character.skills.hell
        .iter()
        .map(|s| SkillDisplayInfo::from_skill_data(s, class))
        .collect();

    // 计算可用技能点
    let total_available_points = d2s.character.stats.unused_skill_points;

    Ok(SkillsResponse {
        class: class_name,
        class_en: class_name_en,
        normal: DifficultySkills {
            difficulty: "normal".to_string(),
            skills: normal_skills,
            available_points: total_available_points,
        },
        nightmare: DifficultySkills {
            difficulty: "nightmare".to_string(),
            skills: nightmare_skills,
            available_points: total_available_points,
        },
        hell: DifficultySkills {
            difficulty: "hell".to_string(),
            skills: hell_skills,
            available_points: total_available_points,
        },
        total_available_points,
    })
}

/// 设置技能等级
#[tauri::command]
pub async fn set_skill_level(
    file_path: String,
    difficulty: String,
    skill_id: u16,
    level: u8,
) -> Result<(), String> {
    // TODO: 实现技能等级设置和保存
    Ok(())
}
