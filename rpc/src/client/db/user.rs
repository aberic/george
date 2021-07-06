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

use tonic::Request;

use comm::errors::{Errs, GeorgeResult};

use crate::client::db::UserRpcClient;
use crate::client::{endpoint, status_check};
use crate::protos::db::db::user_service_client::UserServiceClient;
use crate::protos::db::db::RequestLogin;

impl UserRpcClient {
    pub fn new(
        remote: &str,
        port: u16,
        tls: bool,
        key: Option<Vec<u8>>,
        cert: Option<Vec<u8>>,
        server_ca: Option<Vec<u8>>,
    ) -> GeorgeResult<UserRpcClient> {
        let (inner, rt) = endpoint(remote, port, tls, key, cert, server_ca)?;
        Ok(UserRpcClient {
            client: UserServiceClient::new(inner),
            rt,
        })
    }
}

impl UserRpcClient {
    pub fn login(&mut self, name: String, pass: String) -> GeorgeResult<()> {
        let request = Request::new(RequestLogin { name, pass });
        match self.rt.block_on(self.client.login(request)) {
            Ok(res) => {
                let resp = res.into_inner();
                status_check(resp.status, resp.msg_err)
            }
            Err(err) => Err(Errs::strs(
                "failed to successfully run the future on RunTime!",
                err,
            )),
        }
    }
}
