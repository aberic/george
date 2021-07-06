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

use crate::client::db::DiskRpcClient;
use crate::client::{endpoint, status_check};
use crate::protos::db::db::disk_service_client::DiskServiceClient;
use crate::protos::db::db::{
    DiskDeleted, DiskSelected, RequestDiskDelete, RequestDiskIOut, RequestDiskInto, RequestDiskOut,
    RequestDiskRemove, RequestDiskSelect,
};

impl DiskRpcClient {
    pub fn new(
        remote: &str,
        port: u16,
        tls: bool,
        key: Option<Vec<u8>>,
        cert: Option<Vec<u8>>,
        server_ca: Option<Vec<u8>>,
    ) -> GeorgeResult<DiskRpcClient> {
        let (inner, rt) = endpoint(remote, port, tls, key, cert, server_ca)?;
        Ok(DiskRpcClient {
            client: DiskServiceClient::new(inner),
            rt,
        })
    }
}

impl DiskRpcClient {
    pub fn put(
        &mut self,
        database_name: String,
        view_name: String,
        key: String,
        value: Vec<u8>,
    ) -> GeorgeResult<()> {
        let request = Request::new(RequestDiskInto {
            database_name,
            view_name,
            key,
            value,
        });
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

    pub fn set(
        &mut self,
        database_name: String,
        view_name: String,
        key: String,
        value: Vec<u8>,
    ) -> GeorgeResult<()> {
        let request = Request::new(RequestDiskInto {
            database_name,
            view_name,
            key,
            value,
        });
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

    pub fn get(
        &mut self,
        database_name: String,
        view_name: String,
        key: String,
    ) -> GeorgeResult<Vec<u8>> {
        let request = Request::new(RequestDiskOut {
            database_name,
            view_name,
            key,
        });
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

    pub fn get_by_index(
        &mut self,
        database_name: String,
        view_name: String,
        index_name: String,
        key: String,
    ) -> GeorgeResult<Vec<u8>> {
        let request = Request::new(RequestDiskIOut {
            database_name,
            view_name,
            index_name,
            key,
        });
        match self.rt.block_on(self.client.get_by_index(request)) {
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

    pub fn remove(
        &mut self,
        database_name: String,
        view_name: String,
        key: String,
    ) -> GeorgeResult<()> {
        let request = Request::new(RequestDiskRemove {
            database_name,
            view_name,
            key,
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

    pub fn select(
        &mut self,
        database_name: String,
        view_name: String,
        constraint_json_bytes: Vec<u8>,
    ) -> GeorgeResult<DiskSelected> {
        let request = Request::new(RequestDiskSelect {
            database_name,
            view_name,
            constraint_json_bytes,
        });
        match self.rt.block_on(self.client.select(request)) {
            Ok(res) => {
                let resp = res.into_inner();
                status_check(resp.status, resp.msg_err)?;
                match resp.selected {
                    Some(res) => Ok(res),
                    None => Err(Errs::data_no_exist_error()),
                }
            }
            Err(err) => Err(Errs::strs(
                "failed to successfully run the future on RunTime!",
                err,
            )),
        }
    }

    pub fn delete(
        &mut self,
        database_name: String,
        view_name: String,
        constraint_json_bytes: Vec<u8>,
    ) -> GeorgeResult<DiskDeleted> {
        let request = Request::new(RequestDiskDelete {
            database_name,
            view_name,
            constraint_json_bytes,
        });
        match self.rt.block_on(self.client.delete(request)) {
            Ok(res) => {
                let resp = res.into_inner();
                status_check(resp.status, resp.msg_err)?;
                match resp.deleted {
                    Some(res) => Ok(res),
                    None => Err(Errs::data_no_exist_error()),
                }
            }
            Err(err) => Err(Errs::strs(
                "failed to successfully run the future on RunTime!",
                err,
            )),
        }
    }
}
