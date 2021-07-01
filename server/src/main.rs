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

use crate::cmd::{Command, Service};

mod cmd;

// ./server start -f /Users/aberic/Documents/path/rust/george/server/src/example/conf.yaml
fn main() {
    // Command::init();
    // 测试时启用如下代码
    // Service::start("server/src/examples/conf.yaml")
    // Service::start("server/src/examples/conf_tls_cross.yaml")
    Service::start("server/src/examples/conf_tls.yaml")
}
