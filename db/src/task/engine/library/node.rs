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

use crate::task::engine::traits::{TNode, TSeed};
use crate::task::engine::DataReal;
use crate::task::rich::Condition;
use crate::task::seed::IndexPolicy;
use crate::task::view::View;
use crate::utils::comm::{is_bytes_fill, level_distance_64};
use crate::utils::enums::IndexType;
use crate::utils::path::{index_path, node_filepath, record_filepath};
use crate::utils::writer::Filed;
use comm::errors::children::{DataExistError, DataNoExistError};
use comm::errors::entrances::{GeorgeError, GeorgeResult};
use comm::io::file::{Filer, FilerHandler, FilerNormal};
use comm::strings::{StringHandler, Strings};
use comm::trans::{trans_bytes_2_u32_as_u64, trans_u32_2_bytes};
use comm::vectors::{Vector, VectorHandler};
use std::ops::Add;

/// 索引B+Tree结点结构
///
/// 包含了索引的根结点、子结点以及叶子结点
///
/// 叶子结点中才会存在Link，其余结点Link为None
///
/// library会创建分散的索引文件，每个索引文件可以存储65536条数据，每条数据长度为4，每个索引文件为256Kb
///
/// record文件最大为2^(4*8)=4GB
///
/// record存储固定长度的数据，长度为12，即view视图真实数据8+链式后续数据4，总计可存3.57913941亿条数据
#[derive(Debug, Clone)]
pub(crate) struct Node {
    view: View,
    index_name: String,
    index_path: String,
    record_filepath: String,
    /// 是否唯一索引
    unique: bool,
    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 需要借助对象包裹，以便更新file，避免self为mut
    record_filer: Filed,
}

