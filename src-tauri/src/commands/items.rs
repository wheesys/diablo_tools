// Copyright 2025 zl. All rights reserved.

//! 物品相关的 Tauri 命令

use crate::core::d2s::D2SFile;
use crate::core::items::{Item, ItemLocation, StoredLocation, EquipSlot};
use serde::{Deserialize, Serialize};
use std::fs;

/// 物品显示信息 (用于前端)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDisplayInfo {
    pub id: String,
    pub name: String,
    pub base_item: String,
    pub quality: String,
    pub quality_en: String,
    pub level: u32,
    pub identified: bool,
    pub ethereal: bool,
    pub socketed: bool,
    pub sockets: u8,
    pub quantity: u32,
    pub position: ItemPositionDisplay,
    pub grid_x: Option<u8>,
    pub grid_y: Option<u8>,
    pub properties: Vec<ItemPropertyDisplay>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPropertyDisplay {
    pub name: String,
    pub value: String,
    pub min: Option<i32>,
    pub max: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPositionDisplay {
    pub location: String, // "inventory", "equipment", "belt", "stash", "cube"
    pub slot: Option<String>, // "helm", "weapon", "ring_left", etc.
}

/// 物品列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemsResponse {
    pub inventory: Vec<ItemDisplayInfo>,
    pub equipment: Vec<ItemDisplayInfo>,
    pub stash: Vec<ItemDisplayInfo>,
    pub cube: Vec<ItemDisplayInfo>,
    pub belt: Vec<ItemDisplayInfo>,
}

impl ItemDisplayInfo {
    /// 从内部 Item 结构转换
    pub fn from_item(item: &Item, index: usize) -> Self {
        let quality = item.base.quality.zh_name().to_string();
        let quality_en = format!("{:?}", item.base.quality);

        // 获取物品位置信息
        let (location, slot, grid_x, grid_y) = match &item.base.location {
            ItemLocation::Stored(stored) => match stored {
                StoredLocation::Inventory => ("inventory".to_string(), None, Some(item.base.grid_x), Some(item.base.grid_y)),
                StoredLocation::HoradricCube => ("cube".to_string(), None, Some(item.base.grid_x), Some(item.base.grid_y)),
                StoredLocation::Stash => ("stash".to_string(), None, Some(item.base.grid_x), Some(item.base.grid_y)),
            },
            ItemLocation::Equipped(equip) => {
                let slot_name = match equip {
                    EquipSlot::Helmet => "helm",
                    EquipSlot::Amulet => "amulet",
                    EquipSlot::Armor => "armor",
                    EquipSlot::Weapon => "weapon-main",
                    EquipSlot::RingRight => "ring-right",
                    EquipSlot::RingLeft => "ring-left",
                    EquipSlot::Belt => "belt",
                    EquipSlot::Boots => "boots",
                    EquipSlot::Gloves => "gloves",
                    EquipSlot::AltWeaponRight => "weapon-off",
                    EquipSlot::AltWeaponLeft => "weapon-off",
                };
                ("equipment".to_string(), Some(slot_name.to_string()), None, None)
            },
            ItemLocation::Belt => ("belt".to_string(), None, None, None),
            ItemLocation::Cursor => ("cursor".to_string(), None, None, None),
            ItemLocation::Item => ("item".to_string(), None, None, None),
        };

        Self {
            id: format!("item-{}", index),
            name: format_item_name(item),
            base_item: item.base.code.clone(),
            quality,
            quality_en,
            level: 1, // TODO: 从物品数据获取
            identified: item.base.identified,
            ethereal: item.base.ethereal,
            socketed: item.base.socketed,
            sockets: item.base.sockets,
            quantity: item.base.quantity,
            position: ItemPositionDisplay {
                location,
                slot,
            },
            grid_x: if grid_x.is_some() { grid_x } else { None },
            grid_y: if grid_y.is_some() { grid_y } else { None },
            properties: format_item_properties(item),
        }
    }
}

/// 格式化物品名称
fn format_item_name(item: &Item) -> String {
    if let Some(name) = &item.personalized_name {
        return name.clone();
    }
    if let Some(name) = &item.crafter_name {
        return format!("{}的 {}", name, item.base.code);
    }

    // 简单返回物品代码 (TODO: 需要物品名称数据库)
    item.base.code.clone()
}

/// 格式化物品属性
fn format_item_properties(item: &Item) -> Vec<ItemPropertyDisplay> {
    let mut props = Vec::new();

    // 添加基础属性
    if item.base.ethereal {
        props.push(ItemPropertyDisplay {
            name: "无形的".to_string(),
            value: String::new(),
            min: None,
            max: None,
        });
    }
    if item.base.socketed && item.base.sockets > 0 {
        props.push(ItemPropertyDisplay {
            name: "孔数".to_string(),
            value: item.base.sockets.to_string(),
            min: None,
            max: None,
        });
    }

    // 添加魔法属性 (TODO: 需要属性名称数据库)
    for modifier in &item.modifiers {
        props.push(ItemPropertyDisplay {
            name: format!("属性 {}", modifier.id),
            value: modifier.values.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            min: None,
            max: None,
        });
    }

    props
}

/// 获取存档中的所有物品
#[tauri::command]
pub async fn get_items(file_path: String) -> Result<ItemsResponse, String> {
    let data = fs::read(&file_path)
        .map_err(|e| format!("读取文件失败: {}", e))?;

    let d2s = D2SFile::parse(&data)
        .map_err(|e| format!("解析存档失败: {}", e))?;

    // 分类物品
    let mut inventory = Vec::new();
    let mut equipment = Vec::new();
    let mut stash = Vec::new();
    let mut cube = Vec::new();
    let mut belt = Vec::new();

    let items = &d2s.character.items;
    for (index, item) in items.items.iter().enumerate() {
        let display = ItemDisplayInfo::from_item(item, index);

        match display.position.location.as_str() {
            "inventory" => inventory.push(display),
            "equipment" => equipment.push(display),
            "stash" => stash.push(display),
            "cube" => cube.push(display),
            "belt" => belt.push(display),
            _ => {}
        }
    }

    Ok(ItemsResponse {
        inventory,
        equipment,
        stash,
        cube,
        belt,
    })
}

/// 获取单个物品的详细信息
#[tauri::command]
pub async fn get_item_details(file_path: String, item_id: String) -> Result<ItemDisplayInfo, String> {
    let data = fs::read(&file_path)
        .map_err(|e| format!("读取文件失败: {}", e))?;

    let d2s = D2SFile::parse(&data)
        .map_err(|e| format!("解析存档失败: {}", e))?;

    let items = &d2s.character.items;
    // 解析 item_id 格式: "item-{index}"
    if let Some(index_str) = item_id.strip_prefix("item-") {
        if let Ok(index) = index_str.parse::<usize>() {
            if index < items.items.len() {
                return Ok(ItemDisplayInfo::from_item(&items.items[index], index));
            }
        }
    }

    Err("物品未找到".to_string())
}
