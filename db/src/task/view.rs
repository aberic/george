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

use crate::task::engine::traits::TIndex;
use crate::utils::comm::{Capacity, EngineType, IndexMold, IndexType, INDEX_CATALOG};
use crate::utils::path::{view_file_path, view_path};
use crate::utils::store;
use crate::utils::store::{
    before_content_bytes, capacity_u8, engine_type_u8, index_type_u8, metadata_2_bytes, Metadata,
    Tag, HD,
};
use crate::utils::writer::obtain_write_append_file;
use chrono::{Duration, Local, NaiveDateTime};
use comm::errors::entrances::{err_str_enhance, err_string, GeorgeResult};
use comm::io::file::create_file;
use comm::io::reader::read_sub_bytes;
use comm::io::writer::{write_file_append_bytes, write_seek_u8s};
use std::collections::HashMap;
use std::fs::{read_dir, File, ReadDir};
use std::io::{Seek, SeekFrom, Write};
use std::sync::{Arc, RwLock};

/// 视图，类似表
#[derive(Debug, Clone)]
pub(super) struct View {
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
    pub(super) fn create(database_name: String, name: String) -> GeorgeResult<Arc<RwLock<View>>> {
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
    pub(super) fn file_append(
        &mut self,
        database_name: String,
        content: Vec<u8>,
    ) -> GeorgeResult<u64> {
        let file_append = self.file_append.clone();
        let mut file_write = file_append.write().unwrap();
        match file_write.seek(SeekFrom::End(0)) {
            Ok(seek_end_before) => {
                match write_file_append_bytes(file_write.try_clone().unwrap(), content.clone()) {
                    Ok(()) => Ok(seek_end_before),
                    Err(_err) => {
                        let file_path = view_file_path(database_name, self.name());
                        self.file_append = obtain_write_append_file(file_path)?;
                        let file_again = self.file_append.write().unwrap();
                        write_file_append_bytes(file_again.try_clone().unwrap(), content)?;
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
    pub(super) fn indexes(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<dyn TIndex>>>>> {
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
    pub(super) fn create_index(
        &self,
        _database_name: String,
        _index_name: String,
        _index_mold: IndexMold,
        _primary: bool,
    ) -> GeorgeResult<()> {
        Ok(())
    }
}

impl View {
    fn description(&self) -> Vec<u8> {
        hex::encode(format!(
            "{}/{}",
            self.name(),
            self.create_time().num_nanoseconds().unwrap().to_string()
        ))
        .into_bytes()
    }

    pub(super) fn recover(database_name: String, hd: HD) -> GeorgeResult<View> {
        match String::from_utf8(hd.description) {
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
                            metadata: hd.metadata,
                            file_append,
                            indexes: Arc::new(Default::default()),
                        };
                        log::info!(
                            "recovery view {} from database {}",
                            view.name(),
                            database_name,
                        );
                        match read_dir(view_path(database_name, view.name())) {
                            // 恢复indexes数据
                            Ok(paths) => view.recovery_indexes(paths),
                            Err(err) => panic!("recovery view read dir failed! error is {}", err),
                        }
                        Ok(view)
                    }
                    Err(err) => Err(err_string(format!(
                        "recovery view from utf8 failed! error is {}",
                        err
                    ))),
                },
                Err(err) => Err(err_string(format!(
                    "recovery view decode failed! error is {}",
                    err
                ))),
            },
            Err(err) => Err(err_string(err.to_string())),
        }
    }
}

impl View {
    /// 恢复indexes数据
    pub(super) fn recovery_indexes(&mut self, paths: ReadDir) {}
}
