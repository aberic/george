use std::collections::HashMap;
use std::ops::Add;
use std::sync::{Arc, RwLock};

use serde_json::{Error, Value};

use comm::errors::entrances::{err_str, err_string, err_string_enhance, GeorgeError, GeorgeResult};

use crate::engine::siam::comm::{i32_2_u64, i64_2_u64};
use crate::engine::traits::TIndex;
use crate::utils::comm::IndexMold;
use crate::utils::store::mold_str;

/// 条件 gt/ge/lt/le/eq/ne 大于/大于等于/小于/小于等于/等于/不等
#[derive(Debug, Clone, Copy)]
pub enum ConditionType {
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

/// 支持比较对象，支持int、string、float和bool
#[derive(Debug, Clone, Copy)]
pub enum ConditionSupport {
    /// string
    String,
    /// float
    Number,
    /// bool
    Bool,
}

/// 条件查询
///
/// 查询过程中不满足条件的记录将被移除出结果集
#[derive(Debug, Clone)]
pub struct Condition {
    /// 参数名，新插入的数据将会尝试将数据对象转成json，并将json中的`param`作为参数使用
    param: String,
    /// 条件 gt/ge/lt/le/eq/ne 大于/大于等于/小于/小于等于/等于/不等
    cond: ConditionType,
    /// 支持比较对象，支持int64、uint64、string、float和bool
    support: ConditionSupport,
    /// 比较对象，支持int、string、float和bool
    value: String,
}

impl Condition {
    /// 参数名，新插入的数据将会尝试将数据对象转成json，并将json中的`param`作为参数使用
    fn param(&self) -> String {
        self.param.clone()
    }
    /// 条件 gt/ge/lt/le/eq/ne 大于/大于等于/小于/小于等于/等于/不等
    fn cond(&self) -> ConditionType {
        self.cond
    }
    /// 支持比较对象，支持int64、uint64、string、float和bool
    fn support(&self) -> ConditionSupport {
        self.support
    }
    fn support_check(&self, mold: IndexMold) -> bool {
        match self.support {
            ConditionSupport::Number => match mold {
                IndexMold::String => false,
                _ => true,
            },
            ConditionSupport::String => match mold {
                IndexMold::String => true,
                _ => false,
            },
            _ => false,
        }
    }
    /// 比较对象值
    fn value_str(&self) -> String {
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
    fn err(&self, to: &str, err: String) -> GeorgeError {
        err_string_enhance(
            format!(
                "{} {} can't parse to {}",
                self.param(),
                self.value_str(),
                to
            ),
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
    pub fn validate(mold: IndexMold, conditions: Vec<Condition>, bytes: Vec<u8>) -> bool {
        match String::from_utf8(bytes.clone()) {
            Ok(value_str) => {
                let res: Result<Value, Error> = serde_json::from_str(value_str.as_ref());
                match res {
                    Ok(v) => {
                        for condition in conditions {
                            if !condition.valid(mold, v.clone()) {
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
    fn valid(&self, mold: IndexMold, value: Value) -> bool {
        return match value[self.param()] {
            Value::Bool(ref val) => match self.support() {
                ConditionSupport::Bool => val.eq(&self.value_bool()),
                _ => false,
            },
            Value::String(ref val) => match self.support() {
                ConditionSupport::String => match self.cond() {
                    ConditionType::EQ => val.eq(&self.value_str()),
                    _ => false,
                },
                _ => false,
            },
            Value::Number(ref val) => match self.support() {
                ConditionSupport::Number => match mold {
                    IndexMold::U64 => self.match_cond_u64(val.as_u64().unwrap()),
                    IndexMold::I64 => self.match_cond_i64(val.as_i64().unwrap()),
                    IndexMold::U32 => self.match_cond_u64(val.as_u64().unwrap()),
                    IndexMold::I32 => self.match_cond_i64(val.as_i64().unwrap()),
                    IndexMold::F64 => self.match_cond_f64(val.as_f64().unwrap()),
                    _ => false,
                },
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
        match self.cond() {
            ConditionType::EQ => val.eq(&self.value_u64().unwrap()),
            ConditionType::GT => val.gt(&self.value_u64().unwrap()),
            ConditionType::GE => val.ge(&self.value_u64().unwrap()),
            ConditionType::LT => val.lt(&self.value_u64().unwrap()),
            ConditionType::LE => val.le(&self.value_u64().unwrap()),
            ConditionType::NE => val.ne(&self.value_u64().unwrap()),
        }
    }
    fn match_cond_i64(&self, val: i64) -> bool {
        match self.cond() {
            ConditionType::EQ => val.eq(&self.value_i64().unwrap()),
            ConditionType::GT => val.gt(&self.value_i64().unwrap()),
            ConditionType::GE => val.ge(&self.value_i64().unwrap()),
            ConditionType::LT => val.lt(&self.value_i64().unwrap()),
            ConditionType::LE => val.le(&self.value_i64().unwrap()),
            ConditionType::NE => val.ne(&self.value_i64().unwrap()),
        }
    }
    fn match_cond_f64(&self, val: f64) -> bool {
        match self.cond() {
            ConditionType::EQ => val.eq(&self.value_f64().unwrap()),
            ConditionType::GT => val.gt(&self.value_f64().unwrap()),
            ConditionType::GE => val.ge(&self.value_f64().unwrap()),
            ConditionType::LT => val.lt(&self.value_f64().unwrap()),
            ConditionType::LE => val.le(&self.value_f64().unwrap()),
            ConditionType::NE => val.ne(&self.value_f64().unwrap()),
        }
    }
}

/// 排序方式
#[derive(Debug, Clone)]
struct Sort {
    /// 参数名，新插入的数据将会尝试将数据对象转成json，并将json中的`param`作为参数使用
    param: String,
    /// 是否升序
    asc: bool,
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
    fn new(constraint_json_bytes: Vec<u8>, delete: bool) -> GeorgeResult<Constraint> {
        let mut constraint = Constraint {
            conditions: vec![],
            skip: 0,
            sort: None,
            limit: 0,
            delete,
        };
        let result: Result<Value, Error> = serde_json::from_slice(constraint_json_bytes.as_slice());
        match result {
            Ok(value) => {
                if value["Limit"].is_u64() {
                    constraint.limit = value["Limit"].as_u64().unwrap();
                }
                if value["Skip"].is_u64() {
                    constraint.skip = value["Skip"].as_u64().unwrap();
                }
                constraint.fit_sort(value["Sort"].clone());
                constraint.fit_conditions(value["Conditions"].clone());
                Ok(constraint)
            }
            Err(err) => Err(err_string(err.to_string())),
        }
    }
    pub fn conditions(&self) -> Vec<Condition> {
        self.conditions.clone()
    }
    pub fn skip(&self) -> u64 {
        self.skip
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
    fn fit_conditions(&mut self, value: Value) {
        if value.is_array() {
            for v in value.as_array().unwrap().iter() {
                let cond: ConditionType;
                match v["Param"].as_str() {
                    Some(ref val_param) => match v["Cond"].as_str() {
                        Some(ref val_cond) => {
                            if val_cond.eq(&"gt") {
                                cond = ConditionType::GT
                            } else if val_cond.eq(&"ge") {
                                cond = ConditionType::GE
                            } else if val_cond.eq(&"lt") {
                                cond = ConditionType::LT
                            } else if val_cond.eq(&"le") {
                                cond = ConditionType::LE
                            } else if val_cond.eq(&"eq") {
                                cond = ConditionType::EQ
                            } else if val_cond.eq(&"ne") {
                                cond = ConditionType::NE
                            } else {
                                break;
                            }
                            match v["Value"] {
                                Value::Bool(ref val_bool) => {
                                    let value_res: String;
                                    if *val_bool {
                                        value_res = String::from("1")
                                    } else {
                                        value_res = String::from("0")
                                    }
                                    self.conditions.push(Condition {
                                        param: val_param.to_string(),
                                        cond,
                                        support: ConditionSupport::Bool,
                                        value: value_res,
                                    })
                                }
                                Value::String(ref val_str) => self.conditions.push(Condition {
                                    param: val_param.to_string(),
                                    cond,
                                    support: ConditionSupport::String,
                                    value: val_str.to_string(),
                                }),
                                Value::Number(ref val_num) => self.conditions.push(Condition {
                                    param: val_param.to_string(),
                                    cond,
                                    support: ConditionSupport::Number,
                                    value: val_num.as_f64().unwrap().to_string(),
                                }),
                                _ => {}
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
}

/// 索引可用状态
#[derive(Debug, Clone)]
struct IndexStatus {
    /// 索引
    index: Arc<RwLock<dyn TIndex>>,
    /// 是否顺序
    asc: bool,
    /// 查询起始值
    start: u64,
    /// 查询终止值
    end: u64,
    /// 索引评级。asc=1；start=2；end=3。
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
}

/// 检索选择器
///
/// 检索顺序 sort -> conditions -> skip -> limit
#[derive(Debug, Clone)]
pub struct Selector {
    /// 索引集合
    pub indexes: Arc<RwLock<HashMap<String, Arc<RwLock<dyn TIndex>>>>>,
    /// 查询约束
    pub constraint: Constraint,
}

impl Selector {
    /// 新建检索选择器
    ///
    /// selector_json_bytes 选择器字节数组，自定义转换策略
    ///
    /// indexes 索引集合
    ///
    /// delete 是否删除检索结果
    pub fn run(
        constraint_json_bytes: Vec<u8>,
        indexes: Arc<RwLock<HashMap<String, Arc<RwLock<dyn TIndex>>>>>,
        delete: bool,
    ) -> GeorgeResult<Expectation> {
        let constraint = Constraint::new(constraint_json_bytes, delete)?;
        let select = Selector {
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
    pub fn exec(&self) -> GeorgeResult<Expectation> {
        let status = self.index()?;
        // todo 移除多余condition
        // status自测
        if status.start > status.end {
            Err(err_string(format!(
                "condition {} end {} can't start from {}",
                status.index.read().unwrap().key_structure(),
                status.end,
                status.start
            )))
        } else {
            status
                .index
                .clone()
                .read()
                .unwrap()
                .select(status.asc, self.constraint.clone())
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
                let index = self.index_param(sort.param);
                match index {
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
            match self.index_param(condition.param.clone()) {
                Some(index) => cs.push(self.index_condition_param(0, true, index)?),
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
        idx: Arc<RwLock<dyn TIndex>>,
    ) -> GeorgeResult<IndexStatus> {
        let mut status = IndexStatus {
            index: idx.clone(),
            asc,
            start: 0,
            end: 0,
            level,
        };
        let idx_r = idx.read().unwrap();
        // 确认排序索引是否存在条件区间
        for condition in self.constraint.conditions.iter() {
            if condition.param.clone() == idx_r.key_structure() {
                if !condition.support_check(idx_r.mold()) {
                    return Err(err_string(format!(
                        "condition param can't support index {}",
                        idx_r.key_structure()
                    )));
                }
                match condition.support() {
                    ConditionSupport::Number => match idx_r.mold() {
                        IndexMold::U64 => match condition.cond {
                            ConditionType::GT => match condition.value_u64() {
                                Ok(res) => status.fit_start(res + 1),
                                Err(err) => return Err(err),
                            },
                            ConditionType::GE => match condition.value_u64() {
                                Ok(res) => status.fit_start(res),
                                Err(err) => return Err(err),
                            },
                            ConditionType::LT => match condition.value_u64() {
                                Ok(res) => status.fit_end(res - 1),
                                Err(err) => return Err(err),
                            },
                            ConditionType::LE => match condition.value_u64() {
                                Ok(res) => status.fit_end(res),
                                Err(err) => return Err(err),
                            },
                            ConditionType::EQ => match condition.value_u64() {
                                Ok(res) => {
                                    status.fit_start(res);
                                    status.fit_end(res)
                                }
                                Err(err) => return Err(err),
                            },
                            _ => {}
                        },
                        IndexMold::I64 => match condition.cond {
                            ConditionType::GT => match condition.value_i64() {
                                Ok(res) => status.fit_start(i64_2_u64(res) + 1),
                                Err(err) => return Err(err),
                            },
                            ConditionType::GE => match condition.value_i64() {
                                Ok(res) => status.fit_start(i64_2_u64(res)),
                                Err(err) => return Err(err),
                            },
                            ConditionType::LT => match condition.value_i64() {
                                Ok(res) => status.fit_end(i64_2_u64(res) - 1),
                                Err(err) => return Err(err),
                            },
                            ConditionType::LE => match condition.value_i64() {
                                Ok(res) => status.fit_end(i64_2_u64(res)),
                                Err(err) => return Err(err),
                            },
                            ConditionType::EQ => match condition.value_i64() {
                                Ok(res) => {
                                    status.fit_start(i64_2_u64(res));
                                    status.fit_end(i64_2_u64(res))
                                }
                                Err(err) => return Err(err),
                            },
                            _ => {}
                        },
                        IndexMold::U32 => match condition.cond {
                            ConditionType::GT => match condition.value_u32() {
                                Ok(res) => status.fit_start(res as u64 + 1),
                                Err(err) => return Err(err),
                            },
                            ConditionType::GE => match condition.value_u32() {
                                Ok(res) => status.fit_start(res as u64),
                                Err(err) => return Err(err),
                            },
                            ConditionType::LT => match condition.value_u32() {
                                Ok(res) => status.fit_end(res as u64 - 1),
                                Err(err) => return Err(err),
                            },
                            ConditionType::LE => match condition.value_u32() {
                                Ok(res) => status.fit_end(res as u64),
                                Err(err) => return Err(err),
                            },
                            ConditionType::EQ => match condition.value_u32() {
                                Ok(res) => {
                                    status.fit_start(res as u64);
                                    status.fit_end(res as u64)
                                }
                                Err(err) => return Err(err),
                            },
                            _ => {}
                        },
                        IndexMold::I32 => match condition.cond {
                            ConditionType::GT => match condition.value_i32() {
                                Ok(res) => status.fit_start(i32_2_u64(res) + 1),
                                Err(err) => return Err(err),
                            },
                            ConditionType::GE => match condition.value_i32() {
                                Ok(res) => status.fit_start(i32_2_u64(res)),
                                Err(err) => return Err(err),
                            },
                            ConditionType::LT => match condition.value_i32() {
                                Ok(res) => status.fit_end(i32_2_u64(res) - 1),
                                Err(err) => return Err(err),
                            },
                            ConditionType::LE => match condition.value_i32() {
                                Ok(res) => status.fit_end(i32_2_u64(res)),
                                Err(err) => return Err(err),
                            },
                            ConditionType::EQ => match condition.value_i32() {
                                Ok(res) => {
                                    status.fit_start(i32_2_u64(res));
                                    status.fit_end(i32_2_u64(res))
                                }
                                Err(err) => return Err(err),
                            },
                            _ => {}
                        },
                        IndexMold::F64 => match condition.cond {
                            // ConditionType::GT => status.fit_start(condition.value_f64().to_bits()),
                            ConditionType::GT => match condition.value_f64() {
                                Ok(res) => status.fit_start(res.to_bits() + 1),
                                Err(err) => return Err(err),
                            },
                            ConditionType::GE => match condition.value_f64() {
                                Ok(res) => status.fit_start(res.to_bits()),
                                Err(err) => return Err(err),
                            },
                            ConditionType::LT => match condition.value_f64() {
                                Ok(res) => status.fit_end(res.to_bits() - 1),
                                Err(err) => return Err(err),
                            },
                            ConditionType::LE => match condition.value_f64() {
                                Ok(res) => status.fit_end(res.to_bits()),
                                Err(err) => return Err(err),
                            },
                            ConditionType::EQ => match condition.value_f64() {
                                Ok(res) => {
                                    status.fit_start(res.to_bits());
                                    status.fit_end(res.to_bits())
                                }
                                Err(err) => return Err(err),
                            },
                            _ => {}
                        },
                        _ => {
                            return Err(err_string(format!(
                                "{} can't parse except Number",
                                mold_str(idx_r.mold())
                            )));
                        }
                    },
                    _ => {}
                }
            }
        }
        Ok(status)
    }

    /// 通过param参数匹配获取索引
    fn index_param(&self, param: String) -> Option<Arc<RwLock<dyn TIndex>>> {
        for (_str, index) in self.indexes.clone().read().unwrap().iter() {
            let key_structure = index.clone().read().unwrap().key_structure();
            if key_structure.eq(&param) {
                return Some(index.clone());
            }
        }
        None
    }
}
