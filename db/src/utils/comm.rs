pub const GEORGE_DB_CONFIG: &str = "GEORGE_DB_CONFIG";
pub const GEORGE_DB_DATA_DIR: &str = "GEORGE_DB_DATA_DIR";
pub const GEORGE_DB_LIMIT_OPEN_FILE: &str = "GEORGE_DB_LIMIT_OPEN_FILE";
pub const GEORGE_DB_LOG_DIR: &str = "GEORGE_DB_LOG_DIR";
pub const GEORGE_DB_LOG_FILE_MAX_SIZE: &str = "GEORGE_DB_LOG_FILE_MAX_SIZE";
pub const GEORGE_DB_LOG_FILE_MAX_COUNT: &str = "GEORGE_DB_LOG_FILE_MAX_COUNT";
pub const GEORGE_DB_LOG_LEVEL: &str = "GEORGE_DB_LOG_LEVEL";
pub const GEORGE_DB_PRODUCTION: &str = "GEORGE_DB_PRODUCTION";

pub const INDEX_CATALOG: &str = "george_db_index_catalog";
pub const INDEX_SEQUENCE: &str = "george_db_index_sequence";

#[derive(Debug, Clone, Copy)]
pub enum IndexType {
    Siam,
}

#[derive(Debug, Clone, Copy)]
pub enum Category {
    Memory,
    Document,
}

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

#[derive(Debug, Clone, Copy)]
pub enum LevelType {
    Small,
    Large,
}

pub fn level(level: LevelType) -> LevelType {
    match level {
        LevelType::Small => LevelType::Small,
        LevelType::Large => LevelType::Large,
    }
}

pub fn level_distance_32(level: u8) -> u32 {
    if level == 1 {
        return LEVEL1DISTANCE32;
    } else if level == 2 {
        return LEVEL2DISTANCE32;
    } else if level == 3 {
        return LEVEL3DISTANCE32;
    } else if level == 4 {
        return LEVEL4DISTANCE32;
    }
    return 0;
}

pub fn level_distance_64(level: u8) -> u64 {
    if level == 1 {
        return LEVEL1DISTANCE64;
    } else if level == 2 {
        return LEVEL2DISTANCE64;
    } else if level == 3 {
        return LEVEL3DISTANCE64;
    } else if level == 4 {
        return LEVEL4DISTANCE64;
    }
    return 0;
}
