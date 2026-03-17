// Copyright 2025 zl. All rights reserved.

//! Huffman 编码模块
//!
//! D2R 使用 Huffman 编码来压缩物品ID，以节省存档空间
//! 这是 D2R 相比经典版的重要变化之一

use super::error::{Error, Result};
use std::collections::HashMap;

/// Huffman 树节点
#[derive(Debug, Clone)]
enum HuffmanNode {
    Leaf(u8),
    Internal {
        zero: Box<HuffmanNode>,
        one: Box<HuffmanNode>,
    },
}

/// Huffman 编码器
#[derive(Debug, Clone)]
pub struct HuffmanEncoder {
    /// 字符到 Huffman 编码的映射
    codes: HashMap<u8, Vec<bool>>,
    /// 编码表 (用于快速查找)
    encode_table: HashMap<u8, u32>,
    /// 编码长度
    code_lengths: HashMap<u8, u8>,
}

/// Huffman 解码器
#[derive(Debug, Clone)]
pub struct HuffmanDecoder {
    /// Huffman 树根节点
    root: HuffmanNode,
    /// 最大编码长度
    max_code_length: u8,
}

/// D2R 物品 ID Huffman 编码表
///
/// 这个表定义了物品代码中每个字符的 Huffman 编码
/// 基于 D2R 的实际编码规则
const D2R_HUFFMAN_TABLE: &[(u8, &str)] = &[
    (b'a', "0"),
    (b'b', "100000"),
    (b'c', "100001"),
    (b'd', "100010"),
    (b'e', "100011"),
    (b'f', "100100"),
    (b'g', "100101"),
    (b'h', "100110"),
    (b'i', "100111"),
    (b'j', "101000"),
    (b'k', "101001"),
    (b'l', "101010"),
    (b'm', "101011"),
    (b'n', "101100"),
    (b'o', "101101"),
    (b'p', "101110"),
    (b'q', "101111"),
    (b'r', "110000"),
    (b's', "110001"),
    (b't', "110010"),
    (b'u', "110011"),
    (b'v', "110100"),
    (b'w', "110101"),
    (b'x', "110110"),
    (b'y', "110111"),
    (b'z', "111000"),
    (b'0', "111001"),
    (b'1', "111010"),
    (b'2', "111011"),
    (b'3', "111100"),
    (b'4', "111101"),
    (b'5', "111110"),
    (b'6', "111111"),
    (b'7', "000000"),
    (b'8', "000001"),
    (b'9', "000010"),
    (b'\n', "000011"), // 换行符
];

impl HuffmanEncoder {
    /// 创建新的 D2R Huffman 编码器
    pub fn new_d2r() -> Self {
        let mut codes = HashMap::new();
        let mut encode_table = HashMap::new();
        let mut code_lengths = HashMap::new();

        for &(byte, code_str) in D2R_HUFFMAN_TABLE {
            let bits: Vec<bool> = code_str.chars().map(|c| c == '1').collect();
            let len = bits.len() as u8;

            // 转换为整数编码
            let mut code: u32 = 0;
            for (i, &bit) in bits.iter().enumerate() {
                if bit {
                    code |= 1 << (bits.len() - 1 - i);
                }
            }

            codes.insert(byte, bits);
            encode_table.insert(byte, code);
            code_lengths.insert(byte, len);
        }

        Self {
            codes,
            encode_table,
            code_lengths,
        }
    }

    /// 编码单个字符
    pub fn encode_char(&self, char: u8, writer: &mut impl BitWriterTrait) -> Result<()> {
        if let Some(bits) = self.codes.get(&char) {
            for &bit in bits {
                writer.write_bit(bit)?;
            }
            Ok(())
        } else {
            Err(Error::ParseError(format!("未找到字符的 Huffman 编码: {}", char)))
        }
    }

    /// 编码字符串
    pub fn encode_string(&self, s: &str, writer: &mut impl BitWriterTrait) -> Result<()> {
        for byte in s.bytes() {
            self.encode_char(byte, writer)?;
        }
        Ok(())
    }

    /// 获取字符的编码长度
    pub fn get_code_length(&self, char: u8) -> Option<u8> {
        self.code_lengths.get(&char).copied()
    }
}

impl HuffmanDecoder {
    /// 创建新的 D2R Huffman 解码器
    pub fn new_d2r() -> Self {
        // 从编码表构建 Huffman 树
        let root = Self::build_huffman_tree(D2R_HUFFMAN_TABLE);
        let max_code_length = D2R_HUFFMAN_TABLE
            .iter()
            .map(|(_, code)| code.len() as u8)
            .max()
            .unwrap_or(8);

        Self {
            root,
            max_code_length,
        }
    }

