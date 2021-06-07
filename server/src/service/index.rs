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
use protocols::impls::db::index::{
    Engine, Index, IndexList, KeyType, RequestIndexCreate, RequestIndexInfo, RequestIndexList,
    ResponseIndexInfo,
};
use protocols::impls::db::service_grpc::IndexService;

use crate::utils::Comm;
use protocols::impls::db::service::Response;

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
        match self
            .task
            .index_map(req.message.database_name, req.message.view_name)
        {
            Ok(index_map) => {
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
            }
            _ => {}
        }
        resp.finish(list)
    }

    fn index_create(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestIndexCreate>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        let response = Response::new();
        match self.task.index_create(
            req.message.database_name,
            req.message.view_name,
            req.message.name,
            self.to_engine(req.message.engine),
            self.to_key_type(req.message.key_type),
            req.message.primary,
            req.message.unique,
            req.message.null,
        ) {
            Ok(()) => resp.finish(response),
            Err(err) => Err(Error::GrpcMessage(GrpcMessageError {
                grpc_status: 0,
                grpc_message: err.to_string(),
            })),
        }
    }

    fn index_info(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestIndexInfo>,
        resp: ServerResponseUnarySink<ResponseIndexInfo>,
    ) -> Result<()> {
        let mut info = ResponseIndexInfo::new();
        let mut item = Index::new();
        match self.task.index(
            req.message.database_name,
            req.message.view_name,
            req.message.name,
        ) {
            Ok(res) => {
                item.set_name(res.name());
                item.set_engine(self.engine(res.engine()));
                item.set_key_type(self.key_type(res.key_type()));
                item.set_primary(res.primary());
                item.set_unique(res.unique());
                item.set_null(res.null());
                item.set_create_time(Comm::time_2_grpc_timestamp(res.create_time()));
                info.set_index(item);
                resp.finish(info)
            }
            Err(err) => Err(Error::GrpcMessage(GrpcMessageError {
                grpc_status: 0,
                grpc_message: err.to_string(),
            })),
        }
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

    fn to_engine(&self, e: Engine) -> db::utils::enums::Engine {
        match e {
            Engine::None => db::utils::enums::Engine::None,
            Engine::Disk => db::utils::enums::Engine::Disk,
            Engine::Sequence => db::utils::enums::Engine::Sequence,
            Engine::Block => db::utils::enums::Engine::Block,
            Engine::Increment => db::utils::enums::Engine::Increment,
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

    fn to_key_type(&self, e: KeyType) -> db::utils::enums::KeyType {
        match e {
            KeyType::Nonsupport => db::utils::enums::KeyType::None,
            KeyType::String => db::utils::enums::KeyType::String,
            KeyType::UInt => db::utils::enums::KeyType::UInt,
            KeyType::Int => db::utils::enums::KeyType::Int,
            KeyType::Bool => db::utils::enums::KeyType::Bool,
            KeyType::Float => db::utils::enums::KeyType::Float,
        }
    }
}
