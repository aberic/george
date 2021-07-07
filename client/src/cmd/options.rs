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

use crate::cmd::Options;
use clap::Arg;

impl Options {
    pub(crate) fn remote() -> Arg<'static, 'static> {
        Arg::with_name("remote")
            .short("H")
            .help("remote host address")
            .takes_value(true)
    }

    pub(crate) fn port() -> Arg<'static, 'static> {
        Arg::with_name("port")
            .short("P")
            .help("remote host port")
            .takes_value(true)
    }

    pub(crate) fn tls() -> Arg<'static, 'static> {
        Arg::with_name("tls")
            .short("t")
            .help("remote host tls")
            .takes_value(true)
    }

    pub(crate) fn key() -> Arg<'static, 'static> {
        Arg::with_name("key")
            .short("k")
            .help("remote host key")
            .takes_value(true)
    }

    pub(crate) fn cert() -> Arg<'static, 'static> {
        Arg::with_name("cert")
            .short("c")
            .help("remote host cert")
            .takes_value(true)
    }

    pub(crate) fn ca() -> Arg<'static, 'static> {
        Arg::with_name("ca")
            .help("remote host ca")
            .takes_value(true)
    }

    pub(crate) fn domain() -> Arg<'static, 'static> {
        Arg::with_name("domain")
            .short("d")
            .help("remote host domain")
            .takes_value(true)
    }

    pub(crate) fn user() -> Arg<'static, 'static> {
        Arg::with_name("user")
            .short("u")
            .help("remote host user")
            .takes_value(true)
    }

    pub(crate) fn pass() -> Arg<'static, 'static> {
        Arg::with_name("pass")
            .short("p")
            .help("remote host pass")
            .takes_value(true)
    }
}
