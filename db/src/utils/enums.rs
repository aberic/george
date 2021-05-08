/*
 * Copyright (c) 2021. Aberic - All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use serde::{Deserialize, Serialize};

pub trait EnumHandler {
    fn tag_u8(tag: Tag) -> u8;
    fn index_type_u8(index_type: IndexType) -> u8;
    fn key_type_u8(key_type: KeyType) -> u8;
    fn tag(b: u8) -> Tag;
    fn index_type(b: u8) -> IndexType;
    fn key_type(b: u8) -> KeyType;
    fn key_type_str(key_type: KeyType) -> String;
}

pub struct Enum {}

impl EnumHandler for Enum {
    fn tag_u8(tag: Tag) -> u8 {
        tag_u8(tag)
    }
    fn index_type_u8(index_type: IndexType) -> u8 {
        index_type_u8(index_type)
    }
    fn key_type_u8(key_type: KeyType) -> u8 {
        key_type_u8(key_type)
    }
    fn tag(b: u8) -> Tag {
        tag(b)
    }
    fn index_type(b: u8) -> IndexType {
        index_type(b)
    }
    fn key_type(b: u8) -> KeyType {
        key_type(b)
    }
    fn key_type_str(key_type: KeyType) -> String {
        key_type_str(key_type)
    }
}

/// 标识符
#[derive(Debug, Clone)]
pub enum Tag {
    /// 引导文件
    Bootstrap,
    /// 缓存页文件
    Page,
    /// 数据库文件
    Database,
    /// 表数据文件
    View,
    /// 索引数据文件
    Index,
}

/// 索引值类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyType {
    /// 字符串索引
    String,
    /// 无符号64位整型
    U64,
    /// 有符号64位整型
    I64,
    /// 无符号64位整型
    U32,
    /// 有符号64位整型
    I32,
    /// 有符号64位浮点类型
    F64,
    /// 有符号32位浮点类型
    F32,
    /// bool类型
    Bool,
    /// 不支持类型
    None,
}

/// 存储引擎类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndexType {
    /// 占位
    None,
    /// 卷宗存储引擎(单文件索引存储-64位)，最合适用于自增
    Sequence,
    /// 卷宗存储引擎(单文件索引存储-32位)
    Dossier,
    /// 文库存储引擎(多文件索引存储-64位)
    Library,
    /// 块存储引擎(区块链索引存储-64位)
    Block,
}

fn tag_u8(tag: Tag) -> u8 {
    match tag {
        Tag::Bootstrap => 0x00,
        Tag::Database => 0x01,
        Tag::View => 0x02,
        Tag::Index => 0x03,
        Tag::Page => 0x04,
    }
}

fn index_type_u8(index_type: IndexType) -> u8 {
    match index_type {
        IndexType::None => 0x00,
        IndexType::Sequence => 0x01,
        IndexType::Dossier => 0x02,
        IndexType::Library => 0x03,
        IndexType::Block => 0x04,
    }
}

fn key_type_u8(key_type: KeyType) -> u8 {
    match key_type {
        KeyType::String => 0x00,
        KeyType::U64 => 0x01,
        KeyType::I64 => 0x02,
        KeyType::U32 => 0x03,
        KeyType::I32 => 0x04,
        KeyType::F64 => 0x05,
        KeyType::F32 => 0x06,
        KeyType::Bool => 0x07,
        KeyType::None => 0x08,
    }
}

fn tag(b: u8) -> Tag {
    match b {
        0x00 => Tag::Bootstrap,
        0x01 => Tag::Database,
        0x02 => Tag::View,
        0x03 => Tag::Index,
        0x04 => Tag::Page,
        _ => Tag::Bootstrap,
    }
}

fn index_type(b: u8) -> IndexType {
    match b {
        0x00 => IndexType::None,
        0x01 => IndexType::Sequence,
        0x02 => IndexType::Dossier,
        0x03 => IndexType::Library,
        0x04 => IndexType::Block,
        _ => IndexType::None,
    }
}

fn key_type(b: u8) -> KeyType {
    match b {
        0x00 => KeyType::String,
        0x01 => KeyType::U64,
        0x02 => KeyType::I64,
        0x03 => KeyType::U32,
        0x04 => KeyType::I32,
        0x05 => KeyType::F64,
        _ => KeyType::String,
    }
}

fn key_type_str(key_type: KeyType) -> String {
    match key_type {
        KeyType::String => String::from("string"),
        KeyType::U64 => String::from("u64"),
        KeyType::I64 => String::from("i64"),
        KeyType::U32 => String::from("u32"),
        KeyType::I32 => String::from("i32"),
        KeyType::F64 => String::from("f64"),
        KeyType::F32 => String::from("f32"),
        KeyType::Bool => String::from("bool"),
        KeyType::None => String::from("none"),
    }
}
