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

use std::sync::RwLock;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use comm::io::file::FilerReader;
use comm::io::Filer;
use comm::Env;

use crate::utils::comm::{
    GEORGE_DB_DATA_DIR, GEORGE_DB_LIMIT_OPEN_FILE, GEORGE_DB_LOG_DIR, GEORGE_DB_LOG_FILE_MAX_COUNT,
    GEORGE_DB_LOG_FILE_MAX_SIZE, GEORGE_DB_LOG_LEVEL, GEORGE_DB_PRODUCTION,
};

pub const VERSION: [u8; 2] = [0x00, 0x00];

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Conf {
    conf: Config,
}

/// 服务基础配置信息，优先读取环境变量中的结果<p>
///
/// 该配置信息可通过指定路径的文件中进行读取，文件格式支持yaml
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// 服务数据存储路径
    pub data_dir: String,
    /// 限制打开文件描述符次数
    pub limit_open_file: u16,
    /// 日志文件目录
    pub log_dir: String,
    /// 日志级别(debug/info/warn/Error/panic/fatal)
    pub log_level: String,
    /// 每个日志文件保存的最大尺寸 单位：M
    pub log_file_max_size: u64,
    /// 文件最多保存多少个
    pub log_file_max_count: u32,
    /// 是否生产环境，在生产环境下控制台不会输出任何日志
    pub production: bool,
}

impl Config {
    /// 服务数据存储路径
    pub fn data_dir(&self) -> String {
        self.data_dir.clone()
    }
    /// 限制打开文件描述符次数
    pub fn limit_open_file(&self) -> u16 {
        self.limit_open_file
    }
    /// 日志文件目录
    pub fn log_dir(&self) -> String {
        self.log_dir.clone()
    }
    /// 日志级别(debug/info/warn/Error/panic/fatal)
    pub fn log_level(&self) -> String {
        self.log_level.clone()
    }
    /// 每个日志文件保存的最大尺寸 单位：M
    pub fn log_file_max_size(&self) -> u64 {
        self.log_file_max_size
    }
    /// 文件最多保存多少个
    pub fn log_file_max_count(&self) -> u32 {
        self.log_file_max_count
    }
    /// 是否生产环境，在生产环境下控制台不会输出任何日志
    pub fn production(&self) -> bool {
        self.production
    }
}

pub static GLOBAL_CONFIG: Lazy<RwLock<Config>> = Lazy::new(|| {
    let config = Config {
        data_dir: "db/src/test/george".to_string(),
        limit_open_file: 100,
        log_dir: "src/test".to_string(),
        log_level: "debug".to_string(),
        log_file_max_size: 1024,
        log_file_max_count: 7,
        production: false,
    };
    RwLock::new(config)
});

pub fn init_config(filepath: String) {
    match Filer::read(filepath.clone()) {
        Ok(config_yaml_str_res) => {
            let conf: Conf;
            match serde_yaml::from_str(&config_yaml_str_res) {
                Ok(conf_own) => {
                    conf = conf_own;
                }
                Err(err) => {
                    println!("err = {}", err);
                    return;
                }
            }
            let mut config = GLOBAL_CONFIG.write().unwrap();
            config.data_dir = config_value(GEORGE_DB_DATA_DIR, &conf.conf.data_dir);
            config.limit_open_file = config_value(
                GEORGE_DB_LIMIT_OPEN_FILE,
                &conf.conf.limit_open_file.to_string(),
            )
            .parse::<u16>()
            .expect("config GEORGE_DB_LIMIT_OPEN_FILE type error");
            config.log_dir = config_value(GEORGE_DB_LOG_DIR, &conf.conf.log_dir);
            config.log_file_max_size = config_value(
                GEORGE_DB_LOG_FILE_MAX_SIZE,
                &conf.conf.log_file_max_size.to_string(),
            )
            .parse::<u64>()
            .expect("config GEORGE_DB_LOG_FILE_MAX_SIZE type error");
            config.log_file_max_count = config_value(
                GEORGE_DB_LOG_FILE_MAX_COUNT,
                &conf.conf.log_file_max_count.to_string(),
            )
            .parse::<u32>()
            .expect("config GEORGE_DB_LOG_FILE_MAX_AGE type error");
            config.log_level = config_value(GEORGE_DB_LOG_LEVEL, &conf.conf.log_level);
            config.production =
                config_value(GEORGE_DB_PRODUCTION, &conf.conf.production.to_string())
                    .parse::<bool>()
                    .expect("config GEORGE_DB_PRODUCTION type error");
        }
        _ => {
            log::info!("No config file match in path {}", filepath);
        }
    }
}

fn config_value(env_name: &str, default: &str) -> String {
    let res = Env::get(env_name, default);
    if res.is_empty() {
        config_default(env_name)
    } else {
        res
    }
}

fn config_default(env_name: &str) -> String {
    if env_name.eq(GEORGE_DB_DATA_DIR) {
        String::from("/var/lib/db")
    } else if env_name.eq(GEORGE_DB_LIMIT_OPEN_FILE) {
        String::from("100")
    } else if env_name.eq(GEORGE_DB_LOG_DIR) {
        String::from("/var/lib/db/log")
    } else if env_name.eq(GEORGE_DB_LOG_FILE_MAX_SIZE) {
        String::from("1024")
    } else if env_name.eq(GEORGE_DB_LOG_FILE_MAX_COUNT) {
        String::from("7")
    } else if env_name.eq(GEORGE_DB_LOG_LEVEL) {
        String::from("debug")
    } else if env_name.eq(GEORGE_DB_PRODUCTION) {
        String::from("false")
    } else {
        String::from("")
    }
}
