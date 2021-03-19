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

use std::collections::HashMap;
use std::fmt;
use std::fs::{read_dir, ReadDir};
use std::ops::Add;
use std::sync::{mpsc, Arc, RwLock};
use std::thread;

use chrono::{Duration, Local, NaiveDateTime};

use comm::errors::children::{DataNoExistError, IndexExistError};
use comm::errors::entrances::{err_str, err_string, err_strs, GeorgeError, GeorgeResult};
use comm::io::file::{Filer, FilerNormal, FilerReader, FilerWriter};
use comm::strings::{StringHandler, Strings};
use comm::trans::{
    trans_bytes_2_u16, trans_bytes_2_u32, trans_bytes_2_u48, trans_bytes_2_u64, trans_u32_2_bytes,
};
use comm::vectors::{Vector, VectorHandler};

use crate::task::engine::memory::seed::Seed as SeedMemory;
use crate::task::engine::traits::{TIndex, TSeed};
use crate::task::index::Index as IndexDefault;
use crate::task::rich::{Expectation, Selector};
use crate::task::seed::{IndexData, Seed as SeedDefault};
use crate::utils::comm::{
    key_fetch, INDEX_CATALOG, INDEX_MEMORY, INDEX_SEQUENCE, VALUE_TYPE_NORMAL,
};
use crate::utils::enums::{IndexType, KeyType, ViewType};
use crate::utils::path::{index_filepath, view_filepath, view_path};
use crate::utils::store::{before_content_bytes, recovery_before_content, Metadata, HD};
use crate::utils::writer::Filed;

/// 视图，类似表
#[derive(Debug, Clone)]
pub(crate) struct View {
    /// 数据库名称
    database_name: String,
    /// 名称
    name: String,
    /// 创建时间
    create_time: Duration,
    /// 文件信息
    metadata: Metadata,
    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 需要借助对象包裹，以便更新file，避免self为mut
    filer: Arc<RwLock<Filed>>,
    /// 是否为内存视图
    mem: bool,
    /// 索引集合
    indexes: Arc<RwLock<HashMap<String, Arc<RwLock<dyn TIndex>>>>>,
    /// 当前归档版本信息
    pigeonhole: Pigeonhole,
}

/// 新建视图
///
/// 具体传参参考如下定义：<p><p>
///
/// ###Params
///
/// mem 是否为内存视图
fn new_view(database_name: String, name: String, mem: bool) -> GeorgeResult<View> {
    let now: NaiveDateTime = Local::now().naive_local();
    let create_time = Duration::nanoseconds(now.timestamp_nanos());
    let filepath = view_filepath(database_name.clone(), name.clone());
    let metadata: Metadata;
    if mem {
        metadata = Metadata::view_mem()
    } else {
        metadata = Metadata::view_disk()
    }
    let view = View {
        database_name: database_name.clone(),
        name,
        create_time,
        metadata,
        filer: Filed::create(filepath.clone())?,
        mem,
        indexes: Default::default(),
        pigeonhole: Pigeonhole::create(0, filepath, create_time),
    };
    if mem {
        view.create_index_in(
            database_name,
            INDEX_MEMORY.to_string(),
            IndexType::Memory,
            KeyType::String,
            true,
            true,
        )?;
    } else {
        // view.create_index_in(
        //     database_name.clone(),
        //     INDEX_CATALOG.to_string(),
        //     IndexType::Library,
        //     KeyType::String,
        //     true,
        // )?;
        view.create_index_in(
            database_name,
            INDEX_SEQUENCE.to_string(),
            IndexType::Dossier,
            KeyType::U64,
            true,
            true,
        )?;
    }
    Ok(view)
}

