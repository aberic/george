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

use clap::Arg;

use crate::cmd::Options;

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

    pub(crate) fn config() -> Arg<'static, 'static> {
        Arg::with_name("config-path")
            .short("c")
            .help("client init config filepath")
            .takes_value(true)
    }
}
