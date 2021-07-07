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

use crate::cmd::Command;

mod cmd;

// ./client -H 127.0.0.1 -P 9219 -u admin -p admin#123
// ./client -H 127.0.0.1 -P 9219 -u admin -p admin#123 -ca ca_path -k key_path -c cert_path -d domain_name
fn main() {
    Command::init()
}
