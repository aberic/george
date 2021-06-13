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
    Error, GrpcMessageError, GrpcStatus, Result, ServerHandlerContext, ServerRequestSingle,
    ServerResponseUnarySink,
};
use protobuf::RepeatedField;

use db::task::traits::{TForm, TMaster};
use db::Task;
use protocols::impls::db::response::Response;
use protocols::impls::db::service_grpc::ViewService;
use protocols::impls::db::view::{
    RequestViewArchive, RequestViewCreate, RequestViewInfo, RequestViewList, RequestViewModify,
    RequestViewRecord, RequestViewRemove, ResponseViewInfo, ResponseViewRecord, View, ViewList,
};

use crate::utils::Comm;

pub(crate) struct ViewServer {
    pub(crate) task: Arc<Task>,
}

impl ViewService for ViewServer {
    fn views(
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
                    let view_r = view.read().unwrap();
                    let mut view_item = View::new();
                    view_item.set_name(view_r.name());
                    view_item.set_comment(view_r.comment());
                    view_item
                        .set_create_time(Comm::proto_time_2_grpc_timestamp(view_r.create_time()));
                    views.push(view_item);
                }
                list.set_views(views);
            }
            _ => {}
        }
        resp.finish(list)
    }

    fn view_create(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestViewCreate>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        let response = Response::new();
        match self.task.view_create(
            req.message.database_name,
            req.message.name,
            req.message.comment,
            req.message.with_increment,
        ) {
            Ok(()) => resp.finish(response),
            Err(err) => Err(Error::GrpcMessage(GrpcMessageError {
                grpc_status: GrpcStatus::Ok as i32,
                grpc_message: err.to_string(),
            })),
        }
    }

    fn view_modify(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestViewModify>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        let response = Response::new();
        match self.task.view_modify(
            req.message.database_name,
            req.message.name,
            req.message.name_new,
            req.message.comment,
        ) {
            Ok(()) => resp.finish(response),
            Err(err) => Err(Error::GrpcMessage(GrpcMessageError {
                grpc_status: GrpcStatus::Ok as i32,
                grpc_message: err.to_string(),
            })),
        }
    }

    fn view_info(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestViewInfo>,
        resp: ServerResponseUnarySink<ResponseViewInfo>,
    ) -> Result<()> {
        let mut info = ResponseViewInfo::new();
        let mut item = View::new();
        match self.task.view(req.message.database_name, req.message.name) {
            Ok(res) => {
                let item_r = res.read().unwrap();
                item.set_name(item_r.name());
                item.set_comment(item_r.comment());
                item.set_create_time(Comm::proto_time_2_grpc_timestamp(item_r.create_time()));
                info.set_view(item);
                resp.finish(info)
            }
            Err(err) => Err(Error::GrpcMessage(GrpcMessageError {
                grpc_status: GrpcStatus::Ok as i32,
                grpc_message: err.to_string(),
            })),
        }
    }

    fn view_remove(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestViewRemove>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        match self
            .task
            .view_remove(req.message.database_name, req.message.name)
        {
            Ok(()) => resp.finish(Response::new()),
            Err(err) => Err(Error::GrpcMessage(GrpcMessageError {
                grpc_status: GrpcStatus::Ok as i32,
                grpc_message: err.to_string(),
            })),
        }
    }

    fn view_archive(
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
            Ok(()) => resp.finish(Response::new()),
            Err(err) => Err(Error::GrpcMessage(GrpcMessageError {
                grpc_status: GrpcStatus::Ok as i32,
                grpc_message: err.to_string(),
            })),
        }
    }

    fn view_record(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestViewRecord>,
        resp: ServerResponseUnarySink<ResponseViewRecord>,
    ) -> Result<()> {
        match self.task.view_record(
            req.message.database_name,
            req.message.name,
            req.message.version as u16,
        ) {
            Ok((filepath, create_time)) => {
                let mut response = ResponseViewRecord::new();
                response.set_filepath(filepath);
                response.set_time(Comm::proto_time_2_grpc_timestamp(create_time));
                resp.finish(response)
            }
            Err(err) => Err(Error::GrpcMessage(GrpcMessageError {
                grpc_status: GrpcStatus::Ok as i32,
                grpc_message: err.to_string(),
            })),
        }
    }
}
