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

use crate::task::database::Database;
use crate::task::rich::Expectation;
use crate::utils::comm::{
    DEFAULT_COMMENT, DEFAULT_DATABASE, DEFAULT_VIEW, GEORGE_DB_CONFIG, INDEX_CATALOG, INDEX_MEMORY,
};
use crate::utils::deploy::{init_config, GLOBAL_CONFIG};
use crate::utils::enums::{IndexType, KeyType};
use crate::utils::path::{bootstrap_filepath, data_path, database_filepath};
use crate::utils::store::recovery_before_content;
use chrono::{Duration, Local, NaiveDateTime};
use comm::env;
use comm::errors::children::{DatabaseExistError, DatabaseNoExistError};
use comm::errors::entrances::{GeorgeError, GeorgeResult};
use comm::io::dir::{Dir, DirHandler};
use comm::io::file::{Filer, FilerHandler, FilerWriter};
use log::LevelFilter;
use logs::{log_level, set_log, LogModule};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs::{read_dir, read_to_string, ReadDir};
use std::sync::{Arc, RwLock};

/// 数据库
pub(super) struct Master {
    /// 视图索引集合
    databases: Arc<RwLock<HashMap<String, Arc<RwLock<Database>>>>>,
    /// 创建时间
    create_time: Duration,
}

