// Copyright 2025 zl. All rights reserved.

//! 测试 D2R 格式检测

fn main() {
    let path = "../doc/杜仲.d2s";
    let data = std::fs::read(path).expect("无法读取存档文件");

    println!("=== D2R 格式检测测试 ===");
    println!("文件大小: {} 字节 (0x{:X})", data.len(), data.len());
    println!();

    // 检查技能标记
    println!("--- 技能标记检测 ---");
    let offset = 0xFB;
    println!("检查偏移 0x{:X}:", offset);
    if data.len() > offset + 4 {
        let marker = &data[offset..offset + 4];
        println!("  找到的字节: {:?}", marker);
        println!("  作为字符串: {}", String::from_utf8_lossy(marker));

        let expected = b"scm ";
        println!("  期望的标记: {:?}", expected);
        println!("  匹配? {}", marker == expected);
    }
    println!();

    // 检查 WS 标记
    println!("--- WS 标记检测 ---");
    let ws_positions = [0x2B8, 0x2BD, 0x2B0, 0x2C0];
    for &pos in &ws_positions {
        if data.len() > pos + 2 {
            let marker = &data[pos..pos + 2];
            println!("  位置 0x{:X}: {:?} -> {}", pos, marker, String::from_utf8_lossy(marker));
        }
    }
    println!();

    // 检查 JM 标记
    println!("--- JM 标记检测 ---");
    let mut jm_found = false;
    let mut jm_pos = 0;
    for pos in 0x300..data.len().saturating_sub(2) {
        if data[pos..pos + 2] == b"JM"[..] {
            jm_found = true;
            jm_pos = pos;
            break;
        }
    }
    println!("  JM 标记位置: 0x{:X} ({})", jm_pos, jm_found);
    println!();

    // 汇总
    println!("--- 检测结果汇总 ---");
    println!("  文件大小 > 0x200 (512)? {}", data.len() > 0x200);
    println!("  文件大小 > 0x400 (1024)? {}", data.len() > 0x400);

    let has_skills = data.len() > 0xFB + 4 && {
        let marker = &data[0xFB..0xFB + 4];
        *marker == b"scm "[..]
    };
    println!("  有 D2R 技能标记? {}", has_skills);

    let has_ws = ws_positions.iter().any(|&pos| {
        data.len() > pos + 2 && data[pos..pos + 2] == b"WS"[..]
    });
    println!("  有 WS 标记? {}", has_ws);
    println!("  有 JM 标记? {}", jm_found);

    let d2r_indicators = [has_skills, has_ws, jm_found].iter().filter(|&&x| x).count();
    println!("  D2R 特征数量: {}", d2r_indicators);
    println!("  判定: {}", if d2r_indicators >= 2 { "D2R" } else { "Classic" });
}
