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

use std::path::Path;

use tokio::runtime::Runtime;
use tonic::transport::Channel;

use comm::errors::GeorgeResult;

use crate::protos::db::db::database_service_client::DatabaseServiceClient;
use crate::protos::db::db::disk_service_client::DiskServiceClient;
use crate::protos::db::db::index_service_client::IndexServiceClient;
use crate::protos::db::db::memory_service_client::MemoryServiceClient;
use crate::protos::db::db::page_service_client::PageServiceClient;
use crate::protos::db::db::user_service_client::UserServiceClient;
use crate::protos::db::db::view_service_client::ViewServiceClient;
use comm::io::file::FilerReader;
use comm::io::Filer;

pub mod database;
mod database_test;
pub mod disk;
pub mod index;
pub mod memory;
pub mod page;
mod page_test;
pub mod user;
pub mod view;

pub trait RpcClient {
    fn new(remote: &str, port: u16) -> GeorgeResult<Self>
    where
        Self: Sized;

    fn new_tls<P: AsRef<Path>>(
        remote: &str,
        port: u16,
        ca_path: P,
        domain_name: impl Into<String>,
    ) -> GeorgeResult<Self>
    where
        Self: Sized,
    {
        let ca = Filer::read_bytes(ca_path)?;
        RpcClient::new_tls_bytes(remote, port, ca, domain_name)
    }

    fn new_tls_bytes(
        remote: &str,
        port: u16,
        ca: Vec<u8>,
        domain_name: impl Into<String>,
    ) -> GeorgeResult<Self>
    where
        Self: Sized;

    fn new_tls_check<P: AsRef<Path>>(
        remote: &str,
        port: u16,
        key_path: P,
        cert_path: P,
        ca_path: P,
        domain_name: impl Into<String>,
    ) -> GeorgeResult<Self>
    where
        Self: Sized,
    {
        let key = Filer::read_bytes(ca_path)?;
        let cert = Filer::read_bytes(key_path)?;
        let ca = Filer::read_bytes(cert_path)?;
        RpcClient::new_tls_check_bytes(remote, port, key, cert, ca, domain_name)
    }

    fn new_tls_check_bytes(
        remote: &str,
        port: u16,
        key: Vec<u8>,
        cert: Vec<u8>,
        ca: Vec<u8>,
        domain_name: impl Into<String>,
    ) -> GeorgeResult<Self>
    where
        Self: Sized;
}

pub struct DatabaseRpcClient {
    client: DatabaseServiceClient<Channel>,
    rt: Runtime,
}

pub struct PageRpcClient {
    client: PageServiceClient<Channel>,
    rt: Runtime,
}

pub struct ViewRpcClient {
    client: ViewServiceClient<Channel>,
    rt: Runtime,
}

pub struct IndexRpcClient {
    client: IndexServiceClient<Channel>,
    rt: Runtime,
}

pub struct DiskRpcClient {
    client: DiskServiceClient<Channel>,
    rt: Runtime,
}

pub struct MemoryRpcClient {
    client: MemoryServiceClient<Channel>,
    rt: Runtime,
}

pub struct UserRpcClient {
    client: UserServiceClient<Channel>,
    rt: Runtime,
}
