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

use crate::client::db::DatabaseRpcClient;
use crate::client::{status_check, Notls, Openssl, RequestCond, Rustls, TLSType};
use crate::client::{RpcClient, TLS};
use crate::protos::db::db::database_service_client::DatabaseServiceClient;
use crate::protos::db::db::{
    Database, RequestDatabaseCreate, RequestDatabaseInfo, RequestDatabaseModify,
    RequestDatabaseRemove,
};
use crate::protos::utils::utils::Req;

impl RpcClient for DatabaseRpcClient {
    fn new(remote: &str, port: u16, cond_op: Option<RequestCond>) -> GeorgeResult<Self>
    where
        Self: Sized,
    {
        let (inner, rt) = Notls::make(remote, port, cond_op)?;
        Ok(DatabaseRpcClient {
            client: DatabaseServiceClient::new(inner),
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
        Ok(DatabaseRpcClient {
            client: DatabaseServiceClient::new(endpoint.0),
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
        Ok(DatabaseRpcClient {
            client: DatabaseServiceClient::new(endpoint.0),
            rt: endpoint.1,
        })
    }

    // fn run<F: Future<Output = Result<Response<T>, tonic::Status>>, T>(
    //     &self,
    //     future: F,
    // ) -> GeorgeResult<T> {
    //     match self.rt.block_on(future) {
    //         Ok(res) => Ok(res.into_inner()),
    //         Err(err) => Err(Errs::strs(
    //             "failed to successfully run the future on RunTime!",
    //             err,
    //         )),
    //     }
    // }
}

impl DatabaseRpcClient {
    pub fn list(&mut self) -> GeorgeResult<Vec<Database>> {
        let request = Request::new(Req {});
        match self.rt.block_on(self.client.list(request)) {
            Ok(res) => {
                let resp = res.into_inner();
                status_check(resp.status, resp.msg_err)?;
                Ok(resp.databases)
            }
            Err(err) => Err(Errs::strs(
                "failed to successfully run the future on RunTime!",
                err,
            )),
        }
    }

    pub fn create(&mut self, name: String, comment: String) -> GeorgeResult<()> {
        let request = Request::new(RequestDatabaseCreate { name, comment });
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

    pub fn info(&mut self, name: String) -> GeorgeResult<Database> {
        let request = Request::new(RequestDatabaseInfo { name });
        match self.rt.block_on(self.client.info(request)) {
            Ok(res) => {
                let resp = res.into_inner();
                status_check(resp.status, resp.msg_err)?;
                match resp.database {
                    Some(res) => Ok(res),
                    None => Err(Errs::database_no_exist_error()),
                }
            }
            Err(err) => Err(Errs::strs(
                "failed to successfully run the future on RunTime!",
                err,
            )),
        }
    }

    pub fn modify(
        &mut self,
        name: String,
        comment_new: String,
        name_new: String,
    ) -> GeorgeResult<()> {
        let request = Request::new(RequestDatabaseModify {
            name,
            name_new,
            comment: comment_new,
        });
        match self.rt.block_on(self.client.modify(request)) {
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

    pub fn remove(&mut self, name: String) -> GeorgeResult<()> {
        let request = Request::new(RequestDatabaseRemove { name });
        match self.rt.block_on(self.client.remove(request)) {
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
