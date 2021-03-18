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

use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::sync::{Arc, RwLock};

use chrono::{Duration, Local, NaiveDateTime};

use comm::errors::entrances::{err_str, err_string, GeorgeResult};
use comm::io::file::{Filer, FilerExecutor, FilerHandler};
use comm::strings::{StringHandler, Strings};
use comm::trans::{trans_bytes_2_u32, trans_u32_2_bytes};
use comm::vectors::{Vector, VectorHandler};

use crate::task::engine::block::node::Node as NodeBlock;
use crate::task::engine::dossier::node::Node as NodeDossier;
use crate::task::engine::library::node::Node as NodeLibrary;
use crate::task::engine::memory::node::Node as NodeMemory;
use crate::task::engine::traits::{TIndex, TNode, TSeed};
use crate::task::rich::{Constraint, Expectation};
use crate::utils::comm::hash_key;
use crate::utils::enums::{Enum, EnumHandler, IndexType, KeyType};
use crate::utils::path::index_filepath;
use crate::utils::store::{before_content_bytes, Metadata, HD};
use crate::utils::writer::obtain_write_append_file;

/// Siam索引
///
/// 5位key及16位md5后key及5位起始seek和4位持续seek
#[derive(Debug)]
pub(crate) struct Index {
    /// 数据库名称
    database_name: String,
    /// 视图名称
    view_name: String,
    /// 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入
    name: String,
    /// 是否主键，主键也是唯一索引，即默认列表依赖索引
    primary: bool,
    /// 是否唯一索引
    unique: bool,
    /// 索引值类型
    key_type: KeyType,
    /// 结点
    root: Arc<RwLock<dyn TNode>>,
    /// 文件信息
    metadata: Metadata,
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
///
/// root 根结点
///
/// metadata 文件信息
fn new_index(
    database_name: String,
    view_name: String,
    name: String,
    primary: bool,
    unique: bool,
    key_type: KeyType,
    root: Arc<RwLock<dyn TNode>>,
    metadata: Metadata,
) -> GeorgeResult<Index> {
    let now: NaiveDateTime = Local::now().naive_local();
    let create_time = Duration::nanoseconds(now.timestamp_nanos());
    let filepath = index_filepath(database_name.clone(), view_name.clone(), name.clone());
    let file_append = obtain_write_append_file(filepath)?;
    let index = Index {
        database_name,
        primary,
        name,
        root,
        metadata,
        create_time,
        file_append,
        key_type,
        view_name,
        unique,
    };
    Ok(index)
}

impl Index {
    pub(crate) fn create(
        database_name: String,
        view_name: String,
        name: String,
        index_type: IndexType,
        primary: bool,
        unique: bool,
        key_type: KeyType,
    ) -> GeorgeResult<Arc<RwLock<dyn TIndex>>> {
        Filer::touch(index_filepath(
            database_name.clone(),
            view_name.clone(),
            name.clone(),
        ))?;
        let root: Arc<RwLock<dyn TNode>>;
        match index_type {
            IndexType::Dossier => {
                root =
                    NodeDossier::create_root(database_name.clone(), view_name.clone(), name.clone())
            }
            IndexType::Library => {
                root =
                    NodeLibrary::create_root(database_name.clone(), view_name.clone(), name.clone())
            }
            IndexType::Block => {
                root =
                    NodeBlock::create_root(database_name.clone(), view_name.clone(), name.clone())
            }
            IndexType::Memory => root = NodeMemory::create_root(),
            _ => return Err(err_str("unsupported engine type with none")),
        }
        let mut index = new_index(
            database_name.clone(),
            view_name.clone(),
            name,
            primary,
            unique,
            key_type,
            root,
            Metadata::index(index_type)?,
        )?;
        let mut metadata_bytes = index.metadata_bytes();
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
                        let filepath = index_filepath(database_name, view_name, self.name());
                        self.file_append = obtain_write_append_file(filepath)?;
                        let file_write_again = self.file_append.write().unwrap();
                        Filer::appends(file_write_again.try_clone().unwrap(), content)?;
                        Ok(seek_end_before)
                    }
                }
            }
            Err(_err) => {
                let filepath = index_filepath(database_name, view_name, self.name());
                self.file_append = obtain_write_append_file(filepath)?;
                let mut file_write_again = self.file_append.write().unwrap();
                let seek_end_before_again = file_write_again.seek(SeekFrom::End(0)).unwrap();
                Filer::appends(file_write_again.try_clone().unwrap(), content)?;
                Ok(seek_end_before_again)
            }
        }
    }
}

