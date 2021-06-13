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

use db::task::traits::TMaster;
use db::Task;
use protocols::impls::db::service_grpc::UserService;

use crate::utils::Comm;
use db::utils::comm::{DATABASE_SYS_NAME, VIEW_USER_NAME};
use protocols::impls::db::response::Response;
use protocols::impls::db::user::RequestLogin;

pub(crate) struct UserServer {
    pub(crate) task: Arc<Task>,
}

impl UserService for UserServer {
    fn login(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestLogin>,
        resp: ServerResponseUnarySink<Response>,
    ) -> Result<()> {
        match self.task.get_disk(
            DATABASE_SYS_NAME.to_string(),
            VIEW_USER_NAME.to_string(),
            req.message.name,
        ) {
            Ok(res) => match String::from_utf8(res) {
                Ok(res) => {
                    if res.eq(&req.message.pass) {
                        resp.finish(Comm::proto_success_db())
                    } else {
                        resp.finish(Comm::proto_failed_db_custom(
                            "user is not exist or pass is wrong!".to_string(),
                        ))
                    }
                }
                Err(err) => resp.finish(Comm::proto_failed_db_custom(err.to_string())),
            },
            Err(err) => resp.finish(Comm::proto_failed_db_custom(err.to_string())),
        }
    }
}
