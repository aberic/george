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
use crate::utils::comm::IndexKey;
use crate::utils::enums::KeyType;
use comm::errors::entrances::{Errs, GeorgeResult};
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
    /// 比较对象为string
    value: String,
    // /// 比较对象为int/float，类索引key，可通过hash转换string生成，长度为无符号32位整型，是数据存放于索引树中的坐标
    // value_hash_32: u32,
    /// 比较对象为int/float，类索引key，可通过hash转换string生成，长度为无符号64位整型，是数据存放于索引树中的坐标
    value_hash_64: u64,
    /// 比较对象为bool
    value_bool: bool,
    /// 索引
    index: Option<Arc<dyn TIndex>>,
}

impl Condition {
    fn new(
        param: String,
        compare: Compare,
        key_type: KeyType,
        value: String,
        index: Option<Arc<dyn TIndex>>,
    ) -> GeorgeResult<Condition> {
        // let value_hash_32 = IndexKey::u32(key_type, value.clone())?;
        let value_hash_64 = IndexKey::u64(key_type, value.clone())?;
        let value_bool: bool;
        if value.eq("true") {
            value_bool = true
        } else {
            value_bool = false
        }
        Ok(Condition {
            param,
            compare,
            key_type,
            value,
            value_hash_64,
            value_bool,
            index,
        })
    }

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

    // /// 比较对象值
    // fn value_hash_32(&self) -> u32 {
    //     self.value_hash_32
    // }

    /// 比较对象值
    fn value_hash_64(&self) -> u64 {
        self.value_hash_64
    }

