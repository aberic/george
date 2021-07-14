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
use cli_table::{Cell, Style, Table};

use george_comm::errors::{Errs, GeorgeResult};
use george_db::utils::comm::INDEX_INCREMENT;
use george_rpc::tools::Trans;

use crate::cmd::{george_error, print_table, Config, Inspect};

impl Inspect {
    pub(crate) fn analysis(
        config: &mut Config,
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
                    database.name.clone().cell(),
                    database.comment.clone().cell(),
                    Trans::grpc_timestamp_2_string(database.create_time.as_ref().unwrap().seconds)
                        .cell(),
                    database.views.len().cell().justify(Justify::Right),
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
                    page.name.clone().cell(),
                    page.comment.clone().cell(),
                    page.size.cell(),
                    page.period.cell(),
                    Trans::grpc_timestamp_2_string(page.create_time.as_ref().unwrap().seconds)
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
                        break;
                    }
                }
                let table = vec![vec![
                    view.name.clone().cell(),
                    view.comment.clone().cell(),
                    increment.cell(),
                    Trans::grpc_timestamp_2_string(view.create_time.as_ref().unwrap().seconds)
                        .cell(),
                    view.indexes.len().cell().justify(Justify::Right),
                    view.filepath.cell(),
                    view.version.cell(),
                ]]
                .table()
                .title(vec![
                    "Name".cell().bold(true),
                    "Comment".cell().bold(true),
                    "Increment".cell().bold(true),
                    "Create Time".cell().bold(true),
                    "Index Count".cell().bold(true),
                    "Filepath".cell().bold(true),
                    "Version".cell().bold(true),
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
                    index.name.clone().cell(),
                    index.unique.cell(),
                    index.primary.cell(),
                    index.null.cell(),
                    Trans::i32_2_key_type_str(index.key_type)?.cell(),
                    Trans::i32_2_engine_str(index.engine)?.cell(),
                    Trans::grpc_timestamp_2_string(index.create_time.as_ref().unwrap().seconds)
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
                "command do not support prefix {} in {}",
                intent, scan
            ))),
        }
    }
}
