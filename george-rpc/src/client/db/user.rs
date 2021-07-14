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

use george_comm::errors::{Errs, GeorgeResult};

use crate::client::db::UserRpcClient;
use crate::client::{status_check, Notls, Openssl, RequestCond, Rustls, TLSType};
use crate::client::{RpcClient, TLS};
use crate::protos::db::db::user_service_client::UserServiceClient;
use crate::protos::db::db::RequestLogin;

impl RpcClient for UserRpcClient {
    fn new(remote: &str, port: u16, cond_op: Option<RequestCond>) -> GeorgeResult<Self>
    where
        Self: Sized,
    {
        let (inner, rt) = Notls::make(remote, port, cond_op)?;
        Ok(UserRpcClient {
            client: UserServiceClient::new(inner),
            rt,
        })
    }

    fn new_tls_bytes(
        tls_type: TLSType,
        remote: &str,
        port: u16,
        ca_bytes: Vec<u8>,
        domain_name: impl Into<String>,
        cond_op: Option<RequestCond>,
    ) -> GeorgeResult<Self>
    where
        Self: Sized,
    {
        let endpoint;
        match tls_type {
            TLSType::Rustls => {
                endpoint = Rustls::new_bytes(remote, port, ca_bytes, domain_name, cond_op)?
            }
            TLSType::Openssl => {
                endpoint = Openssl::new_bytes(remote, port, ca_bytes, domain_name, cond_op)?
            }
        }
        Ok(UserRpcClient {
            client: UserServiceClient::new(endpoint.0),
            rt: endpoint.1,
        })
    }

    fn new_tls_bytes_check(
        tls_type: TLSType,
        remote: &str,
        port: u16,
        key_bytes: Vec<u8>,
        cert_bytes: Vec<u8>,
        ca_bytes: Vec<u8>,
        domain_name: impl Into<String>,
        cond_op: Option<RequestCond>,
    ) -> GeorgeResult<Self>
    where
        Self: Sized,
    {
        let endpoint;
        match tls_type {
            TLSType::Rustls => {
                endpoint = Rustls::new_bytes_check(
                    remote,
                    port,
                    key_bytes,
                    cert_bytes,
                    ca_bytes,
                    domain_name,
                    cond_op,
                )?
            }
            TLSType::Openssl => {
                endpoint = Openssl::new_bytes_check(
                    remote,
                    port,
                    key_bytes,
                    cert_bytes,
                    ca_bytes,
                    domain_name,
                    cond_op,
                )?
            }
        }
        Ok(UserRpcClient {
            client: UserServiceClient::new(endpoint.0),
            rt: endpoint.1,
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
