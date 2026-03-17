// Copyright 2025 zl. All rights reserved.

//! D2S 文件结构详细分析

fn main() {
    let path = "doc/杜仲.d2s";
    let data = std::fs::read(path).expect("无法读取存档文件");

    println!("=== D2S 文件结构分析 ===\n");

    // 这个文件似乎有非标准结构
    // 让我们完整显示文件的十六进制

    for i in (0..data.len()).step_by(16) {
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
                // UTF-8 decode attempt
                if b >= 32 && b <= 126 {
                    print!("{}", b as char);
                } else if b >= 0x80 {
                    print!(".");
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }

    println!("\n=== 关键发现 ===");

    // 查找名字 "杜仲"
    println!("\n查找 UTF-8 编码的中文名:");
    let mut pos = 0;
    while pos < data.len() - 3 {
        // 检查 UTF-8 中文字符
        if data[pos] >= 0xE0 && data[pos] <= 0xEF {
            // 可能是 3字节 UTF-8
            if pos + 2 < data.len() {
                let s = String::from_utf8_lossy(&data[pos..pos+3]);
                if !s.chars().all(|c| c == '\u{FFFD}') {
                    println!("  位置 0x{:04X}: {}", pos, s);
                }
            }
        }
        pos += 1;
    }

    // 检查版本标记
    println!("\n文件类型判断:");
    let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
    let version = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);

    println!("  魔数: 0x{:08X} {}", magic, if magic == 0xAA55AA55 { "✓ D2S" } else { "未知" });
    println!("  版本: 0x{:04X} {}", version, match version {
        0x60 => "D2 1.00-1.06 (经典)",
        0x69 => "D2 1.10+ (经典)",
        0x96 => "D2R (重制版)",
        0x97 => "术士君临 (D2R 2.6+)",
        _ => "未知版本",
    });

    // 分析名字长度字段
    println!("\n分析 0x14 位置 (预期是名字长度):");
    println!("  0x14: 0x{:02X} ({})", data[0x14], data[0x14]);

    // 分析 0x15-0x20 的数据
    println!("\n分析 0x15-0x20 区域:");
    let mut i = 0x15;
    while i <= 0x20 {
        print!("  0x{:02X}: 0x{:02X}", i, data[i]);
        if i >= 0x18 && i + 1 < data.len() {
            // 尝试解释为小端序数值
            let val = u16::from_le_bytes([data[i], data[i+1]]);
            println!(" (可能是数值: {})", val);
            i += 2;
        } else {
            println!();
            i += 1;
        }
    }

    // 检查是否是 D2R 格式
    println!("\nD2R 格式检查:");
    // D2R 通常有 "cFM" 或 "cF " 标记
    if data.len() > 0x30 {
        let d2r_marker1 = &data[0x28..0x2B];
        let d2r_marker2 = &data[0x2C..0x2F];
        println!("  0x28-0x2A: {:?} (D2R 位置)", d2r_marker1);
        println!("  0x2C-0x2E: {:?} (D2R 位置)", d2r_marker2);
    }
}
