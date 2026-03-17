// Copyright 2025 zl. All rights reserved.

//! 物品系统解析
//!
//! D2R 物品格式说明:
//! - 物品块以 "JM" 开头 (D2R单个物品可能没有JM头，但物品列表有)
//! - 物品ID使用 Huffman 编码 (D2R)
//! - 物品包含: 基础属性、魔法属性、镶嵌物等

use super::bit_reader::BitReader;
use super::bit_writer::BitWriter;
use super::huffman::{HuffmanDecoder, HuffmanEncoder};
use super::error::{Error, Result};
use serde::{Deserialize, Serialize};

/// 物品质量
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemQuality {
    LowQuality = 1,
    Normal = 2,
    Superior = 3,
    Magic = 4,
    Set = 5,
    Rare = 6,
    Unique = 7,
}

impl ItemQuality {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::LowQuality),
            2 => Some(Self::Normal),
            3 => Some(Self::Superior),
            4 => Some(Self::Magic),
            5 => Some(Self::Set),
            6 => Some(Self::Rare),
            7 => Some(Self::Unique),
            _ => None,
        }
    }

    pub fn zh_name(&self) -> &'static str {
        match self {
            ItemQuality::LowQuality => "劣质",
            ItemQuality::Normal => "普通",
            ItemQuality::Superior => "优越",
            ItemQuality::Magic => "魔法",
            ItemQuality::Set => "套装",
            ItemQuality::Rare => "稀有",
            ItemQuality::Unique => "暗金",
        }
    }
}

/// 物品位置/父容器
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemLocation {
    Stored(StoredLocation),
    Equipped(EquipSlot),
    Belt,
    Cursor,
    Item,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StoredLocation {
    Inventory = 1,
    HoradricCube = 4,
    Stash = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EquipSlot {
    Helmet = 1,
    Amulet = 2,
    Armor = 3,
    Weapon = 4,
    RingRight = 5,
    RingLeft = 6,
    Belt = 7,
    Boots = 8,
    Gloves = 9,
    AltWeaponRight = 11,
    AltWeaponLeft = 12,
}

/// 物品基础信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemBase {
    /// 物品代码 (3字符, 如 "ceg" = Grand Charm)
    pub code: String,
    /// 物品质量
    pub quality: ItemQuality,
    /// 是否已鉴定
    pub identified: bool,
    /// 是否以太
    pub ethereal: bool,
    /// 是否有孔
    pub socketed: bool,
    /// 孔数量
    pub sockets: u8,
    /// 物品位置
    pub location: ItemLocation,
    /// 网格位置 (用于库存/仓库)
    pub grid_x: u8,
    pub grid_y: u8,
    /// 耐久度
    pub durability: Option<u16>,
    /// 最大耐久度
    pub max_durability: Option<u16>,
    /// 数量 (用于叠堆物品，如金币、药水)
    pub quantity: u32,
}

/// 物品属性 (魔法属性)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemModifier {
    /// 属性ID (对应 ItemStatCost.txt)
    pub id: u16,
    /// 属性值
    pub values: Vec<u32>,
}

/// 完整物品数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub base: ItemBase,
    pub modifiers: Vec<ItemModifier>,
    /// 镶嵌物品 (如宝石、符文)
    pub socketed_items: Vec<Item>,
    /// 个人化名称
    pub personalized_name: Option<String>,
    /// 制造者名称
    pub crafter_name: Option<String>,
}

/// 物品列表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemList {
    pub items: Vec<Item>,
}

impl ItemList {
    /// 解析物品列表
    ///
    /// 格式:
    /// - 2字节: "JM" 标记
    /// - 2字节: 物品数量
    /// - N个物品: 每个物品可变长度
    pub fn parse(reader: &mut BitReader, version: super::d2s::D2SVersion) -> Result<Self> {
        reader.align_to_byte();

        // 检查物品列表头
        let jm = reader.read_bytes(2)?;
        if jm != b"JM" {
            return Err(Error::ParseError("无效的物品列表头 (应为JM)".to_string()));
        }

        // 读取物品数量
        let count = reader.read_u16_le()? as usize;

        let mut items = Vec::with_capacity(count);
        for _ in 0..count {
            items.push(Self::parse_item(reader, version)?);
        }

        Ok(Self { items })
    }

