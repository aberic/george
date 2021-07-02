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

use crate::protos::db::db::{Engine, KeyType};
use crate::protos::utils::utils::Timestamp;
use crate::tools::Trans;
use comm::errors::{Errs, GeorgeResult};
use comm::Time;

impl Trans {
    pub fn proto_time_2_grpc_timestamp(time: Time) -> Timestamp {
        let (seconds, nanos) = time.secs_nanos();
        Timestamp { seconds, nanos }
    }
    pub fn proto_grpc_timestamp_2_time(secs: i64) -> Time {
        Time::from_secs(secs)
    }

    pub fn db_2_engine(e: db::utils::enums::Engine) -> Engine {
        match e {
            db::utils::enums::Engine::None => Engine::None,
            db::utils::enums::Engine::Disk => Engine::Disk,
            db::utils::enums::Engine::Sequence => Engine::Sequence,
            db::utils::enums::Engine::Block => Engine::Block,
            db::utils::enums::Engine::Increment => Engine::Increment,
        }
    }

    pub fn db_2_engine_i32(e: db::utils::enums::Engine) -> i32 {
        match e {
            db::utils::enums::Engine::None => Engine::None as i32,
            db::utils::enums::Engine::Disk => Engine::Disk as i32,
            db::utils::enums::Engine::Sequence => Engine::Sequence as i32,
            db::utils::enums::Engine::Block => Engine::Block as i32,
            db::utils::enums::Engine::Increment => Engine::Increment as i32,
        }
    }

    pub fn i32_2_db_engine(res: i32) -> GeorgeResult<db::utils::enums::Engine> {
        if (Engine::None as i32) == res {
            Ok(db::utils::enums::Engine::None)
        } else if (Engine::Disk as i32) == res {
            Ok(db::utils::enums::Engine::Disk)
        } else if (Engine::Sequence as i32) == res {
            Ok(db::utils::enums::Engine::Sequence)
        } else if (Engine::Block as i32) == res {
            Ok(db::utils::enums::Engine::Block)
        } else if (Engine::Increment as i32) == res {
            Ok(db::utils::enums::Engine::Increment)
        } else {
            Err(Errs::string(format!("no match engine with {}", res)))
        }
    }

    pub fn db_2_key_type(e: db::utils::enums::KeyType) -> KeyType {
        match e {
            db::utils::enums::KeyType::None => KeyType::Nonsupport,
            db::utils::enums::KeyType::String => KeyType::String,
            db::utils::enums::KeyType::UInt => KeyType::UInt,
            db::utils::enums::KeyType::Int => KeyType::Int,
            db::utils::enums::KeyType::Bool => KeyType::Bool,
            db::utils::enums::KeyType::Float => KeyType::Float,
        }
    }

    pub fn db_2_key_type_i32(e: db::utils::enums::KeyType) -> i32 {
        match e {
            db::utils::enums::KeyType::None => KeyType::Nonsupport as i32,
            db::utils::enums::KeyType::String => KeyType::String as i32,
            db::utils::enums::KeyType::UInt => KeyType::UInt as i32,
            db::utils::enums::KeyType::Int => KeyType::Int as i32,
            db::utils::enums::KeyType::Bool => KeyType::Bool as i32,
            db::utils::enums::KeyType::Float => KeyType::Float as i32,
        }
    }

    pub fn key_type_2_db(e: KeyType) -> db::utils::enums::KeyType {
        match e {
            KeyType::Nonsupport => db::utils::enums::KeyType::None,
            KeyType::String => db::utils::enums::KeyType::String,
            KeyType::UInt => db::utils::enums::KeyType::UInt,
            KeyType::Int => db::utils::enums::KeyType::Int,
            KeyType::Bool => db::utils::enums::KeyType::Bool,
            KeyType::Float => db::utils::enums::KeyType::Float,
        }
    }

    pub fn i32_2_db_key_type(res: i32) -> GeorgeResult<db::utils::enums::KeyType> {
        if (KeyType::Nonsupport as i32) == res {
            Ok(db::utils::enums::KeyType::None)
        } else if (KeyType::String as i32) == res {
            Ok(db::utils::enums::KeyType::String)
        } else if (KeyType::UInt as i32) == res {
            Ok(db::utils::enums::KeyType::UInt)
        } else if (KeyType::Int as i32) == res {
            Ok(db::utils::enums::KeyType::Int)
        } else if (KeyType::Bool as i32) == res {
            Ok(db::utils::enums::KeyType::Bool)
        } else if (KeyType::Float as i32) == res {
            Ok(db::utils::enums::KeyType::Float)
        } else {
            Err(Errs::string(format!("no match key type with {}", res)))
        }
    }
}
