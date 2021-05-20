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

use std::sync::{Arc, RwLock};

use chrono::{Duration, Local, NaiveDateTime};

use comm::errors::entrances::{Errs, GeorgeResult};
use comm::strings::{StringHandler, Strings};

use crate::task::engine::block::node::Node as NB;
use crate::task::engine::dossier::node::Node as ND;
use crate::task::engine::sequence::node::Node as NS;
use crate::task::engine::traits::{TIndex, TNode, TSeed};
use crate::task::rich::{Constraint, Expectation};
use crate::task::view::View;
use crate::utils::enums::{Enum, EnumHandler, IndexType, KeyType};
use crate::utils::path::Paths;
use crate::utils::store::{ContentBytes, Metadata, HD};
use crate::utils::writer::Filed;
use serde_json::Value;

/// Siam索引
///
/// 5位key及16位md5后key及5位起始seek和4位持续seek
#[derive(Debug)]
pub(crate) struct Index {
    view: Arc<RwLock<View>>,
    /// 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入
    name: String,
    /// 是否主键，主键也是唯一索引，即默认列表依赖索引
    primary: bool,
    /// 是否唯一索引
    unique: bool,
    /// 是否允许为空
    null: bool,
    /// 索引值类型
    key_type: KeyType,
    /// 结点
    root: Arc<dyn TNode>,
    /// 文件信息
    metadata: Metadata,
    /// 创建时间
    create_time: Duration,
    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 需要借助对象包裹，以便更新file，避免self为mut
    filer: Filed,
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
    view: Arc<RwLock<View>>,
    name: String,
    primary: bool,
    unique: bool,
    null: bool,
    key_type: KeyType,
    root: Arc<dyn TNode>,
    metadata: Metadata,
) -> GeorgeResult<Index> {
    let now: NaiveDateTime = Local::now().naive_local();
    let create_time = Duration::nanoseconds(now.timestamp_nanos());
    let v_c = view.clone();
    let v_r = v_c.read().unwrap();
    let filepath = Paths::index_filepath(v_r.database_name(), v_r.name(), name.clone());
    let index = Index {
        view,
        primary,
        name,
        root,
        metadata,
        create_time,
        key_type,
        unique,
        null,
        filer: Filed::create(filepath)?,
    };
    Ok(index)
}

impl Index {
    pub(crate) fn create(
        view: Arc<RwLock<View>>,
        name: String,
        index_type: IndexType,
        primary: bool,
        unique: bool,
        null: bool,
        key_type: KeyType,
    ) -> GeorgeResult<Arc<dyn TIndex>> {
        let root: Arc<dyn TNode>;
        match index_type {
            IndexType::Sequence => root = NS::create(view.clone(), name.clone(), key_type)?,
            IndexType::Disk => root = ND::create(view.clone(), name.clone(), key_type, unique)?,
            IndexType::Block => root = NB::create(name.clone(), key_type),
            _ => return Err(Errs::str("unsupported engine type with none")),
        }
        let index = new_index(
            view,
            name,
            primary,
            unique,
            null,
            key_type,
            root,
            Metadata::index(index_type)?,
        )?;
        let mut metadata_bytes = index.metadata_bytes();
        let mut description = index.description();
        // 初始化为32 + 8，即head长度加正文描述符长度
        let mut before_description = ContentBytes::before(44, description.len() as u32);
        metadata_bytes.append(&mut before_description);
        metadata_bytes.append(&mut description);
        index.append(metadata_bytes)?;
        Ok(Arc::new(index))
    }
    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
    ///
    /// #Return
    ///
    /// seek_end_before 写之前文件字节数据长度
    fn append(&self, content: Vec<u8>) -> GeorgeResult<u64> {
        self.filer.append(content)
    }
}

