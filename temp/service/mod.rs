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

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::Path;
use std::sync::{Arc, RwLock};
use std::thread;

use protobuf::{RepeatedField, SingularPtrField};
use tonic::transport::{Identity, ServerTlsConfig};

use comm::errors::GeorgeResult;
use comm::io::file::FilerReader;
use comm::io::Filer;
use db::task::traits::{TForm, TMaster};
use db::Task;
use deploy::{Init, LogPolicy};
use protocols::impls::db::index::{Engine, Index, KeyType};
use protocols::impls::db::service_grpc::{
    DiskServiceServer, IndexServiceServer, MemoryServiceServer, PageServiceServer,
    UserServiceServer, ViewServiceServer,
};
use protocols::impls::db::view::View;
use protocols::impls::utils::Comm;
use rpc::protos::db::db::database_service_server::DatabaseServiceServer;
use rpc::server::db::DatabaseServer;
use tokio::runtime::Runtime;

pub mod database;
mod disk;
mod index;
mod memory;
mod page;
mod user;
mod view;

pub const DATABASE_SYS: &str = "sys";
pub const VIEW_USER: &str = "user";
pub const DEFAULT_COMMENT: &str = "system default";

pub struct Server;

impl Server {
    /// filepath e.g: `server/src/example/conf.yaml` | `server/src/example/conf_tls.yaml`
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
        match init_data(task.clone()) {
            Err(err) => panic!("Init data failed! {}", err),
            _ => {}
        }

        log::info!("listener port: {}", init.port_unwrap());
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), init.port_unwrap());

        let database_service = DatabaseServiceServer::new(DatabaseServer::new(task.clone()));

        let server_future;
        if init.tls() {
            let cert = Filer::read_bytes(init.server_cert_unwrap()).unwrap();
            let key = Filer::read_bytes(init.server_key_unwrap()).unwrap();
            let identity = Identity::from_pem(cert, key);
            server_future = tonic::transport::Server::builder()
                .tls_config(ServerTlsConfig::new().identity(identity))
                .unwrap()
                .add_service(database_service)
                .serve(addr)
        } else {
            server_future = tonic::transport::Server::builder()
                .add_service(database_service)
                .serve(addr)
        }
        let rt = Runtime::new().expect("failed to obtain a new RunTime object");
        rt.block_on(server_future)
            .expect("failed to successfully run the future on RunTime");
    }

    // /// filepath e.g: `server/src/example/conf.yaml`
    // pub fn start<P: AsRef<Path>>(filepath: P) {
    //     let init: Init;
    //     match Init::from(filepath) {
    //         Ok(res) => init = res,
    //         Err(err) => panic!("Init from failed! {}", err),
    //     }
    //     log_policy(init.clone());
    //     let task: Arc<Task>;
    //     match Task::new(init.clone()) {
    //         Ok(res) => task = Arc::new(res),
    //         Err(err) => panic!("Task new failed! {}", err),
    //     }
    //     match init_data(task.clone()) {
    //         Err(err) => panic!("Init data failed! {}", err),
    //         _ => {}
    //     }
    //     let mut server = grpc::ServerBuilder::new_plain();
    //     server.http.set_port(init.port_unwrap());
    //     server.http.conf.no_delay = Some(true);
    //     server.http.conf.thread_name = Some("george-server".to_string());
    //     server.http.conf.reuse_port = Some(true);
    //     // server.http.set_cpu_pool_threads(4);
    //     server.add_service(UserServiceServer::new_service_def(UserServer {
    //         task: task.clone(),
    //     }));
    //     server.add_service(PageServiceServer::new_service_def(PageServer {
    //         task: task.clone(),
    //     }));
    //     server.add_service(DatabaseServiceServer::new_service_def(DatabaseServer {
    //         task: task.clone(),
    //     }));
    //     server.add_service(ViewServiceServer::new_service_def(ViewServer {
    //         task: task.clone(),
    //     }));
    //     server.add_service(IndexServiceServer::new_service_def(IndexServer {
    //         task: task.clone(),
    //     }));
    //     server.add_service(DiskServiceServer::new_service_def(DiskServer {
    //         task: task.clone(),
    //     }));
    //     server.add_service(MemoryServiceServer::new_service_def(MemoryServer {
    //         task: task.clone(),
    //     }));
    //     let _server = server.build().expect("Could not start server");
    //     loop {
    //         thread::park();
    //     }
    // }
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

