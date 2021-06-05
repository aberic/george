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

use chrono::Duration;
use tokio::sync::mpsc::Sender;

use comm::errors::{Errs, GeorgeResult};
use comm::io::file::{FilerHandler, FilerReader};
use comm::io::Filer;
use comm::strings::StringHandler;
use comm::vectors::VectorHandler;
use comm::Trans;
use comm::Vector;
use comm::{Strings, Time};
use ge::utils::enums::Tag;
use ge::GeFactory;

use crate::task::engine::traits::{Pigeonhole, TIndex, TSeed};
use crate::task::rich::{Expectation, Selector};
use crate::task::traits::TForm;
use crate::task::Seed;
use crate::task::View;
use crate::task::{Index as IndexDefault, GLOBAL_THREAD_POOL};
use crate::utils::comm::{IndexKey, INDEX_DISK, INDEX_INCREMENT};
use crate::utils::enums::{Engine, KeyType};
use crate::utils::Paths;

/// 新建视图
fn new_view(database_name: String, name: String, comment: String) -> GeorgeResult<View> {
    let time = Time::now();
    let filepath = Paths::view_filepath(database_name.clone(), name.clone());
    let pigeonhole = Pigeonhole::create(0, filepath.clone(), time);
    let description = Some(View::description(
        name.clone(),
        comment.clone(),
        time,
        pigeonhole.clone(),
    ));
    let view = View {
        database_name,
        name,
        comment,
        create_time: time,
        ge: GeFactory {}.create(Tag::View, filepath, description)?,
        indexes: Default::default(),
        pigeonhole,
    };
    Ok(view)
}

impl View {
    /// 新建视图
    ///
    /// ##param
    /// * with_increment 是否带自增ID
    pub(crate) fn create(
        database_name: String,
        name: String,
        comment: String,
        with_increment: bool,
    ) -> GeorgeResult<Arc<RwLock<View>>> {
        let view_new = new_view(database_name, name, comment)?;
        let view = Arc::new(RwLock::new(view_new));
        view.read().unwrap().create_index(
            view.clone(),
            INDEX_DISK.to_string(),
            Engine::Disk,
            KeyType::String,
            true,
            true,
            false,
        )?;
        if with_increment {
            view.read().unwrap().create_index(
                view.clone(),
                INDEX_INCREMENT.to_string(),
                Engine::Increment,
                KeyType::UInt,
                false,
                true,
                false,
            )?;
        }
        Ok(view)
    }

    /// 创建时间
    pub fn create_time(&self) -> Time {
        self.create_time.clone()
    }

    /// 索引集合
    pub fn index_map(&self) -> Arc<RwLock<HashMap<String, Arc<dyn TIndex>>>> {
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
    pub(crate) fn record(&self, version: u16) -> GeorgeResult<(String, Time)> {
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
    pub(crate) fn archive(&mut self, archive_file_path: String) -> GeorgeResult<()> {
        let header_bytes = self.ge.metadata().header().to_vec()?;
        let description_content_bytes_vc = self.ge.history()?;
        self.ge.archive(archive_file_path)?;
        self.ge.rebuild(header_bytes, description_content_bytes_vc)
    }

    /// 视图变更
    pub(crate) fn modify(&mut self, name: String, comment: String) -> GeorgeResult<()> {
        let time = Time::now();
        let view_path_new = Paths::view_path(self.database_name(), name.clone());
        let pigeonhole = Pigeonhole::create(0, view_path_new.clone(), time);
        let description_bytes = View::description(name.clone(), comment.clone(), time, pigeonhole);
        self.ge.modify(description_bytes)?;
        if self.name().ne(&name) {
            let view_path_old = Paths::view_path(self.database_name(), self.name());
            match Filer::rename(view_path_old, view_path_new) {
                Ok(_) => {
                    self.name = name;
                    self.comment = comment;
                    self.create_time = time;
                    Ok(())
                }
                Err(err) => Err(Errs::strs("file rename failed", err.to_string())),
            }
        } else {
            self.comment = comment;
            self.create_time = time;
            Ok(())
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
        index_type: Engine,
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
        self.ge.append(content)
    }
}

impl TForm for View {
    fn database_name(&self) -> String {
        self.database_name.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn comment(&self) -> String {
        self.comment.clone()
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
    fn description(
        name: String,
        comment: String,
        create_time: Time,
        pigeonhole: Pigeonhole,
    ) -> Vec<u8> {
        hex::encode(format!(
            "{}:#?{}:#?{}:#?{}",
            name,
            comment,
            create_time.nano_string().unwrap(),
            pigeonhole.to_string()
        ))
        .into_bytes()
    }

    /// 通过文件描述恢复结构信息
    pub(crate) fn recover(database_name: String, name: String) -> GeorgeResult<Arc<RwLock<View>>> {
        let filepath = Paths::view_filepath(database_name.clone(), name.clone());
        let ge = GeFactory {}.recovery(Tag::View, filepath)?;
        let description_str = Strings::from_utf8(ge.description_content_bytes()?)?;
        match hex::decode(description_str) {
            Ok(vu8) => {
                let real = Strings::from_utf8(vu8)?;
                let mut split = real.split(":#?");
                let name = split.next().unwrap().to_string();
                let comment = split.next().unwrap().to_string();
                let duration = Duration::nanoseconds(
                    split.next().unwrap().to_string().parse::<i64>().unwrap(),
                );
                let pigeonhole = Pigeonhole::from_string(split.next().unwrap().to_string())?;
                let time = Time::from(duration);
                let view = Arc::new(RwLock::new(View {
                    database_name: database_name.clone(),
                    name: name.clone(),
                    comment,
                    create_time: time,
                    ge,
                    indexes: Arc::new(Default::default()),
                    pigeonhole: pigeonhole.clone(),
                }));
                log::info!("recovery view {} from database {}", name, database_name,);
                match read_dir(Paths::view_path(database_name.clone(), name.clone())) {
                    // 恢复indexes数据
                    Ok(paths) => {
                        view.read().unwrap().recovery_indexes(view.clone(), paths)?;
                        log::debug!(
                            "view {{db={}, name={}, create_time={}, pigeonhole={:#?}}}",
                            database_name,
                            name,
                            time.format("%Y-%m-%d %H:%M:%S"),
                            pigeonhole,
                        );
                        Ok(view)
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
        let index = IndexDefault::recover(view, index_name.clone())?;
        log::debug!(
            "index [db={}, view={}, name={}, create_time={}]",
            self.database_name(),
            self.name(),
            index_name.clone(),
            index.create_time().format("%Y-%m-%d %H:%M:%S"),
        );
        // 如果已存在该view，则不处理
        if !self.exist_index(index_name.clone()) {
            self.index_map().write().unwrap().insert(index_name, index);
        }
        Ok(())
    }
}