impl View {
    pub(crate) fn create(database_name: String, name: String) -> GeorgeResult<Arc<RwLock<View>>> {
        let view = new_view(database_name, name, false)?;
        view.init()?;
        Ok(Arc::new(RwLock::new(view)))
    }
    pub(crate) fn create_m(database_name: String, name: String) -> GeorgeResult<Arc<RwLock<View>>> {
        let view = new_view(database_name, name, true)?;
        view.init()?;
        Ok(Arc::new(RwLock::new(view)))
    }
    fn init(&self) -> GeorgeResult<()> {
        let mut metadata_bytes = self.metadata_bytes();
        let mut description = self.description();
        // 初始化为32 + 8，即head长度加正文描述符长度
        let mut before_description = before_content_bytes(40, description.len() as u32);
        metadata_bytes.append(&mut before_description);
        metadata_bytes.append(&mut description);
        self.file_append(metadata_bytes)?;
        Ok(())
    }
    /// 数据库名称
    pub(crate) fn database_name(&self) -> String {
        self.database_name.clone()
    }
    /// 名称
    pub(crate) fn name(&self) -> String {
        self.name.clone()
    }
    /// 创建时间
    pub(crate) fn create_time(&self) -> Duration {
        self.create_time.clone()
    }
    /// 文件信息
    pub(crate) fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }
    /// 文件字节信息
    pub(crate) fn metadata_bytes(&self) -> Vec<u8> {
        self.metadata.bytes()
    }
    /// 文件信息
    pub(crate) fn view_type(&self) -> ViewType {
        self.metadata().view_type()
    }
    /// 索引集合
    pub(crate) fn index_map(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<dyn TIndex>>>>> {
        self.indexes.clone()
    }
    /// 获取默认索引
    pub(crate) fn index_catalog(&self) -> GeorgeResult<Arc<RwLock<dyn TIndex>>> {
        match self.index_map().read().unwrap().get(INDEX_CATALOG) {
            Some(idx) => Ok(idx.clone()),
            None => Err(err_str("index catalog does't found")),
        }
    }
    /// 获取索引
    pub(crate) fn index(&self, index_name: &str) -> GeorgeResult<Arc<RwLock<dyn TIndex>>> {
        match self.index_map().read().unwrap().get(index_name) {
            Some(idx) => Ok(idx.clone()),
            None => Err(err_string(format!("index {} doesn't found", index_name))),
        }
    }
    /// 当前归档版本信息
    pub(crate) fn pigeonhole(&self) -> Pigeonhole {
        self.pigeonhole.clone()
    }
    /// 当前视图版本号
    pub(crate) fn version(&self) -> u16 {
        self.pigeonhole().now().version()
    }
    /// 当前视图文件地址
    pub(crate) fn filepath(&self) -> String {
        self.pigeonhole().now().filepath()
    }
    /// 当前视图文件地址
    pub(crate) fn filepath_by_version(&self, version: u16) -> GeorgeResult<String> {
        if version == self.version() {
            Ok(self.filepath())
        } else {
            for (ver, record) in self.pigeonhole().history.iter() {
                if version.eq(ver) {
                    return Ok(record.filepath());
                }
            }
            Err(err_str("no view version found while get view filepath"))
        }
    }
    /// 当前归档版本信息
    pub(crate) fn record(&self, version: u16) -> GeorgeResult<Record> {
        if self.pigeonhole().now().version.eq(&version) {
            Ok(self.pigeonhole().now())
        } else {
            for (ver, record) in self.pigeonhole().history().iter() {
                if version.eq(ver) {
                    return Ok(record.clone());
                }
            }
            Err(err_str("no view version found"))
        }
    }
    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
    ///
    /// #Return
    ///
    /// seek_end_before 写之前文件字节数据长度
    fn file_append(&self, content: Vec<u8>) -> GeorgeResult<u64> {
        self.filer.write().unwrap().append(content)
    }
    /// 视图变更
    pub(crate) fn modify(&mut self, database_name: String, name: String) -> GeorgeResult<()> {
        let old_name = self.name();
        let filepath = view_filepath(database_name.clone(), old_name.clone());
        let content_old = Filer::read_sub(filepath.clone(), 0, 40)?;
        self.name = name;
        let description = self.description();
        let seek_end = self.file_append(description.clone())?;
        log::debug!(
            "view {} modify to {} with file seek_end = {}",
            old_name.clone(),
            self.name(),
            seek_end
        );
        let content_new = before_content_bytes(seek_end as u32, description.len() as u32);
        // 更新首部信息，初始化head为32，描述起始4字节，长度4字节
        Filer::write_seek(filepath.clone(), 32, content_new)?;
        let view_path_old = view_path(database_name.clone(), old_name);
        let view_path_new = view_path(database_name.clone(), self.name());
        match std::fs::rename(view_path_old, view_path_new) {
            Ok(_) => {
                for (_name, index) in self.index_map().write().unwrap().iter() {
                    index
                        .write()
                        .unwrap()
                        .modify(database_name.clone(), self.name())
                }
                self.database_name = database_name;
                Ok(())
            }
            Err(err) => {
                // 回滚数据
                Filer::write_seek(filepath, 0, content_old)?;
                Err(err_strs("file rename failed", err))
            }
        }
    }
    fn exist_index(&self, index_name: String) -> bool {
        return match self.index_map().read().unwrap().get(index_name.as_str()) {
            Some(_) => true,
            None => false,
        };
    }
    /// 创建索引
    pub(crate) fn create_index(
        &self,
        database_name: String,
        index_name: String,
        index_type: IndexType,
        key_type: KeyType,
        unique: bool,
    ) -> GeorgeResult<()> {
        match self.view_type() {
            ViewType::Memory => Err(err_str("this memory view allow only one index")),
            _ => self.create_index_in(
                database_name,
                index_name,
                index_type,
                key_type,
                false,
                unique,
            ),
        }
    }
    /// 创建索引内部方法，绕开外部调用验证
    fn create_index_in(
        &self,
        database_name: String,
        index_name: String,
        index_type: IndexType,
        key_type: KeyType,
        primary: bool,
        unique: bool,
    ) -> GeorgeResult<()> {
        if self.exist_index(index_name.clone()) {
            return Err(GeorgeError::from(IndexExistError));
        }
        let view_name = self.name();
        let name = index_name.clone();
        let index;
        match index_type {
            IndexType::None => return Err(err_str("unsupported engine type with none")),
            _ => {
                index = IndexDefault::create(
                    database_name,
                    view_name,
                    name,
                    index_type,
                    primary,
                    unique,
                    key_type,
                )?
            }
        }
        self.index_map().write().unwrap().insert(index_name, index);
        Ok(())
    }
}

