// Copyright 2025 zl. All rights reserved.

//! 完整的 D2S 文件解析测试

use diablo_tools_tauri::core::bit_reader::BitReader;

fn main() {
    let path = "doc/杜仲.d2s";

    println!("=== D2S 完整解析测试 ===");
    println!("文件: {}\n", path);

    // 读取文件
    let data = std::fs::read(path).expect("无法读取存档文件");
    println!("文件大小: {} 字节\n", data.len());

    let mut reader = BitReader::new(data);

    // 1. 解析文件头 (16字节)
    println!("--- 文件头 ---");
    let magic = reader.read_u32_le().expect("读取魔数失败");
    println!("魔数: 0x{:08X} {}", magic,
        if magic == 0xAA55AA55 { "✓" } else { "✗" });

    let version = reader.read_u32_le().expect("读取版本失败");
    println!("版本: 0x{:04X} ({})", version,
        match version {
            0x60 => "经典版 1.00+",
            0x69 => "经典版",
            0x96 => "D2R",
            0x97 => "术士君临",
            _ => "未知",
        });

    let file_size = reader.read_u32_le().expect("读取文件大小失败");
    println!("文件大小: {} 字节", file_size);

    let checksum = reader.read_u32_le().expect("读取校验和失败");
    println!("校验和: 0x{:08X}\n", checksum);

    // 2. 角色状态 (4字节)
    println!("--- 角色状态 ---");
    let status = reader.read_u32_le().expect("读取状态失败");
    println!("状态: 0x{:08X}", status);
    println!("  - 已 Hardcore: {}", (status & 0x04) != 0);
    println!("  - 已死亡: {}", (status & 0x08) != 0);
    println!("  - 已创建难度: 噩梦={} 地狱={}",
        (status & 0x10) != 0,
        (status & 0x20) != 0);

    // 3. 角色名
    println!("\n--- 角色信息 ---");
    reader.align_to_byte();
    let name_bytes = reader.read_bytes(16).expect("读取角色名失败");
    let name_len = name_bytes.iter().position(|&b| b == 0).unwrap_or(16);
    let name = String::from_utf8_lossy(&name_bytes[..name_len]);
    println!("角色名: {}", name);

    // 4. 职业
    let class = reader.read_u8().expect("读取职业失败");
    println!("职业: {} (0x{:02X})",
        match class {
            0 => "亚马逊",
            1 => "法师",
            2 => "死灵法师",
            3 => "圣骑士",
            4 => "野蛮人",
            5 => "德鲁伊",
            6 => "刺客",
            _ => "未知",
        }, class);

    // 5. 显示的等级
    let shown_level = reader.read_u8().expect("读取等级失败");
    println!("等级: {}", shown_level);

    // 6. 时间戳
    reader.read_u32_le().expect("读取时间戳失败"); // 跳过

    // 7. 未知数据
    let unknown = reader.read_u32_le().expect("读取未知数据失败");
    println!("未知标志: 0x{:08X}", unknown);

    // 8. 属性数据开始位置
    let checksum_offset = reader.read_u32_le().expect("读取校验和偏移失败") as usize;
    println!("校验和偏移: 0x{:04X} ({})", checksum_offset, checksum_offset);

    // 跳到属性数据 (0x26 = 38)
    println!("\n--- 属性数据 (偏移 0x26) ---");
    // 简单地显示接下来的字节
    let current_pos = reader.position();
    println!("当前位置: 0x{:04X}", current_pos);

    // 尝试读取一些属性值
    // 经典版使用不同的属性存储格式
    // 这里只做基本验证

    // 检查技能数据位置 (0x2FD = 764)
    println!("\n--- 检查技能数据位置 ---");
    // 跳到技能位置
    let _ = std::io::Seek::seek(&mut reader, std::io::SeekFrom::Start(0x2FD));
    let skill_marker = reader.read_bytes(2).expect("读取技能标记失败");
    println!("技能标记: {:02X} {:02X} ({:?})",
        skill_marker[0], skill_marker[1], skill_marker);

    // 检查传送点位置 (0x279 = 633)
    println!("\n--- 检查传送点位置 ---");
    let _ = std::io::Seek::seek(&mut reader, std::io::SeekFrom::Start(0x279));
    let wp_marker = reader.read_bytes(2).expect("读取传送点标记失败");
    println!("传送点标记: {:02X} {:02X} ({:?})",
        wp_marker[0], wp_marker[1], wp_marker);

    // 检查任务位置 (0x14F = 335)
    println!("\n--- 检查任务位置 ---");
    let _ = std::io::Seek::seek(&mut reader, std::io::SeekFrom::Start(0x14F));
    // 任务数据通常没有特殊标记
    println!("任务区域从偏移 0x14F 开始");

    println!("\n✓ 基本解析完成！");
    println!("\n注意: 这是经典版D2存档，与D2R格式有差异。");
    println!("经典版不需要Huffman编码，属性/技能格式也不同。");
}
