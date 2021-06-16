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

use futures::executor;
use grpc::ClientStubExt;

use comm::errors::{Errs, GeorgeResult};
use protocols::impls::db::index::{
    Engine, IndexList, KeyType, RequestIndexCreate, RequestIndexInfo, RequestIndexList,
};
use protocols::impls::db::response::Status;
use protocols::impls::db::service_grpc::IndexServiceClient;

use crate::service::{Index, Tools};

impl Index {
    pub(crate) fn new(remote: &str, port: u16) -> Index {
        Index {
            client: IndexServiceClient::new_plain(remote, port, Default::default()).unwrap(),
        }
    }
}

impl Index {
    pub(crate) fn list(&self, database_name: String, view_name: String) -> GeorgeResult<IndexList> {
        let mut req = RequestIndexList::new();
        req.set_database_name(database_name);
        req.set_view_name(view_name);
        let resp = self
            .client
            .list(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => Ok(resp),
            Err(err) => Err(Errs::strs("index list failed!", err)),
        }
    }

    pub(crate) fn create(
        &self,
        database_name: String,
        view_name: String,
        name: String,
        unique: bool,
        primary: bool,
        null: bool,
        key_type: KeyType,
        engine: Engine,
    ) -> GeorgeResult<()> {
        let mut req = RequestIndexCreate::new();
        req.set_database_name(database_name);
        req.set_view_name(view_name);
        req.set_name(name);
        req.set_unique(unique);
        req.set_primary(primary);
        req.set_null(null);
        req.set_key_type(key_type);
        req.set_engine(engine);
        let resp = self
            .client
            .create(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => match resp.status {
                Status::Ok => Ok(()),
                _ => Err(Tools::response_err(resp)),
            },
            Err(err) => Err(Errs::strs("index create failed!", err)),
        }
    }

    pub(crate) fn info(
        &self,
        database_name: String,
        view_name: String,
        name: String,
    ) -> GeorgeResult<protocols::impls::db::index::Index> {
        let mut req = RequestIndexInfo::new();
        req.set_database_name(database_name);
        req.set_view_name(view_name);
        req.set_name(name);
        let resp = self
            .client
            .info(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => match resp.status {
                Status::Ok => Ok(resp.index.unwrap()),
                _ => Err(Tools::response_cus(resp.status, resp.msg_err)),
            },
            Err(err) => Err(Errs::strs("index info failed!", err)),
        }
    }
}
