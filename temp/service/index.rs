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
use protocols::impls::comm::response::{Response, Status};
use protocols::impls::db::index::{
    Index, IndexList, RequestIndexCreate, RequestIndexInfo, RequestIndexList, ResponseIndexInfo,
};
use protocols::impls::db::service_grpc::IndexService;
use protocols::impls::utils::Comm;

use crate::service::Enums;

pub(crate) struct IndexServer {
    pub(crate) task: Arc<Task>,
}

impl IndexService for IndexServer {
    fn list(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestIndexList>,
        resp: ServerResponseUnarySink<IndexList>,
    ) -> Result<()> {
        let mut list = IndexList::new();
        let mut indexes: RepeatedField<Index> = RepeatedField::new();
        match self
            .task
            .index_map(req.message.database_name, req.message.view_name)
        {
            Ok(index_map) => {
                let index_map_r = index_map.read().unwrap();
                for index in index_map_r.values() {
                    let mut index_item = Index::new();
                    index_item.set_name(index.name());
                    index_item.set_engine(Enums::db_2_engine(index.engine()));
                    index_item.set_primary(index.primary());
                    index_item.set_unique(index.unique());
                    index_item.set_null(index.null());
                    index_item.set_key_type(Enums::db_2_key_type(index.key_type()));
                    index_item
                        .set_create_time(Comm::proto_time_2_grpc_timestamp(index.create_time()));
                    indexes.push(index_item);
                }
                list.set_indexes(indexes);
            }
            _ => {}
        }
        resp.finish(list)
    }

    fn create(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestIndexCreate>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        match self.task.index_create(
            req.message.database_name,
            req.message.view_name,
            req.message.name,
            Enums::engine_2_db(req.message.engine),
            Enums::key_type_2_db(req.message.key_type),
            req.message.primary,
            req.message.unique,
            req.message.null,
        ) {
            Ok(()) => resp.finish(Comm::proto_success_db()),
            Err(err) => resp.finish(Comm::proto_failed_db_custom(err.to_string())),
        }
    }

    fn info(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestIndexInfo>,
        resp: ServerResponseUnarySink<ResponseIndexInfo>,
    ) -> Result<()> {
        let mut response = ResponseIndexInfo::new();
        let mut item = Index::new();
        match self.task.index(
            req.message.database_name,
            req.message.view_name,
            req.message.name,
        ) {
            Ok(res) => {
                item.set_name(res.name());
                item.set_engine(Enums::db_2_engine(res.engine()));
                item.set_key_type(Enums::db_2_key_type(res.key_type()));
                item.set_primary(res.primary());
                item.set_unique(res.unique());
                item.set_null(res.null());
                item.set_create_time(Comm::proto_time_2_grpc_timestamp(res.create_time()));
                response.set_index(item);
                response.set_status(Status::Ok);
            }
            Err(err) => {
                response.set_status(Status::Custom);
                response.set_msg_err(err.to_string());
            }
        }
        resp.finish(response)
    }
}
