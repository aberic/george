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

use protobuf::well_known_types::Timestamp;

use comm::Time;

use crate::impls::db;
use crate::impls::utils::Comm;

impl Comm {
    pub fn proto_time_2_grpc_timestamp(time: Time) -> Timestamp {
        let mut timestamp = Timestamp::new();
        let (secs, nanos) = time.secs_nanos();
        timestamp.set_seconds(secs);
        timestamp.set_nanos(nanos);
        timestamp
    }
    pub fn proto_grpc_timestamp_2_time(secs: i64) -> Time {
        Time::from_secs(secs)
    }

    pub fn proto_success_db() -> db::response::Response {
        let mut response = db::response::Response::new();
        response.set_status(db::response::Status::Ok);
        response
    }

    pub fn proto_failed_db_custom(msg: String) -> db::response::Response {
        let mut response = db::response::Response::new();
        response.set_status(db::response::Status::Custom);
        response.set_msg_err(msg);
        response
    }

    pub fn parse_str(str: String) -> String {
        trim_parse(str)
    }

    pub fn split_str(str: String) -> Vec<String> {
        let mut vss: Vec<String> = vec![];
        let mut vsi = str.split(" ");
        let mut v = vsi.next();
        while v.is_some() {
            vss.push(v.unwrap().to_string());
            v = vsi.next()
        }
        vss
    }
}

fn trim_parse(str: String) -> String {
    let str = trim_n(str);
    match str.strip_suffix(" ") {
        Some(str) => match str.strip_prefix(" ") {
            Some(str) => str.to_string(),
            None => str.to_string(),
        },
        None => match str.strip_suffix(" ") {
            Some(str) => str.to_string(),
            None => str.to_string(),
        },
    }
}

fn trim_n(str: String) -> String {
    if str.contains("\n") {
        let str = str.replace("\n", " ");
        trim_n(str)
    } else {
        trim_t(str)
    }
}

fn trim_t(str: String) -> String {
    if str.contains("  ") {
        let str = str.replace("  ", " ");
        trim_t(str)
    } else {
        str.to_string()
    }
}