/// db for disk
impl View {
    /// 插入数据，如果存在则返回已存在<p><p>
    ///
    /// ###Params
    ///
    /// key string
    ///
    /// value 当前结果value信息<p><p>
    ///
    /// ###Return
    ///
    /// IndexResult<()>
    pub(crate) fn put(&self, key: String, value: Vec<u8>) -> GeorgeResult<()> {
        self.save(key, value, false, false)
    }
    /// 插入数据，无论存在与否都会插入或更新数据<p><p>
    ///
    /// ###Params
    ///
    /// key string
    ///
    /// value 当前结果value信息<p><p>
    ///
    /// ###Return
    ///
    /// IndexResult<()>
    pub(crate) fn set(&self, key: String, value: Vec<u8>) -> GeorgeResult<()> {
        self.save(key, value, true, false)
    }
    /// 获取数据，返回存储对象<p><p>
    ///
    /// ###Params
    ///
    /// index_name 索引名称
    ///
    /// key string
    ///
    /// ###Return
    ///
    /// Seed value信息
    pub(crate) fn get(&self, index_name: &str, key: String) -> GeorgeResult<Vec<u8>> {
        let index = self.index(index_name)?;
        let idx = index.read().unwrap();
        let view_info_index = idx.get(key.clone())?;
        match index_name {
            INDEX_MEMORY => Ok(view_info_index),
            INDEX_SEQUENCE => {
                let version = trans_bytes_2_u16(Vector::sub(view_info_index.clone(), 0, 2)?)?;
                let seek = trans_bytes_2_u48(Vector::sub(view_info_index, 2, 8)?)?;
                let filepath = self.filepath_by_version(version)?;
                self.read_content(filepath, seek)
            }
            _ => {
                let index_data_list = self.fetch_view_info_index(view_info_index)?;
                for index_data in index_data_list {
                    if index_data.equal_key(key.clone()) {
                        return Ok(index_data.value());
                    }
                }
                Err(GeorgeError::from(DataNoExistError))
            }
        }
    }
    /// 删除数据<p><p>
    ///
    /// ###Params
    ///
    /// key string<p><p>
    ///
    /// ###Return
    ///
    /// IndexResult<()>
    pub(crate) fn remove(&self, key: String) -> GeorgeResult<()> {
        self.save(key, vec![], true, true)
    }
    /// 条件检索
    ///
    /// selector_json_bytes 选择器字节数组，自定义转换策略
    pub fn select(&self, constraint_json_bytes: Vec<u8>) -> GeorgeResult<Expectation> {
        Selector::run(constraint_json_bytes, self.indexes.clone(), false)
    }
    /// 条件删除
    ///
    /// selector_json_bytes 选择器字节数组，自定义转换策略
    pub fn delete(&self, constraint_json_bytes: Vec<u8>) -> GeorgeResult<Expectation> {
        Selector::run(constraint_json_bytes, self.indexes.clone(), true)
    }
}

