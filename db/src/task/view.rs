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

use crate::task::engine::dossier::index::Index;
use crate::task::engine::traits::TIndex;
use crate::utils::comm::{EngineType, IndexMold};
use crate::utils::path::{index_file_path, view_file_path, view_path};
use crate::utils::store::{
    before_content_bytes, metadata_2_bytes, recovery_before_content, Metadata, Tag, HD,
};
use crate::utils::writer::obtain_write_append_file;
use chrono::{Duration, Local, NaiveDateTime};
use comm::errors::children::IndexExistError;
use comm::errors::entrances::{err_str, err_str_enhance, err_string, GeorgeError, GeorgeResult};
use comm::io::file::create_file;
use comm::io::reader::read_sub_bytes;
use comm::io::writer::{write_file_append_bytes, write_seek_u8s};
use std::collections::HashMap;
use std::fs::{read_dir, File, ReadDir};
use std::io::{Seek, SeekFrom};
use std::sync::{Arc, RwLock};

/// 视图，类似表
#[derive(Debug, Clone)]
pub(crate) struct View {
    /// 名称
    name: String,
    /// 创建时间
    create_time: Duration,
    /// 文件信息
    metadata: Metadata,
    /// 根据文件路径获取该文件追加写入的写对象
    file_append: Arc<RwLock<File>>,
    /// 索引集合
    indexes: Arc<RwLock<HashMap<String, Arc<RwLock<dyn TIndex>>>>>,
}

/// 新建视图
///
/// 具体传参参考如下定义：<p><p>
///
/// ###Params
///
/// id 视图唯一ID
///
/// name 视图名称
///
/// comment 视图描述
///
/// category 视图类型
///
/// level 视图规模/级别
fn new_view(database_name: String, name: String) -> GeorgeResult<View> {
    let now: NaiveDateTime = Local::now().naive_local();
    let create_time = Duration::nanoseconds(now.timestamp_nanos());
    let file_path = view_file_path(database_name, name.clone());
    let file_append = obtain_write_append_file(file_path)?;
    let view = View {
        name,
        create_time,
        metadata: Metadata::default(Tag::View),
        file_append,
        indexes: Default::default(),
    };
    Ok(view)
}

impl View {
    pub(crate) fn create(database_name: String, name: String) -> GeorgeResult<Arc<RwLock<View>>> {
        create_file(view_file_path(database_name.clone(), name.clone()), true)?;
        let mut view = new_view(database_name.clone(), name)?;
        let mut metadata_bytes = metadata_2_bytes(view.metadata());
        let mut description = view.description();
        // 初始化为32 + 8，即head长度加正文描述符长度
        let mut before_description = before_content_bytes(40, description.len() as u32);
        metadata_bytes.append(&mut before_description);
        metadata_bytes.append(&mut description);
        view.file_append(database_name, metadata_bytes)?;
        Ok(Arc::new(RwLock::new(view)))
    }
    /// 名称
    pub(crate) fn name(&self) -> String {
        self.name.clone()
    }
    /// 创建时间
    pub(crate) fn create_time(&self) -> Duration {
        self.create_time.clone()
    }
    /// 文件信息
    pub(crate) fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }
    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
    ///
    /// #Return
    ///
    /// seek_end_before 写之前文件字节数据长度
    fn file_append(&mut self, database_name: String, content: Vec<u8>) -> GeorgeResult<u64> {
        let file_append = self.file_append.clone();
        let mut file_write = file_append.write().unwrap();
        match file_write.seek(SeekFrom::End(0)) {
            Ok(seek_end_before) => {
                match write_file_append_bytes(file_write.try_clone().unwrap(), content.clone()) {
                    Ok(()) => Ok(seek_end_before),
                    Err(_err) => {
                        let file_path = view_file_path(database_name, self.name());
                        self.file_append = obtain_write_append_file(file_path)?;
                        let file_write_again = self.file_append.write().unwrap();
                        write_file_append_bytes(file_write_again.try_clone().unwrap(), content)?;
                        Ok(seek_end_before)
                    }
                }
            }
            Err(_err) => {
                let file_path = view_file_path(database_name, self.name());
                self.file_append = obtain_write_append_file(file_path)?;
                let mut file_write_again = self.file_append.write().unwrap();
                let seek_end_before_again = file_write_again.seek(SeekFrom::End(0)).unwrap();
                write_file_append_bytes(file_write_again.try_clone().unwrap(), content)?;
                Ok(seek_end_before_again)
            }
        }
    }
    /// 索引集合
    pub(crate) fn index_map(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<dyn TIndex>>>>> {
        self.indexes.clone()
    }
    /// 视图变更
    pub(crate) fn modify(&mut self, database_name: String, name: String) -> GeorgeResult<()> {
        let old_name = self.name();
        let filepath = view_file_path(database_name.clone(), old_name.clone());
        let content_old = read_sub_bytes(filepath.clone(), 0, 40)?;
        self.name = name;
        let description = self.description();
        let seek_end = self.file_append(database_name.clone(), description.clone())?;
        log::debug!(
            "view {} modify to {} with file seek_end = {}",
            old_name.clone(),
            self.name(),
            seek_end
        );
        let content_new = before_content_bytes(seek_end as u32, description.len() as u32);
        // 更新首部信息，初始化head为32，描述起始4字节，长度4字节
        write_seek_u8s(filepath.clone(), 32, content_new.as_slice())?;
        let view_path_old = view_path(database_name.clone(), old_name);
        let view_path_new = view_path(database_name.clone(), self.name());
        match std::fs::rename(view_path_old, view_path_new) {
            Ok(_) => Ok(()),
            Err(err) => {
                // 回滚数据
                write_seek_u8s(filepath, 0, content_old.as_slice())?;
                Err(err_str_enhance("file rename error: ", err.to_string()))
            }
        }
    }
    fn exist_index(&self, index_name: String) -> bool {
        return match self.index_map().read().unwrap().get(index_name.as_str()) {
            Some(_) => true,
            None => false,
        };
    }
    /// 创建索引
    pub(crate) fn create_index(
        &self,
        database_name: String,
        index_name: String,
        engine_type: EngineType,
        index_mold: IndexMold,
        primary: bool,
    ) -> GeorgeResult<()> {
        if self.exist_index(index_name.clone()) {
            return Err(GeorgeError::IndexExistError(IndexExistError));
        }
        let view_name = self.name();
        let name = index_name.clone();
        let index;
        match engine_type {
            EngineType::None => return Err(err_str("unsupported engine type with none")),
            EngineType::Memory => {
                index = Index::create(database_name, view_name, name, primary, index_mold)?
            }
            EngineType::Dossier => {
                index = Index::create(database_name, view_name, name, primary, index_mold)?
            }
            EngineType::Library => {
                index = Index::create(database_name, view_name, name, primary, index_mold)?
            }
            EngineType::Block => {
                index = Index::create(database_name, view_name, name, primary, index_mold)?
            }
        }
        self.index_map()
            .write()
            .unwrap()
            .insert(index_name, index.clone());
        Ok(())
    }
}

