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

use db::task::traits::TMaster;
use db::Task;
use protocols::impls::db::page::{
    Page, PageList, RequestPageCreate, RequestPageInfo, RequestPageModify, RequestPageRemove,
    ResponsePageInfo,
};
use protocols::impls::db::response::{Response, Status};
use protocols::impls::db::service::Request;
use protocols::impls::db::service_grpc::PageService;
use protocols::impls::utils::Comm;

pub(crate) struct PageServer {
    pub(crate) task: Arc<Task>,
}

impl PageService for PageServer {
    fn list(
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
            page_item.set_create_time(Comm::proto_time_2_grpc_timestamp(page_r.create_time()));
            pages.push(page_item);
        }
        list.set_pages(pages);
        resp.finish(list)
    }

    fn create(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestPageCreate>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        match self.task.page_create(
            req.message.name,
            req.message.comment,
            req.message.size,
            req.message.period,
        ) {
            Ok(()) => resp.finish(Comm::proto_success_db()),
            Err(err) => resp.finish(Comm::proto_failed_db_custom(err.to_string())),
        }
    }

    fn modify(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestPageModify>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        match self
            .task
            .page_modify(req.message.name, req.message.name_new)
        {
            Ok(()) => resp.finish(Comm::proto_success_db()),
            Err(err) => resp.finish(Comm::proto_failed_db_custom(err.to_string())),
        }
    }

    fn info(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestPageInfo>,
        resp: ServerResponseUnarySink<ResponsePageInfo>,
    ) -> Result<()> {
        let mut response = ResponsePageInfo::new();
        let mut item = Page::new();
        match self.task.page(req.message.name) {
            Ok(res) => {
                let item_r = res.read().unwrap();
                item.set_name(item_r.name());
                item.set_comment(item_r.comment());
                item.set_size(item_r.size());
                item.set_period(item_r.period());
                item.set_create_time(Comm::proto_time_2_grpc_timestamp(item_r.create_time()));
                response.set_page(item);
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
        req: ServerRequestSingle<RequestPageRemove>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        match self.task.page_remove(req.message.name) {
            Ok(()) => resp.finish(Comm::proto_success_db()),
            Err(err) => resp.finish(Comm::proto_failed_db_custom(err.to_string())),
        }
    }
}
