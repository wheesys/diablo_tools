// Copyright 2025 zl. All rights reserved.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("核心解析错误: {0}")]
    Core(String),

    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),
}

// 实现从核心错误到应用错误的转换
impl From<crate::core::error::Error> for Error {
    fn from(err: crate::core::error::Error) -> Self {
        Error::Core(err.to_string())
    }
}
