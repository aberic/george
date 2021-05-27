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

use serde::{Deserialize, Serialize};

use comm::errors::entrances::GeorgeResult;
use comm::io::file::{Filer, FilerWriter};
use comm::trans::Trans;

use crate::task::engine::traits::TSeed;
use crate::task::engine::DataReal;
use crate::task::view::View;
use crate::utils::enums::IndexType;
use comm::vectors::{Vector, VectorHandler};
use std::sync::{Arc, RwLock};

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

/// B+Tree索引叶子结点内防hash碰撞数组结构中单体结构
///
/// 搭配Index使用
///
/// 叶子节点下真实存储数据的集合单体结构
#[derive(Debug)]
pub(crate) struct Seed {
    real: DataReal,
    /// 除主键索引外的其它索引操作策略集合
    policies: Vec<IndexPolicy>,
    view: View,
}

/// 封装方法函数
impl Seed {
    /// 新建seed
    pub fn create(view: View, key: String, value: Vec<u8>) -> Arc<RwLock<Seed>> {
        Arc::new(RwLock::new(Seed {
            real: DataReal {
                sequence: 0,
                key,
                value,
            },
            policies: Vec::new(),
            view,
        }))
    }

    /// 新建seed
    pub fn create_cus(view: View, key: String, sequence: u64, value: Vec<u8>) -> Arc<RwLock<Seed>> {
        Arc::new(RwLock::new(Seed {
            real: DataReal {
                sequence,
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

    fn sequence(&self) -> u64 {
        self.real.sequence
    }

    fn modify(&mut self, index_policy: IndexPolicy) {
        match index_policy.index_type {
            IndexType::Increment => self.real.set_seq(index_policy.seek / 8),
            IndexType::Sequence => self.real.set_seq(index_policy.seek / 8),
            _ => {}
        }
        self.policies.push(index_policy)
    }

    fn save(&self) -> GeorgeResult<()> {
        if self.policies.len() == 0 {
            return Ok(());
        }
        let value = self.values()?;
        // 内容持续长度(4字节)
        let mut seed_bytes_len_bytes = Trans::u32_2_bytes(value.len() as u32);
        // 执行真实存储操作，即索引将seed存入后，允许检索到该结果，但该结果值不存在，仅当所有索引存入都成功，才会执行本方法完成真实存储操作
        let view_seek_start = self.view.write_content(value)?;
        // 记录视图文件属性(版本号/数据归档/定位文件用2字节)+数据在表文件中起始偏移量p(6字节)
        // 数据在视图文件中起始偏移量p(6字节)
        let mut view_seek_start_bytes = Trans::u48_2_bytes(view_seek_start);
        // 生成视图文件属性，版本号(2字节)
        let view_version_bytes = Trans::u16_2_bytes(self.view.version());
        // 循环定位记录使用文件属性
        let mut view_info_index = view_version_bytes.clone();
        // 记录表文件属性(版本/数据归档/定位文件用2字节)+数据持续长度+数据在表文件中起始偏移量p(6字节)
        // view_info_index = view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节)
        view_info_index.append(&mut seed_bytes_len_bytes);
        view_info_index.append(&mut view_seek_start_bytes);

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
