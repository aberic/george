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
use protocols::impls::db::index::{Engine, Index, IndexList, KeyType, RequestIndexList};
use protocols::impls::db::service_grpc::IndexService;

use crate::utils::Comm;

pub(crate) struct IndexServer {
    pub(crate) task: Arc<Task>,
}

impl IndexService for IndexServer {
    fn indexes(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestIndexList>,
        resp: ServerResponseUnarySink<IndexList>,
    ) -> Result<()> {
        let mut list = IndexList::new();
        let mut indexes: RepeatedField<Index> = RepeatedField::new();
        let db_map = self.task.database_map();
        let db_map_r = db_map.read().unwrap();
        let view_map;
        match db_map_r.get(req.message.get_database_name()) {
            Some(db) => {
                view_map = db.read().unwrap().view_map();
            }
            None => return resp.finish(list),
        }
        let view_map_r = view_map.read().unwrap();
        let index_map;
        match view_map_r.get(req.message.get_view_name()) {
            Some(view) => {
                index_map = view.read().unwrap().index_map();
            }
            None => return resp.finish(list),
        }
        let index_map_r = index_map.read().unwrap();
        for index in index_map_r.values() {
            let mut index_item = Index::new();
            index_item.set_name(index.name());
            index_item.set_engine(self.engine(index.engine()));
            index_item.set_primary(index.primary());
            index_item.set_unique(index.unique());
            index_item.set_null(index.null());
            index_item.set_key_type(self.key_type(index.key_type()));
            index_item.set_create_time(Comm::time_2_grpc_timestamp(index.create_time()));
            indexes.push(index_item);
        }
        list.set_indexes(indexes);
        resp.finish(list)
    }
}

impl IndexServer {
    fn engine(&self, e: db::utils::enums::Engine) -> Engine {
        match e {
            db::utils::enums::Engine::None => Engine::None,
            db::utils::enums::Engine::Disk => Engine::Disk,
            db::utils::enums::Engine::Sequence => Engine::Sequence,
            db::utils::enums::Engine::Block => Engine::Block,
            db::utils::enums::Engine::Increment => Engine::Increment,
        }
    }

    fn key_type(&self, e: db::utils::enums::KeyType) -> KeyType {
        match e {
            db::utils::enums::KeyType::None => KeyType::Nonsupport,
            db::utils::enums::KeyType::String => KeyType::String,
            db::utils::enums::KeyType::UInt => KeyType::UInt,
            db::utils::enums::KeyType::Int => KeyType::Int,
            db::utils::enums::KeyType::Bool => KeyType::Bool,
            db::utils::enums::KeyType::Float => KeyType::Float,
        }
    }
}
