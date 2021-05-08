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

use comm::errors::children::{DataExistError, DataNoExistError};
use comm::errors::entrances::{GeorgeError, GeorgeResult};
use comm::io::file::{Filer, FilerHandler};
use comm::trans::{trans_bytes_2_u64, trans_u64_2_bytes};
use comm::vectors::{Vector, VectorHandler};

use crate::task::engine::traits::{TNode, TSeed};
use crate::task::engine::{DataReal, RootBytes};
use crate::task::rich::Condition;
use crate::task::seed::IndexPolicy;
use crate::task::view::View;
use crate::utils::comm::{level_distance_32, HashKey, HashKeyHandler};
use crate::utils::enums::{IndexType, KeyType};
use crate::utils::path::{index_path, node_filepath, record_filepath};
use crate::utils::writer::Filed;

const BYTES_LEN_FOR_DOSSIER: usize = 2048;

/// 索引B+Tree结点结构
///
/// 包含了索引的根结点、子结点以及叶子结点
///
/// 叶子结点中才会存在Link，其余结点Link为None
///
/// 子项是32位node集合，在node集合中每一个node的默认字节长度是8，数量是256，即一次性读取2048个字节
///
/// record文件最大为2^(8*8)=16348PB
///
/// record存储固定长度的数据，长度为16，即view视图真实数据8+链式后续数据8，总计可存(2^64)/16条数据
#[derive(Debug, Clone)]
pub(crate) struct Node {
    view: Arc<RwLock<View>>,
    index_name: String,
    key_type: KeyType,
    index_path: String,
    /// 索引文件路径
    ///
    /// 当有新的数据加入时，新数据存储地址在`node_file`中记录8字节
    ///
    /// 该项与`unique`和`record_filepath`组合使用
    ///
    /// 当`unique`为true时，则存储的8字节为view视图真实数据地址
    ///
    /// 当`unique`为false时，则与`record_file`搭配使用，启动碰撞链式结构
    node_filepath: String,
    /// 用于记录重复索引链式结构信息
    ///
    /// 当有新的数据加入时，新数据存储地址在`node_file`中记录8字节，为`数据地址`
    ///
    /// `数据地址`指向`record_file`中起始偏移量，持续16字节，组成为`view中真实数据地址8字节 + 下一数据地址8字节`
    ///
    /// 当下一数据地址为空时，则表示当前链式结构已到尾部
    ///
    /// 当`unique`为true时，该项不启用
    record_filepath: String,
    /// 是否唯一索引
    unique: bool,
    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 需要借助对象包裹，以便更新file，避免self为mut
    node_filer: Filed,
    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 需要借助对象包裹，以便更新file，避免self为mut
    record_filer: Filed,
    /// 存储根结点所属各子结点坐标顺序字节数组
    ///
    /// 子项是32位node集合，在node集合中每一个node的默认字节长度是8，数量是256，即一次性读取2048个字节
    root_bytes: Arc<RwLock<RootBytes>>,
}

impl Node {
    /// 新建根结点
    ///
    /// 该结点没有Links，也没有preNode，是B+Tree的创世结点
    pub fn create(
        view: Arc<RwLock<View>>,
        index_name: String,
        key_type: KeyType,
        unique: bool,
    ) -> GeorgeResult<Arc<Self>> {
        let v_c = view.clone();
        let v_r = v_c.read().unwrap();
        let index_path = index_path(v_r.database_name(), v_r.name(), index_name.clone());
        let node_filepath = node_filepath(index_path.clone(), String::from("dossier"));
        let node_filer = Filed::create(node_filepath.clone())?;
        let record_filepath = record_filepath(index_path.clone());
        let record_filer = Filed::create(record_filepath.clone())?;
        record_filer.append(vec![0x86, 0x87])?;
        let rb = RootBytes::create(BYTES_LEN_FOR_DOSSIER);
        node_filer.append(rb.bytes())?;
        let root_bytes = Arc::new(RwLock::new(rb));
        Ok(Arc::new(Node {
            view,
            index_name,
            key_type,
            index_path,
            node_filepath,
            record_filepath,
            unique,
            node_filer,
            record_filer,
            root_bytes,
        }))
    }

