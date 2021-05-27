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
use crate::task::engine::disk::node::Node as ND;
use crate::task::engine::increment::node::Node as NI;
use crate::task::engine::sequence::node::Node as NS;
use crate::task::engine::traits::{TIndex, TNode, TSeed};
use crate::task::engine::DataReal;
use crate::task::rich::{Constraint, Expectation};
use crate::task::view::View;
use crate::utils::enums::{Enum, EnumHandler, IndexType, KeyType};
use crate::utils::path::Paths;
use crate::utils::store::{ContentBytes, Metadata, HD};
use crate::utils::writer::Filed;
use comm::json::{Json, JsonExec, JsonGet, JsonNew};

/// Siam索引
///
/// 5位key及16位md5后key及5位起始seek和4位持续seek
#[derive(Debug)]
pub(crate) struct Index {
    view: Arc<RwLock<View>>,
    /// 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入
    name: String,
    /// 存储引擎类型
    index_type: IndexType,
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
/// * view 视图
/// * name 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入
/// * index_type 存储引擎类型
/// * primary 是否主键，主键也是唯一索引，即默认列表依赖索引
/// * unique 是否唯一索引
/// * null 是否允许为空
/// * key_type 索引值类型
/// * root 根结点
/// * metadata 索引文件信息
fn new_index(
    view: Arc<RwLock<View>>,
    name: String,
    index_type: IndexType,
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
        index_type,
    };
    Ok(index)
}

impl Index {
    /// 创建索引
    ///
    /// ###Params
    /// * view 视图
    /// * name 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入
    /// * index_type 存储引擎类型
    /// * primary 是否主键，主键也是唯一索引，即默认列表依赖索引
    /// * unique 是否唯一索引
    /// * null 是否允许为空
    /// * key_type 索引值类型
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
            IndexType::Increment => root = NI::create(view.clone(), name.clone())?,
            IndexType::Sequence => root = NS::create(view.clone(), name.clone())?,
            IndexType::Disk => root = ND::create(view.clone(), name.clone(), key_type, unique)?,
            IndexType::Block => root = NB::create(name.clone(), key_type),
            _ => return Err(Errs::str("unsupported engine type with none")),
        }
        let index = new_index(
            view,
            name,
            index_type.clone(),
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

    fn index_type(&self) -> IndexType {
        self.index_type.clone()
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

    fn get(&self, key: String) -> GeorgeResult<DataReal> {
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
            "index {} status with left = {} & start = {} & end = {} & skip = {} & limit = {} & delete = {} & conditions = {:#?}",
            self.name(),
            left,
            start,
            end,
            constraint.skip(),
            constraint.limit(),
            constraint.delete(),
            constraint.conditions()
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
                let sort_param = sort.param();
                let param = sort_param.as_str();
                values.sort_by(|a, b| {
                    let json_a = Json::new(a.clone()).unwrap();
                    let json_b = Json::new(b.clone()).unwrap();
                    match sort.index() {
                        Some(index) => match index.key_type() {
                            KeyType::Int => {
                                let opa = json_a.get_i64(param).unwrap();
                                let opb = json_b.get_i64(param).unwrap();
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
                            }
                            KeyType::UInt => {
                                let opa = json_a.get_u64(param).unwrap();
                                let opb = json_b.get_u64(param).unwrap();
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
                            }
                            KeyType::Float => {
                                let opa = json_a.get_f64(param).unwrap();
                                let opb = json_b.get_f64(param).unwrap();
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
                            }
                            KeyType::String => {
                                let opa = json_a.get_string(param).unwrap();
                                let opb = json_b.get_string(param).unwrap();
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
                            }
                            KeyType::Bool => {
                                let opa = json_a.get_bool(param).unwrap();
                                let opb = json_b.get_bool(param).unwrap();
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
                            }
                            _ => panic!("{} can not match each other when sort", param),
                        },
                        None => {
                            if json_a.is_i64(param) && json_b.is_i64(param) {
                                let opa = json_a.get_i64(param).unwrap();
                                let opb = json_b.get_i64(param).unwrap();
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
                            } else if json_a.is_u64(param) && json_b.is_u64(param) {
                                let opa = json_a.get_i64(param).unwrap();
                                let opb = json_b.get_i64(param).unwrap();
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
                            } else if json_a.is_f64(param) && json_b.is_f64(param) {
                                let opa = json_a.get_i64(param).unwrap();
                                let opb = json_b.get_i64(param).unwrap();
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
                            } else if json_a.is_string(param) && json_b.is_string(param) {
                                let opa = json_a.get_i64(param).unwrap();
                                let opb = json_b.get_i64(param).unwrap();
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
                            } else if json_a.is_bool(param) && json_b.is_bool(param) {
                                let opa = json_a.get_i64(param).unwrap();
                                let opb = json_b.get_i64(param).unwrap();
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
                                panic!("{} can not match each other when sort", param)
                            }
                        }
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
            "{}:#?{}:#?{}:#?{}:#?{}:#?{}:#?{}",
            self.name,
            Enum::index_type_u8(self.index_type()),
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
                let index_type =
                    Enum::index_type(split.next().unwrap().to_string().parse::<u8>().unwrap());
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
                    IndexType::Increment => root = NI::recovery(view.clone(), name.clone())?,
                    IndexType::Sequence => root = NS::recovery(view.clone(), name.clone())?,
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
                    index_type,
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
