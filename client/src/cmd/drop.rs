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

use crate::cmd::{george_error, Config, Drop};

impl Drop {
    pub(crate) fn analysis(
        config: &Config,
        used: String,
        scan: String,
        vss: Vec<String>,
    ) -> GeorgeResult<()> {
        if vss.len() < 3 {
            return Err(george_error(scan));
        }
        let intent = vss[1].as_str();
        match intent {
            "database" => {
                // drop database [database:string]
                if vss.len() != 3 {
                    return Err(george_error(scan));
                }
                let name = vss[2].clone();
                config.database.remove(name)
            }
            "page" => {
                // drop page [page:string]
                if vss.len() != 3 {
                    return Err(george_error(scan));
                }
                let name = vss[2].clone();
                config.page.remove(name)
            }
            "ledger" => Err(Errs::str("no support ledger now!")),
            "view" => {
                // drop view [view:string]
                if used.is_empty() {
                    return Err(Errs::str(
                        "database name not defined, please use `use [database/page/ledger] [database]` first!",
                    ));
                }
                if vss.len() != 3 {
                    return Err(george_error(scan));
                }
                let name = vss[2].clone();
                config.view.remove(used, name)
            }
            _ => Err(Errs::string(format!(
                "command do not support prefix {} in {}",
                intent, scan
            ))),
        }
    }
}
