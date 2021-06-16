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

use crate::cmd::{Config, Info};
use cli_table::format::Justify;
use cli_table::{print_stdout, Cell, Style, Table};
use protocols::impls::utils::Comm;

impl Info {
    pub(crate) fn analysis(config: &Config, _used: String, vss: Vec<String>) -> GeorgeResult<()> {
        let intent = vss[1].as_str();
        match intent {
            "database" => {
                let name: String;
                if vss.len() == 3 {
                    name = vss[2].clone();
                } else if vss.len() == 4 {
                    name = vss[2].clone();
                } else {
                    return Err(Errs::str("database name is none!"));
                }
                match config.database.info(name) {
                    Ok(database) => {
                        let table = vec![vec![
                            database.get_name().cell(),
                            database.get_comment().cell(),
                            Comm::proto_grpc_timestamp_2_time(database.get_create_time().seconds)
                                .to_string("%Y-%m-%d %H:%M:%S")
                                .cell(),
                            database.get_views().len().cell().justify(Justify::Right),
                        ]]
                        .table()
                        .title(vec![
                            "Name".cell().bold(true),
                            "Comment".cell().bold(true),
                            "Create Time".cell().bold(true),
                            "View Count".cell().bold(true),
                        ])
                        .bold(true);
                        match print_stdout(table) {
                            Ok(()) => Ok(()),
                            Err(err) => Err(Errs::strs("print stdout", err)),
                        }
                    }
                    Err(err) => Err(Errs::strs("database info", err)),
                }
            }
            "page" => Err(Errs::str("no support page now!")),
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
