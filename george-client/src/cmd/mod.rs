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

use cli_table::{print_stdout, Table};

use george_comm::errors::{Errs, GeorgeError, GeorgeResult};
use george_deploy::ConfigServerTLS;
use george_rpc::client::db::{
    DatabaseRpcClient, DiskRpcClient, IndexRpcClient, MemoryRpcClient, PageRpcClient,
    UserRpcClient, ViewRpcClient,
};
use george_rpc::client::RequestCond;

mod alter;
mod client;
mod command;
mod config;
mod create;
mod delete;
mod drop;
mod get;
mod insert;
mod inspect;
mod options;
mod put;
mod remove;
mod select;
mod set;
mod show;

/// yaml解析辅助结构
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub(crate) struct Config {
    config: Option<Conf>,
}

/// 基础配置信息，优先读取环境变量中的结果<p>
///
/// 该配置信息可通过指定路径的文件中进行读取，文件格式支持yaml
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub(crate) struct Conf {
    tls: Option<ConfigServerTLS>,
    http: Option<RequestCond>,
}

pub(crate) struct Command;

pub(crate) struct Options;

pub(crate) struct Show;

pub(crate) struct Inspect;

pub(crate) struct Create;

pub(crate) struct Alter;

pub(crate) struct Drop;

pub(crate) struct Put;

pub(crate) struct Set;

pub(crate) struct Get;

pub(crate) struct Remove;

pub(crate) struct Insert;

pub(crate) struct Select;

pub(crate) struct Delete;

pub(crate) struct Client {
    user: UserRpcClient,
    database: DatabaseRpcClient,
    page: PageRpcClient,
    view: ViewRpcClient,
    index: IndexRpcClient,
    disk: DiskRpcClient,
    memory: MemoryRpcClient,
}

pub(crate) fn george_error(scan: String) -> GeorgeError {
    Errs::string(format!("error command with '{}'", scan))
}

pub(crate) fn george_errors<Err: ToString>(scan: String, err: Err) -> GeorgeError {
    Errs::strings(format!("error command with '{}'", scan), err)
}

pub(crate) fn print_table<T: Table>(table: T) -> GeorgeResult<()> {
    match print_stdout(table) {
        Ok(()) => Ok(()),
        Err(err) => Err(Errs::strs("print stdout", err)),
    }
}
