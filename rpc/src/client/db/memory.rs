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

use crate::client::db::{MemoryRpcClient, RpcClient};
use crate::client::{endpoint, endpoint_tls_bytes, endpoint_tls_check_bytes, status_check};
use crate::protos::db::db::memory_service_client::MemoryServiceClient;
use crate::protos::db::db::{
    RequestMemoryInto, RequestMemoryOut, RequestMemoryPInto, RequestMemoryPOut,
    RequestMemoryPRemove, RequestMemoryRemove,
};

impl RpcClient for MemoryRpcClient {
    fn new(remote: &str, port: u16) -> GeorgeResult<Self>
    where
        Self: Sized,
    {
        let (inner, rt) = endpoint(remote, port)?;
        Ok(MemoryRpcClient {
            client: MemoryServiceClient::new(inner),
            rt,
        })
    }

    fn new_tls_bytes(
        remote: &str,
        port: u16,
        ca: Vec<u8>,
        domain_name: impl Into<String>,
    ) -> GeorgeResult<Self>
    where
        Self: Sized,
    {
        let (inner, rt) = endpoint_tls_bytes(remote, port, ca, domain_name)?;
        Ok(MemoryRpcClient {
            client: MemoryServiceClient::new(inner),
            rt,
        })
    }

    fn new_tls_check_bytes(
        remote: &str,
        port: u16,
        key: Vec<u8>,
        cert: Vec<u8>,
        ca: Vec<u8>,
        domain_name: impl Into<String>,
    ) -> GeorgeResult<Self>
    where
        Self: Sized,
    {
        let (inner, rt) = endpoint_tls_check_bytes(remote, port, key, cert, ca, domain_name)?;
        Ok(MemoryRpcClient {
            client: MemoryServiceClient::new(inner),
            rt,
        })
    }
}

impl MemoryRpcClient {
    pub fn put(&mut self, key: String, value: Vec<u8>) -> GeorgeResult<()> {
        let request = Request::new(RequestMemoryInto { key, value });
        match self.rt.block_on(self.client.put(request)) {
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

    pub fn set(&mut self, key: String, value: Vec<u8>) -> GeorgeResult<()> {
        let request = Request::new(RequestMemoryInto { key, value });
        match self.rt.block_on(self.client.set(request)) {
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

    pub fn get(&mut self, key: String) -> GeorgeResult<Vec<u8>> {
        let request = Request::new(RequestMemoryOut { key });
        match self.rt.block_on(self.client.get(request)) {
            Ok(res) => {
                let resp = res.into_inner();
                status_check(resp.status, resp.msg_err)?;
                Ok(resp.value)
            }
            Err(err) => Err(Errs::strs(
                "failed to successfully run the future on RunTime!",
                err,
            )),
        }
    }

    pub fn remove(&mut self, key: String) -> GeorgeResult<()> {
        let request = Request::new(RequestMemoryRemove { key });
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

    pub fn put_by_page(
        &mut self,
        page_name: String,
        key: String,
        value: Vec<u8>,
    ) -> GeorgeResult<()> {
        let request = Request::new(RequestMemoryPInto {
            page_name,
            key,
            value,
        });
        match self.rt.block_on(self.client.put_by_page(request)) {
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

    pub fn set_by_page(
        &mut self,
        page_name: String,
        key: String,
        value: Vec<u8>,
    ) -> GeorgeResult<()> {
        let request = Request::new(RequestMemoryPInto {
            page_name,
            key,
            value,
        });
        match self.rt.block_on(self.client.set_by_page(request)) {
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

    pub fn get_by_page(&mut self, page_name: String, key: String) -> GeorgeResult<Vec<u8>> {
        let request = Request::new(RequestMemoryPOut { page_name, key });
        match self.rt.block_on(self.client.get_by_page(request)) {
            Ok(res) => {
                let resp = res.into_inner();
                status_check(resp.status, resp.msg_err)?;
                Ok(resp.value)
            }
            Err(err) => Err(Errs::strs(
                "failed to successfully run the future on RunTime!",
                err,
            )),
        }
    }

    pub fn remove_by_page(&mut self, page_name: String, key: String) -> GeorgeResult<()> {
        let request = Request::new(RequestMemoryPRemove { page_name, key });
        match self.rt.block_on(self.client.remove_by_page(request)) {
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
