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

use crate::cmd::{george_error, print_table, Config, Show};

impl Show {
    pub(crate) fn analysis(
        config: &Config,
        used: String,
        scan: String,
        vss: Vec<String>,
    ) -> GeorgeResult<()> {
        if vss.len() < 2 {
            return Err(george_error(scan));
        }
        let intent = vss[1].as_str();
        match intent {
            "databases" => {
                // show databases;
                let db_list = config.database.list()?;
                let list = db_list.databases;
                let mut table = vec![];
                for db in list.iter() {
                    table.push(vec![
                        db.get_name().cell(),
                        db.get_comment().cell(),
                        Comm::proto_grpc_timestamp_2_time(db.get_create_time().seconds)
                            .to_string("%Y-%m-%d %H:%M:%S")
                            .cell(),
                        db.get_views().len().cell().justify(Justify::Right),
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
                let page_list = config.page.list()?;
                let list = page_list.pages;
                let mut table = vec![];
                for page in list.iter() {
                    table.push(vec![
                        page.get_name().cell(),
                        page.get_comment().cell(),
                        page.get_size().cell(),
                        page.get_period().cell(),
                        Comm::proto_grpc_timestamp_2_time(page.get_create_time().seconds)
                            .to_string("%Y-%m-%d %H:%M:%S")
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
                let view_list = config.view.list(used)?;
                let list = view_list.views;
                let mut table = vec![];
                for view in list.iter() {
                    table.push(vec![
                        view.get_name().cell(),
                        view.get_comment().cell(),
                        Comm::proto_grpc_timestamp_2_time(view.get_create_time().seconds)
                            .to_string("%Y-%m-%d %H:%M:%S")
                            .cell(),
                        view.get_indexes().len().cell().justify(Justify::Right),
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
            "indexes" => {
                // show indexes from [view:string];
                if vss.len() != 4 {
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
                        for view in list.views.iter() {
                            if view_name.eq(view.get_name()) {
                                view_exist = true
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
                let index_list = config.index.list(used, view_name)?;
                let list = index_list.indexes;
                let mut table = vec![];
                for index in list.iter() {
                    table.push(vec![
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
                "command do not support prefix {}",
                intent
            ))),
        }
    }
}