impl View {
    /// 生成文件描述
    fn description(&self) -> Vec<u8> {
        hex::encode(format!(
            "{}/{}",
            self.name(),
            self.create_time().num_nanoseconds().unwrap().to_string()
        ))
        .into_bytes()
    }
    /// 通过文件描述恢复结构信息
    pub(crate) fn recover(database_name: String, hd: HD) -> GeorgeResult<View> {
        match String::from_utf8(hd.description()) {
            Ok(description_str) => match hex::decode(description_str) {
                Ok(vu8) => match String::from_utf8(vu8) {
                    Ok(real) => {
                        let mut split = real.split("/");
                        let name = split.next().unwrap().to_string();
                        let create_time = Duration::nanoseconds(
                            split.next().unwrap().to_string().parse::<i64>().unwrap(),
                        );
                        let file_path = view_file_path(database_name.clone(), name.clone());
                        let file_append = obtain_write_append_file(file_path)?;
                        let mut view = View {
                            name,
                            create_time,
                            metadata: hd.metadata(),
                            file_append,
                            indexes: Arc::new(Default::default()),
                        };
                        log::info!(
                            "recovery view {} from database {}",
                            view.name(),
                            database_name,
                        );
                        match read_dir(view_path(database_name.clone(), view.name())) {
                            // 恢复indexes数据
                            Ok(paths) => view.recovery_indexes(database_name, paths),
                            Err(err) => panic!("recovery view read dir failed! error is {}", err),
                        }
                        Ok(view)
                    }
                    Err(err) => Err(err_string(format!(
                        "recovery index from utf8 2 failed! error is {}",
                        err
                    ))),
                },
                Err(err) => Err(err_string(format!(
                    "recovery view decode failed! error is {}",
                    err
                ))),
            },
            Err(err) => Err(err_string(format!(
                "recovery index from utf8 1 failed! error is {}",
                err
            ))),
        }
    }
}

impl View {
    /// 恢复indexes数据
    fn recovery_indexes(&mut self, database_name: String, paths: ReadDir) {
        // 遍历view目录下文件
        for path in paths {
            match path {
                // 所有目录文件被默认为index根目录
                Ok(dir) => {
                    if dir.path().is_dir() {
                        let index_name = dir.file_name().to_str().unwrap().to_string();
                        log::debug!("recovery index from {}", index_name);
                        // 恢复index数据
                        self.recovery_index(database_name.clone(), index_name.clone());
                    }
                }
                Err(err) => panic!("recovery indexes path failed! error is {}", err),
            }
        }
    }

    /// 恢复view数据
    fn recovery_index(&self, database_name: String, index_name: String) {
        let index_file_path =
            index_file_path(database_name.clone(), self.name(), index_name.clone());
        match recovery_before_content(index_file_path.clone()) {
            Ok(hd) => {
                // 恢复view数据
                match Index::recover(database_name.clone(), self.name(), hd.clone()) {
                    Ok(index) => {
                        log::debug!(
                            "index [db={}, view={}, name={}, create_time={}, {:#?}]",
                            database_name.clone(),
                            self.name(),
                            index_name.clone(),
                            index
                                .clone()
                                .read()
                                .unwrap()
                                .create_time()
                                .num_nanoseconds()
                                .unwrap()
                                .to_string(),
                            hd.metadata()
                        );
                        // 如果已存在该view，则不处理
                        if self.exist_index(index_name.clone()) {
                            return;
                        }
                        self.index_map().write().unwrap().insert(index_name, index);
                    }
                    Err(err) => panic!("recovery index failed! error is {}", err),
                }
            }
            Err(err) => panic!(
                "recovery index when recovery before content failed! error is {}",
                err
            ),
        }
    }
}
