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

use comm::errors::{Errs, GeorgeResult};

use crate::task::engine::memory::Seed;

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
            None => Err(Errs::data_no_exist_error()),
        }
    }
    pub(crate) fn is_none(&self) -> bool {
        self.value.is_none()
    }
}
