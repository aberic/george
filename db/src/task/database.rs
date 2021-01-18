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

use crate::task::view::View;
use crate::utils::comm::{Capacity, EngineType, IndexType};
use crate::utils::path::{database_file_path, database_path, view_file_path};
use crate::utils::store::{
    before_content_bytes, metadata_2_bytes, recovery_before_content, Metadata, Tag, HD,
};
use crate::utils::writer::obtain_write_append_file;
use chrono::{Duration, Local, NaiveDateTime};
use comm::errors::children::{ViewExistError, ViewNoExistError};
use comm::errors::entrances::{err_str_enhance, err_string, GeorgeError, GeorgeResult};
use comm::io::file::create_file;
use comm::io::reader::{read_sub, read_sub_bytes};
use comm::io::writer::{write_file_append_bytes, write_seek_u8s};
use std::collections::HashMap;
use std::fs::{read_dir, File, ReadDir};
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::macos::fs::MetadataExt;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct Database {
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
    let file_path = database_file_path(name.clone());
    let file_append = obtain_write_append_file(file_path)?;
    Ok(Database {
        name,
        create_time,
        metadata: Metadata::default(Tag::Database),
        file_append,
        views: Arc::new(Default::default()),
    })
}

impl Database {
    pub(super) fn create(name: String) -> GeorgeResult<Arc<RwLock<Database>>> {
        create_file(database_file_path(name.clone()), true)?;
        let mut db = new_database(name)?;
        let mut metadata_bytes = metadata_2_bytes(db.metadata());
        let mut description = db.description();
        // 初始化为32 + 8，即head长度加正文描述符长度
        let mut before_description = before_content_bytes(40, description.len() as u32);
        metadata_bytes.append(&mut before_description);
        metadata_bytes.append(&mut description);
        db.file_append(metadata_bytes)?;
        Ok(Arc::new(RwLock::new(db)))
    }
    /// 名称
    pub(super) fn name(&self) -> String {
        self.name.clone()
    }
    /// 创建时间
    pub(super) fn create_time(&self) -> Duration {
        self.create_time.clone()
    }
    /// 文件信息
    pub(super) fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }
    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
    ///
    /// #Return
    ///
    /// seek_end_before 写之前文件字节数据长度
    pub(super) fn file_append(&mut self, content: Vec<u8>) -> GeorgeResult<u64> {
        let file_append = self.file_append.clone();
        let mut file_write = file_append.write().unwrap();
        match file_write.seek(SeekFrom::End(0)) {
            Ok(seek_end_before) => {
                match write_file_append_bytes(file_write.try_clone().unwrap(), content.clone()) {
                    Ok(()) => Ok(seek_end_before),
                    Err(_err) => {
                        self.file_append =
                            obtain_write_append_file(database_file_path(self.name()))?;
                        let file_again = self.file_append.write().unwrap();
                        write_file_append_bytes(file_again.try_clone().unwrap(), content)?;
                        Ok(seek_end_before)
                    }
                }
            }
            Err(_err) => {
                self.file_append = obtain_write_append_file(database_file_path(self.name()))?;
                let mut file_write_again = self.file_append.write().unwrap();
                let seek_end_before_again = file_write_again.seek(SeekFrom::End(0)).unwrap();
                write_file_append_bytes(file_write_again.try_clone().unwrap(), content)?;
                Ok(seek_end_before_again)
            }
        }
    }
    /// 视图索引集合
    pub(super) fn view_map(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<View>>>>> {
        self.views.clone()
    }
    pub(crate) fn modify(&mut self, name: String) -> GeorgeResult<()> {
        let old_name = self.name();
        let filepath = database_file_path(old_name.clone());
        let content = read_sub_bytes(filepath.clone(), 0, 40)?;
        self.name = name;
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
        write_seek_u8s(filepath.clone(), 32, content_new.as_slice())?;
        let database_path_old = database_path(old_name);
        let database_path_new = database_path(self.name());
        match std::fs::rename(database_path_old, database_path_new) {
            Ok(_) => {
                self.file_append = obtain_write_append_file(database_file_path(self.name()))?;
                Ok(())
            }
            Err(err) => {
                // 回滚数据
                write_seek_u8s(filepath, 0, content.as_slice())?;
                Err(err_str_enhance("file rename error: ", err.to_string()))
            }
        }
    }
    pub(crate) fn exist_view(&self, view_name: String) -> bool {
        for res in self.views.clone().read().unwrap().iter() {
            if res.0.eq(&view_name) {
                return true;
            }
        }
        return false;
    }
    /// 创建视图
    pub(super) fn create_view(&self, name: String) -> GeorgeResult<()> {
        if self.exist_view(name.clone()) {
            return Err(GeorgeError::ViewExistError(ViewExistError));
        }
        let view = View::create(self.name(), name.clone())?;
        self.view_map().write().unwrap().insert(name, view.clone());
        Ok(())
    }
    /// 修改视图
    pub fn modify_view(&self, name: String, new_name: String) -> GeorgeResult<()> {
        if !self.exist_view(name.clone()) {
            return Err(GeorgeError::ViewNoExistError(ViewNoExistError));
        }
        if self.exist_view(new_name.clone()) {
            return Err(GeorgeError::ViewNoExistError(ViewNoExistError));
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
}

impl Database {
    fn description(&mut self) -> Vec<u8> {
        hex::encode(format!(
            "{}/{}",
            self.name(),
            self.create_time().num_nanoseconds().unwrap().to_string(),
        ))
        .into_bytes()
    }

    pub(super) fn recover(hd: HD) -> GeorgeResult<Database> {
        match String::from_utf8(hd.description) {
            Ok(description_str) => match hex::decode(description_str) {
                Ok(vu8) => match String::from_utf8(vu8) {
                    Ok(real) => {
                        let mut split = real.split("/");
                        let name = split.next().unwrap().to_string();
                        let create_time = Duration::nanoseconds(
                            split.next().unwrap().to_string().parse::<i64>().unwrap(),
                        );
                        let file_path = database_file_path(name.clone());
                        let file_append = obtain_write_append_file(file_path)?;
                        let database = Database {
                            name,
                            create_time,
                            metadata: hd.metadata,
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
                            Err(err) => {
                                panic!("recovery databases read dir failed! error is {}", err)
                            }
                        }
                        Ok(database)
                    }
                    Err(err) => Err(err_string(format!(
                        "recovery database from utf8 failed! error is {}",
                        err
                    ))),
                },
                Err(err) => Err(err_string(format!(
                    "recovery database decode failed! error is {}",
                    err
                ))),
            },
            Err(err) => Err(err_string(err.to_string())),
        }
    }
}

impl Database {
    /// 恢复views数据
    pub(super) fn recovery_views(&self, paths: ReadDir) {
        // 遍历data目录下文件
        for path in paths {
            match path {
                // 所有目录文件被默认为database根目录
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
        let view_file_path = view_file_path(self.name(), view_name);
        match recovery_before_content(Tag::View, view_file_path.clone()) {
            Ok(hd) => {
                log::trace!("head = {:#?}", hd.metadata);
                // 恢复view数据
                match View::recover(self.name(), hd) {
                    Ok(view) => {
                        log::debug!(
                            "view [db={}, name={}, create_time={}]",
                            self.name(),
                            view.name(),
                            view.create_time().num_nanoseconds().unwrap().to_string()
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
