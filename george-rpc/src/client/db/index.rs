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

use crate::client::db::IndexRpcClient;
use crate::client::{status_check, Notls, Openssl, RequestCond, Rustls, TLSType};
use crate::client::{RpcClient, TLS};
use crate::protos::db::db::index_service_client::IndexServiceClient;
use crate::protos::db::db::{
    Engine, Index, KeyType, RequestIndexCreate, RequestIndexInfo, RequestIndexList,
};

impl RpcClient for IndexRpcClient {
    fn new(remote: &str, port: u16, cond_op: Option<RequestCond>) -> GeorgeResult<Self>
    where
        Self: Sized,
    {
        let (inner, rt) = Notls::make(remote, port, cond_op)?;
        Ok(IndexRpcClient {
            client: IndexServiceClient::new(inner),
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
        Ok(IndexRpcClient {
            client: IndexServiceClient::new(endpoint.0),
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
        Ok(IndexRpcClient {
            client: IndexServiceClient::new(endpoint.0),
            rt: endpoint.1,
        })
    }
}

impl IndexRpcClient {
    pub fn list(&mut self, database_name: String, view_name: String) -> GeorgeResult<Vec<Index>> {
        let request = Request::new(RequestIndexList {
            database_name,
            view_name,
        });
        match self.rt.block_on(self.client.list(request)) {
            Ok(res) => {
                let resp = res.into_inner();
                status_check(resp.status, resp.msg_err)?;
                Ok(resp.indexes)
            }
            Err(err) => Err(Errs::strs(
                "failed to successfully run the future on RunTime!",
                err,
            )),
        }
    }

    pub fn create(
        &mut self,
        database_name: String,
        view_name: String,
        name: String,
        unique: bool,
        primary: bool,
        null: bool,
        key_type: KeyType,
        engine: Engine,
    ) -> GeorgeResult<()> {
        let request = Request::new(RequestIndexCreate {
            database_name,
            view_name,
            name,
            engine: engine as i32,
            primary,
            unique,
            null,
            key_type: key_type as i32,
        });
        match self.rt.block_on(self.client.create(request)) {
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

    pub fn info(
        &mut self,
        database_name: String,
        view_name: String,
        name: String,
    ) -> GeorgeResult<Index> {
        let request = Request::new(RequestIndexInfo {
            database_name,
            view_name,
            name,
        });
        match self.rt.block_on(self.client.info(request)) {
            Ok(res) => {
                let resp = res.into_inner();
                status_check(resp.status, resp.msg_err)?;
                match resp.index {
                    Some(res) => Ok(res),
                    None => Err(Errs::index_no_exist_error()),
                }
            }
            Err(err) => Err(Errs::strs(
                "failed to successfully run the future on RunTime!",
                err,
            )),
        }
    }
}
