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

use std::fmt::Debug;
use std::sync::{Arc, RwLock};

use chrono::{Duration, Local, NaiveDateTime};

use comm::errors::entrances::err_string;
use comm::errors::entrances::GeorgeResult;
use comm::io::file::{Filer, FilerExecutor, FilerHandler};

use crate::task::engine::memory::node::Node;
use crate::task::engine::traits::{TIndex, TSeed};
use crate::utils::enums::{EngineType, IndexMold};
use crate::utils::path::index_file_path;
use crate::utils::store::{before_content_bytes, metadata_2_bytes, Metadata, HD};
use crate::utils::writer::obtain_write_append_file;
use comm::strings::{StringHandler, Strings};
use std::fs::File;
use std::io::{Seek, SeekFrom};

/// Siam索引
///
/// 5位key及16位md5后key及5位起始seek和4位持续seek
#[derive(Debug)]
pub struct Index {
    /// 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入
    name: String,
    /// 文件信息
    metadata: Metadata,
    /// 结点
    root: Arc<Node>,
    /// 创建时间
    create_time: Duration,
    /// 根据文件路径获取该文件追加写入的写对象
    file_append: Arc<RwLock<File>>,
}

/// 新建索引
///
/// 该索引需要定义ID，此外索引所表达的字段组成内容也是必须的，并通过primary判断索引类型，具体传参参考如下定义：<p><p>
///
/// ###Params
///
/// index_name 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入
///
/// primary 是否主键
fn new_index(
    database_name: String,
    view_name: String,
    name: String,
    metadata: Metadata,
) -> GeorgeResult<Index> {
    let now: NaiveDateTime = Local::now().naive_local();
    let create_time = Duration::nanoseconds(now.timestamp_nanos());
    let file_path = index_file_path(database_name, view_name, name.clone());
    let file_append = obtain_write_append_file(file_path)?;
    return Ok(Index {
        name,
        root: Node::create_root(),
        metadata,
        create_time,
        file_append,
    });
}

/// 封装方法函数
impl Index {
    /// 新建索引
    ///
    /// 该索引需要定义ID，此外索引所表达的字段组成内容也是必须的，并通过primary判断索引类型，具体传参参考如下定义：<p><p>
    ///
    /// ###Params
    ///
    /// index_name 索引名称，可以自定义；<p>
    /// siam::Index 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入<p><p>
    ///
    /// primary 是否主键
    ///
    /// level 视图规模/级别
    pub(crate) fn create(
        database_name: String,
        view_name: String,
        name: String,
    ) -> GeorgeResult<Arc<RwLock<dyn TIndex>>> {
        Filer::touch(index_file_path(
            database_name.clone(),
            view_name.clone(),
            name.clone(),
        ))?;
        let mut index = new_index(
            database_name.clone(),
            view_name.clone(),
            name,
            Metadata::index(EngineType::Memory)?,
        )?;
        let mut metadata_bytes = metadata_2_bytes(index.metadata());
        let mut description = index.description();
        // 初始化为32 + 8，即head长度加正文描述符长度
        let mut before_description = before_content_bytes(40, description.len() as u32);
        metadata_bytes.append(&mut before_description);
        metadata_bytes.append(&mut description);
        index.file_append(database_name, view_name, metadata_bytes)?;
        Ok(Arc::new(RwLock::new(index)))
    }
    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
    ///
    /// #Return
    ///
    /// seek_end_before 写之前文件字节数据长度
    fn file_append(
        &mut self,
        database_name: String,
        view_name: String,
        content: Vec<u8>,
    ) -> GeorgeResult<u64> {
        let file_append = self.file_append.clone();
        let mut file_write = file_append.write().unwrap();
        match file_write.seek(SeekFrom::End(0)) {
            Ok(seek_end_before) => {
                match Filer::appends(file_write.try_clone().unwrap(), content.clone()) {
                    Ok(()) => Ok(seek_end_before),
                    Err(_err) => {
                        let file_path = index_file_path(database_name, view_name, self.name());
                        self.file_append = obtain_write_append_file(file_path)?;
                        let file_write_again = self.file_append.write().unwrap();
                        Filer::appends(file_write_again.try_clone().unwrap(), content)?;
                        Ok(seek_end_before)
                    }
                }
            }
            Err(_err) => {
                let file_path = index_file_path(database_name, view_name, self.name());
                self.file_append = obtain_write_append_file(file_path)?;
                let mut file_write_again = self.file_append.write().unwrap();
                let seek_end_before_again = file_write_again.seek(SeekFrom::End(0)).unwrap();
                Filer::appends(file_write_again.try_clone().unwrap(), content)?;
                Ok(seek_end_before_again)
            }
        }
    }
}

/// 封装方法函数
impl TIndex for Index {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn mold(&self) -> IndexMold {
        IndexMold::String
    }
    fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }
    fn create_time(&self) -> Duration {
        self.create_time.clone()
    }
    fn put(
        &self,
        _database_name: String,
        _view_name: String,
        key: String,
        seed: Arc<RwLock<dyn TSeed>>,
    ) -> GeorgeResult<()> {
        self.root.put(key, seed)
    }
    fn get(
        &self,
        _database_name: String,
        _view_name: String,
        key: String,
    ) -> GeorgeResult<Vec<u8>> {
        self.root.get(key)
    }
}

impl Index {
    fn description(&mut self) -> Vec<u8> {
        hex::encode(format!(
            "{}:#?{}",
            self.name,
            self.create_time().num_nanoseconds().unwrap().to_string(),
        ))
        .into_bytes()
    }
    /// 通过文件描述恢复结构信息
    pub(crate) fn recover(
        database_name: String,
        view_name: String,
        hd: HD,
    ) -> GeorgeResult<Arc<RwLock<dyn TIndex>>> {
        let description_str = Strings::from_utf8(hd.description())?;
        match hex::decode(description_str) {
            Ok(vu8) => {
                let real = Strings::from_utf8(vu8)?;
                let mut split = real.split(":#?");
                let name = split.next().unwrap().to_string();
                let create_time = Duration::nanoseconds(
                    split.next().unwrap().to_string().parse::<i64>().unwrap(),
                );
                let file_path =
                    index_file_path(database_name.clone(), view_name.clone(), name.clone());
                let file_append = obtain_write_append_file(file_path)?;
                let index = Index {
                    name,
                    create_time,
                    metadata: hd.metadata(),
                    file_append,
                    root: Node::create_root(),
                };
                log::info!(
                    "recovery index {} from database.view {}.{}",
                    index.name(),
                    database_name,
                    view_name
                );
                Ok(Arc::new(RwLock::new(index)))
            }
            Err(err) => Err(err_string(format!(
                "recovery index decode failed! error is {}",
                err
            ))),
        }
    }
}
