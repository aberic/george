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
use std::fs::{read_dir, ReadDir};
use std::sync::{Arc, RwLock};

use chrono::{Duration, Local, NaiveDateTime};
use tokio::sync::mpsc::Sender;

use comm::errors::{Errs, GeorgeResult};
use comm::io::file::FilerReader;
use comm::io::Filer;
use comm::strings::StringHandler;
use comm::vectors::VectorHandler;
use comm::Strings;
use comm::Trans;
use comm::Vector;

use crate::task::engine::traits::{Pigeonhole, TForm, TIndex, TSeed};
use crate::task::rich::{Expectation, Selector};
use crate::task::Seed;
use crate::task::View;
use crate::task::{Index as IndexDefault, GLOBAL_THREAD_POOL};
use crate::utils::comm::{IndexKey, INDEX_DISK, INDEX_INCREMENT};
use crate::utils::enums::{IndexType, KeyType};
use crate::utils::store::{ContentBytes, Metadata, HD};
use crate::utils::writer::Filed;
use crate::utils::Paths;

/// 新建视图
fn new_view(database_name: String, name: String) -> GeorgeResult<View> {
    let now: NaiveDateTime = Local::now().naive_local();
    let create_time = Duration::nanoseconds(now.timestamp_nanos());
    let filepath = Paths::view_filepath(database_name.clone(), name.clone());
    let metadata = Metadata::view();
    let view = View {
        database_name,
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
fn mock_new_view(database_name: String, name: String) -> GeorgeResult<View> {
    let now: NaiveDateTime = Local::now().naive_local();
    let create_time = Duration::nanoseconds(now.timestamp_nanos());
    let filepath = Paths::view_filepath(database_name.clone(), name.clone());
    let metadata = Metadata::view();
    let view = View {
        database_name,
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
    pub(crate) fn create(
        database_name: String,
        name: String,
        with_sequence: bool,
    ) -> GeorgeResult<Arc<RwLock<View>>> {
        let view_new = new_view(database_name, name)?;
        let view = Arc::new(RwLock::new(view_new));
        view.clone().read().unwrap().init()?;
        view.read().unwrap().create_index(
            view.clone(),
            INDEX_DISK.to_string(),
            IndexType::Disk,
            KeyType::String,
            true,
            true,
            false,
        )?;
        if with_sequence {
            view.read().unwrap().create_index(
                view.clone(),
                INDEX_INCREMENT.to_string(),
                IndexType::Increment,
                KeyType::UInt,
                false,
                true,
                false,
            )?;
        }
        Ok(view)
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

    /// 创建时间
    pub(crate) fn create_time(&self) -> Duration {
        self.create_time.clone()
    }

    /// 文件信息
    pub(crate) fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }

    /// 索引集合
    pub(crate) fn index_map(&self) -> Arc<RwLock<HashMap<String, Arc<dyn TIndex>>>> {
        self.indexes.clone()
    }

    /// 获取索引
    fn index(&self, index_name: &str) -> GeorgeResult<Arc<dyn TIndex>> {
        match self.index_map().read().unwrap().get(index_name) {
            Some(idx) => Ok(idx.clone()),
            None => Err(Errs::string(format!("index {} doesn't found", index_name))),
        }
    }

    /// 当前视图版本号
    fn version(&self) -> u16 {
        self.pigeonhole().now().version()
    }

    /// 当前视图文件地址
    fn filepath(&self) -> String {
        self.pigeonhole().now().filepath()
    }

    /// 文件字节信息
    fn metadata_bytes(&self) -> Vec<u8> {
        self.metadata.bytes()
    }

    /// 当前归档版本信息
    fn pigeonhole(&self) -> Pigeonhole {
        self.pigeonhole.clone()
    }

    /// 当前视图文件地址
    fn filepath_by_version(&self, version: u16) -> GeorgeResult<String> {
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

    fn exist_index(&self, index_name: String) -> bool {
        return match self.index_map().read().unwrap().get(index_name.as_str()) {
            Some(_) => true,
            None => false,
        };
    }

    /// 指定归档版本信息
    ///
    /// #param
    /// * version 版本号
    ///
    /// #return
    /// * filepath 当前归档版本文件所处路径
    /// * create_time 归档时间
    pub(crate) fn record(&self, version: u16) -> GeorgeResult<(String, Duration)> {
        if self.pigeonhole().now().version.eq(&version) {
            let record = self.pigeonhole().now();
            Ok((record.filepath(), record.create_time()))
        } else {
            for (ver, record) in self.pigeonhole().history().iter() {
                if version.eq(ver) {
                    return Ok((record.filepath(), record.create_time()));
                }
            }
            Err(Errs::str("no view version found"))
        }
    }

    /// 整理归档
    ///
    /// archive_file_path 归档路径
    pub(crate) fn archive(&self, archive_file_path: String) -> GeorgeResult<()> {
        self.filer.clone().archive(archive_file_path)?;
        self.init()
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

    /// 创建索引
    ///
    /// ###Params
    /// * view 视图
    /// * index_name 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入
    /// * index_type 存储引擎类型
    /// * key_type 索引值类型
    /// * primary 是否主键，主键也是唯一索引，即默认列表依赖索引
    /// * unique 是否唯一索引
    /// * null 是否允许为空
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
            return Err(Errs::index_exist_error());
        }
        self.index_map().write().unwrap().insert(
            index_name.clone(),
            IndexDefault::create(
                view, index_name, index_type, primary, unique, null, key_type,
            )?,
        );
        Ok(())
    }

    /// 追加写入的写对象
    ///
    /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
    ///
    /// #Return
    ///
    /// seek_end_before 写之前文件字节数据长度
    fn append(&self, content: Vec<u8>) -> GeorgeResult<u64> {
        self.filer.append(content)
    }

    /// 读取`start`起始且持续`last`长度的数据
    fn read(&self, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
        self.filer.read(start, last)
    }

    /// 写入的写对象到指定坐标
    ///
    /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
    fn write(&self, seek: u64, content: Vec<u8>) -> GeorgeResult<()> {
        self.filer.write(seek, content)
    }
}

impl TForm for View {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn database_name(&self) -> String {
        self.database_name.clone()
    }

    fn write_content(&self, value: Vec<u8>) -> GeorgeResult<Vec<u8>> {
        // 内容持续长度(4字节)
        let mut seed_bytes_len_bytes = Trans::u32_2_bytes(value.len() as u32);
        // 将数据存入view，返回数据在view中的起始坐标
        let view_seek_start = self.append(value)?;
        // 记录视图文件属性(版本号/数据归档/定位文件用2字节)+数据在表文件中起始偏移量p(6字节)
        // 数据在视图文件中起始偏移量p(6字节)
        let mut view_seek_start_bytes = Trans::u48_2_bytes(view_seek_start);
        // 生成视图文件属性，版本号(2字节)
        let view_version_bytes = Trans::u16_2_bytes(self.version());
        // 循环定位记录使用文件属性
        let mut view_info_index = view_version_bytes.clone();
        // 记录表文件属性(版本/数据归档/定位文件用2字节)+数据持续长度+数据在表文件中起始偏移量p(6字节)
        // view_info_index = view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节)
        view_info_index.append(&mut seed_bytes_len_bytes);
        view_info_index.append(&mut view_seek_start_bytes);
        Ok(view_info_index)
    }

    fn read_content(&self, version: u16, data_len: u32, seek: u64) -> GeorgeResult<Vec<u8>> {
        let filepath = self.filepath_by_version(version)?;
        Filer::read_sub(filepath, seek, data_len as usize)
    }

    fn read_content_by_info(&self, view_info_index: Vec<u8>) -> GeorgeResult<Vec<u8>> {
        // 读取view版本号(2字节)
        let version = Trans::bytes_2_u16(Vector::sub(view_info_index.clone(), 0, 2)?)?;
        // 读取view持续长度(4字节)
        let data_len = Trans::bytes_2_u32(Vector::sub(view_info_index.clone(), 2, 6)?)?;
        // 读取view偏移量(6字节)
        let seek = Trans::bytes_2_u48(Vector::sub(view_info_index.clone(), 6, 12)?)?;
        let filepath = self.filepath_by_version(version)?;
        Filer::read_sub(filepath, seek, data_len as usize)
    }

    fn rm(&self, key: String, value: Vec<u8>) -> GeorgeResult<()> {
        self.remove(key, value)
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
        GLOBAL_THREAD_POOL.task_block_on(self.save(key, value, false))
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
        GLOBAL_THREAD_POOL.task_block_on(self.save(key, value, true))
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
        Ok(index.get(key.clone())?.value())
    }

    /// 删除数据<p><p>
    ///
    /// ###Params
    ///
    /// key string<p><p>
    ///
    /// ###Return
    ///
    /// GeorgeResult<()>
    pub(crate) fn remove(&self, key: String, value: Vec<u8>) -> GeorgeResult<()> {
        let real = self.index(INDEX_DISK)?.get(key.clone())?;
        GLOBAL_THREAD_POOL.task_block_on(self.del(key, real.increment, value))
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
    async fn save(&self, key: String, value: Vec<u8>, force: bool) -> GeorgeResult<()> {
        let seed = Seed::create(Arc::new(self.clone()), key.clone(), value.clone());
        let mut receives = Vec::new();
        for (index_name, index) in self.index_map().read().unwrap().iter() {
            let (sender, receive) = tokio::sync::mpsc::channel(32);
            receives.push(receive);
            GLOBAL_THREAD_POOL.spawn(self.clone().index_put_exec(
                index_name.clone(),
                index.clone(),
                key.clone(),
                value.clone(),
                seed.clone(),
                force,
                sender,
            ));
        }
        for receive in receives.iter_mut() {
            let message = receive.recv().await;
            match message {
                Some(res) => match res {
                    Err(err) => return Err(err),
                    _ => {}
                },
                _ => {}
            }
        }
        let seed_w = seed.write().unwrap();
        seed_w.save()
    }

    async fn index_put_exec(
        self,
        index_name: String,
        index: Arc<dyn TIndex>,
        key: String,
        value: Vec<u8>,
        seed: Arc<RwLock<Seed>>,
        force: bool,
        sender: Sender<GeorgeResult<()>>,
    ) {
        match index_name.as_str() {
            INDEX_DISK => {
                self.send_put(index_name, index, key, seed, force, sender)
                    .await
            }
            INDEX_INCREMENT => {
                self.send_put(index_name, index, key, seed, force, sender)
                    .await
            }
            _ => match IndexKey::fetch(index_name.clone(), value) {
                Ok(res) => {
                    self.send_put(index_name, index, res, seed, force, sender)
                        .await
                }
                Err(err) => {
                    log::warn!("key fetch error: {}", err);
                    match sender.send(Ok(())).await {
                        Err(err) => {
                            log::error!(
                                "sender send put error in database {} view {} index {} while exec key {} {}",
                                self.database_name(),
                                self.name(),
                                index_name,
                                key,
                                err
                            );
                        }
                        _ => {}
                    }
                }
            },
        }
    }

    async fn send_put(
        self,
        index_name: String,
        index: Arc<dyn TIndex>,
        key: String,
        seed: Arc<RwLock<Seed>>,
        force: bool,
        sender: Sender<GeorgeResult<()>>,
    ) {
        match sender.send(index.put(key.clone(), seed, force)).await {
            Err(err) => {
                log::error!(
                    "sender send put error in database {} view {} index {} while exec key {} {}",
                    self.database_name(),
                    self.name(),
                    index_name,
                    key,
                    err
                );
            }
            _ => {}
        }
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
    async fn del(&self, key: String, increment: u64, value: Vec<u8>) -> GeorgeResult<()> {
        let seed = Seed::create_cus(
            Arc::new(self.clone()),
            key.clone(),
            increment,
            value.clone(),
        );
        let mut receives = Vec::new();
        for (index_name, index) in self.index_map().read().unwrap().iter() {
            let (sender, receive) = tokio::sync::mpsc::channel(32);
            receives.push(receive);
            GLOBAL_THREAD_POOL.spawn(self.clone().index_del_exec(
                index_name.clone(),
                index.clone(),
                key.clone(),
                value.clone(),
                seed.clone(),
                sender,
            ));
        }
        for receive in receives.iter_mut() {
            let message = receive.recv().await;
            match message {
                Some(res) => match res {
                    Err(err) => return Err(err),
                    _ => {}
                },
                _ => {}
            }
        }
        let seed_w = seed.write().unwrap();
        seed_w.remove()
    }

    async fn index_del_exec(
        self,
        index_name: String,
        index: Arc<dyn TIndex>,
        key: String,
        value: Vec<u8>,
        seed: Arc<RwLock<Seed>>,
        sender: Sender<GeorgeResult<()>>,
    ) {
        match index_name.as_str() {
            INDEX_DISK => self.send_del(index_name, index, key, seed, sender).await,
            INDEX_INCREMENT => self.send_del(index_name, index, key, seed, sender).await,
            _ => match IndexKey::fetch(index_name.clone(), value) {
                Ok(res) => self.send_del(index_name, index, res, seed, sender).await,
                Err(err) => {
                    log::warn!("key fetch error: {}", err);
                    match sender.send(Ok(())).await {
                        Err(err) => {
                            log::error!(
                                "sender send del error in database {} view {} index {} while exec key {} {}",
                                self.database_name(),
                                self.name(),
                                index_name,
                                key,
                                err
                            );
                        }
                        _ => {}
                    }
                }
            },
        }
    }

    async fn send_del(
        self,
        index_name: String,
        index: Arc<dyn TIndex>,
        key: String,
        seed: Arc<RwLock<Seed>>,
        sender: Sender<GeorgeResult<()>>,
    ) {
        match sender.send(index.del(key.clone(), seed)).await {
            Err(err) => {
                log::error!(
                    "sender send del error in database {} view {} index {} while exec key {} {}",
                    self.database_name(),
                    self.name(),
                    index_name,
                    key,
                    err
                );
            }
            _ => {}
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
    pub(crate) fn recover(
        database_name: String,
        hd: HD,
    ) -> GeorgeResult<(String, Arc<RwLock<View>>)> {
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
                        log::debug!(
                            "view [db={}, name={}, create_time={}, pigeonhole={:#?}, {:#?}]",
                            view.database_name(),
                            view.name(),
                            view.create_time().num_nanoseconds().unwrap().to_string(),
                            view.pigeonhole(),
                            hd.metadata()
                        );
                        Ok((view.name(), view_bak))
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
