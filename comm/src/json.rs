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
use serde::Serialize;
use serde_json::Value;

pub struct Json {
    value: Value,
}

pub struct JsonArray {
    value: Value,
}

pub trait JsonHandler {
    fn object<Object>(object: &Object) -> GeorgeResult<Self>
    where
        Object: ?Sized + Serialize,
        Self: std::marker::Sized;

    fn value(&self) -> Value;

    fn to_string(&self) -> String {
        self.value().to_string()
    }

    fn to_vec(&self) -> Vec<u8> {
        self.value().to_string().into_bytes()
    }

    fn to_object<Object>(&self) -> GeorgeResult<Object>
    where
        Object: DeserializeOwned,
    {
        match serde_json::from_value(self.value()) {
            Ok(t) => Ok(t),
            Err(err) => Err(Errs::strs("json to object", err)),
        }
    }

    fn obj_2_bytes<Object>(value: &Object) -> GeorgeResult<Vec<u8>>
    where
        Object: ?Sized + Serialize,
    {
        match serde_json::to_vec(value) {
            Ok(res) => Ok(res),
            Err(err) => Err(Errs::strs("object to bytes", err)),
        }
    }

    fn obj_2_string<Object>(value: &Object) -> GeorgeResult<String>
    where
        Object: ?Sized + Serialize,
    {
        match serde_json::to_string(value) {
            Ok(res) => Ok(res),
            Err(err) => Err(Errs::strs("object to string", err)),
        }
    }

    fn obj_2_value<Object>(value: &Object) -> GeorgeResult<Value>
    where
        Object: ?Sized + Serialize,
    {
        match serde_json::to_value(value) {
            Ok(res) => Ok(res),
            Err(err) => Err(Errs::strs("object to string", err)),
        }
    }

    fn bytes_2_obj<Object>(data: &[u8]) -> GeorgeResult<Object>
    where
        Object: DeserializeOwned,
    {
        match serde_json::from_slice(data) {
            Ok(t) => Ok(t),
            Err(err) => Err(Errs::strs("json to object", err)),
        }
    }

    fn string_2_obj<Object>(data: &str) -> GeorgeResult<Object>
    where
        Object: DeserializeOwned,
    {
        match serde_json::from_str(data) {
            Ok(t) => Ok(t),
            Err(err) => Err(Errs::strs("json to object", err)),
        }
    }

    fn value_2_obj<Object>(data: Value) -> GeorgeResult<Object>
    where
        Object: DeserializeOwned,
    {
        match serde_json::from_value(data) {
            Ok(t) => Ok(t),
            Err(err) => Err(Errs::strs("json to object", err)),
        }
    }
}

pub trait JsonNew<T>: Sized {
    fn new(data: T) -> GeorgeResult<Self>;
    fn from(&mut self, data: T) -> GeorgeResult<()>;
}

pub trait JsonExec<Param> {
    /// 表示json中不存在`param`或者`param`的值为null
    fn has(&self, param: Param) -> bool;
    fn take_string(&mut self, param: Param) -> GeorgeResult<String>;
    fn take_u64(&mut self, param: Param) -> GeorgeResult<u64>;
    fn take_i64(&mut self, param: Param) -> GeorgeResult<i64>;
    fn take_f64(&mut self, param: Param) -> GeorgeResult<f64>;
    fn take_bool(&mut self, param: Param) -> GeorgeResult<bool>;
    fn take_object(&mut self, param: Param) -> GeorgeResult<Json>;
    fn take_array(&mut self, param: Param) -> GeorgeResult<JsonArray>;
    fn is_string(&self, param: Param) -> bool;
    fn is_u64(&self, param: Param) -> bool;
    fn is_i64(&self, param: Param) -> bool;
    fn is_f64(&self, param: Param) -> bool;
    fn is_bool(&self, param: Param) -> bool;
    fn is_object(&self, param: Param) -> bool;
    fn is_array(&self, param: Param) -> bool;
}

pub trait JsonGet<Param> {
    fn get_value(&self, param: Param) -> GeorgeResult<Value>;
    fn get_string(&self, param: Param) -> GeorgeResult<String>;
    fn get_u64(&self, param: Param) -> GeorgeResult<u64>;
    fn get_i64(&self, param: Param) -> GeorgeResult<i64>;
    fn get_f64(&self, param: Param) -> GeorgeResult<f64>;
    fn get_bool(&self, param: Param) -> GeorgeResult<bool>;
    fn get_object(&self, param: Param) -> GeorgeResult<Json>;
    fn get_array(&self, param: Param) -> GeorgeResult<JsonArray>;
}

