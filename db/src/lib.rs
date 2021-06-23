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

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use comm::errors::GeorgeResult;
use comm::Time;
use deploy::{Init, LogPolicy};

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
            "db::task::master".to_string(),
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
