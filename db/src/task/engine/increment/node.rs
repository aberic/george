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

use comm::errors::{Errs, GeorgeResult};
use comm::vectors::VectorHandler;
use comm::Vector;
use ge::utils::enums::Tag;
use ge::{GeFactory, METADATA_SIZE};

use crate::task::engine;
use crate::task::engine::increment::Node;
use crate::task::engine::traits::{TNode, TSeed};
use crate::task::engine::DataReal;
use crate::task::rich::Condition;
use crate::task::seed::IndexPolicy;
use crate::task::traits::TForm;
use crate::utils::enums::Engine;
use crate::utils::Paths;

impl Node {
    /// 新建根结点
    ///
    /// 该结点没有Links，也没有preNode，是B+Tree的创世结点
    pub fn create(form: Arc<RwLock<dyn TForm>>, index_name: String) -> GeorgeResult<Arc<Self>> {
        let atomic_key = Arc::new(AtomicU64::new(1));
        let v_c = form.clone();
        let v_r = v_c.read().unwrap();
        let index_path = Paths::index_path(v_r.database_name(), v_r.name(), index_name.clone());
        let node_filepath = Paths::node_filepath(index_path, String::from("increment"));
        Ok(Arc::new(Node {
            form,
            atomic_key,
            index_name,
            ge: GeFactory {}.create(Tag::Node, node_filepath, None)?,
        }))
    }

    /// 恢复根结点
    pub fn recovery(form: Arc<RwLock<dyn TForm>>, index_name: String) -> GeorgeResult<Arc<Self>> {
        let v_c = form.clone();
        let v_r = v_c.read().unwrap();
        let index_path = Paths::index_path(v_r.database_name(), v_r.name(), index_name.clone());
        let node_filepath = Paths::node_filepath(index_path, String::from("increment"));
        let ge = GeFactory {}.recovery(Tag::Node, node_filepath)?;
        let last_key = (ge.len()? - METADATA_SIZE) / 12;
        let atomic_key = Arc::new(AtomicU64::new(last_key));
        Ok(Arc::new(Node {
            form,
            atomic_key,
            index_name,
            ge,
        }))
    }

    fn node_filepath(&self) -> String {
        self.ge.filepath()
    }

    fn read(&self, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
        self.ge.read_allow_none(start, last)
    }
}

/// 封装方法函数
impl TNode for Node {
    /// 插入数据<p><p>
    ///
    /// ###Params
    ///
    /// hash_key u64
    ///
    /// ###Return
    ///
    /// EngineResult<()>
    fn put(&self, key: String, seed: Arc<RwLock<dyn TSeed>>, force: bool) -> GeorgeResult<()> {
        let hash_key = self.atomic_key.fetch_add(1, Ordering::Relaxed);
        self.put_in_node(key, hash_key, seed, force)
    }

    fn get(&self, key: String) -> GeorgeResult<DataReal> {
        match key.parse::<u64>() {
            Ok(hash_key) => self.get_in_node(hash_key),
            Err(err) => Err(Errs::strings(format!("key {} parse to u64", key), err)),
        }
    }

    fn del(&self, key: String, seed: Arc<RwLock<dyn TSeed>>) -> GeorgeResult<()> {
        let hash_key = seed.clone().read().unwrap().increment();
        self.del_in_node(key, hash_key, seed)
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
        // 由`view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节)`组成
        let seek = METADATA_SIZE + hash_key * 12;
        if !force {
            // 由`view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节)`组成
            let res = self.read(seek, 12)?;
            if Vector::is_fill(res) {
                return Err(Errs::str("auto increment key has been used"));
            }
        }
        seed.write().unwrap().modify_4_put(IndexPolicy::create(
            key,
            Engine::Increment,
            self.node_filepath(),
            seek,
        ));
        Ok(())
    }

    fn get_in_node(&self, hash_key: u64) -> GeorgeResult<DataReal> {
        // 由`view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节)`组成
        let seek = METADATA_SIZE + hash_key * 12;
        // 由`view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节)`组成
        let res = self.read(seek, 12)?;
        return if Vector::is_fill(res.clone()) {
            // 从view视图中读取真实数据内容
            let info = self.form.read().unwrap().read_content_by_info(res)?;
            Ok(DataReal::from(info)?)
        } else {
            Err(Errs::data_no_exist_error())
        };
    }

    fn del_in_node(
        &self,
        key: String,
        hash_key: u64,
        seed: Arc<RwLock<dyn TSeed>>,
    ) -> GeorgeResult<()> {
        // 由`view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节)`组成
        let seek = METADATA_SIZE + hash_key * 12;
        // 由`view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节)`组成
        let res = self.read(seek, 12)?;
        if Vector::is_fill(res) {
            seed.write().unwrap().modify_4_del(IndexPolicy::create(
                key,
                Engine::Increment,
                self.node_filepath(),
                seek,
            ));
        }
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

        // 由`view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节)`组成
        let mut key_start = METADATA_SIZE + start * 12;
        let key_end: u64;
        if end > 0 {
            key_end = METADATA_SIZE + end * 12;
        } else {
            key_end = METADATA_SIZE + (self.atomic_key.load(Ordering::Relaxed) - 1) * 12;
        }
        loop {
            if limit <= 0 || key_start > key_end {
                break;
            }
            // 由`view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节)`组成
            let res = self.read(key_start, 12)?;
            let (valid, value_bytes) =
                engine::check(self.form.clone(), conditions.clone(), delete, res)?;
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
            // 由`view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节)`组成
            key_start += 12;
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

        // 由`view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节)`组成
        let key_start = METADATA_SIZE + start * 12;
        let mut key_end: u64;
        if end > 0 {
            key_end = METADATA_SIZE + end * 12;
        } else {
            key_end = METADATA_SIZE + (self.atomic_key.load(Ordering::Relaxed) - 1) * 12;
        }
        loop {
            if limit <= 0 || key_start > key_end {
                break;
            }
            // 由`view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节)`组成
            let res = self.read(key_end, 12)?;
            let (valid, value_bytes) =
                engine::check(self.form.clone(), conditions.clone(), delete, res)?;
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
            key_end -= 12;
        }
        Ok((total, count, values))
    }
}
