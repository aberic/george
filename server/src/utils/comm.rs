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

use crate::utils::Comm;
use comm::errors::GeorgeResult;
use comm::Time;
use db::task::traits::TMaster;
use db::task::{Database, View};
use db::Task;
use grpc::{Error, GrpcMessageError, Result, ServerResponseUnarySink};
use protobuf::well_known_types::Timestamp;
use protocols::impls::db::service::Response;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

impl Comm {
    pub fn time_2_grpc_timestamp(time: Time) -> Timestamp {
        let mut timestamp = Timestamp::new();
        timestamp.set_seconds(time.sec());
        timestamp.set_nanos(time.nanos());
        timestamp
    }

    pub fn view_map(
        task: Arc<Task>,
        database_name: &str,
    ) -> Option<Arc<RwLock<HashMap<String, Arc<RwLock<View>>>>>> {
        let db_map = task.database_map();
        let db_map_r = db_map.read().unwrap();
        match db_map_r.get(database_name) {
            Some(db) => Some(db.read().unwrap().view_map()),
            None => None,
        }
    }
}
