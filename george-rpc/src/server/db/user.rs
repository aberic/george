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

use tonic::{Request, Response, Status};

use george_db::task::traits::TMaster;
use george_db::Task;

use crate::protos::db::db::user_service_server::UserService;
use crate::protos::db::db::RequestLogin;
use crate::protos::utils::utils::Resp;
use crate::server::db::{UserServer, DATABASE_SYS, VIEW_USER};
use crate::tools::Results;

impl UserServer {
    pub fn new(task: Arc<Task>) -> Self {
        UserServer { task }
    }
}

#[tonic::async_trait]
impl UserService for UserServer {
    async fn login(&self, request: Request<RequestLogin>) -> Result<Response<Resp>, Status> {
        match self.task.get_disk(
            DATABASE_SYS.to_string(),
            VIEW_USER.to_string(),
            request.get_ref().name.clone(),
        ) {
            Ok(res) => match String::from_utf8(res) {
                Ok(res) => {
                    if res.eq(&request.get_ref().pass) {
                        Results::success()
                    } else {
                        Results::failed_custom("user is not exist or pass is wrong!".to_string())
                    }
                }
                Err(err) => Results::failed_errs(err),
            },
            Err(err) => Results::failed_err(err),
        }
    }
}
