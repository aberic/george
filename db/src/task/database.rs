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

use comm::errors::{Errs, GeorgeResult};
use comm::io::file::FilerHandler;
use comm::io::Filer;
use comm::strings::StringHandler;
use comm::{Strings, Time};
use ge::utils::enums::Tag;
use ge::GeFactory;

use crate::task::rich::Expectation;
use crate::task::{Database, View};
use crate::utils::Paths;

impl Database {
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
    pub(crate) fn create(name: String, comment: String) -> GeorgeResult<Arc<RwLock<Database>>> {
        let time = Time::now();
        let filepath = Paths::database_filepath(name.clone());
        let description = Some(Database::description(name.clone(), comment.clone(), time));
        let database = Database {
            name,
            comment,
            create_time: time,
            ge: GeFactory {}.create(Tag::Database, filepath, description)?,
            views: Arc::new(Default::default()),
        };
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
    pub(crate) fn create_time(&self) -> Time {
        self.create_time.clone()
    }

    /// 视图索引集合
    pub(crate) fn view_map(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<View>>>>> {
        self.views.clone()
    }

    pub(crate) fn modify(&mut self, name: String, comment: String) -> GeorgeResult<()> {
        let time = Time::now();
        let description_bytes = Database::description(name.clone(), comment.clone(), time);
        self.ge.modify(description_bytes)?;
        if self.name().ne(&name) {
            let database_path_old = Paths::database_path(self.name());
            let database_path_new = Paths::database_path(name.clone());
            match Filer::rename(database_path_old, database_path_new) {
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

    /// 根据视图name获取视图
    pub(super) fn view(&self, view_name: String) -> GeorgeResult<Arc<RwLock<View>>> {
        match self.view_map().read().unwrap().get(&view_name) {
            Some(view) => Ok(view.clone()),
            None => Err(Errs::view_no_exist_error()),
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
    pub(crate) fn create_view(
        &self,
        name: String,
        comment: String,
        with_sequence: bool,
    ) -> GeorgeResult<()> {
        if self.exist_view(name.clone()) {
            return Err(Errs::view_exist_error());
        }
        self.view_map().write().unwrap().insert(
            name.clone(),
            View::create(self.name(), name, comment, with_sequence)?,
        );
        Ok(())
    }

    /// 删除视图
    pub(super) fn remove_view(&self, view_name: String) -> GeorgeResult<()> {
        if !self.exist_view(view_name.clone()) {
            Err(Errs::view_exist_error())
        } else {
            self.view_map().write().unwrap().remove(&view_name);
            Ok(())
        }
    }

    /// 修改视图
    pub(crate) fn modify_view(
        &self,
        view_name: String,
        view_new_name: String,
        comment: String,
    ) -> GeorgeResult<()> {
        if !self.exist_view(view_name.clone()) {
            return Err(Errs::view_no_exist_error());
        }
        if self.exist_view(view_new_name.clone()) {
            return Err(Errs::view_no_exist_error());
        }
        let view = self.view(view_name.clone())?;
        view.clone()
            .write()
            .unwrap()
            .modify(view_new_name.clone(), comment)?;
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
    ) -> GeorgeResult<(String, Time)> {
        self.view(view_name)?.read().unwrap().record(version)
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
            _ => Err(Errs::view_no_exist_error()),
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
            _ => Err(Errs::view_no_exist_error()),
        };
    }
}

impl Database {
    /// 生成文件描述
    fn description(name: String, comment: String, create_time: Time) -> Vec<u8> {
        hex::encode(format!(
            "{}:#?{}:#?{}",
            name,
            comment,
            create_time.nano_string().unwrap(),
        ))
        .into_bytes()
    }

    /// 通过文件描述恢复结构信息
    pub(crate) fn recover(name: String) -> GeorgeResult<Database> {
        let filepath = Paths::database_filepath(name.clone());
        let ge = GeFactory {}.recovery(Tag::Database, filepath)?;
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
                let database = Database {
                    name,
                    comment,
                    create_time: Time::from(duration),
                    ge,
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
        let view = View::recover(self.name(), view_name.clone())?;
        // 如果已存在该view，则不处理
        if !self.exist_view(view_name.clone()) {
            self.view_map().write().unwrap().insert(view_name, view);
        }
        Ok(())
    }
}
