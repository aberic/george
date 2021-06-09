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

use crate::cmd::{Command, Config, Options};
use clap::{App, ArgMatches};
use comm::errors::{Errs, GeorgeResult};
use deploy::Builder;

impl Command {
    pub fn init() {
        match_value(
            App::new("george-client")
                .version(Builder::version())
                .name("george-client")
                .author(Builder::author())
                .about(Builder::about())
                .arg(Options::remote())
                .arg(Options::port())
                .arg(Options::user())
                .arg(Options::pass())
                .get_matches(),
        )
    }
}

fn match_value(matches: ArgMatches) {
    let remote: &str;
    let port: u16;
    let name: String;
    let pass: String;
    if matches.is_present("remote") {
        remote = remote_fn(&matches);
        port = port_fn(&matches).unwrap();
        name = user_fn(&matches);
        pass = pass_fn(&matches);
    } else if matches.is_present("port") {
        remote = "127.0.0.1";
        port = port_fn(&matches).unwrap();
        name = user_fn(&matches);
        pass = pass_fn(&matches);
    } else if matches.is_present("user") {
        remote = "127.0.0.1";
        port = 9219;
        name = user_fn(&matches);
        pass = pass_fn(&matches);
    } else {
        println!("user & pass must be assign!");
        return;
    }
    let config = Config::new(remote, port);
    match config.login(name, pass) {
        Ok(()) => config.scan(),
        Err(err) => println!("login failed! {}", err),
    }
}

fn remote_fn<'a>(matches: &'a ArgMatches) -> &'a str {
    if let Some(res) = matches.value_of("remote") {
        res
    } else {
        "127.0.0.1"
    }
}

fn port_fn(matches: &ArgMatches) -> GeorgeResult<u16> {
    if let Some(res) = matches.value_of("port") {
        if let Ok(port) = res.parse::<u16>() {
            Ok(port)
        } else {
            Err(Errs::str("port must be u16!"))
        }
    } else {
        Ok(9219)
    }
}

fn user_fn(matches: &ArgMatches) -> String {
    if let Some(res) = matches.value_of("user") {
        res.to_string()
    } else {
        String::from("")
    }
}

fn pass_fn(matches: &ArgMatches) -> String {
    if let Some(res) = matches.value_of("pass") {
        res.to_string()
    } else {
        String::from("")
    }
}
