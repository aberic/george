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

use grpc::{Result, ServerHandlerContext, ServerRequestSingle, ServerResponseUnarySink};

use db::task::traits::TMaster;
use db::Task;
use protocols::impls::comm::response::{Response, Status};
use protocols::impls::db::memory::{
    RequestMemoryInto, RequestMemoryOut, RequestMemoryPInto, RequestMemoryPOut,
    RequestMemoryPRemove, RequestMemoryRemove, ResponseMemoryOut, ResponseMemoryPOut,
};
use protocols::impls::db::service_grpc::MemoryService;
use protocols::impls::utils::Comm;

pub(crate) struct MemoryServer {
    pub(crate) task: Arc<Task>,
}

impl MemoryService for MemoryServer {
    fn put(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestMemoryInto>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        match self
            .task
            .put_memory_default(req.message.key, req.message.value)
        {
            Ok(()) => resp.finish(Comm::proto_success_db()),
            Err(err) => resp.finish(Comm::proto_failed_db_custom(err.to_string())),
        }
    }

    fn set(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestMemoryInto>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        match self
            .task
            .set_memory_default(req.message.key, req.message.value)
        {
            Ok(()) => resp.finish(Comm::proto_success_db()),
            Err(err) => resp.finish(Comm::proto_failed_db_custom(err.to_string())),
        }
    }

    fn get(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestMemoryOut>,
        resp: ServerResponseUnarySink<ResponseMemoryOut>,
    ) -> Result<()> {
        let mut response = ResponseMemoryOut::new();
        match self.task.get_memory_default(req.message.key) {
            Ok(res) => {
                response.set_value(res);
                response.set_status(Status::Ok);
            }
            Err(err) => {
                response.set_status(Status::Custom);
                response.set_msg_err(err.to_string());
            }
        }
        resp.finish(response)
    }

    fn remove(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestMemoryRemove>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        match self.task.remove_memory_default(req.message.key) {
            Ok(()) => resp.finish(Comm::proto_success_db()),
            Err(err) => resp.finish(Comm::proto_failed_db_custom(err.to_string())),
        }
    }

    fn put_by_page(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestMemoryPInto>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        match self
            .task
            .put_memory(req.message.page_name, req.message.key, req.message.value)
        {
            Ok(()) => resp.finish(Comm::proto_success_db()),
            Err(err) => resp.finish(Comm::proto_failed_db_custom(err.to_string())),
        }
    }

    fn set_by_page(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestMemoryPInto>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        match self
            .task
            .set_memory(req.message.page_name, req.message.key, req.message.value)
        {
            Ok(()) => resp.finish(Comm::proto_success_db()),
            Err(err) => resp.finish(Comm::proto_failed_db_custom(err.to_string())),
        }
    }

    fn get_by_page(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestMemoryPOut>,
        resp: ServerResponseUnarySink<ResponseMemoryPOut>,
    ) -> Result<()> {
        let mut response = ResponseMemoryPOut::new();
        match self.task.get_memory(req.message.page_name, req.message.key) {
            Ok(res) => {
                response.set_value(res);
                response.set_status(Status::Ok);
            }
            Err(err) => {
                response.set_status(Status::Custom);
                response.set_msg_err(err.to_string());
            }
        }
        resp.finish(response)
    }

    fn remove_by_page(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestMemoryPRemove>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        match self
            .task
            .remove_memory(req.message.page_name, req.message.key)
        {
            Ok(()) => resp.finish(Comm::proto_success_db()),
            Err(err) => resp.finish(Comm::proto_failed_db_custom(err.to_string())),
        }
    }
}
