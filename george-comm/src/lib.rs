/*
 * Copyright (c) 2020. Aberic - All Rights Reserved.
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

#[macro_use]
extern crate lazy_static;
extern crate phf;

use chrono::Duration;

pub mod cryptos;
pub mod env;
mod env_test;
pub mod errors;
pub mod io;
pub mod json;
mod json_test;
pub mod merkle;
pub mod openssl;
pub mod pool;
pub mod strings;
mod strings_test;
pub mod time;
mod time_test;
pub mod trans;
mod trans_test;
pub mod vectors;
mod vectors_test;
pub mod yaml;
mod yaml_test;

pub struct Env;

pub struct Strings;

#[derive(Debug, Copy, Clone)]
pub struct Time {
    duration: Duration,
}

pub struct Trans;

pub struct Vector;

pub struct Json {
    value: serde_json::Value,
}

pub struct JsonArray {
    value: serde_json::Value,
}

pub struct Yaml {
    value: serde_yaml::Value,
}

pub struct YamlArray {
    value: serde_yaml::Value,
}
