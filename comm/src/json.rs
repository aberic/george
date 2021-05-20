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

use crate::errors::entrances::{Errs, GeorgeResult};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

pub struct Json {
    root: Value,
}

pub trait JsonFrom<Vec, String> {
    fn from_slice(data: Vec) -> GeorgeResult<Json>;
    fn from_string(data: String) -> GeorgeResult<Json>;
}

impl Json {
    pub fn root(&self) -> Value {
        self.root.clone()
    }

    pub fn to_string(&self) -> String {
        self.root.to_string()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.root.to_string().into_bytes()
    }

    pub fn to_obj<T>(&self) -> GeorgeResult<T>
    where
        T: DeserializeOwned,
    {
        match serde_json::from_value(self.root()) {
            Ok(t) => Ok(t),
            Err(err) => Err(Errs::strs("json to object", err)),
        }
    }

    pub fn obj_2_vec<T>(value: &T) -> GeorgeResult<Vec<u8>>
    where
        T: ?Sized + Serialize,
    {
        match serde_json::to_vec(value) {
            Ok(res) => Ok(res),
            Err(err) => Err(Errs::strs("object to bytes", err)),
        }
    }

    pub fn obj_2_string<T>(value: &T) -> GeorgeResult<String>
    where
        T: ?Sized + Serialize,
    {
        match serde_json::to_string(value) {
            Ok(res) => Ok(res),
            Err(err) => Err(Errs::strs("object to string", err)),
        }
    }
}

impl JsonFrom<&[u8], &str> for Json {
    fn from_slice(data: &[u8]) -> GeorgeResult<Json> {
        Ok(Json {
            root: from_slice(data)?,
        })
    }

    fn from_string(data: &str) -> GeorgeResult<Json> {
        Ok(Json {
            root: from_string(data)?,
        })
    }
}

impl JsonFrom<Vec<u8>, String> for Json {
    fn from_slice(data: Vec<u8>) -> GeorgeResult<Json> {
        Ok(Json {
            root: from_slice(data.as_slice())?,
        })
    }

    fn from_string(data: String) -> GeorgeResult<Json> {
        Ok(Json {
            root: from_string(data.as_str())?,
        })
    }
}

fn from_slice(data: &[u8]) -> GeorgeResult<Value> {
    match serde_json::from_slice(data) {
        Ok(dr) => Ok(dr),
        Err(err) => Err(Errs::strs("json from slice", err)),
    }
}

fn from_string(data: &str) -> GeorgeResult<Value> {
    match serde_json::from_str(data) {
        Ok(dr) => Ok(dr),
        Err(err) => Err(Errs::strs("json from string", err)),
    }
}
