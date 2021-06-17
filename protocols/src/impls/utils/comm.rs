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
use crate::impls::db::index::{Engine, KeyType};
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

    pub fn key_type_str(key_type: KeyType) -> String {
        match key_type {
            KeyType::String => "String".to_string(),
            KeyType::Int => "Int".to_string(),
            KeyType::Float => "Float".to_string(),
            KeyType::Bool => "Bool".to_string(),
            KeyType::UInt => "UInt".to_string(),
            KeyType::Nonsupport => "Nonsupport".to_string(),
        }
    }

    pub fn engine_str(engine: Engine) -> String {
        match engine {
            Engine::None => "None".to_string(),
            Engine::Disk => "Disk".to_string(),
            Engine::Increment => "Increment".to_string(),
            Engine::Block => "Block".to_string(),
            Engine::Sequence => "Sequence".to_string(),
        }
    }

    pub fn key_type_from_str(key_type: String) -> KeyType {
        match key_type.as_str() {
            "String" => KeyType::String,
            "Int" => KeyType::Int,
            "Float" => KeyType::Float,
            "Bool" => KeyType::Bool,
            "UInt" => KeyType::UInt,
            _ => KeyType::Nonsupport,
        }
    }

    pub fn engine_from_str(engine: String) -> Engine {
        match engine.as_str() {
            "Disk" => Engine::Disk,
            "Increment" => Engine::Increment,
            "Block" => Engine::Block,
            "Sequence" => Engine::Sequence,
            _ => Engine::None,
        }
    }

    pub fn trim_str(str: String) -> String {
        trim_str(str)
    }

    pub fn parse_str(str: String) -> String {
        let str = trim_str(str);
        str[0..str.len() - 1].to_string()
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

fn trim_str(str: String) -> String {
    let str = str.to_lowercase();
    trim_parse(str)
}

fn trim_parse(str: String) -> String {
    let str = trim_n(str);
    let str = trim_t(str);
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
        str.to_string()
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
