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
use comm::trans::{trans_bytes_2_u16, trans_bytes_2_u64, trans_u32_2_bytes};

use crate::task::engine::dossier::index::Index;
use crate::task::engine::traits::{TIndex, TSeed};
use crate::task::seed::Seed;
use crate::utils::comm::{key_fetch, INDEX_CATALOG};
use crate::utils::enums::{EngineType, IndexMold, Tag};
use crate::utils::path::{index_file_path, view_file_path, view_path};
use crate::utils::store::{
    before_content_bytes, metadata_2_bytes, recovery_before_content, Metadata, HD,
};
use crate::utils::writer::Filed;
use comm::io::file::{Filer, FilerReader, FilerWriter};
use comm::vectors::{Vector, VectorHandler};

/// 视图，类似表
#[derive(Debug, Clone)]
pub(crate) struct View {
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
/// id 视图唯一ID
///
/// name 视图名称
///
/// comment 视图描述
///
/// category 视图类型
///
/// level 视图规模/级别
fn new_view(database_name: String, name: String) -> GeorgeResult<View> {
    let now: NaiveDateTime = Local::now().naive_local();
    let create_time = Duration::nanoseconds(now.timestamp_nanos());
    let file_path = view_file_path(database_name.clone(), name.clone());
    let view = View {
        name,
        create_time,
        metadata: Metadata::default(Tag::View),
        filer: Filed::create(file_path.clone())?,
        indexes: Default::default(),
        pigeonhole: Pigeonhole::create(0, file_path, create_time),
    };
    view.create_index(
        database_name,
        INDEX_CATALOG.to_string(),
        EngineType::Library,
        IndexMold::String,
        true,
    )?;
    Ok(view)
}

impl View {
    pub(crate) fn create(database_name: String, name: String) -> GeorgeResult<Arc<RwLock<View>>> {
        let view = new_view(database_name.clone(), name)?;
        view.init(database_name)?;
        Ok(Arc::new(RwLock::new(view)))
    }
    fn init(&self, database_name: String) -> GeorgeResult<()> {
        let mut metadata_bytes = metadata_2_bytes(self.metadata());
        let mut description = self.description();
        // 初始化为32 + 8，即head长度加正文描述符长度
        let mut before_description = before_content_bytes(40, description.len() as u32);
        metadata_bytes.append(&mut before_description);
        metadata_bytes.append(&mut description);
        self.file_append(database_name, metadata_bytes)?;
        Ok(())
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
    /// 索引集合
    pub(crate) fn index_map(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<dyn TIndex>>>>> {
        self.indexes.clone()
    }
    /// 当前归档版本信息
    pub(crate) fn pigeonhole(&self) -> Pigeonhole {
        self.pigeonhole.clone()
    }
    /// 当前视图版本号
    pub(crate) fn version(&self) -> u16 {
        self.pigeonhole().now().version()
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
    fn file_append(&self, database_name: String, content: Vec<u8>) -> GeorgeResult<u64> {
        self.filer.write().unwrap().append(database_name, content)
    }
    /// 视图变更
    pub(crate) fn modify(&mut self, database_name: String, name: String) -> GeorgeResult<()> {
        let old_name = self.name();
        let filepath = view_file_path(database_name.clone(), old_name.clone());
        let content_old = Filer::read_sub(filepath.clone(), 0, 40)?;
        self.name = name;
        let description = self.description();
        let seek_end = self.file_append(database_name.clone(), description.clone())?;
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
            Ok(_) => Ok(()),
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
        engine_type: EngineType,
        index_mold: IndexMold,
        primary: bool,
    ) -> GeorgeResult<()> {
        if self.exist_index(index_name.clone()) {
            return Err(GeorgeError::from(IndexExistError));
        }
        let view_name = self.name();
        let name = index_name.clone();
        let index;
        match engine_type {
            EngineType::None => return Err(err_str("unsupported engine type with none")),
            EngineType::Memory => {
                index = Index::create(database_name, view_name, name, primary, index_mold)?
            }
            EngineType::Dossier => {
                index = Index::create(database_name, view_name, name, primary, index_mold)?
            }
            EngineType::Library => {
                index = Index::create(database_name, view_name, name, primary, index_mold)?
            }
            EngineType::Block => {
                index = Index::create(database_name, view_name, name, primary, index_mold)?
            }
        }
        self.index_map()
            .write()
            .unwrap()
            .insert(index_name, index.clone());
        Ok(())
    }
}

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
    pub(crate) fn put(
        &self,
        database_name: String,
        key: String,
        value: Vec<u8>,
    ) -> GeorgeResult<()> {
        self.save(database_name, key, value, false, false)
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
    pub(crate) fn set(
        &self,
        database_name: String,
        key: String,
        value: Vec<u8>,
    ) -> GeorgeResult<()> {
        self.save(database_name, key, value, true, false)
    }
    /// 获取数据，返回存储对象<p><p>
    ///
    /// ###Params
    ///
    /// key string
    ///
    /// ###Return
    ///
    /// Seed value信息
    pub(crate) fn get(&self, database_name: String, key: String) -> GeorgeResult<Vec<u8>> {
        match self.index_map().read().unwrap().get(INDEX_CATALOG) {
            Some(index) => {
                let view_value_seek_bytes =
                    index
                        .read()
                        .unwrap()
                        .get(database_name, self.name(), key.clone())?;
                let version = trans_bytes_2_u16(Vector::sub(view_value_seek_bytes.clone(), 0, 2));
                let seek = trans_bytes_2_u64(Vector::sub(view_value_seek_bytes, 2, 8));
                let record = self.record(version)?;
                // todo view read method!
                Ok(vec![])
            }
            None => Err(GeorgeError::from(DataNoExistError)),
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
    pub(crate) fn remove(&self, database_name: String, key: String) -> GeorgeResult<()> {
        self.save(database_name, key, vec![], true, true)
    }
}

impl View {
    /// 整理归档
    ///
    /// archive_file_path 归档路径
    pub(crate) fn archive(
        &self,
        database_name: String,
        archive_file_path: String,
    ) -> GeorgeResult<()> {
        self.filer.write().unwrap().archive(archive_file_path)?;
        self.init(database_name)
    }
    /// 组装写入视图的内容，即持续长度+该长度的原文内容
    ///
    /// 将数据存入view，返回数据在view中的起始偏移量坐标
    pub(crate) fn write_content(
        &self,
        database_name: String,
        mut value: Vec<u8>,
    ) -> GeorgeResult<u64> {
        // 内容持续长度(4字节)
        let mut seed_bytes_len_bytes = trans_u32_2_bytes(value.len() as u32);
        // 真实存储内容，内容持续长度(4字节)+内容字节数组
        seed_bytes_len_bytes.append(&mut value);
        // 将数据存入view，返回数据在view中的起始坐标
        self.file_append(database_name, seed_bytes_len_bytes)
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
    fn save(
        &self,
        database_name: String,
        key: String,
        value: Vec<u8>,
        force: bool,
        remove: bool,
    ) -> GeorgeResult<()> {
        let seed = Arc::new(RwLock::new(Seed::create(key.clone())));
        let mut receives = Vec::new();
        for (index_name, index) in self.index_map().read().unwrap().iter() {
            let (sender, receive) = mpsc::channel();
            receives.push(receive);
            let database_name_clone = database_name.clone();
            let view_name = self.name();
            let index_name_clone = index_name.clone();
            let index_clone = index.clone();
            let key_clone = key.clone();
            let value_clone = value.clone();
            let seed_clone = seed.clone();
            thread::spawn(move || {
                let index_read = index_clone.read().unwrap();
                match index_name_clone.as_str() {
                    INDEX_CATALOG => sender.send(index_read.put(
                        database_name_clone,
                        view_name,
                        key_clone,
                        seed_clone,
                    )),
                    _ => match key_fetch(index_name_clone, value_clone) {
                        Ok(res) => sender.send(index_read.put(
                            database_name_clone,
                            view_name,
                            res,
                            seed_clone,
                        )),
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
            seed.write().unwrap().remove(database_name, self.clone())
        } else {
            seed.write()
                .unwrap()
                .save(database_name, self.clone(), value, force)
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
        match String::from_utf8(hd.description()) {
            Ok(description_str) => match hex::decode(description_str) {
                Ok(vu8) => match String::from_utf8(vu8) {
                    Ok(real) => {
                        let mut split = real.split(":#?");
                        let name = split.next().unwrap().to_string();
                        let create_time = Duration::nanoseconds(
                            split.next().unwrap().to_string().parse::<i64>().unwrap(),
                        );
                        let pigeonhole =
                            Pigeonhole::from_string(split.next().unwrap().to_string())?;
                        let file_path = view_file_path(database_name.clone(), name.clone());
                        let mut view = View {
                            name,
                            create_time,
                            metadata: hd.metadata(),
                            filer: Filed::recovery(file_path)?,
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
                            Ok(paths) => view.recovery_indexes(database_name, paths),
                            Err(err) => panic!("recovery view read dir failed! error is {}", err),
                        }
                        Ok(view)
                    }
                    Err(err) => Err(err_string(format!(
                        "recovery index from utf8 2 failed! error is {}",
                        err
                    ))),
                },
                Err(err) => Err(err_string(format!(
                    "recovery view decode failed! error is {}",
                    err
                ))),
            },
            Err(err) => Err(err_string(format!(
                "recovery index from utf8 1 failed! error is {}",
                err
            ))),
        }
    }
    /// 恢复indexes数据
    fn recovery_indexes(&mut self, database_name: String, paths: ReadDir) {
        // 遍历view目录下文件
        for path in paths {
            match path {
                // 所有目录文件被默认为index根目录
                Ok(dir) => {
                    if dir.path().is_dir() {
                        let index_name = dir.file_name().to_str().unwrap().to_string();
                        log::debug!("recovery index from {}", index_name);
                        // 恢复index数据
                        self.recovery_index(database_name.clone(), index_name.clone());
                    }
                }
                Err(err) => panic!("recovery indexes path failed! error is {}", err),
            }
        }
    }

    /// 恢复view数据
    fn recovery_index(&self, database_name: String, index_name: String) {
        let index_file_path =
            index_file_path(database_name.clone(), self.name(), index_name.clone());
        match recovery_before_content(index_file_path.clone()) {
            Ok(hd) => {
                // 恢复view数据
                match Index::recover(database_name.clone(), self.name(), hd.clone()) {
                    Ok(index) => {
                        log::debug!(
                            "index [db={}, view={}, name={}, create_time={}, {:#?}]",
                            database_name.clone(),
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
                            hd.metadata()
                        );
                        // 如果已存在该view，则不处理
                        if self.exist_index(index_name.clone()) {
                            return;
                        }
                        self.index_map().write().unwrap().insert(index_name, index);
                    }
                    Err(err) => panic!("recovery index failed! error is {}", err),
                }
            }
            Err(err) => panic!(
                "recovery index when recovery before content failed! error is {}",
                err
            ),
        }
    }
}

/// 归档服务
#[derive(Debug, Clone)]
pub(crate) struct Pigeonhole {
    now: Record,
    history: HashMap<u16, Record>,
}

impl Pigeonhole {
    fn create(version: u16, file_path: String, create_time: Duration) -> Pigeonhole {
        Pigeonhole {
            now: Record {
                version,
                file_path,
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
            Ok(vu8) => match String::from_utf8(vu8) {
                Ok(real) => {
                    let mut split = real.split("$_$!");
                    let now = Record::from_string(split.next().unwrap().to_string())?;
                    let history =
                        Pigeonhole::history_from_string(split.next().unwrap().to_string())?;
                    Ok(Pigeonhole { now, history })
                }
                Err(err) => Err(err_string(format!(
                    "recovery pigeonhole from utf8 2 failed! error is {}",
                    err
                ))),
            },
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
    file_path: String,
    /// 归档时间
    create_time: Duration,
}

impl fmt::Debug for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let time_from_stamp = NaiveDateTime::from_timestamp(self.create_time().num_seconds(), 0);
        let time_format = time_from_stamp.format("%Y-%m-%d %H:%M:%S");
        write!(
            f,
            "[version = {:#?}, file_path = {}, create_time = {}]",
            self.version(),
            self.file_path(),
            time_format
        )
    }
}

impl Record {
    fn create(version: u16, file_path: String, create_time: Duration) -> Record {
        Record {
            version,
            file_path,
            create_time,
        }
    }
    /// 归档版本，默认新建为[0x00,0x00]，版本每次归档操作递增，最多归档65536次
    pub(crate) fn version(&self) -> u16 {
        self.version
    }
    /// 当前归档版本文件所处路径
    pub(crate) fn file_path(&self) -> String {
        self.file_path.clone()
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
            self.file_path(),
            self.create_time().num_nanoseconds().unwrap().to_string()
        ))
    }
    /// 通过文件描述恢复结构信息
    pub(crate) fn from_string(record_desc: String) -> GeorgeResult<Record> {
        match hex::decode(record_desc) {
            Ok(vu8) => match String::from_utf8(vu8) {
                Ok(real) => {
                    let mut split = real.split("|");
                    let version = split.next().unwrap().to_string().parse::<u16>().unwrap();
                    let file_path = split.next().unwrap().to_string();
                    let create_time = Duration::nanoseconds(
                        split.next().unwrap().to_string().parse::<i64>().unwrap(),
                    );
                    Ok(Record::create(version, file_path, create_time))
                }
                Err(err) => Err(err_string(format!(
                    "recovery pigeonhole from utf8 2 failed! error is {}",
                    err
                ))),
            },
            Err(err) => Err(err_string(format!(
                "recovery pigeonhole from utf8 1 failed! error is {}",
                err
            ))),
        }
    }
}
