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
use comm::errors::entrances::{Errs, GeorgeError, GeorgeResult};
use comm::io::file::{Filer, FilerReader};
use comm::strings::{StringHandler, Strings};

use crate::task::engine::traits::{TIndex, TSeed};
use crate::task::index::Index as IndexDefault;
use crate::task::rich::{Expectation, Selector};
use crate::task::seed::Seed;
use crate::utils::comm::{IndexKey, INDEX_CATALOG, INDEX_SEQUENCE};
use crate::utils::enums::{IndexType, KeyType};
use crate::utils::path::Paths;
use crate::utils::store::{ContentBytes, Metadata, HD};
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
    indexes: Arc<RwLock<HashMap<String, Arc<dyn TIndex>>>>,
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
    let filepath = Paths::view_filepath(database_name.clone(), name.clone());
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
    Ok(view)
}

/// 新建视图
///
/// 具体传参参考如下定义：<p><p>
///
/// ###Params
///
/// mem 是否为内存视图
fn mock_new_view(database_name: String, name: String) -> GeorgeResult<View> {
    let now: NaiveDateTime = Local::now().naive_local();
    let create_time = Duration::nanoseconds(now.timestamp_nanos());
    let filepath = Paths::view_filepath(database_name.clone(), name.clone());
    let metadata = Metadata::view_disk();
    let view = View {
        database_name: database_name.clone(),
        name,
        create_time,
        metadata,
        filer: Filed::mock(filepath.clone())?,
        indexes: Default::default(),
        pigeonhole: Pigeonhole::create(0, filepath, create_time),
    };
    Ok(view)
}

impl View {
    pub(crate) fn create(database_name: String, name: String) -> GeorgeResult<Arc<RwLock<View>>> {
        let view = new_view(database_name, name)?;
        let view_bak = Arc::new(RwLock::new(view));
        view_bak.clone().read().unwrap().init()?;
        view_bak.read().unwrap().create_index(
            view_bak.clone(),
            INDEX_CATALOG.to_string(),
            IndexType::Disk,
            KeyType::String,
            true,
            true,
            false,
        )?;
        view_bak.read().unwrap().create_index(
            view_bak.clone(),
            INDEX_SEQUENCE.to_string(),
            IndexType::Sequence,
            KeyType::U64,
            false,
            true,
            false,
        )?;
        Ok(view_bak)
    }

