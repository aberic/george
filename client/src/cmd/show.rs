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

use crate::cmd::{Config, Show};
use cli_table::format::Justify;
use cli_table::{print_stdout, Cell, Style, Table};
use protocols::impls::utils::Comm;

impl Show {
    pub(crate) fn analysis(config: &Config, used: String, vss: Vec<String>) -> GeorgeResult<()> {
        let intent = vss[1].as_str();
        match intent {
            "databases" => {
                let db_list = config.database.databases()?;
                let list = db_list.databases;
                let mut table = vec![];
                for db in list.iter() {
                    table.push(vec![
                        db.get_name().cell(),
                        db.get_comment().cell(),
                        Comm::proto_grpc_timestamp_2_time(db.get_create_time().seconds)
                            .to_string("%Y-%m-%d %H:%M:%S")
                            .cell()
                            .justify(Justify::Right),
                    ])
                }
                match print_stdout(
                    table
                        .table()
                        .title(vec![
                            "Name".cell().bold(true),
                            "Comment".cell().bold(true),
                            "Create Time".cell().bold(true),
                        ])
                        .bold(true),
                ) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(Errs::strs("print stdout", err)),
                }
            }
            "pages" => Err(Errs::str("no support pages now!")),
            "ledgers" => Err(Errs::str("no support ledgers now!")),
            "views" => Err(Errs::str("no support views now!")),
            _ => Err(Errs::string(format!(
                "command do not support prefix {}",
                intent
            ))),
        }
    }
}