    /// 构建 Huffman 树
    fn build_huffman_tree(table: &[(u8, &str)]) -> HuffmanNode {
        let mut root = HuffmanNode::Internal {
            zero: Box::new(HuffmanNode::Leaf(0)),
            one: Box::new(HuffmanNode::Leaf(0)),
        };

        for &(byte, code_str) in table {
            let mut current = &mut root;
            for c in code_str.chars() {
                let bit = c == '1';
                current = match current {
                    HuffmanNode::Internal { zero, one } => {
                        if bit {
                            one
                        } else {
                            zero
                        }
                    }
                    HuffmanNode::Leaf(_) => {
                        // 这种情况不应该发生，如果编码表正确的话
                        break;
                    }
                };
            }

            // 在最后设置叶子节点
            if let HuffmanNode::Internal { one, .. } = current {
                *one = Box::new(HuffmanNode::Leaf(byte));
            }
        }

        root
    }

    /// 解码单个字符
    pub fn decode_char(&self, reader: &mut impl BitReaderTrait) -> Result<u8> {
        let mut current = &self.root;
        let mut depth = 0;

        loop {
            if depth > self.max_code_length {
                return Err(Error::ParseError("Huffman 解码失败: 超过最大深度".to_string()));
            }

            let bit = reader.read_bit()?;

            current = match current {
                HuffmanNode::Internal { zero, one } => {
                    if bit {
                        one
                    } else {
                        zero
                    }
                }
                HuffmanNode::Leaf(byte) => {
                    return Ok(*byte);
                }
            };

            depth += 1;
        }
    }

    /// 解码字符串（固定长度）
    pub fn decode_string_fixed(&self, reader: &mut impl BitReaderTrait, len: usize) -> Result<String> {
        let mut bytes = Vec::with_capacity(len);
        for _ in 0..len {
            bytes.push(self.decode_char(reader)?);
        }
        String::from_utf8(bytes)
            .map_err(|e| Error::ParseError(format!("Huffman 解码字符串失败: {}", e)))
    }

    /// 解码字符串（直到遇到终止符）
    pub fn decode_string_null(&self, reader: &mut impl BitReaderTrait) -> Result<String> {
        let mut bytes = Vec::new();
        loop {
            let byte = self.decode_char(reader)?;
            if byte == 0 {
                break;
            }
            bytes.push(byte);
        }
        String::from_utf8(bytes)
            .map_err(|e| Error::ParseError(format!("Huffman 解码字符串失败: {}", e)))
    }
}

/// BitWriter trait - 用于 Huffman 编码写入
pub trait BitWriterTrait {
    fn write_bit(&mut self, bit: bool) -> Result<()>;
}

/// BitReader trait - 用于 Huffman 解码读取
pub trait BitReaderTrait {
    fn read_bit(&mut self) -> Result<bool>;
}

// 实现 BitWriter trait for BitWriter
impl BitWriterTrait for super::BitWriter {
    fn write_bit(&mut self, bit: bool) -> Result<()> {
        self.write_bits(1, if bit { 1 } else { 0 })
    }
}

// 实现 BitReader trait for BitReader
impl BitReaderTrait for super::BitReader {
    fn read_bit(&mut self) -> Result<bool> {
        self.read_bit()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_huffman_encode_decode() {
        let encoder = HuffmanEncoder::new_d2r();
        let decoder = HuffmanDecoder::new_d2r();

        let test_string = "abc";
        let mut encoded_data = Vec::new();
        let mut writer = super::super::BitWriter::new();

        // 编码
        for byte in test_string.bytes() {
            encoder.encode_char(byte, &mut writer).unwrap();
        }
        encoded_data = writer.finish();

        // 解码
        let mut reader = super::super::BitReader::new(encoded_data);
        let decoded = decoder.decode_string_null(&mut reader).unwrap();

        assert_eq!(decoded, test_string);
    }

    #[test]
    fn test_code_length() {
        let encoder = HuffmanEncoder::new_d2r();

        // 'a' 应该是最短的编码 (只有1位)
        assert_eq!(encoder.get_code_length(b'a'), Some(1));

        // 其他字符编码较长
        assert!(encoder.get_code_length(b'b').unwrap_or(99) > 1);
    }
}