    /// 解析单个物品
    fn parse_item(reader: &mut BitReader, version: super::d2s::D2SVersion) -> Result<Item> {
        reader.align_to_byte();

        // 物品头标记 (D2R可选)
        let has_jm = if version.is_d2r() {
            // D2R: 某些情况下物品没有JM头
            let peek = reader.peek_bytes(2)?;
            peek == b"JM"
        } else {
            true
        };

        if has_jm {
            let jm = reader.read_bytes(2)?;
            if jm != b"JM" {
                return Err(Error::ParseError("无效的物品头 (应为JM)".to_string()));
            }
        }

        // 解析物品基础数据 (位字段)
        let _item_flags = reader.read_bits(4)?;
        let identified = reader.read_bit()?;
        reader.read_bits(6)?; // 未知
        let socketed = reader.read_bit()?;
        let _picked_up = reader.read_bit()?;
        reader.read_bits(2)?; // 未知
        let _is_ear = reader.read_bit()?;
        let _starter_gear = reader.read_bit()?;
        reader.read_bits(3)?; // 未知
        let _compact = reader.read_bit()?;
        let ethereal = reader.read_bit()?;
        let _unknown = reader.read_bit()?;
        let personalized = reader.read_bit()?;
        let _unknown2 = reader.read_bit()?;
        let _runeword = reader.read_bit()?;
        reader.read_bits(15)?; // 未知

        // 物品位置
        let parent = reader.read_bits(3)?;
        let equipped = reader.read_bit()?;
        let column = reader.read_bits(4)?;
        let row = reader.read_bits(3)?;
        let _stored = reader.read_bits(3)?;

        reader.read_bits(4)?; // 未知

        // 物品代码 (24位 = 3字节，使用Huffman编码在D2R中)
        let code = if version.is_d2r() {
            // D2R: 使用 Huffman 编码
            let decoder = HuffmanDecoder::new_d2r();
            decoder.decode_string_null(reader)?
        } else {
            // 经典版: 直接读取3字节
            let mut code_bytes = [0u8; 3];
            for b in &mut code_bytes {
                *b = reader.read_u8()?;
            }
            String::from_utf8_lossy(&code_bytes).trim_end_matches('\0').to_string()
        };

        // 如果不是 Compact，继续读取扩展数据
        // TODO: 检查 compact 标志
        let modifiers = Vec::new();

        Ok(Item {
            base: ItemBase {
                code,
                quality: ItemQuality::Normal,
                identified,
                ethereal,
                socketed,
                sockets: 0,
                location: Self::parse_location(parent, equipped, column, row)?,
                grid_x: column as u8,
                grid_y: row as u8,
                durability: None,
                max_durability: None,
                quantity: 0,
            },
            modifiers,
            socketed_items: Vec::new(),
            personalized_name: None,
            crafter_name: None,
        })
    }

    fn parse_location(parent: u32, equipped: bool, column: u32, _row: u32) -> Result<ItemLocation> {
        Ok(if equipped {
            ItemLocation::Equipped(match column {
                1 => EquipSlot::Helmet,
                2 => EquipSlot::Amulet,
                3 => EquipSlot::Armor,
                4 => EquipSlot::Weapon,
                5 => EquipSlot::RingRight,
                6 => EquipSlot::RingLeft,
                7 => EquipSlot::Belt,
                8 => EquipSlot::Boots,
                9 => EquipSlot::Gloves,
                11 => EquipSlot::AltWeaponRight,
                12 => EquipSlot::AltWeaponLeft,
                _ => return Err(Error::ParseError("无效的装备位置".to_string())),
            })
        } else {
            match parent {
                0 => ItemLocation::Item,
                1 => ItemLocation::Stored(StoredLocation::Inventory),
                2 => ItemLocation::Belt,
                4 => ItemLocation::Cursor,
                5 => ItemLocation::Stored(StoredLocation::HoradricCube),
                6 => ItemLocation::Stored(StoredLocation::Stash),
                _ => return Err(Error::ParseError("无效的物品位置".to_string())),
            }
        })
    }

