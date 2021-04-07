/*
 * Copyright (c) 2020. Aberic - All Rights Reserved.
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

use crate::utils::enums::KeyType;
use comm::cryptos::hash::{
    hashcode64_f64_real, hashcode64_i64_real, CRCTypeHandler, Hash, HashType,
};
use comm::errors::entrances::{err_str, err_string, GeorgeResult};
use comm::strings::{StringHandler, Strings};
use serde_json::{Error, Number, Value};

pub const GEORGE_DB_CONFIG: &str = "GEORGE_DB_CONFIG";
pub const GEORGE_DB_DATA_DIR: &str = "GEORGE_DB_DATA_DIR";
pub const GEORGE_DB_LIMIT_OPEN_FILE: &str = "GEORGE_DB_LIMIT_OPEN_FILE";
pub const GEORGE_DB_LOG_DIR: &str = "GEORGE_DB_LOG_DIR";
pub const GEORGE_DB_LOG_FILE_MAX_SIZE: &str = "GEORGE_DB_LOG_FILE_MAX_SIZE";
pub const GEORGE_DB_LOG_FILE_MAX_COUNT: &str = "GEORGE_DB_LOG_FILE_MAX_COUNT";
pub const GEORGE_DB_LOG_LEVEL: &str = "GEORGE_DB_LOG_LEVEL";
pub const GEORGE_DB_PRODUCTION: &str = "GEORGE_DB_PRODUCTION";

pub const DEFAULT_DATABASE: &str = "sys";
pub const DEFAULT_VIEW: &str = "view";
pub const DEFAULT_COMMENT: &str = "system default";

/// 默认KV存储索引
pub const INDEX_CATALOG: &str = "george_db_index_catalog";
/// 默认自增序列ID索引，不保证连续性，只保证有序性
pub const INDEX_SEQUENCE: &str = "george_db_index_sequence";
/// 默认KV存储索引内存版
pub const INDEX_MEMORY: &str = "george_db_index_memory";

/// 数据结果数据类型，正常数据类型
pub const VALUE_TYPE_NORMAL: u8 = 0x00;
/// 数据结果数据类型，碰撞数据类型
pub const VALUE_TYPE_CRASH: u8 = 0x01;

/// LEVEL1DISTANCE level1间隔 256^3 = 16777216 | 测试 4^3 = 64 | 4294967296
const LEVEL1DISTANCE32: u32 = 16777216;
/// LEVEL2DISTANCE level2间隔 256^2 = 65536 | 测试 4^2 = 16
const LEVEL2DISTANCE32: u32 = 65536;
/// LEVEL3DISTANCE level3间隔 256^1 = 256 | 测试 4^1 = 4
const LEVEL3DISTANCE32: u32 = 256;
/// LEVEL4DISTANCE level4间隔 256^0 = 1 | 测试 4^0 = 1
const LEVEL4DISTANCE32: u32 = 1;

/// LEVEL1DISTANCE level1间隔 65536^3 = 281474976710656 | 测试 4^3 = 64 | 9223372036854775808 * 2<p>
/// 18446744073709551615<p>
/// 9223372036854775808
const LEVEL1DISTANCE64: u64 = 281474976710656;
/// LEVEL2DISTANCE level2间隔 65536^2 = 4294967296 | 测试 4^2 = 16
const LEVEL2DISTANCE64: u64 = 4294967296;
/// LEVEL3DISTANCE level3间隔 65536^1 = 65536 | 测试 4^1 = 4
const LEVEL3DISTANCE64: u64 = 65536;
/// LEVEL4DISTANCE level4间隔 65536^0 = 1 | 测试 4^0 = 1
const LEVEL4DISTANCE64: u64 = 1;

/// 获取在2^32量级组成树的指定层中元素的间隔数，即每一度中存在的元素数量
pub fn level_distance_32(level: u8) -> u32 {
    match level {
        1 => return LEVEL1DISTANCE32,
        2 => return LEVEL2DISTANCE32,
        3 => return LEVEL3DISTANCE32,
        4 => return LEVEL4DISTANCE32,
        _ => 0,
    }
}

/// 获取在2^64量级组成树的指定层中元素的间隔数，即每一度中存在的元素数量
pub fn level_distance_64(level: u8) -> u64 {
    match level {
        1 => return LEVEL1DISTANCE64,
        2 => return LEVEL2DISTANCE64,
        3 => return LEVEL3DISTANCE64,
        4 => return LEVEL4DISTANCE64,
        _ => 0,
    }
}

pub fn key_fetch(index_name: String, value: Vec<u8>) -> GeorgeResult<String> {
    let value_str = Strings::from_utf8(value)?;
    let res: Result<Value, Error> = serde_json::from_str(value_str.as_ref());
    match res {
        Ok(v) => match v[index_name.clone()] {
            Value::Null => Err(err_string(format!(
                "key structure {} do not support none!",
                index_name
            ))),
            Value::Object(..) => Err(err_string(format!(
                "key structure {} do not support object!",
                index_name
            ))),
            Value::Array(..) => Err(err_string(format!(
                "key structure {} do not support array!",
                index_name
            ))),
            _ => Ok(format!("{}", v[index_name])),
        },
        Err(err) => Err(err_string(err.to_string())),
    }
}

/// 检查字节数组是否已有数据，即不为空且每一个字节都不是0x00
pub fn is_bytes_fill(bs: Vec<u8>) -> bool {
    let bs_len = bs.len();
    let mut i = 0;
    while i < bs_len {
        if bs[i].ne(&0x00) {
            return true;
        }
        i += 1;
    }
    false
}

pub fn hash_key(key_type: KeyType, key: String) -> GeorgeResult<u64> {
    let mut hash_key: u64 = 0;
    match key_type {
        KeyType::String => hash_key = Hash::crc64(HashType::String, key)?,
        KeyType::Bool => hash_key = Hash::crc64(HashType::Bool, key)?,
        KeyType::U32 => hash_key = Hash::crc64(HashType::U32, key)?,
        KeyType::U64 => hash_key = Hash::crc64(HashType::U64, key)?,
        KeyType::F32 => hash_key = Hash::crc64(HashType::F32, key)?,
        KeyType::F64 => hash_key = Hash::crc64(HashType::F64, key)?,
        KeyType::I32 => hash_key = Hash::crc64(HashType::I32, key)?,
        KeyType::I64 => hash_key = Hash::crc64(HashType::I64, key)?,
        _ => return Err(err_str("key type not support!")),
    }
    Ok(hash_key)
}

pub fn hash_key_number(key_type: KeyType, key: &Number) -> GeorgeResult<u64> {
    let mut hash_key: u64 = 0;
    match key_type {
        KeyType::U32 => hash_key = key.as_u64().unwrap(),
        KeyType::U64 => hash_key = key.as_u64().unwrap(),
        KeyType::F32 => hash_key = hashcode64_f64_real(key.as_f64().unwrap()),
        KeyType::F64 => hash_key = hashcode64_f64_real(key.as_f64().unwrap()),
        KeyType::I32 => hash_key = hashcode64_i64_real(key.as_i64().unwrap()),
        KeyType::I64 => hash_key = hashcode64_i64_real(key.as_i64().unwrap()),
        _ => return Err(err_str("key type not support!")),
    }
    Ok(hash_key)
}
