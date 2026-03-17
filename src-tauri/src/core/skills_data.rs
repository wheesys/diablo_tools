// Copyright 2025 zl. All rights reserved.

//! 技能数据解析 - 位于偏移 0x2FD

use super::bit_reader::BitReader;
use super::bit_writer::BitWriter;
use super::error::{Error, Result};
use super::skills::CharacterClass;
use serde::{Deserialize, Serialize};

/// 技能数据
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillData {
    pub id: u16,
    pub level: u8,
}

impl SkillData {
    pub fn new(id: u16) -> Self {
        Self { id, level: 0 }
    }
}

/// 技能列表 - 按难度组织
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillList {
    pub normal: Vec<SkillData>,
    pub nightmare: Vec<SkillData>,
    pub hell: Vec<SkillData>,
}

impl SkillList {
    /// 解析技能数据
    ///
    /// 技能数据结构 (D2R):
    /// - 每个技能用一个字节表示等级 (0-99，255表示未学习)
    /// - 技能按ID顺序排列，各职业有不同数量的技能
    pub fn parse(reader: &mut BitReader, class: CharacterClass) -> Result<Self> {
        let skill_count = class.skill_count();

        // 读取普通难度技能
        let mut normal = Vec::with_capacity(skill_count);
        for id in 0..skill_count as u16 {
            let level = reader.read_u8()?;
            if level != 255 && level > 0 {
                normal.push(SkillData { id, level });
            } else {
                normal.push(SkillData { id, level: 0 });
            }
        }

        // 读取噩梦难度技能
        let mut nightmare = Vec::with_capacity(skill_count);
        for id in 0..skill_count as u16 {
            let level = reader.read_u8()?;
            if level != 255 && level > 0 {
                nightmare.push(SkillData { id, level });
            } else {
                nightmare.push(SkillData { id, level: 0 });
            }
        }

        // 读取地狱难度技能
        let mut hell = Vec::with_capacity(skill_count);
        for id in 0..skill_count as u16 {
            let level = reader.read_u8()?;
            if level != 255 && level > 0 {
                hell.push(SkillData { id, level });
            } else {
                hell.push(SkillData { id, level: 0 });
            }
        }

        Ok(Self {
            normal,
            nightmare,
            hell,
        })
    }

    /// 写入技能数据
    pub fn write(&self, writer: &mut BitWriter, class: CharacterClass) -> Result<()> {
        let skill_count = class.skill_count();

        // 写入普通难度技能
        for skill in &self.normal {
            if skill.level > 0 {
                writer.write_u8(skill.level)?;
            } else {
                writer.write_u8(255)?;
            }
        }
        // 补齐到技能数量
        for _ in self.normal.len()..skill_count {
            writer.write_u8(255)?;
        }

        // 写入噩梦难度技能
        for skill in &self.nightmare {
            if skill.level > 0 {
                writer.write_u8(skill.level)?;
            } else {
                writer.write_u8(255)?;
            }
        }
        for _ in self.nightmare.len()..skill_count {
            writer.write_u8(255)?;
        }

        // 写入地狱难度技能
        for skill in &self.hell {
            if skill.level > 0 {
                writer.write_u8(skill.level)?;
            } else {
                writer.write_u8(255)?;
            }
        }
        for _ in self.hell.len()..skill_count {
            writer.write_u8(255)?;
        }

        Ok(())
    }

    /// 获取指定难度的技能
    pub fn get_difficulty_skills(&self, difficulty: super::quests::Difficulty) -> &[SkillData] {
        match difficulty {
            super::quests::Difficulty::Normal => &self.normal,
            super::quests::Difficulty::Nightmare => &self.nightmare,
            super::quests::Difficulty::Hell => &self.hell,
        }
    }

    /// 设置技能等级
    pub fn set_skill_level(&mut self, difficulty: super::quests::Difficulty, skill_id: u16, level: u8) {
        let skills = match difficulty {
            super::quests::Difficulty::Normal => &mut self.normal,
            super::quests::Difficulty::Nightmare => &mut self.nightmare,
            super::quests::Difficulty::Hell => &mut self.hell,
        };

        if let Some(skill) = skills.get_mut(skill_id as usize) {
            skill.level = level;
        }
    }
}

impl Default for SkillList {
    fn default() -> Self {
        Self {
            normal: Vec::new(),
            nightmare: Vec::new(),
            hell: Vec::new(),
        }
    }
}

