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

use std::str::Split;
use std::sync::Arc;

use comm::errors::{Errs, GeorgeResult};
use db::Task;

use crate::cmd::Show;

impl Show {
    pub fn analysis(task: Arc<Task>, used: String, vss: Vec<String>) -> GeorgeResult<Vec<u8>> {
        let intent = vss[1].as_str();
        match intent {
            "databases" => {
                let ds = DatabaseServer { task };
                let list = ds.database_list();
                match list.write_to_bytes() {
                    Ok(v8s) => Ok(v8s),
                    Err(err) => Err(Errs::strs("struct write_to_bytes", err)),
                }
            }
            "pages" => Ok("show pages success!".as_bytes().to_vec()),
            "ledgers" => Ok("show ledgers success!".as_bytes().to_vec()),
            "views" => Ok("show views success!".as_bytes().to_vec()),
            _ => Err(Errs::string(format!(
                "command not support prefix {}",
                intent
            ))),
        }
    }
}
