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
use std::fmt;
use std::ops::Add;
use std::sync::{Arc, RwLock};

use chrono::Duration;
use serde::__private::fmt::Debug;

use comm::errors::{Errs, GeorgeResult};
use comm::strings::StringHandler;
use comm::{Strings, Time};

use crate::task::engine::DataReal;
use crate::task::rich::{Condition, Constraint, Expectation};
use crate::task::seed::IndexPolicy;
use crate::task::traits::TForm;
use crate::utils::enums::{Engine, KeyType};

/// 索引通用特性，遵循此特性创建索引可以更方便的针对进行扩展
///
/// 该特性包含了索引的基本方法，理论上都需要进行实现才能使用
pub(crate) trait TIndex: Send + Sync + Debug {
    fn form(&self) -> Arc<RwLock<dyn TForm>>;
    fn database_name(&self) -> String;
    fn view_name(&self) -> String;
    /// 索引名称，可以自定义；<p>
    /// siam::Index 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入<p><p>
    fn name(&self) -> String;
    /// 存储引擎类型
    fn engine(&self) -> Engine;
    /// 索引值类型
    fn key_type(&self) -> KeyType;
    /// 创建时间
    fn create_time(&self) -> Time;
    /// 插入数据，如果存在原值，不覆盖原结果<p><p>
    ///
    /// ###Params
    ///
    /// key string
    ///
    /// force 如果存在原值，是否覆盖原结果
    ///
    /// ###Return
    ///
    /// EngineResult<()>
    fn put(&self, key: String, seed: Arc<RwLock<dyn TSeed>>, force: bool) -> GeorgeResult<()>;
    /// 获取数据，返回存储对象<p><p>
    ///
    /// ###Params
    ///
    /// key strings
    ///
    /// ###Return
    ///
    /// DataReal 真实存入文件中的信息
    fn get(&self, key: String) -> GeorgeResult<DataReal>;
    /// 删除数据<p><p>
    ///
    /// ###Params
    ///
    /// key strings
    ///
    /// ###Return
    ///
    /// Seed value信息
    fn del(&self, key: String, seed: Arc<RwLock<dyn TSeed>>) -> GeorgeResult<()>;
    /// 通过查询约束获取数据集
    ///
    /// ###Params
    ///
    /// left 是否左查询
    ///
    /// constraint 查询约束
    ///
    /// ###Return
    ///
    /// Expectation 经由Selector后的期望结果
    fn select(
        &self,
        left: bool,
        start: u64,
        end: u64,
        constraint: Constraint,
    ) -> GeorgeResult<Expectation>;
}

/// 结点通用特性，遵循此特性创建结点可以更方便的针对db进行扩展
///
/// 该特性包含了结点的基本方法，理论上都需要进行实现才能使用
pub(crate) trait TNode: Send + Sync + Debug {
    /// 插入数据<p><p>
    ///
    /// force 如果存在原值，是否覆盖原结果
    ///
    /// ###Return
    ///
    /// EngineResult<()>
    fn put(&self, key: String, seed: Arc<RwLock<dyn TSeed>>, force: bool) -> GeorgeResult<()>;
    /// 获取数据，返回存储对象<p><p>
    ///
    /// ###Params
    ///
    /// key string
    ///
    /// ###Return
    ///
    /// DataReal 真实存入文件中的信息
    fn get(&self, key: String) -> GeorgeResult<DataReal>;
    /// 删除数据<p><p>
    ///
    /// ###Params
    ///
    /// key string
    ///
    /// ###Return
    ///
    /// Seed value信息
    fn del(&self, key: String, seed: Arc<RwLock<dyn TSeed>>) -> GeorgeResult<()>;
    /// 通过查询约束获取数据集
    ///
    /// ###Params
    ///
    /// * left 是否左查询
    /// * start 查询起始坐标，如为0则表示前置数据没有起始符
    /// * end 查询终止坐标，如为0则表示后续数据没有终止符
    /// * skip 结果集跳过数量
    /// * limit 结果集限制数量
    /// * delete 是否删除检索结果
    /// * conditions 条件集合
    ///
    /// ###Return
    ///
    /// * total 检索过程中遍历的总条数（也表示文件读取次数，文件描述符次数远小于该数，一般文件描述符数为1，即共用同一文件描述符）
    /// * count 检索结果过程中遍历的总条数
    /// * values 检索结果集合
    fn select(
        &self,
        left: bool,
        start: u64,
        end: u64,
        skip: u64,
        limit: u64,
        delete: bool,
        conditions: Vec<Condition>,
    ) -> GeorgeResult<(u64, u64, Vec<Vec<u8>>)>;
}

/// B+Tree索引叶子结点内防hash碰撞数组对象中对象特性
///
/// 搭配Index使用
///
/// thread::spawn需要一个实现Send的闭包。Arc<T>只实现发送，需要实现发送和同步以支持多线程调用
///
/// 发送和同步是自动特性。可以添加自动特征到一个dyn特征类型
///
/// 如果要写dyn Trait + Send + Sync到处都是，那么需要声明Send和Sync是Trait的Super Traits
pub(crate) trait TSeed: Send + Sync + Debug {
    /// 获取当前结果原始key信息
    fn key(&self) -> String;
    /// value最终存储在文件中的内容
    fn value(&self) -> Vec<u8>;
    /// value最终存储在文件中的内容
    fn increment(&self) -> u64;
    /// 修改value值
    fn modify_4_put(&mut self, index_policy: IndexPolicy);
    /// 修改value值
    fn modify_4_del(&mut self, index_policy: IndexPolicy);
    /// 存储操作
    fn save(&self) -> GeorgeResult<()>;
    /// 删除操作
    fn remove(&self) -> GeorgeResult<()>;
}

