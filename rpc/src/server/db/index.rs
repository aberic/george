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

use db::task::traits::TMaster;
use db::Task;

use crate::protos::db::db::index_service_server::IndexService;
use crate::protos::db::db::{
    Index, RequestIndexCreate, RequestIndexInfo, RequestIndexList, ResponseIndexInfo,
    ResponseIndexList,
};
use crate::protos::utils::utils::Resp;
use crate::server::db::IndexServer;
use crate::tools::{Results, Trans};

impl IndexServer {
    pub fn new(task: Arc<Task>) -> Self {
        IndexServer { task }
    }
}

#[tonic::async_trait]
impl IndexService for IndexServer {
    async fn list(
        &self,
        request: Request<RequestIndexList>,
    ) -> Result<Response<ResponseIndexList>, Status> {
        let resp;
        match self.task.index_map(
            request.get_ref().database_name.clone(),
            request.get_ref().view_name.clone(),
        ) {
            Ok(index_map) => {
                let index_map_r = index_map.read().unwrap();
                let mut indexes: Vec<Index> = vec![];
                for index in index_map_r.values() {
                    indexes.push(Index {
                        name: index.name(),
                        engine: Trans::db_2_engine_i32(index.engine()),
                        primary: index.primary(),
                        unique: index.unique(),
                        null: index.null(),
                        key_type: Trans::db_2_key_type_i32(index.key_type()),
                        create_time: Some(Trans::time_2_grpc_timestamp(index.create_time())),
                    });
                }
                resp = ResponseIndexList {
                    status: Results::success_status(),
                    msg_err: "".to_string(),
                    indexes,
                }
            }
            Err(err) => {
                resp = ResponseIndexList {
                    status: Results::failed_status(err.clone()),
                    msg_err: err.to_string(),
                    indexes: vec![],
                };
            }
        }
        Results::response(resp)
    }

    async fn create(&self, request: Request<RequestIndexCreate>) -> Result<Response<Resp>, Status> {
        let engine;
        match Trans::i32_2_db_engine(request.get_ref().engine) {
            Ok(res) => engine = res,
            Err(err) => return Results::failed_err(err),
        }
        let key_type;
        match Trans::i32_2_db_key_type(request.get_ref().key_type) {
            Ok(res) => key_type = res,
            Err(err) => return Results::failed_err(err),
        }
        match self.task.index_create(
            request.get_ref().database_name.clone(),
            request.get_ref().view_name.clone(),
            request.get_ref().name.clone(),
            engine,
            key_type,
            request.get_ref().primary,
            request.get_ref().unique,
            request.get_ref().null,
        ) {
            Ok(()) => Results::success(),
            Err(err) => Results::failed_err(err),
        }
    }

    async fn info(
        &self,
        request: Request<RequestIndexInfo>,
    ) -> Result<Response<ResponseIndexInfo>, Status> {
        let resp;
        match self.task.index(
            request.get_ref().database_name.clone(),
            request.get_ref().view_name.clone(),
            request.get_ref().name.clone(),
        ) {
            Ok(res) => {
                resp = ResponseIndexInfo {
                    status: Results::success_status(),
                    msg_err: "".to_string(),
                    index: Some(Index {
                        name: res.name(),
                        engine: Trans::db_2_engine_i32(res.engine()),
                        primary: res.primary(),
                        unique: res.unique(),
                        null: res.null(),
                        key_type: Trans::db_2_key_type_i32(res.key_type()),
                        create_time: Some(Trans::time_2_grpc_timestamp(res.create_time())),
                    }),
                }
            }
            Err(err) => {
                resp = ResponseIndexInfo {
                    status: Results::failed_status(err.clone()),
                    msg_err: err.to_string(),
                    index: None,
                };
            }
        }
        Results::response(resp)
    }
}
