use std::env::var;

pub const GEORGE_PORT: &str = "GEORGE_PORT";
pub const GEORGE_DATA_DIR: &str = "GEORGE_DATA_DIR";
pub const GEORGE_LIMIT_OPEN_FILE: &str = "GEORGE_LIMIT_OPEN_FILE";
pub const GEORGE_TLS: &str = "GEORGE_TLS";
pub const GEORGE_TLS_KEY_FILE: &str = "GEORGE_TLS_KEY_FILE";
pub const GEORGE_TLS_CERT_FILE: &str = "GEORGE_TLS_CERT_FILE";
pub const GEORGE_LIMIT: &str = "GEORGE_LIMIT";
pub const GEORGE_LIMIT_MILLISECOND: &str = "GEORGE_LIMIT_MILLISECOND";
pub const GEORGE_LIMIT_COUNT: &str = "GEORGE_LIMIT_COUNT";
pub const GEORGE_LIMIT_INTERVAL_MICROSECOND: &str = "GEORGE_LIMIT_INTERVAL_MICROSECOND";
pub const GEORGE_LOG_DIR: &str = "GEORGE_LOG_DIR";
pub const GEORGE_LOG_FILE_MAX_SIZE: &str = "GEORGE_LOG_FILE_MAX_SIZE";
pub const GEORGE_LOG_FILE_MAX_AGE: &str = "GEORGE_LOG_FILE_MAX_AGE";
pub const GEORGE_LOG_UTC: &str = "GEORGE_LOG_UTC";
pub const GEORGE_LOG_LEVEL: &str = "GEORGE_LOG_LEVEL";
pub const GEORGE_PRODUCTION: &str = "GEORGE_PRODUCTION";
pub const GEORGE_GENESIS_BLOCK_FILE: &str = "GEORGE_GENESIS_BLOCK_FILE";
pub const GEORGE_BLOCK_DIR_PATH: &str = "GEORGE_BLOCK_DIR_PATH";

pub fn get(name: &str, default: &str) -> String {
    match var(name) {
        Ok(res) => res,
        _ => default.to_string(),
    }
}
