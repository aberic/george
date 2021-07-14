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

use crate::{ConfigLog, LogPolicy};

impl ConfigLog {
    pub(crate) fn default() -> ConfigLog {
        ConfigLog {
            log_dir: Some("george-server/src/test/george/log".to_string()),
            log_level: Some("debug".to_string()),
            log_file_max_size: Some(100),
            log_file_max_count: Some(100),
            additive: Some(false),
        }
    }

    pub(crate) fn check(&mut self) {
        match self.log_dir {
            None => self.log_dir = Some("george-server/src/test/george/log".to_string()),
            _ => {}
        }
        match self.log_level {
            None => self.log_level = Some("debug".to_string()),
            _ => {}
        }
        match self.log_file_max_size {
            None => self.log_file_max_size = Some(100),
            _ => {}
        }
        match self.log_file_max_count {
            None => self.log_file_max_count = Some(100),
            _ => {}
        }
        match self.additive {
            None => self.additive = Some(false),
            _ => {}
        }
    }
}

impl LogPolicy {
    /// * dir 日志文件目录
    /// * name 日志文件名
    /// * pkg 日志截取包名，如：`george-db::task::master`
    /// * additive 是否在主日志文件中同步记录
    pub fn new(dir: String, name: String, pkg: String, additive: bool) -> LogPolicy {
        LogPolicy {
            dir,
            name,
            pkg,
            additive,
        }
    }
}
