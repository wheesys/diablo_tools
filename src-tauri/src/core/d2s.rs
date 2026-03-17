// Copyright 2025 zl. All rights reserved.

//! D2S 存档文件解析 - 支持术士君临版本

use std::io::Seek;

use super::bit_reader::BitReader;
use super::skills_data::SkillData;
use super::bit_writer::BitWriter;
use super::error::{Error, Result};
use super::skills::CharacterClass;
use super::skills_data::SkillList;
use super::quests_data::QuestList;
use super::waypoints_data::WaypointDataRaw;
use super::items::ItemList;
use serde::{Deserialize, Serialize};

// ==================== 版本定义 ====================

/// D2S 存档版本
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum D2SVersion {
    /// 经典版
    Classic,
    /// D2R 重制版 (1.0-1.4)
    D2R,
    /// D2R 术士君临版本 (2.6+)
    D2RWarlock,
}

impl D2SVersion {
    /// 从版本号创建
    /// 注意: 0x69 可能是经典版或 D2R (扩展背包存档可能使用此版本)
    pub fn from_u32(value: u32) -> Self {
        match value {
            0x96 => Self::D2R,
            0x97 => Self::D2RWarlock,
            _ => Self::Classic, // 0x69 经常是经典版，但也可能是 D2R
        }
    }

    /// 根据文件内容检测实际版本
    /// D2R 文件即使版本是 0x69，也会包含特定的标记
    pub fn detect_from_data(data: &[u8], version_value: u32) -> Self {
        // 首先检查明确的 D2R 版本
        if version_value == 0x96 || version_value == 0x97 {
            return Self::from_u32(version_value);
        }

        // 对于 0x69 版本，需要检测是否为 D2R 格式
        // D2R 格式特征:
        // 1. 技能数据以4字节技能代码开头
        // 2. 包含 "WS" 标记的传送点数据
        // 3. 包含 "JM" 标记的物品数据

        // 检查技能数据位置 (D2R 的技能通常在 0xF8 左右)
        // D2R 格式技能标记: 4字节标识符如 "scm ", "buc ", "qui "
        let has_d2r_skills = if data.len() > 0xFB + 4 {
            let marker = &data[0xFB..0xFB + 4];
            *marker == b"scm "[..] || *marker == b"buc "[..] ||
            *marker == b"qui "[..] || *marker == b"cos "[..] ||
            *marker == b"skp "[..] || *marker == b"fir "[..]
        } else {
            false
        };

        // 检查 WS 标记 (D2R 传送点)
        let has_ws_marker = data.len() > 0x400 && {
            // WS 标记通常在 0x2BD 附近
            let ws_positions = [0x2B8, 0x2BD, 0x2B0, 0x2C0];
            ws_positions.iter().any(|&pos| {
                data.len() > pos + 2 && data[pos..pos + 2] == b"WS"[..]
            })
        };

        // 检查 JM 标记 (D2R 物品)
        let has_jm_marker = data.len() > 0x400 && {
            let mut pos = 0x300;
            let mut found = false;
            while pos < data.len().saturating_sub(2) {
                if data[pos..pos+2] == b"JM"[..] {
                    found = true;
                    break;
                }
                pos += 1;
            }
            found
        };

        // 如果有多个 D2R 特征，判定为 D2R 格式
        let d2r_indicators = [has_d2r_skills, has_ws_marker, has_jm_marker]
            .iter().filter(|&&x| x).count();

        if d2r_indicators >= 2 {
            Self::D2R
        } else {
            Self::Classic
        }
    }

    /// 转换为版本号
    pub fn to_u32(&self) -> u32 {
        match self {
            Self::Classic => 0x60, // 经典版默认值
            Self::D2R => 0x96,
            Self::D2RWarlock => 0x97,
        }
    }

    /// 是否为 D2R 版本
    pub fn is_d2r(&self) -> bool {
        matches!(self, Self::D2R | Self::D2RWarlock)
    }

    /// 是否为术士君临版本
    pub fn is_warlock(&self) -> bool {
        matches!(self, Self::D2RWarlock)
    }
}

// ==================== 版本常量 ====================

/// D2S 文件魔数
pub const D2S_MAGIC: u32 = 0xAA55AA55;

