/*
 * Copyright (c) 2020. Aberic - All Rights Reserved.
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

//! 一个数据库底层实现，并致力于做到与数据大小无关性能平衡的、支持文件写准确预执行的、通用开源的框架。<p>
//!
//! [`george-db`]提供外部交互接口[`TMaster`]，由[`Task`]暴露使用，内部交付[`Master`]进行具体实现，同时
//! 也会提供诸如[`Database`]、[`Page`]、[`View`]、[`TIndex`]、[`Engine`]、[`KeyType`]和[`Expectation`]
//! 等结构体或接口。
//!
//! # 数据库底层
//! `george`底层基于`B+Tree`和`递增链式结构`实现。<p>
//! 无序数据入库使用`B+Tree`索引，支持64位，数据的增删查改效率与数据体量关联性不大。<p>
//! 有序或依赖底层自增数据入库使用`递增链式结构`实现，支持64位，存在非空占位的情况，非顺序递增正数`key`避免使用。
//!
//! # 存储支持
//! `george`支持磁盘存储和内存存储两种方式，磁盘存储支持富查询，参考[`Constraint`]实现。内存存储仅使用`B+Tree`索引，在
//! 计划中会实现富查询，并实现与磁盘存储组合富查询，以实现效率最大化。
//!
//! # 案例集
//! 相关案例可以参考[`george-examples`]
//!
//! # 应用方法
//! 相关应用可以参考[`george-server`]和[`george-client`]

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use george_comm::errors::GeorgeResult;
use george_comm::Time;
use george_deploy::{Init, LogPolicy};

use crate::task::engine::traits::TIndex;
use crate::task::rich::Expectation;
use crate::task::traits::TMaster;
use crate::task::{Database, Master, Page, View, GLOBAL_THREAD_POOL};
use crate::utils::deploy::GLOBAL_CONFIG;
use crate::utils::enums::{Engine, KeyType};

mod example;
pub mod task;
pub mod utils;

#[derive(Debug, Clone)]
pub struct Task {
    master: Master,
}

impl Task {
    pub fn default() -> GeorgeResult<Task> {
        let init = Init::from("src/example/conf.yaml").unwrap();
        GLOBAL_CONFIG.write().unwrap().init(init.db_unwrap());
        log::info!("config & log init success!");
        GLOBAL_THREAD_POOL.init();
        log::info!("thread pool init success!");
        Ok(Task {
            master: Master::generate()?,
        })
    }

    pub fn new(init: Init) -> GeorgeResult<Task> {
        GLOBAL_CONFIG.write().unwrap().init(init.db_unwrap());
        init.add_log_policy(LogPolicy::new(
            format!("{}/{}", init.log_dir_unwrap(), "records"),
            "exec".to_string(),
            "george-db::task::master".to_string(),
            true,
        ));
        log::info!("config & log init success!");
        GLOBAL_THREAD_POOL.init();
        log::info!("thread pool init success!");
        Ok(Task {
            master: Master::generate()?,
        })
    }
}

impl TMaster for Task {
    fn init(&self) -> bool {
        self.master.init()
    }

    fn create_time(&self) -> Time {
        self.master.create_time()
    }

    fn page_map(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<Page>>>>> {
        self.master.page_map()
    }

    fn page_create(
        &self,
        name: String,
        comment: String,
        size: u64,
        period: u32,
    ) -> GeorgeResult<()> {
        self.master.page_create(name, comment, size, period)
    }

    fn page_remove(&self, page_name: String) -> GeorgeResult<()> {
        self.master.page_remove(page_name)
    }

    fn page_modify(&self, page_name: String, page_new_name: String) -> GeorgeResult<()> {
        self.master.page_modify(page_name, page_new_name)
    }

    fn page(&self, page_name: String) -> GeorgeResult<Arc<RwLock<Page>>> {
        self.master.page(page_name)
    }

    fn database_map(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<Database>>>>> {
        self.master.database_map()
    }

    fn database_create(&self, database_name: String, database_comment: String) -> GeorgeResult<()> {
        self.master.database_create(database_name, database_comment)
    }

    fn database_remove(&self, database_name: String) -> GeorgeResult<()> {
        self.master.database_remove(database_name)
    }

    fn database_modify(
        &self,
        database_name: String,
        database_new_name: String,
        database_comment: String,
    ) -> GeorgeResult<()> {
        self.master
            .database_modify(database_name, database_new_name, database_comment)
    }

    fn database(&self, database_name: String) -> GeorgeResult<Arc<RwLock<Database>>> {
        self.master.database(database_name)
    }

    fn view_map(
        &self,
        database_name: String,
    ) -> GeorgeResult<Arc<RwLock<HashMap<String, Arc<RwLock<View>>>>>> {
        self.master.view_map(database_name)
    }

    fn view_create(
        &self,
        database_name: String,
        view_name: String,
        comment: String,
        with_increment: bool,
    ) -> GeorgeResult<()> {
        self.master
            .view_create(database_name, view_name, comment, with_increment)
    }

    fn view_modify(
        &self,
        database_name: String,
        view_name: String,
        view_new_name: String,
        comment: String,
    ) -> GeorgeResult<()> {
        self.master
            .view_modify(database_name, view_name, view_new_name, comment)
    }

    fn view_archive(
        &self,
        database_name: String,
        view_name: String,
        archive_file_path: String,
    ) -> GeorgeResult<()> {
        self.master
            .view_archive(database_name, view_name, archive_file_path)
    }

    fn view_record(
        &self,
        database_name: String,
        view_name: String,
        version: u16,
    ) -> GeorgeResult<(String, Time)> {
        self.master.view_record(database_name, view_name, version)
    }

    fn view_records(
        &self,
        database_name: String,
        view_name: String,
    ) -> GeorgeResult<Vec<(String, Time, u16)>> {
        self.master.view_records(database_name, view_name)
    }

    fn view_remove(&self, database_name: String, view_name: String) -> GeorgeResult<()> {
        self.master.view_remove(database_name, view_name)
    }

    fn view(&self, database_name: String, view_name: String) -> GeorgeResult<Arc<RwLock<View>>> {
        self.master.view(database_name, view_name)
    }

    fn index_map(
        &self,
        database_name: String,
        view_name: String,
    ) -> GeorgeResult<Arc<RwLock<HashMap<String, Arc<dyn TIndex>>>>> {
        self.master.index_map(database_name, view_name)
    }

    fn index_create(
        &self,
        database_name: String,
        view_name: String,
        index_name: String,
        engine: Engine,
        key_type: KeyType,
        primary: bool,
        unique: bool,
        null: bool,
    ) -> GeorgeResult<()> {
        self.master.index_create(
            database_name,
            view_name,
            index_name,
            engine,
            key_type,
            primary,
            unique,
            null,
        )
    }

    fn index(
        &self,
        database_name: String,
        view_name: String,
        name: String,
    ) -> GeorgeResult<Arc<dyn TIndex>> {
        self.master.index(database_name, view_name, name)
    }

    fn put_disk(
        &self,
        database_name: String,
        view_name: String,
        key: String,
        value: Vec<u8>,
    ) -> GeorgeResult<()> {
        self.master.put_disk(database_name, view_name, key, value)
    }

    fn set_disk(
        &self,
        database_name: String,
        view_name: String,
        key: String,
        value: Vec<u8>,
    ) -> GeorgeResult<()> {
        self.master.set_disk(database_name, view_name, key, value)
    }

    fn get_disk(
        &self,
        database_name: String,
        view_name: String,
        key: String,
    ) -> GeorgeResult<Vec<u8>> {
        self.master.get_disk(database_name, view_name, key)
    }

    fn get_disk_by_index(
        &self,
        database_name: String,
        view_name: String,
        index_name: String,
        key: String,
    ) -> GeorgeResult<Vec<u8>> {
        self.master
            .get_disk_by_index(database_name, view_name, index_name, key)
    }

    fn remove_disk(
        &self,
        database_name: String,
        view_name: String,
        key: String,
    ) -> GeorgeResult<()> {
        self.master.remove_disk(database_name, view_name, key)
    }

    fn select_disk(
        &self,
        database_name: String,
        view_name: String,
        constraint_json_bytes: Vec<u8>,
    ) -> GeorgeResult<Expectation> {
        self.master
            .select_disk(database_name, view_name, constraint_json_bytes)
    }

    fn delete_disk(
        &self,
        database_name: String,
        view_name: String,
        constraint_json_bytes: Vec<u8>,
    ) -> GeorgeResult<Expectation> {
        self.master
            .delete_disk(database_name, view_name, constraint_json_bytes)
    }

    fn put_memory(&self, page_name: String, key: String, value: Vec<u8>) -> GeorgeResult<()> {
        self.master.put_memory(page_name, key, value)
    }

    fn set_memory(&self, page_name: String, key: String, value: Vec<u8>) -> GeorgeResult<()> {
        self.master.set_memory(page_name, key, value)
    }

    fn get_memory(&self, page_name: String, key: String) -> GeorgeResult<Vec<u8>> {
        self.master.get_memory(page_name, key)
    }

    fn remove_memory(&self, page_name: String, key: String) -> GeorgeResult<()> {
        self.master.remove_memory(page_name, key)
    }
}
