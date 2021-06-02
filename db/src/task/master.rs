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
use log::LevelFilter;

use comm::errors::{Errs, GeorgeResult};
use comm::io::dir::DirHandler;
use comm::io::file::FilerHandler;
use comm::io::{Dir, Filer};
use comm::Time;
use ge::utils::enums::Tag;
use ge::Ge;
use logs::LogModule;

use crate::task::rich::Expectation;
use crate::task::Page;
use crate::task::{Database, Master};
use crate::utils::comm::{DEFAULT_COMMENT, DEFAULT_NAME, INDEX_DISK};
use crate::utils::deploy::GLOBAL_CONFIG;
use crate::utils::enums::{IndexType, KeyType};
use crate::utils::Paths;

impl Master {
    /// 初始化
    pub(crate) fn init(&self) -> GeorgeResult<()> {
        log::info!("bootstrap init!");
        self.create_page(DEFAULT_NAME.to_string(), DEFAULT_COMMENT.to_string(), 0, 0)?;
        self.create_database(DEFAULT_NAME.to_string(), DEFAULT_COMMENT.to_string())?;
        self.create_view(
            DEFAULT_NAME.to_string(),
            DEFAULT_NAME.to_string(),
            DEFAULT_COMMENT.to_string(),
            true,
        )
    }

    /// 生成Master
    pub(super) fn generate() -> Arc<Self> {
        // 尝试创建数据根目录，有则什么也不做，无则创建
        Dir::mk_uncheck(Paths::data_path()).expect("create data path failed!");
        // 启动文件
        let bootstrap_file_path = Paths::bootstrap_filepath();
        let init: bool;
        let ge: Ge;
        let duration: Duration;
        if Filer::exist(bootstrap_file_path.clone()) {
            ge = Ge::recovery(bootstrap_file_path.clone()).expect("recovery ge failed!");
            let description = ge
                .description_content_bytes()
                .expect("recovery description failed!");
            duration = Duration::nanoseconds(
                String::from_utf8(description)
                    .expect("description bytes to string failed!")
                    .parse::<i64>()
                    .unwrap(),
            );
            init = false;
        } else {
            let now: NaiveDateTime = Local::now().naive_local();
            duration = Duration::nanoseconds(now.timestamp_nanos());
            let description = duration
                .num_nanoseconds()
                .unwrap()
                .to_string()
                .as_bytes()
                .to_vec();
            Ge::new(bootstrap_file_path.clone(), Tag::Bootstrap, description)
                .expect("create ge failed!");
            init = true;
        }

        let create_time = Time::from(duration);
        log::info!(
            "george create at {}",
            create_time.to_string("%Y-%m-%d %H:%M:%S")
        );

        let master = Master {
            default_page_name: DEFAULT_NAME.to_string(),
            pages: Arc::new(Default::default()),
            databases: Default::default(),
            create_time: Time::from(duration),
        };
        let master_arc = Arc::new(master);
        if init {
            log::info!("initialize new data");
            master_arc.init().expect("initialize failed!");
        } else {
            log::info!("recovery exist data from bootstrap");
            log::debug!(
                "recovery exist data from bootstrap file {}",
                bootstrap_file_path
            );
            master_arc.recovery().expect("recovery failed!");
        }
        master_arc
    }

