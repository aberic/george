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
use std::option::Option::Some;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use openssl::ssl::{select_next_proto, AlpnError, SslAcceptor, SslFiletype, SslMethod};
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use tonic::transport::{Certificate, Identity, ServerTlsConfig};

use comm::errors::{Errs, GeorgeResult};
use comm::io::file::FilerReader;
use comm::io::Filer;
use comm::openssl::tonic::ALPN_H2_WIRE;
use db::task::traits::TMaster;
use db::Task;
use deploy::{Init, LogPolicy};
use rpc::protos::db::db::database_service_server::DatabaseServiceServer;
use rpc::protos::db::db::disk_service_server::DiskServiceServer;
use rpc::protos::db::db::index_service_server::IndexServiceServer;
use rpc::protos::db::db::memory_service_server::MemoryServiceServer;
use rpc::protos::db::db::page_service_server::PageServiceServer;
use rpc::protos::db::db::user_service_server::UserServiceServer;
use rpc::protos::db::db::view_service_server::ViewServiceServer;
use rpc::server::db::{
    DatabaseServer, DiskServer, IndexServer, MemoryServer, PageServer, UserServer, ViewServer,
};
use rpc::server::db::{DATABASE_SYS, DEFAULT_COMMENT, VIEW_USER};

use crate::cmd::Service;

impl Service {
    /// filepath e.g: `server/src/example/conf.yaml` | `server/src/example/conf_tls.yaml`
    pub fn start<P: AsRef<Path>>(filepath: P) {
        let (init, task, addr) = run_prepare(filepath).unwrap();
        let rt = Runtime::new().expect("failed to obtain a new RunTime object");
        rt.block_on(run(init, task, addr))
            .expect("failed to successfully run the future on RunTime");
    }
}

fn run_prepare<P: AsRef<Path>>(filepath: P) -> GeorgeResult<(Init, Arc<Task>, SocketAddr)> {
    let init = Init::from(filepath)?;
    log_policy(init.clone());
    let task = Arc::new(Task::new(init.clone())?);
    init_data(task.clone())?;

    log::info!("listener port: {}", init.port_unwrap());
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), init.port_unwrap());

    Ok((init, task, addr))
}

async fn run(init: Init, task: Arc<Task>, addr: SocketAddr) -> GeorgeResult<()> {
    let mut server = tonic::transport::Server::builder();
    if init.tls() {
        let mut tls_config = ServerTlsConfig::new();
        match init.server().unwrap().tls_key {
            Some(res) => {
                let key = Filer::read_bytes(res)?;
                match init.server().unwrap().tls_cert {
                    Some(res) => {
                        let cert = Filer::read_bytes(res)?;
                        let identity = Identity::from_pem(cert, key);
                        tls_config = tls_config.identity(identity);
                        log::info!("listener tls config identity success!");
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        match init.server().unwrap().tls_client_root_cert {
            Some(res) => {
                let client_ca = Filer::read_bytes(res)?;
                let cert = Certificate::from_pem(client_ca);
                tls_config = tls_config.client_ca_root(cert);
                log::info!("listener tls config client ca root success!");
            }
            _ => {}
        }
        server = server.tls_config(tls_config).unwrap();
        log::info!("listener tls open!");
    }
    if let Some(res) = init.server().unwrap().timeout {
        server.timeout(Duration::from_secs(res));
    }
    if let Some(res) = init.server().unwrap().concurrency_limit_per_connection {
        server = server.concurrency_limit_per_connection(res);
    }
    if let Some(res) = init.server().unwrap().tcp_nodelay {
        server = server.tcp_nodelay(res);
    }
    if let Some(res) = init.server().unwrap().tcp_keepalive {
        server = server.tcp_keepalive(Some(Duration::from_millis(res)));
    }
    if let Some(res) = init.server().unwrap().http2_keepalive_interval {
        server = server.http2_keepalive_interval(Some(Duration::from_millis(res)));
    }
    if let Some(res) = init.server().unwrap().http2_keepalive_timeout {
        server = server.http2_keepalive_timeout(Some(Duration::from_millis(res)));
    }
    if let Some(res) = init.server().unwrap().initial_connection_window_size {
        server = server.initial_connection_window_size(res);
    }
    if let Some(res) = init.server().unwrap().initial_stream_window_size {
        server = server.initial_stream_window_size(res);
    }
    if let Some(res) = init.server().unwrap().max_concurrent_streams {
        server = server.max_concurrent_streams(res);
    }
    if let Some(res) = init.server().unwrap().max_frame_size {
        server = server.max_frame_size(res);
    }

    let database_service = DatabaseServiceServer::new(DatabaseServer::new(task.clone()));
    let disk_service = DiskServiceServer::new(DiskServer::new(task.clone()));
    let index_service = IndexServiceServer::new(IndexServer::new(task.clone()));
    let memory_service = MemoryServiceServer::new(MemoryServer::new(task.clone()));
    let page_service = PageServiceServer::new(PageServer::new(task.clone()));
    let user_service = UserServiceServer::new(UserServer::new(task.clone()));
    let view_service = ViewServiceServer::new(ViewServer::new(task.clone()));

    match server
        .add_service(database_service)
        .add_service(disk_service)
        .add_service(index_service)
        .add_service(memory_service)
        .add_service(page_service)
        .add_service(user_service)
        .add_service(view_service)
        .serve(addr)
        .await
    {
        Ok(()) => Ok(()),
        Err(err) => Err(Errs::strs("serve with incoming", err)),
    }
}

async fn run_with_openssl(init: Init, task: Arc<Task>, addr: SocketAddr) -> GeorgeResult<()> {
    let mut server = tonic::transport::Server::builder();

    let mut acceptor_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    acceptor_builder
        .set_private_key_file(init.server_key_unwrap(), SslFiletype::PEM)
        .unwrap();
    acceptor_builder
        .set_certificate_chain_file(init.server_cert_unwrap())
        .unwrap();
    acceptor_builder.check_private_key().unwrap();
    acceptor_builder.set_alpn_protos(ALPN_H2_WIRE).unwrap();
    acceptor_builder.set_alpn_select_callback(|_ssl, alpn| {
        select_next_proto(ALPN_H2_WIRE, alpn).ok_or(AlpnError::NOACK)
    });
    let acceptor = acceptor_builder.build();

    let listener = TcpListener::bind(addr).await.unwrap();
    let listener = listener.into_std().unwrap();
    let incoming = tokio_stream::wrappers::TcpListenerStream::new(
        tokio::net::TcpListener::from_std(listener).unwrap(),
    );
    let incoming = comm::openssl::tonic::incoming(incoming, acceptor);

    let database_service = DatabaseServiceServer::new(DatabaseServer::new(task.clone()));
    let disk_service = DiskServiceServer::new(DiskServer::new(task.clone()));
    let index_service = IndexServiceServer::new(IndexServer::new(task.clone()));
    let memory_service = MemoryServiceServer::new(MemoryServer::new(task.clone()));
    let page_service = PageServiceServer::new(PageServer::new(task.clone()));
    let user_service = UserServiceServer::new(UserServer::new(task.clone()));
    let view_service = ViewServiceServer::new(ViewServer::new(task.clone()));

    match server
        .add_service(database_service)
        .add_service(disk_service)
        .add_service(index_service)
        .add_service(memory_service)
        .add_service(page_service)
        .add_service(user_service)
        .add_service(view_service)
        .serve_with_incoming(incoming)
        .await
    {
        Ok(()) => Ok(()),
        Err(err) => Err(Errs::strs("serve with incoming", err)),
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