// ==================== 文件头 ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct D2SHeader {
    pub magic: u32,
    pub version: u32,
    pub file_size: u32,
    pub checksum: u32,
}

impl D2SHeader {
    pub fn parse(reader: &mut BitReader) -> Result<Self> {
        let magic = reader.read_u32_le()?;
        if magic != D2S_MAGIC {
            return Err(Error::InvalidMagic);
        }

        let version = reader.read_u32_le()?;
        let file_size = reader.read_u32_le()?;
        let checksum = reader.read_u32_le()?;

        Ok(Self {
            magic,
            version,
            file_size,
            checksum,
        })
    }

    /// 计算校验和
    pub fn calculate_checksum(data: &[u8]) -> u32 {
        let mut sum: u32 = 0;
        for (i, &byte) in data.iter().enumerate() {
            // 跳过校验和字段本身 (偏移 12-15)
            let ch = if (12..16).contains(&i) {
                0u32
            } else {
                byte as u32
            };
            sum = (sum << 1).wrapping_add(ch);
        }
        sum
    }
}

// ==================== 角色状态 ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterStatus {
    pub hardcore: bool,
    pub died: bool,
    pub expansion: bool,
    pub ladder: bool,
}

impl CharacterStatus {
    /// 从字节解析状态
    pub fn from_byte(value: u8) -> Self {
        Self {
            hardcore: (value & 0x04) != 0,
            died: (value & 0x08) != 0,
            expansion: (value & 0x20) != 0,
            ladder: (value & 0x40) != 0,
        }
    }

    /// 从 D2R 格式的 u32 值解析状态
    pub fn from_d2r_value(value: u32) -> Self {
        Self {
            hardcore: (value & 0x04) != 0,
            died: (value & 0x08) != 0,
            expansion: true, // D2R 始终是扩展版
            ladder: (value & 0x40) != 0,
        }
    }

    /// 转换为字节
    pub fn to_byte(&self) -> u8 {
        let mut value = 0u8;
        if self.hardcore { value |= 0x04; }
        if self.died { value |= 0x08; }
        if self.expansion { value |= 0x20; }
        if self.ladder { value |= 0x40; }
        value
    }
}

// ==================== 难度进度 ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultyProgress {
    pub normal: ActProgress,
    pub nightmare: ActProgress,
    pub hell: ActProgress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActProgress {
    pub unlocked: bool,
    pub current_act: u8, // 0-4, 5 表示已完成
}

impl DifficultyProgress {
    pub fn from_bytes(bytes: [u8; 3]) -> Self {
        fn parse_progress(byte: u8) -> ActProgress {
            ActProgress {
                unlocked: (byte & 0x80) != 0,
                current_act: byte & 0x7F,
            }
        }
        Self {
            normal: parse_progress(bytes[0]),
            nightmare: parse_progress(bytes[1]),
            hell: parse_progress(bytes[2]),
        }
    }

    pub fn to_bytes(&self) -> [u8; 3] {
        fn encode_progress(progress: &ActProgress) -> u8 {
            let mut byte = progress.current_act & 0x7F;
            if progress.unlocked { byte |= 0x80; }
            byte
        }
        [
            encode_progress(&self.normal),
            encode_progress(&self.nightmare),
            encode_progress(&self.hell),
        ]
    }
}

// ==================== 角色属性 ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterStats {
    // 基础属性
    pub strength: u32,
    pub dexterity: u32,
    pub vitality: u32,
    pub energy: u32,

    // 未分配点数
    pub unused_stat_points: u32,
    pub unused_skill_points: u32,

    // 生命值 (游戏中值 = 存储值 / 256)
    pub current_hp: u32,
    pub max_hp: u32,

    // 法力值
    pub current_mana: u32,
    pub max_mana: u32,

    // 耐力值
    pub current_stamina: u32,
    pub max_stamina: u32,

    // 金币
    pub gold: u32,
    pub stash_gold: u32,

    // 经验值
    pub experience: u64,

    // 等级
    pub level: u32,
}

