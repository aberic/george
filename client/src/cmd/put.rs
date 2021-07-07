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

use crate::cmd::{george_error, Config, Put};

impl Put {
    pub(crate) fn analysis(
        config: &mut Config,
        disk: bool,
        used: String,
        scan: String,
        vss: Vec<String>,
    ) -> GeorgeResult<()> {
        let len = vss.len();
        if len < 3 {
            return Err(george_error(scan));
        }
        if disk {
            // put [view:string] [key:string] [value:string]
            // put [view:string] [key:string] [value:string]
            if used.is_empty() {
                return Err(Errs::str(
                    "database name not defined, please use `use [database/page/ledger] [database]` first!",
                ));
            }
            if len != 4 {
                return Err(george_error(scan));
            }
            let view_name = vss[1].clone();
            let key = vss[2].clone();
            let value = vss[3].as_bytes().to_vec();
            config.disk.put(used, view_name, key, value)
        } else {
            // put [key:string] [value:string]
            // put [key:string] [value:string]
            if len != 3 {
                return Err(george_error(scan));
            }
            let key = vss[1].clone();
            let value = vss[2].as_bytes().to_vec();
            if used.is_empty() {
                config.memory.put(key, value)
            } else {
                config.memory.put_by_page(used, key, value)
            }
        }
    }
}
