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

use grpc::{
    Error, GrpcMessageError, Result, ServerHandlerContext, ServerRequestSingle,
    ServerResponseUnarySink,
};

use db::task::traits::TMaster;
use db::Task;
use protobuf::RepeatedField;
use protocols::impls::db::disk::{
    RequestDiskDelete, RequestDiskIOut, RequestDiskInto, RequestDiskOut, RequestDiskRemove,
    RequestDiskSelect, ResponseDiskDelete, ResponseDiskOut, ResponseDiskSelect,
};
use protocols::impls::db::service::Response;
use protocols::impls::db::service_grpc::DiskService;

pub(crate) struct DiskServer {
    pub(crate) task: Arc<Task>,
}

impl DiskService for DiskServer {
    fn put(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestDiskInto>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        match self.task.put_disk(
            req.message.database_name,
            req.message.view_name,
            req.message.key,
            req.message.value,
        ) {
            Ok(()) => resp.finish(Response::new()),
            Err(err) => Err(Error::GrpcMessage(GrpcMessageError {
                grpc_status: 0,
                grpc_message: err.to_string(),
            })),
        }
    }

    fn set(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestDiskInto>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        match self.task.set_disk(
            req.message.database_name,
            req.message.view_name,
            req.message.key,
            req.message.value,
        ) {
            Ok(()) => resp.finish(Response::new()),
            Err(err) => Err(Error::GrpcMessage(GrpcMessageError {
                grpc_status: 0,
                grpc_message: err.to_string(),
            })),
        }
    }

    fn get(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestDiskOut>,
        resp: ServerResponseUnarySink<ResponseDiskOut>,
    ) -> Result<()> {
        match self.task.get_disk(
            req.message.database_name,
            req.message.view_name,
            req.message.key,
        ) {
            Ok(v8s) => {
                let mut response = ResponseDiskOut::new();
                response.set_value(v8s);
                resp.finish(response)
            }
            Err(err) => Err(Error::GrpcMessage(GrpcMessageError {
                grpc_status: 0,
                grpc_message: err.to_string(),
            })),
        }
    }

    fn get_by_index(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestDiskIOut>,
        resp: ServerResponseUnarySink<ResponseDiskOut>,
    ) -> Result<()> {
        match self.task.get_disk_by_index(
            req.message.database_name,
            req.message.view_name,
            req.message.index_name,
            req.message.key,
        ) {
            Ok(v8s) => {
                let mut response = ResponseDiskOut::new();
                response.set_value(v8s);
                resp.finish(response)
            }
            Err(err) => Err(Error::GrpcMessage(GrpcMessageError {
                grpc_status: 0,
                grpc_message: err.to_string(),
            })),
        }
    }

    fn remove(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestDiskRemove>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        match self.task.remove_disk(
            req.message.database_name,
            req.message.view_name,
            req.message.key,
        ) {
            Ok(()) => resp.finish(Response::new()),
            Err(err) => Err(Error::GrpcMessage(GrpcMessageError {
                grpc_status: 0,
                grpc_message: err.to_string(),
            })),
        }
    }

    fn select(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestDiskSelect>,
        resp: ServerResponseUnarySink<ResponseDiskSelect>,
    ) -> Result<()> {
        match self.task.select_disk(
            req.message.database_name,
            req.message.view_name,
            req.message.constraint_json_bytes,
        ) {
            Ok(exp) => {
                let mut response = ResponseDiskSelect::new();
                response.set_total(exp.total);
                response.set_count(exp.count);
                response.set_index_name(exp.index_name);
                response.set_asc(exp.asc);
                response.set_values(RepeatedField::from(exp.values));
                resp.finish(response)
            }
            Err(err) => Err(Error::GrpcMessage(GrpcMessageError {
                grpc_status: 0,
                grpc_message: err.to_string(),
            })),
        }
    }

    fn delete(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestDiskDelete>,
        resp: ServerResponseUnarySink<ResponseDiskDelete>,
    ) -> Result<()> {
        match self.task.delete_disk(
            req.message.database_name,
            req.message.view_name,
            req.message.constraint_json_bytes,
        ) {
            Ok(exp) => {
                let mut response = ResponseDiskDelete::new();
                response.set_total(exp.total);
                response.set_count(exp.count);
                response.set_index_name(exp.index_name);
                response.set_asc(exp.asc);
                response.set_values(RepeatedField::from(exp.values));
                resp.finish(response)
            }
            Err(err) => Err(Error::GrpcMessage(GrpcMessageError {
                grpc_status: 0,
                grpc_message: err.to_string(),
            })),
        }
    }
}
