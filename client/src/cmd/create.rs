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

use comm::errors::{Errs, GeorgeResult};

use crate::cmd::{Config, Create};

impl Create {
    pub(crate) fn analysis(config: &Config, _used: String, vss: Vec<String>) -> GeorgeResult<()> {
        let intent = vss[1].as_str();
        match intent {
            "database" => {
                let name: String;
                let mut comment: String = "".to_string();
                if vss.len() == 3 {
                    name = vss[2].clone();
                } else if vss.len() == 4 {
                    name = vss[2].clone();
                    comment = vss[3].clone();
                } else {
                    return Err(Errs::str("database name is none!"));
                }
                match config.database.create(name, comment) {
                    Ok(()) => {
                        println!("create database {} success!", intent);
                        Ok(())
                    }
                    Err(err) => Err(Errs::strs("database create", err)),
                }
            }
            "page" => {
                let name: String;
                let mut comment: String = "".to_string();
                if vss.len() == 3 {
                    name = vss[2].clone();
                } else if vss.len() == 4 {
                    name = vss[2].clone();
                    comment = vss[3].clone();
                } else {
                    return Err(Errs::str("page name is none!"));
                }
                match config.database.create(name, comment) {
                    Ok(()) => {
                        println!("create database {} success!", intent);
                        Ok(())
                    }
                    Err(err) => Err(Errs::strs("database create", err)),
                }
            }
            "ledger" => Err(Errs::str("no support ledger now!")),
            "view" => Err(Errs::str("no support view now!")),
            "index" => Err(Errs::str("no support index now!")),
            _ => Err(Errs::string(format!(
                "command do not support prefix {}",
                intent
            ))),
        }
    }
}
