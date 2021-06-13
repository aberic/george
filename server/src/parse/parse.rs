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

use crate::parse::{Delete, Get, Insert, Parse, Put, Select, Set, Show};
use crate::utils::Comm;
use comm::errors::{Errs, GeorgeResult};
use db::Task;
use std::sync::Arc;

impl Parse {
    pub fn analysis(task: Arc<Task>, scan: String) -> GeorgeResult<Vec<u8>> {
        log::info!("command scan: {}", scan);
        let parse = Comm::parse_str(scan);
        log::info!("command parse: {}", parse);
        let mut vss = Comm::split_str(parse.clone());
        if vss.len() == 0 {
            return Err(Errs::string(format!("error command with '{}'", parse)));
        }
        let intent = vss[0].as_str();
        match intent {
            "show" => Show::analysis(task, vss),
            "put" => Put::analysis(task, vss),
            "set" => Set::analysis(task, vss),
            "insert" => Insert::analysis(task, vss),
            "get" => Get::analysis(task, vss),
            "select" => Select::analysis(task, vss),
            "delete" => Delete::analysis(task, vss),
            _ => Err(Errs::string(format!(
                "command not support prefix {} in '{}'",
                intent, parse
            ))),
        }
    }
}
