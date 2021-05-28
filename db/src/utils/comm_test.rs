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

use serde::{Deserialize, Serialize};

use comm::json::JsonHandler;
use comm::Json;

use crate::utils::comm::IndexKey;

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u8,
    blog: String,
    addr: String,
    married: bool,
    job: Job,
}

#[derive(Serialize, Deserialize)]
struct Job {
    company: String,
    age: u8,
}

#[test]
fn key_fetch_test() {
    let user = User {
        name: "a".to_string(),
        age: 10,
        blog: "false".to_string(),
        addr: "c".to_string(),
        married: false,
        job: Job {
            company: "d".to_string(),
            age: 20,
        },
    };
    let json_bytes = Json::obj_2_bytes(&user).unwrap();
    println!(
        "res1 = {:#?}",
        IndexKey::fetch(String::from("name"), json_bytes.clone())
    );
    println!(
        "res2 = {:#?}",
        IndexKey::fetch(String::from("age"), json_bytes.clone())
    );
    println!(
        "res3 = {:#?}",
        IndexKey::fetch(String::from("blog"), json_bytes.clone())
    );
    println!(
        "res4 = {:#?}",
        IndexKey::fetch(String::from("married"), json_bytes.clone())
    );
    println!(
        "res4 = {:#?}",
        IndexKey::fetch(String::from("job"), json_bytes.clone())
    );
}
