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
use protocols::impls::comm::request::Request;
use protocols::impls::comm::response::Status;
use protocols::impls::db::database::{
    DatabaseList, RequestDatabaseCreate, RequestDatabaseInfo, RequestDatabaseModify,
    RequestDatabaseRemove,
};
use protocols::impls::db::service_grpc::DatabaseServiceClient;

use crate::service::{Database, Tools};

impl Database {
    pub(crate) fn new(remote: &str, port: u16) -> Database {
        Database {
            client: DatabaseServiceClient::new_plain(remote, port, Default::default()).unwrap(),
        }
    }
}

impl Database {
    pub(crate) fn list(&self) -> GeorgeResult<DatabaseList> {
        let req = Request::new();
        let resp = self
            .client
            .list(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => Ok(resp),
            Err(err) => Err(Errs::strs("database list failed!", err)),
        }
    }

    pub(crate) fn create(&self, name: String, comment: String) -> GeorgeResult<()> {
        let mut req = RequestDatabaseCreate::new();
        req.set_name(name);
        req.set_comment(comment);
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
            Err(err) => Err(Errs::strs("database create failed!", err)),
        }
    }

    pub(crate) fn info(
        &self,
        name: String,
    ) -> GeorgeResult<protocols::impls::db::database::Database> {
        let mut req = RequestDatabaseInfo::new();
        req.set_name(name);
        let resp = self
            .client
            .info(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => Ok(resp.database.unwrap()),
            Err(err) => Err(Errs::strs("database info failed!", err)),
        }
    }

    pub(crate) fn modify(
        &self,
        name: String,
        comment_new: String,
        name_new: String,
    ) -> GeorgeResult<()> {
        let mut req = RequestDatabaseModify::new();
        req.set_name(name);
        req.set_comment(comment_new);
        req.set_name_new(name_new);
        let resp = self
            .client
            .modify(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => match resp.status {
                Status::Ok => Ok(()),
                _ => Err(Tools::response_err(resp)),
            },
            Err(err) => Err(Errs::strs("database modify failed!", err)),
        }
    }

    pub(crate) fn remove(&self, name: String) -> GeorgeResult<()> {
        let mut req = RequestDatabaseRemove::new();
        req.set_name(name);
        let resp = self
            .client
            .remove(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => match resp.status {
                Status::Ok => Ok(()),
                _ => Err(Tools::response_err(resp)),
            },
            Err(err) => Err(Errs::strs("database remove failed!", err)),
        }
    }
}
