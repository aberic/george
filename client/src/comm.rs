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

use deploy::Builder;

pub(crate) struct Config {
    host: String,
    port: u16,
}

impl Config {
    pub(crate) fn new(host: String, port: u16) -> Self {
        Config { host, port }
    }

    pub(crate) fn version(&self) -> &str {
        Builder::version()
    }

    pub(crate) fn author(&self) -> &str {
        Builder::author()
    }

    pub(crate) fn about(&self) -> &str {
        Builder::author()
    }

    pub(crate) fn host(&self) -> &str {
        self.host.as_str()
    }

    pub(crate) fn port(&self) -> u16 {
        self.port
    }
}
