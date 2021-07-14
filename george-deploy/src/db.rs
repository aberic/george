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

use crate::ConfigDB;

impl ConfigDB {
    pub fn default() -> ConfigDB {
        ConfigDB {
            data_dir: Some("george-server/src/test/george".to_string()),
            thread_count: Some(10),
        }
    }

    pub fn new(data_dir: String, thread_count: usize) -> ConfigDB {
        ConfigDB {
            data_dir: Some(data_dir),
            thread_count: Some(thread_count),
        }
    }

    pub(crate) fn check(&mut self) {
        match self.data_dir {
            None => self.data_dir = Some("george-server/src/test/george".to_string()),
            _ => {}
        }
        match self.thread_count {
            None => self.thread_count = Some(10),
            _ => {}
        }
    }
}
