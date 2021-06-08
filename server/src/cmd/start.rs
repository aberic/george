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

use crate::cmd::Start;
use crate::service::Server;
use clap::{App, Arg, ArgMatches, SubCommand};
use comm::io::file::FilerHandler;
use comm::io::Filer;
use deploy::comm::DEPLOY_START_CONFIG_FILEPATH;
use deploy::Builder;
use std::io::Error;
use std::process::{Child, Command};

impl Start {
    pub fn subcommand() -> App<'static, 'static> {
        SubCommand::with_name("start")
            .version(Builder::version())
            .about("this is start")
            .arg(
                Arg::with_name("file")
                    .short("f")
                    .long("config-filepath")
                    .help("this is file")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("filepath")
                    .long("config-filepath-custom")
                    .help("this is file")
                    .takes_value(true),
            )
    }

    pub fn matches(matches: ArgMatches) {
        if let Some(res) = matches.value_of("file") {
            println!("prepare run with file...");
            start(Some(res.to_string()))
        } else if let Some(res) = matches.value_of("filepath") {
            println!("prepare run...");
            Server::start(res)
        } else {
            println!("prepare run with no file...");
            start(None)
        }
    }
}

fn start(filepath: Option<String>) {
    if Filer::exist("lock") {
        println!("george already started!");
        return;
    }
    println!("starting...");
    let cmd_child: Child;
    match filepath {
        Some(path) => {
            cmd_child = Command::new("./server start --config-filepath-custom")
                .arg(path)
                .spawn()
                .expect("server failed to start")
        }
        None => {
            cmd_child = Command::new("./server start --config-filepath-custom")
                .arg(DEPLOY_START_CONFIG_FILEPATH)
                .spawn()
                .expect("server failed to start")
        }
    }
    match cmd_child.wait_with_output() {
        Err(err) => println!("george start error, {}", err),
        _ => {}
    }
}
