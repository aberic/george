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

use std::sync::Arc;

use grpc::{Result, ServerHandlerContext, ServerRequestSingle, ServerResponseUnarySink};

use db::Task;
use protocols::impls::db::service_grpc::ParseService;

use crate::parse::Parse;
use protocols::impls::db::parse::{RequestParse, ResponseParse};
use protocols::impls::db::response::Status;

pub(crate) struct ParseServer {
    pub(crate) task: Arc<Task>,
}

impl ParseService for ParseServer {
    fn parse(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestParse>,
        resp: ServerResponseUnarySink<ResponseParse>,
    ) -> Result<()> {
        match Parse::analysis(self.task.clone(), req.message.scan_str) {
            Ok(res) => {
                let mut response = ResponseParse::new();
                response.set_status(Status::Ok);
                response.set_res(res);
                resp.finish(response)
            }
            Err(err) => {
                let mut response = ResponseParse::new();
                response.set_status(Status::Custom);
                response.set_msg_err(err.to_string());
                resp.finish(response)
            }
        }
    }
}
