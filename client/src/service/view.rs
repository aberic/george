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
use protocols::impls::db::response::Status;
use protocols::impls::db::service_grpc::ViewServiceClient;
use protocols::impls::db::view::{
    RequestViewArchive, RequestViewCreate, RequestViewInfo, RequestViewList, RequestViewModify,
    RequestViewRecord, RequestViewRecords, RequestViewRemove, ViewList, ViewRecord,
};

use crate::service::{Tools, View};

impl View {
    pub(crate) fn new(remote: &str, port: u16) -> View {
        View {
            client: ViewServiceClient::new_plain(remote, port, Default::default()).unwrap(),
        }
    }
}

impl View {
    pub(crate) fn list(&self, database_name: String) -> GeorgeResult<ViewList> {
        let mut req = RequestViewList::new();
        req.set_database_name(database_name);
        let resp = self
            .client
            .list(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => Ok(resp),
            Err(err) => Err(Errs::strs("view list failed!", err)),
        }
    }

    pub(crate) fn create(
        &self,
        database_name: String,
        name: String,
        comment: String,
        increment: bool,
    ) -> GeorgeResult<()> {
        let mut req = RequestViewCreate::new();
        req.set_database_name(database_name);
        req.set_name(name);
        req.set_comment(comment);
        req.set_with_increment(increment);
        let resp = self
            .client
            .create(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => match resp.status {
                Status::Ok => Ok(()),
                _ => Err(Tools::response_err(resp)),
            },
            Err(err) => Err(Errs::strs("view create failed!", err)),
        }
    }

    pub(crate) fn info(
        &self,
        database_name: String,
        name: String,
    ) -> GeorgeResult<protocols::impls::db::view::View> {
        let mut req = RequestViewInfo::new();
        req.set_database_name(database_name);
        req.set_name(name);
        let resp = self
            .client
            .info(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => match resp.status {
                Status::Ok => Ok(resp.view.unwrap()),
                _ => Err(Tools::response_cus(resp.status, resp.msg_err)),
            },
            Err(err) => Err(Errs::strs("view info failed!", err)),
        }
    }

    pub(crate) fn modify(
        &self,
        database_name: String,
        name: String,
        name_new: String,
        comment: String,
    ) -> GeorgeResult<()> {
        let mut req = RequestViewModify::new();
        req.set_database_name(database_name);
        req.set_name(name);
        req.set_name_new(name_new);
        req.set_comment(comment);
        let resp = self
            .client
            .modify(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => match resp.status {
                Status::Ok => Ok(()),
                _ => Err(Tools::response_err(resp)),
            },
            Err(err) => Err(Errs::strs("view modify failed!", err)),
        }
    }

    pub(crate) fn remove(&self, database_name: String, name: String) -> GeorgeResult<()> {
        let mut req = RequestViewRemove::new();
        req.set_database_name(database_name);
        req.set_name(name);
        let resp = self
            .client
            .remove(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => match resp.status {
                Status::Ok => Ok(()),
                _ => Err(Tools::response_err(resp)),
            },
            Err(err) => Err(Errs::strs("view remove failed!", err)),
        }
    }

    pub(crate) fn archive(
        &self,
        database_name: String,
        name: String,
        filepath: String,
    ) -> GeorgeResult<()> {
        let mut req = RequestViewArchive::new();
        req.set_database_name(database_name);
        req.set_name(name);
        req.set_archive_file_path(filepath);
        let resp = self
            .client
            .archive(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => match resp.status {
                Status::Ok => Ok(()),
                _ => Err(Tools::response_err(resp)),
            },
            Err(err) => Err(Errs::strs("view archive failed!", err)),
        }
    }

    pub(crate) fn record(
        &self,
        database_name: String,
        name: String,
        version: u32,
    ) -> GeorgeResult<ViewRecord> {
        let mut req = RequestViewRecord::new();
        req.set_database_name(database_name);
        req.set_name(name);
        req.set_version(version);
        let resp = self
            .client
            .record(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => match resp.status {
                Status::Ok => Ok(resp.record.unwrap()),
                _ => Err(Tools::response_cus(resp.status, resp.msg_err)),
            },
            Err(err) => Err(Errs::strs("view record failed!", err)),
        }
    }

    pub(crate) fn records(
        &self,
        database_name: String,
        name: String,
    ) -> GeorgeResult<Vec<ViewRecord>> {
        let mut req = RequestViewRecords::new();
        req.set_database_name(database_name);
        req.set_name(name);
        let resp = self
            .client
            .records(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => match resp.status {
                Status::Ok => Ok(resp.records.to_vec()),
                _ => Err(Tools::response_cus(resp.status, resp.msg_err)),
            },
            Err(err) => Err(Errs::strs("view record failed!", err)),
        }
    }
}
