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
use crate::utils::enums::KeyType;
use comm::errors::entrances::{
    err_str, err_string, err_strings, err_strs, GeorgeError, GeorgeResult,
};
use comm::trans::{trans_i32_2_u64, trans_i64_2_u64};
use serde_json::{Error, Value};
use std::collections::HashMap;
use std::ops::Add;
use std::sync::{Arc, RwLock};

/// 比较条件 gt/ge/lt/le/eq/ne 大于/大于等于/小于/小于等于/等于/不等
#[derive(Debug, Clone, Copy)]
pub enum Compare {
    /// 大于
    GT,
    /// 大于等于
    GE,
    /// 小于
    LT,
    /// 小于等于
    LE,
    /// 等于
    EQ,
    /// 不等
    NE,
}

/// 条件查询
///
/// 查询过程中不满足条件的记录将被移除出结果集
#[derive(Debug, Clone)]
pub struct Condition {
    /// 参数名，新插入的数据将会尝试将数据对象转成json，并将json中的`param`作为参数使用
    param: String,
    /// 条件 gt/ge/lt/le/eq/ne 大于/大于等于/小于/小于等于/等于/不等
    compare: Compare,
    /// 索引值类型
    key_type: KeyType,
    /// 比较对象，支持int、string、float和bool
    value: String,
    /// 索引
    index: Option<Arc<RwLock<dyn TIndex>>>,
}