/// 封装方法函数w
impl TIndex for Index {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn key_type(&self) -> KeyType {
        self.key_type.clone()
    }
    fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }
    fn metadata_bytes(&self) -> Vec<u8> {
        self.metadata.bytes()
    }
    fn create_time(&self) -> Duration {
        self.create_time.clone()
    }
    fn modify(&mut self, database_name: String, view_name: String) {
        self.root
            .write()
            .unwrap()
            .modify(database_name.clone(), view_name.clone());
        self.database_name = database_name;
        self.view_name = view_name;
    }
    fn put(&self, key: String, seed: Arc<RwLock<dyn TSeed>>, force: bool) -> GeorgeResult<()> {
        let hash_key = hash_key(self.key_type(), key.clone())?;
        self.root.write().unwrap().put(hash_key, seed, force)
    }
    fn get(&self, key: String) -> GeorgeResult<Vec<u8>> {
        let hash_key = hash_key(self.key_type(), key.clone())?;
        self.root.read().unwrap().get(key, hash_key)
    }
    fn del(&self, key: String) -> GeorgeResult<()> {
        let hash_key = hash_key(self.key_type(), key.clone())?;
        self.root.write().unwrap().del(key, hash_key)
    }
    fn select(
        &self,
        left: bool,
        start: u64,
        end: u64,
        constraint: Constraint,
    ) -> GeorgeResult<Expectation> {
        unimplemented!()
    }
}

impl Index {
    /// 生成文件描述
    fn description(&self) -> Vec<u8> {
        let mut part1 = hex::encode(format!(
            "{}:#?{}:#?{}:#?{}:#?{}",
            self.name,
            self.primary,
            self.unique,
            Enum::key_type_u8(self.key_type),
            self.create_time().num_nanoseconds().unwrap().to_string(),
        ))
        .into_bytes();
        // 长度为524288的字节数组
        let mut part2 = self
            .root
            .read()
            .unwrap()
            .node_bytes()
            .read()
            .unwrap()
            .to_vec();
        let mut des_bytes = trans_u32_2_bytes(part2.len() as u32);
        des_bytes.append(&mut part1);
        des_bytes.append(&mut part2);
        des_bytes
    }
    /// 通过文件描述恢复结构信息
    pub(crate) fn recover(
        database_name: String,
        view_name: String,
        hd: HD,
    ) -> GeorgeResult<Arc<RwLock<dyn TIndex>>> {
        let des_bytes = hd.description();
        let des_bytes_len = des_bytes.len();
        let part2_len_bytes = vec![des_bytes[0], des_bytes[1], des_bytes[2], des_bytes[3]];
        let part2_bytes_len = trans_bytes_2_u32(part2_len_bytes)? as usize;
        let middle_pos = des_bytes_len - part2_bytes_len;
        let part1 = Vector::sub(des_bytes.clone(), 4, middle_pos)?;
        let part2 = Vector::sub(des_bytes, middle_pos, des_bytes_len)?;
        let description_str = Strings::from_utf8(part1)?;
        match hex::decode(description_str) {
            Ok(vu8) => {
                let real = Strings::from_utf8(vu8)?;
                let mut split = real.split(":#?");
                let name = split.next().unwrap().to_string();
                let primary = split.next().unwrap().to_string().parse::<bool>().unwrap();
                let unique = split.next().unwrap().to_string().parse::<bool>().unwrap();
                let key_type =
                    Enum::key_type(split.next().unwrap().to_string().parse::<u8>().unwrap());
                let create_time = Duration::nanoseconds(
                    split.next().unwrap().to_string().parse::<i64>().unwrap(),
                );
                let filepath =
                    index_filepath(database_name.clone(), view_name.clone(), name.clone());
                let file_append = obtain_write_append_file(filepath)?;
                let root: Arc<RwLock<dyn TNode>>;
                match hd.index_type() {
                    IndexType::Dossier => {
                        root = NodeDossier::recovery_root(
                            database_name.clone(),
                            view_name.clone(),
                            name.clone(),
                        )?
                    }
                    IndexType::Library => {
                        root = NodeLibrary::recovery_root(
                            database_name.clone(),
                            view_name.clone(),
                            name.clone(),
                            part2,
                        )
                    }
                    IndexType::Block => {
                        root = NodeBlock::recovery_root(
                            database_name.clone(),
                            view_name.clone(),
                            name.clone(),
                            part2,
                        )
                    }
                    IndexType::Memory => root = NodeMemory::recovery_root(),
                    _ => return Err(err_str("unsupported engine type")),
                }
                log::info!(
                    "recovery index {} from database.view {}.{}",
                    name.clone(),
                    database_name.clone(),
                    view_name.clone()
                );
                let index = Index {
                    database_name,
                    view_name,
                    name,
                    primary,
                    unique,
                    create_time,
                    metadata: hd.metadata(),
                    file_append,
                    root,
                    key_type,
                };
                Ok(Arc::new(RwLock::new(index)))
            }
            Err(err) => Err(err_string(format!(
                "recovery index decode failed! error is {}",
                err
            ))),
        }
    }
}