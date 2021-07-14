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

use crate::cmd::{george_error, print_table, Config, Delete};

impl Delete {
    pub(crate) fn analysis(
        config: &mut Config,
        used: String,
        scan: String,
        vss: Vec<String>,
    ) -> GeorgeResult<()> {
        // delete [view:string] [constraint:string]
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
        let deleted = config.disk.delete(used, view_name, constraint_json_bytes)?;
        let table = vec![vec![
            deleted.total.cell(),
            deleted.count.cell(),
            deleted.index_name.cell(),
            deleted.asc.cell().justify(Justify::Right),
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
