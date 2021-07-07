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
use crate::protos::utils::utils::{Status, Timestamp};
use crate::tools::Trans;
use comm::errors::{Errs, GeorgeResult};
use comm::Time;

impl Trans {
    pub fn time_2_grpc_timestamp(time: Time) -> Timestamp {
        let (seconds, nanos) = time.secs_nanos();
        Timestamp { seconds, nanos }
    }

    pub fn grpc_timestamp_2_time(secs: i64) -> Time {
        Time::from_secs(secs)
    }

    pub fn grpc_timestamp_2_string(secs: i64) -> String {
        Time::from_secs(secs).to_string("%Y-%m-%d %H:%M:%S")
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

    pub fn engine_from_str(engine: String) -> Engine {
        match engine.as_str() {
            "Disk" => Engine::Disk,
            "Increment" => Engine::Increment,
            "Block" => Engine::Block,
            "Sequence" => Engine::Sequence,
            _ => Engine::None,
        }
    }

    pub fn i32_2_engine_str(res: i32) -> GeorgeResult<String> {
        if (Engine::None as i32) == res {
            Ok("None".to_string())
        } else if (Engine::Disk as i32) == res {
            Ok("Disk".to_string())
        } else if (Engine::Sequence as i32) == res {
            Ok("Increment".to_string())
        } else if (Engine::Block as i32) == res {
            Ok("Block".to_string())
        } else if (Engine::Increment as i32) == res {
            Ok("Sequence".to_string())
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

    pub fn i32_2_key_type_str(res: i32) -> GeorgeResult<String> {
        if (KeyType::Nonsupport as i32) == res {
            Ok("Nonsupport".to_string())
        } else if (KeyType::String as i32) == res {
            Ok("String".to_string())
        } else if (KeyType::UInt as i32) == res {
            Ok("UInt".to_string())
        } else if (KeyType::Int as i32) == res {
            Ok("Int".to_string())
        } else if (KeyType::Bool as i32) == res {
            Ok("Bool".to_string())
        } else if (KeyType::Float as i32) == res {
            Ok("Float".to_string())
        } else {
            Err(Errs::string(format!("no match key type with {}", res)))
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

    pub fn i32_2_status(res: i32) -> GeorgeResult<Status> {
        if (Status::Ok as i32) == res {
            Ok(Status::Ok)
        } else if (Status::Cancelled as i32) == res {
            Ok(Status::Cancelled)
        } else if (Status::Unknown as i32) == res {
            Ok(Status::Unknown)
        } else if (Status::Argument as i32) == res {
            Ok(Status::Argument)
        } else if (Status::DeadlineExceeded as i32) == res {
            Ok(Status::DeadlineExceeded)
        } else if (Status::NotFound as i32) == res {
            Ok(Status::NotFound)
        } else if (Status::AlreadyExists as i32) == res {
            Ok(Status::AlreadyExists)
        } else if (Status::PermissionDenied as i32) == res {
            Ok(Status::PermissionDenied)
        } else if (Status::Unauthenticated as i32) == res {
            Ok(Status::Unauthenticated)
        } else if (Status::ResourceExhausted as i32) == res {
            Ok(Status::ResourceExhausted)
        } else if (Status::FailedPrecondition as i32) == res {
            Ok(Status::FailedPrecondition)
        } else if (Status::Aborted as i32) == res {
            Ok(Status::Aborted)
        } else if (Status::OutOfRange as i32) == res {
            Ok(Status::OutOfRange)
        } else if (Status::Unimplemented as i32) == res {
            Ok(Status::Unimplemented)
        } else if (Status::Internal as i32) == res {
            Ok(Status::Internal)
        } else if (Status::Unavailable as i32) == res {
            Ok(Status::Unavailable)
        } else if (Status::DataLoss as i32) == res {
            Ok(Status::DataLoss)
        } else if (Status::Custom as i32) == res {
            Ok(Status::Custom)
        } else {
            Err(Errs::string(format!("no match status with {}", res)))
        }
    }
}
