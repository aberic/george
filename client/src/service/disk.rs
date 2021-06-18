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
use protocols::impls::comm::response::Status;
use protocols::impls::db::disk::{
    DiskDeleted, DiskSelected, RequestDiskDelete, RequestDiskIOut, RequestDiskInto, RequestDiskOut,
    RequestDiskRemove, RequestDiskSelect,
};
use protocols::impls::db::service_grpc::DiskServiceClient;

use crate::service::{Disk, Tools};

impl Disk {
    pub(crate) fn new(remote: &str, port: u16) -> Disk {
        Disk {
            client: DiskServiceClient::new_plain(remote, port, Default::default()).unwrap(),
        }
    }
}

impl Disk {
    pub(crate) fn put(
        &self,
        database_name: String,
        view_name: String,
        key: String,
        value: Vec<u8>,
    ) -> GeorgeResult<()> {
        let mut req = RequestDiskInto::new();
        req.set_database_name(database_name);
        req.set_view_name(view_name);
        req.set_key(key);
        req.set_value(value);
        let resp = self
            .client
            .put(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => match resp.status {
                Status::Ok => Ok(()),
                _ => Err(Tools::response_err(resp)),
            },
            Err(err) => Err(Errs::strs("disk put failed!", err)),
        }
    }

    pub(crate) fn set(
        &self,
        database_name: String,
        view_name: String,
        key: String,
        value: Vec<u8>,
    ) -> GeorgeResult<()> {
        let mut req = RequestDiskInto::new();
        req.set_database_name(database_name);
        req.set_view_name(view_name);
        req.set_key(key);
        req.set_value(value);
        let resp = self
            .client
            .set(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => match resp.status {
                Status::Ok => Ok(()),
                _ => Err(Tools::response_err(resp)),
            },
            Err(err) => Err(Errs::strs("disk set failed!", err)),
        }
    }

    pub(crate) fn get(
        &self,
        database_name: String,
        view_name: String,
        key: String,
    ) -> GeorgeResult<Vec<u8>> {
        let mut req = RequestDiskOut::new();
        req.set_database_name(database_name);
        req.set_view_name(view_name);
        req.set_key(key);
        let resp = self
            .client
            .get(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => match resp.status {
                Status::Ok => Ok(resp.value),
                _ => Err(Tools::response_cus(resp.status, resp.msg_err)),
            },
            Err(err) => Err(Errs::strs("disk get failed!", err)),
        }
    }

    pub(crate) fn get_by_index(
        &self,
        database_name: String,
        view_name: String,
        index_name: String,
        key: String,
    ) -> GeorgeResult<Vec<u8>> {
        let mut req = RequestDiskIOut::new();
        req.set_database_name(database_name);
        req.set_view_name(view_name);
        req.set_index_name(index_name);
        req.set_key(key);
        let resp = self
            .client
            .get_by_index(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => match resp.status {
                Status::Ok => Ok(resp.value),
                _ => Err(Tools::response_cus(resp.status, resp.msg_err)),
            },
            Err(err) => Err(Errs::strs("disk get by index failed!", err)),
        }
    }

    pub(crate) fn remove(
        &self,
        database_name: String,
        view_name: String,
        key: String,
    ) -> GeorgeResult<()> {
        let mut req = RequestDiskRemove::new();
        req.set_database_name(database_name);
        req.set_view_name(view_name);
        req.set_key(key);
        let resp = self
            .client
            .remove(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => match resp.status {
                Status::Ok => Ok(()),
                _ => Err(Tools::response_cus(resp.status, resp.msg_err)),
            },
            Err(err) => Err(Errs::strs("disk remove failed!", err)),
        }
    }

    pub(crate) fn select(
        &self,
        database_name: String,
        view_name: String,
        constraint_json_bytes: Vec<u8>,
    ) -> GeorgeResult<DiskSelected> {
        let mut req = RequestDiskSelect::new();
        req.set_database_name(database_name);
        req.set_view_name(view_name);
        req.set_constraint_json_bytes(constraint_json_bytes);
        let resp = self
            .client
            .select(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => match resp.status {
                Status::Ok => Ok(resp.selected.unwrap()),
                _ => Err(Tools::response_cus(resp.status, resp.msg_err)),
            },
            Err(err) => Err(Errs::strs("disk select failed!", err)),
        }
    }

    pub(crate) fn delete(
        &self,
        database_name: String,
        view_name: String,
        constraint_json_bytes: Vec<u8>,
    ) -> GeorgeResult<DiskDeleted> {
        let mut req = RequestDiskDelete::new();
        req.set_database_name(database_name);
        req.set_view_name(view_name);
        req.set_constraint_json_bytes(constraint_json_bytes);
        let resp = self
            .client
            .delete(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => match resp.status {
                Status::Ok => Ok(resp.deleted.unwrap()),
                _ => Err(Tools::response_cus(resp.status, resp.msg_err)),
            },
            Err(err) => Err(Errs::strs("disk delete failed!", err)),
        }
    }
}