/// 归档服务
#[derive(Clone)]
pub(crate) struct Pigeonhole {
    now: Record,
    pub(crate) history: HashMap<u16, Record>,
}

impl fmt::Debug for Pigeonhole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut histories = String::from("");
        for (_, his) in self.history.iter() {
            histories = histories.add(his.to_string().as_str());
        }
        write!(f, "[now = {:#?}, histories = {:#?}]", self.now, histories)
    }
}

impl Pigeonhole {
    pub(crate) fn create(version: u16, filepath: String, create_time: Time) -> Pigeonhole {
        Pigeonhole {
            now: Record {
                version,
                filepath,
                create_time,
            },
            history: Default::default(),
        }
    }

    /// 当前归档版本
    pub(crate) fn now(&self) -> Record {
        self.now.clone()
    }

    /// 历史归档版本
    pub(crate) fn history(&self) -> HashMap<u16, Record> {
        self.history.clone()
    }

    fn history_to_string(&self) -> String {
        let mut res = String::from("");
        for (_, record) in self.history.iter() {
            if res.is_empty() {
                res = res.add(&record.to_string());
            } else {
                res = res.add("@_@!");
                res = res.add(&record.to_string());
            }
        }
        res
    }

    fn history_from_string(history_desc: String) -> GeorgeResult<HashMap<u16, Record>> {
        let mut history: HashMap<u16, Record> = Default::default();
        if !history_desc.is_empty() {
            let split = history_desc.split("$_$!");
            for record_desc in split.into_iter() {
                let record = Record::from_string(String::from(record_desc))?;
                history.insert(record.version, record);
            }
        }
        Ok(history)
    }

    /// 生成文件描述
    pub(crate) fn to_string(&self) -> String {
        hex::encode(format!(
            "{}$_$!{}",
            self.now().to_string(),
            self.history_to_string()
        ))
    }

    /// 通过文件描述恢复结构信息
    pub(crate) fn from_string(pigeonhole_desc: String) -> GeorgeResult<Pigeonhole> {
        match hex::decode(pigeonhole_desc) {
            Ok(vu8) => {
                let real = Strings::from_utf8(vu8)?;
                let mut split = real.split("$_$!");
                let now = Record::from_string(split.next().unwrap().to_string())?;
                let history = Pigeonhole::history_from_string(split.next().unwrap().to_string())?;
                Ok(Pigeonhole { now, history })
            }
            Err(err) => Err(Errs::string(format!(
                "recovery pigeonhole from utf8 1 failed! error is {}",
                err
            ))),
        }
    }
}

/// 归档记录
#[derive(Clone)]
pub(crate) struct Record {
    /// 归档版本，默认新建为[0x00,0x00]，版本每次归档操作递增，最多归档65536次
    pub(crate) version: u16,
    /// 当前归档版本文件所处路径
    filepath: String,
    /// 归档时间
    create_time: Time,
}

impl fmt::Debug for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[version = {:#?}, filepath = {}, create_time = {}]",
            self.version(),
            self.filepath(),
            self.create_time.format("%Y-%m-%d %H:%M:%S")
        )
    }
}

impl Record {
    fn create(version: u16, filepath: String, create_time: Time) -> Record {
        Record {
            version,
            filepath,
            create_time,
        }
    }

    /// 归档版本，默认新建为[0x00,0x00]，版本每次归档操作递增，最多归档65536次
    pub(crate) fn version(&self) -> u16 {
        self.version
    }

    /// 当前归档版本文件所处路径
    pub(crate) fn filepath(&self) -> String {
        self.filepath.clone()
    }

    /// 归档时间
    pub(crate) fn create_time(&self) -> Time {
        self.create_time.clone()
    }

    /// 生成文件描述
    pub(crate) fn to_string(&self) -> String {
        hex::encode(format!(
            "{}|{}|{}",
            self.version(),
            self.filepath(),
            self.create_time().nano_string().unwrap()
        ))
    }

    /// 通过文件描述恢复结构信息
    pub(crate) fn from_string(record_desc: String) -> GeorgeResult<Record> {
        match hex::decode(record_desc) {
            Ok(vu8) => {
                let real = Strings::from_utf8(vu8)?;
                let mut split = real.split("|");
                let version = split.next().unwrap().to_string().parse::<u16>().unwrap();
                let filepath = split.next().unwrap().to_string();
                let duration = Duration::nanoseconds(
                    split.next().unwrap().to_string().parse::<i64>().unwrap(),
                );
                Ok(Record::create(version, filepath, Time::from(duration)))
            }
            Err(err) => Err(Errs::string(format!(
                "recovery pigeonhole from utf8 1 failed! error is {}",
                err
            ))),
        }
    }
}