    /// 恢复根结点
    pub fn recovery(
        view: Arc<RwLock<View>>,
        index_name: String,
        key_type: KeyType,
        unique: bool,
    ) -> GeorgeResult<Arc<Self>> {
        let v_c = view.clone();
        let v_r = v_c.read().unwrap();
        let index_path = index_path(v_r.database_name(), v_r.name(), index_name.clone());
        let node_filepath = node_filepath(index_path.clone(), String::from("dossier"));
        let node_filer = Filed::recovery(node_filepath.clone())?;
        let record_filepath = record_filepath(index_path.clone());
        let record_filer = Filed::recovery(record_filepath.clone())?;
        let rb = node_filer.read(0, BYTES_LEN_FOR_DOSSIER)?;
        let root_bytes = Arc::new(RwLock::new(RootBytes::recovery(rb, BYTES_LEN_FOR_DOSSIER)?));
        Ok(Arc::new(Node {
            view,
            index_name,
            key_type,
            index_path,
            node_filepath,
            record_filepath,
            unique,
            node_filer,
            record_filer,
            root_bytes,
        }))
    }

    fn database_name(&self) -> String {
        self.view.clone().read().unwrap().database_name()
    }

    fn view_name(&self) -> String {
        self.view.clone().read().unwrap().name()
    }

    fn index_name(&self) -> String {
        self.index_name.clone()
    }

    fn key_type(&self) -> KeyType {
        self.key_type.clone()
    }

    fn index_path(&self) -> String {
        self.index_path.clone()
    }

    fn node_filepath(&self) -> String {
        self.node_filepath.clone()
    }

    fn node_bytes(&self) -> Vec<u8> {
        self.root_bytes.read().unwrap().bytes()
    }

    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
    ///
    /// #Return
    ///
    /// seek_end_before 写之前文件字节数据长度
    fn node_append(&self, content: Vec<u8>) -> GeorgeResult<u64> {
        self.node_filer.append(content)
    }

    fn node_read(&self, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
        self.node_filer.clone().read(start, last)
    }

    fn node_write(&self, seek: u64, content: Vec<u8>) -> GeorgeResult<()> {
        self.node_filer.write(seek, content)
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
        self.record_filer.append(content)
    }

    fn record_read(&self, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
        self.record_filer.clone().read(start, last)
    }

    fn record_write(&self, seek: u64, content: Vec<u8>) -> GeorgeResult<()> {
        self.record_filer.write(seek, content)
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
        let hash_key = HashKey::obtain(IndexType::Dossier, self.key_type(), key.clone())?;
        self.put_in_node(0, self.node_bytes(), key, 1, hash_key, seed, force)
    }

    fn get(&self, key: String) -> GeorgeResult<Vec<u8>> {
        let hash_key = HashKey::obtain(IndexType::Dossier, self.key_type(), key.clone())?;
        self.get_in_node(self.node_bytes(), key, 1, hash_key)
    }

