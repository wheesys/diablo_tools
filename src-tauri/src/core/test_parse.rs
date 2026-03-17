// Copyright 2025 zl. All rights reserved.

//! 测试 D2S 文件解析

use super::bit_reader::BitReader;
use super::d2s::{D2SFile, D2SHeader, D2SVersion, D2S_MAGIC};
use super::error::Result;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duzhong_save() {
        let data = std::fs::read("doc/杜仲.d2s").expect("无法读取存档文件");

        println!("文件大小: {} 字节", data.len());
        println!("文件头: {:02X} {:02X} {:02X} {:02X}",
            data[0], data[1], data[2], data[3]);

        let mut reader = BitReader::new(data);

        // 解析文件头
        let header = D2SHeader::parse(&mut reader);
        match header {
            Ok(h) => {
                println!("✓ 文件头解析成功");
                println!("  魔数: 0x{:08X}", h.magic);
                println!("  版本: 0x{:04X}", h.version);
                println!("  文件大小: {}", h.file_size);
                println!("  校验和: 0x{:08X}", h.checksum);
            }
            Err(e) => {
                println!("✗ 文件头解析失败: {:?}", e);
                return;
            }
        }

        // 检查版本
        let version = D2SVersion::from_u32(header.unwrap().version);
        println!("  检测到的版本: {:?}", version);
    }
}

// 单独的可执行测试函数
pub fn test_parse_d2s_file(path: &str) -> Result<()> {
    let data = std::fs::read(path)?;
    println!("=== D2S 文件解析测试 ===");
    println!("文件: {}", path);
    println!("大小: {} 字节", data.len());

    // 版本检测 (使用智能检测) - 需要在创建 reader 之前完成
    let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
    let version_value = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
    let version = D2SVersion::detect_from_data(&data, version_value);

    println!("\n[文件头]");
    println!("魔数: 0x{:08X} ({})", magic,
        if magic == D2S_MAGIC { "正确" } else { "错误" });
    println!("版本: 0x{:04X}", version_value);

    println!("\n[版本检测]");
    println!("类型: {:?}", version);
    println!("是否D2R: {}", version.is_d2r());
    println!("是否术士君临: {}", version.is_warlock());

    Ok(())
}
