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

use crate::{Conf, Config, ConfigDB, ConfigLog, ConfigServer};
use george_comm::errors::{Errs, GeorgeResult};
use george_comm::io::file::{FilerHandler, FilerReader};
use george_comm::io::Filer;
use std::path::Path;

impl Conf {
    pub(crate) fn from<P: AsRef<Path>>(filepath: P) -> GeorgeResult<Conf> {
        match Filer::read_bytes(&filepath) {
            Ok(conf_yaml_bytes) => match serde_yaml::from_slice(&conf_yaml_bytes) {
                Ok(res) => {
                    let mut conf: Conf = res;
                    conf.check();
                    Ok(conf)
                }
                Err(err) => Err(Errs::strs("serde yaml", err)),
            },
            _ => Err(Errs::string(format!(
                "No config file match in path {}",
                Filer::absolute(filepath).unwrap()
            ))),
        }
    }

    pub(crate) fn config(&self) -> Option<Config> {
        self.config.clone()
    }

    pub(crate) fn log(&self) -> Option<ConfigLog> {
        match self.config.clone() {
            Some(config) => config.log,
            None => None,
        }
    }

    pub(crate) fn db(&self) -> Option<ConfigDB> {
        match self.config.clone() {
            Some(config) => config.db,
            None => None,
        }
    }

    pub(crate) fn server(&self) -> Option<ConfigServer> {
        match self.config.clone() {
            Some(config) => config.server,
            None => None,
        }
    }

    pub(crate) fn log_unwrap(&self) -> ConfigLog {
        self.config()
            .expect("It's not gonna happen!")
            .log
            .expect("It's not gonna happen!")
    }

    pub(crate) fn check(&mut self) {
        match self.config() {
            Some(mut res) => {
                res.check();
                self.config = Some(res)
            }
            None => self.config = Some(Config::default()),
        }
    }
}
