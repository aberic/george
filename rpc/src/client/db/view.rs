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

use crate::client::db::{RpcClient, ViewRpcClient};
use crate::client::{endpoint, endpoint_tls_bytes, endpoint_tls_check_bytes, status_check};
use crate::protos::db::db::view_service_client::ViewServiceClient;
use crate::protos::db::db::{
    RequestViewArchive, RequestViewCreate, RequestViewInfo, RequestViewList, RequestViewModify,
    RequestViewRecord, RequestViewRecords, RequestViewRemove, View, ViewRecord,
};

impl RpcClient for ViewRpcClient {
    fn new(remote: &str, port: u16) -> GeorgeResult<Self>
    where
        Self: Sized,
    {
        let (inner, rt) = endpoint(remote, port)?;
        Ok(ViewRpcClient {
            client: ViewServiceClient::new(inner),
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
        Ok(ViewRpcClient {
            client: ViewServiceClient::new(inner),
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
        Ok(ViewRpcClient {
            client: ViewServiceClient::new(inner),
            rt,
        })
    }
}

impl ViewRpcClient {
    pub fn list(&mut self, database_name: String) -> GeorgeResult<Vec<View>> {
        let request = Request::new(RequestViewList { database_name });
        match self.rt.block_on(self.client.list(request)) {
            Ok(res) => {
                let resp = res.into_inner();
                status_check(resp.status, resp.msg_err)?;
                Ok(resp.views)
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
        name: String,
        comment: String,
        increment: bool,
    ) -> GeorgeResult<()> {
        let request = Request::new(RequestViewCreate {
            database_name,
            name,
            comment,
            with_increment: increment,
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

    pub fn info(&mut self, database_name: String, name: String) -> GeorgeResult<View> {
        let request = Request::new(RequestViewInfo {
            database_name,
            name,
        });
        match self.rt.block_on(self.client.info(request)) {
            Ok(res) => {
                let resp = res.into_inner();
                status_check(resp.status, resp.msg_err)?;
                match resp.view {
                    Some(res) => Ok(res),
                    None => Err(Errs::view_no_exist_error()),
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
        database_name: String,
        name: String,
        name_new: String,
        comment: String,
    ) -> GeorgeResult<()> {
        let request = Request::new(RequestViewModify {
            database_name,
            name,
            name_new,
            comment,
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

    pub fn remove(&mut self, database_name: String, name: String) -> GeorgeResult<()> {
        let request = Request::new(RequestViewRemove {
            database_name,
            name,
        });
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

    pub fn archive(
        &mut self,
        database_name: String,
        name: String,
        archive_file_path: String,
    ) -> GeorgeResult<()> {
        let request = Request::new(RequestViewArchive {
            database_name,
            name,
            archive_file_path,
        });
        match self.rt.block_on(self.client.archive(request)) {
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

    pub fn record(
        &mut self,
        database_name: String,
        name: String,
        version: u32,
    ) -> GeorgeResult<ViewRecord> {
        let request = Request::new(RequestViewRecord {
            database_name,
            name,
            version,
        });
        match self.rt.block_on(self.client.record(request)) {
            Ok(res) => {
                let resp = res.into_inner();
                status_check(resp.status, resp.msg_err)?;
                match resp.record {
                    Some(res) => Ok(res),
                    None => Err(Errs::view_no_exist_error()),
                }
            }
            Err(err) => Err(Errs::strs(
                "failed to successfully run the future on RunTime!",
                err,
            )),
        }
    }

    pub fn records(
        &mut self,
        database_name: String,
        name: String,
    ) -> GeorgeResult<Vec<ViewRecord>> {
        let request = Request::new(RequestViewRecords {
            database_name,
            name,
        });
        match self.rt.block_on(self.client.records(request)) {
            Ok(res) => {
                let resp = res.into_inner();
                status_check(resp.status, resp.msg_err)?;
                Ok(resp.records)
            }
            Err(err) => Err(Errs::strs(
                "failed to successfully run the future on RunTime!",
                err,
            )),
        }
    }
}
