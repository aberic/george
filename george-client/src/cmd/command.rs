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

use clap::{App, ArgMatches};

use george_comm::errors::{Errs, GeorgeResult};
use george_comm::io::file::FilerReader;
use george_comm::io::Filer;
use george_deploy::Builder;
use george_rpc::client::{RpcClient, TLSType};

use crate::cmd::{Client, Command, Config, Options};

impl Command {
    pub fn init() {
        // todo config yaml
        match match_value(
            App::new("george-client")
                .version(Builder::version())
                .name("george-client")
                .author(Builder::author())
                .about(Builder::about())
                .arg(Options::remote())
                .arg(Options::port())
                .arg(Options::user())
                .arg(Options::pass())
                .arg(Options::config())
                .get_matches(),
        ) {
            Err(err) => println!("{}", err),
            _ => {}
        }
    }
}

fn match_value(matches: ArgMatches) -> GeorgeResult<()> {
    let remote: &str;
    let port: u16;
    let name: String;
    let pass: String;
    let config_path: String;
    if matches.is_present("remote") {
        remote = remote_fn(&matches);
        port = port_fn(&matches)?;
        name = user_fn(&matches);
        pass = pass_fn(&matches);
        config_path = config_fn(&matches);
    } else {
        return Err(Errs::str("remote & port & user & pass must be assign!"));
    }
    let config = Config::from(config_path)?;
    let mut client;
    if config.tls() {
        if let Some(ca) = config.tls_ca() {
            if config.tls_key().is_none() && config.tls_cert().is_none() {
                client = Client::new_tls(
                    config.tls_type(),
                    remote,
                    port,
                    ca,
                    config.domain(),
                    config.http_config(),
                )?;
            } else if config.tls_key().is_some() && config.tls_cert().is_some() {
                let key = config.tls_key_unwrap();
                let cert = config.tls_cert_unwrap();
                client = Client::new_tls_check(
                    config.tls_type(),
                    remote,
                    port,
                    key,
                    cert,
                    ca,
                    config.domain(),
                    config.http_config(),
                )?;
            } else {
                return Err(Errs::str("key & cert must be assign together!"));
            }
        } else {
            return Err(Errs::str("ca must be assign!"));
        }
    } else {
        client = Client::new(remote, port, config.http_config())?;
    }
    client.login(name, pass)?;
    Ok(client.scan())
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

fn config_fn(matches: &ArgMatches) -> String {
    if let Some(res) = matches.value_of("config-path") {
        res.to_string()
    } else {
        String::from("")
    }
}