impl Condition {
    /// 参数名，新插入的数据将会尝试将数据对象转成json，并将json中的`param`作为参数使用
    fn param(&self) -> String {
        self.param.clone()
    }
    /// 条件 gt/ge/lt/le/eq/ne 大于/大于等于/小于/小于等于/等于/不等
    fn compare(&self) -> Compare {
        self.compare
    }
    /// 支持比较对象，支持int64、uint64、string、float和bool
    fn key_type(&self) -> KeyType {
        self.key_type
    }
    /// 比较对象值
    fn value(&self) -> String {
        self.value.clone()
    }
    /// 比较对象值
    fn value_u64(&self) -> GeorgeResult<u64> {
        match self.value.clone().parse() {
            Ok(res) => Ok(res),
            Err(err) => Err(self.err("u64", err.to_string())),
        }
    }
    fn value_i64(&self) -> GeorgeResult<i64> {
        match self.value.clone().parse() {
            Ok(res) => Ok(res),
            Err(err) => Err(self.err("i64", err.to_string())),
        }
    }
    fn value_u32(&self) -> GeorgeResult<u32> {
        match self.value.clone().parse() {
            Ok(res) => Ok(res),
            Err(err) => Err(self.err("u32", err.to_string())),
        }
    }
    fn value_i32(&self) -> GeorgeResult<i32> {
        match self.value.clone().parse() {
            Ok(res) => Ok(res),
            Err(err) => Err(self.err("i32", err.to_string())),
        }
    }
    fn value_f64(&self) -> GeorgeResult<f64> {
        match self.value.clone().parse() {
            Ok(res) => Ok(res),
            Err(err) => Err(self.err("f64", err.to_string())),
        }
    }
    /// 比较对象值
    fn value_bool(&self) -> bool {
        self.value.clone().eq("1")
    }
    /// 比较对象值
    fn value_compare(&self) -> GeorgeResult<u64> {
        match self.key_type {
            KeyType::U64 => self.value_u64(),
            KeyType::I64 => trans_i64_2_u64(self.value_i64()?),
            KeyType::U32 => self.value_u32() as u64,
            KeyType::I32 => trans_i32_2_u64(self.value_i32()?),
            KeyType::F64 => self.value_f64()?.to_bits(),
            _ => Err(self.err("success", err.to_string())),
        }
    }
    fn err(&self, to: &str, err: String) -> GeorgeError {
        err_strings(
            format!("{} {} can't parse to {}", self.param(), self.value(), to),
            err,
        )
    }
    /// 约束是否有效
    ///
    /// mold 索引值类型
    ///
    /// conditions 条件集合
    ///
    /// bytes 检索到的字节数组
    pub fn validate(conditions: Vec<Condition>, bytes: Vec<u8>) -> bool {
        match String::from_utf8(bytes.clone()) {
            Ok(value_str) => {
                let res: Result<Value, Error> = serde_json::from_str(value_str.as_ref());
                match res {
                    Ok(v) => {
                        for condition in conditions {
                            if !condition.valid(v.clone()) {
                                return false;
                            }
                        }
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
    /// 条件 gt/lt/eq/ne 大于/小于/等于/不等
    fn valid(&self, value: Value) -> bool {
        return match value[self.param()] {
            Value::Bool(ref val) => match self.key_type() {
                KeyType::Bool => val.eq(&self.value_bool()),
                _ => false,
            },
            Value::String(ref val) => match self.key_type() {
                KeyType::String => match self.compare() {
                    Compare::EQ => val.eq(&self.value()),
                    _ => false,
                },
                _ => false,
            },
            Value::Number(ref val) => match self.key_type() {
                KeyType::U64 => self.match_cond_u64(val.as_u64().unwrap()),
                KeyType::I64 => self.match_cond_i64(val.as_i64().unwrap()),
                KeyType::U32 => self.match_cond_u64(val.as_u64().unwrap()),
                KeyType::I32 => self.match_cond_i64(val.as_i64().unwrap()),
                KeyType::F64 => self.match_cond_f64(val.as_f64().unwrap()),
                _ => {
                    log::debug!("select valid condition does't support");
                    false
                }
            },
            _ => {
                log::debug!("select valid constraint value is not bool/string/number");
                false
            }
        };
    }
    fn match_cond_u64(&self, val: u64) -> bool {
        match self.compare() {
            Compare::EQ => val.eq(&self.value_u64().unwrap()),
            Compare::GT => val.gt(&self.value_u64().unwrap()),
            Compare::GE => val.ge(&self.value_u64().unwrap()),
            Compare::LT => val.lt(&self.value_u64().unwrap()),
            Compare::LE => val.le(&self.value_u64().unwrap()),
            Compare::NE => val.ne(&self.value_u64().unwrap()),
        }
    }
    fn match_cond_i64(&self, val: i64) -> bool {
        match self.compare() {
            Compare::EQ => val.eq(&self.value_i64().unwrap()),
            Compare::GT => val.gt(&self.value_i64().unwrap()),
            Compare::GE => val.ge(&self.value_i64().unwrap()),
            Compare::LT => val.lt(&self.value_i64().unwrap()),
            Compare::LE => val.le(&self.value_i64().unwrap()),
            Compare::NE => val.ne(&self.value_i64().unwrap()),
        }
    }
    fn match_cond_f64(&self, val: f64) -> bool {
        match self.compare() {
            Compare::EQ => val.eq(&self.value_f64().unwrap()),
            Compare::GT => val.gt(&self.value_f64().unwrap()),
            Compare::GE => val.ge(&self.value_f64().unwrap()),
            Compare::LT => val.lt(&self.value_f64().unwrap()),
            Compare::LE => val.le(&self.value_f64().unwrap()),
            Compare::NE => val.ne(&self.value_f64().unwrap()),
        }
    }
}

/// 排序方式
#[derive(Debug, Clone)]
pub struct Sort {
    /// 参数名，新插入的数据将会尝试将数据对象转成json，并将json中的`param`作为参数使用
    param: String,
    /// 是否升序
    asc: bool,
}

impl Sort {
    pub fn param(&self) -> String {
        self.param.clone()
    }
    pub fn asc(&self) -> bool {
        self.asc
    }
}

/// 查询约束
#[derive(Debug, Clone)]
pub struct Constraint {
    /// 条件查询集合
    conditions: Vec<Condition>,
    /// 结果集跳过数量
    skip: u64,
    /// 排序方式
    sort: Option<Sort>,
    /// 结果集限制数量
    limit: u64,
    /// 是否删除检索结果
    delete: bool,
}

impl Constraint {
    /// 新建查询约束
    ///
    /// selector_json_bytes 选择器字节数组，自定义转换策略
    ///
    /// delete 是否删除检索结果
    fn new(
        constraint_json_bytes: Vec<u8>,
        indexes: Arc<RwLock<HashMap<String, Arc<RwLock<dyn TIndex>>>>>,
        delete: bool,
    ) -> GeorgeResult<Constraint> {
        let mut constraint = Constraint {
            conditions: vec![],
            skip: 0,
            sort: None,
            limit: 10,
            delete,
        };
        let result: Result<Value, Error> = serde_json::from_slice(constraint_json_bytes.as_slice());
        match result {
            Ok(value) => {
                if value["Limit"].is_u64() {
                    let l = value["Limit"].as_u64().unwrap();
                    if l > 0 {
                        constraint.limit = l;
                    }
                }
                if value["Skip"].is_u64() {
                    constraint.skip = value["Skip"].as_u64().unwrap();
                }
                constraint.fit_sort(value["Sort"].clone());
                constraint.fit_conditions(indexes, value["Conditions"].clone());
                Ok(constraint)
            }
            Err(err) => Err(err_strs("new constraint", err)),
        }
    }
    pub fn conditions(&self) -> Vec<Condition> {
        self.conditions.clone()
    }
    pub fn skip(&self) -> u64 {
        self.skip
    }
    pub fn sort(&self) -> Option<Sort> {
        self.sort.clone()
    }
    pub fn limit(&self) -> u64 {
        self.limit
    }
    pub fn delete(&self) -> bool {
        self.delete
    }
    /// 解析`json value`并获取排序索引
    fn fit_sort(&mut self, value: Value) {
        if value.is_object() {
            if value["Param"].is_string() {
                let mut sort = Sort {
                    param: value["Param"].as_str().unwrap().to_string(),
                    asc: false,
                };
                if !value["Asc"].is_null() {
                    sort.asc = value["Asc"].as_bool().unwrap();
                }
                self.sort = Some(sort);
            }
        }
    }

    /// 解析`json value`并获取条件索引
    fn fit_conditions(
        &mut self,
        indexes: Arc<RwLock<HashMap<String, Arc<RwLock<dyn TIndex>>>>>,
        value: Value,
    ) -> GeorgeResult<()> {
        if value.is_array() {
            for v in value.as_array().unwrap().iter() {
                let vp: &str;
                let compare: Compare;
                let mut key_type: KeyType;
                match v["Param"].as_str() {
                    Some(ref val_param) => match v["Cond"].as_str() {
                        Some(ref val_cond) => {
                            vp = val_param;
                            if val_cond.eq(&"gt") {
                                compare = Compare::GT
                            } else if val_cond.eq(&"ge") {
                                compare = Compare::GE
                            } else if val_cond.eq(&"lt") {
                                compare = Compare::LT
                            } else if val_cond.eq(&"le") {
                                compare = Compare::LE
                            } else if val_cond.eq(&"eq") {
                                compare = Compare::EQ
                            } else if val_cond.eq(&"ne") {
                                compare = Compare::NE
                            } else {
                                break;
                            }
                        }
                        _ => return Err(err_str("fit conditions no match cond")),
                    },
                    _ => return Err(err_str("fit conditions no match param")),
                }
                match v["Type"].as_str() {
                    Some(ref val_type) => match val_type.to_lowercase().as_str() {
                        "bool" => key_type = KeyType::Bool,
                        "string" => key_type = KeyType::String,
                        "i64" => key_type = KeyType::I64,
                        "u64" => key_type = KeyType::U64,
                        "f64" => key_type = KeyType::F64,
                        _ => {
                            return Err(err_str(
                                "fit conditions type only support bool,string,i64,u64 and f64!",
                            ))
                        }
                    },
                    _ => return Err(err_str("fit conditions no match type")),
                }
                let indexes_clone = indexes.clone();
                let index_r = indexes_clone.read().unwrap();
                let mut index: Option<Arc<RwLock<dyn TIndex>>> = None;
                match index_r.get(vp) {
                    Some(idx) => {
                        index = Some(idx.clone());
                        key_type = idx.read().unwrap().key_type()
                    }
                    None => {}
                }
                match v["Value"] {
                    Value::Number(ref val_num) => {
                        match key_type {
                            KeyType::String => {
                                return Err(err_str("fit conditions no match key type"))
                            }
                            KeyType::Bool => {
                                return Err(err_str("fit conditions no match key type"))
                            }
                            _ => {}
                        }
                        self.conditions.push(Condition {
                            param: vp.to_string(),
                            compare,
                            key_type,
                            value: val_num.as_f64().unwrap().to_string(),
                            index,
                        })
                    }
                    Value::Bool(ref val_bool) => {
                        match key_type {
                            KeyType::Bool => {}
                            _ => return Err(err_str("fit conditions no match key type")),
                        }
                        let value_res: String;
                        if *val_bool {
                            value_res = String::from("1")
                        } else {
                            value_res = String::from("0")
                        }
                        self.conditions.push(Condition {
                            param: vp.to_string(),
                            compare,
                            key_type,
                            value: value_res,
                            index,
                        })
                    }
                    Value::String(ref val_str) => {
                        match key_type {
                            KeyType::String => {}
                            _ => return Err(err_str("fit conditions no match key type")),
                        }
                        self.conditions.push(Condition {
                            param: vp.to_string(),
                            compare,
                            key_type,
                            value: val_str.to_string(),
                            index,
                        })
                    }
                    _ => {
                        return Err(err_str(
                            "fit conditions value type only support bool,string and number",
                        ))
                    }
                }
            }
            Ok(())
        } else {
            return Err(err_str("fit conditions conditions is not array"));
        }
    }
}

/// 索引可用状态
#[derive(Debug, Clone)]
pub struct IndexStatus {
    /// 索引
    index: Arc<RwLock<dyn TIndex>>,
    /// 是否顺序
    asc: bool,
    /// 查询起始值
    start: u64,
    /// 查询终止值
    end: u64,
    /// 条件查询集合
    conditions: Vec<Condition>,
    /// 索引评级。asc=1；start=2；end=2。
    level: u8,
}

impl IndexStatus {
    fn index(&mut self) -> Arc<RwLock<dyn TIndex>> {
        self.index.clone()
    }
    fn fit_index(&mut self, index: Arc<RwLock<dyn TIndex>>) {
        self.index = index
    }
    fn fit_start(&mut self, start: u64) {
        if start > self.start {
            self.start = start;
            self.level = self.level.add(2)
        }
    }
    fn fit_end(&mut self, end: u64) {
        if 0 == self.end || end < self.end {
            self.end = end;
            self.level = self.level.add(2)
        }
    }
    fn append_condition(&mut self, condition: Condition) {
        self.conditions.push(condition)
    }
}

/// 经由`Selector`后的期望结果
#[derive(Debug)]
pub struct Expectation {
    /// total 检索过程中遍历的总条数（也表示文件读取次数，文件描述符次数远小于该数，一般文件描述符数为1，即共用同一文件描述符）
    pub total: u64,
    /// 检索结果总条数
    pub count: u64,
    ///  使用到的索引名称，如果没用上则为空
    pub index_name: String,
    /// 索引是否顺序
    pub asc: bool,
    /// values 检索结果集合
    pub values: Vec<Vec<u8>>,
}

/// 检索选择器
///
/// 检索顺序 sort -> conditions -> skip -> limit
#[derive(Debug, Clone)]
pub struct Selector {
    /// 索引集合
    indexes: Arc<RwLock<HashMap<String, Arc<RwLock<dyn TIndex>>>>>,
    /// 查询约束
    constraint: Constraint,
}

impl Selector {
    /// 新建检索选择器
    ///
    /// selector_json_bytes 选择器字节数组，自定义转换策略
    ///
    /// indexes 索引集合
    ///
    /// delete 是否删除检索结果
    fn run(
        constraint_json_bytes: Vec<u8>,
        indexes: Arc<RwLock<HashMap<String, Arc<RwLock<dyn TIndex>>>>>,
        delete: bool,
    ) -> GeorgeResult<Expectation> {
        let constraint = Constraint::new(constraint_json_bytes, indexes.clone(), delete)?;
        let mut select = Selector {
            indexes,
            constraint,
        };
        select.exec()
    }

    /// 执行富查询
    ///
    /// # return
    ///
    /// count 检索结果总条数
    ///
    /// index_name 使用到的索引名称，如果没用上则为空
    ///
    /// values 检索结果集合
    pub fn exec(&mut self) -> GeorgeResult<Expectation> {
        let status = self.index()?;
        log::debug!(
            "index status with start = {} & end = {}",
            status.start,
            status.end
        );
        // status自测
        if status.end != 0 && status.start > status.end {
            Err(err_string(format!(
                "condition {} end {} can't start from {}",
                status.index.read().unwrap().name(),
                status.end,
                status.start
            )))
        } else {
            self.constraint.conditions = status.conditions;
            status.index.clone().read().unwrap().select(
                status.asc,
                status.start,
                status.end,
                self.constraint.clone(),
            )
        }
    }

    /// 获取最佳索引
    ///
    /// 检索顺序 sort -> conditions -> skip -> limit
    fn index(&self) -> GeorgeResult<IndexStatus> {
        let mut oi = self.index_sort()?;
        match oi {
            Some(is) => return Ok(is),
            None => {}
        }

        oi = self.index_condition()?;
        match oi {
            Some(is) => return Ok(is),
            None => {}
        }

        match self.indexes.read().unwrap().iter().next() {
            Some(idx) => Ok(IndexStatus {
                index: idx.1.clone(),
                asc: true,
                start: 0,
                end: 0,
                conditions: self.constraint.conditions(),
                level: 0,
            }),
            None => Err(err_str("no index found!")),
        }
    }

    /// 通过sort所包含参数匹配索引
    fn index_sort(&self) -> GeorgeResult<Option<IndexStatus>> {
        match self.constraint.sort.clone() {
            Some(sort) => {
                // 通过参数匹配到排序索引
                match self.indexes.clone().read().unwrap().get(&sort.param) {
                    Some(idx) => {
                        let is = self.index_condition_param(1, sort.asc, idx)?;
                        Ok(Some(is))
                    }
                    None => Ok(None),
                }
            }
            None => Ok(None),
        }
    }

    /// 通过condition所包含参数匹配索引
    fn index_condition(&self) -> GeorgeResult<Option<IndexStatus>> {
        let mut cs: Vec<IndexStatus> = vec![];
        for condition in self.constraint.conditions.iter() {
            match condition.clone().index {
                Some(index) => cs.push(self.index_condition_param(0, true, &index)?),
                None => {}
            }
        }
        if cs.is_empty() {
            Ok(None)
        } else {
            cs.sort_by(|a, b| b.level.cmp(&a.level));
            Ok(Some(cs.get(0).unwrap().clone()))
        }
    }

    /// 通过condition所包含参数匹配索引
    ///
    /// level 起始分，asc有意义为1，无意义为0
    fn index_condition_param(
        &self,
        level: u8,
        asc: bool,
        idx: &Arc<RwLock<dyn TIndex>>,
    ) -> GeorgeResult<IndexStatus> {
        let mut status = IndexStatus {
            index: idx.clone(),
            asc,
            start: 0,
            end: 0,
            conditions: vec![],
            level,
        };
        let idx_r = idx.read().unwrap();
        // 确认排序索引是否存在条件区间
        for condition in self.constraint.conditions.iter() {
            match condition.key_type() {
                KeyType::F64 => {
                    match condition.value_f64() {
                        Ok(res) => match condition.compare {
                            // ConditionType::GT => status.fit_start(condition.value_f64().to_bits()),
                            Compare::GT => status.fit_start(res.to_bits() + 1),
                            Compare::GE => status.fit_start(res.to_bits()),
                            Compare::LT => status.fit_end(res.to_bits() - 1),
                            Compare::LE => status.fit_end(res.to_bits()),
                            Compare::EQ => {
                                if asc {
                                    status.fit_start(res.to_bits())
                                } else {
                                    status.fit_end(res.to_bits())
                                }
                            }
                            Compare::NE => {}
                        },
                        Err(err) => {
                            return Err(err_strs("index condition param, while value 2 f64", err))
                        }
                    }
                }
                _ => {
                    return Err(err_string(format!(
                        "{} can't parse except Number",
                        mold_str(idx_r.mold())
                    )));
                }
            }
        }
        Ok(status)
    }
}