    fn del(&self, _key: String, _seed: Arc<RwLock<dyn TSeed>>) -> GeorgeResult<()> {
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
    /// node_bytes_seek 当前操作结点的字节数组起始坐标
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
        node_bytes_seek: u64,
        node_bytes: Vec<u8>,
        key: String,
        level: u8,
        flexible_key: u32,
        seed: Arc<RwLock<dyn TSeed>>,
        force: bool,
    ) -> GeorgeResult<()>
    where
        Self: Sized,
    {
        // 通过当前树下一层高获取结点间间隔数量，即每一度中存在的元素数量
        let distance = level_distance_32(level);
        // 通过当前层真实key除以下一层间隔数获取结点处在下一层的度数
        let next_degree = flexible_key / distance;
        // 相对当前结点字节数组，下一结点在字节数组中的偏移量
        let next_node_start = (next_degree * 8) as usize;
        // 下一结点字节数组起始坐标
        let mut next_node_seek_bytes = Vector::sub_last(node_bytes, next_node_start, 8)?;
        // 下一结点的真实坐标
        let next_node_seek: u64;
        // 由view视图执行save操作时反写进record文件中value起始seek
        let record_view_info_seek: u64;
        // 如果当前层高为4，则达到最底层，否则递归下一层逻辑
        if level == 4 {
            // 如果存在坐标值，则继续，否则新建
            if Vector::is_fill(next_node_seek_bytes.clone()) {
                // 索引执行插入真实坐标
                next_node_seek = trans_bytes_2_u64(next_node_seek_bytes)?;
                // 已存在该索引值，需要继续判断插入可行性
                // 如果唯一且非强制覆盖，返回数据已存在
                if self.unique && !force {
                    return Err(GeorgeError::from(DataExistError));
                }
                record_view_info_seek =
                    self.record_view_info_seek_put(key.clone(), next_node_seek, force)?;
            } else {
                // 不存在下一坐标值，新建
                // record追加新链式子结构
                record_view_info_seek = self.record_append(Vector::create_empty_bytes(16))?;
                // record新追加链式子结构坐标字节数组
                let record_seek_bytes = trans_u64_2_bytes(record_view_info_seek);
                // record起始链式结构在node文件中真实坐标
                next_node_seek = node_bytes_seek + next_node_start as u64;
                // 将record新追加链式子结构坐标字节数组写入record起始链式结构在node文件中真实坐标
                self.node_write(next_node_seek, record_seek_bytes)?;
            }
            seed.write().unwrap().modify(IndexPolicy::create(
                key,
                IndexType::Dossier,
                self.record_filepath(),
                record_view_info_seek,
            ));
            Ok(())
        } else {
            // 下一结点字节数组
            let next_node_bytes: Vec<u8>;
            // 如果存在坐标值，则继续，否则新建
            if Vector::is_fill(next_node_seek_bytes.clone()) {
                next_node_seek = trans_bytes_2_u64(next_node_seek_bytes)?;
                next_node_bytes = self.node_read(next_node_seek, BYTES_LEN_FOR_DOSSIER)?;
            } else {
                // 创建新的结点字节数组
                next_node_bytes = Vector::create_empty_bytes(BYTES_LEN_FOR_DOSSIER);
                // 将新的结点字节数组写入node_file并返回写入前的起始坐标
                next_node_seek = self.node_append(next_node_bytes.clone())?;
                next_node_seek_bytes = trans_u64_2_bytes(next_node_seek);
                // 下一结点坐标记录在文件中的坐标
                let next_node_seek_real_seek = node_bytes_seek + next_node_start as u64;
                self.node_write(next_node_seek_real_seek, next_node_seek_bytes)?;
            }
            // 通过当前层真实key减去下一层的度数与间隔数的乘积获取结点所在下一层的真实key
            let next_flexible_key = flexible_key - next_degree * distance;
            self.put_in_node(
                next_node_seek,
                next_node_bytes,
                key,
                level + 1,
                next_flexible_key,
                seed,
                force,
            )
        }
    }

    /// 获取由view视图执行save操作时反写进record文件中value起始seek
    fn record_view_info_seek_put(
        &self,
        key: String,
        record_seek: u64,
        force: bool,
    ) -> GeorgeResult<u64> {
        // 读取record中该坐标值
        let res = self.record_read(record_seek, 16)?;
        // record存储固定长度的数据，长度为16，即view视图真实数据8+链式后续数据8
        // 读取view视图真实数据坐标
        let view_info_seek_bytes = Vector::sub_last(res.clone(), 0, 8)?;
        // 如果view视图真实数据坐标为空
        // 处理因断点、宕机等意外导致后续索引数据写入成功而视图数据写入失败的问题
        if !Vector::is_fill(view_info_seek_bytes.clone()) {
            Ok(record_seek)
        } else {
            // 从view视图中读取真实数据内容
            let info = self
                .view
                .read()
                .unwrap()
                .read_content_by(view_info_seek_bytes)?;
            // 将字节数组内容转换为可读kv
            let date = DataReal::from(info)?;
            // 因为hash key指向同一碰撞，对比key是否相同
            if date.key == key {
                // 如果key相同，则判断是否强制覆盖
                if force {
                    // 如果强制覆盖，则返回当前待覆盖坐标
                    Ok(record_seek)
                } else {
                    // 如果不能覆盖，则返回数据已存在
                    Err(GeorgeError::from(DataExistError))
                }
            // 如果key不同，则发生hash碰撞，开启索引链式结构循环坐标定位
            } else {
                // record存储固定长度的数据，长度为16，即view视图真实数据8+链式后续数据8
                // 读取链式后续数据坐标
                let record_next_seek_bytes = Vector::sub_last(res, 8, 8)?;
                // 如果链式后续数据有值，则进入下一轮判定
                if Vector::is_fill(record_next_seek_bytes.clone()) {
                    let record_next_seek = trans_bytes_2_u64(record_next_seek_bytes)?;
                    self.record_view_info_seek_put(key, record_next_seek, force)
                // 如果链式后续数据无值，则插入新数据
                } else {
                    // record追加新链式子结构
                    let record_next_seek = self.record_append(Vector::create_empty_bytes(16))?;
                    // record新追加链式子结构坐标字节数组
                    let record_next_seek_bytes = trans_u64_2_bytes(record_next_seek);
                    // 当前record中链式子结构坐标在record文件中记录的坐标位置
                    let record_next_seek_seek = record_seek + 8;
                    // 将下一record的坐标写入当前record链式子结构字节数组中
                    self.record_write(record_next_seek_seek, record_next_seek_bytes)?;
                    Ok(record_next_seek)
                }
            }
        }
    }

