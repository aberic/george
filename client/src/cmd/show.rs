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

use comm::errors::{Errs, GeorgeResult};
use rpc::tools::Trans;

use crate::cmd::{george_error, print_table, Config, Show};

impl Show {
    pub(crate) fn analysis(
        config: &mut Config,
        disk: bool,
        used: String,
        scan: String,
        vss: Vec<String>,
    ) -> GeorgeResult<()> {
        let len = vss.len();
        if len == 1 {
            let table = vec![vec![used.cell(), disk.cell().justify(Justify::Right)]]
                .table()
                .title(vec!["Used".cell().bold(true), "Disk".cell().bold(true)])
                .bold(true);
            return print_table(table);
        }
        let intent = vss[1].as_str();
        match intent {
            "databases" => {
                // show databases;
                let list = config.database.list()?;
                let mut table = vec![];
                for db in list.iter() {
                    table.push(vec![
                        db.name.clone().cell(),
                        db.comment.clone().cell(),
                        Trans::grpc_timestamp_2_string(db.create_time.as_ref().unwrap().seconds)
                            .cell(),
                        db.views.len().cell().justify(Justify::Right),
                    ])
                }
                print_table(
                    table
                        .table()
                        .title(vec![
                            "Name".cell().bold(true),
                            "Comment".cell().bold(true),
                            "Create Time".cell().bold(true),
                            "View Count".cell().bold(true),
                        ])
                        .bold(true),
                )
            }
            "pages" => {
                // show pages;
                let list = config.page.list()?;
                let mut table = vec![];
                for page in list.iter() {
                    table.push(vec![
                        page.name.clone().cell(),
                        page.comment.clone().cell(),
                        page.size.cell(),
                        page.period.cell(),
                        Trans::grpc_timestamp_2_string(page.create_time.as_ref().unwrap().seconds)
                            .cell()
                            .justify(Justify::Right),
                    ])
                }
                print_table(
                    table
                        .table()
                        .title(vec![
                            "Name".cell().bold(true),
                            "Comment".cell().bold(true),
                            "Size".cell().bold(true),
                            "Period".cell().bold(true),
                            "Create Time".cell().bold(true),
                        ])
                        .bold(true),
                )
            }
            "ledgers" => Err(Errs::str("no support ledgers now!")),
            "views" => {
                // show views;
                if used.is_empty() {
                    return Err(Errs::str(
                        "database name not defined, please use `use [database/page/ledger] [database]` first!",
                    ));
                }
                let list = config.view.list(used)?;
                let mut table = vec![];
                for view in list.iter() {
                    table.push(vec![
                        view.name.clone().cell(),
                        view.comment.clone().cell(),
                        Trans::grpc_timestamp_2_string(view.create_time.as_ref().unwrap().seconds)
                            .cell(),
                        view.indexes.len().cell().justify(Justify::Right),
                    ])
                }
                print_table(
                    table
                        .table()
                        .title(vec![
                            "Name".cell().bold(true),
                            "Comment".cell().bold(true),
                            "Create Time".cell().bold(true),
                            "Index Count".cell().bold(true),
                        ])
                        .bold(true),
                )
            }
            "record" => {
                // show record [view:string] [version:u16]
                if used.is_empty() {
                    return Err(Errs::str(
                        "database name not defined, please use `use [database/page/ledger] [database]` first!",
                    ));
                }
                if len != 4 {
                    return Err(george_error(scan));
                }
                let name = vss[2].clone();
                let version = vss[3].parse::<u32>().unwrap();
                let record = config.view.record(used, name, version)?;
                let table = vec![vec![
                    record.filepath.cell(),
                    Trans::grpc_timestamp_2_string(record.time.as_ref().unwrap().seconds).cell(),
                    version.cell().justify(Justify::Right),
                ]]
                .table()
                .title(vec![
                    "Filepath".cell().bold(true),
                    "Time".cell().bold(true),
                    "Version".cell().bold(true),
                ])
                .bold(true);
                print_table(table)
            }
            "records" => {
                // show records [view:string]
                if used.is_empty() {
                    return Err(Errs::str(
                        "database name not defined, please use `use [database/page/ledger] [database]` first!",
                    ));
                }
                if len != 3 {
                    return Err(george_error(scan));
                }
                let name = vss[2].clone();
                let records = config.view.records(used, name)?;
                let mut table = vec![];
                for record in records {
                    table.push(vec![
                        record.filepath.cell(),
                        Trans::grpc_timestamp_2_string(record.time.as_ref().unwrap().seconds)
                            .cell(),
                        record.version.cell().justify(Justify::Right),
                    ])
                }
                print_table(
                    table
                        .table()
                        .title(vec![
                            "Filepath".cell().bold(true),
                            "Time".cell().bold(true),
                            "Version".cell().bold(true),
                        ])
                        .bold(true),
                )
            }
            "indexes" => {
                // show indexes from [view:string];
                if len != 4 {
                    return Err(george_error(scan));
                }
                if used.is_empty() {
                    return Err(Errs::str(
                        "database name not defined, please use `use [database/page/ledger] [database]` first!",
                    ));
                }
                if vss[2].ne("from") {
                    return Err(george_error(scan));
                }
                let view_name = vss[3].clone();
                let mut view_exist = false;
                match config.view.list(used.clone()) {
                    Ok(list) => {
                        for view in list.iter() {
                            if view_name.eq(view.name.as_str()) {
                                view_exist = true;
                                break;
                            }
                        }
                    }
                    _ => {
                        return Err(Errs::string(format!(
                            "no view matched {} in {}!",
                            view_name, used
                        )))
                    }
                }
                if !view_exist {
                    return Err(Errs::string(format!(
                        "no view matched {} in {}!",
                        view_name, used
                    )));
                }
                let list = config.index.list(used, view_name)?;
                let mut table = vec![];
                for index in list.iter() {
                    table.push(vec![
                        index.name.clone().cell(),
                        index.unique.cell(),
                        index.primary.cell(),
                        index.null.cell(),
                        Trans::i32_2_key_type_str(index.key_type)?.cell(),
                        Trans::i32_2_engine_str(index.engine)?.cell(),
                        Trans::grpc_timestamp_2_string(index.create_time.as_ref().unwrap().seconds)
                            .cell()
                            .justify(Justify::Right),
                    ])
                }
                print_table(
                    table
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
                        .bold(true),
                )
            }
            _ => Err(Errs::string(format!(
                "command do not support prefix {} in {}",
                intent, scan
            ))),
        }
    }
}
