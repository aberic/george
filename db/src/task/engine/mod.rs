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
use crate::utils::comm::is_bytes_fill;
use comm::errors::entrances::{err_strs, GeorgeResult};
use comm::io::file::{Filer, FilerWriter};
use comm::trans::{trans_bytes_2_u16, trans_bytes_2_u48};
use comm::vectors::{Vector, VectorHandler};
use serde::{Deserialize, Serialize};

pub(super) mod block;
pub(super) mod dossier;
pub(super) mod library;
pub(super) mod memory;
pub mod traits;

/// 检查值有效性
fn check(
    database_name: String,
    view_name: String,
    node_filepath: String,
    key: u64,
    conditions: Vec<Condition>,
    delete: bool,
    res: Vec<u8>,
) -> GeorgeResult<(bool, Vec<u8>)> {
    if is_bytes_fill(res.clone()) {
        let version = trans_bytes_2_u16(Vector::sub(res.clone(), 0, 2)?)?;
        let seek = trans_bytes_2_u48(Vector::sub(res, 2, 8)?)?;
        let value_bytes = GLOBAL_MASTER.read_content_by(database_name, view_name, version, seek)?;
        if Condition::validate(conditions.clone(), value_bytes.clone()) {
            if delete {
                Filer::write_seek(node_filepath, key, Vector::create_empty_bytes(8))?;
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
#[derive(Serialize, Deserialize)]
pub struct DataReal {
    key: String,
    value: Vec<u8>,
}

impl DataReal {
    pub(crate) fn bytes(key: String, value: Vec<u8>) -> GeorgeResult<Vec<u8>> {
        match serde_json::to_vec(&DataReal { key, value }) {
            Ok(v8s) => Ok(v8s),
            Err(err) => Err(err_strs("data real 2 bytes", err)),
        }
    }
    pub(crate) fn value_bytes(real_bytes: Vec<u8>) -> GeorgeResult<Vec<u8>> {
        Ok(DataReal::from(real_bytes)?.value)
    }
    fn from(real_bytes: Vec<u8>) -> GeorgeResult<DataReal> {
        match serde_json::from_slice(real_bytes.as_slice()) {
            Ok(dr) => Ok(dr),
            Err(err) => Err(err_strs("data real from u8s", err)),
        }
    }
}
