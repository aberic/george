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

use db::task::traits::{TForm, TMaster};
use db::Task;

use crate::protos::db::db::view_service_server::ViewService;
use crate::protos::db::db::{
    RequestViewArchive, RequestViewCreate, RequestViewInfo, RequestViewList, RequestViewModify,
    RequestViewRecord, RequestViewRecords, RequestViewRemove, ResponseViewInfo, ResponseViewList,
    ResponseViewRecord, ResponseViewRecords, View, ViewRecord,
};
use crate::protos::utils::utils::Resp;
use crate::server::db::ViewServer;
use crate::tools::{Children, Results, Trans};

impl ViewServer {
    pub fn new(task: Arc<Task>) -> Self {
        ViewServer { task }
    }
}

#[tonic::async_trait]
impl ViewService for ViewServer {
    async fn list(
        &self,
        request: Request<RequestViewList>,
    ) -> Result<Response<ResponseViewList>, Status> {
        let resp;
        let mut views: Vec<View> = vec![];
        match self.task.view_map(request.get_ref().database_name.clone()) {
            Ok(view_map) => {
                let view_map_r = view_map.read().unwrap();
                for view in view_map_r.values() {
                    let indexes = Children::indexes(view.clone());
                    let view_r = view.read().unwrap();
                    views.push(View {
                        name: view_r.name(),
                        comment: view_r.comment(),
                        create_time: Some(Trans::time_2_grpc_timestamp(view_r.create_time())),
                        indexes,
                        filepath: view_r.filepath(),
                        version: view_r.version() as u32,
                    });
                }
                resp = ResponseViewList {
                    status: Results::success_status(),
                    msg_err: "".to_string(),
                    views,
                }
            }
            Err(err) => {
                resp = ResponseViewList {
                    status: Results::failed_status(err.clone()),
                    msg_err: err.to_string(),
                    views,
                };
            }
        }
        Results::response(resp)
    }

    async fn create(&self, request: Request<RequestViewCreate>) -> Result<Response<Resp>, Status> {
        match self.task.view_create(
            request.get_ref().database_name.clone(),
            request.get_ref().name.clone(),
            request.get_ref().comment.clone(),
            request.get_ref().with_increment,
        ) {
            Ok(()) => Results::success(),
            Err(err) => Results::failed_err(err),
        }
    }

    async fn modify(&self, request: Request<RequestViewModify>) -> Result<Response<Resp>, Status> {
        match self.task.view_modify(
            request.get_ref().database_name.clone(),
            request.get_ref().name.clone(),
            request.get_ref().name_new.clone(),
            request.get_ref().comment.clone(),
        ) {
            Ok(()) => Results::success(),
            Err(err) => Results::failed_err(err),
        }
    }

    async fn info(
        &self,
        request: Request<RequestViewInfo>,
    ) -> Result<Response<ResponseViewInfo>, Status> {
        let resp;
        match self.task.view(
            request.get_ref().database_name.clone(),
            request.get_ref().name.clone(),
        ) {
            Ok(res) => {
                let indexes = Children::indexes(res.clone());
                let view_r = res.read().unwrap();
                resp = ResponseViewInfo {
                    status: Results::success_status(),
                    msg_err: "".to_string(),
                    view: Some(View {
                        name: view_r.name(),
                        comment: view_r.comment(),
                        create_time: Some(Trans::time_2_grpc_timestamp(view_r.create_time())),
                        indexes,
                        filepath: view_r.filepath(),
                        version: view_r.version() as u32,
                    }),
                }
            }
            Err(err) => {
                resp = ResponseViewInfo {
                    status: Results::failed_status(err.clone()),
                    msg_err: err.to_string(),
                    view: None,
                };
            }
        }
        Results::response(resp)
    }

    async fn remove(&self, request: Request<RequestViewRemove>) -> Result<Response<Resp>, Status> {
        match self.task.view_remove(
            request.get_ref().database_name.clone(),
            request.get_ref().name.clone(),
        ) {
            Ok(()) => Results::success(),
            Err(err) => Results::failed_err(err),
        }
    }

    async fn archive(
        &self,
        request: Request<RequestViewArchive>,
    ) -> Result<Response<Resp>, Status> {
        match self.task.view_archive(
            request.get_ref().database_name.clone(),
            request.get_ref().name.clone(),
            request.get_ref().archive_file_path.clone(),
        ) {
            Ok(()) => Results::success(),
            Err(err) => Results::failed_err(err),
        }
    }

    async fn record(
        &self,
        request: Request<RequestViewRecord>,
    ) -> Result<Response<ResponseViewRecord>, Status> {
        let resp;
        match self.task.view_record(
            request.get_ref().database_name.clone(),
            request.get_ref().name.clone(),
            request.get_ref().version as u16,
        ) {
            Ok((filepath, create_time)) => {
                resp = ResponseViewRecord {
                    status: Results::success_status(),
                    msg_err: "".to_string(),
                    record: Some(ViewRecord {
                        filepath,
                        time: Some(Trans::time_2_grpc_timestamp(create_time)),
                        version: request.get_ref().version,
                    }),
                }
            }
            Err(err) => {
                resp = ResponseViewRecord {
                    status: Results::failed_status(err.clone()),
                    msg_err: err.to_string(),
                    record: None,
                };
            }
        }
        Results::response(resp)
    }

    async fn records(
        &self,
        request: Request<RequestViewRecords>,
    ) -> Result<Response<ResponseViewRecords>, Status> {
        let resp;
        let mut records: Vec<ViewRecord> = vec![];
        match self.task.view_records(
            request.get_ref().database_name.clone(),
            request.get_ref().name.clone(),
        ) {
            Ok(v8s) => {
                for (filepath, time, version) in v8s {
                    records.push(ViewRecord {
                        filepath,
                        time: Some(Trans::time_2_grpc_timestamp(time)),
                        version: version as u32,
                    });
                }
                resp = ResponseViewRecords {
                    status: Results::success_status(),
                    msg_err: "".to_string(),
                    records,
                }
            }
            Err(err) => {
                resp = ResponseViewRecords {
                    status: Results::failed_status(err.clone()),
                    msg_err: err.to_string(),
                    records,
                };
            }
        }
        Results::response(resp)
    }
}
