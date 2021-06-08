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

use crate::{Config, ConfigDB, ConfigLog, ConfigServer};

impl Config {
    pub fn default() -> Config {
        Config {
            log: Some(ConfigLog::default()),
            db: Some(ConfigDB::default()),
            server: Some(ConfigServer::default()),
        }
    }

    pub(crate) fn check(&mut self) {
        match self.log {
            Some(_) => self.log.clone().unwrap().check(),
            None => self.log = Some(ConfigLog::default()),
        }
        match self.db {
            Some(_) => self.db.clone().unwrap().check(),
            None => self.db = Some(ConfigDB::default()),
        }
        match self.server {
            Some(_) => self.server.clone().unwrap().check(),
            None => self.server = Some(ConfigServer::default()),
        }
    }
}
