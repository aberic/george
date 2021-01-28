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
    fn engine_type_u8(engine_type: EngineType) -> u8;
    fn capacity_u8(capacity: Capacity) -> u8;
    fn mold_u8(mold: IndexMold) -> u8;
    fn index_type_u8(index_type: IndexType) -> u8;
    fn tag(b: u8) -> Tag;
    fn engine_type(b: u8) -> EngineType;
    fn mold(b: u8) -> IndexMold;
    fn capacity(b: u8) -> Capacity;
    fn index_type(b: u8) -> IndexType;
    fn mold_str(mold: IndexMold) -> String;
}

pub struct Enum {}

impl EnumHandler for Enum {
    fn tag_u8(tag: Tag) -> u8 {
        tag_u8(tag)
    }
    fn engine_type_u8(engine_type: EngineType) -> u8 {
        engine_type_u8(engine_type)
    }
    fn capacity_u8(capacity: Capacity) -> u8 {
        capacity_u8(capacity)
    }
    fn mold_u8(mold: IndexMold) -> u8 {
        mold_u8(mold)
    }
    fn index_type_u8(index_type: IndexType) -> u8 {
        index_type_u8(index_type)
    }
    fn tag(b: u8) -> Tag {
        tag(b)
    }
    fn engine_type(b: u8) -> EngineType {
        engine_type(b)
    }
    fn mold(b: u8) -> IndexMold {
        mold(b)
    }
    fn capacity(b: u8) -> Capacity {
        capacity(b)
    }
    fn index_type(b: u8) -> IndexType {
        index_type(b)
    }
    fn mold_str(mold: IndexMold) -> String {
        mold_str(mold)
    }
}

/// 标识符
#[derive(Debug, Clone)]
pub enum Tag {
    /// 引导文件
    Bootstrap,
    /// 数据库文件
    Database,
    /// 表数据文件
    View,
    /// 索引数据文件
    Index,
}

/// 索引类型
///
/// 主键溯源；主键不溯源；普通索引
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum IndexType {
    /// 占位
    None,
    /// 普通索引
    Normal,
    /// 主键不溯源
    Major,
    /// 主键溯源
    Trace,
}

/// 索引值类型
#[derive(Debug, Clone, Copy)]
pub enum IndexMold {
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
}

/// 存储引擎类型
#[derive(Debug, Clone, Copy)]
pub enum EngineType {
    /// 占位
    None,
    /// 内存存储引擎
    Memory,
    /// 卷宗存储引擎(单文件索引存储-64位)，最合适用于自增
    Dossier,
    /// 文库存储引擎(多文件索引存储-64位)
    Library,
    /// 块存储引擎(区块链索引存储-64位)
    Block,
}

/// 存储量级
#[derive(Debug, Clone, Copy)]
pub enum Capacity {
    /// 占位
    None,
    /// 低级，支持存储2^32个元素
    U32,
    /// 高级，支持存储2^64个元素
    U64,
}

fn tag_u8(tag: Tag) -> u8 {
    match tag {
        Tag::Bootstrap => 0x00,
        Tag::Database => 0x01,
        Tag::View => 0x02,
        Tag::Index => 0x03,
    }
}

fn engine_type_u8(engine_type: EngineType) -> u8 {
    match engine_type {
        EngineType::None => 0x00,
        EngineType::Memory => 0x01,
        EngineType::Dossier => 0x02,
        EngineType::Library => 0x03,
        EngineType::Block => 0x04,
    }
}

fn capacity_u8(capacity: Capacity) -> u8 {
    match capacity {
        Capacity::None => 0x00,
        Capacity::U32 => 0x01,
        Capacity::U64 => 0x02,
    }
}

fn mold_u8(mold: IndexMold) -> u8 {
    match mold {
        IndexMold::String => 0x00,
        IndexMold::U64 => 0x01,
        IndexMold::I64 => 0x02,
        IndexMold::U32 => 0x03,
        IndexMold::I32 => 0x04,
        IndexMold::F64 => 0x05,
        IndexMold::F32 => 0x06,
        IndexMold::Bool => 0x07,
    }
}

fn index_type_u8(index_class: IndexType) -> u8 {
    match index_class {
        IndexType::None => 0x00,
        IndexType::Normal => 0x01,
        IndexType::Major => 0x02,
        IndexType::Trace => 0x03,
    }
}

fn tag(b: u8) -> Tag {
    match b {
        0x00 => Tag::Bootstrap,
        0x01 => Tag::Database,
        0x02 => Tag::View,
        0x03 => Tag::Index,
        _ => Tag::Bootstrap,
    }
}

fn engine_type(b: u8) -> EngineType {
    match b {
        0x00 => EngineType::None,
        0x01 => EngineType::Memory,
        0x02 => EngineType::Dossier,
        0x03 => EngineType::Library,
        0x04 => EngineType::Block,
        _ => EngineType::None,
    }
}

fn mold(b: u8) -> IndexMold {
    match b {
        0x00 => IndexMold::String,
        0x01 => IndexMold::U64,
        0x02 => IndexMold::I64,
        0x03 => IndexMold::U32,
        0x04 => IndexMold::I32,
        0x05 => IndexMold::F64,
        _ => IndexMold::String,
    }
}

fn capacity(b: u8) -> Capacity {
    match b {
        0x00 => Capacity::None,
        0x01 => Capacity::U32,
        0x02 => Capacity::U64,
        _ => Capacity::U32,
    }
}

fn index_type(b: u8) -> IndexType {
    match b {
        0x00 => IndexType::None,
        0x01 => IndexType::Normal,
        0x02 => IndexType::Major,
        0x03 => IndexType::Trace,
        _ => IndexType::None,
    }
}

fn mold_str(mold: IndexMold) -> String {
    match mold {
        IndexMold::String => String::from("string"),
        IndexMold::U64 => String::from("u64"),
        IndexMold::I64 => String::from("i64"),
        IndexMold::U32 => String::from("u32"),
        IndexMold::I32 => String::from("i32"),
        IndexMold::F64 => String::from("f64"),
        IndexMold::F32 => String::from("f32"),
        IndexMold::Bool => String::from("bool"),
    }
}