impl JsonHandler for Json {
    fn object<Object>(object: &Object) -> GeorgeResult<Self>
    where
        Object: ?Sized + Serialize,
    {
        match serde_json::to_value(object) {
            Ok(res) => Ok(Json { value: res }),
            Err(err) => Err(Errs::strs("object to bytes", err)),
        }
    }

    fn value(&self) -> Value {
        self.value.clone()
    }
}

impl JsonNew<&[u8]> for Json {
    fn new(data: &[u8]) -> GeorgeResult<Self> {
        Ok(Json {
            value: from_slice(data)?,
        })
    }

    fn from(&mut self, data: &[u8]) -> GeorgeResult<()> {
        self.value = from_slice(data)?;
        Ok(())
    }
}

impl JsonNew<Vec<u8>> for Json {
    fn new(data: Vec<u8>) -> GeorgeResult<Self> {
        Ok(Json {
            value: from_slice(data.as_slice())?,
        })
    }

    fn from(&mut self, data: Vec<u8>) -> GeorgeResult<()> {
        self.value = from_slice(data.as_slice())?;
        Ok(())
    }
}

impl JsonNew<&str> for Json {
    fn new(data: &str) -> GeorgeResult<Self> {
        Ok(Json {
            value: from_string(data)?,
        })
    }

    fn from(&mut self, data: &str) -> GeorgeResult<()> {
        self.value = from_string(data)?;
        Ok(())
    }
}

impl JsonNew<String> for Json {
    fn new(data: String) -> GeorgeResult<Self> {
        Ok(Json {
            value: from_string(data.as_str())?,
        })
    }

    fn from(&mut self, data: String) -> GeorgeResult<()> {
        self.value = from_string(data.as_str())?;
        Ok(())
    }
}

impl JsonNew<Value> for Json {
    fn new(value: Value) -> GeorgeResult<Self> {
        Ok(Json { value })
    }

    fn from(&mut self, value: Value) -> GeorgeResult<()> {
        self.value.clone_from(&value);
        Ok(())
    }
}

impl JsonNew<&Value> for Json {
    fn new(value: &Value) -> GeorgeResult<Self> {
        Ok(Json {
            value: value.clone(),
        })
    }

    fn from(&mut self, value: &Value) -> GeorgeResult<()> {
        self.value.clone_from(value);
        Ok(())
    }
}

impl JsonExec<&str> for Json {
    fn has(&self, param: &str) -> bool {
        self.value[param] == Value::Null
    }

    fn take_string(&mut self, param: &str) -> GeorgeResult<String> {
        match self.value[param].take().as_str() {
            Some(res) => Ok(res.to_string()),
            None => Err(Errs::string(format!(
                "param {} not found or can not trans string!",
                param
            ))),
        }
    }

    fn take_u64(&mut self, param: &str) -> GeorgeResult<u64> {
        match self.value[param].take().as_u64() {
            Some(res) => Ok(res),
            None => Err(Errs::string(format!(
                "param {} not found or can not trans u64!",
                param
            ))),
        }
    }

    fn take_i64(&mut self, param: &str) -> GeorgeResult<i64> {
        match self.value[param].take().as_i64() {
            Some(res) => Ok(res),
            None => Err(Errs::string(format!(
                "param {} not found or can not trans i64!",
                param
            ))),
        }
    }

    fn take_f64(&mut self, param: &str) -> GeorgeResult<f64> {
        match self.value[param].take().as_f64() {
            Some(res) => Ok(res),
            None => Err(Errs::string(format!(
                "param {} not found or can not trans f64!",
                param
            ))),
        }
    }

    fn take_bool(&mut self, param: &str) -> GeorgeResult<bool> {
        match self.value[param].take().as_bool() {
            Some(res) => Ok(res),
            None => Err(Errs::string(format!(
                "param {} not found or can not trans bool!",
                param
            ))),
        }
    }

    fn take_object(&mut self, param: &str) -> GeorgeResult<Json> {
        Json::new(self.value[param].take())
    }

