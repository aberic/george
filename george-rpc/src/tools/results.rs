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

use std::error::Error;

use tonic::Response;

use george_comm::errors::GeorgeError;

use crate::protos::utils::utils::{Resp, Status};
use crate::tools::Results;

impl Results {
    pub fn success_status() -> i32 {
        Status::Ok as i32
    }

    pub fn failed_status(err: GeorgeError) -> i32 {
        status(err) as i32
    }

    pub fn response<T>(message: T) -> Result<Response<T>, tonic::Status> {
        Ok(Response::new(message))
    }

    pub fn success() -> Result<Response<Resp>, tonic::Status> {
        let resp = Resp {
            status: Status::Ok as i32,
            msg_err: "".to_string(),
        };
        Ok(Response::new(resp))
    }

    pub fn failed(msg_err: String, status: Status) -> Result<Response<Resp>, tonic::Status> {
        let resp = Resp {
            status: status as i32,
            msg_err,
        };
        Ok(Response::new(resp))
    }

    pub fn failed_custom(msg_err: String) -> Result<Response<Resp>, tonic::Status> {
        let resp = Resp {
            status: Status::Custom as i32,
            msg_err,
        };
        Ok(Response::new(resp))
    }

    pub fn failed_errs<E: Error>(err: E) -> Result<Response<Resp>, tonic::Status> {
        let resp = Resp {
            status: Status::Custom as i32,
            msg_err: err.to_string(),
        };
        Ok(Response::new(resp))
    }

    pub fn failed_err(err: GeorgeError) -> Result<Response<Resp>, tonic::Status> {
        let status = status(err.clone());
        let resp = Resp {
            status: status as i32,
            msg_err: err.to_string(),
        };
        Ok(Response::new(resp))
    }

    pub fn status(err: GeorgeError) -> Status {
        status(err)
    }
}

fn status(err: GeorgeError) -> Status {
    match err {
        GeorgeError::DirExistError(_)
        | GeorgeError::FileExistError(_)
        | GeorgeError::DataExistError(_)
        | GeorgeError::PageExistError(_)
        | GeorgeError::DatabaseExistError(_)
        | GeorgeError::ViewExistError(_)
        | GeorgeError::IndexExistError(_) => Status::AlreadyExists,
        GeorgeError::PageNoExistError(_)
        | GeorgeError::DirNoExistError(_)
        | GeorgeError::FileNoExistError(_)
        | GeorgeError::DataNoExistError(_)
        | GeorgeError::DatabaseNoExistError(_)
        | GeorgeError::ViewNoExistError(_)
        | GeorgeError::IndexNoExistError(_)
        | GeorgeError::NoneError(_) => Status::NotFound,
        GeorgeError::MethodNoSupportError(_) => Status::Unimplemented,
        GeorgeError::StringError(_) => Status::Custom,
    }
}