impl View {
    /// 整理归档
    ///
    /// archive_file_path 归档路径
    pub(crate) fn archive(&self, archive_file_path: String) -> GeorgeResult<()> {
        self.filer.write().unwrap().archive(archive_file_path)?;
        self.init()
    }
    /// 取出表记录表内容索引
    ///
    /// 索引属性，主键溯源；主键不溯源；普通索引
    ///
    /// index_info_index 索引记录表内容索引，记录表文件属性(数据归档/定位文件用2字节)+数据在表文件中起始偏移量p(6字节)
    ///
    /// key 原始key
    fn fetch_view_info_index(&self, index_info_index: Vec<u8>) -> GeorgeResult<Vec<IndexData>> {
        let mut index_data_list: Vec<IndexData> = vec![];
        // 当前记录数据所属视图文件版本信息
        let version = trans_bytes_2_u16(Vector::sub(index_info_index.clone(), 0, 2)?)?;
        // 根据版本信息获取当前视图文件路径
        let filepath = self.path(version)?;
        // 当前记录数据数据在视图文件中起始偏移量p(6字节)
        let offset = trans_bytes_2_u64(Vector::sub(index_info_index, 2, 8)?)?;
        // 当前数据所在文件对象
        let file = Filer::reader(filepath.clone())?;
        // 定位数据字节数组=数据类型(1字节)+持续长度(4字节)
        let pos_bytes: Vec<u8>;
        match file.try_clone() {
            Ok(f) => pos_bytes = Filer::read_subs(f, offset, 5)?,
            Err(err) => return Err(err_strs("view fetch file try clone1", err)),
        }
        // 定位文件持续长度(4字节)
        let data_len = trans_bytes_2_u32(Vector::sub(pos_bytes.clone(), 1, 5)?)?;
        // 定位文件数据类型(1字节)
        let value_type_bytes: &u8;
        // 获取数据类型(1字节)
        match pos_bytes.get(0) {
            Some(vtb) => value_type_bytes = vtb,
            None => return Err(err_str("pos bytes get none")),
        }
        // 定位文件数据起始偏移量
        let offset_data = offset + 5;
        let view_info_index: Vec<u8>;
        match file.try_clone() {
            Ok(f) => view_info_index = Filer::read_subs(f, offset_data, data_len as usize)?,
            Err(err) => return Err(err_strs("view fetch file try clone1", err)),
        }
        // 正常数据类型
        if VALUE_TYPE_NORMAL.eq(value_type_bytes) {
            index_data_list.push(IndexData::create(self.clone(), view_info_index)?)
        } else {
            // 碰撞数据类型
        }
        Ok(index_data_list)
    }
    // /// 循环定位文件内容，(表内容索引(8字节)+原始key长度(2字节)+原始key)(循环)
    // ///
    // /// 找出与'key'相同的原始key数据
    // fn traverse(&self, key: String, value_type_bytes: &u8, data_bytes: Vec<u8>) -> GeorgeResult<Vec<u8>> {
    //     // 判断数据类型
    //     if VALUE_TYPE_NORMAL.eq(value_type_bytes) { // 正常数据类型，只有一条数据
    //         let original_key = Strings::from_utf8(Vector::sub(data_bytes.clone(), 10, data_bytes.len())?)?;
    //         if original_key.eq(&key) {
    //
    //         }
    //     } else if VALUE_TYPE_CRASH.eq(value_type_bytes) { // 碰撞数据类型，有多条循环数据
    //         let original_key_len = trans_bytes_2_u16(Vector::sub(data_bytes.clone(), 8, 10)?)?;
    //         let original_key = Vector::sub(data_bytes.clone(), original_key_len as usize, )
    //     } else {
    //     }
    //     Ok(vec![])
    // }
    // fn value_from(&self, value_bytes: Vec<u8>) -> GeorgeResult<IndexData> {
    //
    // }
    /// 取出可用数据集合
    ///
    /// data_info 记录表文件属性(数据归档/定位文件用2字节)+数据在表文件中起始偏移量p(6字节)
    ///
    /// key 原始key
    pub(crate) fn path(&self, version: u16) -> GeorgeResult<String> {
        if self.version() == version {
            Ok(self.filepath())
        } else {
            match self.pigeonhole().history().get(&version) {
                Some(record) => Ok(record.filepath()),
                None => Err(err_str("index exist but value is none!")),
            }
        }
    }
    /// 组装写入视图的内容，即持续长度+该长度的原文内容
    ///
    /// 将数据存入view，返回数据在view中的起始偏移量坐标
    pub(crate) fn write_content(&self, mut value: Vec<u8>) -> GeorgeResult<u64> {
        // 内容持续长度(4字节)
        let mut seed_bytes_len_bytes = trans_u32_2_bytes(value.len() as u32);
        // 真实存储内容，内容持续长度(4字节)+内容字节数组
        seed_bytes_len_bytes.append(&mut value);
        // 将数据存入view，返回数据在view中的起始坐标
        self.file_append(seed_bytes_len_bytes)
    }
    /// 读取已组装写入视图的内容，即持续长度+该长度的原文内容
    ///
    /// 在view中的起始偏移量坐标读取数据
    ///
    /// seek 读取偏移量
    pub(crate) fn read_content(&self, filepath: String, seek: u64) -> GeorgeResult<Vec<u8>> {
        let file = Filer::reader(filepath)?;
        let last: u32;
        match file.try_clone() {
            Ok(f) => {
                let bs = Filer::read_subs(f, seek, 4)?;
                last = trans_bytes_2_u32(bs)?
            }
            Err(err) => return Err(err_strs("get while file try clone", err)),
        }
        Filer::read_subs(file, seek + 4, last as usize)
    }
    /// 插入数据业务方法<p><p>
    ///
    /// ###Params
    ///
    /// key string
    ///
    /// value 当前结果value信息<p><p>
    ///
    /// force 如果存在原值，是否覆盖原结果<p><p>
    ///
    /// ###Return
    ///
    /// IndexResult<()>
    fn save(&self, key: String, value: Vec<u8>, force: bool, remove: bool) -> GeorgeResult<()> {
        let seed = Arc::new(RwLock::new(SeedDefault::create(key.clone(), value.clone())));
        let mut receives = Vec::new();
        for (index_name, index) in self.index_map().read().unwrap().iter() {
            let (sender, receive) = mpsc::channel();
            receives.push(receive);
            let index_name_clone = index_name.clone();
            let index_clone = index.clone();
            let key_clone = key.clone();
            let value_clone = value.clone();
            let seed_clone = seed.clone();
            thread::spawn(move || {
                let index_read = index_clone.read().unwrap();
                match index_name_clone.as_str() {
                    INDEX_CATALOG => sender.send(index_read.put(key_clone, seed_clone, force)),
                    INDEX_SEQUENCE => {
                        if remove {
                            sender.send(index_read.del(key_clone.clone()))
                        } else {
                            sender.send(index_read.put(key_clone.clone(), seed_clone, force))
                        }
                    }
                    INDEX_MEMORY => {
                        if remove {
                            sender.send(index_read.del(key_clone.clone()))
                        } else {
                            sender.send(index_read.put(
                                key_clone.clone(),
                                Arc::new(RwLock::new(SeedMemory::create(key_clone, value_clone))),
                                force,
                            ))
                        }
                    }
                    _ => match key_fetch(index_name_clone, value_clone) {
                        Ok(res) => sender.send(index_read.put(res, seed_clone, force)),
                        Err(err) => {
                            log::debug!("key fetch error: {}", err);
                            sender.send(Ok(()))
                        }
                    },
                }
            });
        }
        for receive in receives.iter() {
            let res = receive.recv();
            match res {
                Ok(gr) => match gr {
                    Err(err) => return Err(err),
                    _ => {}
                },
                Err(err) => return Err(err_string(err.to_string())),
            }
        }
        if remove {
            seed.write().unwrap().remove()
        } else {
            seed.write().unwrap().save(self.clone())
        }
    }
}

