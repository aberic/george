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
use protobuf::{RepeatedField, SingularPtrField};

use crate::service::index::IndexServer;
use crate::service::{Children, Enums};
use db::task::traits::{TForm, TMaster};
use db::Task;
use protobuf::well_known_types::Timestamp;
use protocols::impls::db::database::{
    Database, DatabaseList, RequestDatabaseCreate, RequestDatabaseInfo, RequestDatabaseModify,
    RequestDatabaseRemove, ResponseDatabaseInfo,
};
use protocols::impls::db::index::Index;
use protocols::impls::db::response::{Response, Status};
use protocols::impls::db::service::Request;
use protocols::impls::db::service_grpc::DatabaseService;
use protocols::impls::db::view::View;
use protocols::impls::utils::Comm;

pub(crate) struct DatabaseServer {
    pub(crate) task: Arc<Task>,
}

impl DatabaseService for DatabaseServer {
    fn list(
        &self,
        _o: ServerHandlerContext,
        _req: ServerRequestSingle<Request>,
        resp: ServerResponseUnarySink<DatabaseList>,
    ) -> Result<()> {
        let mut list = DatabaseList::new();
        let mut databases: RepeatedField<Database> = RepeatedField::new();
        let db_map = self.task.database_map();
        let db_map_r = db_map.read().unwrap();
        for db in db_map_r.values() {
            let views = Children::views(db.clone());
            let db_r = db.read().unwrap();
            let mut database = Database::new();
            database.set_name(db_r.name());
            database.set_comment(db_r.comment());
            database.set_create_time(Comm::proto_time_2_grpc_timestamp(db_r.create_time()));
            database.set_views(views);
            databases.push(database);
        }
        list.set_databases(databases);
        resp.finish(list)
    }

    fn create(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestDatabaseCreate>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        match self.task.database_create(
            req.message.get_name().to_string(),
            req.message.get_comment().to_string(),
        ) {
            Ok(()) => resp.finish(Comm::proto_success_db()),
            Err(err) => resp.finish(Comm::proto_failed_db_custom(err.to_string())),
        }
    }

    fn modify(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestDatabaseModify>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        match self
            .task
            .database_modify(req.message.name, req.message.name_new, req.message.comment)
        {
            Ok(()) => resp.finish(Comm::proto_success_db()),
            Err(err) => resp.finish(Comm::proto_failed_db_custom(err.to_string())),
        }
    }

    fn info(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestDatabaseInfo>,
        resp: ServerResponseUnarySink<ResponseDatabaseInfo>,
    ) -> Result<()> {
        let mut response = ResponseDatabaseInfo::new();
        let mut item = Database::new();
        match self.task.database(req.message.name) {
            Ok(res) => {
                let views = Children::views(res.clone());
                let item_r = res.read().unwrap();
                item.set_name(item_r.name());
                item.set_comment(item_r.comment());
                item.set_create_time(Comm::proto_time_2_grpc_timestamp(item_r.create_time()));
                item.set_views(views);
                response.set_status(Status::Ok);
                response.set_database(item);
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
        req: ServerRequestSingle<RequestDatabaseRemove>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        match self.task.database_remove(req.message.name) {
            Ok(()) => resp.finish(Comm::proto_success_db()),
            Err(err) => resp.finish(Comm::proto_failed_db_custom(err.to_string())),
        }
    }
}