    fn get_in_node(
        &self,
        node_bytes: Vec<u8>,
        key: String,
        level: u8,
        flexible_key: u32,
    ) -> GeorgeResult<Vec<u8>> {
        // 通过当前树下一层高获取结点间间隔数量，即每一度中存在的元素数量
        let distance = level_distance_32(level);
        // 通过当前层真实key除以下一层间隔数获取结点处在下一层的度数
        let next_degree = flexible_key / distance;
        // 相对当前结点字节数组，下一结点在字节数组中的偏移量
        let next_node_start = (next_degree * 8) as usize;
        // 下一结点字节数组起始坐标
        let next_node_seek_bytes = Vector::sub_last(node_bytes, next_node_start, 8)?;
        // 如果当前层高为4，则达到最底层，否则递归下一层逻辑
        if level == 4 {
            self.judge_seek_bytes(key, next_node_seek_bytes)
        } else {
            // 如果存在坐标值，则继续，否则新建
            if Vector::is_fill(next_node_seek_bytes.clone()) {
                // 下一结点的真实坐标
                let next_node_seek = trans_bytes_2_u64(next_node_seek_bytes)?;
                // 下一结点字节数组
                let next_node_bytes = self.node_read(next_node_seek, BYTES_LEN_FOR_DOSSIER)?;
                // 通过当前层真实key减去下一层的度数与间隔数的乘积获取结点所在下一层的真实key
                let next_flexible_key = flexible_key - next_degree * distance;
                self.get_in_node(next_node_bytes, key, level + 1, next_flexible_key)
            } else {
                // 如果为空，则返回无此数据
                Err(GeorgeError::from(DataNoExistError))
            }
        }
    }

    /// 期望根据`下一结点偏移量字节数组`获取由view视图执行save操作时反写进record文件中value起始seek
    ///
    /// 如果存在坐标值，则继续，否则返回无此数据
    fn judge_seek_bytes(
        &self,
        key: String,
        next_node_seek_bytes: Vec<u8>,
    ) -> GeorgeResult<Vec<u8>> {
        // 如果存在坐标值，则继续，否则返回无此数据
        if Vector::is_fill(next_node_seek_bytes.clone()) {
            // 索引执行插入真实坐标
            let next_node_seek = trans_bytes_2_u64(next_node_seek_bytes)?;
            self.record_view_info_seek_get(key, next_node_seek)
        } else {
            // 如果为空，则返回无此数据
            Err(GeorgeError::from(DataNoExistError))
        }
    }