// 技能ID常量定义
pub mod skill_ids {
    /// 亚马逊技能
    pub mod amazon {
        pub const MAGIC_ARROW: u16 = 0;
        pub const FIRE_ARROW: u16 = 1;
        pub const INNER_SIGHT: u16 = 2;
        pub const CRITICAL_STRIKE: u16 = 3;
        pub const DODGE: u16 = 4;
        pub const SLOW_MISSILES: u16 = 5;
        pub const AVOID: u16 = 6;
        pub const PENETRATE: u16 = 7;
        pub const POWER_STRIKE: u16 = 8;
        pub const LIGHTNING_BOLT: u16 = 9;
        pub const CHARGED_STRIKE: u16 = 10;
        pub const PLAGUE_JAVELIN: u16 = 11;
        pub const IMPALE: u16 = 12;
        pub const LIGHTNING_FURY: u16 = 13;
        pub const LIGHTNING_STRIKE: u16 = 14;
        pub const FADE: u16 = 15;
        pub const VALKYRIE: u16 = 16;
        pub const DECAY: u16 = 17;
        pub const LIGHTNING_STRIKE_SYNERGY: u16 = 18;
        pub const EXPLOSIVE_ARROW: u16 = 19;
    }

    /// 法师技能
    pub mod sorceress {
        pub const FIRE_BOLT: u16 = 0;
        pub const WARMTH: u16 = 1;
        pub const CHARGED_BOLT: u16 = 2;
        pub const ICE_BOLT: u16 = 3;
        pub const FROST_NOVA: u16 = 4;
        pub const SHIVER_ARMOR: u16 = 5;
        pub const INFERNO: u16 = 6;
        pub const BLAZE: u16 = 7;
        pub const FIRE_BALL: u16 = 8;
        pub const TELEKINESIS: u16 = 9;
        pub const FROST_NOVA_SYNERGY: u16 = 10;
        pub const ICE_BLAST: u16 = 11;
        pub const BLAZE_SYNERGY: u16 = 12;
        pub const FIRE_WALL: u16 = 13;
        pub const ENCHANT: u16 = 14;
        pub const LIGHTNING: u16 = 15;
        pub const CHAIN_LIGHTNING: u16 = 16;
        pub const TELEPORT: u16 = 17;
        pub const GLACIAL_SPIKE: u16 = 18;
        pub const METEOR: u16 = 19;
        pub const THUNDER_STORM: u16 = 20;
        pub const ENERGY_SHIELD: u16 = 21;
        pub const BLIZZARD: u16 = 22;
        pub const HYDRA: u16 = 23;
        pub const FIRE_MASTERY: u16 = 24;
        pub const LIGHTNING_MASTERY: u16 = 25;
        pub const COLD_MASTERY: u16 = 26;
        pub const FIRE_SYNERGY: u16 = 27;
        pub const LIGHTNING_SYNERGY: u16 = 28;
        pub const COLD_SYNERGY: u16 = 29;
    }

    /// 死灵法师技能
    pub mod necromancer {
        pub const AMP_DMG: u16 = 0;
        pub const TEETH: u16 = 1;
        pub const BONE_ARMOR: u16 = 2;
        pub const SKELETON_MASTERY: u16 = 3;
        pub const SKELETON: u16 = 4;
        pub const CLAY_GOLEM: u16 = 5;
        pub const DECREPIFY: u16 = 6;
        pub const CURSE: u16 = 7;
        pub const REVIVE: u16 = 8;
        pub const DIM_VISION: u16 = 9;
        pub const BONE_WALL: u16 = 10;
        pub const GOLEM_MASTERY: u16 = 11;
        pub const BLOOD_GOLEM: u16 = 12;
        pub const IRON_MAIDEN: u16 = 13;
        pub const TERROR: u16 = 14;
        pub const BONE_SPIRIT: u16 = 15;
        pub const ATTRACT: u16 = 16;
        pub const CONFUSE: u16 = 17;
        pub const LIFE_TAP: u16 = 18;
        pub const BONESPEAR: u16 = 19;
        pub const LOWER_RES: u16 = 20;
        pub const POISON_DAGGER: u16 = 21;
        pub const CORPSE_EXPLOSION: u16 = 22;
        pub const BONE_PRISON: u16 = 23;
        pub const FIRE_GOLEM: u16 = 24;
        pub const POISON_EXPLOSION: u16 = 25;
        pub const POISON_NOVA: u16 = 26;
    }

