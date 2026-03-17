// Copyright 2025 zl. All rights reserved.

//! D2S 文件十六进制分析

fn main() {
    let path = "doc/杜仲.d2s";
    let data = std::fs::read(path).expect("无法读取存档文件");

    println!("=== D2S 文件十六进制分析 ===\n");

    // 显示前0x40 (64)字节的详细分析
    println!("--- 文件头 (0x00-0x3F) ---");
    for i in (0..0x40).step_by(16) {
        print!("{:04X}: ", i);
        for j in 0..16 {
            if i + j < data.len() {
                print!("{:02X} ", data[i + j]);
            } else {
                print!("   ");
            }
        }
        print!(" | ");
        for j in 0..16 {
            if i + j < data.len() {
                let b = data[i + j];
                if b >= 32 && b <= 126 {
                    print!("{}", b as char);
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }

    println!("\n--- 字段分析 ---");

    // 魔数 (0x00-0x03)
    let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
    println!("魔数 (0x00-0x03): 0x{:08X} {}",
        magic, if magic == 0xAA55AA55 { "✓" } else { "✗" });

    // 版本 (0x04-0x07)
    let version = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
    println!("版本 (0x04-0x07): 0x{:04X} ({})", version,
        match version {
            0x60 => "D2 1.00+",
            0x61 => "D2 1.03-1.06",
            0x62 => "D2 1.07+",
            0x64 => "D2 1.09+",
            0x69 => "D2 1.10+ (经典)",
            0x96 => "D2R",
            0x97 => "术士君临",
            _ => "未知",
        });

    // 文件大小 (0x08-0x0B)
    let file_size = u32::from_le_bytes([data[8], data[9], data[10], data[11]]);
    println!("文件大小 (0x08-0x0B): {} 字节", file_size);

    // 校验和 (0x0C-0x0F)
    let checksum = u32::from_le_bytes([data[12], data[13], data[14], data[15]]);
    println!("校验和 (0x0C-0x0F): 0x{:08X}", checksum);

    // 状态标志 (0x10-0x13)
    let status = u32::from_le_bytes([data[16], data[17], data[18], data[19]]);
    println!("状态标志 (0x10-0x13): 0x{:08X}", status);
    println!("  - 已Hardcore: {}", (status & 4) != 0);
    println!("  - 已死亡: {}", (status & 8) != 0);

    // 名字长度 (0x14) - D2格式
    println!("\n--- 名字字段 ---");
    let name_len = data[0x14] as usize;
    println!("名字长度 (0x14): {} 字节", name_len);

    // 名字数据 (从 0x15 开始)
    let name_start = 0x15;
    let name_end = name_start + name_len;
    if name_end <= data.len() {
        let name_bytes = &data[name_start..name_end];
        let name = String::from_utf8_lossy(name_bytes);
        println!("名字 (0x15-0x{:02X}): '{}'", name_end - 1, name);
        println!("  原始字节: {:?}", name_bytes);
    }

    // 头部其余部分
    let header_end = name_end;

    // 职业和等级
    if header_end < data.len() {
        println!("\n--- 职业和等级 ---");
        println!("职业 (0x{:02X}): 0x{:02X} ({})",
            header_end, data[header_end],
            match data[header_end] {
                0 => "亚马逊",
                1 => "法师",
                2 => "死灵法师",
                3 => "圣骑士",
                4 => "野蛮人",
                5 => "德鲁伊",
                6 => "刺客",
                0xFF => "未设置(新角色)",
                _ => "未知",
            });

        println!("等级 (0x{:02X}): {}", header_end + 1, data[header_end + 1]);
    }

    // 显示更多数据
    println!("\n--- 后续数据 (0x30-0x80) ---");
    for i in (0x30..0x80).step_by(16) {
        print!("{:04X}: ", i);
        for j in 0..16 {
            if i + j < data.len() {
                print!("{:02X} ", data[i + j]);
            } else {
                print!("   ");
            }
        }
        print!(" | ");
        for j in 0..16 {
            if i + j < data.len() {
                let b = data[i + j];
                if b >= 32 && b <= 126 {
                    print!("{}", b as char);
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }

    // 检查已知标记位置
    println!("\n--- 检查已知数据位置 ---");
    println!("技能位置 (0x2FD): 0x{:02X} 0x{:02X}", data[0x2FD], data[0x2FE]);
    println!("传送点位置 (0x279): 0x{:02X} 0x{:02X}", data[0x279], data[0x27A]);
}