impl View {
    /// 生成文件描述
    fn description(&self) -> Vec<u8> {
        hex::encode(format!(
            "{}:#?{}:#?{}:#?{}",
            self.name(),
            self.mem,
            self.create_time().num_nanoseconds().unwrap().to_string(),
            self.pigeonhole().to_string()
        ))
        .into_bytes()
    }
    /// 通过文件描述恢复结构信息
    pub(crate) fn recover(database_name: String, hd: HD) -> GeorgeResult<View> {
        let description_str = Strings::from_utf8(hd.description())?;
        match hex::decode(description_str) {
            Ok(vu8) => {
                let real = Strings::from_utf8(vu8)?;
                let mut split = real.split(":#?");
                let name = split.next().unwrap().to_string();
                let mem = split.next().unwrap().to_string().parse::<bool>().unwrap();
                let create_time = Duration::nanoseconds(
                    split.next().unwrap().to_string().parse::<i64>().unwrap(),
                );
                let pigeonhole = Pigeonhole::from_string(split.next().unwrap().to_string())?;
                let filepath = view_filepath(database_name.clone(), name.clone());
                let mut view = View {
                    database_name: database_name.clone(),
                    name,
                    create_time,
                    metadata: hd.metadata(),
                    filer: Filed::recovery(filepath)?,
                    mem,
                    indexes: Arc::new(Default::default()),
                    pigeonhole,
                };
                log::info!(
                    "recovery view {} from database {}",
                    view.name(),
                    database_name,
                );
                match read_dir(view_path(database_name.clone(), view.name())) {
                    // 恢复indexes数据
                    Ok(paths) => view.recovery_indexes(database_name, paths)?,
                    Err(err) => panic!("recovery view read dir failed! error is {}", err),
                }
                Ok(view)
            }
            Err(err) => Err(err_string(format!(
                "recovery view decode failed! error is {}",
                err
            ))),
        }
    }
    /// 恢复indexes数据
    fn recovery_indexes(&mut self, database_name: String, paths: ReadDir) -> GeorgeResult<()> {
        // 遍历view目录下文件
        for path in paths {
            match path {
                // 所有目录文件被默认为index根目录
                Ok(dir) => {
                    if dir.path().is_dir() {
                        let index_name = dir.file_name().to_str().unwrap().to_string();
                        log::debug!("recovery index from {}", index_name);
                        // 恢复index数据
                        self.recovery_index(database_name.clone(), index_name.clone())?;
                    }
                }
                Err(err) => panic!("recovery indexes path failed! error is {}", err),
            }
        }
        Ok(())
    }