    /// 圣骑士技能
    pub mod paladin {
        pub const SACRED_HAMMER: u16 = 0;
        pub const HOLY_BOLT: u16 = 1;
        pub const SMITE: u16 = 2;
        pub const MIGHT: u16 = 3;
        pub const THORNS: u16 = 4;
        pub const DEFERENCE: u16 = 5;
        pub const HOLY_FIRE: u16 = 6;
        pub const BLESSING: u16 = 7;
        pub const ZEAL: u16 = 8;
        pub const CHARGE: u16 = 9;
        pub const HOLY_SHOCK: u16 = 10;
        pub const SALVATION: u16 = 11;
        pub const VENGEANCE: u16 = 12;
        pub const BLESSED_HAMMER: u16 = 13;
        pub const CONVICTION: u16 = 14;
        pub const HOLY_SHIELD: u16 = 15;
        pub const FOH: u16 = 16;
        pub const REDEMPTION: u16 = 17;
        pub const FIST_OF_THE_HEAVENS: u16 = 18;
        pub const MEDITATION: u16 = 19;
    }

    /// 野蛮人技能
    pub mod barbarian {
        pub const BASH: u16 = 0;
        pub const SWORD_MASTERY: u16 = 1;
        pub const AXE_MASTERY: u16 = 2;
        pub const MACE_MASTERY: u16 = 3;
        pub const POLE_MASTERY: u16 = 4;
        pub const THROW_MASTERY: u16 = 5;
        pub const SPEAR_MASTERY: u16 = 6;
        pub const FIND_ITEM: u16 = 7;
        pub const HOWL: u16 = 8;
        pub const TAUNT: u16 = 9;
        pub const SHOUT: u16 = 10;
        pub const STUN: u16 = 11;
        pub const WHIRLWIND: u16 = 12;
        pub const BATTLE_CRY: u16 = 13;
        pub const WARCRY: u16 = 14;
        pub const FIND_POTION: u16 = 15;
        pub const LEAP: u16 = 16;
        pub const BATTLE_ORDERS: u16 = 17;
        pub const GRIM_WARD: u16 = 18;
        pub const NATURAL_RES: u16 = 19;
        pub const BERSERK: u16 = 20;
        pub const CONCENTRATE: u16 = 21;
        pub const IRON_SKIN: u16 = 22;
        pub const INCREASED_STAMINA: u16 = 23;
        pub const INCREASED_SPEED: u16 = 24;
        pub const BATTLE_COMMAND: u16 = 25;
        pub const WARCRY_BERSERK: u16 = 26;
        pub const GRIM_WARD_SYNERGY: u16 = 27;
        pub const INCREASED_SPEED_SYNERGY: u16 = 28;
        pub const FIND_ITEM_SYNERGY: u16 = 29;
    }

    /// 德鲁伊技能
    pub mod druid {
        pub const RAVEN: u16 = 0;
        pub const STORM_REACH: u16 = 1;
        pub const WEREWOLF: u16 = 2;
        pub const LYCANTHROPY: u16 = 3;
        pub const FIRESTORM: u16 = 4;
        pub const OAK_SAGE: u16 = 5;
        pub const SPIRIT_WOLF: u16 = 6;
        pub const WEREBEAR: u16 = 7;
        pub const MOLLEN_BORE: u16 = 8;
        pub const FISSURE: u16 = 9;
        pub const CYCLONE_ARMOR: u16 = 10;
        pub const GRIZZLY: u16 = 11;
        pub const TWISTER: u16 = 12;
        pub const Tornado: u16 = 13;
        pub const HURRICANE: u16 = 14;
        pub const CREEPING_PAIN: u16 = 15;
        pub const VOLCANO: u16 = 16;
        pub const ARCTIC_BLAST: u16 = 17;
        pub const FIRE_CLAWS: u16 = 18;
        pub const FERAL_RAGE: u16 = 19;
        pub const MOLTON_BORE: u16 = 20;
        pub const FIRESTORM_SYNERGY: u16 = 21;
        pub const CYCLONE_ARMOR_SYNERGY: u16 = 22;
        pub const TORNADO_SYNERGY: u16 = 23;
        pub const FISSURE_SYNERGY: u16 = 24;
        pub const VOLCANO_SYNERGY: u16 = 25;
        pub const HURRICANE_ARMOR_SYNERGY: u16 = 26;
        pub const GRIZZLY_SYNERGY: u16 = 27;
        pub const CARRION_VINE: u16 = 28;
        pub const SOLAR_CREEP: u16 = 29;
    }

