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

use protocols::impls::db::service_grpc::ParseServiceClient;

use crate::service::Parse;
use comm::errors::{Errs, GeorgeResult};
use protocols::impls::db::parse::RequestParse;
use protocols::impls::db::response::Status;

impl Parse {
    pub(crate) fn new(remote: &str, port: u16) -> Parse {
        Parse {
            client: ParseServiceClient::new_plain(remote, port, Default::default()).unwrap(),
        }
    }
}

impl Parse {
    pub(crate) fn scan(&self, scan_str: String) -> GeorgeResult<Vec<u8>> {
        let mut req = RequestParse::new();
        req.set_scan_str(scan_str);
        let resp = self
            .client
            .parse(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok((_m, resp, _md)) => match resp.status {
                Status::Ok => Ok(resp.res),
                _ => Err(Errs::string(format!(
                    "parse failed! status: {:#?}, msg: {}",
                    resp.status, resp.msg_err
                ))),
            },
            Err(err) => Err(Errs::strs("parse failed!", err)),
        }
    }
}
