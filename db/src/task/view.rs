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

use comm::errors::children::IndexExistError;
use comm::errors::entrances::{err_str, err_string, err_strs, GeorgeError, GeorgeResult};
use comm::io::file::{Filer, FilerNormal};
use comm::strings::{StringHandler, Strings};
use comm::trans::{trans_bytes_2_u16, trans_bytes_2_u32, trans_bytes_2_u48, trans_u32_2_bytes};
use comm::vectors::{Vector, VectorHandler};

use crate::task::engine::traits::{TIndex, TSeed};
use crate::task::engine::DataReal;
use crate::task::index::Index as IndexDefault;
use crate::task::rich::{Expectation, Selector};
use crate::task::seed::Seed;
use crate::utils::comm::{hash_key, key_fetch, INDEX_CATALOG, INDEX_SEQUENCE};
use crate::utils::enums::{IndexType, KeyType};
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
    filer: Filed,
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
fn new_view(database_name: String, name: String) -> GeorgeResult<View> {
    let now: NaiveDateTime = Local::now().naive_local();
    let create_time = Duration::nanoseconds(now.timestamp_nanos());
    let filepath = view_filepath(database_name.clone(), name.clone());
    let metadata = Metadata::view_disk();
    let view = View {
        database_name: database_name.clone(),
        name,
        create_time,
        metadata,
        filer: Filed::create(filepath.clone())?,
        indexes: Default::default(),
        pigeonhole: Pigeonhole::create(0, filepath, create_time),
    };
    // view.create_index(
    //     INDEX_CATALOG.to_string(),
    //     IndexType::Library,
    //     KeyType::String,
    //     true,
    //     true,
    //     false,
    // )?;
    view.create_index(
        INDEX_SEQUENCE.to_string(),
        IndexType::Dossier,
        KeyType::U64,
        false,
        true,
        false,
    )?;

    Ok(view)
}

