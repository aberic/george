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

use tokio::runtime::Runtime;
use tonic::transport::Channel;

use crate::protos::db::db::database_service_client::DatabaseServiceClient;
use crate::protos::db::db::disk_service_client::DiskServiceClient;
use crate::protos::db::db::index_service_client::IndexServiceClient;
use crate::protos::db::db::memory_service_client::MemoryServiceClient;
use crate::protos::db::db::page_service_client::PageServiceClient;
use crate::protos::db::db::user_service_client::UserServiceClient;
use crate::protos::db::db::view_service_client::ViewServiceClient;

mod database;
mod database_test;
mod disk;
mod index;
mod memory;
mod page;
mod page_test;
mod user;
mod view;

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
