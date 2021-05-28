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

use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

use comm::errors::GeorgeResult;
use comm::io::file::FilerWriter;
use comm::io::Filer;
use comm::vectors::VectorHandler;
use comm::Vector;

use crate::task::engine::traits::{TForm, TSeed};
use crate::task::engine::DataReal;
use crate::task::{Seed, View};
use crate::utils::enums::IndexType;

/// 待处理索引操作策略
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct IndexPolicy {
    index_type: IndexType,
    /// 使用当前索引的原始key
    original_key: String,
    /// 待处理索引文件路径
    node_filepath: String,
    /// 待写入索引内容起始偏移量
    seek: u64,
    /// 自行处理内容
    custom: Vec<u8>,
}

impl IndexPolicy {
    pub fn create(
        key: String,
        index_type: IndexType,
        node_filepath: String,
        seek: u64,
    ) -> IndexPolicy {
        IndexPolicy {
            index_type,
            original_key: key,
            node_filepath,
            seek,
            custom: vec![],
        }
    }

    pub fn create_custom(
        key: String,
        node_filepath: String,
        seek: u64,
        custom: Vec<u8>,
    ) -> IndexPolicy {
        IndexPolicy {
            index_type: IndexType::None,
            original_key: key,
            node_filepath,
            seek,
            custom,
        }
    }

    fn node_file_path(&self) -> String {
        self.node_filepath.clone()
    }
}

/// 封装方法函数
impl Seed {
    /// 新建seed
    pub fn create(view: View, key: String, value: Vec<u8>) -> Arc<RwLock<Seed>> {
        Arc::new(RwLock::new(Seed {
            real: DataReal {
                increment: 0,
                key,
                value,
            },
            policies: Vec::new(),
            view,
        }))
    }

    /// 新建seed
    pub fn create_cus(
        view: View,
        key: String,
        increment: u64,
        value: Vec<u8>,
    ) -> Arc<RwLock<Seed>> {
        Arc::new(RwLock::new(Seed {
            real: DataReal {
                increment,
                key,
                value,
            },
            policies: Vec::new(),
            view,
        }))
    }

    fn values(&self) -> GeorgeResult<Vec<u8>> {
        self.real.values()
    }
}

/// 封装方法函数
impl TSeed for Seed {
    fn key(&self) -> String {
        self.real.key()
    }

    fn value(&self) -> Vec<u8> {
        self.real.value()
    }

    fn increment(&self) -> u64 {
        self.real.increment
    }

    fn modify_4_put(&mut self, index_policy: IndexPolicy) {
        match index_policy.index_type {
            IndexType::Increment => self.real.set_seq(index_policy.seek / 8),
            _ => {}
        }
        self.policies.push(index_policy)
    }

    fn modify_4_del(&mut self, index_policy: IndexPolicy) {
        self.policies.push(index_policy)
    }

    fn save(&self) -> GeorgeResult<()> {
        if self.policies.len() == 0 {
            return Ok(());
        }
        let value = self.values()?;
        // view_info_index view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节)
        let view_info_index = self.view.write_content(value)?;

        // 将在数据在view中的坐标存入各个index
        for policy in self.policies.to_vec() {
            match policy.index_type {
                IndexType::None => {
                    Filer::write_seek(policy.node_file_path(), policy.seek, policy.custom)?
                }
                _ => Filer::write_seek(
                    policy.node_file_path(),
                    policy.seek,
                    view_info_index.clone(),
                )?,
            }
        }
        Ok(())
    }

    fn remove(&self) -> GeorgeResult<()> {
        // 将在数据在view中的空坐标存入各个index
        // 坐标内容由view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节)组成，因此是12个字节
        for policy in self.policies.to_vec() {
            match policy.index_type {
                IndexType::None => {
                    Filer::write_seek(policy.node_file_path(), policy.seek, policy.custom)?
                }
                _ => Filer::write_seek(
                    policy.node_file_path(),
                    policy.seek,
                    Vector::create_empty_bytes(12),
                )?,
            }
        }
        Ok(())
    }
}