    fn take_array(&mut self, param: &str) -> GeorgeResult<JsonArray> {
        JsonArray::new(self.value[param].take())
    }

    fn is_string(&self, param: &str) -> bool {
        self.value[param].is_string()
    }

    fn is_u64(&self, param: &str) -> bool {
        self.value[param].is_u64()
    }

    fn is_i64(&self, param: &str) -> bool {
        self.value[param].is_i64()
    }

    fn is_f64(&self, param: &str) -> bool {
        self.value[param].is_f64()
    }

    fn is_bool(&self, param: &str) -> bool {
        self.value[param].is_boolean()
    }

    fn is_object(&self, param: &str) -> bool {
        self.value[param].is_object()
    }

    fn is_array(&self, param: &str) -> bool {
        self.value[param].is_array()
    }
}

impl JsonExec<String> for Json {
    fn has(&self, param: String) -> bool {
        self.value[param] != Value::Null
    }

    fn take_string(&mut self, param: String) -> GeorgeResult<String> {
        match self.value[param.clone()].take().as_str() {
            Some(res) => Ok(res.to_string()),
            None => Err(Errs::string(format!(
                "param {} not found or can not trans string!",
                param
            ))),
        }
    }

    fn take_u64(&mut self, param: String) -> GeorgeResult<u64> {
        match self.value[param.clone()].take().as_u64() {
            Some(res) => Ok(res),
            None => Err(Errs::string(format!(
                "param {} not found or can not trans u64!",
                param
            ))),
        }
    }

    fn take_i64(&mut self, param: String) -> GeorgeResult<i64> {
        match self.value[param.clone()].take().as_i64() {
            Some(res) => Ok(res),
            None => Err(Errs::string(format!(
                "param {} not found or can not trans i64!",
                param
            ))),
        }
    }

    fn take_f64(&mut self, param: String) -> GeorgeResult<f64> {
        match self.value[param.clone()].take().as_f64() {
            Some(res) => Ok(res),
            None => Err(Errs::string(format!(
                "param {} not found or can not trans f64!",
                param
            ))),
        }
    }

    fn take_bool(&mut self, param: String) -> GeorgeResult<bool> {
        match self.value[param.clone()].take().as_bool() {
            Some(res) => Ok(res),
            None => Err(Errs::string(format!(
                "param {} not found or can not trans bool!",
                param
            ))),
        }
    }

    fn take_object(&mut self, param: String) -> GeorgeResult<Json> {
        Json::new(self.value[param.clone()].take())
    }

    fn take_array(&mut self, param: String) -> GeorgeResult<JsonArray> {
        JsonArray::new(self.value[param.clone()].take())
    }

    fn is_string(&self, param: String) -> bool {
        self.value[param].is_string()
    }

    fn is_u64(&self, param: String) -> bool {
        self.value[param].is_u64()
    }

    fn is_i64(&self, param: String) -> bool {
        self.value[param].is_i64()
    }

    fn is_f64(&self, param: String) -> bool {
        self.value[param].is_f64()
    }

    fn is_bool(&self, param: String) -> bool {
        self.value[param].is_boolean()
    }

    fn is_object(&self, param: String) -> bool {
        self.value[param].is_object()
    }

    fn is_array(&self, param: String) -> bool {
        self.value[param].is_array()
    }
}

impl JsonGet<&str> for Json {
    fn get_value(&self, param: &str) -> GeorgeResult<Value> {
        Ok(self.value[param].clone())
    }

    fn get_string(&self, param: &str) -> GeorgeResult<String> {
        match self.value[param].as_str() {
            Some(res) => Ok(res.to_string()),
            None => Err(Errs::string(format!(
                "param {} not found or can not trans string!",
                param
            ))),
        }
    }

    fn get_u64(&self, param: &str) -> GeorgeResult<u64> {
        match self.value[param].as_u64() {
            Some(res) => Ok(res),
            None => Err(Errs::string(format!(
                "param {} not found or can not trans u64!",
                param
            ))),
        }
    }

    fn get_i64(&self, param: &str) -> GeorgeResult<i64> {
        match self.value[param].as_i64() {
            Some(res) => Ok(res),
            None => Err(Errs::string(format!(
                "param {} not found or can not trans i64!",
                param
            ))),
        }
    }

