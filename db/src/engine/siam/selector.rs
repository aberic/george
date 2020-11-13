use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use serde_json::{Error, Value};

use comm::errors::entrances::{err_str, err_string, GeorgeResult};

use crate::engine::traits::TIndex;

/// 条件查询
///
/// 查询过程中不满足条件的记录将被移除出结果集
#[derive(Debug, Clone)]
struct Condition {
    /// 参数名，新插入的数据将会尝试将数据对象转成json，并将json中的`param`作为参数使用
    param: String,
    /// 条件 gt/lt/eq/diff 大于/小于/等于/不等
    cond: String,
    /// 比较对象，支持int、string、float和bool
    value: *mut dyn Any,
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
    indexes: Arc<RwLock<HashMap<String, Arc<RwLock<dyn TIndex>>>>>,
    /// 查询约束
    constraint: Constraint,
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
                constraint.fit_sort(value.clone());
                constraint.fit_conditions(value.clone());
                Ok(constraint)
            }
            Err(err) => Err(err_string(err.to_string())),
        }
    }
    /// 解析`json value`并获取排序索引
    fn fit_sort(&mut self, value: Value) {
        if value["Sort"].is_object() {
            if value["Sort"]["Param"].is_string() {
                let mut sort = Sort {
                    param: value["Sort"]["Param"].as_str().unwrap().to_string(),
                    asc: false,
                };
                if !value["Sort"]["Asc"].is_null() {
                    sort.asc = value["Sort"]["Asc"].as_bool().unwrap();
                }
                self.sort = Some(sort);
            }
        }
    }

    /// 解析`json value`并获取条件索引
    fn fit_conditions(&mut self, value: Value) {
        if value["Conditions"].is_array() {
            for v in value["Conditions"].as_array().unwrap().iter() {
                if !v["Param"].is_string() || !v["Cond"].is_string() {
                    break;
                }
                if !v["Cond"].as_str().unwrap().eq("gt")
                    && !v["Cond"].as_str().unwrap().eq("lt")
                    && !v["Cond"].as_str().unwrap().eq("eq")
                    && !v["Cond"].as_str().unwrap().eq("diff")
                {
                    break;
                }
                let mut v_res = v["Value"].to_string();
                self.conditions.push(Condition {
                    param: v["Param"].as_str().unwrap().to_string(),
                    cond: v["Cond"].as_str().unwrap().to_string(),
                    value: &mut v_res,
                })
            }
        }
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
        let mut constraint = Constraint::new(constraint_json_bytes, delete)?;
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
            Some(index) => {
                if asc {
                    index.read().unwrap().select(true, self.constraint.clone())
                } else {
                    index.read().unwrap().select(false, self.constraint.clone())
                }
            }
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
}
