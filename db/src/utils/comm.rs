use comm::errors::entrances::{err_string, GeorgeResult};
use serde_json::{Error, Value};

pub const GEORGE_DB_CONFIG: &str = "GEORGE_DB_CONFIG";
pub const GEORGE_DB_DATA_DIR: &str = "GEORGE_DB_DATA_DIR";
pub const GEORGE_DB_LIMIT_OPEN_FILE: &str = "GEORGE_DB_LIMIT_OPEN_FILE";
pub const GEORGE_DB_LOG_DIR: &str = "GEORGE_DB_LOG_DIR";
pub const GEORGE_DB_LOG_FILE_MAX_SIZE: &str = "GEORGE_DB_LOG_FILE_MAX_SIZE";
pub const GEORGE_DB_LOG_FILE_MAX_COUNT: &str = "GEORGE_DB_LOG_FILE_MAX_COUNT";
pub const GEORGE_DB_LOG_LEVEL: &str = "GEORGE_DB_LOG_LEVEL";
pub const GEORGE_DB_PRODUCTION: &str = "GEORGE_DB_PRODUCTION";

/// 默认KV存储索引
pub const INDEX_CATALOG: &str = "george_db_index_catalog";
/// 默认自增序列ID索引，不保证连续性，只保证有序性
pub const INDEX_SEQUENCE: &str = "george_db_index_sequence";

/// 索引类型
#[derive(Debug, Clone, Copy)]
pub enum IndexType {
    /// 静态索引方法(static index access method)
    Siam,
}

/// 存储类型
#[derive(Debug, Clone, Copy)]
pub enum Category {
    /// 内存存储类型
    Memory,
    /// 文档存储类型
    Document,
}

/// 获取存储类型
pub(crate) fn category(category: Category) -> Category {
    match category {
        Category::Memory => Category::Memory,
        Category::Document => Category::Document,
    }
}

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

/// 存储量级
#[derive(Debug, Clone, Copy)]
pub enum LevelType {
    /// 低级，支持存储2^32个元素
    Small,
    /// 高级，支持存储2^64个元素
    Large,
}

/// 获取存储量级
pub fn level(level: LevelType) -> LevelType {
    match level {
        LevelType::Small => LevelType::Small,
        LevelType::Large => LevelType::Large,
    }
}

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

pub fn key_fetch(key_structure: String, value: Vec<u8>) -> GeorgeResult<String> {
    match String::from_utf8(value) {
        Ok(value_str) => {
            let res: Result<Value, Error> = serde_json::from_str(value_str.as_ref());
            match res {
                Ok(v) => match v[key_structure.clone()] {
                    Value::Null => Err(err_string(format!(
                        "key structure {} do not support none!",
                        key_structure
                    ))),
                    Value::Object(..) => Err(err_string(format!(
                        "key structure {} do not support object!",
                        key_structure
                    ))),
                    Value::Array(..) => Err(err_string(format!(
                        "key structure {} do not support array!",
                        key_structure
                    ))),
                    Value::Bool(..) => Err(err_string(format!(
                        "key structure {} do not support bool!",
                        key_structure
                    ))),
                    _ => Ok(format!("{}", v[key_structure])),
                },
                Err(err) => Err(err_string(err.to_string())),
            }
        }
        Err(err) => Err(err_string(err.to_string())),
    }
}
