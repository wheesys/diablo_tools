// Copyright 2025 zl. All rights reserved.

use super::error::{Error, Result};

/// 位级写入器，用于写入 D2S 二进制文件
#[derive(Debug, Clone)]
pub struct BitWriter {
    data: Vec<u8>,
    current_byte: u8,
    bit_offset: u8,
}

impl BitWriter {
    /// 创建新的位写入器
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            current_byte: 0,
            bit_offset: 0,
        }
    }

    /// 写入单个位
    pub fn write_bit(&mut self, bit: bool) -> Result<()> {
        self.write_bits(1, if bit { 1 } else { 0 })
    }

    /// 写入指定位数 (最多32位)
    pub fn write_bits(&mut self, bits: u8, value: u32) -> Result<()> {
        if bits > 32 {
            return Err(Error::WriteError("最多写入32位".to_string()));
        }

        let mut bits_remaining = bits;
        let mut value = value;

        while bits_remaining > 0 {
            let bits_to_write = self.bit_offset.min(8 - self.bit_offset).min(bits_remaining);

            let mask = (1u8 << bits_to_write) - 1;
            let value_bits = (value & mask as u32) as u8;
            self.current_byte |= value_bits << self.bit_offset;

            self.bit_offset += bits_to_write;
            bits_remaining -= bits_to_write;
            value >>= bits_to_write;

            if self.bit_offset >= 8 {
                self.data.push(self.current_byte);
                self.current_byte = 0;
                self.bit_offset = 0;
            }
        }

        Ok(())
    }

    /// 写入单个字节
    pub fn write_u8(&mut self, value: u8) -> Result<()> {
        if self.bit_offset == 0 {
            self.data.push(value);
            Ok(())
        } else {
            self.write_bits(8, value as u32)
        }
    }

    /// 写入16位小端序整数
    pub fn write_u16_le(&mut self, value: u16) -> Result<()> {
        self.write_u8((value & 0xFF) as u8)?;
        self.write_u8(((value >> 8) & 0xFF) as u8)
    }

    /// 写入32位小端序整数
    pub fn write_u32_le(&mut self, value: u32) -> Result<()> {
        self.write_u8((value & 0xFF) as u8)?;
        self.write_u8(((value >> 8) & 0xFF) as u8)?;
        self.write_u8(((value >> 16) & 0xFF) as u8)?;
        self.write_u8(((value >> 24) & 0xFF) as u8)
    }

    /// 写入字节数组
    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        for &byte in bytes {
            self.write_u8(byte)?;
        }
        Ok(())
    }

    /// 写入字符串 (固定长度，空填充)
    pub fn write_string(&mut self, s: &str, len: usize) -> Result<()> {
        let bytes = s.as_bytes();
        for i in 0..len {
            if i < bytes.len() {
                self.write_u8(bytes[i])?;
            } else {
                self.write_u8(0)?;
            }
        }
        Ok(())
    }

    /// 对齐到字节边界
    pub fn align_to_byte(&mut self) {
        if self.bit_offset != 0 {
            self.data.push(self.current_byte);
            self.current_byte = 0;
            self.bit_offset = 0;
        }
    }

    /// 完成写入并返回数据
    pub fn finish(mut self) -> Vec<u8> {
        self.align_to_byte();
        self.data
    }

    /// 获取当前数据长度（字节）
    pub fn len(&self) -> usize {
        if self.bit_offset > 0 {
            self.data.len() + 1
        } else {
            self.data.len()
        }
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.data.is_empty() && self.bit_offset == 0
    }
}

impl Default for BitWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_bits() {
        let mut writer = BitWriter::new();
        writer.write_bits(4, 0b1010).unwrap();
        writer.write_bits(8, 0b11001100).unwrap();
        writer.align_to_byte();

        let data = writer.finish();
        assert_eq!(data.len(), 2);
        assert_eq!(data[0], 0b11001010);
    }

    #[test]
    fn test_write_u32_le() {
        let mut writer = BitWriter::new();
        writer.write_u32_le(0x12345678).unwrap();
        let data = writer.finish();

        assert_eq!(data[0], 0x78);
        assert_eq!(data[1], 0x56);
        assert_eq!(data[2], 0x34);
        assert_eq!(data[3], 0x12);
    }
}