    /// 解析物品属性
    fn parse_modifiers(reader: &mut BitReader) -> Result<Vec<ItemModifier>> {
        let mut modifiers = Vec::new();

        loop {
            // 读取属性ID (9位，0x1FF = 511 = 列表结束)
            let id = reader.read_bits(9)?;

            if id == 0x1FF {
                break; // 结束标记
            }

            // 读取属性值
            let mut values = Vec::new();
            // TODO: 根据属性ID确定值的位数和数量
            // 这里简化处理
            let value = reader.read_bits(32)? as u32;
            values.push(value);

            modifiers.push(ItemModifier { id: id as u16, values });
        }

        Ok(modifiers)
    }

    /// 写入物品列表
    pub fn write(&self, writer: &mut BitWriter, version: super::d2s::D2SVersion) -> Result<()> {
        writer.align_to_byte();

        // 写入JM头
        writer.write_bytes(b"JM")?;

        // 写入物品数量
        writer.write_u16_le(self.items.len() as u16)?;

        // 写入每个物品
        for item in &self.items {
            Self::write_item(writer, item, version)?;
        }

        Ok(())
    }

    fn write_item(writer: &mut BitWriter, item: &Item, version: super::d2s::D2SVersion) -> Result<()> {
        writer.align_to_byte();

        // 写入JM头
        writer.write_bytes(b"JM")?;

        // 写入物品基础标志 (TODO: 完整实现)
        writer.write_bits(1, 0)?; // item_flags
        writer.write_bit(item.base.identified)?;
        writer.write_bits(6, 0)?;
        writer.write_bit(item.base.socketed)?;
        writer.write_bit(false)?; // picked_up
        writer.write_bits(2, 0)?;
        writer.write_bit(false)?; // is_ear
        writer.write_bit(false)?; // starter_gear
        writer.write_bits(3, 0)?;
        writer.write_bit(false)?; // compact
        writer.write_bit(item.base.ethereal)?;
        writer.write_bit(false)?; // unknown
        writer.write_bit(item.personalized_name.is_some())?;
        writer.write_bit(false)?; // unknown2
        writer.write_bit(false)?; // runeword
        writer.write_bits(15, 0)?;

        // 写入位置信息
        // TODO: 根据 location 设置正确的位
        writer.write_bits(3, 1)?; // parent: Inventory
        writer.write_bit(false)?; // equipped
        writer.write_bits(4, item.base.grid_x as u32)?;
        writer.write_bits(3, item.base.grid_y as u32)?;
        writer.write_bits(3, 0)?; // stored: Inventory
        writer.write_bits(4, 0)?;

        // 写入物品代码
        if version.is_d2r() {
            // D2R: 使用 Huffman 编码
            let encoder = HuffmanEncoder::new_d2r();
            writer.write_bytes(&item.base.code.as_bytes())?;
        } else {
            // 经典版: 直接写入
            let code_bytes = item.base.code.as_bytes();
            for &b in code_bytes {
                writer.write_u8(b)?;
            }
            for _ in code_bytes.len()..3 {
                writer.write_u8(0)?;
            }
        }

        // TODO: 写入属性、镶嵌物等

        Ok(())
    }
}

/// BitReader 扩展 - peek 功能
trait BitReaderExt {
    fn peek_bytes(&mut self, count: usize) -> Result<Vec<u8>>;
}

impl BitReaderExt for BitReader {
    fn peek_bytes(&mut self, count: usize) -> Result<Vec<u8>> {
        let mut bytes = Vec::with_capacity(count);
        for _ in 0..count {
            bytes.push(self.read_u8()?);
        }
        // 注意: 这是一个简化实现，会移动读取位置
        // 完整实现需要保存和恢复位偏移
        Ok(bytes)
    }
}

impl Default for ItemList {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_quality_names() {
        assert_eq!(ItemQuality::Unique.zh_name(), "暗金");
        assert_eq!(ItemQuality::Set.zh_name(), "套装");
    }
}
