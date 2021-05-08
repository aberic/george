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

use comm::errors::children::DataNoExistError;
use comm::errors::entrances::{GeorgeError, GeorgeResult};

/// B+Tree索引叶子结点内防hash碰撞数组结构中单体结构
///
/// 搭配Index使用
///
/// 叶子节点下真实存储数据的集合单体结构
#[derive(Debug)]
pub(crate) struct Seed {
    /// 当前结果原始key信息
    key: String,
    value: Option<Vec<u8>>,
    old_value: Option<Vec<u8>>,
}

/// 封装方法函数
impl Seed {
    /// 新建seed
    pub(crate) fn create(key: String, value: Vec<u8>) -> Seed {
        return Seed {
            key,
            value: Some(value),
            old_value: None,
        };
    }
}

/// 封装方法函数
impl Seed {
    pub(crate) fn key(&self) -> String {
        self.key.clone()
    }
    pub(crate) fn value(&self) -> GeorgeResult<Vec<u8>> {
        match self.value.clone() {
            Some(v) => Ok(v),
            None => Err(GeorgeError::from(DataNoExistError)),
        }
    }
    pub(crate) fn is_none(&self) -> bool {
        self.value.is_none()
    }
}
