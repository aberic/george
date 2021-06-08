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

mod comm;
mod database;
mod disk;
mod index;
mod memory;
mod memory_page;
mod page;
mod show;
mod view;

use crate::comm::Config;
use crate::show::Show;
use clap::{App, Arg};
use futures::executor;
use grpc::ClientStubExt;
use protocols::impls::db::service::Request;
use protocols::impls::db::service_grpc::DatabaseServiceClient;

struct Database;

fn main() {
    let config = Config::new("127.0.0.1".to_string(), 9219);

    // test
    Database::list(config)

    // let matches = App::new("george")
    //     .version(config.version())
    //     .author(config.author())
    //     .about(config.about())
    //     .arg(
    //         Arg::with_name("test1")
    //             .short("m")
    //             .long("test1cmd")
    //             .help("this is test1")
    //             .empty_values(true),
    //     )
    //     .arg(
    //         Arg::with_name("test2")
    //             .short("n")
    //             .long("test2cmd")
    //             .help("this is test2")
    //             .empty_values(false),
    //     )
    //     .arg(Show::show_databases())
    //     .get_matches();
    //
    // if let Some(res) = matches.value_of("test1") {
    //     println!("test1 out: {}", res);
    // }
    //
    // if let Some(res) = matches.value_of("test2") {
    //     println!("test2 out: {}", res);
    // }
}
