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

use chrono::Duration;

use george_comm::errors::{Errs, GeorgeResult};
use george_comm::json::{JsonExec, JsonGet, JsonNew};
use george_comm::strings::StringHandler;
use george_comm::Strings;
use george_comm::{Json, Time};
use george_ge::utils::enums::Tag;
use george_ge::GeFactory;

use crate::task::engine::DataReal;
// use crate::task::engine::block::Node as NB;
use crate::task::engine::disk::Node as ND;
use crate::task::engine::increment::Node as NI;
use crate::task::engine::sequence::Node as NS;
use crate::task::engine::traits::{TIndex, TNode, TSeed};
use crate::task::rich::{Constraint, Expectation};
use crate::task::traits::TForm;
use crate::task::Index;
use crate::utils::enums::{Engine, KeyType};
use crate::utils::Paths;
use crate::utils::{Enum, EnumHandler};

impl Index {
    /// 创建索引
    ///
    /// ###Params
    /// * view 视图
    /// * name 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入
    /// * engine 存储引擎类型
    /// * primary 是否主键，主键也是唯一索引，即默认列表依赖索引
    /// * unique 是否唯一索引
    /// * null 是否允许为空
    /// * key_type 索引值类型
    pub(crate) fn create(
        form: Arc<RwLock<dyn TForm>>,
        name: String,
        engine: Engine,
        primary: bool,
        unique: bool,
        null: bool,
        key_type: KeyType,
    ) -> GeorgeResult<Arc<dyn TIndex>> {
        let root: Arc<dyn TNode>;
        match engine {
            Engine::Increment => root = NI::create(form.clone(), name.clone())?,
            Engine::Sequence => root = NS::create(form.clone(), name.clone())?,
            Engine::Disk => root = ND::create(form.clone(), name.clone(), key_type, unique)?,
            // IndexType::Block => root = NB::create(name.clone(), key_type),
            _ => return Err(Errs::str("unsupported engine type with none")),
        }
        let create_time = Time::now();
        let v_c = form.clone();
        let v_r = v_c.read().unwrap();
        let filepath = Paths::index_filepath(v_r.database_name(), v_r.name(), name.clone());
        let description = Some(Index::descriptions(
            name.clone(),
            engine,
            primary,
            unique,
            null,
            key_type,
            create_time,
        ));
        Ok(Arc::new(Index {
            form,
            primary,
            name,
            root,
            create_time,
            key_type,
            unique,
            null,
            ge: GeFactory {}.create(Tag::Index, filepath, description)?,
            engine,
        }))
    }
}

/// 封装方法函数w
impl TIndex for Index {
    fn form(&self) -> Arc<RwLock<dyn TForm>> {
        self.form.clone()
    }

    fn database_name(&self) -> String {
        self.form().read().unwrap().database_name()
    }

