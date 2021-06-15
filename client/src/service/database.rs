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

use protocols::impls::db::service_grpc::DatabaseServiceClient;
use protocols::impls::db::user::RequestLogin;

use crate::service::Database;
use comm::errors::{Errs, GeorgeResult};
use protocols::impls::db::response::Status;
use protocols::impls::db::service::Request;

impl Database {
    pub(crate) fn new(remote: &str, port: u16) -> Database {
        Database {
            client: DatabaseServiceClient::new_plain(remote, port, Default::default()).unwrap(),
        }
    }
}

impl Database {
    pub(crate) fn databases(&self, name: String, pass: String) -> GeorgeResult<()> {
        let mut req = Request::new();
        let resp = self
            .client
            .databases(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => match resp.status {
                Status::Ok => Ok(()),
                _ => Err(Errs::string(format!(
                    "login failed! status: {:#?}, msg: {}",
                    resp.status, resp.msg_err
                ))),
            },
            Err(err) => Err(Errs::strs("login failed!", err)),
        }
    }
}