    /// 刺客技能
    pub mod assassin {
        pub const FIRE_TRAITS: u16 = 0;
        pub const CLAW_MASTERY: u16 = 1;
        pub const PSYCHIC_HAMMER: u16 = 2;
        pub const TIGER_STRIKE: u16 = 3;
        pub const DRAGON_TAIL: u16 = 4;
        pub const CLAW: u16 = 5;
        pub const Cobra: u16 = 6;
        pub const Phoenix: u16 = 7;
        pub const BladeShield: u16 = 8;
        pub const WeaponBlock: u16 = 9;
        pub const CloakOfShadows: u16 = 10;
        pub const MindBlast: u16 = 11;
        pub const Fade: u16 = 12;
        pub const ShadowMaster: u16 = 13;
        pub const BladeSentinel: u16 = 14;
        pub const ShockField: u16 = 15;
        pub const ChargedBoltSentry: u16 = 16;
        pub const WakeOfFire: u16 = 17;
        pub const DeathSentry: u16 = 18;
        pub const BladeFury: u16 = 19;
        pub const ShadowWarrior: u16 = 20;
        pub const PhoenixStrike: u16 = 21;
        pub const LightningSentry: u16 = 22;
        pub const FistOfFire: u16 = 23;
        pub const ClawsOfThunder: u16 = 24;
        pub const CoT: u16 = 25;
        pub const BoS: u16 = 26;
        pub const FadeSynergy: u16 = 27;
        pub const VenomSynergy: u16 = 28;
        pub const FireBlast: u16 = 29;
    }

    /// 术士技能 (术士君临新增)
    pub mod warlock {
        pub const SOUL_SPIKE: u16 = 0;        // 灵魂尖刺
        pub const DARK_BARGAIN: u16 = 1;      // 黑暗契约
        pub const SHADOW_BIND: u16 = 2;       // 暗影束缚
        pub const BONE_ARMOR: u16 = 3;        // 骨甲 (与死灵法师共享)
        pub const CURSE_OF_WEAKNESS: u16 = 4; // 虚弱诅咒
        pub const SOUL_HARVEST: u16 = 5;      // 灵魂收割
        pub const HELLFIRE: u16 = 6;          // 地狱火
        pub const DEMON_PACT: u16 = 7;        // 恶魔契约
        pub const DARK_RITUAL: u16 = 8;       // 黑暗仪式
        pub const SOUL_WARD: u16 = 9;         // 灵魂守护
        pub const CHAOS_BOLT: u16 = 10;       // 混沌箭
        pub const NETHER_PROTECTION: u16 = 11; // 冥界保护
        pub const APOCALYPSE: u16 = 12;       // 天启
        pub const DEMON_SUMMON: u16 = 13;     // 恶魔召唤
        pub const DARK_MENDING: u16 = 14;     // 黑暗治疗
        pub const SOUL_BURN: u16 = 15;        // 灵魂燃烧
        pub const HEX: u16 = 16;              // 妖术
        pub const SHADOW_MASTERY: u16 = 17;   // 暗影精通
        pub const HELLFIRE_SYNERGY: u16 = 18;
        pub const APOCALYPSE_SYNERGY: u16 = 19;
        pub const DEMON_PACT_SYNERGY: u16 = 20;
        pub const CHAOS_MASTERY: u16 = 21;
        pub const SOUL_SIPHON: u16 = 22;      // 灵魂虹吸
        pub const DARK_BLAST: u16 = 23;       // 黑暗冲击
        pub const NETHER_WARD: u16 = 24;      // 冥界守护
        pub const CURSE_OF_AGONY: u16 = 25;   // 痛苦诅咒
        pub const SUMMON_DEMON: u16 = 26;     // 召唤恶魔
        pub const VAMPIRIC_TOUCH: u16 = 27;   // 吸血鬼之触
        pub const DOOM: u16 = 28;              // 末日
        pub const DARK_MENDING_SYNERGY: u16 = 29;
        pub const SHADOW_BLAST: u16 = 30;      // 暗影冲击
        pub const HELLSTORM: u16 = 31;         // 地狱风暴
        pub const UNHOLY_PACT: u16 = 32;      // 邪恶契约
        pub const MASTER_OF_DEATH: u16 = 33;  // 死亡大师
        pub const SOUL_EXCHANGE: u16 = 34;    // 灵魂交换
    }
}