    /// 恢复view数据
    fn recovery_index(&self, database_name: String, index_name: String) -> GeorgeResult<()> {
        let index_file_path =
            index_filepath(database_name.clone(), self.name(), index_name.clone());
        let db_name = database_name.clone();
        let view_name = self.name();
        let hd = recovery_before_content(index_file_path.clone())?;
        let metadata = hd.metadata();
        let index;
        // 恢复index数据
        match hd.index_type() {
            IndexType::None => panic!("index engine type error"),
            _ => index = IndexDefault::recover(database_name, view_name, hd)?,
        }
        log::debug!(
            "index [db={}, view={}, name={}, create_time={}, {:#?}]",
            db_name,
            self.name(),
            index_name.clone(),
            index
                .clone()
                .read()
                .unwrap()
                .create_time()
                .num_nanoseconds()
                .unwrap()
                .to_string(),
            metadata
        );
        // 如果已存在该view，则不处理
        if !self.exist_index(index_name.clone()) {
            self.index_map().write().unwrap().insert(index_name, index);
        }
        Ok(())
    }
}

/// 归档服务
#[derive(Clone)]
pub(crate) struct Pigeonhole {
    now: Record,
    history: HashMap<u16, Record>,
}

impl fmt::Debug for Pigeonhole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut histories = String::from("");
        for (_, his) in self.history.iter() {
            histories = histories.add(his.to_string().as_str());
        }
        write!(f, "[now = {:#?}, histories = {:#?}]", self.now, histories)
    }
}

