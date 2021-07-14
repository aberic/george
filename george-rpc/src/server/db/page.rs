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

use crate::protos::db::db::page_service_server::PageService;
use crate::protos::db::db::{
    Page, RequestPageCreate, RequestPageInfo, RequestPageModify, RequestPageRemove,
    ResponsePageInfo, ResponsePageList,
};
use crate::protos::utils::utils::{Req, Resp};
use crate::server::db::PageServer;
use crate::tools::{Results, Trans};

impl PageServer {
    pub fn new(task: Arc<Task>) -> Self {
        PageServer { task }
    }
}

#[tonic::async_trait]
impl PageService for PageServer {
    async fn list(&self, _request: Request<Req>) -> Result<Response<ResponsePageList>, Status> {
        let mut pages: Vec<Page> = vec![];
        let page_map = self.task.page_map();
        let page_map_r = page_map.read().unwrap();
        for page in page_map_r.values() {
            let page_r = page.read().unwrap();
            pages.push(Page {
                name: page_r.name(),
                comment: page_r.comment(),
                size: page_r.size(),
                period: page_r.period(),
                create_time: Some(Trans::time_2_grpc_timestamp(page_r.create_time())),
            });
        }
        Results::response(ResponsePageList {
            status: Results::success_status(),
            msg_err: "".to_string(),
            pages,
        })
    }

    async fn create(&self, request: Request<RequestPageCreate>) -> Result<Response<Resp>, Status> {
        match self.task.page_create(
            request.get_ref().name.clone(),
            request.get_ref().comment.clone(),
            request.get_ref().size,
            request.get_ref().period,
        ) {
            Ok(()) => Results::success(),
            Err(err) => Results::failed_errs(err),
        }
    }

    async fn modify(&self, request: Request<RequestPageModify>) -> Result<Response<Resp>, Status> {
        match self.task.page_modify(
            request.get_ref().name.clone(),
            request.get_ref().name_new.clone(),
        ) {
            Ok(()) => Results::success(),
            Err(err) => Results::failed_errs(err),
        }
    }

    async fn info(
        &self,
        request: Request<RequestPageInfo>,
    ) -> Result<Response<ResponsePageInfo>, Status> {
        let resp;
        match self.task.page(request.get_ref().name.clone()) {
            Ok(res) => {
                let page_r = res.read().unwrap();
                resp = ResponsePageInfo {
                    status: Results::success_status(),
                    msg_err: "".to_string(),
                    page: Some(Page {
                        name: page_r.name(),
                        comment: page_r.comment(),
                        size: page_r.size(),
                        period: page_r.period(),
                        create_time: Some(Trans::time_2_grpc_timestamp(page_r.create_time())),
                    }),
                }
            }
            Err(err) => {
                resp = ResponsePageInfo {
                    status: Results::failed_status(err.clone()),
                    msg_err: err.to_string(),
                    page: None,
                };
            }
        }
        Results::response(resp)
    }

    async fn remove(&self, request: Request<RequestPageRemove>) -> Result<Response<Resp>, Status> {
        match self.task.page_remove(request.get_ref().name.clone()) {
            Ok(()) => Results::success(),
            Err(err) => Results::failed_err(err),
        }
    }
}