/// 初始化
fn init_data(task: Arc<Task>) -> GeorgeResult<()> {
    if !task.init() {
        log::info!("server init!");
        task.page_create(DATABASE_SYS.to_string(), DEFAULT_COMMENT.to_string(), 0, 0)?;
        task.database_create(DATABASE_SYS.to_string(), DEFAULT_COMMENT.to_string())?;
        task.view_create(
            DATABASE_SYS.to_string(),
            VIEW_USER.to_string(),
            DEFAULT_COMMENT.to_string(),
            true,
        )?;
        task.put_disk(
            DATABASE_SYS.to_string(),
            VIEW_USER.to_string(),
            "admin".to_string(),
            "admin#123".as_bytes().to_vec(),
        )?;
        log::info!("server init success!");
    }
    Ok(())
}

pub(crate) struct Enums;

impl Enums {
    pub(crate) fn db_2_engine(e: db::utils::enums::Engine) -> Engine {
        match e {
            db::utils::enums::Engine::None => Engine::None,
            db::utils::enums::Engine::Disk => Engine::Disk,
            db::utils::enums::Engine::Sequence => Engine::Sequence,
            db::utils::enums::Engine::Block => Engine::Block,
            db::utils::enums::Engine::Increment => Engine::Increment,
        }
    }

    pub(crate) fn engine_2_db(e: Engine) -> db::utils::enums::Engine {
        match e {
            Engine::None => db::utils::enums::Engine::None,
            Engine::Disk => db::utils::enums::Engine::Disk,
            Engine::Sequence => db::utils::enums::Engine::Sequence,
            Engine::Block => db::utils::enums::Engine::Block,
            Engine::Increment => db::utils::enums::Engine::Increment,
        }
    }

    pub(crate) fn db_2_key_type(e: db::utils::enums::KeyType) -> KeyType {
        match e {
            db::utils::enums::KeyType::None => KeyType::Nonsupport,
            db::utils::enums::KeyType::String => KeyType::String,
            db::utils::enums::KeyType::UInt => KeyType::UInt,
            db::utils::enums::KeyType::Int => KeyType::Int,
            db::utils::enums::KeyType::Bool => KeyType::Bool,
            db::utils::enums::KeyType::Float => KeyType::Float,
        }
    }

    pub(crate) fn key_type_2_db(e: KeyType) -> db::utils::enums::KeyType {
        match e {
            KeyType::Nonsupport => db::utils::enums::KeyType::None,
            KeyType::String => db::utils::enums::KeyType::String,
            KeyType::UInt => db::utils::enums::KeyType::UInt,
            KeyType::Int => db::utils::enums::KeyType::Int,
            KeyType::Bool => db::utils::enums::KeyType::Bool,
            KeyType::Float => db::utils::enums::KeyType::Float,
        }
    }
}

pub(crate) struct Children;

impl Children {
    pub(crate) fn indexes(view: Arc<RwLock<db::task::View>>) -> RepeatedField<Index> {
        let view_r = view.read().unwrap();
        let indexes = view_r.index_map();
        let indexes_r = indexes.read().unwrap();
        let mut indexes: RepeatedField<Index> = RepeatedField::new();
        for (_name, index) in indexes_r.iter() {
            indexes.push(Index {
                name: index.name(),
                engine: Enums::db_2_engine(index.engine()),
                primary: index.primary(),
                unique: index.unique(),
                null: index.null(),
                key_type: Enums::db_2_key_type(index.key_type()),
                create_time: SingularPtrField::some(Comm::proto_time_2_grpc_timestamp(
                    index.create_time(),
                )),
                unknown_fields: Default::default(),
                cached_size: Default::default(),
            })
        }
        indexes
    }

    pub(crate) fn views(database: Arc<RwLock<db::task::Database>>) -> RepeatedField<View> {
        let database_r = database.read().unwrap();
        let views = database_r.view_map();
        let views_r = views.read().unwrap();
        let mut views: RepeatedField<View> = RepeatedField::new();
        for (_name, view) in views_r.iter() {
            let indexes = Children::indexes(view.clone());
            let view_r = view.read().unwrap();
            views.push(View {
                name: view_r.name(),
                comment: view_r.comment(),
                create_time: SingularPtrField::some(Comm::proto_time_2_grpc_timestamp(
                    view_r.create_time(),
                )),
                indexes,
                filepath: view_r.filepath(),
                version: view_r.version() as u32,
                unknown_fields: Default::default(),
                cached_size: Default::default(),
            })
        }
        views
    }
}
