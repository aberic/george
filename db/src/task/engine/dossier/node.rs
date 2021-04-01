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

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

use comm::errors::entrances::{err_str, GeorgeError, GeorgeResult};
use comm::io::file::{Filer, FilerReader};

use crate::task::engine::check;
use crate::task::engine::traits::{TNode, TSeed};
use crate::task::rich::Condition;
use crate::task::seed::IndexPolicy;
use crate::task::view::View;
use crate::utils::comm::is_bytes_fill;
use crate::utils::enums::IndexType;
use crate::utils::path::{index_path, node_filepath};
use crate::utils::writer::Filed;
use comm::errors::children::DataNoExistError;
use comm::vectors::{Vector, VectorHandler};

/// 索引B+Tree结点结构
///
/// 包含了索引的根结点、子结点以及叶子结点
///
/// 叶子结点中才会存在Link，其余结点Link为None
#[derive(Debug, Clone)]
pub(crate) struct Node {
    view: View,
    atomic_key: Arc<AtomicU64>,
    index_name: String,
    node_filepath: String,
    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 需要借助对象包裹，以便更新file，避免self为mut
    filer: Filed,
}

impl Node {
    /// 新建根结点
    ///
    /// 该结点没有Links，也没有preNode，是B+Tree的创世结点
    pub fn create(view: View, index_name: String) -> GeorgeResult<Arc<RwLock<Self>>> {
        let atomic_key = Arc::new(AtomicU64::new(1));
        let index_path = index_path(view.database_name(), view.name(), index_name.clone());
        let node_filepath = node_filepath(index_path, String::from("increment"));
        let filer = Filed::create(node_filepath.clone())?;
        filer.append(Vector::create_empty_bytes(8))?;
        Ok(Arc::new(RwLock::new(Node {
            view,
            atomic_key,
            index_name,
            node_filepath,
            filer,
        })))
    }
    /// 恢复根结点
    pub fn recovery(view: View, index_name: String) -> GeorgeResult<Arc<RwLock<Self>>> {
        let index_path = index_path(view.database_name(), view.name(), index_name.clone());
        let node_filepath = node_filepath(index_path, String::from("increment"));
        let file_len = Filer::len(node_filepath.clone())?;
        let last_key = file_len / 8;
        // log::debug!("atomic_key_u32 = {}", atomic_key_u32);
        let atomic_key = Arc::new(AtomicU64::new(last_key));
        let filer = Filed::recovery(node_filepath.clone())?;
        Ok(Arc::new(RwLock::new(Node {
            view,
            atomic_key,
            index_name,
            node_filepath,
            filer,
        })))
    }
    fn index_name(&self) -> String {
        self.index_name.clone()
    }
    fn database_name(&self) -> String {
        self.view.database_name()
    }
    fn view_name(&self) -> String {
        self.view.name()
    }
    fn node_filepath(&self) -> String {
        self.node_filepath.clone()
    }
    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
    ///
    /// #Return
    ///
    /// seek_end_before 写之前文件字节数据长度
    fn append(&self, content: Vec<u8>) -> GeorgeResult<u64> {
        self.filer.clone().append(content)
    }
    fn read(&self, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
        self.filer.clone().read(start, last)
    }
    fn write(&self, seek: u64, content: Vec<u8>) -> GeorgeResult<()> {
        self.filer.clone().write(seek, content)
    }
}

/// 封装方法函数
impl TNode for Node {
    fn modify(&mut self) -> GeorgeResult<()> {
        let index_path = index_path(self.database_name(), self.view_name(), self.index_name());
        self.node_filepath = node_filepath(index_path, String::from("increment"));
        Ok(())
    }
    /// 插入数据<p><p>
    ///
    /// ###Params
    ///
    /// hash_key u64
    ///
    /// ###Return
    ///
    /// EngineResult<()>
    fn put(
        &self,
        key: String,
        mut hash_key: u64,
        seed: Arc<RwLock<dyn TSeed>>,
        force: bool,
    ) -> GeorgeResult<()> {
        if hash_key == 0 {
            hash_key = self.atomic_key.fetch_add(1, Ordering::Relaxed);
        }
        self.put_in_node(key, hash_key, seed, force)
    }
    fn get(&self, _key: String, hash_key: u64) -> GeorgeResult<Vec<u8>> {
        self.get_in_node(hash_key)
    }
    fn del(&self, _key: String, hash_key: u64) -> GeorgeResult<()> {
        self.del_in_node(hash_key)
    }
    fn select(
        &self,
        left: bool,
        start: u64,
        end: u64,
        skip: u64,
        limit: u64,
        delete: bool,
        conditions: Vec<Condition>,
    ) -> GeorgeResult<(u64, u64, Vec<Vec<u8>>)> {
        if left {
            self.left_query(start, end, conditions, skip, limit, delete)
        } else {
            self.right_query(start, end, conditions, skip, limit, delete)
        }
    }
}