/// 获取技能中文名称
pub fn get_skill_name(class: CharacterClass, skill_id: u16) -> &'static str {
        match class {
            CharacterClass::Amazon => get_amazon_skill_name(skill_id),
            CharacterClass::Sorceress => get_sorceress_skill_name(skill_id),
            CharacterClass::Necromancer => get_necromancer_skill_name(skill_id),
            CharacterClass::Paladin => get_paladin_skill_name(skill_id),
            CharacterClass::Barbarian => get_barbarian_skill_name(skill_id),
            CharacterClass::Druid => get_druid_skill_name(skill_id),
            CharacterClass::Assassin => get_assassin_skill_name(skill_id),
            CharacterClass::Warlock => get_warlock_skill_name(skill_id),
        }
    }

    fn get_amazon_skill_name(id: u16) -> &'static str {
        match id {
            0 => "魔法箭",
            1 => "火焰箭",
            2 => "内视",
            3 => "致命攻击",
            4 => "闪避",
            5 => "减速导弹",
            6 => "回避",
            7 => "穿透",
            8 => "强力打击",
            9 => "闪电",
            10 => "充能一击",
            11 => "瘟疫标枪",
            12 => "刺入",
            13 => "闪电之怒",
            14 => "闪电打击",
            15 => "消逝",
            16 => "女武神",
            17 => "腐蚀",
            18 => "闪电打击(协同)",
            19 => "爆炸箭",
            _ => "未知技能",
        }
    }

    fn get_sorceress_skill_name(id: u16) -> &'static str {
        match id {
            0 => "火弹",
            1 => "温暖",
            2 => "充能弹",
            3 => "冰弹",
            4 => "冰霜新星",
            5 => "碎冰甲",
            6 => "地狱火",
            7 => "烈焰之径",
            8 => "火球",
            9 => "念动力",
            10 => "冰霜新星(协同)",
            11 => "冰封球",
            12 => "烈焰之径(协同)",
            13 => "火墙",
            14 => "附魔",
            15 => "闪电",
            16 => "连锁闪电",
            17 => "传送",
            18 => "冰尖柱",
            19 => "陨石",
            20 => "雷暴",
            21 => "能量盾",
            22 => "暴风雪",
            23 => "九头蛇",
            24 => "火焰精通",
            25 => "闪电精通",
            26 => "冰霜精通",
            27 => "火系协同",
            28 => "电系协同",
            29 => "冰系协同",
            _ => "未知技能",
        }
    }

    fn get_necromancer_skill_name(id: u16) -> &'static str {
        match id {
            0 => "伤害加深",
            1 => "尸骨",
            2 => "骨甲",
            3 => "骷髅掌控",
            4 => "骷髅召唤",
            5 => "粘土石魔",
            6 => "衰老",
            7 => "攻击性魔咒",
            8 => "复活",
            9 => "黯淡",
            10 => "骨墙",
            11 => "石魔精通",
            12 => "鲜血石魔",
            13 => "钢铁处女",
            14 => "恐惧",
            15 => "骨魂",
            16 => "吸引",
            17 => "迷惑",
            18 => "偷取生命",
            19 => "骨矛",
            20 => "降低抗性",
            21 => "毒 dagger",
            22 => "尸体爆炸",
            23 => "骨牢",
            24 => "火焰石魔",
            25 => "毒爆",
            26 => "毒新星",
            _ => "未知技能",
        }
    }

    fn get_paladin_skill_name(id: u16) -> &'static str {
        match id {
            0 => "祝福之锤",
            1 => "圣弹",
            2 => "重击",
            3 => "力量",
            4 => "荆棘",
            5 => "抗性",
            6 => "圣火",
            7 => "祝福",
            8 => "热诚",
            9 => "冲锋",
            10 => "圣电",
            11 => "救赎",
            12 => "复仇",
            13 => "神圣之锤",
            14 => "信念",
            15 => "圣盾",
            16 => "天堂之拳",
            17 => "赎罪",
            18 => "天堂之怒",
            19 => "冥想",
            _ => "未知技能",
        }
    }

    fn get_barbarian_skill_name(id: u16) -> &'static str {
        match id {
            0 => "重击",
            1 => "剑掌握",
            2 => "斧掌握",
            3 => "锤掌握",
            4 => "长矛掌握",
            5 => "投掷掌握",
            6 => "矛掌握",
            7 => "寻找物品",
            8 => "吼叫",
            9 => "嘲弄",
            10 => "大叫",
            11 => "击晕",
            12 => "旋风斩",
            13 => "战斗吼叫",
            14 => "战吼",
            15 => "寻找药水",
            16 => "跳跃",
            17 => "大叫",
            18 => " grim ward",
            19 => "自然抗性",
            20 => "狂暴",
            21 => "专注",
            22 => "铁皮",
            23 => "增加耐力",
            24 => "加速",
            25 => "战斗命令",
            26 => "战吼-狂暴",
            27 => "grim ward 协同",
            28 => "加速 协同",
            29 => "寻找物品 协同",
            _ => "未知技能",
        }
    }

    fn get_druid_skill_name(id: u16) -> &'static str {
        match id {
            0 => "乌鸦",
            1 => "风暴 reach",
            2 => "狼人变身",
            3 => "变形",
            4 => "火风暴",
            5 => "橡木智者",
            6 => "狼灵召唤",
            7 => "熊人变身",
            8 => "猛毒 strikes",
            9 => "裂地",
            10 => "旋风甲",
            11 => "灰熊",
            12 => "龙卷风",
            13 => "飓风",
            14 => "飓风甲",
            15 => "折磨",
            16 => "火山",
            17 => "极地风暴",
            18 => "火爪",
            19 => "野性狂暴",
            20 => "熔岩 borer 协同",
            21 => "旋风甲 协同",
            22 => "龙卷风 协同",
            23 => "裂地 协同",
            24 => "火山 协同",
            25 => "飓风甲 协同",
            26 => "灰熊 协同",
            27 => "食尸藤",
            28 => "太阳藤蔓",
            _ => "未知技能",
        }
    }

    fn get_assassin_skill_name(id: u16) -> &'static str {
        match id {
            0 => "火掌",
            1 => "爪掌握",
            2 => "心灵锤",
            3 => "虎击",
            4 => "龙尾",
            5 => "爪",
            6 => "眼镜蛇",
            7 => "凤凰",
            8 => "刀盾",
            9 => "武器格挡",
            10 => "暗影斗篷",
            11 => "心灵爆破",
            12 => "消逝",
            13 => "暗影大师",
            14 => "刀锋守卫",
            15 => "震荡场",
            16 => "闪电守卫",
            17 => "火焰苏醒",
            18 => "死亡守卫",
            19 => "刀锋风暴",
            20 => "暗影战士",
            21 => "凤凰击",
            22 => "闪电守卫",
            23 => "火焰拳",
            24 => "雷光之拳",
            25 => "能量",
            26 => "速度",
            27 => "消逝 协同",
            28 => "剧毒 协同",
            29 => "火焰爆炸",
            _ => "未知技能",
        }
    }

    fn get_warlock_skill_name(id: u16) -> &'static str {
        match id {
            0 => "灵魂尖刺",
            1 => "黑暗契约",
            2 => "暗影束缚",
            3 => "骨甲",
            4 => "虚弱诅咒",
            5 => "灵魂收割",
            6 => "地狱火",
            7 => "恶魔契约",
            8 => "黑暗仪式",
            9 => "灵魂守护",
            10 => "混沌箭",
            11 => "冥界保护",
            12 => "天启",
            13 => "恶魔召唤",
            14 => "黑暗治疗",
            15 => "灵魂燃烧",
            16 => "妖术",
            17 => "暗影精通",
            18 => "地狱火 协同",
            19 => "天启 协同",
            20 => "恶魔契约 协同",
            21 => "混沌精通",
            22 => "灵魂虹吸",
            23 => "黑暗冲击",
            24 => "冥界守护",
            25 => "痛苦诅咒",
            26 => "召唤恶魔",
            27 => "吸血鬼之触",
            28 => "末日",
            29 => "黑暗治疗 协同",
            30 => "暗影冲击",
            31 => "地狱风暴",
            32 => "邪恶契约",
            33 => "死亡大师",
            34 => "灵魂交换",
            _ => "未知技能",
        }
    }
