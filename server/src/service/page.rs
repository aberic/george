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
    Page, PageList, RequestPageCreate, RequestPageInfo, RequestPageModify, RequestPageRemove,
    ResponsePageInfo,
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
        match self.task.page_create(
            req.message.get_name().to_string(),
            req.message.get_comment().to_string(),
            req.message.get_size(),
            req.message.get_period(),
        ) {
            Ok(()) => resp.finish(Response::new()),
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
        match self
            .task
            .page_modify(req.message.name, req.message.name_new)
        {
            Ok(()) => resp.finish(Response::new()),
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
        let mut info = ResponsePageInfo::new();
        let mut item = Page::new();
        match self.task.page(req.message.name) {
            Ok(res) => {
                let item_r = res.read().unwrap();
                item.set_name(item_r.name());
                item.set_comment(item_r.comment());
                item.set_size(item_r.size());
                item.set_period(item_r.period());
                item.set_create_time(Comm::time_2_grpc_timestamp(item_r.create_time()));
                info.set_page(item);
                resp.finish(info)
            }
            Err(err) => Err(Error::GrpcMessage(GrpcMessageError {
                grpc_status: 0,
                grpc_message: err.to_string(),
            })),
        }
    }

    fn page_remove(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestPageRemove>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        match self.task.page_remove(req.message.name) {
            Ok(()) => resp.finish(Response::new()),
            Err(err) => Err(Error::GrpcMessage(GrpcMessageError {
                grpc_status: 0,
                grpc_message: err.to_string(),
            })),
        }
    }
}
