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

use george_comm::errors::{Errs, GeorgeResult};
use george_rpc::client::db::{
    DatabaseRpcClient, DiskRpcClient, IndexRpcClient, MemoryRpcClient, PageRpcClient,
    UserRpcClient, ViewRpcClient,
};
use george_rpc::client::{RequestCond, RpcClient, TLSType};
use george_rpc::tools::Trim;

use crate::cmd::{
    george_error, Alter, Client, Create, Delete, Drop, Get, Insert, Inspect, Put, Remove, Select,
    Set, Show,
};

impl RpcClient for Client {
    fn new(remote: &str, port: u16, cond_op: Option<RequestCond>) -> GeorgeResult<Self>
    where
        Self: Sized,
    {
        let user = UserRpcClient::new(remote, port, cond_op)?;
        let database = DatabaseRpcClient::new(remote, port, cond_op)?;
        let page = PageRpcClient::new(remote, port, cond_op)?;
        let view = ViewRpcClient::new(remote, port, cond_op)?;
        let index = IndexRpcClient::new(remote, port, cond_op)?;
        let disk = DiskRpcClient::new(remote, port, cond_op)?;
        let memory = MemoryRpcClient::new(remote, port, cond_op)?;
        Ok(Client {
            user,
            database,
            page,
            view,
            index,
            disk,
            memory,
        })
    }

    fn new_tls_bytes(
        tls_type: TLSType,
        remote: &str,
        port: u16,
        ca_bytes: Vec<u8>,
        domain_name: impl Into<String>,
        cond_op: Option<RequestCond>,
    ) -> GeorgeResult<Self>
    where
        Self: Sized,
    {
        let dn = domain_name.into();
        let user = UserRpcClient::new_tls_bytes(
            tls_type,
            remote,
            port,
            ca_bytes.clone(),
            dn.clone(),
            cond_op,
        )?;
        let database = DatabaseRpcClient::new_tls_bytes(
            tls_type,
            remote,
            port,
            ca_bytes.clone(),
            dn.clone(),
            cond_op,
        )?;
        let page = PageRpcClient::new_tls_bytes(
            tls_type,
            remote,
            port,
            ca_bytes.clone(),
            dn.clone(),
            cond_op,
        )?;
        let view = ViewRpcClient::new_tls_bytes(
            tls_type,
            remote,
            port,
            ca_bytes.clone(),
            dn.clone(),
            cond_op,
        )?;
        let index = IndexRpcClient::new_tls_bytes(
            tls_type,
            remote,
            port,
            ca_bytes.clone(),
            dn.clone(),
            cond_op,
        )?;
        let disk = DiskRpcClient::new_tls_bytes(
            tls_type,
            remote,
            port,
            ca_bytes.clone(),
            dn.clone(),
            cond_op,
        )?;
        let memory = MemoryRpcClient::new_tls_bytes(
            tls_type,
            remote,
            port,
            ca_bytes.clone(),
            dn.clone(),
            cond_op,
        )?;
        Ok(Client {
            user,
            database,
            page,
            view,
            index,
            disk,
            memory,
        })
    }

