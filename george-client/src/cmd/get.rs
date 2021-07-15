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
use george_comm::strings::StringHandler;
use george_comm::Strings;

use crate::cmd::{george_error, Client, Get};

impl Get {
    pub(crate) fn analysis(
        client: &mut Client,
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
            // get [view:string] [key:string]
            // get [view:string] [key:string]
            // get [view:string] [key:string] [index:string]
            // get [view:string] [key:string] [index:string]
            if used.is_empty() {
                return Err(Errs::str(
                    "database name not defined, please use `use [database/page/ledger] [database]` first!",
                ));
            }
            if len == 3 {
                let view_name = vss[1].clone();
                let key = vss[2].clone();
                let value = client.disk.get(used, view_name, key)?;
                println!("{}", Strings::from_utf8(value)?);
                Ok(())
            } else if len == 4 {
                let view_name = vss[1].clone();
                let key = vss[2].clone();
                let index_name = vss[3].clone();
                let value = client
                    .disk
                    .fetch_by_index(used, view_name, index_name, key)?;
                println!("{}", Strings::from_utf8(value)?);
                Ok(())
            } else {
                return Err(george_error(scan));
            }
        } else {
            // get [key:string]
            // get [key:string]
            if len != 2 {
                return Err(george_error(scan));
            }
            let key = vss[1].clone();
            if used.is_empty() {
                let value = client.memory.get(key)?;
                println!("{}", Strings::from_utf8(value)?);
            } else {
                let value = client.memory.fetch_by_page(used, key)?;
                println!("{}", Strings::from_utf8(value)?);
            }
            Ok(())
        }
    }
}
