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

use serde_json::Value;

use comm::cryptos::hash::HashCRCTypeHandler;
use comm::cryptos::Hash;
use comm::errors::{Errs, GeorgeResult};
use comm::json::{JsonGet, JsonNew};
use comm::Json;

use crate::utils::enums::KeyType;

pub const GEORGE_DB_CONFIG: &str = "GEORGE_DB_CONFIG";
pub const GEORGE_DB_DATA_DIR: &str = "GEORGE_DB_DATA_DIR";
pub const GEORGE_DB_THREAD_COUNT: &str = "GEORGE_DB_THREAD_COUNT";
pub const GEORGE_DB_LOG_DIR: &str = "GEORGE_DB_LOG_DIR";
pub const GEORGE_DB_LOG_FILE_MAX_SIZE: &str = "GEORGE_DB_LOG_FILE_MAX_SIZE";
pub const GEORGE_DB_LOG_FILE_MAX_COUNT: &str = "GEORGE_DB_LOG_FILE_MAX_COUNT";
pub const GEORGE_DB_LOG_LEVEL: &str = "GEORGE_DB_LOG_LEVEL";
pub const GEORGE_DB_PRODUCTION: &str = "GEORGE_DB_PRODUCTION";

pub const DEFAULT_NAME: &str = "sys";
pub const DEFAULT_COMMENT: &str = "system default";

/// 默认KV存储索引
pub const INDEX_DISK: &str = "george_db_index_disk";
/// 默认自增序列ID索引，不保证连续性，只保证有序性
pub const INDEX_INCREMENT: &str = "george_db_index_increment";
/// 默认`Block KV`存储索引，区块hash存储索引，根据块hash查询区块
pub const INDEX_BLOCK_HASH: &str = "george_db_index_block_hash";
/// 默认`Block Height`存储索引，区块高度存储索引，根据块高查询区块
pub const INDEX_BLOCK_HEIGHT: &str = "george_db_index_block_height";
/// 默认`Tx KV`存储索引，交易hash存储索引，根据交易hash查询区块、查询交易
pub const INDEX_TX_HASH: &str = "george_db_index_tx_hash";

/// 数据结果数据类型，正常数据类型
pub const VALUE_TYPE_NORMAL: u8 = 0x00;
/// 数据结果数据类型，碰撞数据类型
pub const VALUE_TYPE_CRASH: u8 = 0x01;

// 1 256 65536 16777216 4294967296 1099511627776 281474976710656 72057594037927936 18446744073709551616
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

/// LEVEL1DISTANCES level1间隔 1170^6 = 2565164201769000000 | 测试 4^3 = 64 | 9223372036854775808 * 2<p>
const LEVEL1DISTANCE64S: u64 = 2565164201769000000;
/// LEVEL2DISTANCES level1间隔 1170^5 = 2192448035700000 | 测试 4^3 = 64 | 9223372036854775808 * 2<p>
const LEVEL2DISTANCE64S: u64 = 2192448035700000;
/// LEVEL3DISTANCES level2间隔 1170^4 = 1873887210000 | 测试 4^2 = 16
const LEVEL3DISTANCE64S: u64 = 1873887210000;
/// LEVEL4DISTANCES level2间隔 1170^3 = 1601613000 | 测试 4^2 = 16
const LEVEL4DISTANCE64S: u64 = 1601613000;
/// LEVEL5DISTANCES level2间隔 1170^2 = 1368900 | 测试 4^2 = 16
const LEVEL5DISTANCE64S: u64 = 1368900;
/// LEVEL6DISTANCES level3间隔 1170^1 = 1170 | 测试 4^1 = 4
const LEVEL6DISTANCE64S: u64 = 1170;
/// LEVEL7DISTANCES level4间隔 1170^0 = 1 | 测试 4^0 = 1
const LEVEL7DISTANCE64S: u64 = 1;

pub struct Distance;

impl Distance {
    /// 获取在2^32量级组成树的指定层中元素的间隔数，即每一度中存在的元素数量
    pub fn level_32(level: u8) -> u32 {
        level_distance_32(level)
    }

    /// 获取在2^64量级组成树的指定层中元素的间隔数，即每一度中存在的元素数量
    pub fn level_64(level: u8) -> u64 {
        level_distance_64(level)
    }

    /// 获取在2^64量级组成树的指定层中元素的间隔数，即每一度中存在的元素数量
    pub fn level_64s(level: u8) -> u64 {
        level_distance_64s(level)
    }
}

/// 获取在2^32量级组成树的指定层中元素的间隔数，即每一度中存在的元素数量
fn level_distance_32(level: u8) -> u32 {
    match level {
        1 => return LEVEL1DISTANCE32,
        2 => return LEVEL2DISTANCE32,
        3 => return LEVEL3DISTANCE32,
        4 => return LEVEL4DISTANCE32,
        _ => 0,
    }
}

/// 获取在2^64量级组成树的指定层中元素的间隔数，即每一度中存在的元素数量
fn level_distance_64(level: u8) -> u64 {
    match level {
        1 => return LEVEL1DISTANCE64,
        2 => return LEVEL2DISTANCE64,
        3 => return LEVEL3DISTANCE64,
        4 => return LEVEL4DISTANCE64,
        _ => 0,
    }
}

/// 获取在2^64量级组成树的指定层中元素的间隔数，即每一度中存在的元素数量
fn level_distance_64s(level: u8) -> u64 {
    match level {
        1 => return LEVEL1DISTANCE64S,
        2 => return LEVEL2DISTANCE64S,
        3 => return LEVEL3DISTANCE64S,
        4 => return LEVEL4DISTANCE64S,
        5 => return LEVEL5DISTANCE64S,
        6 => return LEVEL6DISTANCE64S,
        7 => return LEVEL7DISTANCE64S,
        _ => 0,
    }
}

pub struct IndexKey;

impl IndexKey {
    pub fn fetch(index_name: String, value: Vec<u8>) -> GeorgeResult<String> {
        key_fetch(index_name, value)
    }

    pub fn hash(key_type: KeyType, key: String) -> GeorgeResult<u64> {
        hash_key_64(key_type, key)
    }
}

fn key_fetch(index_name: String, value: Vec<u8>) -> GeorgeResult<String> {
    let json = Json::new(value)?;
    let value = json.get_value(index_name.clone())?;
    match value {
        Value::Null => Err(Errs::string(format!(
            "key structure {} do not support none!",
            index_name
        ))),
        Value::Object(_) => Err(Errs::string(format!(
            "key structure {} do not support object!",
            index_name
        ))),
        Value::Array(_) => Err(Errs::string(format!(
            "key structure {} do not support array!",
            index_name
        ))),
        _ => Ok(format!("{}", value)),
    }
}

fn hash_key_64(key_type: KeyType, key: String) -> GeorgeResult<u64> {
    match key_type {
        KeyType::String => Hash::crc64_string(key),
        KeyType::Bool => Hash::crc64_bool(key),
        KeyType::UInt => Hash::crc64_u64(key),
        KeyType::Float => Hash::crc64_f64(key),
        KeyType::Int => Hash::crc64_i64(key),
        _ => Err(Errs::str("key type not support!")),
    }
}
