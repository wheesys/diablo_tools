// Copyright 2025 zl. All rights reserved.

use crate::core::d2s::{D2SFile, CharacterInfo};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, serde::Serialize)]
pub struct OpenD2SResponse {
    pub path: String,
    pub character: CharacterDisplayInfo,
}

/// 角色显示信息 (用于前端展示)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CharacterDisplayInfo {
    // 基本信息
    pub name: String,
    pub class: String,
    pub class_en: String,
    pub level: u8,

    // 状态
    pub hardcore: bool,
    pub died: bool,
    pub expansion: bool,

    // 难度进度
    pub normal_unlocked: bool,
    pub nightmare_unlocked: bool,
    pub hell_unlocked: bool,
    pub normal_act: u8,
    pub nightmare_act: u8,
    pub hell_act: u8,

    // 属性
    pub strength: u32,
    pub dexterity: u32,
    pub vitality: u32,
    pub energy: u32,
    pub unused_stat_points: u32,
    pub unused_skill_points: u32,

    // 生命/法力 (显示值，已除以256)
    pub current_hp: u32,
    pub max_hp: u32,
    pub current_mana: u32,
    pub max_mana: u32,
    pub current_stamina: u32,
    pub max_stamina: u32,

    // 金币
    pub gold: u32,
    pub stash_gold: u32,

    // 经验 (使用字符串以支持大数值)
    pub experience: String,

    // 时间戳
    pub created_at: u32,
    pub last_played: u32,
}

impl From<CharacterInfo> for CharacterDisplayInfo {
    fn from(info: CharacterInfo) -> Self {
        Self {
            name: info.name.clone(),
            class: info.class_name().to_string(),
            class_en: info.class_name_en().to_string(),
            level: info.level,

            hardcore: info.status.hardcore,
            died: info.status.died,
            expansion: info.status.expansion,

            normal_unlocked: info.difficulty.normal.unlocked,
            nightmare_unlocked: info.difficulty.nightmare.unlocked,
            hell_unlocked: info.difficulty.hell.unlocked,
            normal_act: info.difficulty.normal.current_act,
            nightmare_act: info.difficulty.nightmare.current_act,
            hell_act: info.difficulty.hell.current_act,

            strength: info.stats.strength,
            dexterity: info.stats.dexterity,
            vitality: info.stats.vitality,
            energy: info.stats.energy,
            unused_stat_points: info.stats.unused_stat_points,
            unused_skill_points: info.stats.unused_skill_points,

            current_hp: info.stats.display_hp(),
            max_hp: info.stats.display_max_hp(),
            current_mana: info.stats.display_mana(),
            max_mana: info.stats.display_max_mana(),
            current_stamina: info.stats.current_stamina / 256,
            max_stamina: info.stats.max_stamina / 256,

            gold: info.stats.gold,
            stash_gold: info.stats.stash_gold,

            experience: info.stats.experience.to_string(),

            created_at: info.created_at,
            last_played: info.last_played,
        }
    }
}

/// 打开 D2S 存档文件
#[tauri::command]
pub async fn open_d2s(file_path: String) -> Result<OpenD2SResponse, String> {
    let data = fs::read(&file_path)
        .map_err(|e| format!("读取文件失败: {}", e))?;

    let d2s = D2SFile::parse(&data)
        .map_err(|e| format!("解析存档失败: {}", e))?;

    Ok(OpenD2SResponse {
        path: file_path,
        character: CharacterDisplayInfo::from(d2s.character),
    })
}

/// 保存 D2S 存档文件
#[tauri::command]
pub async fn save_d2s(_file_path: String, _character: CharacterDisplayInfo) -> Result<(), String> {
    // TODO: 实现保存逻辑
    Ok(())
}

/// 获取角色信息
#[tauri::command]
pub async fn get_character_info(file_path: String) -> Result<CharacterDisplayInfo, String> {
    let data = fs::read(&file_path)
        .map_err(|e| format!("读取文件失败: {}", e))?;

    let d2s = D2SFile::parse(&data)
        .map_err(|e| format!("解析存档失败: {}", e))?;

    Ok(CharacterDisplayInfo::from(d2s.character))
}

/// 备份存档文件
#[tauri::command]
pub async fn backup_save(file_path: String) -> Result<String, String> {
    let path = Path::new(&file_path);
    let file_name = path.file_name()
        .and_then(|n| n.to_str())
        .ok_or("无效的文件名")?;

    // 创建备份文件名: CharacterName.d2s -> CharacterName_backup_YYYYMMDD_HHMMSS.d2s
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let backup_name = format!("{}_backup_{}.d2s",
        file_name.strip_suffix(".d2s").unwrap_or(file_name),
        timestamp
    );

    let backup_path = path.parent()
        .unwrap_or(Path::new("."))
        .join(&backup_name);

    fs::copy(&file_path, &backup_path)
        .map_err(|e| format!("备份失败: {}", e))?;

    Ok(backup_path.to_string_lossy().to_string())
}
