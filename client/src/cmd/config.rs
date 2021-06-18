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

use std::io;
use std::io::Write;

use comm::errors::{Errs, GeorgeResult};
use protocols::impls::utils::Comm;

use crate::cmd::{
    george_error, Alter, Config, Create, Delete, Drop, Get, Insert, Inspect, Put, Remove, Select,
    Set, Show,
};
use crate::service::{Database, Disk, Index, Memory, Page, User, View};

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
        let mut disk: bool = false;
        let mut used = String::from("");
        while io::stdin().read_line(&mut new_str).is_ok() {
            new_str = Comm::trim_str(new_str);
            if new_str.ends_with(";") {
                all_str.push_str(new_str.as_str());
                let scan = Comm::parse_str(all_str.clone());
                if scan.starts_with("use ") {
                    match self.use_check(scan) {
                        Ok((d, u)) => {
                            disk = d;
                            used = u;
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
                } else if scan.starts_with("clear") {
                    match self.clear_check(scan) {
                        Ok(()) => {
                            disk = false;
                            used = String::from("");
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
                match self.parse(disk, used.clone(), scan.clone()) {
                    Ok(()) => println!("exec \"{}\" on {} success!", scan, used),
                    Err(err) => println!("exec \"{}\" on {} error: {}", scan, used, err),
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

    /// 验证use语法
    ///
    /// #return
    ///
    /// * bool 是否为磁盘存储类型
    /// * String 存储引擎名称
    fn use_check(&self, scan: String) -> GeorgeResult<(bool, String)> {
        let vss = Comm::split_str(scan.clone());
        if vss.len() != 3 {
            return Err(george_error(scan));
        }
        let disk: bool;
        let used = vss[1].as_str();
        let name = vss[2].clone();
        match used {
            "database" => match self.database.list() {
                Ok(list) => {
                    for database in list.databases.iter() {
                        if name.eq(database.get_name()) {
                            disk = true;
                            return Ok((disk, name));
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
                            disk = false;
                            return Ok((disk, name));
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

    /// 恢复初始状态
    fn clear_check(&self, scan: String) -> GeorgeResult<()> {
        let vss = Comm::split_str(scan.clone());
        if vss.len() != 1 {
            return Err(george_error(scan));
        }
        let clear = vss[0].as_str();
        match clear {
            "clear" => Ok(()),
            _ => Err(Errs::string(format!(
                "command do not support prefix {} in '{}'",
                clear, scan
            ))),
        }
    }

    fn parse(&self, disk: bool, used: String, scan: String) -> GeorgeResult<()> {
        let vss = Comm::split_str(scan.clone());
        if vss.len() == 0 {
            return Err(george_error(scan));
        }
        let intent = vss[0].as_str();
        match intent {
            "show" => Show::analysis(&self, disk, used, scan, vss),
            "inspect" => Inspect::analysis(&self, used, scan, vss),
            "create" => Create::analysis(&self, used, scan, vss),
            "alter" => Alter::analysis(&self, used, scan, vss),
            "drop" => Drop::analysis(&self, used, scan, vss),
            "put" => Put::analysis(&self, disk, used, scan, vss),
            "set" => Set::analysis(&self, disk, used, scan, vss),
            "insert" => Insert::analysis(&self, used, scan, vss),
            "get" => Get::analysis(&self, disk, used, scan, vss),
            "remove" => Remove::analysis(&self, disk, used, scan, vss),
            "select" => Select::analysis(&self, used, scan, vss),
            "delete" => Delete::analysis(&self, used, scan, vss),
            _ => Err(Errs::string(format!(
                "command do not support prefix {} in '{}'",
                intent, scan
            ))),
        }
    }
}