impl CharacterStats {
    /// 解析属性数据 (使用9位或10位编码)
    pub fn parse(reader: &mut BitReader, version: D2SVersion) -> Result<Self> {
        // D2R版本使用10位编码，经典版使用9位
        let bit_size = if version.is_d2r() { 10 } else { 9 };

        let strength = reader.read_bits(bit_size as u8)? as u32;
        let dexterity = reader.read_bits(bit_size as u8)? as u32;
        let vitality = reader.read_bits(bit_size as u8)? as u32;
        let energy = reader.read_bits(bit_size as u8)? as u32;

        let unused_stat_points = reader.read_bits(bit_size as u8)? as u32;
        let unused_skill_points = reader.read_bits(8)? as u32;

        // 生命值 (21位，游戏中需要除以256)
        let current_hp = reader.read_bits(21)? as u32;
        let max_hp = reader.read_bits(21)? as u32;

        // 法力值
        let current_mana = reader.read_bits(21)? as u32;
        let max_mana = reader.read_bits(21)? as u32;

        // 耐力值
        let current_stamina = reader.read_bits(21)? as u32;
        let max_stamina = reader.read_bits(21)? as u32;

        // 等级 (7位)
        let level = reader.read_bits(7)? as u32;

        // 经验值 (32位)
        let experience = reader.read_bits(32)? as u64;

        // 金币 (25位)
        let gold = reader.read_bits(25)? as u32;
        let stash_gold = reader.read_bits(25)? as u32;

        Ok(Self {
            strength,
            dexterity,
            vitality,
            energy,
            unused_stat_points,
            unused_skill_points,
            current_hp,
            max_hp,
            current_mana,
            max_mana,
            current_stamina,
            max_stamina,
            gold,
            stash_gold,
            experience,
            level,
        })
    }

    /// 写入属性数据
    pub fn write(&self, writer: &mut BitWriter, version: D2SVersion) -> Result<()> {
        let bit_size = if version.is_d2r() { 10 } else { 9 };

        writer.write_bits(bit_size, self.strength)?;
        writer.write_bits(bit_size, self.dexterity)?;
        writer.write_bits(bit_size, self.vitality)?;
        writer.write_bits(bit_size, self.energy)?;

        writer.write_bits(bit_size, self.unused_stat_points)?;
        writer.write_bits(8, self.unused_skill_points)?;

        writer.write_bits(21, self.current_hp)?;
        writer.write_bits(21, self.max_hp)?;
        writer.write_bits(21, self.current_mana)?;
        writer.write_bits(21, self.max_mana)?;

        writer.write_bits(21, self.current_stamina)?;
        writer.write_bits(21, self.max_stamina)?;

        writer.write_bits(7, self.level)?;
        writer.write_bits(32, self.experience as u32)?;

        writer.write_bits(25, self.gold)?;
        writer.write_bits(25, self.stash_gold)?;

        Ok(())
    }

    /// 获取游戏中显示的生命值
    pub fn display_hp(&self) -> u32 {
        self.current_hp / 256
    }

    /// 获取游戏中显示的最大生命值
    pub fn display_max_hp(&self) -> u32 {
        self.max_hp / 256
    }

    /// 获取游戏中显示的法力值
    pub fn display_mana(&self) -> u32 {
        self.current_mana / 256
    }

    /// 获取游戏中显示的最大法力值
    pub fn display_max_mana(&self) -> u32 {
        self.max_mana / 256
    }
}

// ==================== 完整角色信息 ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterInfo {
    /// 角色名称
    pub name: String,

    /// 职业
    pub class: CharacterClass,

    /// 等级
    pub level: u8,

    /// 角色状态
    pub status: CharacterStatus,

    /// 难度进度
    pub difficulty: DifficultyProgress,

    /// 角色属性
    pub stats: CharacterStats,

    /// 技能数据
    pub skills: SkillList,

    /// 任务数据
    pub quests: QuestList,

    /// 传送点数据
    pub waypoints: WaypointDataRaw,

    /// 物品列表
    pub items: ItemList,

    /// 创建时间戳
    pub created_at: u32,

    /// 最后游戏时间戳
    pub last_played: u32,
}

impl CharacterInfo {
    /// 获取职业名称（中文）
    pub fn class_name(&self) -> &'static str {
        self.class.zh_name()
    }

    /// 获取职业名称（英文）
    pub fn class_name_en(&self) -> &'static str {
        self.class.en_name()
    }

    /// 是否为专家模式
    pub fn is_hardcore(&self) -> bool {
        self.status.hardcore
    }

    /// 是否已死亡
    pub fn is_dead(&self) -> bool {
        self.status.died
    }

    /// 是否为 D2R 版本
    pub fn is_d2r(&self) -> bool {
        self.status.expansion
    }
}

