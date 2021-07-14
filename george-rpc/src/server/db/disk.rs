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

use george_db::task::traits::TMaster;
use george_db::Task;

use crate::protos::db::db::disk_service_server::DiskService;
use crate::protos::db::db::{
    DiskDeleted, DiskSelected, RequestDiskDelete, RequestDiskIOut, RequestDiskInto, RequestDiskOut,
    RequestDiskRemove, RequestDiskSelect, ResponseDiskDelete, ResponseDiskOut, ResponseDiskSelect,
};
use crate::protos::utils::utils::Resp;
use crate::server::db::DiskServer;
use crate::tools::Results;

impl DiskServer {
    pub fn new(task: Arc<Task>) -> Self {
        DiskServer { task }
    }
}

#[tonic::async_trait]
impl DiskService for DiskServer {
    async fn put(&self, request: Request<RequestDiskInto>) -> Result<Response<Resp>, Status> {
        match self.task.put_disk(
            request.get_ref().database_name.clone(),
            request.get_ref().view_name.clone(),
            request.get_ref().key.clone(),
            request.get_ref().value.clone(),
        ) {
            Ok(()) => Results::success(),
            Err(err) => Results::failed_errs(err),
        }
    }

    async fn set(&self, request: Request<RequestDiskInto>) -> Result<Response<Resp>, Status> {
        match self.task.set_disk(
            request.get_ref().database_name.clone(),
            request.get_ref().view_name.clone(),
            request.get_ref().key.clone(),
            request.get_ref().value.clone(),
        ) {
            Ok(()) => Results::success(),
            Err(err) => Results::failed_errs(err),
        }
    }

    async fn get(
        &self,
        request: Request<RequestDiskOut>,
    ) -> Result<Response<ResponseDiskOut>, Status> {
        let resp;
        match self.task.get_disk(
            request.get_ref().database_name.clone(),
            request.get_ref().view_name.clone(),
            request.get_ref().key.clone(),
        ) {
            Ok(v8s) => {
                resp = ResponseDiskOut {
                    status: Results::success_status(),
                    msg_err: "".to_string(),
                    value: v8s,
                }
            }
            Err(err) => {
                resp = ResponseDiskOut {
                    status: Results::failed_status(err.clone()),
                    msg_err: err.to_string(),
                    value: vec![],
                };
            }
        }
        Results::response(resp)
    }

    async fn get_by_index(
        &self,
        request: Request<RequestDiskIOut>,
    ) -> Result<Response<ResponseDiskOut>, Status> {
        let resp;
        match self.task.get_disk_by_index(
            request.get_ref().database_name.clone(),
            request.get_ref().view_name.clone(),
            request.get_ref().index_name.clone(),
            request.get_ref().key.clone(),
        ) {
            Ok(v8s) => {
                resp = ResponseDiskOut {
                    status: Results::success_status(),
                    msg_err: "".to_string(),
                    value: v8s,
                }
            }
            Err(err) => {
                resp = ResponseDiskOut {
                    status: Results::failed_status(err.clone()),
                    msg_err: err.to_string(),
                    value: vec![],
                };
            }
        }
        Results::response(resp)
    }

    async fn remove(&self, request: Request<RequestDiskRemove>) -> Result<Response<Resp>, Status> {
        match self.task.remove_disk(
            request.get_ref().database_name.clone(),
            request.get_ref().view_name.clone(),
            request.get_ref().key.clone(),
        ) {
            Ok(()) => Results::success(),
            Err(err) => Results::failed_errs(err),
        }
    }

    async fn select(
        &self,
        request: Request<RequestDiskSelect>,
    ) -> Result<Response<ResponseDiskSelect>, Status> {
        let resp;
        match self.task.select_disk(
            request.get_ref().database_name.clone(),
            request.get_ref().view_name.clone(),
            request.get_ref().constraint_json_bytes.clone(),
        ) {
            Ok(exp) => {
                let selected = DiskSelected {
                    total: exp.total,
                    count: exp.count,
                    index_name: exp.index_name.clone(),
                    asc: exp.asc,
                    values: exp.values,
                };
                resp = ResponseDiskSelect {
                    status: Results::success_status(),
                    msg_err: "".to_string(),
                    selected: Some(selected),
                }
            }
            Err(err) => {
                resp = ResponseDiskSelect {
                    status: Results::failed_status(err.clone()),
                    msg_err: err.to_string(),
                    selected: None,
                };
            }
        }
        Results::response(resp)
    }

    async fn delete(
        &self,
        request: Request<RequestDiskDelete>,
    ) -> Result<Response<ResponseDiskDelete>, Status> {
        let resp;
        match self.task.delete_disk(
            request.get_ref().database_name.clone(),
            request.get_ref().view_name.clone(),
            request.get_ref().constraint_json_bytes.clone(),
        ) {
            Ok(exp) => {
                let deleted = DiskDeleted {
                    total: exp.total,
                    count: exp.count,
                    index_name: exp.index_name.clone(),
                    asc: exp.asc,
                    values: exp.values,
                };
                resp = ResponseDiskDelete {
                    status: Results::success_status(),
                    msg_err: "".to_string(),
                    deleted: Some(deleted),
                }
            }
            Err(err) => {
                resp = ResponseDiskDelete {
                    status: Results::failed_status(err.clone()),
                    msg_err: err.to_string(),
                    deleted: None,
                };
            }
        }
        Results::response(resp)
    }
}
