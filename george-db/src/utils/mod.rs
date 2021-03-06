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

use serde::{Deserialize, Serialize};

use crate::utils::enums::{Engine, KeyType};

pub mod comm;
mod comm_test;
pub mod deploy;
mod deploy_test;
pub mod enums;
pub mod path;
mod path_test;

pub struct Enum {}

pub trait EnumHandler {
    fn engine_u8(index_type: Engine) -> u8;
    fn key_type_u8(key_type: KeyType) -> u8;
    fn engine(b: u8) -> Engine;
    fn key_type(b: u8) -> KeyType;
}

pub struct Paths;

/// 服务基础配置信息，优先读取环境变量中的结果<p>
///
/// 该配置信息可通过指定路径的文件中进行读取，文件格式支持yaml
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// 服务数据存储路径
    pub data_dir: String,
    /// 限制打开文件描述符次数
    pub thread_count: usize,
}
