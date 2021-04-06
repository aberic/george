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
use crate::utils::comm::is_bytes_fill;
use comm::errors::entrances::{err_strs, GeorgeResult};
use comm::io::file::{Filer, FilerWriter};
use comm::vectors::{Vector, VectorHandler};
use serde::{Deserialize, Serialize};

pub(super) mod block;
pub(super) mod dossier;
pub(super) mod library;
pub(super) mod memory;
pub mod traits;

/// 检查值有效性
fn check(
    view: View,
    node_filepath: String,
    seek: u64,
    conditions: Vec<Condition>,
    delete: bool,
    view_info_index: Vec<u8>,
) -> GeorgeResult<(bool, Vec<u8>)> {
    if is_bytes_fill(view_info_index.clone()) {
        let value_bytes = DataReal::value_bytes(view.read_content_by(view_info_index)?)?;
        if Condition::validate(conditions.clone(), value_bytes.clone()) {
            if delete {
                Filer::write_seek(node_filepath, seek, Vector::create_empty_bytes(8))?;
            }
            Ok((true, value_bytes))
        } else {
            Ok((false, vec![]))
        }
    } else {
        Ok((false, vec![]))
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
