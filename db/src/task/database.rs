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

use comm::errors::children::{ViewExistError, ViewNoExistError};
use comm::errors::entrances::{Errs, GeorgeError, GeorgeResult};
use comm::strings::{StringHandler, Strings};

use crate::task::rich::Expectation;
use crate::task::view::View;
use crate::utils::path::Paths;
use crate::utils::store::{ContentBytes, Metadata, HD};
use crate::utils::writer::Filed;
use comm::json::Json;

#[derive(Debug, Clone)]
pub(crate) struct Database {
    /// 名称
    name: String,
    /// 描述
    comment: String,
    /// 创建时间
    create_time: Duration,
    /// 文件信息
    metadata: Metadata,
    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 需要借助对象包裹，以便更新file，避免self为mut
    filer: Filed,
    /// 视图集合
    views: Arc<RwLock<HashMap<String, Arc<RwLock<View>>>>>,
}

/// 新建数据库
///
/// 具体传参参考如下定义：<p><p>
///
/// ###Params
///
/// id 数据库唯一ID
///
/// name 数据库名称
///
/// comment 数据库描述
fn new_database(name: String, comment: String) -> GeorgeResult<Database> {
    let now: NaiveDateTime = Local::now().naive_local();
    let create_time = Duration::nanoseconds(now.timestamp_nanos());
    let filepath = Paths::database_filepath(name.clone());
    Ok(Database {
        name,
        comment,
        create_time,
        metadata: Metadata::database(),
        filer: Filed::create(filepath)?,
        views: Arc::new(Default::default()),
    })
}

impl Database {
    pub(crate) fn create(name: String, comment: String) -> GeorgeResult<Arc<RwLock<Database>>> {
        let database = new_database(name, comment)?;
        let mut metadata_bytes = database.metadata_bytes();
        let mut description = database.description();
        // 初始化为32 + 8，即head长度加正文描述符长度
        let mut before_description = ContentBytes::before(44, description.len() as u32);
        metadata_bytes.append(&mut before_description);
        metadata_bytes.append(&mut description);
        database.append(metadata_bytes)?;
        Ok(Arc::new(RwLock::new(database)))
    }

    /// 名称
    pub(crate) fn name(&self) -> String {
        self.name.clone()
    }

    /// 描述
    pub(crate) fn comment(&self) -> String {
        self.comment.clone()
    }

    /// 创建时间
    pub(crate) fn create_time(&self) -> Duration {
        self.create_time.clone()
    }

