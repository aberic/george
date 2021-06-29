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

use crate::protos::chain::utils::{Req, Resp};
use crate::protos::db::db::page_service_server::PageService;
use crate::protos::db::db::{
    PageList, RequestPageCreate, RequestPageInfo, RequestPageModify, RequestPageRemove,
    ResponsePageInfo,
};
use crate::server::db::PageServer;

impl PageServer {
    pub fn new(task: Arc<Task>) -> Self {
        PageServer { task }
    }
}

#[tonic::async_trait]
impl PageService for PageServer {
    async fn list(&self, request: Request<Req>) -> Result<Response<PageList>, Status> {
        todo!()
    }

    async fn create(&self, request: Request<RequestPageCreate>) -> Result<Response<Resp>, Status> {
        todo!()
    }

    async fn modify(&self, request: Request<RequestPageModify>) -> Result<Response<Resp>, Status> {
        todo!()
    }

    async fn info(
        &self,
        request: Request<RequestPageInfo>,
    ) -> Result<Response<ResponsePageInfo>, Status> {
        todo!()
    }

    async fn remove(&self, request: Request<RequestPageRemove>) -> Result<Response<Resp>, Status> {
        todo!()
    }
}
