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

use comm::errors::{Errs, GeorgeResult};
use comm::json::JsonHandler;
use comm::vectors::VectorHandler;
use comm::Json;
use comm::Vector;
use serde::{Deserialize, Serialize};

pub(super) mod block;
pub(super) mod disk;
pub(super) mod increment;
pub(super) mod memory;
pub(super) mod sequence;
pub(super) mod traits;

/// 真实存储数据
///
/// 执行`put`、`set`及`insert`等方法插入数据时，存入文件中的真实数据为[序列号 + key + value]组合
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataReal {
    /// 自增ID
    pub(crate) increment: u64,
    pub(crate) key: String,
    pub(crate) value: Vec<u8>,
}

impl DataReal {
    pub(crate) fn key(&self) -> String {
        self.key.clone()
    }

    pub(crate) fn value(&self) -> Vec<u8> {
        self.value.clone()
    }

    pub(crate) fn values(&self) -> GeorgeResult<Vec<u8>> {
        Json::obj_2_bytes(&self)
    }

    pub(crate) fn set_seq(&mut self, sequence: u64) {
        self.increment = sequence
    }

    pub(crate) fn from(real_bytes: Vec<u8>) -> GeorgeResult<DataReal> {
        Json::bytes_2_obj(real_bytes.as_slice())
    }
}

/// 根结点所属各子结点坐标顺序字节数组
#[derive(Debug, Clone)]
pub struct RootBytes {
    /// 存储根结点所属各子结点坐标顺序字节数组
    ///
    /// 如果子项是32位node集合，在node集合中每一个node的默认字节长度是8，数量是256，即一次性读取2048个字节
    pub(crate) bytes: Vec<u8>,
}

impl RootBytes {
    pub(crate) fn create(len: usize) -> RootBytes {
        let bytes = Vector::create_empty_bytes(len);
        RootBytes { bytes }
    }

    pub(crate) fn recovery(bytes: Vec<u8>, len: usize) -> GeorgeResult<RootBytes> {
        let bytes_len = bytes.len();
        if bytes_len != len {
            Err(Errs::string(format!(
                "bytes len is {}, while expect {}",
                bytes_len, len
            )))
        } else {
            Ok(RootBytes { bytes })
        }
    }

    pub(crate) fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    /// 修改存储根结点所属各子结点坐标顺序字节数组
    ///
    /// * start 从该位置起进行修改
    /// * target 从`start`起将后续字节替换为target内容
    pub(crate) fn modify(&mut self, start: usize, target: Vec<u8>) {
        self.bytes = Vector::modify(self.bytes(), target, start)
    }
}