    /// 比较对象值
    fn value_bool(&self) -> bool {
        self.value_bool
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
                KeyType::Bool => match self.compare() {
                    Compare::EQ => self.value_bool().eq(val),
                    Compare::GT => self.value_bool().gt(val),
                    Compare::GE => self.value_bool().ge(val),
                    Compare::LT => self.value_bool().lt(val),
                    Compare::LE => self.value_bool().le(val),
                    Compare::NE => self.value_bool().ne(val),
                },
                _ => false,
            },
            Value::String(ref val) => match self.key_type() {
                KeyType::String => match self.compare() {
                    Compare::EQ => self.value().eq(val),
                    Compare::GT => self.value().gt(val),
                    Compare::GE => self.value().ge(val),
                    Compare::LT => self.value().lt(val),
                    Compare::LE => self.value().le(val),
                    Compare::NE => self.value().ne(val),
                },
                _ => false,
            },
            Value::Number(ref val) => match IndexKey::number64(self.key_type(), val) {
                Ok(real) => self.compare_value_64(real),
                _ => false,
            },
            _ => false,
        };
    }

    // /// 条件 gt/lt/eq/ne 大于/小于/等于/不等
    // fn compare_value_32(&self, value_hash: u32) -> bool {
    //     match self.compare() {
    //         Compare::EQ => value_hash == self.value_hash_32(),
    //         Compare::GT => value_hash > self.value_hash_32(),
    //         Compare::GE => value_hash >= self.value_hash_32(),
    //         Compare::LT => value_hash < self.value_hash_32(),
    //         Compare::LE => value_hash <= self.value_hash_32(),
    //         Compare::NE => value_hash != self.value_hash_32(),
    //     }
    // }

    /// 条件 gt/lt/eq/ne 大于/小于/等于/不等
    fn compare_value_64(&self, value_hash: u64) -> bool {
        match self.compare() {
            Compare::EQ => value_hash == self.value_hash_64(),
            Compare::GT => value_hash > self.value_hash_64(),
            Compare::GE => value_hash >= self.value_hash_64(),
            Compare::LT => value_hash < self.value_hash_64(),
            Compare::LE => value_hash <= self.value_hash_64(),
            Compare::NE => value_hash != self.value_hash_64(),
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
    /// 索引
    index: Option<Arc<dyn TIndex>>,
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
    /// * 如果条件语句`Limit`条件为`u64`，则填充赋值
    /// * 如果条件语句`Skip`条件为`u64`，则填充赋值
    /// * 解析json value中Sort条件并尝试获取排序限定
    /// * 解析json value中`Conditions`条件并尝试获取条件限定
    ///
    /// # param
    /// * constraint_json_bytes 选择器字节数组，自定义转换策略
    /// * indexes 索引集合
    /// * delete 是否删除检索结果
    fn new(
        constraint_json_bytes: Vec<u8>,
        indexes: Arc<RwLock<HashMap<String, Arc<dyn TIndex>>>>,
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
                // 如果条件语句`Limit`条件为`u64`，则填充赋值
                if value["Limit"].is_u64() {
                    constraint.limit = value["Limit"].as_u64().unwrap();
                }
                // 如果条件语句`Skip`条件为`u64`，则填充赋值
                if value["Skip"].is_u64() {
                    constraint.skip = value["Skip"].as_u64().unwrap();
                }
                // 解析json value中Sort条件并尝试获取排序限定
                constraint.fit_sort(indexes.clone(), value["Sort"].clone());
                // 解析json value中`Conditions`条件并尝试获取条件限定
                constraint.fit_conditions(indexes, value["Conditions"].clone())?;
                Ok(constraint)
            }
            Err(err) => Err(Errs::strs("new constraint", err)),
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

    /// 解析`json value`中`Sort`条件并尝试获取排序限定
    ///
    /// * 如果条件语句`Sort`条件非对象格式，则不处理，否则继续尝试填充赋值
    /// * 如果条件语句`Param`不是字符串类型，则不处理，否则继续尝试填充赋值
    /// * 如果条件语句`Asc`为bool类型，则继续尝试填充赋值
    /// * 如果条件语句`Asc`非空，则填充Asc，否则默认为true
    /// * 初始化可匹配索引，该索引最终可为`None`，后续被赋值也用于索引选择优化
    ///
    /// # param
    /// * indexes 索引集合
    /// * value 条件语句`Sort`条件反射对象
    fn fit_sort(&mut self, indexes: Arc<RwLock<HashMap<String, Arc<dyn TIndex>>>>, value: Value) {
        // 如果条件语句`Sort`条件非对象格式，则不处理，否则继续尝试填充赋值
        if value.is_object() {
            // 如果条件语句`Param`不是字符串类型，则不处理，否则继续尝试填充赋值
            if value["Param"].is_string() {
                let param = value["Param"].as_str().unwrap();
                // 如果条件语句`Asc`为bool类型，则继续尝试填充赋值
                if value["Asc"].is_boolean() {
                    let indexes_clone = indexes.clone();
                    let index_r = indexes_clone.read().unwrap();
                    // 初始化当前单一对象可匹配索引为`None`，该索引最终可为`None`，后续被赋值也用于索引选择优化
                    let mut index: Option<Arc<dyn TIndex>> = None;
                    // 尝试在索引集合中通过`Param`获取可匹配索引，如有，则进行对应赋值
                    match index_r.get(param) {
                        Some(idx) => {
                            // 赋值当前可匹配索引
                            index = Some(idx.clone());
                        }
                        None => {}
                    }
                    // 排序条件
                    let asc: bool;
                    // 如果条件语句`Asc`非空，则填充Asc，否则默认为true
                    if !value["Asc"].is_null() {
                        asc = value["Asc"].as_bool().unwrap();
                    } else {
                        asc = true;
                    }
                    self.sort = Some(Sort {
                        param: param.to_string(),
                        asc,
                        index,
                    })
                }
            }
        }
    }

    /// 解析json value中`Conditions`条件并尝试获取条件限定
    /// * 如果条件语句`Conditions`条件非数组格式，则不处理，否则继续尝试填充赋值
    /// * 条件数组单一对象中`Param`条件字符串，一般内容如`age`、`level`等
    /// * 解析`Param`，任一单一对象中都不能缺省`Param`，否则返回对应错误
    /// * 解析`Cond`，任一单一对象中都不能缺省`Cond`，否则返回对应错误
    /// * 解析`Value`，任一单一对象中都不能缺省`Value`，否则返回对应错误
    /// * 为所有单一对象初始化可匹配索引，该索引最终可为`None`，后续被赋值也用于索引选择优化
    /// * 将单一对象解析出来的新条件追加到条件查询集合
    ///
    /// # param
    /// * indexes 索引集合
    /// * value 条件语句`Conditions`条件反射对象
    fn fit_conditions(
        &mut self,
        indexes: Arc<RwLock<HashMap<String, Arc<dyn TIndex>>>>,
        value: Value,
    ) -> GeorgeResult<()> {
        // 如果条件语句`Conditions`条件非数组格式，则不处理，否则继续尝试填充赋值
        if value.is_array() {
            // 遍历筛选条件数组
            for v in value.as_array().unwrap().iter() {
                // 条件数组单一对象中`Param`条件字符串，一般内容如`age`、`level`等
                let param: &str;
                // 比较条件 gt/ge/lt/le/eq/ne 大于/大于等于/小于/小于等于/等于/不等
                let compare: Compare;
                // 条件值类型，初始化默认为None
                let mut key_type: KeyType = KeyType::None;
                // 开始解析单一对象
                // 解析`Param`，任一单一对象中都不能缺省`Param`，否则返回对应错误
                match v["Param"].as_str() {
                    Some(ref val_param) => param = val_param,
                    _ => return Err(Errs::str("fit conditions no match param")),
                }
                // 解析`Cond`，任一单一对象中都不能缺省`Cond`，否则返回对应错误
                match v["Cond"].as_str() {
                    Some(ref val_cond) => {
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
                            return Err(Errs::string(format!(
                                "fit conditions cond {} only support gt,ge,lt,le,eq and ne",
                                val_cond
                            )));
                        }
                    }
                    _ => return Err(Errs::str("fit conditions no match cond")),
                }
                let indexes_clone = indexes.clone();
                let index_r = indexes_clone.read().unwrap();
                // 初始化当前单一对象可匹配索引为`None`，该索引最终可为`None`，后续被赋值也用于索引选择优化
                let mut index: Option<Arc<dyn TIndex>> = None;
                // 尝试在索引集合中通过`Param`获取可匹配索引，如有，则进行对应赋值
                match index_r.get(param) {
                    Some(idx) => {
                        // 赋值当前单一对象可匹配索引
                        index = Some(idx.clone());
                        // 赋值条件值类型
                        key_type = idx.key_type();
                    }
                    None => {}
                }
                // `Value`值字符串形式
                let val_str: String;
                // 解析`Value`，任一单一对象中都不能缺省`Value`，否则返回对应错误
                // `Value`值类型需要与key_type匹配，否则返回对应错误
                match v["Value"] {
                    // 如果`Value`为数字类型(包括整数和浮点数)
                    Value::Number(ref res) => {
                        log::debug!("value number, key_type = {:#?}", key_type);
                        match key_type {
                            KeyType::None => key_type = KeyType::F64,
                            KeyType::String => {
                                return Err(Errs::str("fit conditions no match key type"))
                            }
                            KeyType::Bool => {
                                return Err(Errs::str("fit conditions no match key type"))
                            }
                            _ => {}
                        }
                        val_str = res.to_string();
                    }
                    // 如果`Value`为布尔类型
                    Value::Bool(ref res) => {
                        match key_type {
                            KeyType::None => key_type = KeyType::None,
                            KeyType::Bool => {}
                            _ => return Err(Errs::str("fit conditions no match key type")),
                        }
                        val_str = res.to_string();
                    }
                    // 如果`Value`为字符串类型
                    Value::String(ref res) => {
                        match key_type {
                            KeyType::None => key_type = KeyType::String,
                            KeyType::String => {}
                            _ => return Err(Errs::str("fit conditions no match key type")),
                        }
                        val_str = res.to_string();
                    }
                    _ => {
                        return Err(Errs::str(
                            "fit conditions value type only support bool,string and number",
                        ))
                    }
                }
                // 追加新的条件到条件查询集合
                self.conditions.push(Condition::new(
                    param.to_string(),
                    compare,
                    key_type,
                    val_str,
                    index,
                )?)
            }
            Ok(())
        } else {
            return Err(Errs::str("fit conditions conditions is not array"));
        }
    }
}