    fn get_f64(&self, param: &str) -> GeorgeResult<f64> {
        match self.value[param].as_f64() {
            Some(res) => Ok(res),
            None => Err(Errs::string(format!(
                "param {} not found or can not trans f64!",
                param
            ))),
        }
    }

    fn get_bool(&self, param: &str) -> GeorgeResult<bool> {
        match self.value[param].as_bool() {
            Some(res) => Ok(res),
            None => Err(Errs::string(format!(
                "param {} not found or can not trans bool!",
                param
            ))),
        }
    }

    fn get_object(&self, param: &str) -> GeorgeResult<Json> {
        match self.value.get(param) {
            Some(res) => Json::new(res),
            None => Err(Errs::string(format!(
                "param {} not found or can not trans object!",
                param
            ))),
        }
    }

    fn get_array(&self, param: &str) -> GeorgeResult<JsonArray> {
        match self.value.get(param) {
            Some(res) => JsonArray::new(res),
            None => Err(Errs::string(format!(
                "param {} not found or can not trans array!",
                param
            ))),
        }
    }
}

impl JsonGet<String> for Json {
    fn get_value(&self, param: String) -> GeorgeResult<Value> {
        Ok(self.value[param].clone())
    }

    fn get_string(&self, param: String) -> GeorgeResult<String> {
        match self.value[param.clone()].as_str() {
            Some(res) => Ok(res.to_string()),
            None => Err(Errs::string(format!(
                "param {} not found or can not trans string!",
                param
            ))),
        }
    }

    fn get_u64(&self, param: String) -> GeorgeResult<u64> {
        match self.value[param.clone()].as_u64() {
            Some(res) => Ok(res),
            None => Err(Errs::string(format!(
                "param {} not found or can not trans u64!",
                param
            ))),
        }
    }

    fn get_i64(&self, param: String) -> GeorgeResult<i64> {
        match self.value[param.clone()].as_i64() {
            Some(res) => Ok(res),
            None => Err(Errs::string(format!(
                "param {} not found or can not trans i64!",
                param
            ))),
        }
    }

    fn get_f64(&self, param: String) -> GeorgeResult<f64> {
        match self.value[param.clone()].as_f64() {
            Some(res) => Ok(res),
            None => Err(Errs::string(format!(
                "param {} not found or can not trans f64!",
                param
            ))),
        }
    }

    fn get_bool(&self, param: String) -> GeorgeResult<bool> {
        match self.value[param.clone()].as_bool() {
            Some(res) => Ok(res),
            None => Err(Errs::string(format!(
                "param {} not found or can not trans bool!",
                param
            ))),
        }
    }

    fn get_object(&self, param: String) -> GeorgeResult<Json> {
        match self.value.get(param.clone()) {
            Some(res) => Json::new(res),
            None => Err(Errs::string(format!(
                "param {} not found or can not trans object!",
                param
            ))),
        }
    }

    fn get_array(&self, param: String) -> GeorgeResult<JsonArray> {
        match self.value.get(param.clone()) {
            Some(res) => JsonArray::new(res),
            None => Err(Errs::string(format!(
                "param {} not found or can not trans array!",
                param
            ))),
        }
    }
}

impl JsonHandler for JsonArray {
    fn object<Object>(object: &Object) -> GeorgeResult<Self>
    where
        Object: ?Sized + Serialize,
    {
        match serde_json::to_value(object) {
            Ok(res) => Ok(JsonArray { value: res }),
            Err(err) => Err(Errs::strs("object to bytes", err)),
        }
    }

    fn value(&self) -> Value {
        self.value.clone()
    }
}

impl JsonNew<&[u8]> for JsonArray {
    fn new(data: &[u8]) -> GeorgeResult<Self> {
        Ok(JsonArray {
            value: from_slice(data)?,
        })
    }

    fn from(&mut self, data: &[u8]) -> GeorgeResult<()> {
        self.value = from_slice(data)?;
        Ok(())
    }
}

impl JsonNew<Vec<u8>> for JsonArray {
    fn new(data: Vec<u8>) -> GeorgeResult<Self> {
        Ok(JsonArray {
            value: from_slice(data.as_slice())?,
        })
    }

    fn from(&mut self, data: Vec<u8>) -> GeorgeResult<()> {
        self.value = from_slice(data.as_slice())?;
        Ok(())
    }
}

