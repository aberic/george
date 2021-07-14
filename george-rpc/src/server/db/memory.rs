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

use george_db::Task;

use crate::protos::db::db::memory_service_server::MemoryService;
use crate::protos::db::db::{
    RequestMemoryInto, RequestMemoryOut, RequestMemoryPInto, RequestMemoryPOut,
    RequestMemoryPRemove, RequestMemoryRemove, ResponseMemoryOut, ResponseMemoryPOut,
};
use crate::protos::utils::utils::Resp;
use crate::server::db::{MemoryServer, DATABASE_SYS};
use crate::tools::Results;
use george_db::task::traits::TMaster;

impl MemoryServer {
    pub fn new(task: Arc<Task>) -> Self {
        MemoryServer { task }
    }
}

#[tonic::async_trait]
impl MemoryService for MemoryServer {
    async fn put(&self, request: Request<RequestMemoryInto>) -> Result<Response<Resp>, Status> {
        match self.task.put_memory(
            DATABASE_SYS.to_string(),
            request.get_ref().key.clone(),
            request.get_ref().value.clone(),
        ) {
            Ok(()) => Results::success(),
            Err(err) => Results::failed_errs(err),
        }
    }

    async fn set(&self, request: Request<RequestMemoryInto>) -> Result<Response<Resp>, Status> {
        match self.task.set_memory(
            DATABASE_SYS.to_string(),
            request.get_ref().key.clone(),
            request.get_ref().value.clone(),
        ) {
            Ok(()) => Results::success(),
            Err(err) => Results::failed_errs(err),
        }
    }

    async fn get(
        &self,
        request: Request<RequestMemoryOut>,
    ) -> Result<Response<ResponseMemoryOut>, Status> {
        let resp;
        match self
            .task
            .get_memory(DATABASE_SYS.to_string(), request.get_ref().key.clone())
        {
            Ok(v8s) => {
                resp = ResponseMemoryOut {
                    status: Results::success_status(),
                    msg_err: "".to_string(),
                    value: v8s,
                }
            }
            Err(err) => {
                resp = ResponseMemoryOut {
                    status: Results::failed_status(err.clone()),
                    msg_err: err.to_string(),
                    value: vec![],
                };
            }
        }
        Results::response(resp)
    }

    async fn remove(
        &self,
        request: Request<RequestMemoryRemove>,
    ) -> Result<Response<Resp>, Status> {
        match self
            .task
            .remove_memory(DATABASE_SYS.to_string(), request.get_ref().key.clone())
        {
            Ok(()) => Results::success(),
            Err(err) => Results::failed_errs(err),
        }
    }

    async fn put_by_page(
        &self,
        request: Request<RequestMemoryPInto>,
    ) -> Result<Response<Resp>, Status> {
        match self.task.put_memory(
            request.get_ref().page_name.clone(),
            request.get_ref().key.clone(),
            request.get_ref().value.clone(),
        ) {
            Ok(()) => Results::success(),
            Err(err) => Results::failed_errs(err),
        }
    }

    async fn set_by_page(
        &self,
        request: Request<RequestMemoryPInto>,
    ) -> Result<Response<Resp>, Status> {
        match self.task.set_memory(
            request.get_ref().page_name.clone(),
            request.get_ref().key.clone(),
            request.get_ref().value.clone(),
        ) {
            Ok(()) => Results::success(),
            Err(err) => Results::failed_errs(err),
        }
    }

    async fn get_by_page(
        &self,
        request: Request<RequestMemoryPOut>,
    ) -> Result<Response<ResponseMemoryPOut>, Status> {
        let resp;
        match self.task.get_memory(
            request.get_ref().page_name.clone(),
            request.get_ref().key.clone(),
        ) {
            Ok(v8s) => {
                resp = ResponseMemoryPOut {
                    status: Results::success_status(),
                    msg_err: "".to_string(),
                    value: v8s,
                }
            }
            Err(err) => {
                resp = ResponseMemoryPOut {
                    status: Results::failed_status(err.clone()),
                    msg_err: err.to_string(),
                    value: vec![],
                };
            }
        }
        Results::response(resp)
    }

    async fn remove_by_page(
        &self,
        request: Request<RequestMemoryPRemove>,
    ) -> Result<Response<Resp>, Status> {
        match self.task.remove_memory(
            request.get_ref().page_name.clone(),
            request.get_ref().key.clone(),
        ) {
            Ok(()) => Results::success(),
            Err(err) => Results::failed_errs(err),
        }
    }
}
