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
use protobuf::RepeatedField;

use db::task::traits::{TForm, TMaster};
use db::Task;
use protocols::impls::comm::response::{Response, Status};
use protocols::impls::db::service_grpc::ViewService;
use protocols::impls::db::view::{
    RequestViewArchive, RequestViewCreate, RequestViewInfo, RequestViewList, RequestViewModify,
    RequestViewRecord, RequestViewRecords, RequestViewRemove, ResponseViewInfo, ResponseViewRecord,
    ResponseViewRecords, View, ViewList, ViewRecord,
};
use protocols::impls::utils::Comm;

use crate::service::Children;

pub(crate) struct ViewServer {
    pub(crate) task: Arc<Task>,
}

impl ViewService for ViewServer {
    fn list(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestViewList>,
        resp: ServerResponseUnarySink<ViewList>,
    ) -> Result<()> {
        let mut list = ViewList::new();
        let mut views: RepeatedField<View> = RepeatedField::new();
        match self.task.view_map(req.message.database_name) {
            Ok(view_map) => {
                let view_map_r = view_map.read().unwrap();
                for view in view_map_r.values() {
                    let indexes = Children::indexes(view.clone());
                    let view_r = view.read().unwrap();
                    let mut view_item = View::new();
                    view_item.set_name(view_r.name());
                    view_item.set_comment(view_r.comment());
                    view_item
                        .set_create_time(Comm::proto_time_2_grpc_timestamp(view_r.create_time()));
                    view_item.set_indexes(indexes);
                    views.push(view_item);
                }
                list.set_views(views);
            }
            _ => {}
        }
        resp.finish(list)
    }

    fn create(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestViewCreate>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        match self.task.view_create(
            req.message.database_name,
            req.message.name,
            req.message.comment,
            req.message.with_increment,
        ) {
            Ok(()) => resp.finish(Comm::proto_success_db()),
            Err(err) => resp.finish(Comm::proto_failed_db_custom(err.to_string())),
        }
    }

    fn modify(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestViewModify>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        match self.task.view_modify(
            req.message.database_name,
            req.message.name,
            req.message.name_new,
            req.message.comment,
        ) {
            Ok(()) => resp.finish(Comm::proto_success_db()),
            Err(err) => resp.finish(Comm::proto_failed_db_custom(err.to_string())),
        }
    }

    fn info(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestViewInfo>,
        resp: ServerResponseUnarySink<ResponseViewInfo>,
    ) -> Result<()> {
        let mut response = ResponseViewInfo::new();
        let mut item = View::new();
        match self.task.view(req.message.database_name, req.message.name) {
            Ok(res) => {
                let indexes = Children::indexes(res.clone());
                let item_r = res.read().unwrap();
                item.set_name(item_r.name());
                item.set_comment(item_r.comment());
                item.set_create_time(Comm::proto_time_2_grpc_timestamp(item_r.create_time()));
                item.set_indexes(indexes);
                item.set_filepath(item_r.filepath());
                item.set_version(item_r.version() as u32);
                response.set_view(item);
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
        req: ServerRequestSingle<RequestViewRemove>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        match self
            .task
            .view_remove(req.message.database_name, req.message.name)
        {
            Ok(()) => resp.finish(Comm::proto_success_db()),
            Err(err) => resp.finish(Comm::proto_failed_db_custom(err.to_string())),
        }
    }

    fn archive(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestViewArchive>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        match self.task.view_archive(
            req.message.database_name,
            req.message.name,
            req.message.archive_file_path,
        ) {
            Ok(()) => resp.finish(Comm::proto_success_db()),
            Err(err) => resp.finish(Comm::proto_failed_db_custom(err.to_string())),
        }
    }

    fn record(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestViewRecord>,
        resp: ServerResponseUnarySink<ResponseViewRecord>,
    ) -> Result<()> {
        let mut response = ResponseViewRecord::new();
        match self.task.view_record(
            req.message.database_name,
            req.message.name,
            req.message.version as u16,
        ) {
            Ok((filepath, create_time)) => {
                let mut record = ViewRecord::new();
                record.set_filepath(filepath);
                record.set_time(Comm::proto_time_2_grpc_timestamp(create_time));
                record.set_version(req.message.version);
                response.set_record(record);
                response.set_status(Status::Ok);
            }
            Err(err) => {
                response.set_status(Status::Custom);
                response.set_msg_err(err.to_string());
            }
        }
        resp.finish(response)
    }

    fn records(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestViewRecords>,
        resp: ServerResponseUnarySink<ResponseViewRecords>,
    ) -> Result<()> {
        let mut response = ResponseViewRecords::new();
        let mut records: RepeatedField<ViewRecord> = RepeatedField::new();
        match self
            .task
            .view_records(req.message.database_name, req.message.name)
        {
            Ok(v8s) => {
                for (filepath, time, version) in v8s {
                    let mut record = ViewRecord::new();
                    record.set_filepath(filepath);
                    record.set_time(Comm::proto_time_2_grpc_timestamp(time));
                    record.set_version(version as u32);
                    records.push(record);
                }
                response.set_records(records);
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
