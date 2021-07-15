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
use george_comm::strings::StringHandler;
use george_comm::Strings;

use crate::cmd::{george_error, print_table, Client, Select};

impl Select {
    pub(crate) fn analysis(
        client: &mut Client,
        used: String,
        scan: String,
        vss: Vec<String>,
    ) -> GeorgeResult<()> {
        // select [view:string] [constraint:string]
        if used.is_empty() {
            return Err(Errs::str(
                "database name not defined, please use `use [database/page/ledger] [database]` first!",
            ));
        }
        if vss.len() != 3 {
            return Err(george_error(scan));
        }
        let view_name = vss[1].clone();
        let constraint_json_bytes = vss[2].as_bytes().to_vec();
        let selected = client.disk.select(used, view_name, constraint_json_bytes)?;
        let mut table = vec![];
        for v8s in selected.values.to_vec() {
            table.push(vec![Strings::from_utf8(v8s)?
                .cell()
                .justify(Justify::Right)])
        }
        print_table(
            table
                .table()
                .title(vec!["Value".cell().bold(true)])
                .bold(true),
        )?;
        let table = vec![vec![
            selected.total.cell(),
            selected.count.cell(),
            selected.index_name.cell(),
            selected.asc.cell().justify(Justify::Right),
        ]]
        .table()
        .title(vec![
            "Total".cell().bold(true),
            "Count".cell().bold(true),
            "Index Name".cell().bold(true),
            "Asc".cell().bold(true),
        ])
        .bold(true);
        print_table(table)
    }
}
