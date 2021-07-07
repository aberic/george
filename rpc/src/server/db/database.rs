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

use crate::protos::db::db::database_service_server::DatabaseService;
use crate::protos::db::db::{
    Database, RequestDatabaseCreate, RequestDatabaseInfo, RequestDatabaseModify,
    RequestDatabaseRemove, ResponseDatabaseInfo, ResponseDatabaseList,
};
use crate::protos::utils::utils::{Req, Resp};
use crate::server::db::DatabaseServer;
use crate::tools::{Children, Results, Trans};

impl DatabaseServer {
    pub fn new(task: Arc<Task>) -> Self {
        DatabaseServer { task }
    }
}

#[tonic::async_trait]
impl DatabaseService for DatabaseServer {
    async fn list(&self, _request: Request<Req>) -> Result<Response<ResponseDatabaseList>, Status> {
        let mut databases: Vec<Database> = vec![];
        let db_map = self.task.database_map();
        let db_map_r = db_map.read().unwrap();
        for db in db_map_r.values() {
            let views = Children::views(db.clone());
            let db_r = db.read().unwrap();
            let database = Database {
                name: db_r.name(),
                comment: db_r.comment(),
                create_time: Some(Trans::time_2_grpc_timestamp(db_r.create_time())),
                views,
            };
            databases.push(database);
        }
        Results::response(ResponseDatabaseList {
            status: Results::success_status(),
            msg_err: "".to_string(),
            databases,
        })
    }

    async fn create(
        &self,
        request: Request<RequestDatabaseCreate>,
    ) -> Result<Response<Resp>, Status> {
        match self.task.database_create(
            request.get_ref().name.clone(),
            request.get_ref().comment.clone(),
        ) {
            Ok(()) => Results::success(),
            Err(err) => Results::failed_errs(err),
        }
    }

    async fn modify(
        &self,
        request: Request<RequestDatabaseModify>,
    ) -> Result<Response<Resp>, Status> {
        match self.task.database_modify(
            request.get_ref().name.clone(),
            request.get_ref().name_new.clone(),
            request.get_ref().comment.clone(),
        ) {
            Ok(()) => Results::success(),
            Err(err) => Results::failed_errs(err),
        }
    }

    async fn info(
        &self,
        request: Request<RequestDatabaseInfo>,
    ) -> Result<Response<ResponseDatabaseInfo>, Status> {
        let resp;
        match self.task.database(request.get_ref().name.clone()) {
            Ok(res) => {
                let views = Children::views(res.clone());
                let item_r = res.read().unwrap();
                resp = ResponseDatabaseInfo {
                    status: Results::success_status(),
                    msg_err: "".to_string(),
                    database: Some(Database {
                        name: item_r.name(),
                        comment: item_r.comment(),
                        create_time: Some(Trans::time_2_grpc_timestamp(item_r.create_time())),
                        views,
                    }),
                }
            }
            Err(err) => {
                resp = ResponseDatabaseInfo {
                    status: Results::failed_status(err.clone()),
                    msg_err: err.to_string(),
                    database: None,
                };
            }
        }
        Results::response(resp)
    }

    async fn remove(
        &self,
        request: Request<RequestDatabaseRemove>,
    ) -> Result<Response<Resp>, Status> {
        match self.task.database_remove(request.get_ref().name.clone()) {
            Ok(()) => Results::success(),
            Err(err) => Results::failed_err(err),
        }
    }
}
