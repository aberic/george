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
use crate::protos::db::db::index_service_server::IndexService;
use crate::protos::db::db::{
    IndexList, RequestIndexCreate, RequestIndexInfo, RequestIndexList, ResponseIndexInfo,
};
use crate::server::db::IndexServer;

impl IndexServer {
    pub fn new(task: Arc<Task>) -> Self {
        IndexServer { task }
    }
}

#[tonic::async_trait]
impl IndexService for IndexServer {
    async fn list(
        &self,
        request: Request<RequestIndexList>,
    ) -> Result<Response<IndexList>, Status> {
        todo!()
    }

    async fn create(&self, request: Request<RequestIndexCreate>) -> Result<Response<Resp>, Status> {
        todo!()
    }

    async fn info(
        &self,
        request: Request<RequestIndexInfo>,
    ) -> Result<Response<ResponseIndexInfo>, Status> {
        todo!()
    }
}