    fn new_tls_bytes_check(
        tls_type: TLSType,
        remote: &str,
        port: u16,
        key_bytes: Vec<u8>,
        cert_bytes: Vec<u8>,
        ca_bytes: Vec<u8>,
        domain_name: impl Into<String>,
        cond_op: Option<RequestCond>,
    ) -> GeorgeResult<Self>
    where
        Self: Sized,
    {
        let dn = domain_name.into();
        let user = UserRpcClient::new_tls_bytes_check(
            tls_type,
            remote,
            port,
            key_bytes.clone(),
            cert_bytes.clone(),
            ca_bytes.clone(),
            dn.clone(),
            cond_op,
        )?;
        let database = DatabaseRpcClient::new_tls_bytes_check(
            tls_type,
            remote,
            port,
            key_bytes.clone(),
            cert_bytes.clone(),
            ca_bytes.clone(),
            dn.clone(),
            cond_op,
        )?;
        let page = PageRpcClient::new_tls_bytes_check(
            tls_type,
            remote,
            port,
            key_bytes.clone(),
            cert_bytes.clone(),
            ca_bytes.clone(),
            dn.clone(),
            cond_op,
        )?;
        let view = ViewRpcClient::new_tls_bytes_check(
            tls_type,
            remote,
            port,
            key_bytes.clone(),
            cert_bytes.clone(),
            ca_bytes.clone(),
            dn.clone(),
            cond_op,
        )?;
        let index = IndexRpcClient::new_tls_bytes_check(
            tls_type,
            remote,
            port,
            key_bytes.clone(),
            cert_bytes.clone(),
            ca_bytes.clone(),
            dn.clone(),
            cond_op,
        )?;
        let disk = DiskRpcClient::new_tls_bytes_check(
            tls_type,
            remote,
            port,
            key_bytes.clone(),
            cert_bytes.clone(),
            ca_bytes.clone(),
            dn.clone(),
            cond_op,
        )?;
        let memory = MemoryRpcClient::new_tls_bytes_check(
            tls_type,
            remote,
            port,
            key_bytes.clone(),
            cert_bytes.clone(),
            ca_bytes.clone(),
            dn.clone(),
            cond_op,
        )?;
        Ok(Client {
            user,
            database,
            page,
            view,
            index,
            disk,
            memory,
        })
    }
}

impl Client {
    pub(crate) fn login(&mut self, name: String, pass: String) -> GeorgeResult<()> {
        self.user.login(name, pass)
    }

    pub fn scan(&mut self) {
        print!("george->: ");
        io::stdout().flush().unwrap();
        let mut new_str = String::new();
        let mut all_str = String::new();
        let mut disk: bool = false;
        let mut used = String::from("");
        while io::stdin().read_line(&mut new_str).is_ok() {
            new_str = Trim::str(new_str);
            if new_str.ends_with(";") {
                all_str.push_str(new_str.as_str());
                let scan = Trim::parse(all_str.clone());
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
                } else if scan.eq("exit") | scan.eq("quit") | scan.eq("e") | scan.eq("q") {
                    break;
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
    fn use_check(&mut self, scan: String) -> GeorgeResult<(bool, String)> {
        let vss = Trim::split(scan.clone());
        if vss.len() != 3 {
            return Err(george_error(scan));
        }
        let disk: bool;
        let used = vss[1].as_str();
        let name = vss[2].clone();
        match used {
            "database" => match self.database.list() {
                Ok(res) => {
                    for database in res.iter() {
                        if name.eq(database.name.as_str()) {
                            disk = true;
                            return Ok((disk, name));
                        }
                    }
                    Err(Errs::string(format!("no database matched {}!", name)))
                }
                _ => Err(Errs::string(format!("no database matched {}!", name))),
            },
            "page" => match self.page.list() {
                Ok(res) => {
                    for page in res.iter() {
                        if name.eq(page.name.as_str()) {
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
        let vss = Trim::split(scan.clone());
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

    fn parse(&mut self, disk: bool, used: String, scan: String) -> GeorgeResult<()> {
        let vss = Trim::split(scan.clone());
        if vss.len() == 0 {
            return Err(george_error(scan));
        }
        let intent = vss[0].as_str();
        match intent {
            "show" => Show::analysis(self, disk, used, scan, vss),
            "inspect" => Inspect::analysis(self, used, scan, vss),
            "create" => Create::analysis(self, used, scan, vss),
            "alter" => Alter::analysis(self, used, scan, vss),
            "drop" => Drop::analysis(self, used, scan, vss),
            "put" => Put::analysis(self, disk, used, scan, vss),
            "set" => Set::analysis(self, disk, used, scan, vss),
            "insert" => Insert::analysis(&self, used, scan, vss),
            "get" => Get::analysis(self, disk, used, scan, vss),
            "remove" => Remove::analysis(self, disk, used, scan, vss),
            "select" => Select::analysis(self, used, scan, vss),
            "delete" => Delete::analysis(self, used, scan, vss),
            _ => Err(Errs::string(format!(
                "command do not support prefix {} in '{}'",
                intent, scan
            ))),
        }
    }
}
