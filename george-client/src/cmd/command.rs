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

use crate::cmd::{Command, Config, Options};

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
                .arg(Options::tls())
                .arg(Options::key())
                .arg(Options::cert())
                .arg(Options::ca())
                .arg(Options::domain())
                .arg(Options::user())
                .arg(Options::pass())
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
    let tls: bool;
    let mut key_bytes_op: Option<Vec<u8>> = None;
    let mut cert_bytes_op: Option<Vec<u8>> = None;
    let mut ca_bytes_op: Option<Vec<u8>> = None;
    let mut domain_name: String = "".to_string();
    let name: String;
    let pass: String;
    if matches.is_present("remote") {
        remote = remote_fn(&matches);
        port = port_fn(&matches)?;
        tls = tls_fn(&matches)?;
        if tls {
            key_bytes_op = key_fn(&matches)?;
            cert_bytes_op = cert_fn(&matches)?;
            ca_bytes_op = ca_fn(&matches)?;
            domain_name = domain_fn(&matches);
        }
        name = user_fn(&matches);
        pass = pass_fn(&matches);
    } else {
        return Err(Errs::str("user & pass must be assign!"));
    }
    let mut config;
    if tls {
        if let Some(ca) = ca_bytes_op {
            if key_bytes_op.is_none() && cert_bytes_op.is_none() {
                config =
                    Config::new_tls_bytes(TLSType::Openssl, remote, port, ca, domain_name, None)?;
            } else if key_bytes_op.is_some() && cert_bytes_op.is_some() {
                let key = key_bytes_op.unwrap();
                let cert = cert_bytes_op.unwrap();
                config = Config::new_tls_bytes_check(
                    TLSType::Openssl,
                    remote,
                    port,
                    key,
                    cert,
                    ca,
                    domain_name,
                    None,
                )?;
            } else {
                return Err(Errs::str("key & cert must be assign together!"));
            }
        } else {
            return Err(Errs::str("ca must be assign!"));
        }
    } else {
        config = Config::new(remote, port, None)?;
    }
    config.login(name, pass)?;
    Ok(config.scan())
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

fn tls_fn(matches: &ArgMatches) -> GeorgeResult<bool> {
    if let Some(res) = matches.value_of("tls") {
        if let Ok(res) = res.parse::<bool>() {
            println!("login with tls {}", res);
            Ok(res)
        } else {
            Err(Errs::str("tls must be bool!"))
        }
    } else {
        println!("login with tls false");
        Ok(false)
    }
}

fn key_fn(matches: &ArgMatches) -> GeorgeResult<Option<Vec<u8>>> {
    if let Some(res) = matches.value_of("key") {
        let path = res.to_string();
        println!("key path = {}", path);
        Ok(Some(Filer::read_bytes(path)?))
    } else {
        Ok(None)
    }
}

fn cert_fn(matches: &ArgMatches) -> GeorgeResult<Option<Vec<u8>>> {
    if let Some(res) = matches.value_of("cert") {
        let path = res.to_string();
        println!("cert path = {}", path);
        Ok(Some(Filer::read_bytes(path)?))
    } else {
        Ok(None)
    }
}

fn ca_fn(matches: &ArgMatches) -> GeorgeResult<Option<Vec<u8>>> {
    if let Some(res) = matches.value_of("ca") {
        let path = res.to_string();
        println!("ca path = {}", path);
        Ok(Some(Filer::read_bytes(path)?))
    } else {
        Ok(None)
    }
}

fn domain_fn(matches: &ArgMatches) -> String {
    if let Some(res) = matches.value_of("domain") {
        res.to_string()
    } else {
        String::from("")
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
