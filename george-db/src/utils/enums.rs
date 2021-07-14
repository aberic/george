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

use crate::utils::{Enum, EnumHandler};

impl EnumHandler for Enum {
    fn engine_u8(engine: Engine) -> u8 {
        engine_u8(engine)
    }

    fn key_type_u8(key_type: KeyType) -> u8 {
        key_type_u8(key_type)
    }

    fn engine(b: u8) -> Engine {
        engine(b)
    }

    fn key_type(b: u8) -> KeyType {
        key_type(b)
    }
}

/// 索引值类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyType {
    /// 字符串索引
    String,
    /// 无符号64位整型
    UInt,
    /// 有符号64位整型
    Int,
    /// 有符号64位浮点类型
    Float,
    /// bool类型
    Bool,
    /// 不支持类型
    None,
}

/// 存储引擎类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Engine {
    /// 占位
    None,
    /// 卷宗存储引擎(单文件索引存储-64位)，最合适用于自增
    Increment,
    /// 卷宗存储引擎(单文件索引存储-64位)，最合适用于不重复u64
    Sequence,
    /// 卷宗存储引擎(单文件索引存储-32位)
    Disk,
    /// 块存储引擎(区块链索引存储-32位)
    Block,
}

fn engine_u8(index_type: Engine) -> u8 {
    match index_type {
        Engine::None => 0x00,
        Engine::Increment => 0x01,
        Engine::Disk => 0x02,
        Engine::Sequence => 0x03,
        Engine::Block => 0x04,
    }
}

fn key_type_u8(key_type: KeyType) -> u8 {
    match key_type {
        KeyType::String => 0x00,
        KeyType::UInt => 0x01,
        KeyType::Int => 0x02,
        KeyType::Float => 0x05,
        KeyType::Bool => 0x07,
        KeyType::None => 0x08,
    }
}

fn engine(b: u8) -> Engine {
    match b {
        0x00 => Engine::None,
        0x01 => Engine::Increment,
        0x02 => Engine::Disk,
        0x03 => Engine::Sequence,
        0x04 => Engine::Block,
        _ => Engine::None,
    }
}

fn key_type(b: u8) -> KeyType {
    match b {
        0x00 => KeyType::String,
        0x01 => KeyType::UInt,
        0x02 => KeyType::Int,
        0x05 => KeyType::Float,
        _ => KeyType::String,
    }
}