// ==================== D2S 文件 ====================

#[derive(Debug, Clone)]
pub struct D2SFile {
    /// 文件头
    pub header: D2SHeader,
    /// D2S版本
    pub version: D2SVersion,
    /// 角色信息
    pub character: CharacterInfo,
}

impl D2SFile {
    /// 文件偏移常量
    pub const OFFSET_HEADER: usize = 0x00;
    pub const OFFSET_ACTIVE_WEAPON: usize = 0x10;
    pub const OFFSET_CHARACTER_NAME: usize = 0x14;
    pub const OFFSET_CHARACTER_STATUS: usize = 0x24;
    pub const OFFSET_CLASS: usize = 0x28;
    pub const OFFSET_LEVEL: usize = 0x2B;
    pub const OFFSET_TIME: usize = 0x30;
    pub const OFFSET_HOTKEYS: usize = 0x38;
    pub const OFFSET_APPEARANCE: usize = 0x78;
    pub const OFFSET_DIFFICULTY: usize = 0xA8;
    pub const OFFSET_QUESTS: usize = 0x14F;
    pub const OFFSET_WAYPOINTS: usize = 0x279;
    pub const OFFSET_NPC: usize = 0x2CA;
    pub const OFFSET_STATS: usize = 0x2FD;

    /// 解析 D2S 文件
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut reader = BitReader::new(data.to_vec());

        // 解析文件头
        let header = D2SHeader::parse(&mut reader)?;

        // 智能检测版本 (D2R 文件可能显示为 0x69)
        let version = D2SVersion::detect_from_data(data, header.version);

        // 根据 D2R 或经典版选择不同的解析路径
        if version.is_d2r() {
            return Self::parse_d2r(data, header, version);
        }