impl Master {
    pub(super) fn database_map(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<Database>>>>> {
        self.databases.clone()
    }
    pub(super) fn create_time(&self) -> Duration {
        self.create_time
    }
    /// 创建数据库
    pub(super) fn create_database(
        &self,
        database_name: String,
        _database_comment: String,
    ) -> GeorgeResult<()> {
        if self.exist_database(database_name.clone()) {
            return Err(GeorgeError::from(DatabaseExistError));
        }
        let db = Database::create(database_name.clone())?;
        self.database_map()
            .write()
            .unwrap()
            .insert(database_name.clone(), db.clone());
        log::debug!("create database {} success!", database_name);
        Ok(())
    }
    /// 修改数据库
    pub(super) fn modify_database(&self, name: String, new_name: String) -> GeorgeResult<()> {
        if !self.exist_database(name.clone()) {
            return Err(GeorgeError::from(DatabaseNoExistError));
        }
        if self.exist_database(new_name.clone()) {
            return Err(GeorgeError::from(DatabaseExistError));
        }
        let databases = self.database_map();
        let mut databases_w = databases.write().unwrap();
        let database = databases_w.get(&name).unwrap().clone();
        database.clone().write().unwrap().modify(new_name.clone())?;
        databases_w.remove(&name);
        databases_w.insert(new_name.clone(), database.clone());
        Ok(())
    }
    /// 根据库name获取库
    pub(super) fn database(&self, database_name: String) -> GeorgeResult<Arc<RwLock<Database>>> {
        match self.database_map().read().unwrap().get(&database_name) {
            Some(database) => Ok(database.clone()),
            None => Err(GeorgeError::from(DatabaseNoExistError)),
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
        _view_comment: String,
        mem: bool,
    ) -> GeorgeResult<()> {
        match self.database_map().read().unwrap().get(&database_name) {
            Some(database_lock) => {
                let database = database_lock.read().unwrap();
                database.create_view(view_name, mem)?;
            }
            None => return Err(GeorgeError::from(DatabaseNoExistError)),
        }
        Ok(())
    }
    /// 修改视图
    pub(super) fn modify_view(
        &self,
        database_name: String,
        view_name: String,
        view_new_name: String,
    ) -> GeorgeResult<()> {
        match self.database_map().read().unwrap().get(&database_name) {
            Some(database_lock) => {
                let database = database_lock.write().unwrap();
                database.modify_view(view_name, view_new_name)
            }
            None => return Err(GeorgeError::from(DatabaseNoExistError)),
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
    /// 当前视图文件地址
    pub(super) fn read_content_by(
        &self,
        database_name: String,
        view_name: String,
        view_info_index: Vec<u8>,
    ) -> GeorgeResult<Vec<u8>> {
        self.database(database_name)?
            .read()
            .unwrap()
            .view(view_name)?
            .read()
            .unwrap()
            .read_content_by(view_info_index)
    }
    /// 在指定库及视图中创建索引
    ///
    /// 该索引需要定义ID，此外索引所表达的字段组成内容也是必须的，并通过primary判断索引类型，具体传参参考如下定义：<p><p>
    ///
    /// ###Params
    ///
    /// index_name 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入
    ///
    /// primary 是否主键
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
        database_read
            .view(view_name)?
            .read()
            .unwrap()
            .create_index(index_name, index_type, key_type, primary, unique, null)
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
    pub(crate) fn put(
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
    pub(crate) fn set(
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
    pub(crate) fn get(
        &self,
        database_name: String,
        view_name: String,
        key: String,
    ) -> GeorgeResult<Vec<u8>> {
        self.database(database_name)?
            .read()
            .unwrap()
            .get(view_name, INDEX_CATALOG, key)
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
    pub(crate) fn get_by_index(
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
    pub(crate) fn remove(
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
    pub fn select(
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
    pub fn delete(
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
    pub(crate) fn put_m(&self, key: String, value: Vec<u8>) -> GeorgeResult<()> {
        self.database(DEFAULT_DATABASE.to_string())?
            .read()
            .unwrap()
            .put(DEFAULT_VIEW.to_string(), key, value)
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
    pub(crate) fn set_m(&self, key: String, value: Vec<u8>) -> GeorgeResult<()> {
        self.database(DEFAULT_DATABASE.to_string())?
            .read()
            .unwrap()
            .set(DEFAULT_VIEW.to_string(), key, value)
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
    pub(crate) fn get_m(&self, key: String) -> GeorgeResult<Vec<u8>> {
        self.database(DEFAULT_DATABASE.to_string())?
            .read()
            .unwrap()
            .get(DEFAULT_VIEW.to_string(), INDEX_MEMORY, key)
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
    pub(crate) fn remove_m(&self, key: String) -> GeorgeResult<()> {
        self.database(DEFAULT_DATABASE.to_string())?
            .read()
            .unwrap()
            .remove(DEFAULT_VIEW.to_string(), key)
    }
}

impl Master {
    /// 初始化或恢复数据
    fn init_or_recovery(&self) {
        let bootstrap_file = bootstrap_filepath();
        match read_to_string(bootstrap_file.clone()) {
            Ok(text) => {
                if text.is_empty() {
                    log::info!("initialize new data");
                    self.init()
                } else {
                    log::info!("recovery exist data from bootstrap");
                    log::debug!("recovery exist data from bootstrap file {}", bootstrap_file);
                    self.recovery()
                }
            }
            Err(err) => panic!("init_or_recovery failed! error is {}", err),
        }
    }

    /// 初始化
    fn init(&self) {
        log::info!("bootstrap init!");
        // 创建系统库，用户表(含权限等信息)、库历史记录表(含变更、归档等信息) todo
        match Filer::write_force(bootstrap_filepath(), vec![0x01]) {
            Err(err) => panic!("init failed! error is {}", err),
            _ => self.init_default(),
        }
    }

    fn init_default(&self) {
        match self.create_database(DEFAULT_DATABASE.to_string(), DEFAULT_COMMENT.to_string()) {
            _ => {}
        }
        match self.create_view(
            DEFAULT_DATABASE.to_string(),
            DEFAULT_VIEW.to_string(),
            DEFAULT_COMMENT.to_string(),
            true,
        ) {
            _ => {}
        }
    }

    /// 恢复sky数据
    fn recovery(&self) {
        log::info!("bootstrap recovery!");
        // 读取data目录下所有文件
        match read_dir(data_path()) {
            Ok(paths) => {
                self.init_default();
                self.recovery_databases(paths)
            }
            Err(err) => panic!("recovery failed! error is {}", err),
        }
    }

    /// 恢复databases数据
    fn recovery_databases(&self, paths: ReadDir) {
        // 遍历data目录下文件
        for path in paths {
            match path {
                // 所有目录文件被默认为database根目录
                Ok(dir) => {
                    if dir.path().is_dir() {
                        let database_name = dir.file_name().to_str().unwrap().to_string();
                        log::debug!("recovery database from {}", database_name);
                        self.recovery_database(database_name);
                    }
                }
                Err(err) => panic!("recovery databases failed! error is {}", err),
            }
        }
    }

    /// 恢复database数据
    fn recovery_database(&self, database_name: String) {
        match recovery_before_content(database_filepath(database_name)) {
            Ok(hd) => {
                // 恢复database数据
                match Database::recover(hd.clone()) {
                    Ok(db) => {
                        log::debug!(
                            "db [name={}, create time = {}, {:#?}]",
                            db.name(),
                            db.create_time().num_nanoseconds().unwrap().to_string(),
                            hd.metadata()
                        );
                        // 如果已存在该database，则不处理
                        if self.exist_database(db.name()) {
                            return;
                        }
                        self.database_map()
                            .write()
                            .unwrap()
                            .insert(db.name(), Arc::new(RwLock::new(db)));
                    }
                    Err(err) => panic!("recovery database failed! error is {}", err),
                }
            }
            Err(err) => panic!("{}", err),
        }
    }
}

pub(super) static GLOBAL_MASTER: Lazy<Arc<Master>> = Lazy::new(|| {
    let now: NaiveDateTime = Local::now().naive_local();
    let create_time = Duration::nanoseconds(now.timestamp_nanos());
    init_config(config_path());
    init_log();
    log::info!("config & log init success!");
    let master = Master {
        databases: Default::default(),
        create_time,
    };
    let master_arc = Arc::new(master);
    // 创建数据根目录
    match Dir::mk(data_path()) {
        Ok(_file) => log::info!("load data path success!"),
        Err(err) => panic!("create data path failed! error is {}", err),
    }
    let bootstrap_file_path = bootstrap_filepath();
    match Filer::exist(bootstrap_file_path.clone()) {
        Ok(b) => {
            if !b {
                // 创建引导文件
                match Filer::touch(bootstrap_file_path) {
                    Err(err) => panic!("create bootstrap file failed! error is {}", err),
                    _ => {}
                }
            }
        }
        Err(err) => panic!("create bootstrap file failed! error is {}", err),
    }
    master_arc.clone().init_or_recovery();
    master_arc
});

fn config_path() -> String {
    env::get(GEORGE_DB_CONFIG, "src/examples/conf.yaml")
}

fn init_log() {
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
    set_log(module_main, vec![module_record]);
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
