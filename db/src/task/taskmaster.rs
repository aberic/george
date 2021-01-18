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
use crate::utils::comm::{Capacity, EngineType, IndexType, GEORGE_DB_CONFIG};
use crate::utils::deploy::{init_config, GLOBAL_CONFIG};
use crate::utils::path::{bootstrap_file_path, data_path, database_file_path};
use crate::utils::store::{metadata_2_bytes, recovery_before_content, Metadata, Tag};
use chrono::{Duration, Local, NaiveDateTime};
use comm::env;
use comm::errors::children::{DatabaseExistError, DatabaseNoExistError};
use comm::errors::entrances::{GeorgeError, GeorgeResult};
use comm::io::file::{create_dir, create_file};
use comm::io::writer::write_append_bytes;
use logs::set_log;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs::{read_dir, read_to_string, ReadDir};
use std::sync::{Arc, RwLock};

/// 数据库
pub(crate) struct Master {
    /// 视图索引集合
    databases: Arc<RwLock<HashMap<String, Arc<RwLock<Database>>>>>,
    /// 创建时间
    create_time: Duration,
}

impl Master {
    pub fn database_map(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<Database>>>>> {
        self.databases.clone()
    }
    pub fn create_time(&self) -> Duration {
        self.create_time
    }
    /// 创建数据库
    pub fn create_database(
        &self,
        database_name: String,
        _database_comment: String,
    ) -> GeorgeResult<()> {
        if self.exist_database(database_name.clone()) {
            return Err(GeorgeError::DatabaseExistError(DatabaseExistError));
        }
        let db = Database::create(database_name.clone())?;
        self.database_map()
            .write()
            .unwrap()
            .insert(database_name.clone(), db.clone());
        log::info!("create database {} success!", database_name.clone());
        Ok(())
    }
    /// 修改数据库
    pub fn modify_database(&self, name: String, new_name: String) -> GeorgeResult<()> {
        if !self.exist_database(name.clone()) {
            return Err(GeorgeError::DatabaseNoExistError(DatabaseNoExistError));
        }
        if self.exist_database(new_name.clone()) {
            return Err(GeorgeError::DatabaseExistError(DatabaseExistError));
        }
        let databases = self.database_map();
        let mut databases_w = databases.write().unwrap();
        let database = databases_w.get(&name).unwrap().clone();
        database.clone().write().unwrap().modify(new_name.clone())?;
        databases_w.remove(&name);
        databases_w.insert(new_name.clone(), database.clone());
        Ok(())
    }
    fn exist_database(&self, database_name: String) -> bool {
        return match self
            .database_map()
            .read()
            .unwrap()
            .get(database_name.as_str())
        {
            Some(_) => true,
            None => false,
        };
    }
    /// 创建视图
    pub(crate) fn create_view(
        &self,
        database_name: String,
        view_name: String,
        _view_comment: String,
    ) -> GeorgeResult<()> {
        match self.database_map().read().unwrap().get(&database_name) {
            Some(database_lock) => {
                let database = database_lock.read().unwrap();
                database.create_view(view_name.clone())?;
            }
            None => return Err(GeorgeError::DatabaseNoExistError(DatabaseNoExistError)),
        }
        // self.create_index(
        //     database_name,
        //     view_name,
        //     INDEX_CATALOG.to_string(),
        //     IndexMold::String,
        //     true,
        // ) todo
        Ok(())
    }
    /// 修改视图
    pub(crate) fn modify_view(
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
            None => return Err(GeorgeError::DatabaseNoExistError(DatabaseNoExistError)),
        }
    }
}

impl Master {
    /// 初始化或恢复数据
    fn init_or_recovery(&self) {
        let bootstrap_file = bootstrap_file_path();
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
        log::debug!("bootstrap init!");
        // 创建系统库，用户表(含权限等信息)、库历史记录表(含变更、归档等信息) todo
        match write_append_bytes(bootstrap_file_path(), vec![0x01]) {
            Err(err) => panic!("init failed! error is {}", err),
            _ => {}
        }
    }

    /// 恢复sky数据
    fn recovery(&self) {
        log::debug!("bootstrap recovery!");
        // 读取data目录下所有文件
        match read_dir(data_path()) {
            Ok(paths) => self.recovery_databases(paths),
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
        match recovery_before_content(Tag::Database, database_file_path(database_name)) {
            Ok(hd) => {
                // 恢复database数据
                match Database::recover(hd) {
                    Ok(db) => {
                        log::debug!(
                            "db [name={}, create time ={}]",
                            db.name(),
                            db.create_time().num_nanoseconds().unwrap().to_string(),
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

pub(crate) static GLOBAL_MASTER: Lazy<Arc<Master>> = Lazy::new(|| {
    let now: NaiveDateTime = Local::now().naive_local();
    let create_time = Duration::nanoseconds(now.timestamp_nanos());
    init_log();
    log::info!("log init success!");
    init_config(config_path());
    log::info!("config init success!");
    let master = Master {
        databases: Default::default(),
        create_time,
    };
    let master_arc = Arc::new(master);
    // 创建数据根目录
    match create_dir(data_path()) {
        Ok(_file) => log::info!("load data path success!"),
        Err(err) => panic!("create data path failed! error is {}", err),
    }
    // 创建引导文件
    match create_file(bootstrap_file_path(), false) {
        Ok(_f) => master_arc.clone().init_or_recovery(),
        Err(err) => panic!("create bootstrap file failed! error is {}", err),
    }
    master_arc
});

fn config_path() -> String {
    env::get(GEORGE_DB_CONFIG, "src/examples/conf.yaml")
}

fn init_log() {
    let config = GLOBAL_CONFIG.read().unwrap();
    set_log(
        String::from("db"),
        config.log_dir(),
        config.log_file_max_size(),
        config.log_file_max_count(),
        config.log_level(),
    );
}
