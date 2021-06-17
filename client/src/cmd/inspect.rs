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

use cli_table::format::Justify;
use cli_table::{print_stdout, Cell, Style, Table};

use comm::errors::{Errs, GeorgeResult};
use protocols::impls::utils::Comm;

use crate::cmd::{george_error, print_table, Config, Inspect};
use db::utils::comm::INDEX_INCREMENT;

impl Inspect {
    pub(crate) fn analysis(
        config: &Config,
        used: String,
        scan: String,
        vss: Vec<String>,
    ) -> GeorgeResult<()> {
        let intent = vss[1].as_str();
        match intent {
            "database" => {
                // inspect database [database:string]
                let name: String;
                if vss.len() == 3 {
                    name = vss[2].clone();
                } else {
                    return Err(Errs::str("database name is none!"));
                }
                let database = config.database.info(name)?;
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
                print_table(table)
            }
            "page" => {
                // inspect page [page:string]
                let name: String;
                if vss.len() == 3 {
                    name = vss[2].clone();
                } else {
                    return Err(Errs::str("page name is none!"));
                }
                let page = config.page.info(name)?;
                let table = vec![vec![
                    page.get_name().cell(),
                    page.get_comment().cell(),
                    page.get_size().cell(),
                    page.get_period().cell(),
                    Comm::proto_grpc_timestamp_2_time(page.get_create_time().seconds)
                        .to_string("%Y-%m-%d %H:%M:%S")
                        .cell()
                        .justify(Justify::Right),
                ]]
                .table()
                .title(vec![
                    "Name".cell().bold(true),
                    "Comment".cell().bold(true),
                    "Size".cell().bold(true),
                    "Period".cell().bold(true),
                    "Create Time".cell().bold(true),
                ])
                .bold(true);
                print_table(table)
            }
            "ledger" => Err(Errs::str("no support ledger now!")),
            "view" => {
                // inspect view [view]
                if used.is_empty() {
                    return Err(Errs::str(
                        "database name not defined, please use `use [database/page/ledger] [database]` first!",
                    ));
                }
                if vss.len() != 3 {
                    return Err(george_error(scan));
                }
                let name: String;
                if vss.len() == 3 {
                    name = vss[2].clone();
                } else {
                    return Err(Errs::str("page name is none!"));
                }
                let view = config.view.info(used, name)?;
                let mut increment = false;
                for index in view.indexes.iter() {
                    if index.name.eq(INDEX_INCREMENT) {
                        increment = true;
                    }
                }
                let table = vec![vec![
                    view.get_name().cell(),
                    view.get_comment().cell(),
                    increment.cell(),
                    Comm::proto_grpc_timestamp_2_time(view.get_create_time().seconds)
                        .to_string("%Y-%m-%d %H:%M:%S")
                        .cell(),
                    view.get_indexes().len().cell().justify(Justify::Right),
                ]]
                .table()
                .title(vec![
                    "Name".cell().bold(true),
                    "Comment".cell().bold(true),
                    "Increment".cell().bold(true),
                    "Create Time".cell().bold(true),
                    "View Count".cell().bold(true),
                ])
                .bold(true);
                print_table(table)
            }
            "index" => {
                // inspect index [index:string] from [view:string]
                if used.is_empty() {
                    return Err(Errs::str(
                        "database name not defined, please use `use [database/page/ledger] [database]` first!",
                    ));
                }
                if vss[3].ne("from") {
                    return Err(george_error(scan));
                }
                let name: String;
                let view_name: String;
                if vss.len() == 5 {
                    name = vss[2].clone();
                    view_name = vss[4].clone();
                } else {
                    return Err(george_error(scan));
                }
                let index = config.index.info(used, view_name, name)?;
                let table = vec![vec![
                    index.get_name().cell(),
                    index.get_unique().cell(),
                    index.get_primary().cell(),
                    index.get_null().cell(),
                    Comm::key_type_str(index.get_key_type()).cell(),
                    Comm::engine_str(index.get_engine()).cell(),
                    Comm::proto_grpc_timestamp_2_time(index.get_create_time().seconds)
                        .to_string("%Y-%m-%d %H:%M:%S")
                        .cell()
                        .justify(Justify::Right),
                ]]
                .table()
                .title(vec![
                    "Name".cell().bold(true),
                    "Unique".cell().bold(true),
                    "Primary".cell().bold(true),
                    "Null".cell().bold(true),
                    "Key Type".cell().bold(true),
                    "Engine".cell().bold(true),
                    "Create Time".cell().bold(true),
                ])
                .bold(true);
                print_table(table)
            }
            _ => Err(Errs::string(format!(
                "command do not support prefix {}",
                intent
            ))),
        }
    }
}
