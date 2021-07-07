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

use crate::client::db::{PageRpcClient, RpcClient};
use crate::client::{endpoint, endpoint_tls_bytes, endpoint_tls_check_bytes, status_check};
use crate::protos::db::db::page_service_client::PageServiceClient;
use crate::protos::db::db::{
    Page, RequestPageCreate, RequestPageInfo, RequestPageModify, RequestPageRemove,
};
use crate::protos::utils::utils::Req;

impl RpcClient for PageRpcClient {
    fn new(remote: &str, port: u16) -> GeorgeResult<Self>
    where
        Self: Sized,
    {
        let (inner, rt) = endpoint(remote, port)?;
        Ok(PageRpcClient {
            client: PageServiceClient::new(inner),
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
        Ok(PageRpcClient {
            client: PageServiceClient::new(inner),
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
        Ok(PageRpcClient {
            client: PageServiceClient::new(inner),
            rt,
        })
    }
}

impl PageRpcClient {
    pub fn list(&mut self) -> GeorgeResult<Vec<Page>> {
        let request = Request::new(Req {});
        match self.rt.block_on(self.client.list(request)) {
            Ok(res) => {
                let resp = res.into_inner();
                status_check(resp.status, resp.msg_err)?;
                Ok(resp.pages)
            }
            Err(err) => Err(Errs::strs(
                "failed to successfully run the future on RunTime!",
                err,
            )),
        }
    }

    pub fn create(
        &mut self,
        name: String,
        comment: String,
        size: u64,
        period: u32,
    ) -> GeorgeResult<()> {
        let request = Request::new(RequestPageCreate {
            name,
            comment,
            size,
            period,
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

    pub fn info(&mut self, name: String) -> GeorgeResult<Page> {
        let request = Request::new(RequestPageInfo { name });
        match self.rt.block_on(self.client.info(request)) {
            Ok(res) => {
                let resp = res.into_inner();
                status_check(resp.status, resp.msg_err)?;
                match resp.page {
                    Some(res) => Ok(res),
                    None => Err(Errs::page_no_exist_error()),
                }
            }
            Err(err) => Err(Errs::strs(
                "failed to successfully run the future on RunTime!",
                err,
            )),
        }
    }

    pub fn modify(&mut self, name: String, name_new: String) -> GeorgeResult<()> {
        let request = Request::new(RequestPageModify { name, name_new });
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
        let request = Request::new(RequestPageRemove { name });
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