    /// 文件字节信息
    pub(crate) fn metadata_bytes(&self) -> Vec<u8> {
        self.metadata.bytes()
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

    /// 视图索引集合
    pub(crate) fn view_map(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<View>>>>> {
        self.views.clone()
    }

    pub(crate) fn modify(&mut self, name: String, comment: String) -> GeorgeResult<()> {
        let old_name = self.name();
        let content = self.read(0, 44)?;
        self.name = name.clone();
        self.comment = comment.clone();
        let description = self.description();
        let seek_end = self.append(description.clone())?;
        log::debug!(
            "database {} modify to {} with file seek_end = {}",
            old_name.clone(),
            self.name(),
            seek_end
        );
        let content_new = ContentBytes::before(seek_end, description.len() as u32);
        // 更新首部信息，初始化head为32，描述起始4字节，长度4字节
        self.write(32, content_new)?;
        let database_path_old = Paths::database_path(old_name);
        let database_path_new = Paths::database_path(self.name());
        match std::fs::rename(database_path_old, database_path_new) {
            Ok(_) => Ok(()),
            Err(err) => {
                // 回滚数据
                self.write(0, content)?;
                Err(Errs::strs("file rename failed", err.to_string()))
            }
        }
    }

    /// 根据视图name获取视图
    pub(super) fn view(&self, view_name: String) -> GeorgeResult<Arc<RwLock<View>>> {
        match self.view_map().read().unwrap().get(&view_name) {
            Some(view) => Ok(view.clone()),
            None => Err(GeorgeError::from(ViewNoExistError)),
        }
    }

    pub(crate) fn exist_view(&self, view_name: String) -> bool {
        return match self.view(view_name) {
            Ok(_) => true,
            Err(_) => false,
        };
    }

    /// 创建视图
    ///
    /// mem 是否为内存视图
    pub(crate) fn create_view(&self, name: String) -> GeorgeResult<()> {
        if self.exist_view(name.clone()) {
            return Err(GeorgeError::from(ViewExistError));
        }
        self.view_map()
            .write()
            .unwrap()
            .insert(name.clone(), View::create(self.name(), name)?);
        Ok(())
    }

    /// 删除视图
    pub(super) fn remove_view(&self, view_name: String) -> GeorgeResult<()> {
        if !self.exist_view(view_name.clone()) {
            Err(GeorgeError::from(ViewExistError))
        } else {
            self.view_map().write().unwrap().remove(&view_name);
            Ok(())
        }
    }

    /// 修改视图
    pub(crate) fn modify_view(&self, view_name: String, view_new_name: String) -> GeorgeResult<()> {
        if !self.exist_view(view_name.clone()) {
            return Err(GeorgeError::from(ViewNoExistError));
        }
        if self.exist_view(view_new_name.clone()) {
            return Err(GeorgeError::from(ViewNoExistError));
        }
        let view = self.view(view_name.clone())?;
        view.clone()
            .write()
            .unwrap()
            .modify(self.name(), view_new_name.clone())?;
        self.remove_view(view_name)?;
        self.recovery_view(view_new_name)
    }

    /// 整理归档
    ///
    /// archive_file_path 归档路径
    pub(crate) fn archive_view(
        &self,
        view_name: String,
        archive_file_path: String,
    ) -> GeorgeResult<()> {
        self.view(view_name)?
            .write()
            .unwrap()
            .archive(archive_file_path)
    }

    /// 指定归档版本信息
    ///
    /// version 版本号
    ///
    /// #return
    /// * filepath 当前归档版本文件所处路径
    /// * create_time 归档时间
    pub(crate) fn view_record(
        &self,
        view_name: String,
        version: u16,
    ) -> GeorgeResult<(String, Duration)> {
        let record = self.view(view_name)?.read().unwrap().record(version)?;
        Ok((record.filepath(), record.create_time()))
    }

    /// 视图文件信息
    pub(crate) fn view_metadata(&self, view_name: String) -> GeorgeResult<String> {
        Json::obj_2_string(&self.view(view_name)?.read().unwrap().metadata())
    }
}

/// db for disk
impl Database {
    /// 插入数据，如果存在则返回已存在<p><p>
    ///
    /// ###Params
    ///
    /// view_name 视图名称<p><p>
    ///
    /// key string
    ///
    /// value 当前结果value信息<p><p>
    ///
    /// ###Return
    ///
    /// IndexResult<()>
    pub(crate) fn put(&self, view_name: String, key: String, value: Vec<u8>) -> GeorgeResult<()> {
        self.view(view_name)?.read().unwrap().put(key, value)
    }

    /// 插入数据，无论存在与否都会插入或更新数据<p><p>
    ///
    /// ###Params
    ///
    /// view_name 视图名称<p><p>
    ///
    /// key string
    ///
    /// value 当前结果value信息<p><p>
    ///
    /// ###Return
    ///
    /// IndexResult<()>
    pub(crate) fn set(&self, view_name: String, key: String, value: Vec<u8>) -> GeorgeResult<()> {
        self.view(view_name)?.read().unwrap().set(key, value)
    }

    /// 获取数据，返回存储对象<p><p>
    ///
    /// ###Params
    ///
    /// view_name 视图名称<p><p>
    ///
    /// index_name 索引名称
    ///
    /// key string
    ///
    /// ###Return
    ///
    /// Seed value信息
    pub(crate) fn get(
        &self,
        view_name: String,
        index_name: &str,
        key: String,
    ) -> GeorgeResult<Vec<u8>> {
        self.view(view_name)?.read().unwrap().get(index_name, key)
    }

    /// 删除数据<p><p>
    ///
    /// ###Params
    ///
    /// view_name 视图名称<p><p>
    ///
    /// key string
    ///
    /// ###Return
    ///
    /// IndexResult<()>
    pub(crate) fn remove(&self, view_name: String, key: String) -> GeorgeResult<()> {
        self.view(view_name)?.read().unwrap().remove(key, vec![])
    }

    /// 条件检索
    ///
    /// selector_json_bytes 选择器字节数组，自定义转换策略
    pub(crate) fn select(
        &self,
        view_name: String,
        constraint_json_bytes: Vec<u8>,
    ) -> GeorgeResult<Expectation> {
        return match self.views.clone().read().unwrap().get(&view_name) {
            Some(view) => view.read().unwrap().select(constraint_json_bytes),
            _ => Err(GeorgeError::ViewNoExistError(ViewNoExistError)),
        };
    }

    /// 条件删除
    ///
    /// selector_json_bytes 选择器字节数组，自定义转换策略
    pub(crate) fn delete(
        &self,
        view_name: String,
        constraint_json_bytes: Vec<u8>,
    ) -> GeorgeResult<Expectation> {
        return match self.views.clone().read().unwrap().get(&view_name) {
            Some(view) => view.read().unwrap().delete(constraint_json_bytes),
            _ => Err(GeorgeError::ViewNoExistError(ViewNoExistError)),
        };
    }
}

impl Database {
    /// 生成文件描述
    fn description(&self) -> Vec<u8> {
        hex::encode(format!(
            "{}:#?{}:#?{}",
            self.name(),
            self.comment(),
            self.create_time().num_nanoseconds().unwrap().to_string(),
        ))
        .into_bytes()
    }

    /// 通过文件描述恢复结构信息
    pub(crate) fn recover(hd: HD) -> GeorgeResult<Database> {
        let description_str = Strings::from_utf8(hd.description())?;
        match hex::decode(description_str) {
            Ok(vu8) => {
                let real = Strings::from_utf8(vu8)?;
                let mut split = real.split(":#?");
                let name = split.next().unwrap().to_string();
                let comment = split.next().unwrap().to_string();
                let create_time = Duration::nanoseconds(
                    split.next().unwrap().to_string().parse::<i64>().unwrap(),
                );
                let filepath = Paths::database_filepath(name.clone());
                let database = Database {
                    name,
                    comment,
                    create_time,
                    metadata: hd.metadata(),
                    filer: Filed::recovery(filepath)?,
                    views: Arc::new(Default::default()),
                };
                log::info!("recovery database {}", database.name());
                // 读取database目录下所有文件
                match read_dir(Paths::database_path(database.name())) {
                    // 恢复views数据
                    Ok(paths) => {
                        database.recovery_views(paths)?;
                        Ok(database)
                    }
                    Err(err) => Err(Errs::strs("recovery databases read dir", err)),
                }
            }
            Err(err) => Err(Errs::strs("recovery database decode", err)),
        }
    }

    /// 恢复views数据
    pub(crate) fn recovery_views(&self, paths: ReadDir) -> GeorgeResult<()> {
        // 遍历database目录下文件
        for path in paths {
            match path {
                // 所有目录文件被默认为view根目录
                Ok(dir) => {
                    if dir.path().is_dir() {
                        let view_name = dir.file_name().to_str().unwrap().to_string();
                        log::debug!("recovery view from {}", view_name);
                        // 恢复view数据
                        match self.recovery_view(view_name) {
                            Err(err) => return Err(Errs::strs("recovery view", err)),
                            _ => {}
                        }
                    }
                }
                Err(err) => return Err(Errs::strs("recovery views path", err)),
            }
        }
        Ok(())
    }

    /// 恢复view数据
    fn recovery_view(&self, view_name: String) -> GeorgeResult<()> {
        let view_file_path = Paths::view_filepath(self.name(), view_name);
        let hd = ContentBytes::recovery(view_file_path.clone())?;
        let view = View::recover(self.name(), hd.clone())?;
        let view_c = view.clone();
        let view_r = view_c.read().unwrap();
        log::debug!(
            "view [db={}, name={}, create_time={}, pigeonhole={:#?}, {:#?}]",
            self.name(),
            view_r.name(),
            view_r.create_time().num_nanoseconds().unwrap().to_string(),
            view_r.pigeonhole(),
            hd.metadata()
        );
        // 如果已存在该view，则不处理
        if !self.exist_view(view_r.name()) {
            self.view_map().write().unwrap().insert(view_r.name(), view);
        }
        Ok(())
    }
}