impl Pigeonhole {
    fn create(version: u16, filepath: String, create_time: Duration) -> Pigeonhole {
        Pigeonhole {
            now: Record {
                version,
                filepath,
                create_time,
            },
            history: Default::default(),
        }
    }
    /// 当前归档版本
    pub(crate) fn now(&self) -> Record {
        self.now.clone()
    }
    /// 历史归档版本
    pub(crate) fn history(&self) -> HashMap<u16, Record> {
        self.history.clone()
    }
    fn history_to_string(&self) -> String {
        let mut res = String::from("");
        for (_, record) in self.history.iter() {
            if res.is_empty() {
                res = res.add(&record.to_string());
            } else {
                res = res.add("@_@!");
                res = res.add(&record.to_string());
            }
        }
        res
    }
    fn history_from_string(history_desc: String) -> GeorgeResult<HashMap<u16, Record>> {
        let mut history: HashMap<u16, Record> = Default::default();
        if !history_desc.is_empty() {
            let split = history_desc.split("$_$!");
            for record_desc in split.into_iter() {
                let record = Record::from_string(String::from(record_desc))?;
                history.insert(record.version, record);
            }
        }
        Ok(history)
    }
    /// 生成文件描述
    fn to_string(&self) -> String {
        hex::encode(format!(
            "{}$_$!{}",
            self.now().to_string(),
            self.history_to_string()
        ))
    }
    /// 通过文件描述恢复结构信息
    pub(crate) fn from_string(pigeonhole_desc: String) -> GeorgeResult<Pigeonhole> {
        match hex::decode(pigeonhole_desc) {
            Ok(vu8) => {
                let real = Strings::from_utf8(vu8)?;
                let mut split = real.split("$_$!");
                let now = Record::from_string(split.next().unwrap().to_string())?;
                let history = Pigeonhole::history_from_string(split.next().unwrap().to_string())?;
                Ok(Pigeonhole { now, history })
            }
            Err(err) => Err(err_string(format!(
                "recovery pigeonhole from utf8 1 failed! error is {}",
                err
            ))),
        }
    }
}

/// 归档记录
#[derive(Clone)]
pub(crate) struct Record {
    /// 归档版本，默认新建为[0x00,0x00]，版本每次归档操作递增，最多归档65536次
    version: u16,
    /// 当前归档版本文件所处路径
    filepath: String,
    /// 归档时间
    create_time: Duration,
}

impl fmt::Debug for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let time_from_stamp = NaiveDateTime::from_timestamp(self.create_time().num_seconds(), 0);
        let time_format = time_from_stamp.format("%Y-%m-%d %H:%M:%S");
        write!(
            f,
            "[version = {:#?}, filepath = {}, create_time = {}]",
            self.version(),
            self.filepath(),
            time_format
        )
    }
}

impl Record {
    fn create(version: u16, filepath: String, create_time: Duration) -> Record {
        Record {
            version,
            filepath,
            create_time,
        }
    }
    /// 归档版本，默认新建为[0x00,0x00]，版本每次归档操作递增，最多归档65536次
    pub(crate) fn version(&self) -> u16 {
        self.version
    }
    /// 当前归档版本文件所处路径
    pub(crate) fn filepath(&self) -> String {
        self.filepath.clone()
    }
    /// 归档时间
    pub(crate) fn create_time(&self) -> Duration {
        self.create_time.clone()
    }
    /// 生成文件描述
    fn to_string(&self) -> String {
        hex::encode(format!(
            "{}|{}|{}",
            self.version(),
            self.filepath(),
            self.create_time().num_nanoseconds().unwrap().to_string()
        ))
    }
    /// 通过文件描述恢复结构信息
    pub(crate) fn from_string(record_desc: String) -> GeorgeResult<Record> {
        match hex::decode(record_desc) {
            Ok(vu8) => {
                let real = Strings::from_utf8(vu8)?;
                let mut split = real.split("|");
                let version = split.next().unwrap().to_string().parse::<u16>().unwrap();
                let filepath = split.next().unwrap().to_string();
                let create_time = Duration::nanoseconds(
                    split.next().unwrap().to_string().parse::<i64>().unwrap(),
                );
                Ok(Record::create(version, filepath, create_time))
            }
            Err(err) => Err(err_string(format!(
                "recovery pigeonhole from utf8 1 failed! error is {}",
                err
            ))),
        }
    }
}