    pub(super) fn page_map(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<Page>>>>> {
        self.pages.clone()
    }

    pub(super) fn database_map(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<Database>>>>> {
        self.databases.clone()
    }

    pub(super) fn create_time(&self) -> Time {
        self.create_time.clone()
    }

    /// 创建缓存页
    ///
    /// ###Params
    ///
    /// * name 缓存页名称
    /// * comment 缓存页描述
    /// * size 可使用内存大小(单位：Mb)，为0则不限
    /// * period 默认有效期(单位：秒)，如为0，则默认为300
    pub(super) fn create_page(
        &self,
        name: String,
        comment: String,
        size: u64,
        period: u32,
    ) -> GeorgeResult<()> {
        if self.exist_page(name.clone()) {
            return Err(Errs::page_exist_error());
        }
        let page = Page::create(name.clone(), comment, size, period)?;
        self.page_map().write().unwrap().insert(name.clone(), page);
        log::debug!("create page {} success!", name);
        Ok(())
    }

    /// 删除缓存页
    pub(super) fn remove_page(&self, page_name: String) -> GeorgeResult<()> {
        if !self.exist_page(page_name.clone()) {
            Err(Errs::page_exist_error())
        } else {
            self.page_map().write().unwrap().remove(&page_name);
            Ok(())
        }
    }

    /// 修改缓存页
    pub(super) fn modify_page(&self, page_name: String, page_new_name: String) -> GeorgeResult<()> {
        if !self.exist_page(page_name.clone()) {
            return Err(Errs::page_no_exist_error());
        }
        if self.exist_page(page_new_name.clone()) {
            return Err(Errs::page_exist_error());
        }
        let page = self.page(page_name.clone())?;
        self.page_map()
            .write()
            .unwrap()
            .insert(page_new_name.clone(), page);
        self.remove_page(page_name)
    }

    /// 根据缓存页name获取库
    pub(super) fn page(&self, page_name: String) -> GeorgeResult<Arc<RwLock<Page>>> {
        match self.page_map().read().unwrap().get(&page_name) {
            Some(page) => Ok(page.clone()),
            None => Err(Errs::page_no_exist_error()),
        }
    }

    pub(super) fn page_default(&self) -> GeorgeResult<Arc<RwLock<Page>>> {
        self.page(self.default_page_name.clone())
    }

    fn exist_page(&self, page_name: String) -> bool {
        return match self.page(page_name) {
            Ok(_) => true,
            Err(_) => false,
        };
    }

    /// 创建数据库
    pub(super) fn create_database(
        &self,
        database_name: String,
        database_comment: String,
    ) -> GeorgeResult<()> {
        if self.exist_database(database_name.clone()) {
            return Err(Errs::database_exist_error());
        }
        let db = Database::create(database_name.clone(), database_comment.clone())?;
        self.database_map()
            .write()
            .unwrap()
            .insert(database_name.clone(), db);
        log::debug!("create database {} success!", database_name);
        Ok(())
    }

    /// 删除数据库
    pub(super) fn remove_database(&self, database_name: String) -> GeorgeResult<()> {
        if !self.exist_database(database_name.clone()) {
            Err(Errs::database_exist_error())
        } else {
            self.database_map().write().unwrap().remove(&database_name);
            Ok(())
        }
    }

    /// 修改数据库
    pub(super) fn modify_database(
        &self,
        database_name: String,
        database_new_name: String,
        database_comment: String,
    ) -> GeorgeResult<()> {
        if !self.exist_database(database_name.clone()) {
            return Err(Errs::database_no_exist_error());
        }
        if self.exist_database(database_new_name.clone()) {
            return Err(Errs::database_exist_error());
        }
        let database = self.database(database_name.clone())?;
        database
            .clone()
            .write()
            .unwrap()
            .modify(database_new_name.clone(), database_comment)?;
        self.remove_database(database_name)?;
        self.recovery_database(database_new_name)
    }

    /// 根据库name获取库
    pub(super) fn database(&self, database_name: String) -> GeorgeResult<Arc<RwLock<Database>>> {
        match self.database_map().read().unwrap().get(&database_name) {
            Some(database) => Ok(database.clone()),
            None => Err(Errs::database_no_exist_error()),
        }
    }

    fn exist_database(&self, database_name: String) -> bool {
        return match self.database(database_name) {
            Ok(_) => true,
            Err(_) => false,
        };
    }

    /// 创建视图
    ///
    /// mem 是否为内存视图
    pub(super) fn create_view(
        &self,
        database_name: String,
        view_name: String,
        comment: String,
        with_sequence: bool,
    ) -> GeorgeResult<()> {
        match self.database_map().read().unwrap().get(&database_name) {
            Some(database_lock) => {
                let database = database_lock.read().unwrap();
                database.create_view(view_name, comment, with_sequence)?;
            }
            None => return Err(Errs::database_no_exist_error()),
        }
        Ok(())
    }

    /// 修改视图
    pub(super) fn modify_view(
        &self,
        database_name: String,
        view_name: String,
        view_new_name: String,
        comment: String,
    ) -> GeorgeResult<()> {
        match self.database_map().read().unwrap().get(&database_name) {
            Some(database_lock) => {
                let database = database_lock.write().unwrap();
                database.modify_view(view_name, view_new_name, comment)
            }
            None => return Err(Errs::database_no_exist_error()),
        }
    }

    /// 整理归档
    ///
    /// archive_file_path 归档路径
    pub(super) fn archive_view(
        &self,
        database_name: String,
        view_name: String,
        archive_file_path: String,
    ) -> GeorgeResult<()> {
        self.database(database_name)?
            .read()
            .unwrap()
            .archive_view(view_name, archive_file_path)
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
        database_name: String,
        view_name: String,
        version: u16,
    ) -> GeorgeResult<(String, Time)> {
        self.database(database_name)?
            .read()
            .unwrap()
            .view_record(view_name, version)
    }

    /// 在指定库及视图中创建索引
    ///
    /// 该索引需要定义ID，此外索引所表达的字段组成内容也是必须的，并通过primary判断索引类型，具体传参参考如下定义：<p><p>
    ///
    /// ###Params
    /// * database_name 数据库名
    /// * view_name 视图名
    /// * index_name 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入
    /// * index_type 存储引擎类型
    /// * key_type 索引值类型
    /// * primary 是否主键，主键也是唯一索引，即默认列表依赖索引
    /// * unique 是否唯一索引
    /// * null 是否允许为空
    pub(super) fn create_index(
        &self,
        database_name: String,
        view_name: String,
        index_name: String,
        index_type: IndexType,
        key_type: KeyType,
        primary: bool,
        unique: bool,
        null: bool,
    ) -> GeorgeResult<()> {
        let database = self.database(database_name)?;
        let database_read = database.read().unwrap();
        let view = database_read.view(view_name)?;
        view.clone().read().unwrap().create_index(
            view, index_name, index_type, key_type, primary, unique, null,
        )
    }
}

/// db for disk
impl Master {
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
    pub(crate) fn put_disk(
        &self,
        database_name: String,
        view_name: String,
        key: String,
        value: Vec<u8>,
    ) -> GeorgeResult<()> {
        self.database(database_name)?
            .read()
            .unwrap()
            .put(view_name, key, value)
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
    pub(crate) fn set_disk(
        &self,
        database_name: String,
        view_name: String,
        key: String,
        value: Vec<u8>,
    ) -> GeorgeResult<()> {
        self.database(database_name)?
            .read()
            .unwrap()
            .set(view_name, key, value)
    }

    /// 获取数据，返回存储对象<p><p>
    ///
    /// ###Params
    ///
    /// view_name 视图名称
    ///
    /// key string
    ///
    /// ###Return
    ///
    /// Seed value信息
    pub(crate) fn get_disk(
        &self,
        database_name: String,
        view_name: String,
        key: String,
    ) -> GeorgeResult<Vec<u8>> {
        self.database(database_name)?
            .read()
            .unwrap()
            .get(view_name, INDEX_DISK, key)
    }

    /// 获取数据，返回存储对象<p><p>
    ///
    /// ###Params
    ///
    /// view_name 视图名称
    ///
    /// index_name 索引名称
    ///
    /// key string
    ///
    /// ###Return
    ///
    /// Seed value信息
    pub(crate) fn get_disk_by_index(
        &self,
        database_name: String,
        view_name: String,
        index_name: String,
        key: String,
    ) -> GeorgeResult<Vec<u8>> {
        self.database(database_name)?
            .read()
            .unwrap()
            .get(view_name, index_name.as_str(), key)
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
    pub(crate) fn remove_disk(
        &self,
        database_name: String,
        view_name: String,
        key: String,
    ) -> GeorgeResult<()> {
        self.database(database_name)?
            .read()
            .unwrap()
            .remove(view_name, key)
    }

    /// 条件检索
    ///
    /// selector_json_bytes 选择器字节数组，自定义转换策略
    pub(crate) fn select_disk(
        &self,
        database_name: String,
        view_name: String,
        constraint_json_bytes: Vec<u8>,
    ) -> GeorgeResult<Expectation> {
        self.database(database_name)?
            .read()
            .unwrap()
            .select(view_name, constraint_json_bytes)
    }

    /// 条件删除
    ///
    /// selector_json_bytes 选择器字节数组，自定义转换策略
    pub(crate) fn delete_disk(
        &self,
        database_name: String,
        view_name: String,
        constraint_json_bytes: Vec<u8>,
    ) -> GeorgeResult<Expectation> {
        self.database(database_name)?
            .read()
            .unwrap()
            .delete(view_name, constraint_json_bytes)
    }
}

/// db for default memory
impl Master {
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
    pub(crate) fn put_memory_default(&self, key: String, value: Vec<u8>) -> GeorgeResult<()> {
        self.page_default()?.read().unwrap().put(key, value)
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
    pub(crate) fn set_memory_default(&self, key: String, value: Vec<u8>) -> GeorgeResult<()> {
        self.page_default()?.read().unwrap().set(key, value)
    }

    /// 获取数据，返回存储对象<p><p>
    ///
    /// ###Params
    ///
    /// view_name 视图名称
    ///
    /// key string
    ///
    /// ###Return
    ///
    /// Seed value信息
    pub(crate) fn get_memory_default(&self, key: String) -> GeorgeResult<Vec<u8>> {
        self.page_default()?.read().unwrap().get(key)
    }

    /// 删除数据<p><p>
    ///
    /// ###Params
    ///
    /// view_name 视图名称
    ///
    /// key string
    ///
    /// ###Return
    ///
    /// Seed value信息
    pub(crate) fn remove_memory_default(&self, key: String) -> GeorgeResult<()> {
        self.page_default()?.read().unwrap().remove(key)
    }
}

/// db for memory
impl Master {
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
    pub(crate) fn put_memory(
        &self,
        page_name: String,
        key: String,
        value: Vec<u8>,
    ) -> GeorgeResult<()> {
        self.page(page_name)?.read().unwrap().put(key, value)
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
    pub(crate) fn set_memory(
        &self,
        page_name: String,
        key: String,
        value: Vec<u8>,
    ) -> GeorgeResult<()> {
        self.page(page_name)?.read().unwrap().set(key, value)
    }

    /// 获取数据，返回存储对象<p><p>
    ///
    /// ###Params
    ///
    /// view_name 视图名称
    ///
    /// key string
    ///
    /// ###Return
    ///
    /// Seed value信息
    pub(crate) fn get_memory(&self, page_name: String, key: String) -> GeorgeResult<Vec<u8>> {
        self.page(page_name)?.read().unwrap().get(key)
    }

    /// 删除数据<p><p>
    ///
    /// ###Params
    ///
    /// view_name 视图名称
    ///
    /// key string
    ///
    /// ###Return
    ///
    /// Seed value信息
    pub(crate) fn remove_memory(&self, page_name: String, key: String) -> GeorgeResult<()> {
        self.page(page_name)?.read().unwrap().remove(key)
    }
}

impl Master {
    /// 恢复sky数据
    pub(crate) fn recovery(&self) -> GeorgeResult<()> {
        log::info!("bootstrap recovery!");
        // 读取data目录下所有文件
        match read_dir(Paths::data_database_path()) {
            Ok(database_paths) => match self.recovery_databases(database_paths) {
                Ok(()) => match read_dir(Paths::data_page_path()) {
                    Ok(page_paths) => self.recovery_pages(page_paths),
                    Err(err) => Err(Errs::strs("recovery read dir page_paths", err)),
                },
                Err(err) => Err(Errs::strs("recovery databases", err)),
            },
            Err(err) => Err(Errs::strs("recovery read dir database_paths", err)),
        }
    }

    /// 恢复databases数据
    fn recovery_databases(&self, paths: ReadDir) -> GeorgeResult<()> {
        // 遍历data目录下文件
        for path in paths {
            match path {
                // 所有目录文件被默认为database根目录
                Ok(dir) => {
                    if dir.path().is_dir() {
                        let database_name = dir.file_name().to_str().unwrap().to_string();
                        log::debug!("recovery database from {}", database_name);
                        match self.recovery_database(database_name) {
                            Err(err) => return Err(Errs::strs("recovery database", err)),
                            _ => {}
                        }
                    }
                }
                Err(err) => return Err(Errs::strs("recovery databases path", err)),
            }
        }
        Ok(())
    }

    /// 恢复database数据
    fn recovery_database(&self, database_name: String) -> GeorgeResult<()> {
        // 恢复database数据
        let db = Database::recover(database_name)?;
        log::debug!(
            "db [name={}, comment={}, create time = {}]",
            db.name(),
            db.comment(),
            db.create_time().format("%Y-%m-%d %H:%M:%S"),
        );
        // 如果已存在该database，则不处理
        if !self.exist_database(db.name()) {
            self.database_map()
                .write()
                .unwrap()
                .insert(db.name(), Arc::new(RwLock::new(db)));
        }
        Ok(())
    }

    /// 恢复pages数据
    fn recovery_pages(&self, paths: ReadDir) -> GeorgeResult<()> {
        // 遍历data目录下文件
        for path in paths {
            match path {
                // 所有目录文件被默认为database根目录
                Ok(dir) => {
                    if dir.path().is_dir() {
                        let page_name = dir.file_name().to_str().unwrap().to_string();
                        log::debug!("recovery page from {}", page_name);
                        match self.recovery_page(page_name) {
                            Err(err) => return Err(Errs::strs("recovery page", err)),
                            _ => {}
                        }
                    }
                }
                Err(err) => return Err(Errs::strs("recovery page path", err)),
            }
        }
        Ok(())
    }

    /// 恢复page数据
    fn recovery_page(&self, page_name: String) -> GeorgeResult<()> {
        // 恢复database数据
        let page = Page::recover(page_name)?;
        log::debug!(
            "page [name={}, create time = {}]",
            page.name(),
            page.create_time().format("%Y-%m-%d %H:%M:%S"),
        );
        // 如果已存在该page，则不处理
        if !self.exist_page(page.name()) {
            self.page_map()
                .write()
                .unwrap()
                .insert(page.name(), Arc::new(RwLock::new(page)));
        }
        Ok(())
    }
}

pub(crate) fn init_log() {
    let module_main = log_module_main();
    let module_record = LogModule {
        name: "exec".to_string(),
        pkg: "db::task::master".to_string(),
        level: LevelFilter::Info,
        additive: true,
        dir: format!("{}/{}", module_main.dir, "records"),
        file_max_size: module_main.file_max_size,
        file_max_count: module_main.file_max_count,
    };
    module_main.set_log(vec![module_record]);
}

fn log_module_main() -> LogModule {
    let config = GLOBAL_CONFIG.read().unwrap();
    LogModule {
        name: String::from("db"),
        pkg: "".to_string(),
        level: log_level(config.log_level()),
        additive: true,
        dir: config.log_dir(),
        file_max_size: config.log_file_max_size(),
        file_max_count: config.log_file_max_count(),
    }
}

pub fn log_level(level: String) -> LevelFilter {
    match level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Off,
    }
}
