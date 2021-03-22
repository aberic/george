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

use std::io::{Seek, SeekFrom};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

use comm::errors::entrances::{err_str, GeorgeError, GeorgeResult};
use comm::io::file::{Filer, FilerHandler, FilerNormal, FilerReader, FilerWriter};

use crate::task::engine::traits::{TNode, TSeed};
use crate::task::master::GLOBAL_MASTER;
use crate::task::rich::Condition;
use crate::task::seed::IndexPolicy;
use crate::utils::comm::is_bytes_fill;
use crate::utils::enums::IndexType;
use crate::utils::path::{index_path, node_filepath};
use comm::errors::children::DataNoExistError;
use comm::trans::{trans_bytes_2_u16, trans_bytes_2_u48};
use comm::vectors::{Vector, VectorHandler};

/// 索引B+Tree结点结构
///
/// 包含了索引的根结点、子结点以及叶子结点
///
/// 叶子结点中才会存在Link，其余结点Link为None
#[derive(Debug, Clone)]
pub(crate) struct Node {
    atomic_key: Arc<AtomicU64>,
    database_name: String,
    view_name: String,
    index_name: String,
    node_file_path: String,
}

impl Node {
    /// 新建根结点
    ///
    /// 该结点没有Links，也没有preNode，是B+Tree的创世结点
    pub fn create_root(
        database_name: String,
        view_name: String,
        index_name: String,
    ) -> Arc<RwLock<Self>> {
        let atomic_key = Arc::new(AtomicU64::new(1));
        let index_path = index_path(database_name.clone(), view_name.clone(), index_name.clone());
        let node_file_path = node_filepath(index_path, String::from("increment"));
        Filer::try_touch(node_file_path.clone());
        Arc::new(RwLock::new(Node {
            atomic_key,
            database_name,
            view_name,
            index_name,
            node_file_path,
        }))
    }
    /// 恢复根结点
    pub fn recovery_root(
        database_name: String,
        view_name: String,
        index_name: String,
    ) -> GeorgeResult<Arc<RwLock<Self>>> {
        let index_path = index_path(database_name.clone(), view_name.clone(), index_name.clone());
        let node_file_path = node_filepath(index_path, String::from("increment"));
        let file = Filer::reader_writer(node_file_path.clone())?;
        let file_len = file.try_clone().unwrap().seek(SeekFrom::End(0)).unwrap();
        let atomic_key_u32 = file_len / 8;
        // log::debug!("atomic_key_u32 = {}", atomic_key_u32);
        let atomic_key = Arc::new(AtomicU64::new(atomic_key_u32));
        Ok(Arc::new(RwLock::new(Node {
            atomic_key,
            database_name,
            view_name,
            index_name,
            node_file_path,
        })))
    }
    fn database_name(&self) -> String {
        self.database_name.clone()
    }
    fn view_name(&self) -> String {
        self.view_name.clone()
    }
    fn node_file_path(&self) -> String {
        self.node_file_path.clone()
    }
}

/// 封装方法函数
impl TNode for Node {
    fn node_bytes(&self) -> Arc<RwLock<Vec<u8>>> {
        Arc::new(RwLock::new(vec![]))
    }
    fn modify(&mut self, database_name: String, view_name: String) {
        self.database_name = database_name;
        self.view_name = view_name;
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
        mut hash_key: u64,
        seed: Arc<RwLock<dyn TSeed>>,
        force: bool,
    ) -> GeorgeResult<()> {
        if hash_key == 0 {
            hash_key = self.atomic_key.fetch_add(1, Ordering::Relaxed);
            self.put_in_node(hash_key, seed, force, false)
        } else {
            self.put_in_node(hash_key, seed, force, true)
        }
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
            self.left_query(start, end, conditions, skip, limit, delete)
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
        hash_key: u64,
        seed: Arc<RwLock<dyn TSeed>>,
        force: bool,
        custom: bool,
    ) -> GeorgeResult<()>
    where
        Self: Sized,
    {
        let seek = hash_key * 8;
        if !force {
            let file = Filer::reader(self.node_file_path())?;
            let res = Filer::read_subs(file, seek, 8)?;
            if is_bytes_fill(res) {
                return if custom {
                    Err(err_str("auto increment key has been used"))
                } else {
                    self.put(0, seed, force)
                };
            }
        }
        seed.write().unwrap().modify(IndexPolicy::bytes(
            IndexType::Dossier,
            self.node_file_path(),
            seek,
        )?)
    }
    fn get_in_node(&self, hash_key: u64) -> GeorgeResult<Vec<u8>> {
        let seek = hash_key * 8;
        let res = Filer::read_sub(self.node_file_path(), seek, 8)?;
        return if is_bytes_fill(res.clone()) {
            Ok(res)
        } else {
            Err(GeorgeError::from(DataNoExistError))
        };
    }
    fn del_in_node(&self, hash_key: u64) -> GeorgeResult<()> {
        let seek = hash_key * 8;
        Filer::write_seek(self.node_file_path(), seek, Vector::create_empty_bytes(8))?;
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
            let res = Filer::read_sub(self.node_file_path(), key_start, 8)?;
            let (check, value_bytes) = self.check(key_end, conditions.clone(), delete, res)?;
            if check {
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
            let res = Filer::read_sub(self.node_file_path(), key_end, 8)?;
            let (valid, value_bytes) = self.check(key_end, conditions.clone(), delete, res)?;
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
    /// 检查值有效性
    fn check(
        &self,
        key: u64,
        conditions: Vec<Condition>,
        delete: bool,
        res: Vec<u8>,
    ) -> GeorgeResult<(bool, Vec<u8>)> {
        if is_bytes_fill(res.clone()) {
            let version = trans_bytes_2_u16(Vector::sub(res.clone(), 0, 2)?)?;
            let seek = trans_bytes_2_u48(Vector::sub(res, 2, 8)?)?;
            let value_bytes = GLOBAL_MASTER.read_content_by(
                self.database_name(),
                self.view_name(),
                version,
                seek,
            )?;
            if Condition::validate(conditions.clone(), value_bytes.clone()) {
                if delete {
                    Filer::write_seek(self.node_file_path(), key, Vector::create_empty_bytes(8))?;
                }
                Ok((true, value_bytes))
            } else {
                Ok((false, vec![]))
            }
        } else {
            Ok((false, vec![]))
        }
    }
}