impl Node {
    /// 存储数据真实操作
    ///
    /// auto_increment_key 自增key
    ///
    /// seed value信息
    ///
    /// custom 是否用户自定义传入key
    fn put_in_node(
        &self,
        key: String,
        hash_key: u64,
        seed: Arc<RwLock<dyn TSeed>>,
        force: bool,
    ) -> GeorgeResult<()>
    where
        Self: Sized,
    {
        let seek = hash_key * 8;
        if !force {
            let res = self.read(seek, 8)?;
            if is_bytes_fill(res) {
                return Err(err_str("auto increment key has been used"));
            }
        }
        seed.write().unwrap().modify(IndexPolicy::create(
            key,
            IndexType::Dossier,
            self.node_filepath(),
            seek,
        ));
        Ok(())
    }
    fn get_in_node(&self, hash_key: u64) -> GeorgeResult<Vec<u8>> {
        let seek = hash_key * 8;
        let res = self.read(seek, 8)?;
        return if is_bytes_fill(res.clone()) {
            Ok(res)
        } else {
            Err(GeorgeError::from(DataNoExistError))
        };
    }
    fn del_in_node(&self, hash_key: u64) -> GeorgeResult<()> {
        let seek = hash_key * 8;
        self.write(seek, Vector::create_empty_bytes(8))?;
        Ok(())
    }
    /// 通过左查询约束获取数据集
    ///
    /// ###Params
    ///
    /// node_bytes 当前操作结点的字节数组
    ///
    /// conditions 条件集合
    ///
    /// skip 结果集跳过数量
    ///
    /// limit 结果集限制数量
    ///
    /// delete 是否删除检索结果
    ///
    /// ###Return
    ///
    /// total 检索过程中遍历的总条数（也表示文件读取次数，文件描述符次数远小于该数，一般文件描述符数为1，即共用同一文件描述符）
    ///
    /// count 检索结果过程中遍历的总条数
    ///
    /// values 检索结果集合
    fn left_query(
        &self,
        start: u64,
        end: u64,
        conditions: Vec<Condition>,
        mut skip: u64,
        mut limit: u64,
        delete: bool,
    ) -> GeorgeResult<(u64, u64, Vec<Vec<u8>>)> {
        let mut total: u64 = 0;
        let mut count: u64 = 0;
        let mut values: Vec<Vec<u8>> = vec![];

        let mut key_start = start * 8;
        let mut key_end: u64;
        if end == 0 {
            key_end = (self.atomic_key.load(Ordering::Relaxed) - 1) * 8;
        } else {
            key_end = end * 8;
        }
        loop {
            if limit <= 0 || key_start > key_end {
                break;
            }
            let res = self.read(key_start, 8)?;
            let (valid, value_bytes) = check(
                self.view.clone(),
                self.node_filepath(),
                key_end,
                conditions.clone(),
                delete,
                res,
            )?;
            if valid {
                if skip <= 0 {
                    limit -= 1;
                    count += 1;
                    values.push(value_bytes)
                } else {
                    skip -= 1;
                }
            }
            total += 1;
            key_start += 8;
        }
        Ok((total, count, values))
    }
    /// 通过右查询约束获取数据集
    ///
    /// ###Params
    ///
    /// node_bytes 当前操作结点的字节数组
    ///
    /// conditions 条件集合
    ///
    /// skip 结果集跳过数量
    ///
    /// limit 结果集限制数量
    ///
    /// delete 是否删除检索结果
    ///
    /// ###Return
    ///
    /// total 检索过程中遍历的总条数（也表示文件读取次数，文件描述符次数远小于该数，一般文件描述符数为1，即共用同一文件描述符）
    ///
    /// count 检索结果过程中遍历的总条数
    ///
    /// values 检索结果集合
    fn right_query(
        &self,
        start: u64,
        end: u64,
        conditions: Vec<Condition>,
        mut skip: u64,
        mut limit: u64,
        delete: bool,
    ) -> GeorgeResult<(u64, u64, Vec<Vec<u8>>)> {
        let mut total: u64 = 0;
        let mut count: u64 = 0;
        let mut values: Vec<Vec<u8>> = vec![];

        let mut key_start = start * 8;
        let mut key_end: u64;
        if end == 0 {
            key_end = (self.atomic_key.load(Ordering::Relaxed) - 1) * 8;
        } else {
            key_end = end * 8;
        }
        loop {
            if limit <= 0 || key_start > key_end {
                break;
            }
            let res = self.read(key_end, 8)?;
            let (valid, value_bytes) = check(
                self.view.clone(),
                self.node_filepath(),
                key_end,
                conditions.clone(),
                delete,
                res,
            )?;
            if valid {
                if skip <= 0 {
                    limit -= 1;
                    count += 1;
                    values.push(value_bytes)
                } else {
                    skip -= 1;
                }
            }
            total += 1;
            key_end -= 8;
        }
        Ok((total, count, values))
    }
}
