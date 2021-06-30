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

use crate::protos::db::db::disk_service_server::DiskService;
use crate::protos::db::db::{
    RequestDiskDelete, RequestDiskIOut, RequestDiskInto, RequestDiskOut, RequestDiskRemove,
    RequestDiskSelect, ResponseDiskDelete, ResponseDiskOut, ResponseDiskSelect,
};
use crate::protos::utils::utils::Resp;
use crate::server::db::DiskServer;

impl DiskServer {
    pub fn new(task: Arc<Task>) -> Self {
        DiskServer { task }
    }
}

#[tonic::async_trait]
impl DiskService for DiskServer {
    async fn put(&self, request: Request<RequestDiskInto>) -> Result<Response<Resp>, Status> {
        todo!()
    }

    async fn set(&self, request: Request<RequestDiskInto>) -> Result<Response<Resp>, Status> {
        todo!()
    }

    async fn get(
        &self,
        request: Request<RequestDiskOut>,
    ) -> Result<Response<ResponseDiskOut>, Status> {
        todo!()
    }

    async fn get_by_index(
        &self,
        request: Request<RequestDiskIOut>,
    ) -> Result<Response<ResponseDiskOut>, Status> {
        todo!()
    }

    async fn remove(&self, request: Request<RequestDiskRemove>) -> Result<Response<Resp>, Status> {
        todo!()
    }

    async fn select(
        &self,
        request: Request<RequestDiskSelect>,
    ) -> Result<Response<ResponseDiskSelect>, Status> {
        todo!()
    }

    async fn delete(
        &self,
        request: Request<RequestDiskDelete>,
    ) -> Result<Response<ResponseDiskDelete>, Status> {
        todo!()
    }
}