impl Node {
    /// 新建根结点
    ///
    /// 该结点没有Links，也没有preNode，是B+Tree的创世结点
    pub fn create(view: View, index_name: String, unique: bool) -> GeorgeResult<Arc<RwLock<Self>>> {
        let index_path = index_path(view.database_name(), view.name(), index_name.clone());
        let record_filepath = record_filepath(index_path.clone());
        let record_filer = Filed::create(record_filepath.clone())?;
        record_filer.append(vec![0x86, 0x87])?;
        Ok(Arc::new(RwLock::new(Node {
            view,
            index_name,
            index_path,
            record_filepath,
            unique,
            record_filer,
        })))
    }
    /// 恢复根结点
    pub fn recovery(
        view: View,
        index_name: String,
        unique: bool,
    ) -> GeorgeResult<Arc<RwLock<Self>>> {
        let index_path = index_path(view.database_name(), view.name(), index_name.clone());
        let record_filepath = record_filepath(index_path.clone());
        let record_filer = Filed::recovery(record_filepath.clone())?;
        Ok(Arc::new(RwLock::new(Node {
            view,
            index_name,
            index_path,
            record_filepath,
            unique,
            record_filer,
        })))
    }
    fn database_name(&self) -> String {
        self.view.database_name()
    }
    fn view_name(&self) -> String {
        self.view.name()
    }
    fn index_name(&self) -> String {
        self.index_name.clone()
    }
    fn index_path(&self) -> String {
        self.index_path.clone()
    }
    fn record_filepath(&self) -> String {
        self.record_filepath.clone()
    }
    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
    ///
    /// #Return
    ///
    /// seek_end_before 写之前文件字节数据长度
    fn record_append(&self, content: Vec<u8>) -> GeorgeResult<u64> {
        self.record_filer.clone().append(content)
    }
    fn record_read(&self, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
        self.record_filer.clone().read(start, last)
    }
    fn record_write(&self, seek: u64, content: Vec<u8>) -> GeorgeResult<()> {
        self.record_filer.clone().write(seek, content)
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
    fn put(
        &self,
        key: String,
        hash_key: u64,
        seed: Arc<RwLock<dyn TSeed>>,
        force: bool,
    ) -> GeorgeResult<()> {
        self.put_in_node(key, String::from(""), 1, hash_key, seed, force)
    }
    fn get(&self, key: String, hash_key: u64) -> GeorgeResult<Vec<u8>> {
        self.get_in_node(key, String::from(""), 1, hash_key)
    }
    fn del(&self, _key: String, _hash_key: u64) -> GeorgeResult<()> {
        unimplemented!()
    }
    fn select(
        &self,
        _left: bool,
        _start: u64,
        _end: u64,
        _skip: u64,
        _limit: u64,
        _delete: bool,
        _conditions: Vec<Condition>,
    ) -> GeorgeResult<(u64, u64, Vec<Vec<u8>>)> {
        unimplemented!()
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
        mut index_filename: String,
        level: u8,
        flexible_key: u64,
        seed: Arc<RwLock<dyn TSeed>>,
        force: bool,
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
            let node_filepath = node_filepath(self.index_path(), index_filename);
            // log::debug!("node_filepath = {}, degree = {}",node_filepath,next_degree);
            let node_file_seek = next_degree * 4;
            // 在record中的偏移量
            let record_seek = self.record_seek(node_filepath.clone(), node_file_seek)?;
            // log::debug!("record_seek = {}", record_seek);
            let seek: u64;
            // 如果是唯一索引，则可能需要判断是否存在已有值
            if self.unique {
                // 如果唯一且强制覆盖
                if force {
                    seek = record_seek;
                } else {
                    // 否则需要进一步判断，判断索引是否为空
                    let res = self.record_read(record_seek, 8)?;
                    // 如果不为空
                    if is_bytes_fill(res) {
                        return Err(GeorgeError::from(DataExistError));
                    } else {
                        // 如果为空
                        seek = record_seek;
                    }
                }
            } else {
                // 如果非唯一索引，则需要读取链式结构
                let mut record_loop_seek = record_seek;
                loop {
                    // 判断索引是否为空
                    let res = self.record_read(record_loop_seek, 8)?;
                    // 如果不为空
                    if is_bytes_fill(res) {
                        // 先查询链式结构是否有后续内容
                        let record_next = record_loop_seek + 8;
                        let seek_next_bytes = self.record_read(record_next, 4)?;
                        // 如果有，则尝试读取后续内容
                        if is_bytes_fill(seek_next_bytes.clone()) {
                            record_loop_seek = trans_bytes_2_u32_as_u64(seek_next_bytes)?;
                        } else {
                            // 如果没有，则新建后续结构
                            seek = self.record_append(Vector::create_empty_bytes(12))?;
                            self.record_write(record_next, trans_u32_2_bytes(seek as u32))?;
                            break;
                        }
                    } else {
                        // 如果为空，使用当前空位补全
                        seek = record_seek;
                        break;
                    }
                }
            }
            seed.write().unwrap().modify(IndexPolicy::create(
                key.clone(),
                IndexType::Library,
                self.record_filepath(),
                seek,
            ));
            seed.write().unwrap().modify(IndexPolicy::create_custom(
                key,
                node_filepath,
                node_file_seek,
                trans_u32_2_bytes(seek as u32),
            ));
            Ok(())
        } else {
            index_filename = index_filename.add(&Strings::left_fits(
                next_degree.to_string(),
                "0".parse().unwrap(),
                5,
            ));
            // 通过当前层真实key减去下一层的度数与间隔数的乘机获取结点所在下一层的真实key
            let next_flexible_key = flexible_key - next_degree * distance;
            self.put_in_node(
                key,
                index_filename,
                level + 1,
                next_flexible_key,
                seed,
                force,
            )
        }
    }
    fn get_in_node(
        &self,
        key: String,
        mut index_filename: String,
        level: u8,
        flexible_key: u64,
    ) -> GeorgeResult<Vec<u8>> {
        // 通过当前树下一层高获取结点间间隔数量，即每一度中存在的元素数量
        let distance = level_distance_64(level);
        // 通过当前层真实key除以下一层间隔数获取结点处在下一层的度数
        let next_degree = flexible_key / distance;
        // 如果当前层高为4，则达到最底层，否则递归下一层逻辑
        if level == 4 {
            let node_filepath = node_filepath(self.index_path(), index_filename);
            // log::debug!("node_filepath = {}, degree = {}", node_filepath, next_degree);
            let node_file_seek = next_degree * 4;
            let record_seek = self.record_seek(node_filepath, node_file_seek)?;
            // log::debug!("record_seek = {}", record_seek);
            let mut record_loop_seek = record_seek;
            loop {
                // 判断索引是否为空
                let res = self.record_read(record_loop_seek, 8)?;
                // 如果不为空
                if is_bytes_fill(res.clone()) {
                    // 读取当前view视图中内容
                    let dr = DataReal::froms(self.database_name(), self.view_name(), res)?;
                    // 如果与查询key匹配，则直接返回
                    if dr.key.eq(&key) {
                        return Ok(dr.value);
                    }
                    // 如果不匹配，继续查询链式结构是否有后续内容
                    let seek_next_bytes = self.record_read(record_loop_seek + 8, 4)?;
                    // 如果有，则尝试读取后续内容
                    if is_bytes_fill(seek_next_bytes.clone()) {
                        record_loop_seek = trans_bytes_2_u32_as_u64(seek_next_bytes)?;
                    } else {
                        // 如果没有，则返回无此数据
                        return Err(GeorgeError::from(DataNoExistError));
                    }
                } else {
                    // 如果为空，则返回无此数据
                    return Err(GeorgeError::from(DataNoExistError));
                }
            }
        } else {
            index_filename = index_filename.add(&Strings::left_fits(
                next_degree.to_string(),
                "0".parse().unwrap(),
                5,
            ));
            // 通过当前层真实key减去下一层的度数与间隔数的乘机获取结点所在下一层的真实key
            let next_flexible_key = flexible_key - next_degree * distance;
            self.get_in_node(key, index_filename, level + 1, next_flexible_key)
        }
    }
    /// 在record中的偏移量
    fn record_seek(&self, node_filepath: String, node_file_seek: u64) -> GeorgeResult<u64> {
        match self.record_read(node_file_seek, 4) {
            Ok(seek_bytes) => {
                // 判断从索引中读取在record中的偏移量字节数组是否为空
                // 如果为空，则新插入占位字节，并以占位字节为起始变更偏移量
                if is_bytes_fill(seek_bytes.clone()) {
                    trans_bytes_2_u32_as_u64(seek_bytes)
                } else {
                    self.record_append(Vector::create_empty_bytes(12))
                }
            }
            _ => {
                Filer::try_touch(node_filepath)?;
                self.record_append(Vector::create_empty_bytes(12))
            }
        }
    }
}