/// 索引可用状态
#[derive(Debug, Clone)]
pub struct IndexStatus {
    /// 索引
    index: Arc<dyn TIndex>,
    /// 是否顺序
    asc: bool,
    /// 是否更新过顺序
    asc_update: bool,
    /// 查询起始值
    start: u64,
    /// 查询终止值
    end: u64,
    /// 条件查询集合
    conditions: Vec<Condition>,
    /// 索引评级。asc=1；start=2；end=2
    level: u8,
}

impl IndexStatus {
    fn new(index: Arc<dyn TIndex>, conditions: Vec<Condition>) -> IndexStatus {
        IndexStatus {
            index,
            asc: true,
            asc_update: false,
            start: 0,
            end: 0,
            conditions,
            level: 0,
        }
    }

    fn fit_sort(&mut self, sort: bool) {
        self.asc = sort;
        // 如果当前排序更新过至少一次，评分不再追加
        if !self.asc_update {
            self.level = self.level.add(1);
        }
    }

    fn fit_start(&mut self, start: u64) {
        // 如果待填充起始值大于当前，则继续更新值
        if start > self.start {
            // 如果当前起始值大于0，则表示已经更新过至少一次，评分不再追加
            if self.start == 0 {
                self.level = self.level.add(2);
            }
            // 更新当前值
            self.start = start;
        }
    }

    fn fit_end(&mut self, end: u64) {
        // 如果待填充终止值小于当前，则继续更新值
        if end < self.end {
            // 如果当前终止值大于0，则表示已经更新过至少一次，评分不再追加
            if self.end == 0 {
                self.level = self.level.add(2);
            }
            // 更新当前值
            self.end = end;
        }
    }
}

