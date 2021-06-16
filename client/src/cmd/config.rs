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

use crate::cmd::{Config, Create, Delete, Get, Info, Insert, Put, Select, Set, Show};
use crate::service::{Database, Disk, Index, Memory, Page, User, View};
use comm::errors::{Errs, GeorgeError, GeorgeResult};
use protocols::impls::db::database::DatabaseList;
use protocols::impls::utils::Comm;
use std::io;
use std::io::Write;

impl Config {
    pub(crate) fn new(remote: &str, port: u16) -> Self {
        let user = User::new(remote, port);
        let database = Database::new(remote, port);
        let page = Page::new(remote, port);
        let view = View::new(remote, port);
        let index = Index::new(remote, port);
        let disk = Disk::new(remote, port);
        let memory = Memory::new(remote, port);
        Config {
            user,
            database,
            page,
            view,
            index,
            disk,
            memory,
        }
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
            new_str = Comm::trim_str(new_str);
            if new_str.ends_with(";") {
                all_str.push_str(new_str.as_str());
                let scan = Comm::parse_str(all_str.clone());
                if scan.starts_with("use ") {
                    match self.use_check(scan) {
                        Ok(res) => {
                            used = res;
                            print!("george->: ");
                            io::stdout().flush().unwrap();
                            new_str.clear();
                            all_str.clear();
                            continue;
                        }
                        Err(err) => {
                            println!("{}", err);
                            print!("george->: ");
                            io::stdout().flush().unwrap();
                            new_str.clear();
                            all_str.clear();
                            continue;
                        }
                    }
                }
                match self.parse(used.clone(), scan) {
                    Ok(()) => {}
                    Err(err) => println!("error: {}", err),
                }
                print!("george->: ");
                io::stdout().flush().unwrap();
                all_str.clear();
            } else {
                all_str.push_str(new_str.as_str());
                all_str.push_str(" ");
            }
            new_str.clear();
        }
    }

    fn use_check(&self, scan: String) -> GeorgeResult<String> {
        let vss = Comm::split_str(scan.clone());
        if vss.len() != 3 {
            return Err(Errs::string(format!("error command with '{}'", scan)));
        }
        let used = vss[1].as_str();
        let name = vss[2].clone();
        match used {
            "database" => match self.database.list() {
                Ok(list) => {
                    for database in list.databases.iter() {
                        if name.eq(database.get_name()) {
                            return Ok(name);
                        }
                    }
                    Err(Errs::string(format!("no database matched {}!", name)))
                }
                _ => Err(Errs::string(format!("no database matched {}!", name))),
            },
            "page" => match self.page.list() {
                Ok(list) => {
                    for page in list.pages.iter() {
                        if name.eq(page.get_name()) {
                            return Ok(name);
                        }
                    }
                    Err(Errs::string(format!("no page matched {}!", name)))
                }
                _ => Err(Errs::string(format!("no page matched {}!", name))),
            },
            "ledger" => Err(Errs::str("no ledger matched!")),
            _ => Err(Errs::string(format!(
                "command do not support prefix {} in '{}'",
                used, scan
            ))),
        }
    }

    fn parse(&self, used: String, scan: String) -> GeorgeResult<()> {
        let vss = Comm::split_str(scan.clone());
        if vss.len() == 0 {
            return Err(Errs::string(format!("error command with '{}'", scan)));
        }
        let intent = vss[0].as_str();
        match intent {
            "show" => Show::analysis(&self, used, scan, vss),
            "info" => Info::analysis(&self, used, vss),
            "create" => Create::analysis(&self, used, vss),
            "put" => Put::analysis(&self, used, vss),
            "set" => Set::analysis(&self, used, vss),
            "insert" => Insert::analysis(&self, used, vss),
            "get" => Get::analysis(&self, used, vss),
            "select" => Select::analysis(&self, used, vss),
            "delete" => Delete::analysis(&self, used, vss),
            _ => Err(Errs::string(format!(
                "command do not support prefix {} in '{}'",
                intent, scan
            ))),
        }
    }
}
