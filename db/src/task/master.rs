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

use comm::errors::{Errs, GeorgeResult};
use comm::io::dir::DirHandler;
use comm::io::file::FilerHandler;
use comm::io::{Dir, Filer};
use comm::strings::StringHandler;
use comm::{Strings, Time};
use ge::utils::enums::Tag;
use ge::GeFactory;

use crate::task::engine::traits::TIndex;
use crate::task::rich::Expectation;
use crate::task::traits::TMaster;
use crate::task::{Database, Master};
use crate::task::{Page, View};
use crate::utils::comm::INDEX_DISK;
use crate::utils::enums::{Engine, KeyType};
use crate::utils::Paths;

impl Master {
    /// 生成Master
    pub(crate) fn generate() -> GeorgeResult<Self> {
        // 尝试创建数据根目录，有则什么也不做，无则创建
        Dir::mk_uncheck(Paths::data_path()).expect("create data path failed!");
        // 启动文件
        let bootstrap_file_path = Paths::bootstrap_filepath();
        let init: bool;
        let duration: Duration;
        if Filer::exist(bootstrap_file_path.clone()) {
            let ge = GeFactory {}.recovery(Tag::Bootstrap, bootstrap_file_path.clone())?;
            let description = ge.description_content_bytes()?;
            match Strings::from_utf8(description)?.parse::<i64>() {
                Ok(res) => duration = Duration::nanoseconds(res),
                Err(err) => return Err(Errs::strs("duration parse i64", err)),
            }
            init = true;
        } else {
            let now: NaiveDateTime = Local::now().naive_local();
            duration = Duration::nanoseconds(now.timestamp_nanos());
            let description = Some(
                duration
                    .num_nanoseconds()
                    .unwrap()
                    .to_string()
                    .as_bytes()
                    .to_vec(),
            );
            GeFactory {}.create(Tag::Bootstrap, bootstrap_file_path.clone(), description)?;
            init = false;
        }

        let create_time = Time::from(duration);
        log::info!(
            "george create at {}",
            create_time.to_string("%Y-%m-%d %H:%M:%S")
        );

        let master = Master {
            init,
            pages: Arc::new(Default::default()),
            databases: Default::default(),
            create_time,
        };
        if init {
            log::info!("recovery exist data from bootstrap");
            log::debug!(
                "recovery exist data from bootstrap file {}",
                bootstrap_file_path
            );
            master.recovery()?;
        }
        Ok(master)
    }

    fn exist_database(&self, database_name: String) -> bool {
        return match self.database(database_name) {
            Ok(_) => true,
            Err(_) => false,
        };
    }

    fn exist_page(&self, page_name: String) -> bool {
        return match self.page(page_name) {
            Ok(_) => true,
            Err(_) => false,
        };
    }
}

impl TMaster for Master {
    fn init(&self) -> bool {
        self.init
    }

    fn create_time(&self) -> Time {
        self.create_time.clone()
    }

    fn page_map(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<Page>>>>> {
        self.pages.clone()
    }

    fn page_create(
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

    fn page_remove(&self, page_name: String) -> GeorgeResult<()> {
        if !self.exist_page(page_name.clone()) {
            Err(Errs::page_exist_error())
        } else {
            self.page_map().write().unwrap().remove(&page_name);
            Dir::rm(Paths::page_path(page_name))
        }
    }

    fn page_modify(&self, page_name: String, page_new_name: String) -> GeorgeResult<()> {
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
        self.page_remove(page_name)
    }

    fn page(&self, page_name: String) -> GeorgeResult<Arc<RwLock<Page>>> {
        match self.page_map().read().unwrap().get(&page_name) {
            Some(page) => Ok(page.clone()),
            None => Err(Errs::page_no_exist_error()),
        }
    }

    fn database_map(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<Database>>>>> {
        self.databases.clone()
    }

    fn database_create(&self, database_name: String, database_comment: String) -> GeorgeResult<()> {
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

    fn database_remove(&self, database_name: String) -> GeorgeResult<()> {
        if !self.exist_database(database_name.clone()) {
            Err(Errs::database_exist_error())
        } else {
            self.database_map().write().unwrap().remove(&database_name);
            Dir::rm(Paths::database_path(database_name))
        }
    }

    fn database_modify(
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
        self.database_remove(database_name)?;
        self.recovery_database(database_new_name)
    }

    fn database(&self, database_name: String) -> GeorgeResult<Arc<RwLock<Database>>> {
        match self.database_map().read().unwrap().get(&database_name) {
            Some(database) => Ok(database.clone()),
            None => Err(Errs::database_no_exist_error()),
        }
    }

    fn view_map(
        &self,
        database_name: String,
    ) -> GeorgeResult<Arc<RwLock<HashMap<String, Arc<RwLock<View>>>>>> {
        Ok(self.database(database_name)?.read().unwrap().view_map())
    }

    fn view_create(
        &self,
        database_name: String,
        view_name: String,
        comment: String,
        with_increment: bool,
    ) -> GeorgeResult<()> {
        self.database(database_name)?.read().unwrap().create_view(
            view_name,
            comment,
            with_increment,
        )
    }

    fn view_modify(
        &self,
        database_name: String,
        view_name: String,
        view_new_name: String,
        comment: String,
    ) -> GeorgeResult<()> {
        self.database(database_name)?.write().unwrap().modify_view(
            view_name,
            view_new_name,
            comment,
        )
    }

    fn view_archive(
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

    fn view_record(
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

    fn view_records(
        &self,
        database_name: String,
        view_name: String,
    ) -> GeorgeResult<Vec<(String, Time, u16)>> {
        self.database(database_name)?
            .read()
            .unwrap()
            .view_records(view_name)
    }

    fn view_remove(&self, database_name: String, view_name: String) -> GeorgeResult<()> {
        self.database(database_name)?
            .read()
            .unwrap()
            .remove_view(view_name)
    }

    fn view(&self, database_name: String, view_name: String) -> GeorgeResult<Arc<RwLock<View>>> {
        self.database(database_name)?
            .read()
            .unwrap()
            .view(view_name)
    }

    fn index_map(
        &self,
        database_name: String,
        view_name: String,
    ) -> GeorgeResult<Arc<RwLock<HashMap<String, Arc<dyn TIndex>>>>> {
        Ok(self
            .view(database_name, view_name)?
            .read()
            .unwrap()
            .index_map())
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
        let view = self.view(database_name, view_name)?;
        view.clone()
            .read()
            .unwrap()
            .create_index(view, index_name, engine, key_type, primary, unique, null)
    }

    fn index(
        &self,
        database_name: String,
        view_name: String,
        name: String,
    ) -> GeorgeResult<Arc<dyn TIndex>> {
        self.view(database_name, view_name)?
            .read()
            .unwrap()
            .index(&name)
    }

    fn put_disk(
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

    fn set_disk(
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

    fn get_disk(
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

    fn get_disk_by_index(
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

    fn remove_disk(
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

    fn select_disk(
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

    fn delete_disk(
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

    fn put_memory(&self, page_name: String, key: String, value: Vec<u8>) -> GeorgeResult<()> {
        self.page(page_name)?.read().unwrap().put(key, value)
    }

    fn set_memory(&self, page_name: String, key: String, value: Vec<u8>) -> GeorgeResult<()> {
        self.page(page_name)?.read().unwrap().set(key, value)
    }

    fn get_memory(&self, page_name: String, key: String) -> GeorgeResult<Vec<u8>> {
        self.page(page_name)?.read().unwrap().get(key)
    }

    fn remove_memory(&self, page_name: String, key: String) -> GeorgeResult<()> {
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
