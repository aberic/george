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

use crate::cmd::{george_error, Alter, Config};

impl Alter {
    pub(crate) fn analysis(
        config: &Config,
        used: String,
        scan: String,
        vss: Vec<String>,
    ) -> GeorgeResult<()> {
        let intent = vss[1].as_str();
        match intent {
            "database" => {
                // alter database [database:string] [database:string]
                // alter database [database:string] [database:string] [comment:string]
                let comment_new: String;
                if vss.len() == 4 {
                    comment_new = "".to_string();
                } else if vss.len() == 5 {
                    comment_new = vss[4].clone();
                } else {
                    return Err(george_error(scan));
                }
                let name = vss[2].clone();
                let name_new = vss[3].clone();
                config.database.modify(name, comment_new, name_new)
            }
            "page" => {
                // alter page [page:string] [page:string]
                if vss.len() != 4 {
                    return Err(george_error(scan));
                }
                let name = vss[2].clone();
                let name_new = vss[3].clone();
                config.page.modify(name, name_new)
            }
            "ledger" => Err(Errs::str("no support ledger now!")),
            "view" => {
                // alter view [view:string] [view:string]
                // alter view [view:string] [view:string] [comment:string]
                if used.is_empty() {
                    return Err(Errs::str(
                        "database name not defined, please use `use [database/page/ledger] [database]` first!",
                    ));
                }
                let comment: String;
                if vss.len() == 4 {
                    comment = "".to_string();
                } else if vss.len() == 5 {
                    comment = vss[4].clone();
                } else {
                    return Err(george_error(scan));
                }
                let name = vss[2].clone();
                let name_new = vss[3].clone();
                config.view.modify(used, name, name_new, comment)
            }
            "archive" => {
                // alter archive [view:string] [filepath:String]
                if used.is_empty() {
                    return Err(Errs::str(
                        "database name not defined, please use `use [database/page/ledger] [database]` first!",
                    ));
                }
                if vss.len() != 4 {
                    return Err(george_error(scan));
                }
                let name = vss[2].clone();
                let filepath = vss[3].clone();
                config.view.archive(used, name, filepath)
            }
            _ => Err(Errs::string(format!(
                "command do not support prefix {}",
                intent
            ))),
        }
    }
}
