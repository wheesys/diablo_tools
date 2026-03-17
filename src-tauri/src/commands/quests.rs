// Copyright 2025 zl. All rights reserved.

//! 任务相关的 Tauri 命令

use crate::core::d2s::D2SFile;
use crate::core::quests::{Act, QuestId};
use crate::core::quests_data::{QuestData, QuestFlags, QuestList};
use serde::{Deserialize, Serialize};
use std::fs;

/// 任务显示信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestDisplayInfo {
    pub id: String,
    pub name: String,
    pub act: u8,
    pub introduced: bool,
    pub completed: bool,
    pub reward_claimed: bool,
}

/// 难度任务信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultyQuests {
    pub difficulty: String,
    pub act1: Vec<QuestDisplayInfo>,
    pub act2: Vec<QuestDisplayInfo>,
    pub act3: Vec<QuestDisplayInfo>,
    pub act4: Vec<QuestDisplayInfo>,
    pub act5: Vec<QuestDisplayInfo>,
}

/// 任务响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestsResponse {
    pub normal: DifficultyQuests,
    pub nightmare: DifficultyQuests,
    pub hell: DifficultyQuests,
}

impl QuestDisplayInfo {
    pub fn from_quest_data(quest: &QuestData) -> Self {
        Self {
            id: format!("{:?}", quest.id),
            name: quest.id.zh_name().to_string(),
            act: quest.id.act() as u8,
            introduced: quest.flags.introduced,
            completed: quest.flags.completed,
            reward_claimed: quest.flags.reward_claimed,
        }
    }
}

/// 按幕分组任务
fn group_by_act(quests: &[QuestData]) -> [Vec<QuestDisplayInfo>; 5] {
    let mut acts = empty_acts();
    for quest in quests {
        let act_index = quest.id.act() as usize;
        if act_index < 5 {
            acts[act_index].push(QuestDisplayInfo::from_quest_data(quest));
        }
    }
    acts
}

/// 获取存档中的任务数据
#[tauri::command]
pub async fn get_quests(file_path: String) -> Result<QuestsResponse, String> {
    let data = fs::read(&file_path)
        .map_err(|e| format!("读取文件失败: {}", e))?;

    let d2s = D2SFile::parse(&data)
        .map_err(|e| format!("解析存档失败: {}", e))?;

    // 按幕分组普通难度任务
    let normal_acts = group_by_act(&d2s.character.quests.normal);

    // 按幕分组噩梦难度任务
    let nightmare_acts = group_by_act(&d2s.character.quests.nightmare);

    // 按幕分组地狱难度任务
    let hell_acts = group_by_act(&d2s.character.quests.hell);

    Ok(QuestsResponse {
        normal: DifficultyQuests {
            difficulty: "normal".to_string(),
            act1: normal_acts[0].clone(),
            act2: normal_acts[1].clone(),
            act3: normal_acts[2].clone(),
            act4: normal_acts[3].clone(),
            act5: normal_acts[4].clone(),
        },
        nightmare: DifficultyQuests {
            difficulty: "nightmare".to_string(),
            act1: nightmare_acts[0].clone(),
            act2: nightmare_acts[1].clone(),
            act3: nightmare_acts[2].clone(),
            act4: nightmare_acts[3].clone(),
            act5: nightmare_acts[4].clone(),
        },
        hell: DifficultyQuests {
            difficulty: "hell".to_string(),
            act1: hell_acts[0].clone(),
            act2: hell_acts[1].clone(),
            act3: hell_acts[2].clone(),
            act4: hell_acts[3].clone(),
            act5: hell_acts[4].clone(),
        },
    })
}

/// 设置任务状态
#[tauri::command]
pub async fn set_quest_status(
    file_path: String,
    difficulty: String,
    quest_id: String,
    introduced: bool,
    completed: bool,
    reward_claimed: bool,
) -> Result<(), String> {
    // TODO: 实现任务状态设置和保存
    Ok(())
}

fn empty_acts() -> [Vec<QuestDisplayInfo>; 5] {
    [vec![], vec![], vec![], vec![], vec![]]
}