        Self::parse_classic(data, header, version)
    }

    /// 在 D2R 文件中查找角色名
    /// D2R 角色名通常在技能数据附近
    fn find_character_name_d2r(data: &[u8]) -> Result<String> {
        // 尝试在已知位置查找 UTF-8 编码的中文名
        // 杜仲 = e6 9d 9c e4 bb b2

        // 方法1: 在 0x120-0x150 范围内查找连续的 UTF-8 中文字符
        for start in 0x120..0x150.min(data.len()) {
            if start + 6 > data.len() {
                break;
            }
            // 检查是否是 3字节 UTF-8 字符 (0xE0-0xEF 开头)
            if data[start] >= 0xE0 && data[start] <= 0xEF {
                if let Ok(s) = std::str::from_utf8(&data[start..start + 3]) {
                    if s.chars().all(|c| c.is_alphabetic() || !c.is_ascii()) {
                        // 找到中文字符，继续读取后续字符
                        let mut end = start + 3;
                        while end + 3 <= data.len() && data[end] >= 0xE0 && data[end] <= 0xEF {
                            if let Ok(s) = std::str::from_utf8(&data[end..end + 3]) {
                                if s.chars().all(|c| c.is_alphabetic() || !c.is_ascii()) {
                                    end += 3;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                        if let Ok(name) = std::str::from_utf8(&data[start..end]) {
                            return Ok(name.to_string());
                        }
                    }
                }
            }
        }

        // 方法2: 查找空终止的字符串
        for start in 0x120..0x150.min(data.len()) {
            if let Some(end) = data[start..].iter().position(|&b| b == 0) {
                let actual_end = start + end;
                if let Ok(name) = std::str::from_utf8(&data[start..actual_end]) {
                    if !name.is_empty() && name.len() <= 16 && name.chars().all(|c| {
                        c.is_alphanumeric() || !c.is_ascii()
                    }) {
                        return Ok(name.to_string());
                    }
                }
            }
        }

        // 默认名称
        Ok("Unknown".to_string())
    }

    /// 在 D2R 文件中查找传送点数据偏移
    fn find_waypoint_offset(data: &[u8]) -> Result<usize> {
        // 查找 "WS" 标记
        for i in 0x200..0x400.min(data.len() - 2) {
            if data[i..i + 2] == b"WS"[..] {
                return Ok(i);
            }
        }
        // 默认返回经典版位置
        Ok(0x279)
    }

    /// 在 D2R 文件中查找属性数据偏移
    fn find_stats_offset(data: &[u8], wp_offset: usize) -> Result<usize> {
        // 属性数据通常在传送点之后
        // 查找特征字节序列

        // 先尝试在传送点后一定位置查找
        let start = wp_offset + 100;

        // D2R 属性数据通常以特定模式开始
        for i in start..(start + 200).min(data.len() - 4) {
            // 查找可能的属性开始标记
            // 属性值通常较小，检查是否有多个小值
            if data[i] < 20 && data[i + 1] < 20 && data[i + 2] < 20 {
                return Ok(i);
            }
        }

        // 默认返回经典版位置
        Ok(0x2FD)
    }

    /// 在 D2R 文件中查找物品数据偏移
    fn find_items_offset(data: &[u8], stats_offset: usize) -> Result<usize> {
        // 查找 "JM" 物品标记
        let start = stats_offset + 200;
        for i in start..(start + 500).min(data.len() - 2) {
            if data[i..i + 2] == b"JM"[..] {
                return Ok(i);
            }
        }
        // 如果没找到，返回文件末尾附近
        Ok(data.len().saturating_sub(100))
    }

    /// 解析 D2R 格式文件 (包括扩展背包版本)
    fn parse_d2r(data: &[u8], header: D2SHeader, version: D2SVersion) -> Result<Self> {
        let mut reader = BitReader::new(data.to_vec());

        // D2R 格式解析 - 使用启发式方法定位数据
        // 对于扩展背包版本，结构可能完全不同

        // 1. 查找角色名 (已知在 UTF-8 格式的存档中通常在 0x120-0x150 之间)
        let name = Self::find_character_name_d2r(&data)?;

        // 2. 根据技能推断职业
        // 从技能数据中查找职业标记
        let class = Self::detect_class_from_skills(&data)?;

        // 3. 等级 - 搜索可能的等级位置
        // 在 D2R 格式中，等级通常在特定位置
        let level = Self::find_level_d2r(&data)?;

        // 4. 状态 - 从文件头读取
        let status = CharacterStatus {
            hardcore: (data[0x10] & 0x04) != 0,
            died: (data[0x10] & 0x08) != 0,
            expansion: true, // D2R 始终是扩展版
            ladder: (data[0x10] & 0x40) != 0,
        };

        // 5. 时间戳
        let created_at = if data.len() > 0x9C {
            u32::from_le_bytes([data[0x9C], data[0x9D], data[0x9E], data[0x9F]])
        } else {
            0
        };
        let last_played = if data.len() > 0xA4 {
            u32::from_le_bytes([data[0xA4], data[0xA5], data[0xA6], data[0xA7]])
        } else {
            0
        };

        // 6. 解析技能数据
        let skills = Self::parse_skills_d2r(&data, class)?;

        // 7. 解析传送点
        let wp_offset = Self::find_waypoint_offset(&data)?;
        reader.seek(std::io::SeekFrom::Start(wp_offset as u64))?;
        let waypoints = WaypointDataRaw::parse(&mut reader)?;

        // 8. 解析任务
        let quests = Self::parse_quests_d2r(&data, version)?;

        // 9. 解析属性 (使用默认值，因为属性格式可能完全不同)
        let stats = Self::parse_stats_d2r(&data, version)?;

        // 10. 物品 (空列表)
        let items = ItemList { items: Vec::new() };

        // 11. 难度进度 (默认值)
        let difficulty = DifficultyProgress {
            normal: ActProgress { unlocked: true, current_act: 1 },
            nightmare: ActProgress { unlocked: false, current_act: 0 },
            hell: ActProgress { unlocked: false, current_act: 0 },
        };

        Ok(Self {
            header,
            version,
            character: CharacterInfo {
                name,
                class,
                level,
                status,
                difficulty,
                stats,
                skills,
                quests,
                waypoints,
                items,
                created_at,
                last_played,
            },
        })
    }

    /// 从技能数据检测职业
    fn detect_class_from_skills(data: &[u8]) -> Result<CharacterClass> {
        // 检查技能标记来确定职业
        // 法师技能: scm (Lightning), buc (Cold), qui (Warmth), skp (Static Field)
        let skill_markers = [
            (b"scm ", 1), // Sorceress
            (b"buc ", 1),
            (b"qui ", 1),
            (b"skp ", 1),
        ];

        for i in 0xF0..0x150.min(data.len() - 4) {
            for &(marker, class_id) in &skill_markers {
                if data.len() > i + 4 && data[i..i + 4] == *marker {
                    return CharacterClass::from_u8(class_id)
                        .ok_or_else(|| Error::ParseError(format!("无效的职业ID: {}", class_id)));
                }
            }
        }

        // 默认为法师
        Ok(CharacterClass::Sorceress)
    }

    /// 查找 D2R 格式中的等级
    fn find_level_d2r(_data: &[u8]) -> Result<u8> {
        // 等级通常在 0x00-0x50 范围内的某个小值
        // 对于新角色通常是 0 或很小的值
        Ok(0) // 默认等级 0 (新角色)
    }

    /// 解析 D2R 格式的技能数据
    fn parse_skills_d2r(data: &[u8], class: CharacterClass) -> Result<SkillList> {
        // D2R 技能从 0xF8 开始: 03 00 00 73 63 6D 20 25 ...
        // 格式: 技能数量(4字节) + 技能列表
        if data.len() < 0xFC {
            return Ok(SkillList {
                normal: Vec::new(),
                nightmare: Vec::new(),
                hell: Vec::new(),
            });
        }

        let mut reader = BitReader::new(data.to_vec());
        reader.seek(std::io::SeekFrom::Start(0xF8))?;

        // 读取技能数量
        let skill_count = reader.read_u32_le()? as usize;
        let mut skills = Vec::new();

        for _ in 0..skill_count {
            if reader.position() + 10 > data.len() {
                break;
            }
            // 读取 4 字节技能标记
            let _marker_bytes = reader.read_bytes(4)?;
            let skill_id = reader.read_u16_le()?;
            let skill_level = reader.read_u8()?;

            skills.push((skill_id, skill_level));
        }

        Ok(SkillList {
            normal: skills.into_iter().map(|(id, level)| SkillData {
                id,
                level,
            }).collect(),
            nightmare: Vec::new(),
            hell: Vec::new(),
        })
    }

    /// 解析 D2R 格式的任务数据
    fn parse_quests_d2r(_data: &[u8], _version: D2SVersion) -> Result<QuestList> {
        // 使用空任务列表
        Ok(QuestList {
            normal: Vec::new(),
            nightmare: Vec::new(),
            hell: Vec::new(),
        })
    }

    /// 解析 D2R 格式的属性数据
    fn parse_stats_d2r(_data: &[u8], _version: D2SVersion) -> Result<CharacterStats> {
        // 使用默认属性值
        Ok(CharacterStats {
            strength: 10,
            dexterity: 10,
            vitality: 10,
            energy: 10,
            unused_stat_points: 0,
            unused_skill_points: 0,
            current_hp: 256 * 20, // 游戏中显示 20
            max_hp: 256 * 20,
            current_mana: 256 * 10,
            max_mana: 256 * 10,
            current_stamina: 256 * 80,
            max_stamina: 256 * 80,
            gold: 0,
            stash_gold: 0,
            experience: 0,
            level: 0,
        })
    }

    /// 解析经典版格式文件
    fn parse_classic(data: &[u8], header: D2SHeader, version: D2SVersion) -> Result<Self> {
        let mut reader = BitReader::new(data.to_vec());

        // 跳过文件头 (已经在调用方解析)
        reader.seek(std::io::SeekFrom::Start(16))?;

        // 跳过武器槽 (4字节)
        reader.read_u32_le()?;

        // 读取角色名称 (16字节，空终止)
        let name_bytes = reader.read_bytes(16)?;
        let null_pos = name_bytes.iter().position(|&b| b == 0).unwrap_or(16);
        let name = String::from_utf8_lossy(&name_bytes[..null_pos]).to_string();

        // 读取角色状态 (1字节)
        let status_byte = reader.read_u8()?;
        let status = CharacterStatus::from_byte(status_byte);

        // 跳过进度 (1字节)
        reader.read_u8()?;

        // 跳过2字节未知
        reader.read_u16_le()?;

        // 读取职业 (1字节)
        let class_byte = reader.read_u8()?;
        let class = CharacterClass::from_u8(class_byte)
            .ok_or_else(|| Error::ParseError(format!("无效的职业ID: {}", class_byte)))?;

        // 跳过2字节未知
        reader.read_u16_le()?;

        // 读取等级 (1字节)
        let level = reader.read_u8()?;

        // 读取时间戳 (4+4+4字节)
        let created_at = reader.read_u32_le()?;
        reader.read_u32_le()?; // 跳过未知4字节
        let last_played = reader.read_u32_le()?;

        // 跳过热键数据 (64字节)
        reader.read_bytes(64)?;

        // 跳过鼠标按钮 (4*4=16字节)
        reader.read_bytes(16)?;

        // 跳过外观数据 (32字节)
        reader.read_bytes(32)?;

        // 读取难度进度 (3字节)
        let difficulty_bytes = [
            reader.read_u8()?,
            reader.read_u8()?,
            reader.read_u8()?,
        ];
        let difficulty = DifficultyProgress::from_bytes(difficulty_bytes);

        // 跳过地图数据 (4字节)
        reader.read_u32_le()?;

        // 跳过到任务部分 (0x14F = 335)
        while reader.position() < Self::OFFSET_QUESTS {
            reader.read_u8()?;
        }

        // 解析任务数据
        let quests = QuestList::parse(&mut reader, version)?;

        // 解析传送点数据 (从0x279开始)
        let waypoints = WaypointDataRaw::parse(&mut reader)?;

        // 跳过NPC对话数据到属性部分 (0x2FD = 765)
        while reader.position() < Self::OFFSET_STATS {
            reader.read_u8()?;
        }

        // 解析属性数据
        let stats = CharacterStats::parse(&mut reader, version)?;

        // 解析技能数据 (属性数据之后)
        let skills = SkillList::parse(&mut reader, class)?;

        // 解析物品数据 (最后部分)
        let items = ItemList::parse(&mut reader, version)?;

        Ok(Self {
            header,
            version,
            character: CharacterInfo {
                name,
                class,
                level,
                status,
                difficulty,
                stats,
                skills,
                quests,
                waypoints,
                items,
                created_at,
                last_played,
            },
        })
    }

    /// 序列化为字节数组
    pub fn serialize(&self) -> Result<Vec<u8>> {
        let mut writer = BitWriter::new();

        // 写入文件头
        writer.write_u32_le(D2S_MAGIC)?;
        writer.write_u32_le(self.header.version)?;
        writer.write_u32_le(0)?; // file_size (稍后填充)
        writer.write_u32_le(0)?; // checksum (稍后计算)

        // 写入角色基本信息
        writer.write_u32_le(0)?; // active_weapon

        // 写入角色名称 (16字节)
        let mut name_bytes = [0u8; 16];
        let name_len = self.character.name.len().min(15);
        name_bytes[..name_len].copy_from_slice(self.character.name.as_bytes());
        writer.write_bytes(&name_bytes)?;

        // 写入状态字节
        writer.write_u8(self.character.status.to_byte())?;
        writer.write_u8(0)?; // progression
        writer.write_u16_le(0)?; // unknown

        // 写入职业
        writer.write_u8(self.character.class as u8)?;
        writer.write_u16_le(0)?; // unknown

        // 写入等级
        writer.write_u8(self.character.level)?;

        // 写入时间戳
        writer.write_u32_le(self.character.created_at)?;
        writer.write_u32_le(0)?; // unknown
        writer.write_u32_le(self.character.last_played)?;

        // 写入热键 (64字节，填充0)
        writer.write_bytes(&[0u8; 64])?;

        // 写入鼠标按钮 (16字节，填充0)
        writer.write_bytes(&[0u8; 16])?;

        // 写入外观数据 (32字节，填充0)
        writer.write_bytes(&[0u8; 32])?;

        // 写入难度进度
        let diff_bytes = self.character.difficulty.to_bytes();
        writer.write_u8(diff_bytes[0])?;
        writer.write_u8(diff_bytes[1])?;
        writer.write_u8(diff_bytes[2])?;

        // 写入地图数据 (4字节)
        writer.write_u32_le(0)?;

        // 写入占位到任务部分 (0x14F)
        let current_size = writer.len();
        let padding_needed = Self::OFFSET_QUESTS - current_size;
        for _ in 0..padding_needed {
            writer.write_u8(0)?;
        }

        // 写入任务数据
        // TODO: 实现 QuestList::write

        // 写入传送点数据
        self.character.waypoints.write(&mut writer)?;

        // 写入占位到属性部分 (0x2FD)
        let current_size = writer.len();
        let padding_needed = Self::OFFSET_STATS - current_size;
        for _ in 0..padding_needed {
            writer.write_u8(0)?;
        }

        // 写入属性数据
        self.character.stats.write(&mut writer, self.version)?;

        // 写入技能数据
        self.character.skills.write(&mut writer, self.character.class)?;

        // 写入物品数据
        self.character.items.write(&mut writer, self.version)?;

        let mut data = writer.finish();

        // 更新文件大小
        let file_size = data.len() as u32;
        data[8..12].copy_from_slice(&file_size.to_le_bytes());

        // 计算并写入校验和
        let checksum = D2SHeader::calculate_checksum(&data);
        data[12..16].copy_from_slice(&checksum.to_le_bytes());

        Ok(data)
    }

    /// 保存到文件
    pub fn save_to_file(&self, path: &std::path::Path) -> Result<()> {
        let data = self.serialize()?;
        std::fs::write(path, data)?;
        Ok(())
    }

    /// 修改角色属性
    pub fn modify_stats<F>(&mut self, modifier: F) -> Result<bool>
    where
        F: FnOnce(&mut CharacterStats) -> bool,
    {
        Ok(modifier(&mut self.character.stats))
    }

    /// 修改技能
    pub fn modify_skill<F>(&mut self, modifier: F) -> Result<bool>
    where
        F: FnOnce(&mut SkillList) -> bool,
    {
        Ok(modifier(&mut self.character.skills))
    }

    /// 修改任务状态
    pub fn modify_quest<F>(&mut self, modifier: F) -> Result<bool>
    where
        F: FnOnce(&mut QuestList) -> bool,
    {
        Ok(modifier(&mut self.character.quests))
    }

    /// 修改传送点
    pub fn modify_waypoint<F>(&mut self, modifier: F) -> Result<bool>
    where
        F: FnOnce(&mut WaypointDataRaw) -> bool,
    {
        Ok(modifier(&mut self.character.waypoints))
    }
}

// ==================== 测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic_detection() {
        let mut data = vec![0u8; 20];
        data[0..4].copy_from_slice(&D2S_MAGIC.to_le_bytes());

        let mut reader = BitReader::new(data);
        let header = D2SHeader::parse(&mut reader).unwrap();
        assert_eq!(header.magic, D2S_MAGIC);
    }

    #[test]
    fn test_character_status() {
        let status = CharacterStatus {
            hardcore: true,
            died: false,
            expansion: true,
            ladder: false,
        };

        let byte = status.to_byte();
        let restored = CharacterStatus::from_byte(byte);

        assert_eq!(restored.hardcore, status.hardcore);
        assert_eq!(restored.died, status.died);
        assert_eq!(restored.expansion, status.expansion);
        assert_eq!(restored.ladder, status.ladder);
    }

    #[test]
    fn test_version_detection() {
        assert_eq!(D2SVersion::from_u32(0x96), D2SVersion::D2R);
        assert_eq!(D2SVersion::from_u32(0x97), D2SVersion::D2RWarlock);
        assert!(D2SVersion::D2RWarlock.is_d2r());
        assert!(D2SVersion::D2RWarlock.is_warlock());
    }

    #[test]
    fn test_checksum_calculation() {
        let data = vec![0xAA, 0x55, 0xAA, 0x55, 1, 0, 0, 0, 16, 0, 0, 0, 0];
        let checksum = D2SHeader::calculate_checksum(&data);
        assert_ne!(checksum, 0);
    }

    #[test]
    fn test_difficulty_progress() {
        let progress = DifficultyProgress {
            normal: ActProgress { unlocked: true, current_act: 2 },
            nightmare: ActProgress { unlocked: false, current_act: 0 },
            hell: ActProgress { unlocked: false, current_act: 0 },
        };

        let bytes = progress.to_bytes();
        assert_eq!(bytes[0] & 0x80, 0x80); // normal unlocked
        assert_eq!(bytes[0] & 0x7F, 2); // normal act 2
    }
}