impl JsonNew<&str> for JsonArray {
    fn new(data: &str) -> GeorgeResult<Self> {
        Ok(JsonArray {
            value: from_string(data)?,
        })
    }

    fn from(&mut self, data: &str) -> GeorgeResult<()> {
        self.value = from_string(data)?;
        Ok(())
    }
}

impl JsonNew<String> for JsonArray {
    fn new(data: String) -> GeorgeResult<Self> {
        Ok(JsonArray {
            value: from_string(data.as_str())?,
        })
    }

    fn from(&mut self, data: String) -> GeorgeResult<()> {
        self.value = from_string(data.as_str())?;
        Ok(())
    }
}

impl JsonNew<Value> for JsonArray {
    fn new(value: Value) -> GeorgeResult<Self> {
        Ok(JsonArray { value })
    }

    fn from(&mut self, value: Value) -> GeorgeResult<()> {
        self.value.clone_from(&value);
        Ok(())
    }
}

impl JsonNew<&Value> for JsonArray {
    fn new(value: &Value) -> GeorgeResult<Self> {
        Ok(JsonArray {
            value: value.clone(),
        })
    }

    fn from(&mut self, value: &Value) -> GeorgeResult<()> {
        self.value.clone_from(value);
        Ok(())
    }
}

impl JsonGet<usize> for JsonArray {
    fn get_value(&self, index: usize) -> GeorgeResult<Value> {
        match self.value.get(index as usize) {
            Some(res) => Ok(res.clone()),
            None => Err(Errs::string(format!(
                "value can not get from json array while index is {}!",
                index
            ))),
        }
    }

    fn get_string(&self, index: usize) -> GeorgeResult<String> {
        match self.value.get(index) {
            Some(res) => match res.as_str() {
                Some(res) => Ok(res.to_string()),
                None => Err(Errs::string(format!(
                    "value can not get from json array while index is {}!",
                    index
                ))),
            },
            None => Err(Errs::str("index out of bound while json array get string!")),
        }
    }

    fn get_u64(&self, index: usize) -> GeorgeResult<u64> {
        match self.value.get(index) {
            Some(res) => match res.as_u64() {
                Some(res) => Ok(res),
                None => Err(Errs::string(format!(
                    "value can not get from json array while index is {}!",
                    index
                ))),
            },
            None => Err(Errs::str("index out of bound while json array get u64!")),
        }
    }

    fn get_i64(&self, index: usize) -> GeorgeResult<i64> {
        match self.value.get(index) {
            Some(res) => match res.as_i64() {
                Some(res) => Ok(res),
                None => Err(Errs::string(format!(
                    "value can not get from json array while index is {}!",
                    index
                ))),
            },
            None => Err(Errs::str("index out of bound while json array get i64!")),
        }
    }

    fn get_f64(&self, index: usize) -> GeorgeResult<f64> {
        match self.value.get(index) {
            Some(res) => match res.as_f64() {
                Some(res) => Ok(res),
                None => Err(Errs::string(format!(
                    "value can not get from json array while index is {}!",
                    index
                ))),
            },
            None => Err(Errs::str("index out of bound while json array get f64!")),
        }
    }

    fn get_bool(&self, index: usize) -> GeorgeResult<bool> {
        match self.value.get(index) {
            Some(res) => match res.as_bool() {
                Some(res) => Ok(res),
                None => Err(Errs::string(format!(
                    "value can not get from json array while index is {}!",
                    index
                ))),
            },
            None => Err(Errs::str("index out of bound while json array get bool!")),
        }
    }

    fn get_object(&self, index: usize) -> GeorgeResult<Json> {
        match self.value.get(index) {
            Some(res) => Json::new(res),
            None => Err(Errs::string(format!(
                "value can not get from json array while index is {}!",
                index
            ))),
        }
    }

    fn get_array(&self, index: usize) -> GeorgeResult<JsonArray> {
        match self.value.get(index) {
            Some(res) => JsonArray::new(res),
            None => Err(Errs::string(format!(
                "value can not get from json array while index is {}!",
                index
            ))),
        }
    }
}

