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

use crate::service::database::DatabaseServer;
use crate::service::disk::DiskServer;
use crate::service::index::IndexServer;
use crate::service::memory::MemoryServer;
use crate::service::page::PageServer;
use crate::service::parse::ParseServer;
use crate::service::user::UserServer;
use crate::service::view::ViewServer;
use db::Task;
use deploy::{Init, LogPolicy};
use protocols::impls::db::service_grpc::{
    DatabaseServiceServer, DiskServiceServer, IndexServiceServer, MemoryServiceServer,
    PageServiceServer, ParseServiceServer, UserServiceServer, ViewServiceServer,
};
use std::path::Path;
use std::sync::Arc;
use std::thread;

mod database;
mod disk;
mod index;
mod memory;
mod page;
mod parse;
mod user;
mod view;

pub struct Server;

impl Server {
    /// filepath e.g: `server/src/example/conf.yaml`
    pub fn start<P: AsRef<Path>>(filepath: P) {
        let init: Init;
        match Init::from(filepath) {
            Ok(res) => init = res,
            Err(err) => panic!("Init from failed! {}", err),
        }
        log_policy(init.clone());
        let task: Arc<Task>;
        match Task::new(init.clone()) {
            Ok(res) => task = Arc::new(res),
            Err(err) => panic!("Task new failed! {}", err),
        }
        let mut server = grpc::ServerBuilder::new_plain();
        server.http.set_port(init.port_unwrap());
        server.http.conf.no_delay = Some(true);
        server.http.conf.thread_name = Some("george-server".to_string());
        server.http.conf.reuse_port = Some(true);
        // server.http.set_cpu_pool_threads(4);
        server.add_service(ParseServiceServer::new_service_def(ParseServer {
            task: task.clone(),
        }));
        server.add_service(UserServiceServer::new_service_def(UserServer {
            task: task.clone(),
        }));
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
}

fn log_policy(init: Init) {
    init.add_log_policy(LogPolicy::new(
        format!("{}/net", init.log_dir_unwrap()),
        "http".to_string(),
        "httpbis::server".to_string(),
        false,
    ));
    init.add_log_policy(LogPolicy::new(
        format!("{}/net", init.log_dir_unwrap()),
        "http".to_string(),
        "httpbis::server::handler_paths".to_string(),
        false,
    ));
    init.add_log_policy(LogPolicy::new(
        format!("{}/net", init.log_dir_unwrap()),
        "http".to_string(),
        "httpbis::server::conn".to_string(),
        false,
    ));
}
