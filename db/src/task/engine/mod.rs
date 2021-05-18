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

use crate::task::master::GLOBAL_MASTER;
use crate::task::rich::Condition;
use crate::task::view::View;
use comm::errors::entrances::{err_string, err_strs, GeorgeResult};
use comm::io::file::{Filer, FilerWriter};
use comm::trans::{trans_bytes_2_u16, trans_bytes_2_u32, trans_bytes_2_u48};
use comm::vectors::{Vector, VectorHandler};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

pub(super) mod block;
pub(super) mod dossier;
pub(super) mod library;
pub(super) mod memory;
pub(super) mod sequence;
pub(super) mod traits;

/// 检查值有效性
fn check(
    index_name: String,
    view: Arc<RwLock<View>>,
    node_filepath: String,
    seek: u64,
    conditions: Vec<Condition>,
    delete: bool,
    view_info_index: Vec<u8>,
) -> GeorgeResult<(bool, Vec<u8>)> {
    if Vector::is_empty(view_info_index.clone()) {
        Ok((false, vec![]))
    } else {
        let v_r = view.read().unwrap();
        // 读取view版本号(2字节)
        let view_version = trans_bytes_2_u16(Vector::sub(view_info_index.clone(), 0, 2)?)?;
        // 读取view长度(4字节)
        let view_data_len = trans_bytes_2_u32(Vector::sub(view_info_index.clone(), 2, 6)?)?;
        // 读取view偏移量(6字节)
        let view_data_seek = trans_bytes_2_u48(Vector::sub(view_info_index.clone(), 6, 12)?)?;
        let real =
            DataReal::from(v_r.read_content_by(view_version, view_data_len, view_data_seek)?)?;
        let value_bytes = real.value();
        if Condition::validate(conditions.clone(), value_bytes.clone()) {
            if delete {
                v_r.remove(index_name, real.key(), real.value())?;
                Filer::write_seek(node_filepath, seek, Vector::create_empty_bytes(8))?
            }
            Ok((true, value_bytes))
        } else {
            Ok((false, vec![]))
        }
    }
}

/// 真实存储数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataReal {
    pub(crate) sequence: u64,
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
        match serde_json::to_vec(&self) {
            Ok(v8s) => Ok(v8s),
            Err(err) => Err(err_strs("data real 2 bytes", err)),
        }
    }
    pub(crate) fn set_seq(&mut self, sequence: u64) {
        self.sequence = sequence
    }
    pub(crate) fn value_bytes(real_bytes: Vec<u8>) -> GeorgeResult<Vec<u8>> {
        Ok(DataReal::from(real_bytes)?.value)
    }
    pub(crate) fn froms(
        database_name: String,
        view_name: String,
        view_info_index: Vec<u8>,
    ) -> GeorgeResult<DataReal> {
        DataReal::from(GLOBAL_MASTER.read_content_by(database_name, view_name, view_info_index)?)
    }
    fn from(real_bytes: Vec<u8>) -> GeorgeResult<DataReal> {
        match serde_json::from_slice(real_bytes.as_slice()) {
            Ok(dr) => Ok(dr),
            Err(err) => Err(err_strs("data real from u8s", err)),
        }
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
            Err(err_string(format!(
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
    pub(crate) fn modify(&mut self, source: Vec<u8>, target: Vec<u8>, start: usize) {
        self.bytes = Vector::modify(source, target, start)
    }
}
