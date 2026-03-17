// Copyright 2025 zl. All rights reserved.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("无效的文件格式: 魔数不匹配")]
    InvalidMagic,

    #[error("不支持的版本: {0}")]
    UnsupportedVersion(u32),

    #[error("读取错误: {0}")]
    ReadError(String),

    #[error("写入错误: {0}")]
    WriteError(String),

    #[error("数据解析错误: {0}")]
    ParseError(String),

    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),
}
