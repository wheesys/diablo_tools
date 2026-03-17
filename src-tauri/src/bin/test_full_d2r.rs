// Copyright 2025 zl. All rights reserved.

//! 完整的 D2R 文件解析测试

use diablo_tools_tauri::core::d2s::D2SFile;

fn main() {
    let path = "../doc/杜仲.d2s";

    println!("=== D2R 完整解析测试 ===");
    println!("文件: {}\n", path);

    // 读取文件
    let data = std::fs::read(path).expect("无法读取存档文件");
    println!("文件大小: {} 字节\n", data.len());

    // 解析文件
    match D2SFile::parse(&data) {
        Ok(d2s) => {
            println!("✓ 解析成功!\n");

            println!("--- 基本信息 ---");
            println!("版本: {:?}", d2s.version);
            println!("角色名: {}", d2s.character.name);
            println!("职业: {}", d2s.character.class_name());
            println!("等级: {}", d2s.character.level);

            println!("\n--- 属性 ---");
            println!("力量: {}", d2s.character.stats.strength);
            println!("敏捷: {}", d2s.character.stats.dexterity);
            println!("体力: {}", d2s.character.stats.vitality);
            println!("能量: {}", d2s.character.stats.energy);

            println!("\n--- 生命/法力 ---");
            println!("生命: {} / {}",
                d2s.character.stats.display_hp(),
                d2s.character.stats.display_max_hp());
            println!("法力: {} / {}",
                d2s.character.stats.display_mana(),
                d2s.character.stats.display_max_mana());

            println!("\n--- 技能数量 ---");
            println!("普通难度: {}", d2s.character.skills.normal.len());
            println!("噩梦难度: {}", d2s.character.skills.nightmare.len());
            println!("地狱难度: {}", d2s.character.skills.hell.len());

            println!("\n✓ D2R 文件解析完成!");
        }
        Err(e) => {
            println!("✗ 解析失败: {:?}\n", e);
        }
    }
}
