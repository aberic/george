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
use crate::protos::db::db::view_service_server::ViewService;
use crate::protos::db::db::{
    RequestViewArchive, RequestViewCreate, RequestViewInfo, RequestViewList, RequestViewModify,
    RequestViewRecord, RequestViewRecords, RequestViewRemove, ResponseViewInfo, ResponseViewRecord,
    ResponseViewRecords, ViewList,
};
use crate::server::db::ViewServer;

impl ViewServer {
    pub fn new(task: Arc<Task>) -> Self {
        ViewServer { task }
    }
}

#[tonic::async_trait]
impl ViewService for ViewServer {
    async fn list(&self, request: Request<RequestViewList>) -> Result<Response<ViewList>, Status> {
        todo!()
    }

    async fn create(&self, request: Request<RequestViewCreate>) -> Result<Response<Resp>, Status> {
        todo!()
    }

    async fn modify(&self, request: Request<RequestViewModify>) -> Result<Response<Resp>, Status> {
        todo!()
    }

    async fn info(
        &self,
        request: Request<RequestViewInfo>,
    ) -> Result<Response<ResponseViewInfo>, Status> {
        todo!()
    }

    async fn remove(&self, request: Request<RequestViewRemove>) -> Result<Response<Resp>, Status> {
        todo!()
    }

    async fn archive(
        &self,
        request: Request<RequestViewArchive>,
    ) -> Result<Response<Resp>, Status> {
        todo!()
    }

    async fn record(
        &self,
        request: Request<RequestViewRecord>,
    ) -> Result<Response<ResponseViewRecord>, Status> {
        todo!()
    }

    async fn records(
        &self,
        request: Request<RequestViewRecords>,
    ) -> Result<Response<ResponseViewRecords>, Status> {
        todo!()
    }
}