/// 经由`Selector`后的期望结果
#[derive(Debug)]
pub struct Expectation {
    /// total 检索过程中遍历的总条数（也表示文件读取次数，文件描述符次数远小于该数，一般文件描述符数为1，即共用同一文件描述符）
    pub total: u64,
    /// 检索结果过程中遍历的总条数
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
/// 最佳检索选择策略 sort -> conditions -> skip -> limit
#[derive(Debug, Clone)]
pub struct Selector {
    /// 索引集合
    indexes: Arc<RwLock<HashMap<String, Arc<dyn TIndex>>>>,
    /// 查询约束
    constraint: Constraint,
}

impl Selector {
    /// 新建检索选择器<p>
    ///
    /// # param
    /// * constraint_json_bytes 选择器字节数组，自定义转换策略
    /// * indexes 索引集合
    /// * delete 是否删除检索结果
    ///
    /// # return
    /// * Expectation 经由Selector后的期望结果
    pub(crate) fn run(
        constraint_json_bytes: Vec<u8>,
        indexes: Arc<RwLock<HashMap<String, Arc<dyn TIndex>>>>,
        delete: bool,
    ) -> GeorgeResult<Expectation> {
        // 新建查询约束
        let constraint = Constraint::new(constraint_json_bytes, indexes.clone(), delete)?;
        // 新建检索选择器并执行富查询
        Selector {
            indexes,
            constraint,
        }
        .exec()
    }
    fn constraint(&self) -> Constraint {
        self.constraint.clone()
    }

    /// 执行富查询<p>
    ///
    /// # return
    /// * Expectation 经由Selector后的期望结果
    fn exec(&mut self) -> GeorgeResult<Expectation> {
        // 获取最佳索引
        let status = self.index()?;
        self.constraint.conditions = status.conditions;
        status.index.clone().select(
            status.asc,
            status.start,
            status.end,
            self.constraint.clone(),
        )
    }

    /// 获取最佳索引，以减少磁盘读取次数为目的，遵守区间大于一切的准则<p>
    /// # Policy
    /// * 如果有多个闭合区间约束的索引，优先选择区间差最小的，如存在`height`[1..10]和`age`[1..5]时，选择`age`
    /// * 如果有一个闭合区间约束的索引，如`height`[1..10]，选择`height`
    /// * 如果存在开闭区间与排序的索引，当索引相同时，如存在`height`[1..]和`height`[asc:false]，选择`height`
    /// * 如果存在开闭区间与排序的索引，当索引不同时，如存在`height`[1..]和`age`[asc:false]，选择`height`
    /// * 索引选择存在相同策略结果时，一般情况下先到先得，但不保证一定，具备随机性
    ///
    /// # return
    /// * Expectation 经由Selector后的期望结果
    fn index(&mut self) -> GeorgeResult<IndexStatus> {
        // 优先进行区间判断，如果不存在区间策略，再进行后续策略
        match self.index_policy() {
            Some(is) => return Ok(is),
            None => {}
        }

        match self.indexes.read().unwrap().iter().next() {
            Some(idx) => Ok(IndexStatus::new(
                idx.1.clone(),
                self.constraint.conditions(),
            )),
            None => Err(Errs::str("no index found!")),
        }
    }

    /// 通过condition所包含参数匹配索引
    fn index_policy(&self) -> Option<IndexStatus> {
        // 新建索引可用状态集合
        let mut cs: Vec<IndexStatus> = vec![];
        // 遍历已有索引集合，从区间条件中进行匹配
        for (index_name, index) in self.indexes.read().unwrap().iter() {
            let mut status = IndexStatus::new(index.clone(), vec![]);
            // 默认顺序
            let mut asc = true;
            // 判断是否存在排序条件
            match self.constraint().sort() {
                Some(sort) => {
                    // 判断该条件是否存在索引支持
                    match sort.clone().index {
                        Some(index) => {
                            // 判断该条件索引是否为正在遍历的索引
                            if index.name().eq(index_name) {
                                asc = sort.asc();
                                status.fit_sort(sort.asc())
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            // 遍历区间条件，与索引名称进行匹配
            for condition in self.constraint().conditions().iter() {
                // 判断该条件是否存在索引支持
                match condition.clone().index {
                    Some(index) => {
                        // 判断该条件索引是否为正在遍历的索引
                        if index.name().eq(index_name) {
                            // 将该索引条件进行填充
                            match condition.compare() {
                                Compare::GT => status.fit_start(condition.value_hash_64() + 1),
                                Compare::GE => status.fit_start(condition.value_hash_64()),
                                Compare::LT => status.fit_end(condition.value_hash_64() - 1),
                                Compare::LE => status.fit_end(condition.value_hash_64()),
                                Compare::EQ => {
                                    if asc {
                                        status.fit_start(condition.value_hash_64())
                                    } else {
                                        status.fit_end(condition.value_hash_64())
                                    }
                                }
                                Compare::NE => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            // 索引状态集合追加
            cs.push(status);
        }
        // 索引状态集合为空则返回None
        if cs.is_empty() {
            None
        } else {
            // 索引状态集合按照评分
            cs.sort_by(|a, b| b.level.cmp(&a.level));
            Some(cs.get(0).unwrap().clone())
        }
    }
}