/// 封装方法函数w
impl TIndex for Index {
    fn view(&self) -> Arc<RwLock<View>> {
        self.view.clone()
    }
    fn database_name(&self) -> String {
        self.view().read().unwrap().database_name()
    }
    fn view_name(&self) -> String {
        self.view().read().unwrap().name()
    }
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
    fn put(&self, key: String, seed: Arc<RwLock<dyn TSeed>>, force: bool) -> GeorgeResult<()> {
        self.root.put(key, seed, force)
    }
    fn get(&self, key: String) -> GeorgeResult<Vec<u8>> {
        self.root.get(key)
    }
    fn del(&self, key: String, seed: Arc<RwLock<dyn TSeed>>) -> GeorgeResult<()> {
        self.root.del(key, seed)
    }
    fn select(
        &self,
        left: bool,
        start: u64,
        end: u64,
        constraint: Constraint,
    ) -> GeorgeResult<Expectation> {
        log::debug!(
            "index status with left = {} & start = {} & end = {} & constraint = {:#?}",
            left,
            start,
            end,
            constraint
        );
        let conditions = constraint.conditions();
        let skip = constraint.skip();
        let limit = constraint.limit();
        let delete = constraint.delete();
        let (total, count, mut values) = self
            .root
            .select(left, start, end, skip, limit, delete, conditions)?;
        match constraint.sort() {
            Some(sort) => {
                values.sort_by(|a, b| {
                    let value_a: Value;
                    let value_b: Value;
                    match String::from_utf8(a.clone()) {
                        Ok(value_str) => match serde_json::from_str(value_str.as_ref()) {
                            Ok(value) => value_a = value,
                            Err(err) => panic!("an unexpected mistake a json from, {}", err),
                        },
                        Err(err) => panic!("an unexpected mistake a string from, {}", err),
                    }
                    match String::from_utf8(b.clone()) {
                        Ok(value_str) => match serde_json::from_str(value_str.as_ref()) {
                            Ok(value) => value_b = value,
                            Err(err) => panic!("an unexpected mistake b json from, {}", err),
                        },
                        Err(err) => panic!("an unexpected mistake b string from, {}", err),
                    }
                    if value_a[sort.param()].is_i64() && value_b[sort.param()].is_i64() {
                        let opa = value_a[sort.param()].as_i64();
                        let opb = value_b[sort.param()].as_i64();
                        if sort.asc() {
                            if opa.gt(&opb) {
                                a.cmp(b)
                            } else {
                                b.cmp(a)
                            }
                        } else {
                            if opa.lt(&opb) {
                                a.cmp(b)
                            } else {
                                b.cmp(a)
                            }
                        }
                    } else if value_a[sort.param()].is_u64() && value_b[sort.param()].is_u64() {
                        let opa = value_a[sort.param()].as_u64();
                        let opb = value_b[sort.param()].as_u64();
                        if sort.asc() {
                            if opa.gt(&opb) {
                                a.cmp(b)
                            } else {
                                b.cmp(a)
                            }
                        } else {
                            if opa.lt(&opb) {
                                a.cmp(b)
                            } else {
                                b.cmp(a)
                            }
                        }
                    } else if value_a[sort.param()].is_f64() && value_b[sort.param()].is_f64() {
                        let opa = value_a[sort.param()].as_f64();
                        let opb = value_b[sort.param()].as_f64();
                        if sort.asc() {
                            if opa.gt(&opb) {
                                a.cmp(b)
                            } else {
                                b.cmp(a)
                            }
                        } else {
                            if opa.lt(&opb) {
                                a.cmp(b)
                            } else {
                                b.cmp(a)
                            }
                        }
                    } else if value_a[sort.param()].is_string() && value_b[sort.param()].is_string()
                    {
                        let opa = value_a[sort.param()].as_str();
                        let opb = value_b[sort.param()].as_str();
                        if sort.asc() {
                            if opa.gt(&opb) {
                                a.cmp(b)
                            } else {
                                b.cmp(a)
                            }
                        } else {
                            if opa.lt(&opb) {
                                a.cmp(b)
                            } else {
                                b.cmp(a)
                            }
                        }
                    } else {
                        panic!("{} can't match each other when sort", sort.param())
                    }
                });
            }
            _ => {}
        }
        Ok(Expectation {
            total,
            count,
            index_name: self.name(),
            asc: left,
            values,
        })
    }
}

impl Index {
    /// 生成文件描述
    fn description(&self) -> Vec<u8> {
        hex::encode(format!(
            "{}:#?{}:#?{}:#?{}:#?{}:#?{}",
            self.name,
            self.primary,
            self.unique,
            self.null,
            Enum::key_type_u8(self.key_type),
            self.create_time().num_nanoseconds().unwrap().to_string(),
        ))
        .into_bytes()
    }
    /// 通过文件描述恢复结构信息
    pub(crate) fn recover(view: Arc<RwLock<View>>, hd: HD) -> GeorgeResult<Arc<dyn TIndex>> {
        let des_bytes = hd.description();
        let description_str = Strings::from_utf8(des_bytes)?;
        match hex::decode(description_str) {
            Ok(vu8) => {
                let real = Strings::from_utf8(vu8)?;
                let mut split = real.split(":#?");
                let name = split.next().unwrap().to_string();
                let primary = split.next().unwrap().to_string().parse::<bool>().unwrap();
                let unique = split.next().unwrap().to_string().parse::<bool>().unwrap();
                let null = split.next().unwrap().to_string().parse::<bool>().unwrap();
                let key_type =
                    Enum::key_type(split.next().unwrap().to_string().parse::<u8>().unwrap());
                let create_time = Duration::nanoseconds(
                    split.next().unwrap().to_string().parse::<i64>().unwrap(),
                );
                let v_c = view.clone();
                let v_r = v_c.read().unwrap();
                let filepath = Paths::index_filepath(v_r.database_name(), v_r.name(), name.clone());
                let root: Arc<dyn TNode>;
                match hd.index_type() {
                    IndexType::Sequence => {
                        root = NS::recovery(view.clone(), name.clone(), key_type)?
                    }
                    IndexType::Disk => {
                        root = ND::recovery(view.clone(), name.clone(), key_type, unique)?
                    }
                    IndexType::Block => root = NB::recovery(name.clone(), key_type),
                    _ => return Err(Errs::str("unsupported engine type")),
                }
                log::info!(
                    "recovery index {} from database.view {}.{}",
                    name.clone(),
                    v_r.database_name(),
                    v_r.name()
                );
                let index = Index {
                    view,
                    name,
                    primary,
                    unique,
                    create_time,
                    metadata: hd.metadata(),
                    root,
                    key_type,
                    null,
                    filer: Filed::recovery(filepath)?,
                };
                Ok(Arc::new(index))
            }
            Err(err) => Err(Errs::string(format!(
                "recovery index decode failed! error is {}",
                err
            ))),
        }
    }
}
