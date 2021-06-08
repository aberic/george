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

use crate::comm::Config;
use clap::{Arg, ArgGroup};
use futures::executor;
use grpc::ClientStubExt;
use protocols::impls::db::service::Request;
use protocols::impls::db::service_grpc::DatabaseServiceClient;

pub struct Show;

impl Show {
    pub fn show_databases() -> Arg<'static, 'static> {
        Arg::with_name("databases")
            .long("databases")
            .help("show databases")
            .empty_values(true)
    }
}

impl Show {
    fn list(config: Config) {
        let client =
            DatabaseServiceClient::new_plain(config.host(), config.port(), Default::default())
                .unwrap();
        let req = Request::new();
        let resp = client
            .databases(grpc::RequestOptions::new(), req)
            .join_metadata_result();
        let resp = executor::block_on(resp);
        match resp {
            Ok(resp) => println!("DBList = {:#?}", resp.1),
            Err(err) => println!("err = {}", err.to_string()),
        }
    }
}
