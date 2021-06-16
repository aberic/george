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

use comm::errors::{Errs, GeorgeError};
use protocols::impls::db::response::{Response, Status};
use protocols::impls::db::service_grpc::{
    DatabaseServiceClient, DiskServiceClient, IndexServiceClient, MemoryServiceClient,
    PageServiceClient, UserServiceClient, ViewServiceClient,
};

mod database;
mod disk;
mod index;
mod memory;
mod page;
mod user;
mod view;
mod view_test;

pub(crate) struct User {
    client: UserServiceClient,
}

pub(crate) struct Database {
    client: DatabaseServiceClient,
}

pub(crate) struct Page {
    client: PageServiceClient,
}

pub(crate) struct View {
    client: ViewServiceClient,
}

pub(crate) struct Index {
    client: IndexServiceClient,
}

pub(crate) struct Disk {
    client: DiskServiceClient,
}

pub(crate) struct Memory {
    client: MemoryServiceClient,
}

pub(crate) struct Tools;

impl Tools {
    fn response_err(resp: Response) -> GeorgeError {
        Tools::response_cus(resp.status, resp.msg_err)
    }

    fn response_cus(status: Status, msg_err: String) -> GeorgeError {
        Errs::string(format!("status {:#?}, error msg: {}", status, msg_err))
    }
}
