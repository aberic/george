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

use george_comm::errors::{Errs, GeorgeResult};
use george_rpc::protos::db::db::{Engine, KeyType};
use george_rpc::tools::Trans;

use crate::cmd::{george_error, george_errors, Client, Create};

impl Create {
    pub(crate) fn analysis(
        client: &mut Client,
        used: String,
        scan: String,
        vss: Vec<String>,
    ) -> GeorgeResult<()> {
        let intent = vss[1].as_str();
        match intent {
            "database" => {
                // create database [database:string]
                // create database [database:string] [comment:string]
                let name: String;
                let comment: String;
                if vss.len() == 3 {
                    name = vss[2].clone();
                    comment = "".to_string();
                } else if vss.len() == 4 {
                    name = vss[2].clone();
                    comment = vss[3].clone();
                } else {
                    return Err(george_error(scan));
                }
                client.database.create(name, comment)
            }
            "page" => {
                // create page [page:string]
                // create page [page:string] [comment:string] [size:string] [period:string]
                let name: String;
                let comment: String;
                // 可使用内存大小(单位：Mb，0：不限制大小)
                let mut size = 0;
                // 默认有效期(单位：秒)，如无设置，默认维300(0：永久有效)
                let mut period = 300;
                if vss.len() == 3 {
                    name = vss[2].clone();
                    comment = "".to_string();
                } else if vss.len() == 4 {
                    name = vss[2].clone();
                    comment = vss[3].clone();
                } else if vss.len() == 5 {
                    name = vss[2].clone();
                    comment = vss[3].clone();
                    size = vss[4].clone().parse::<u64>().unwrap();
                } else if vss.len() == 6 {
                    name = vss[2].clone();
                    comment = vss[3].clone();
                    size = vss[4].clone().parse::<u64>().unwrap();
                    period = vss[5].clone().parse::<u32>().unwrap();
                } else {
                    return Err(george_error(scan));
                }
                client.page.create(name, comment, size, period)
            }
            "ledger" => Err(Errs::str("no support ledger now!")),
            "view" => {
                // create view [view:string] [increment:bool]
                // create view [view:string] [increment:bool] [comment:string]
                if used.is_empty() {
                    return Err(Errs::str(
                        "database name not defined, please use `use [database/page/ledger] [database]` first!",
                    ));
                }
                let name = vss[2].clone();
                let increment: bool;
                let comment: String;
                if vss.len() == 4 {
                    comment = "".to_string();
                } else if vss.len() == 5 {
                    comment = vss[4].clone();
                } else {
                    return Err(george_error(scan));
                }
                match vss[3].parse::<bool>() {
                    Ok(b) => increment = b,
                    Err(err) => return Err(george_errors(scan, err)),
                }
                client.view.create(used, name, comment, increment)
            }
            "index" => {
                // create index [index:string] from [view:string] [primary:bool] [unique:bool] [null:bool] [key_type:string] [engine:string]
                // create index [index:string] from [view:string]
                if used.is_empty() {
                    return Err(Errs::str(
                        "database name not defined, please use `use [database/page/ledger] [database]` first!",
                    ));
                }
                let name = vss[2].clone();
                let view_name = vss[4].clone();
                let primary: bool;
                let unique: bool;
                let null: bool;
                let key_type: KeyType;
                let engine: Engine;
                if vss.len() == 5 {
                    primary = false;
                    unique = false;
                    null = true;
                    key_type = KeyType::String;
                    engine = Engine::Disk;
                } else if vss.len() == 10 {
                    match vss[5].parse::<bool>() {
                        Ok(b) => primary = b,
                        Err(err) => return Err(george_errors(scan, err)),
                    }
                    match vss[6].parse::<bool>() {
                        Ok(b) => unique = b,
                        Err(err) => return Err(george_errors(scan, err)),
                    }
                    match vss[7].parse::<bool>() {
                        Ok(b) => null = b,
                        Err(err) => return Err(george_errors(scan, err)),
                    }
                    key_type = Trans::key_type_from_str(vss[8].clone());
                    engine = Trans::engine_from_str(vss[9].clone());
                } else {
                    return Err(george_error(scan));
                }
                client.index.create(
                    used, view_name, name, unique, primary, null, key_type, engine,
                )
            }
            _ => Err(Errs::string(format!(
                "command do not support prefix {} in {}",
                intent, scan
            ))),
        }
    }
}