    fn view_name(&self) -> String {
        self.form().read().unwrap().name()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn engine(&self) -> Engine {
        self.engine.clone()
    }

    fn primary(&self) -> bool {
        self.primary
    }

    fn unique(&self) -> bool {
        self.unique
    }

    fn null(&self) -> bool {
        self.null
    }

    fn key_type(&self) -> KeyType {
        self.key_type.clone()
    }

    fn create_time(&self) -> Time {
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
                                    opa.cmp(&opb)
                                } else {
                                    opb.cmp(&opa)
                                }
                            }
                            KeyType::UInt => {
                                let opa = json_a.get_u64(param).unwrap();
                                let opb = json_b.get_u64(param).unwrap();
                                if sort.asc() {
                                    opa.cmp(&opb)
                                } else {
                                    opb.cmp(&opa)
                                }
                            }
                            KeyType::Float => {
                                let opa = json_a.get_f64(param).unwrap().to_bits();
                                let opb = json_b.get_f64(param).unwrap().to_bits();
                                if sort.asc() {
                                    opa.cmp(&opb)
                                } else {
                                    opb.cmp(&opa)
                                }
                            }
                            KeyType::String => {
                                let opa = json_a.get_string(param).unwrap();
                                let opb = json_b.get_string(param).unwrap();
                                if sort.asc() {
                                    opa.cmp(&opb)
                                } else {
                                    opb.cmp(&opa)
                                }
                            }
                            KeyType::Bool => {
                                let opa = json_a.get_bool(param).unwrap();
                                let opb = json_b.get_bool(param).unwrap();
                                if sort.asc() {
                                    opa.cmp(&opb)
                                } else {
                                    opb.cmp(&opa)
                                }
                            }
                            _ => panic!("{} can not match each other when sort", param),
                        },
                        None => {
                            if json_a.is_i64(param) && json_b.is_i64(param) {
                                let opa = json_a.get_i64(param).unwrap();
                                let opb = json_b.get_i64(param).unwrap();
                                if sort.asc() {
                                    opa.cmp(&opb)
                                } else {
                                    opb.cmp(&opa)
                                }
                            } else if json_a.is_u64(param) && json_b.is_u64(param) {
                                let opa = json_a.get_i64(param).unwrap();
                                let opb = json_b.get_i64(param).unwrap();
                                if sort.asc() {
                                    opa.cmp(&opb)
                                } else {
                                    opb.cmp(&opa)
                                }
                            } else if json_a.is_f64(param) && json_b.is_f64(param) {
                                let opa = json_a.get_f64(param).unwrap().to_bits();
                                let opb = json_b.get_f64(param).unwrap().to_bits();
                                if sort.asc() {
                                    opa.cmp(&opb)
                                } else {
                                    opb.cmp(&opa)
                                }
                            } else if json_a.is_string(param) && json_b.is_string(param) {
                                let opa = json_a.get_i64(param).unwrap();
                                let opb = json_b.get_i64(param).unwrap();
                                if sort.asc() {
                                    opa.cmp(&opb)
                                } else {
                                    opb.cmp(&opa)
                                }
                            } else if json_a.is_bool(param) && json_b.is_bool(param) {
                                let opa = json_a.get_i64(param).unwrap();
                                let opb = json_b.get_i64(param).unwrap();
                                if sort.asc() {
                                    opa.cmp(&opb)
                                } else {
                                    opb.cmp(&opa)
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
    fn descriptions(
        name: String,
        engine: Engine,
        primary: bool,
        unique: bool,
        null: bool,
        key_type: KeyType,
        create_time: Time,
    ) -> Vec<u8> {
        hex::encode(format!(
            "{}:#?{}:#?{}:#?{}:#?{}:#?{}:#?{}",
            name,
            Enum::engine_u8(engine),
            primary,
            unique,
            null,
            Enum::key_type_u8(key_type),
            create_time.nano_string().unwrap(),
        ))
        .into_bytes()
    }

    /// 通过文件描述恢复结构信息
    pub(crate) fn recover(
        form: Arc<RwLock<dyn TForm>>,
        name: String,
    ) -> GeorgeResult<Arc<dyn TIndex>> {
        let v_c = form.clone();
        let v_r = v_c.read().unwrap();
        let filepath = Paths::index_filepath(v_r.database_name(), v_r.name(), name.clone());
        let ge = GeFactory {}.recovery(Tag::Index, filepath)?;
        let description_str = Strings::from_utf8(ge.description_content_bytes()?)?;
        match hex::decode(description_str) {
            Ok(vu8) => {
                let real = Strings::from_utf8(vu8)?;
                let mut split = real.split(":#?");
                let name = split.next().unwrap().to_string();
                let engine = Enum::engine(split.next().unwrap().to_string().parse::<u8>().unwrap());
                let primary = split.next().unwrap().to_string().parse::<bool>().unwrap();
                let unique = split.next().unwrap().to_string().parse::<bool>().unwrap();
                let null = split.next().unwrap().to_string().parse::<bool>().unwrap();
                let key_type =
                    Enum::key_type(split.next().unwrap().to_string().parse::<u8>().unwrap());
                let duration = Duration::nanoseconds(
                    split.next().unwrap().to_string().parse::<i64>().unwrap(),
                );
                let root: Arc<dyn TNode>;
                match engine {
                    Engine::Increment => root = NI::recovery(form.clone(), name.clone())?,
                    Engine::Sequence => root = NS::recovery(form.clone(), name.clone())?,
                    Engine::Disk => {
                        root = ND::recovery(form.clone(), name.clone(), key_type, unique)?
                    }
                    // IndexType::Block => root = NB::recovery(name.clone(), key_type),
                    _ => return Err(Errs::str("unsupported engine type")),
                }
                let create_time = Time::from(duration);
                log::info!(
                    "recovery index {} from database.view {}.{} created at {}",
                    name.clone(),
                    v_r.database_name(),
                    v_r.name(),
                    create_time.format("%Y-%m-%d %H:%M:%S")
                );
                let index = Index {
                    form,
                    name,
                    engine,
                    primary,
                    unique,
                    create_time,
                    root,
                    key_type,
                    null,
                    ge,
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
