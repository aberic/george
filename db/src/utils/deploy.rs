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

use deploy::ConfigDB;

use crate::utils::Config;

pub const VERSION: [u8; 2] = [0x00, 0x00];

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Conf {
    conf: Config,
}

impl Config {
    pub(crate) fn init(&mut self, config_db: ConfigDB) {
        self.data_dir = match config_db.data_dir {
            Some(dir) => dir,
            None => "db/src/test/george".to_string(),
        };
        self.thread_count = match config_db.thread_count {
            Some(count) => count,
            None => 100,
        };
    }
    /// 服务数据存储路径
    pub(crate) fn data_dir(&self) -> String {
        self.data_dir.clone()
    }
    /// 限制打开文件描述符次数
    pub(crate) fn thread_count(&self) -> usize {
        self.thread_count
    }
}

pub static GLOBAL_CONFIG: Lazy<RwLock<Config>> = Lazy::new(|| {
    let config = Config {
        data_dir: "db/src/test/george".to_string(),
        thread_count: 100,
    };
    RwLock::new(config)
});
