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

use crate::task::rich::Expectation;
use crate::task::view::View;
use crate::utils::path::{database_filepath, database_path, view_filepath};
use crate::utils::store::{before_content_bytes, recovery_before_content, Metadata, HD};
use crate::utils::writer::obtain_write_append_file;
use chrono::{Duration, Local, NaiveDateTime};
use comm::errors::children::{ViewExistError, ViewNoExistError};
use comm::errors::entrances::{err_string, err_strs, GeorgeError, GeorgeResult};
use comm::io::file::{Filer, FilerExecutor, FilerHandler, FilerReader, FilerWriter};
use comm::strings::{StringHandler, Strings};
use std::collections::HashMap;
use std::fs::{read_dir, File, ReadDir};
use std::io::{Seek, SeekFrom};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub(crate) struct Database {
    /// 名称
    name: String,
    /// 创建时间
    create_time: Duration,
    /// 文件信息
    metadata: Metadata,
    /// 根据文件路径获取该文件追加写入的写对象
    file_append: Arc<RwLock<File>>,
    /// 视图索引集合
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
fn new_database(name: String) -> GeorgeResult<Database> {
    let now: NaiveDateTime = Local::now().naive_local();
    let create_time = Duration::nanoseconds(now.timestamp_nanos());
    let filepath = database_filepath(name.clone());
    let file_append = obtain_write_append_file(filepath)?;
    Ok(Database {
        name,
        create_time,
        metadata: Metadata::database(),
        file_append,
        views: Arc::new(Default::default()),
    })
}

impl Database {
    pub(crate) fn create(name: String) -> GeorgeResult<Arc<RwLock<Database>>> {
        Filer::touch(database_filepath(name.clone()))?;
        let mut database = new_database(name)?;
        let mut metadata_bytes = database.metadata_bytes();
        let mut description = database.description();
        // 初始化为32 + 8，即head长度加正文描述符长度
        let mut before_description = before_content_bytes(40, description.len() as u32);
        metadata_bytes.append(&mut before_description);
        metadata_bytes.append(&mut description);
        database.file_append(metadata_bytes)?;
        Ok(Arc::new(RwLock::new(database)))
    }
    /// 名称
    pub(crate) fn name(&self) -> String {
        self.name.clone()
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
    fn file_append(&mut self, content: Vec<u8>) -> GeorgeResult<u64> {
        let file_append = self.file_append.clone();
        let mut file_write = file_append.write().unwrap();
        match file_write.seek(SeekFrom::End(0)) {
            Ok(seek_end_before) => match file_write.try_clone() {
                Ok(f) => match Filer::appends(f, content.clone()) {
                    Ok(()) => Ok(seek_end_before),
                    Err(_err) => {
                        self.file_append =
                            obtain_write_append_file(database_filepath(self.name()))?;
                        let file_write_again = self.file_append.write().unwrap();
                        match file_write_again.try_clone() {
                            Ok(f) => Filer::appends(f, content)?,
                            Err(err) => {
                                return Err(err_strs("database append file try clone3", err))
                            }
                        }
                        Ok(seek_end_before)
                    }
                },
                Err(err) => Err(err_strs("database append file try clone2", err)),
            },
            Err(_err) => {
                self.file_append = obtain_write_append_file(database_filepath(self.name()))?;
                let mut file_write_again = self.file_append.write().unwrap();
                let seek_end_before_again = file_write_again.seek(SeekFrom::End(0)).unwrap();
                match file_write_again.try_clone() {
                    Ok(f) => Filer::appends(f, content)?,
                    Err(err) => return Err(err_strs("database append file try clone1", err)),
                }
                Ok(seek_end_before_again)
            }
        }
    }
    /// 视图索引集合
    pub(crate) fn view_map(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<View>>>>> {
        self.views.clone()
    }
    pub(crate) fn modify(&mut self, name: String) -> GeorgeResult<()> {
        let old_name = self.name();
        let filepath = database_filepath(old_name.clone());
        let content = Filer::read_sub(filepath.clone(), 0, 40)?;
        self.name = name.clone();
        let description = self.description();
        let seek_end = self.file_append(description.clone())?;
        log::debug!(
            "database {} modify to {} with file seek_end = {}",
            old_name.clone(),
            self.name(),
            seek_end
        );
        let content_new = before_content_bytes(seek_end as u32, description.len() as u32);
        // 更新首部信息，初始化head为32，描述起始4字节，长度4字节
        Filer::write_seek(filepath.clone(), 32, content_new)?;
        let database_path_old = database_path(old_name);
        let database_path_new = database_path(self.name());
        match std::fs::rename(database_path_old, database_path_new) {
            Ok(_) => {
                self.file_append = obtain_write_append_file(database_filepath(self.name()))?;
                for (view_name, view) in self.views.write().unwrap().iter() {
                    view.write()
                        .unwrap()
                        .modify(name.clone(), view_name.clone())?;
                }
                Ok(())
            }
            Err(err) => {
                // 回滚数据
                Filer::write_seek(filepath, 0, content)?;
                Err(err_strs("file rename failed", err.to_string()))
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
    /// 修改视图
    pub(crate) fn modify_view(&self, name: String, new_name: String) -> GeorgeResult<()> {
        if !self.exist_view(name.clone()) {
            return Err(GeorgeError::from(ViewNoExistError));
        }
        if self.exist_view(new_name.clone()) {
            return Err(GeorgeError::from(ViewNoExistError));
        }
        let views = self.view_map();
        let mut views_w = views.write().unwrap();
        let view = views_w.get(&name).unwrap().clone();
        view.clone()
            .write()
            .unwrap()
            .modify(self.name(), new_name.clone())?;
        views_w.remove(&name);
        views_w.insert(new_name.clone(), view.clone());
        Ok(())
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
            .read()
            .unwrap()
            .archive(archive_file_path)
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
        self.view(view_name)?.read().unwrap().remove(key)
    }
    /// 条件检索
    ///
    /// selector_json_bytes 选择器字节数组，自定义转换策略
    pub fn select(
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
    pub fn delete(
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
    fn description(&mut self) -> Vec<u8> {
        hex::encode(format!(
            "{}:#?{}",
            self.name(),
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
                let create_time = Duration::nanoseconds(
                    split.next().unwrap().to_string().parse::<i64>().unwrap(),
                );
                let filepath = database_filepath(name.clone());
                let file_append = obtain_write_append_file(filepath)?;
                let database = Database {
                    name,
                    create_time,
                    metadata: hd.metadata(),
                    file_append,
                    views: Arc::new(Default::default()),
                };
                log::info!("recovery database {}", database.name());
                // 读取database目录下所有文件
                match read_dir(database_path(database.name())) {
                    // 恢复views数据
                    Ok(paths) => {
                        database.recovery_views(paths);
                    }
                    Err(err) => panic!("recovery databases read dir failed! error is {}", err),
                }
                Ok(database)
            }
            Err(err) => Err(err_string(format!(
                "recovery database decode failed! error is {}",
                err
            ))),
        }
    }
    /// 恢复views数据
    pub(crate) fn recovery_views(&self, paths: ReadDir) {
        // 遍历database目录下文件
        for path in paths {
            match path {
                // 所有目录文件被默认为view根目录
                Ok(dir) => {
                    if dir.path().is_dir() {
                        let view_name = dir.file_name().to_str().unwrap().to_string();
                        log::debug!("recovery view from {}", view_name);
                        // 恢复view数据
                        self.recovery_view(view_name.clone());
                    }
                }
                Err(err) => panic!("recovery views path failed! error is {}", err),
            }
        }
    }

    /// 恢复view数据
    fn recovery_view(&self, view_name: String) {
        let view_file_path = view_filepath(self.name(), view_name);
        match recovery_before_content(view_file_path.clone()) {
            Ok(hd) => {
                // 恢复view数据
                match View::recover(self.name(), hd.clone()) {
                    Ok(view) => {
                        log::debug!(
                            "view [db={}, name={}, create_time={}, pigeonhole={:#?}, {:#?}]",
                            self.name(),
                            view.name(),
                            view.create_time().num_nanoseconds().unwrap().to_string(),
                            view.pigeonhole(),
                            hd.metadata()
                        );
                        // 如果已存在该view，则不处理
                        if self.exist_view(view.name()) {
                            return;
                        }
                        self.view_map()
                            .write()
                            .unwrap()
                            .insert(view.name(), Arc::new(RwLock::new(view)));
                    }
                    Err(err) => panic!("recovery view failed! error is {}", err),
                }
            }
            Err(err) => panic!(
                "recovery view when recovery before content failed! error is {}",
                err
            ),
        }
    }
}