impl JsonGet<i32> for JsonArray {
    fn get_value(&self, index: i32) -> GeorgeResult<Value> {
        match self.value.get(index as usize) {
            Some(res) => Ok(res.clone()),
            None => Err(Errs::string(format!(
                "value can not get from json array while index is {}!",
                index
            ))),
        }
    }

    fn get_string(&self, index: i32) -> GeorgeResult<String> {
        match self.value.get(index as usize) {
            Some(res) => match res.as_str() {
                Some(res) => Ok(res.to_string()),
                None => Err(Errs::string(format!(
                    "value can not get from json array while index is {}!",
                    index
                ))),
            },
            None => Err(Errs::str("index out of bound while json array get string!")),
        }
    }

    fn get_u64(&self, index: i32) -> GeorgeResult<u64> {
        match self.value.get(index as usize) {
            Some(res) => match res.as_u64() {
                Some(res) => Ok(res),
                None => Err(Errs::string(format!(
                    "value can not get from json array while index is {}!",
                    index
                ))),
            },
            None => Err(Errs::str("index out of bound while json array get u64!")),
        }
    }

    fn get_i64(&self, index: i32) -> GeorgeResult<i64> {
        match self.value.get(index as usize) {
            Some(res) => match res.as_i64() {
                Some(res) => Ok(res),
                None => Err(Errs::string(format!(
                    "value can not get from json array while index is {}!",
                    index
                ))),
            },
            None => Err(Errs::str("index out of bound while json array get i64!")),
        }
    }

    fn get_f64(&self, index: i32) -> GeorgeResult<f64> {
        match self.value.get(index as usize) {
            Some(res) => match res.as_f64() {
                Some(res) => Ok(res),
                None => Err(Errs::string(format!(
                    "value can not get from json array while index is {}!",
                    index
                ))),
            },
            None => Err(Errs::str("index out of bound while json array get f64!")),
        }
    }

    fn get_bool(&self, index: i32) -> GeorgeResult<bool> {
        match self.value.get(index as usize) {
            Some(res) => match res.as_bool() {
                Some(res) => Ok(res),
                None => Err(Errs::string(format!(
                    "value can not get from json array while index is {}!",
                    index
                ))),
            },
            None => Err(Errs::str("index out of bound while json array get bool!")),
        }
    }

    fn get_object(&self, index: i32) -> GeorgeResult<Json> {
        match self.value.get(index as usize) {
            Some(res) => Json::new(res),
            None => Err(Errs::str("index out of bound while json array get string!")),
        }
    }

    fn get_array(&self, index: i32) -> GeorgeResult<JsonArray> {
        match self.value.get(index as usize) {
            Some(res) => JsonArray::new(res),
            None => Err(Errs::str("index out of bound while json array get string!")),
        }
    }
}

impl JsonGet<u32> for JsonArray {
    fn get_value(&self, index: u32) -> GeorgeResult<Value> {
        match self.value.get(index as usize) {
            Some(res) => Ok(res.clone()),
            None => Err(Errs::string(format!(
                "value can not get from json array while index is {}!",
                index
            ))),
        }
    }

    fn get_string(&self, index: u32) -> GeorgeResult<String> {
        match self.value.get(index as usize) {
            Some(res) => match res.as_str() {
                Some(res) => Ok(res.to_string()),
                None => Err(Errs::string(format!(
                    "value can not get from json array while index is {}!",
                    index
                ))),
            },
            None => Err(Errs::str("index out of bound while json array get string!")),
        }
    }

    fn get_u64(&self, index: u32) -> GeorgeResult<u64> {
        match self.value.get(index as usize) {
            Some(res) => match res.as_u64() {
                Some(res) => Ok(res),
                None => Err(Errs::string(format!(
                    "value can not get from json array while index is {}!",
                    index
                ))),
            },
            None => Err(Errs::str("index out of bound while json array get u64!")),
        }
    }

    fn get_i64(&self, index: u32) -> GeorgeResult<i64> {
        match self.value.get(index as usize) {
            Some(res) => match res.as_i64() {
                Some(res) => Ok(res),
                None => Err(Errs::string(format!(
                    "value can not get from json array while index is {}!",
                    index
                ))),
            },
            None => Err(Errs::str("index out of bound while json array get i64!")),
        }
    }

