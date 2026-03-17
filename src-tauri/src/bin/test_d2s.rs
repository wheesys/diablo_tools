// Copyright 2025 zl. All rights reserved.

//! D2S 文件解析测试工具

use diablo_tools_tauri::core::test_parse;

fn main() {
    let path = "../doc/杜仲.d2s";

    println!("开始解析存档文件: {}\n", path);

    match test_parse::test_parse_d2s_file(path) {
        Ok(_) => println!("\n✓ 解析完成!"),
        Err(e) => println!("\n✗ 解析失败: {:?}", e),
    }
}
