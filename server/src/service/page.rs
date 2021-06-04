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
use protobuf::RepeatedField;

use db::task::traits::TMaster;
use db::Task;
use protocols::impls::db::page::{
    Page, PageList, RequestPageCreate, RequestPageInfo, RequestPageModify, ResponsePageInfo,
};
use protocols::impls::db::service::{Request, Response};
use protocols::impls::db::service_grpc::PageService;

use crate::utils::Comm;

pub(crate) struct PageServer {
    pub(crate) task: Arc<Task>,
}

impl PageService for PageServer {
    fn pages(
        &self,
        _o: ServerHandlerContext,
        _req: ServerRequestSingle<Request>,
        resp: ServerResponseUnarySink<PageList>,
    ) -> Result<()> {
        let mut list = PageList::new();
        let mut pages: RepeatedField<Page> = RepeatedField::new();
        let page_map = self.task.page_map();
        let page_map_r = page_map.read().unwrap();
        for page in page_map_r.values() {
            let page_r = page.read().unwrap();
            let mut page_item = Page::new();
            page_item.set_name(page_r.name());
            page_item.set_comment(page_r.comment());
            page_item.set_size(page_r.size());
            page_item.set_period(page_r.period());
            page_item.set_create_time(Comm::time_2_grpc_timestamp(page_r.create_time()));
            pages.push(page_item);
        }
        list.set_pages(pages);
        resp.finish(list)
    }

    fn page_create(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestPageCreate>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        let response = Response::new();
        match self.task.create_page(
            req.message.get_name().to_string(),
            req.message.get_comment().to_string(),
            req.message.get_size(),
            req.message.get_period(),
        ) {
            Ok(()) => resp.finish(response),
            Err(err) => Err(Error::GrpcMessage(GrpcMessageError {
                grpc_status: 0,
                grpc_message: err.to_string(),
            })),
        }
    }

    fn page_modify(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestPageModify>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        let response = Response::new();
        match self
            .task
            .modify_page(req.message.name, req.message.name_new)
        {
            Ok(()) => resp.finish(response),
            Err(err) => Err(Error::GrpcMessage(GrpcMessageError {
                grpc_status: 0,
                grpc_message: err.to_string(),
            })),
        }
    }

    fn page_info(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestPageInfo>,
        resp: ServerResponseUnarySink<ResponsePageInfo>,
    ) -> Result<()> {
        let mut page_info = ResponsePageInfo::new();
        let mut page = Page::new();
        match self.task.page(req.message.page_name) {
            Ok(p) => {
                let page_r = p.read().unwrap();
                page.set_name(page_r.name());
                page.set_comment(page_r.comment());
                page.set_size(page_r.size());
                page.set_period(page_r.period());
                page.set_create_time(Comm::time_2_grpc_timestamp(page_r.create_time()));
                page_info.set_page(page);
                resp.finish(page_info)
            }
            Err(err) => Err(Error::GrpcMessage(GrpcMessageError {
                grpc_status: 0,
                grpc_message: err.to_string(),
            })),
        }
    }
}
