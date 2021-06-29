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

use tonic::{Request, Response, Status};

use db::Task;

use crate::protos::chain::utils::Resp;
use crate::protos::db::db::memory_service_server::MemoryService;
use crate::protos::db::db::{
    RequestMemoryInto, RequestMemoryOut, RequestMemoryPInto, RequestMemoryPOut,
    RequestMemoryPRemove, RequestMemoryRemove, ResponseMemoryOut, ResponseMemoryPOut,
};
use crate::server::db::MemoryServer;

impl MemoryServer {
    pub fn new(task: Arc<Task>) -> Self {
        MemoryServer { task }
    }
}

#[tonic::async_trait]
impl MemoryService for MemoryServer {
    async fn put(&self, request: Request<RequestMemoryInto>) -> Result<Response<Resp>, Status> {
        todo!()
    }

    async fn set(&self, request: Request<RequestMemoryInto>) -> Result<Response<Resp>, Status> {
        todo!()
    }

    async fn get(
        &self,
        request: Request<RequestMemoryOut>,
    ) -> Result<Response<ResponseMemoryOut>, Status> {
        todo!()
    }

    async fn remove(
        &self,
        request: Request<RequestMemoryRemove>,
    ) -> Result<Response<Resp>, Status> {
        todo!()
    }

    async fn put_by_page(
        &self,
        request: Request<RequestMemoryPInto>,
    ) -> Result<Response<Resp>, Status> {
        todo!()
    }

    async fn set_by_page(
        &self,
        request: Request<RequestMemoryPInto>,
    ) -> Result<Response<Resp>, Status> {
        todo!()
    }

    async fn get_by_page(
        &self,
        request: Request<RequestMemoryPOut>,
    ) -> Result<Response<ResponseMemoryPOut>, Status> {
        todo!()
    }

    async fn remove_by_page(
        &self,
        request: Request<RequestMemoryPRemove>,
    ) -> Result<Response<Resp>, Status> {
        todo!()
    }
}