    fn init(&self) -> GeorgeResult<()> {
        let mut metadata_bytes = self.metadata_bytes();
        let mut description = self.description();
        // 初始化为32 + 8，即head长度加正文描述符长度
        let mut before_description = ContentBytes::before(44, description.len() as u32);
        metadata_bytes.append(&mut before_description);
        metadata_bytes.append(&mut description);
        self.append(metadata_bytes)?;
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
    pub(crate) fn index_map(&self) -> Arc<RwLock<HashMap<String, Arc<dyn TIndex>>>> {
        self.indexes.clone()
    }

    /// 获取索引
    pub(crate) fn index(&self, index_name: &str) -> GeorgeResult<Arc<dyn TIndex>> {
        match self.index_map().read().unwrap().get(index_name) {
            Some(idx) => Ok(idx.clone()),
            None => Err(Errs::string(format!("index {} doesn't found", index_name))),
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
            Err(Errs::str("no view version found while get view filepath"))
        }
    }

    /// 指定归档版本信息
    ///
    /// version 版本号
    pub(crate) fn record(&self, version: u16) -> GeorgeResult<Record> {
        if self.pigeonhole().now().version.eq(&version) {
            Ok(self.pigeonhole().now())
        } else {
            for (ver, record) in self.pigeonhole().history().iter() {
                if version.eq(ver) {
                    return Ok(record.clone());
                }
            }
            Err(Errs::str("no view version found"))
        }
    }

    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
    ///
    /// #Return
    ///
    /// seek_end_before 写之前文件字节数据长度
    fn append(&self, content: Vec<u8>) -> GeorgeResult<u64> {
        self.filer.append(content)
    }

    fn read(&self, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
        self.filer.read(start, last)
    }

    fn write(&self, seek: u64, content: Vec<u8>) -> GeorgeResult<()> {
        self.filer.write(seek, content)
    }

    /// 视图变更
    pub(crate) fn modify(&mut self, database_name: String, name: String) -> GeorgeResult<()> {
        let old_db_name = self.database_name();
        let old_view_name = self.name();
        let content_old = self.read(0, 44)?;
        self.database_name = database_name.clone();
        self.name = name.clone();
        let description = self.description();
        let seek_end = self.append(description.clone())?;
        log::debug!(
            "view {} modify to {} with file seek_end = {}",
            old_view_name.clone(),
            self.name(),
            seek_end
        );
        let content_new = ContentBytes::before(seek_end, description.len() as u32);
        // 更新首部信息，初始化head为32，描述起始4字节，长度4字节
        self.write(32, content_new)?;
        let view_path_old = Paths::view_path(old_db_name.clone(), old_view_name.clone());
        let view_path_new = Paths::view_path(database_name.clone(), self.name());
        match std::fs::rename(view_path_old, view_path_new) {
            Ok(_) => Ok(()),
            Err(err) => {
                // 回滚数据
                self.write(0, content_old)?;
                Err(Errs::strs("file rename failed", err))
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
        view: Arc<RwLock<View>>,
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
                view, index_name, index_type, primary, unique, null, key_type,
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
        self.save(key, value, false)
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
        self.save(key, value, true)
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
        index.get(key.clone())
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
    pub(crate) fn remove(&self, key: String, value: Vec<u8>) -> GeorgeResult<()> {
        self.del(key, value)
    }

    /// 条件检索
    ///
    /// selector_json_bytes 选择器字节数组，自定义转换策略
    pub(crate) fn select(&self, constraint_json_bytes: Vec<u8>) -> GeorgeResult<Expectation> {
        Selector::run(constraint_json_bytes, self.indexes.clone(), false)
    }

    /// 条件删除
    ///
    /// selector_json_bytes 选择器字节数组，自定义转换策略
    pub(crate) fn delete(&self, constraint_json_bytes: Vec<u8>) -> GeorgeResult<Expectation> {
        Selector::run(constraint_json_bytes, self.indexes.clone(), true)
    }
}

impl View {
    /// 整理归档
    ///
    /// archive_file_path 归档路径
    pub(crate) fn archive(&self, archive_file_path: String) -> GeorgeResult<()> {
        self.filer.clone().archive(archive_file_path)?;
        self.init()
    }

    /// 组装写入视图的内容，即持续长度+该长度的原文内容
    ///
    /// 将数据存入view，返回数据在view中的起始偏移量坐标
    pub(crate) fn write_content(&self, value: Vec<u8>) -> GeorgeResult<u64> {
        // 将数据存入view，返回数据在view中的起始坐标
        self.append(value)
    }

    /// 读取已组装写入视图的内容，根据view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节)
    ///
    /// * version view版本号
    /// * data_len view数据持续长度
    /// * seek view数据偏移量
    pub(crate) fn read_content(
        &self,
        version: u16,
        data_len: u32,
        seek: u64,
    ) -> GeorgeResult<Vec<u8>> {
        let filepath = self.filepath_by_version(version)?;
        Filer::read_sub(filepath, seek, data_len as usize)
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
    fn save(&self, key: String, value: Vec<u8>, force: bool) -> GeorgeResult<()> {
        let seed = Seed::create(self.clone(), key.clone(), value.clone());
        let mut receives = Vec::new();
        for (index_name, index) in self.index_map().read().unwrap().iter() {
            let (sender, receive) = mpsc::channel();
            receives.push(receive);
            let index_name_clone = index_name.clone();
            let index_clone = index.clone();
            let key_clone = key.clone();
            let value_clone = value.clone();
            let seed_clone = seed.clone();
            thread::spawn(move || match index_name_clone.as_str() {
                INDEX_CATALOG => sender.send(index_clone.put(key_clone, seed_clone, force)),
                INDEX_SEQUENCE => sender.send(index_clone.put(key_clone, seed_clone, force)),
                _ => match IndexKey::fetch(index_name_clone, value_clone) {
                    Ok(res) => sender.send(index_clone.put(res, seed_clone, force)),
                    Err(err) => {
                        log::debug!("key fetch error: {}", err);
                        sender.send(Ok(()))
                    }
                },
            });
        }
        for receive in receives.iter() {
            let res = receive.recv();
            match res {
                Ok(gr) => match gr {
                    Err(err) => return Err(err),
                    _ => {}
                },
                Err(err) => return Err(Errs::string(err.to_string())),
            }
        }
        let seed_w = seed.write().unwrap();
        seed_w.save()
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
    fn del(&self, key: String, value: Vec<u8>) -> GeorgeResult<()> {
        let seed = Seed::create(self.clone(), key.clone(), value.clone());
        let mut receives = Vec::new();
        for (index_name, index) in self.index_map().read().unwrap().iter() {
            let (sender, receive) = mpsc::channel();
            receives.push(receive);
            let index_name_clone = index_name.clone();
            let index_clone = index.clone();
            let key_clone = key.clone();
            let value_clone = value.clone();
            let seed_clone = seed.clone();
            thread::spawn(move || match index_name_clone.as_str() {
                INDEX_CATALOG => sender.send(index_clone.del(key_clone, seed_clone)),
                INDEX_SEQUENCE => sender.send(index_clone.del(key_clone, seed_clone)),
                _ => match IndexKey::fetch(index_name_clone, value_clone) {
                    Ok(res) => sender.send(index_clone.del(res, seed_clone)),
                    Err(err) => {
                        log::debug!("key fetch error: {}", err);
                        sender.send(Ok(()))
                    }
                },
            });
        }
        for receive in receives.iter() {
            let res = receive.recv();
            match res {
                Ok(gr) => match gr {
                    Err(err) => return Err(err),
                    _ => {}
                },
                Err(err) => return Err(Errs::string(err.to_string())),
            }
        }
        let seed_w = seed.write().unwrap();
        seed_w.remove()
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
    pub(crate) fn recover(database_name: String, hd: HD) -> GeorgeResult<Arc<RwLock<View>>> {
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
                let filepath = Paths::view_filepath(database_name.clone(), name.clone());
                let view = View {
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
                let view_bak = Arc::new(RwLock::new(view.clone()));
                match read_dir(Paths::view_path(database_name, view.name())) {
                    // 恢复indexes数据
                    Ok(paths) => {
                        view_bak
                            .read()
                            .unwrap()
                            .recovery_indexes(view_bak.clone(), paths)?;
                        Ok(view_bak)
                    }
                    Err(err) => Err(Errs::strs("recovery view read dir", err)),
                }
            }
            Err(err) => Err(Errs::strs("recovery view decode", err)),
        }
    }

    /// 恢复indexes数据
    fn recovery_indexes(&self, view: Arc<RwLock<View>>, paths: ReadDir) -> GeorgeResult<()> {
        // 遍历view目录下文件
        for path in paths {
            match path {
                // 所有目录文件被默认为index根目录
                Ok(dir) => {
                    if dir.path().is_dir() {
                        let index_name = dir.file_name().to_str().unwrap().to_string();
                        log::debug!("recovery index from {}", index_name);
                        // 恢复index数据
                        self.recovery_index(view.clone(), index_name.clone())?;
                    }
                }
                Err(err) => return Err(Errs::strs("recovery indexes path", err)),
            }
        }
        Ok(())
    }

    /// 恢复view数据
    fn recovery_index(&self, view: Arc<RwLock<View>>, index_name: String) -> GeorgeResult<()> {
        let index_file_path =
            Paths::index_filepath(self.database_name(), self.name(), index_name.clone());
        let hd = ContentBytes::recovery(index_file_path.clone())?;
        let metadata = hd.metadata();
        let index;
        // 恢复index数据
        match hd.index_type() {
            IndexType::None => return Err(Errs::str("index engine type error")),
            _ => index = IndexDefault::recover(view, hd)?,
        }
        log::debug!(
            "index [db={}, view={}, name={}, create_time={}, {:#?}]",
            self.database_name(),
            self.name(),
            index_name.clone(),
            index.create_time().num_nanoseconds().unwrap().to_string(),
            metadata
        );
        // 如果已存在该view，则不处理
        if !self.exist_index(index_name.clone()) {
            self.index_map().write().unwrap().insert(index_name, index);
        }
        Ok(())
    }
}

impl View {
    pub(crate) fn mock_create(
        database_name: String,
        name: String,
    ) -> GeorgeResult<Arc<RwLock<View>>> {
        let view = mock_new_view(database_name, name)?;
        let view_bak = Arc::new(RwLock::new(view));
        view_bak.clone().read().unwrap().init()?;
        Ok(view_bak)
    }

    pub(crate) fn mock_create_single(database_name: String, name: String) -> GeorgeResult<View> {
        let view = mock_new_view(database_name, name)?;
        view.init()?;
        Ok(view)
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
            Err(err) => Err(Errs::string(format!(
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
            Err(err) => Err(Errs::string(format!(
                "recovery pigeonhole from utf8 1 failed! error is {}",
                err
            ))),
        }
    }
}
