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
use tokio::sync::mpsc;

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

    pub fn matches(matches: &ArgMatches) {
        if let Some(res) = matches.value_of("file") {
            println!("prepare run with file...");
            // start(Some(res.to_string()))
            Server::start(res)
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
        println!("george already started or try to use restart instead!");
        return;
    }
    println!("starting...");
    let fp: String;
    match filepath {
        Some(path) => fp = path,
        None => fp = DEPLOY_START_CONFIG_FILEPATH.to_string(),
    }
    // let program = format!("./server start --config-filepath-custom {}", fp);
    // println!("program: {}", program);
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(server_start(fp));
    println!("server started, [PID] {} running...", 100);
}

async fn server_start(filepath: String) {
    let (sender, mut receiver) = mpsc::channel(32);
    tokio::spawn(async move {
        match sender.send(Server::start(filepath)).await {
            Err(err) => {
                panic!("sender send error {}", err);
            }
            _ => {}
        }
    });

    println!("send finish...");
    while let Some(_) = receiver.recv().await {
        println!("success!");
    }
}
