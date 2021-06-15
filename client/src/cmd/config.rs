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

use crate::cmd::{Config, Delete, Get, Insert, Put, Select, Set, Show};
use crate::service::{Parse, User};
use crate::utils::Comm;
use comm::errors::{Errs, GeorgeResult};
use std::io;
use std::io::Write;

impl Config {
    pub(crate) fn new(remote: &str, port: u16) -> Self {
        let user = User::new(remote, port);
        let parse = Parse::new(remote, port);
        Config { user, parse }
    }

    pub(crate) fn login(&self, name: String, pass: String) -> GeorgeResult<()> {
        self.user.login(name, pass)
    }

    pub fn scan(&self) {
        print!("george->: ");
        io::stdout().flush().unwrap();
        let mut new_str = String::new();
        let mut all_str = String::new();
        let mut used = String::from("");
        while io::stdin().read_line(&mut new_str).is_ok() {
            if new_str.contains(";") {
                if new_str.starts_with("use") {
                    let mut vsi = new_str.split(" ");
                    let _v = vsi.next();
                    let v = vsi.next();
                    if v.is_some() {
                        used = v.unwrap().to_string();
                        print!("george->: ");
                        io::stdout().flush().unwrap();
                        new_str.clear();
                        continue;
                    }
                }
                all_str.push_str(new_str.as_str());
                match self.parse(used.clone(), all_str.clone()) {
                    Ok(res) => match String::from_utf8(res) {
                        Ok(res) => {
                            println!("{}", res);
                        }
                        Err(err) => println!("error: {}", err),
                    },
                    Err(err) => println!("error: {}", err),
                }
                print!("george->: ");
                io::stdout().flush().unwrap();
                all_str.clear();
            } else {
                all_str.push_str(new_str.as_str());
            }
            new_str.clear();
        }
    }

    pub(crate) fn parse(&self, used: String, scan_str: String) -> GeorgeResult<Vec<u8>> {
        let parse = Comm::parse_str(scan);
        log::info!("command used {} parse: {}", used, parse);
        let mut vss = Comm::split_str(parse.clone());
        if vss.len() == 0 {
            return Err(Errs::string(format!("error command with '{}'", parse)));
        }
        let intent = vss[0].as_str();
        match intent {
            "show" => Show::analysis(task, used, vss),
            "put" => Put::analysis(task, used, vss),
            "set" => Set::analysis(task, used, vss),
            "insert" => Insert::analysis(task, used, vss),
            "get" => Get::analysis(task, used, vss),
            "select" => Select::analysis(task, used, vss),
            "delete" => Delete::analysis(task, used, vss),
            _ => Err(Errs::string(format!(
                "command not support prefix {} in '{}'",
                intent, parse
            ))),
        }
        // Scan::run(&self.parse)
    }
}
