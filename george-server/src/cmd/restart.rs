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

use crate::cmd::Restart;
use clap::{App, Arg, ArgMatches, SubCommand};
use george_deploy::Builder;

impl Restart {
    pub fn subcommand() -> App<'static, 'static> {
        SubCommand::with_name("restart")
            .version(Builder::version())
            .about("this is restart")
            .arg(
                Arg::with_name("file")
                    .short("f")
                    .long("config-filename")
                    .help("this is file")
                    .takes_value(true),
            )
    }

    pub fn matches(matches: &ArgMatches) {
        if let Some(res) = matches.value_of("file") {
            println!("restart file out: {}", res);
        } else {
            println!("restart default");
        }
    }
}
