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
use protocols::impls::db::memory::{
    RequestMemoryInto, RequestMemoryOut, RequestMemoryPInto, RequestMemoryPOut,
    RequestMemoryPRemove, RequestMemoryRemove,
};
use protocols::impls::db::service_grpc::MemoryServiceClient;

use crate::service::{Memory, Tools};

impl Memory {
    pub(crate) fn new(remote: &str, port: u16) -> Memory {
        Memory {
            client: MemoryServiceClient::new_plain(remote, port, Default::default()).unwrap(),
        }
    }
}

impl Memory {
    pub(crate) fn put(&self, key: String, value: Vec<u8>) -> GeorgeResult<()> {
        let mut req = RequestMemoryInto::new();
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
            Err(err) => Err(Errs::strs("memory put failed!", err)),
        }
    }

    pub(crate) fn set(&self, key: String, value: Vec<u8>) -> GeorgeResult<()> {
        let mut req = RequestMemoryInto::new();
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
            Err(err) => Err(Errs::strs("memory set failed!", err)),
        }
    }

    pub(crate) fn get(&self, key: String) -> GeorgeResult<Vec<u8>> {
        let mut req = RequestMemoryOut::new();
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
            Err(err) => Err(Errs::strs("memory get failed!", err)),
        }
    }

    pub(crate) fn remove(&self, key: String) -> GeorgeResult<()> {
        let mut req = RequestMemoryRemove::new();
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
            Err(err) => Err(Errs::strs("memory remove failed!", err)),
        }
    }

    pub(crate) fn put_by_page(
        &self,
        page_name: String,
        key: String,
        value: Vec<u8>,
    ) -> GeorgeResult<()> {
        let mut req = RequestMemoryPInto::new();
        req.set_page_name(page_name);
        req.set_key(key);
        req.set_value(value);
        let resp = self
            .client
            .put_by_page(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => match resp.status {
                Status::Ok => Ok(()),
                _ => Err(Tools::response_err(resp)),
            },
            Err(err) => Err(Errs::strs("memory put by page failed!", err)),
        }
    }

    pub(crate) fn set_by_page(
        &self,
        page_name: String,
        key: String,
        value: Vec<u8>,
    ) -> GeorgeResult<()> {
        let mut req = RequestMemoryPInto::new();
        req.set_page_name(page_name);
        req.set_key(key);
        req.set_value(value);
        let resp = self
            .client
            .set_by_page(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => match resp.status {
                Status::Ok => Ok(()),
                _ => Err(Tools::response_err(resp)),
            },
            Err(err) => Err(Errs::strs("memory set by page failed!", err)),
        }
    }

    pub(crate) fn get_by_page(&self, page_name: String, key: String) -> GeorgeResult<Vec<u8>> {
        let mut req = RequestMemoryPOut::new();
        req.set_page_name(page_name);
        req.set_key(key);
        let resp = self
            .client
            .get_by_page(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => match resp.status {
                Status::Ok => Ok(resp.value),
                _ => Err(Tools::response_cus(resp.status, resp.msg_err)),
            },
            Err(err) => Err(Errs::strs("memory get by page failed!", err)),
        }
    }

    pub(crate) fn remove_by_page(&self, page_name: String, key: String) -> GeorgeResult<()> {
        let mut req = RequestMemoryPRemove::new();
        req.set_page_name(page_name);
        req.set_key(key);
        let resp = self
            .client
            .remove_by_page(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => match resp.status {
                Status::Ok => Ok(()),
                _ => Err(Tools::response_cus(resp.status, resp.msg_err)),
            },
            Err(err) => Err(Errs::strs("memory remove by page failed!", err)),
        }
    }
}
