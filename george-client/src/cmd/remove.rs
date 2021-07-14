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

use crate::cmd::{george_error, Config, Remove};

impl Remove {
    pub(crate) fn analysis(
        config: &mut Config,
        disk: bool,
        used: String,
        scan: String,
        vss: Vec<String>,
    ) -> GeorgeResult<()> {
        let len = vss.len();
        if len < 2 {
            return Err(george_error(scan));
        }
        if disk {
            // remove [view:string] [key:string]
            // remove [view:string] [key:string]
            if used.is_empty() {
                return Err(Errs::str(
                    "database name not defined, please use `use [database/page/ledger] [database]` first!",
                ));
            }
            if len != 3 {
                return Err(george_error(scan));
            }
            let view_name = vss[1].clone();
            let key = vss[2].clone();
            config.disk.remove(used, view_name, key)
        } else {
            // remove [key:string]
            // remove [key:string]
            if len != 2 {
                return Err(george_error(scan));
            }
            let key = vss[1].clone();
            if used.is_empty() {
                config.memory.remove(key)
            } else {
                config.memory.remove_by_page(used, key)
            }
        }
    }
}
