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

use db::Task;
use deploy::Init;
use protocols::impls::db::service_grpc::{
    DatabaseServiceServer, DiskServiceServer, IndexServiceServer, MemoryServiceServer,
    PageServiceServer, ViewServiceServer,
};

use crate::service::database::DatabaseServer;
use crate::service::disk::DiskServer;
use crate::service::index::IndexServer;
use crate::service::memory::MemoryServer;
use crate::service::page::PageServer;
use crate::service::view::ViewServer;

pub mod service;
mod utils;

fn main() {
    let init = Init::from("server/src/example/conf.yaml").expect("Task new failed!");
    let task = Arc::new(Task::new(init).expect("Task new failed!"));
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(9000);
    // server.http.set_cpu_pool_threads(4);
    server.add_service(PageServiceServer::new_service_def(PageServer {
        task: task.clone(),
    }));
    server.add_service(DatabaseServiceServer::new_service_def(DatabaseServer {
        task: task.clone(),
    }));
    server.add_service(ViewServiceServer::new_service_def(ViewServer {
        task: task.clone(),
    }));
    server.add_service(IndexServiceServer::new_service_def(IndexServer {
        task: task.clone(),
    }));
    server.add_service(DiskServiceServer::new_service_def(DiskServer {
        task: task.clone(),
    }));
    server.add_service(MemoryServiceServer::new_service_def(MemoryServer {
        task: task.clone(),
    }));
    let _server = server.build().expect("Could not start server");
    loop {
        thread::park();
    }
}

#[test]
fn test() {
    println!("test!");
}