    /// 获取由view视图执行save操作时反写进record文件中value起始seek
    fn record_view_info_seek_get(&self, key: String, record_seek: u64) -> GeorgeResult<Vec<u8>> {
        // 读取record中该坐标值
        let res = self.record_read(record_seek, 16)?;
        // record存储固定长度的数据，长度为16，即view视图真实数据8+链式后续数据8
        // 读取view视图真实数据坐标
        let view_info_seek_bytes = Vector::sub_last(res.clone(), 0, 8)?;
        // 如果view视图真实数据坐标为空
        // 处理因断点、宕机等意外导致后续索引数据写入成功而视图数据写入失败的问题
        if !Vector::is_fill(view_info_seek_bytes.clone()) {
            // record存储固定长度的数据，长度为16，即view视图真实数据8+链式后续数据8
            // 读取链式后续数据坐标
            let record_next_seek_bytes = Vector::sub_last(res, 8, 8)?;
            self.judge_seek_bytes(key, record_next_seek_bytes)
        } else {
            // 从view视图中读取真实数据内容
            let info = self
                .view
                .read()
                .unwrap()
                .read_content_by(view_info_seek_bytes)?;
            // 将字节数组内容转换为可读kv
            let date = DataReal::from(info)?;
            // 因为hash key指向同一碰撞，对比key是否相同
            if date.key == key {
                Ok(date.value)
            // 如果key不同，则发生hash碰撞，开启索引链式结构循环坐标定位
            } else {
                // record存储固定长度的数据，长度为16，即view视图真实数据8+链式后续数据8
                // 读取链式后续数据坐标
                let record_next_seek_bytes = Vector::sub_last(res, 8, 8)?;
                self.judge_seek_bytes(key, record_next_seek_bytes)
            }
        }
    }
}

impl Node {
    pub fn mock_create(
        view: Arc<RwLock<View>>,
        index_name: String,
        key_type: KeyType,
        unique: bool,
    ) -> GeorgeResult<Arc<Self>> {
        let v_c = view.clone();
        let v_r = v_c.read().unwrap();
        let index_path = index_path(v_r.database_name(), v_r.name(), index_name.clone());
        let node_filepath = node_filepath(index_path.clone(), String::from("dossier"));
        let node_filer = Filed::mock(node_filepath.clone())?;
        let record_filepath = record_filepath(index_path.clone());
        let record_filer = Filed::mock(record_filepath.clone())?;
        record_filer.append(vec![0x86, 0x87])?;
        let rb = RootBytes::create(BYTES_LEN_FOR_DOSSIER);
        node_filer.append(rb.bytes())?;
        let root_bytes = Arc::new(RwLock::new(rb));
        Ok(Arc::new(Node {
            view,
            index_name,
            key_type,
            index_path,
            node_filepath,
            record_filepath,
            unique,
            node_filer,
            record_filer,
            root_bytes,
        }))
    }

    pub fn mock_recovery(
        view: Arc<RwLock<View>>,
        index_name: String,
        key_type: KeyType,
        unique: bool,
    ) -> GeorgeResult<Arc<Self>> {
        let v_c = view.clone();
        let v_r = v_c.read().unwrap();
        let index_path = index_path(v_r.database_name(), v_r.name(), index_name.clone());
        let node_filepath = node_filepath(index_path.clone(), String::from("dossier"));
        let node_filer = Filed::mock(node_filepath.clone())?;
        let record_filepath = record_filepath(index_path.clone());
        let record_filer: Filed;
        if Filer::exist(record_filepath.clone()) {
            record_filer = Filed::mock(record_filepath.clone())?;
        } else {
            record_filer = Filed::mock(record_filepath.clone())?;
            record_filer.append(vec![0x86, 0x87])?;
        }
        let root_bytes: Arc<RwLock<RootBytes>>;
        match node_filer.read(0, BYTES_LEN_FOR_DOSSIER) {
            Ok(rb) => {
                root_bytes = Arc::new(RwLock::new(RootBytes::recovery(rb, BYTES_LEN_FOR_DOSSIER)?))
            }
            Err(_) => {
                let rb = RootBytes::create(BYTES_LEN_FOR_DOSSIER);
                node_filer.append(rb.bytes())?;
                root_bytes = Arc::new(RwLock::new(rb))
            }
        }
        Ok(Arc::new(Node {
            view,
            index_name,
            key_type,
            index_path,
            node_filepath,
            record_filepath,
            unique,
            node_filer,
            record_filer,
            root_bytes,
        }))
    }
}
