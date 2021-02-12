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

use crate::task::engine::traits::TSeed;
use crate::task::seed::IndexPolicy;
use crate::utils::comm::level_distance_64;
use crate::utils::path::{index_path, node_file_path};
use comm::errors::entrances::GeorgeResult;
use comm::io::file::{Filer, FilerReader};
use comm::strings::{StringHandler, Strings};
use comm::vectors::{Vector, VectorHandler};
use std::ops::Add;

/// 索引B+Tree结点结构
///
/// 包含了索引的根结点、子结点以及叶子结点
///
/// 叶子结点中才会存在Link，其余结点Link为None
#[derive(Debug, Clone)]
pub(crate) struct Node {
    /// 存储结点所属各子结点坐标顺序字符串
    ///
    /// 子项是64位node集合，在node集合中每一个node的默认字节长度是8，数量是524288，即一次性读取524288个字节
    node_bytes: Arc<RwLock<Vec<u8>>>,
}

impl Node {
    /// 新建根结点
    ///
    /// 该结点没有Links，也没有preNode，是B+Tree的创世结点
    pub fn create_root() -> Arc<RwLock<Self>> {
        return Arc::new(RwLock::new(Node {
            node_bytes: Arc::new(RwLock::new(Vector::create_empty_bytes(524288))),
        }));
    }
    /// 恢复根结点
    pub fn recovery_root(v8s: Vec<u8>) -> Arc<RwLock<Self>> {
        return Arc::new(RwLock::new(Node {
            node_bytes: Arc::new(RwLock::new(v8s)),
        }));
    }
}

/// 封装方法函数
impl Node {
    /// 存储结点所属各子结点坐标顺序字符串
    ///
    /// 如果子项是node集合，在node集合中每一个node的默认字节长度是8，数量是65536，即一次性读取524288个字节
    pub(crate) fn node_bytes(&self) -> Arc<RwLock<Vec<u8>>> {
        self.node_bytes.clone()
    }
    /// 插入数据<p><p>
    ///
    /// ###Params
    ///
    /// original_key 使用当前索引的原始key
    ///
    /// key u64
    ///
    /// ###Return
    ///
    /// EngineResult<()>
    pub(crate) fn put(
        &self,
        original_key: String,
        database_name: String,
        view_name: String,
        index_name: String,
        key: u64,
        seed: Arc<RwLock<dyn TSeed>>,
    ) -> GeorgeResult<()> {
        let index_path = index_path(database_name, view_name, index_name);
        self.put_in_node(original_key, index_path, String::from(""), 1, key, seed)
    }
    pub(crate) fn get(
        &self,
        database_name: String,
        view_name: String,
        index_name: String,
        key: u64,
    ) -> GeorgeResult<Vec<u8>> {
        let index_path = index_path(database_name, view_name, index_name);
        self.get_in_node(index_path, String::from(""), 1, key)
    }
}

impl Node {
    /// 存储数据真实操作
    ///
    /// key 使用当前索引的原始key
    ///
    /// node_bytes 当前操作结点的字节数组
    ///
    /// level 当前操作结点层
    ///
    /// flexible_key 下一级最左最小树所对应真实key
    ///
    /// Seed value信息
    ///
    /// root 是否根结点
    ///
    /// node_seek 当前操作结点在文件中的真实起始位置
    fn put_in_node(
        &self,
        key: String,
        index_path: String,
        mut index_file_name: String,
        level: u8,
        flexible_key: u64,
        seed: Arc<RwLock<dyn TSeed>>,
    ) -> GeorgeResult<()>
    where
        Self: Sized,
    {
        // 通过当前树下一层高获取结点间间隔数量，即每一度中存在的元素数量
        let distance = level_distance_64(level);
        // 通过当前层真实key除以下一层间隔数获取结点处在下一层的度数
        let next_degree = flexible_key / distance;
        // 如果当前层高为4，则达到最底层，否则递归下一层逻辑
        if level == 4 {
            let index_file_path = node_file_path(index_path, index_file_name);
            log::debug!(
                "node_file_path = {}, degree = {}",
                index_file_path,
                next_degree
            );
            seed.write()
                .unwrap()
                .modify(IndexPolicy::bytes(key, index_file_path, next_degree * 8)?)
        } else {
            index_file_name = index_file_name.add(&Strings::left_fits(
                next_degree.to_string(),
                "0".parse().unwrap(),
                5,
            ));
            // 通过当前层真实key减去下一层的度数与间隔数的乘机获取结点所在下一层的真实key
            let next_flexible_key = flexible_key - next_degree * distance;
            self.put_in_node(
                key,
                index_path,
                index_file_name,
                level + 1,
                next_flexible_key,
                seed,
            )
        }
    }
    fn get_in_node(
        &self,
        index_path: String,
        mut index_file_name: String,
        level: u8,
        flexible_key: u64,
    ) -> GeorgeResult<Vec<u8>> {
        // 通过当前树下一层高获取结点间间隔数量，即每一度中存在的元素数量
        let distance = level_distance_64(level);
        // 通过当前层真实key除以下一层间隔数获取结点处在下一层的度数
        let next_degree = flexible_key / distance;
        // 如果当前层高为4，则达到最底层，否则递归下一层逻辑
        if level == 4 {
            let index_file_path = node_file_path(index_path, index_file_name);
            log::debug!(
                "node_file_path = {}, degree = {}",
                index_file_path,
                next_degree
            );
            Filer::read_sub(index_file_path, next_degree, 8)
        } else {
            index_file_name = index_file_name.add(&Strings::left_fits(
                next_degree.to_string(),
                "0".parse().unwrap(),
                5,
            ));
            // 通过当前层真实key减去下一层的度数与间隔数的乘机获取结点所在下一层的真实key
            let next_flexible_key = flexible_key - next_degree * distance;
            self.get_in_node(index_path, index_file_name, level + 1, next_flexible_key)
        }
    }
}