    fn get_f64(&self, index: u32) -> GeorgeResult<f64> {
        match self.value.get(index as usize) {
            Some(res) => match res.as_f64() {
                Some(res) => Ok(res),
                None => Err(Errs::string(format!(
                    "value can not get from json array while index is {}!",
                    index
                ))),
            },
            None => Err(Errs::str("index out of bound while json array get f64!")),
        }
    }

    fn get_bool(&self, index: u32) -> GeorgeResult<bool> {
        match self.value.get(index as usize) {
            Some(res) => match res.as_bool() {
                Some(res) => Ok(res),
                None => Err(Errs::string(format!(
                    "value can not get from json array while index is {}!",
                    index
                ))),
            },
            None => Err(Errs::str("index out of bound while json array get bool!")),
        }
    }

    fn get_object(&self, index: u32) -> GeorgeResult<Json> {
        match self.value.get(index as usize) {
            Some(res) => Json::new(res),
            None => Err(Errs::str("index out of bound while json array get string!")),
        }
    }

    fn get_array(&self, index: u32) -> GeorgeResult<JsonArray> {
        match self.value.get(index as usize) {
            Some(res) => JsonArray::new(res),
            None => Err(Errs::str("index out of bound while json array get string!")),
        }
    }
}

impl JsonGet<u64> for JsonArray {
    fn get_value(&self, index: u64) -> GeorgeResult<Value> {
        match self.value.get(index as usize) {
            Some(res) => Ok(res.clone()),
            None => Err(Errs::string(format!(
                "value can not get from json array while index is {}!",
                index
            ))),
        }
    }

    fn get_string(&self, index: u64) -> GeorgeResult<String> {
        match self.value.get(index as usize) {
            Some(res) => match res.as_str() {
                Some(res) => Ok(res.to_string()),
                None => Err(Errs::string(format!(
                    "value can not get from json array while index is {}!",
                    index
                ))),
            },
            None => Err(Errs::str("index out of bound while json array get string!")),
        }
    }

    fn get_u64(&self, index: u64) -> GeorgeResult<u64> {
        match self.value.get(index as usize) {
            Some(res) => match res.as_u64() {
                Some(res) => Ok(res),
                None => Err(Errs::string(format!(
                    "value can not get from json array while index is {}!",
                    index
                ))),
            },
            None => Err(Errs::str("index out of bound while json array get u64!")),
        }
    }

    fn get_i64(&self, index: u64) -> GeorgeResult<i64> {
        match self.value.get(index as usize) {
            Some(res) => match res.as_i64() {
                Some(res) => Ok(res),
                None => Err(Errs::string(format!(
                    "value can not get from json array while index is {}!",
                    index
                ))),
            },
            None => Err(Errs::str("index out of bound while json array get i64!")),
        }
    }

    fn get_f64(&self, index: u64) -> GeorgeResult<f64> {
        match self.value.get(index as usize) {
            Some(res) => match res.as_f64() {
                Some(res) => Ok(res),
                None => Err(Errs::string(format!(
                    "value can not get from json array while index is {}!",
                    index
                ))),
            },
            None => Err(Errs::str("index out of bound while json array get f64!")),
        }
    }

    fn get_bool(&self, index: u64) -> GeorgeResult<bool> {
        match self.value.get(index as usize) {
            Some(res) => match res.as_bool() {
                Some(res) => Ok(res),
                None => Err(Errs::string(format!(
                    "value can not get from json array while index is {}!",
                    index
                ))),
            },
            None => Err(Errs::str("index out of bound while json array get bool!")),
        }
    }

    fn get_object(&self, index: u64) -> GeorgeResult<Json> {
        match self.value.get(index as usize) {
            Some(res) => Json::new(res),
            None => Err(Errs::str("index out of bound while json array get bool!")),
        }
    }

    fn get_array(&self, index: u64) -> GeorgeResult<JsonArray> {
        match self.value.get(index as usize) {
            Some(res) => JsonArray::new(res),
            None => Err(Errs::str("index out of bound while json array get bool!")),
        }
    }
}

impl Default for Json {
    fn default() -> Json {
        Json {
            value: Default::default(),
        }
    }
}

impl Default for JsonArray {
    fn default() -> JsonArray {
        JsonArray {
            value: Default::default(),
        }
    }
}

fn from_slice(data: &[u8]) -> GeorgeResult<Value> {
    // let map:Map<String, Value> = serde_json::from_slice(data).unwrap();
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
