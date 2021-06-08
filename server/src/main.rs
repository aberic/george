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

use std::sync::Arc;
use std::thread;

use clap::{App, Arg, ArgMatches};

use db::Task;
use deploy::{Builder, Init};
use protocols::impls::db::service_grpc::{
    DatabaseServiceServer, DiskServiceServer, IndexServiceServer, MemoryServiceServer,
    PageServiceServer, ViewServiceServer,
};

use crate::cmd::Command;
use crate::service::database::DatabaseServer;
use crate::service::disk::DiskServer;
use crate::service::index::IndexServer;
use crate::service::memory::MemoryServer;
use crate::service::page::PageServer;
use crate::service::view::ViewServer;
use crate::service::Server;

mod cmd;
pub mod service;
mod utils;

fn main() {
    Command::init();
}

#[test]
fn test() {
    println!("test!");
}
