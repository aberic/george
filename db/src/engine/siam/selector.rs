use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use serde::Deserializer;
use serde_json::{Error, Value};

use comm::errors::entrances::{err_str, err_string, GeorgeResult};

use crate::engine::traits::TIndex;

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
    STRING,
    /// i64
    I64,
    /// u64
    U64,
    /// float
    F64,
    /// bool
    BOOL,
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
    /// 比较对象值
    fn value_str(&self) -> String {
        self.value.clone()
    }
    /// 比较对象值
    fn value_i64(&self) -> i64 {
        self.value.clone().parse().unwrap()
    }
    /// 比较对象值
    fn value_u64(&self) -> u64 {
        self.value.clone().parse().unwrap()
    }
    /// 比较对象值
    fn value_f64(&self) -> f64 {
        self.value.clone().parse().unwrap()
    }
    /// 比较对象值
    fn value_bool(&self) -> bool {
        self.value.clone().eq("1")
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

/// 经由`Selector`后的期望结果
#[derive(Debug)]
pub struct Expectation {
    /// 检索结果总条数
    pub count: u64,
    ///  使用到的索引名称，如果没用上则为空
    pub index_name: String,
    /// values 检索结果集合
    pub values: Vec<Vec<u8>>,
}

impl Constraint {
    /// 新建查询约束
    ///
    /// selector_json_bytes 选择器字节数组，自定义转换策略
    ///
    /// delete 是否删除检索结果
    pub fn new(constraint_json_bytes: Vec<u8>, delete: bool) -> GeorgeResult<Constraint> {
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
                                        support: ConditionSupport::BOOL,
                                        value: value_res,
                                    })
                                }
                                Value::String(ref val_str) => self.conditions.push(Condition {
                                    param: val_param.to_string(),
                                    cond,
                                    support: ConditionSupport::STRING,
                                    value: val_str.to_string(),
                                }),
                                Value::Number(ref val_num) => self.conditions.push(Condition {
                                    param: val_param.to_string(),
                                    cond,
                                    support: ConditionSupport::F64,
                                    value: v["Value"].as_f64().unwrap().to_string(),
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

    /// 条件 gt/lt/eq/ne 大于/小于/等于/不等
    fn valid_constraint(&self, value: Value, condition: Condition) -> bool {
        return match value[condition.param()] {
            Value::Bool(ref val) => match condition.support() {
                ConditionSupport::BOOL => val.eq(&condition.value_bool()),
                _ => false,
            },
            Value::String(ref val) => match condition.support() {
                ConditionSupport::STRING => match condition.cond() {
                    ConditionType::EQ => val.eq(&condition.value_str()),
                    _ => false,
                },
                _ => false,
            },
            Value::Number(ref val) => match condition.support() {
                ConditionSupport::I64 => match condition.cond() {
                    ConditionType::EQ => val.as_i64().unwrap().eq(&condition.value_i64()),
                    ConditionType::GT => val.as_i64().unwrap().gt(&condition.value_i64()),
                    ConditionType::GE => val.as_i64().unwrap().ge(&condition.value_i64()),
                    ConditionType::LT => val.as_i64().unwrap().lt(&condition.value_i64()),
                    ConditionType::LE => val.as_i64().unwrap().le(&condition.value_i64()),
                    ConditionType::NE => val.as_i64().unwrap().ne(&condition.value_i64()),
                },
                ConditionSupport::U64 => match condition.cond() {
                    ConditionType::EQ => val.as_u64().unwrap().eq(&condition.value_u64()),
                    ConditionType::GT => val.as_u64().unwrap().gt(&condition.value_u64()),
                    ConditionType::GE => val.as_u64().unwrap().ge(&condition.value_u64()),
                    ConditionType::LT => val.as_u64().unwrap().lt(&condition.value_u64()),
                    ConditionType::LE => val.as_u64().unwrap().le(&condition.value_u64()),
                    ConditionType::NE => val.as_u64().unwrap().ne(&condition.value_u64()),
                },
                ConditionSupport::F64 => match condition.cond() {
                    ConditionType::EQ => val.as_f64().unwrap().eq(&condition.value_f64()),
                    ConditionType::GT => val.as_f64().unwrap().gt(&condition.value_f64()),
                    ConditionType::GE => val.as_f64().unwrap().ge(&condition.value_f64()),
                    ConditionType::LT => val.as_f64().unwrap().lt(&condition.value_f64()),
                    ConditionType::LE => val.as_f64().unwrap().le(&condition.value_f64()),
                    ConditionType::NE => val.as_f64().unwrap().ne(&condition.value_f64()),
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

    /// 约束是否有效
    pub fn valid(&self, bytes: Vec<u8>) -> bool {
        let mut b = false;
        match String::from_utf8(bytes.clone()) {
            Ok(value_str) => {
                let res: Result<Value, Error> = serde_json::from_str(value_str.as_ref());
                match res {
                    Ok(v) => {
                        for condition in self.conditions.clone() {
                            if self.valid_constraint(v.clone(), condition) {
                                b = true
                            } else {
                                return false;
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        b
    }
}

impl Selector {
    /// 新建检索选择器
    ///
    /// selector_json_bytes 选择器字节数组，自定义转换策略
    ///
    /// indexes 索引集合
    ///
    /// delete 是否删除检索结果
    pub fn new(
        constraint_json_bytes: Vec<u8>,
        indexes: Arc<RwLock<HashMap<String, Arc<RwLock<dyn TIndex>>>>>,
        delete: bool,
    ) -> GeorgeResult<Selector> {
        let constraint = Constraint::new(constraint_json_bytes, delete)?;
        Ok(Selector {
            indexes,
            constraint,
        })
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
    pub fn run(&self) -> GeorgeResult<Expectation> {
        let (asc, idx) = self.index();
        match idx {
            Some(index) => index.read().unwrap().select(asc, self.constraint.clone()),
            None => Err(err_str("no index found!")),
        }
    }

    /// 获取最佳索引
    ///
    /// 检索顺序 sort -> conditions -> skip -> limit
    fn index(&self) -> (bool, Option<Arc<RwLock<dyn TIndex>>>) {
        match self.index_sort() {
            Some(index) => match self.constraint.sort.clone() {
                Some(s) => return (s.asc, Some(index)),
                None => {}
            },
            None => {}
        }
        match self.index_condition() {
            Some(index) => return (true, Some(index)),
            None => {}
        }
        match self.indexes.read().unwrap().iter().next() {
            Some(index) => (true, Some(index.1.clone())),
            None => (true, None),
        }
    }

    /// 通过sort所包含参数匹配索引
    fn index_sort(&self) -> Option<Arc<RwLock<dyn TIndex>>> {
        match self.constraint.sort.clone() {
            Some(sort) => self.index_param(sort.param),
            None => None,
        }
    }

    /// 通过condition所包含参数匹配索引
    fn index_condition(&self) -> Option<Arc<RwLock<dyn TIndex>>> {
        for condition in self.constraint.conditions.iter() {
            match self.index_param(condition.param.clone()) {
                Some(index) => return Some(index),
                None => {}
            }
        }
        None
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
