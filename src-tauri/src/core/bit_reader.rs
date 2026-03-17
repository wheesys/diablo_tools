// Copyright 2025 zl. All rights reserved.

use super::error::{Error, Result};
use std::io::{Seek, SeekFrom};

/// 位级读取器，用于解析 D2S 二进制文件
#[derive(Debug, Clone)]
pub struct BitReader {
    data: Vec<u8>,
    position: usize,
    bit_offset: u8,
}

impl Seek for BitReader {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        match pos {
            SeekFrom::Start(offset) => {
                self.position = offset as usize;
                self.bit_offset = 0;
                Ok(self.position as u64)
            }
            SeekFrom::Current(offset) => {
                let new_pos = self.position as i64 + offset;
                if new_pos < 0 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "seek before start",
                    ));
                }
                self.position = new_pos as usize;
                self.bit_offset = 0;
                Ok(self.position as u64)
            }
            SeekFrom::End(offset) => {
                let new_pos = self.data.len() as i64 + offset;
                if new_pos < 0 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "seek before start",
                    ));
                }
                self.position = new_pos as usize;
                self.bit_offset = 0;
                Ok(self.position as u64)
            }
        }
    }
}

impl BitReader {
    /// 创建新的位读取器
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            position: 0,
            bit_offset: 0,
        }
    }

    /// 读取指定位数 (最多32位)
    pub fn read_bits(&mut self, bits: u8) -> Result<u32> {
        if bits > 32 {
            return Err(Error::ReadError("最多读取32位".to_string()));
        }

        let mut result: u32 = 0;
        let mut bits_remaining = bits;

        while bits_remaining > 0 {
            if self.position >= self.data.len() {
                return Err(Error::ReadError("读取超出文件边界".to_string()));
            }

            let byte = self.data[self.position];
            let available_bits = 8 - self.bit_offset;
            let bits_to_read = available_bits.min(bits_remaining);

            let mask = ((1u32 << bits_to_read) - 1) << self.bit_offset;
            let value = ((byte as u32 & mask) >> self.bit_offset) as u32;
            result |= value << (bits - bits_remaining);

            bits_remaining -= bits_to_read;
            self.bit_offset += bits_to_read;

            if self.bit_offset >= 8 {
                self.bit_offset = 0;
                self.position += 1;
            }
        }

        Ok(result)
    }

    /// 读取单个字节
    pub fn read_u8(&mut self) -> Result<u8> {
        if self.bit_offset == 0 {
            if self.position >= self.data.len() {
                return Err(Error::ReadError("读取超出文件边界".to_string()));
            }
            let byte = self.data[self.position];
            self.position += 1;
            Ok(byte)
        } else {
            Ok(self.read_bits(8)? as u8)
        }
    }

    /// 读取16位小端序整数
    pub fn read_u16_le(&mut self) -> Result<u16> {
        let low = self.read_u8()? as u16;
        let high = self.read_u8()? as u16;
        Ok(low | (high << 8))
    }

    /// 读取32位小端序整数
    pub fn read_u32_le(&mut self) -> Result<u32> {
        let b0 = self.read_u8()? as u32;
        let b1 = self.read_u8()? as u32;
        let b2 = self.read_u8()? as u32;
        let b3 = self.read_u8()? as u32;
        Ok(b0 | (b1 << 8) | (b2 << 16) | (b3 << 24))
    }

    /// 读取指定字节数
    pub fn read_bytes(&mut self, len: usize) -> Result<Vec<u8>> {
        let mut bytes = Vec::with_capacity(len);
        for _ in 0..len {
            bytes.push(self.read_u8()?);
        }
        Ok(bytes)
    }

    /// 获取当前位置（字节级别）
    pub fn position(&self) -> usize {
        self.position
    }

    /// 对齐到字节边界
    pub fn align_to_byte(&mut self) {
        if self.bit_offset != 0 {
            self.bit_offset = 0;
            self.position += 1;
        }
    }

    /// 读取单个位
    pub fn read_bit(&mut self) -> Result<bool> {
        let bit = self.read_bits(1)?;
        Ok(bit != 0)
    }

    /// 检查是否还有数据
    pub fn has_more(&self) -> bool {
        self.position < self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_bits() {
        let data = vec![0b10110011, 0b11011100];
        let mut reader = BitReader::new(data);

        assert_eq!(reader.read_bits(4).unwrap(), 0b0011); // 低4位
        assert_eq!(reader.read_bits(8).unwrap(), 0b10110011); // 完整的第一个字节
        assert_eq!(reader.read_bits(4).unwrap(), 0b1100); // 第二个字节低4位
    }
}
