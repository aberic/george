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
use protobuf::RepeatedField;
use protocols::impls::db::disk::{
    DiskDeleted, DiskSelected, RequestDiskDelete, RequestDiskIOut, RequestDiskInto, RequestDiskOut,
    RequestDiskRemove, RequestDiskSelect, ResponseDiskDelete, ResponseDiskOut, ResponseDiskSelect,
};
use protocols::impls::db::response::{Response, Status};
use protocols::impls::db::service_grpc::DiskService;
use protocols::impls::utils::Comm;

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
            Ok(()) => resp.finish(Comm::proto_success_db()),
            Err(err) => resp.finish(Comm::proto_failed_db_custom(err.to_string())),
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
            Ok(()) => resp.finish(Comm::proto_success_db()),
            Err(err) => resp.finish(Comm::proto_failed_db_custom(err.to_string())),
        }
    }

    fn get(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestDiskOut>,
        resp: ServerResponseUnarySink<ResponseDiskOut>,
    ) -> Result<()> {
        let mut response = ResponseDiskOut::new();
        match self.task.get_disk(
            req.message.database_name,
            req.message.view_name,
            req.message.key,
        ) {
            Ok(v8s) => {
                response.set_value(v8s);
                response.set_status(Status::Ok);
            }
            Err(err) => {
                response.set_status(Status::Custom);
                response.set_msg_err(err.to_string());
            }
        }
        resp.finish(response)
    }

    fn get_by_index(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestDiskIOut>,
        resp: ServerResponseUnarySink<ResponseDiskOut>,
    ) -> Result<()> {
        let mut response = ResponseDiskOut::new();
        match self.task.get_disk_by_index(
            req.message.database_name,
            req.message.view_name,
            req.message.index_name,
            req.message.key,
        ) {
            Ok(v8s) => {
                response.set_value(v8s);
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
        req: ServerRequestSingle<RequestDiskRemove>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        match self.task.remove_disk(
            req.message.database_name,
            req.message.view_name,
            req.message.key,
        ) {
            Ok(()) => resp.finish(Comm::proto_success_db()),
            Err(err) => resp.finish(Comm::proto_failed_db_custom(err.to_string())),
        }
    }

    fn select(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestDiskSelect>,
        resp: ServerResponseUnarySink<ResponseDiskSelect>,
    ) -> Result<()> {
        let mut response = ResponseDiskSelect::new();
        match self.task.select_disk(
            req.message.database_name,
            req.message.view_name,
            req.message.constraint_json_bytes,
        ) {
            Ok(exp) => {
                let mut selected = DiskSelected::new();
                selected.set_total(exp.total);
                selected.set_count(exp.count);
                selected.set_index_name(exp.index_name);
                selected.set_asc(exp.asc);
                selected.set_values(RepeatedField::from(exp.values));
                response.set_selected(selected);
                response.set_status(Status::Ok);
            }
            Err(err) => {
                response.set_status(Status::Custom);
                response.set_msg_err(err.to_string());
            }
        }
        resp.finish(response)
    }

    fn delete(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestDiskDelete>,
        resp: ServerResponseUnarySink<ResponseDiskDelete>,
    ) -> Result<()> {
        let mut response = ResponseDiskDelete::new();
        match self.task.delete_disk(
            req.message.database_name,
            req.message.view_name,
            req.message.constraint_json_bytes,
        ) {
            Ok(exp) => {
                let mut deleted = DiskDeleted::new();
                deleted.set_total(exp.total);
                deleted.set_count(exp.count);
                deleted.set_index_name(exp.index_name);
                deleted.set_asc(exp.asc);
                deleted.set_values(RepeatedField::from(exp.values));
                response.set_deleted(deleted);
                response.set_status(Status::Ok);
            }
            Err(err) => {
                response.set_status(Status::Custom);
                response.set_msg_err(err.to_string());
            }
        }
        resp.finish(response)
    }
}
