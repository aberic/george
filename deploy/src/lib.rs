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

use logs::LogModule;
use serde::{Deserialize, Serialize};

mod builder;
pub mod comm;
mod conf;
mod conf_test;
mod config;
mod db;
mod init;
mod log;
mod server;

pub struct Builder;

#[derive(Debug, PartialEq, Clone)]
pub struct Init {
    conf: Conf,
    log_main: LogModule,
}

/// yaml解析辅助结构
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Conf {
    config: Option<Config>,
}

/// 基础配置信息，优先读取环境变量中的结果<p>
///
/// 该配置信息可通过指定路径的文件中进行读取，文件格式支持yaml
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    log: Option<ConfigLog>,
    db: Option<ConfigDB>,
    server: Option<ConfigServer>,
}

/// 日志配置信息，优先读取环境变量中的结果<p>
///
/// 该配置信息可通过指定路径的文件中进行读取，文件格式支持yaml
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ConfigLog {
    /// 日志文件目录
    pub log_dir: Option<String>,
    /// 日志级别(debug/info/warn/Error/panic/fatal)
    pub log_level: Option<String>,
    /// 每个日志文件保存的最大尺寸 单位：M
    pub log_file_max_size: Option<u64>,
    /// 文件最多保存多少个
    pub log_file_max_count: Option<u32>,
    /// 是否在主日志文件中同步记录
    pub additive: Option<bool>,
}

/// 模块日志策略
pub struct LogPolicy {
    /// 日志文件目录
    dir: String,
    /// 日志文件名
    name: String,
    /// 日志截取包名，如：`db::task::master`
    pkg: String,
}

///  数据库配置信息，优先读取环境变量中的结果<p>
///
/// 该配置信息可通过指定路径的文件中进行读取，文件格式支持yaml
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ConfigDB {
    /// 服务数据存储路径
    pub data_dir: Option<String>,
    /// 限制打开文件描述符次数
    pub thread_count: Option<usize>,
}

/// 服务配置信息，优先读取环境变量中的结果<p>
///
/// 该配置信息可通过指定路径的文件中进行读取，文件格式支持yaml
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ConfigServer {
    /// 服务端口号
    pub port: Option<u16>,
}