impl View {
    pub(crate) fn create(database_name: String, name: String) -> GeorgeResult<Arc<RwLock<View>>> {
        let mut view = new_view(database_name, name)?;
        view.init()?;
        Ok(Arc::new(RwLock::new(view)))
    }
    fn init(&mut self) -> GeorgeResult<()> {
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
    fn file_append(&mut self, content: Vec<u8>) -> GeorgeResult<u64> {
        self.filer.append(content)
    }
    /// 视图变更
    pub(crate) fn modify(&mut self, database_name: String, name: String) -> GeorgeResult<()> {
        let old_name = self.name();
        let content_old = self.filer.read(0, 40)?;
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
        self.filer.write(32, content_new)?;
        let view_path_old = view_path(database_name.clone(), old_name);
        let view_path_new = view_path(database_name.clone(), self.name());
        match std::fs::rename(view_path_old, view_path_new) {
            Ok(_) => {
                self.database_name = database_name;
                for (_name, index) in self.index_map().write().unwrap().iter() {
                    index.write().unwrap().modify()?
                }
                Ok(())
            }
            Err(err) => {
                // 回滚数据
                self.filer.write(0, content_old)?;
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
        index_name: String,
        index_type: IndexType,
        key_type: KeyType,
        primary: bool,
        unique: bool,
        null: bool,
    ) -> GeorgeResult<()> {
        if self.exist_index(index_name.clone()) {
            return Err(GeorgeError::from(IndexExistError));
        }
        self.index_map().write().unwrap().insert(
            index_name.clone(),
            IndexDefault::create(
                self.clone(),
                index_name,
                index_type,
                primary,
                unique,
                null,
                key_type,
            )?,
        );
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
        let index_read = index.read().unwrap();
        let hash_key = hash_key(index_read.key_type(), key.clone())?;
        let view_info_index = index_read.get(key.clone(), hash_key)?;
        match index_name {
            INDEX_CATALOG => Ok(view_info_index),
            _ => DataReal::value_bytes(self.read_content_by(view_info_index)?),
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
    pub(crate) fn archive(&mut self, archive_file_path: String) -> GeorgeResult<()> {
        self.filer.archive(archive_file_path)?;
        self.init()
    }
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
    pub(crate) fn write_content(&mut self, mut value: Vec<u8>) -> GeorgeResult<u64> {
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
    /// 读取已组装写入视图的内容，即持续长度+该长度的原文内容
    ///
    /// 在view中的起始偏移量坐标读取数据
    ///
    /// seek 读取偏移量
    pub(crate) fn read_content_by(&self, res: Vec<u8>) -> GeorgeResult<Vec<u8>> {
        let version = trans_bytes_2_u16(Vector::sub(res.clone(), 0, 2)?)?;
        let seek = trans_bytes_2_u48(Vector::sub(res, 2, 8)?)?;
        let filepath = self.filepath_by_version(version)?;
        self.read_content(filepath, seek)
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
        let seed = Seed::create(self.clone(), key.clone(), value.clone(), remove);
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
                    INDEX_CATALOG => match hash_key(index_read.key_type(), key_clone.clone()) {
                        Ok(hash_key) => {
                            if remove {
                                sender.send(index_read.del(key_clone, hash_key))
                            } else {
                                sender.send(index_read.put(key_clone, hash_key, seed_clone, force))
                            }
                        }
                        Err(err) => sender.send(GeorgeResult::Err(err)),
                    },
                    INDEX_SEQUENCE => {
                        if remove {
                            match hash_key(index_read.key_type(), key_clone.clone()) {
                                Ok(hash_key) => sender.send(index_read.del(key_clone, hash_key)),
                                Err(err) => sender.send(GeorgeResult::Err(err)),
                            }
                        } else {
                            sender.send(index_read.put(key_clone, 0, seed_clone, force))
                        }
                    }
                    // INDEX_MEMORY => match hash_key(index_read.key_type(), key_clone.clone()) {
                    //     Ok(hash_key) => {
                    //         if remove {
                    //             sender.send(index_read.del(key_clone, hash_key))
                    //         } else {
                    //             sender.send(index_read.put(
                    //                 hash_key,
                    //                 Arc::new(RwLock::new(SeedMemory::create(
                    //                     key_clone,
                    //                     value_clone,
                    //                 ))),
                    //                 force,
                    //             ))
                    //         }
                    //     }
                    //     Err(err) => sender.send(GeorgeResult::Err(err)),
                    // },
                    _ => match key_fetch(index_name_clone, value_clone) {
                        Ok(res) => match hash_key(index_read.key_type(), res.clone()) {
                            Ok(hash_key) => {
                                if remove {
                                    sender.send(index_read.del(res, hash_key))
                                } else {
                                    sender.send(index_read.put(res, hash_key, seed_clone, force))
                                }
                            }
                            Err(err) => sender.send(GeorgeResult::Err(err)),
                        },
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
            seed.write().unwrap().save()
        }
    }
}

impl View {
    /// 生成文件描述
    fn description(&self) -> Vec<u8> {
        hex::encode(format!(
            "{}:#?{}:#?{}",
            self.name(),
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
                    indexes: Arc::new(Default::default()),
                    pigeonhole,
                };
                log::info!(
                    "recovery view {} from database {}",
                    view.name(),
                    database_name,
                );
                match read_dir(view_path(database_name, view.name())) {
                    // 恢复indexes数据
                    Ok(paths) => view.recovery_indexes(paths)?,
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
    fn recovery_indexes(&mut self, paths: ReadDir) -> GeorgeResult<()> {
        // 遍历view目录下文件
        for path in paths {
            match path {
                // 所有目录文件被默认为index根目录
                Ok(dir) => {
                    if dir.path().is_dir() {
                        let index_name = dir.file_name().to_str().unwrap().to_string();
                        log::debug!("recovery index from {}", index_name);
                        // 恢复index数据
                        self.recovery_index(index_name.clone())?;
                    }
                }
                Err(err) => panic!("recovery indexes path failed! error is {}", err),
            }
        }
        Ok(())
    }

    /// 恢复view数据
    fn recovery_index(&self, index_name: String) -> GeorgeResult<()> {
        let index_file_path = index_filepath(self.database_name(), self.name(), index_name.clone());
        let hd = recovery_before_content(index_file_path.clone())?;
        let metadata = hd.metadata();
        let index;
        // 恢复index数据
        match hd.index_type() {
            IndexType::None => panic!("index engine type error"),
            _ => index = IndexDefault::recover(self.clone(), hd)?,
        }
        log::debug!(
            "index [db={}, view={}, name={}, create_time={}, {:#?}]",
            self.database_name(),
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
