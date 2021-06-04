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
use protocols::impls::db::database::{Database, DatabaseList};
use protocols::impls::db::service::Request;
use protocols::impls::db::service_grpc::DatabaseService;

use crate::utils::Comm;

pub(crate) struct DatabaseServer {
    pub(crate) task: Arc<Task>,
}

impl DatabaseService for DatabaseServer {
    fn databases(
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
            let db_r = db.read().unwrap();
            let mut database = Database::new();
            database.set_name(db_r.name());
            database.set_comment(db_r.comment());
            database.set_create_time(Comm::time_2_grpc_timestamp(db_r.create_time()));
            databases.push(database);
        }
        list.set_databases(databases);
        resp.finish(list)
    }
}
