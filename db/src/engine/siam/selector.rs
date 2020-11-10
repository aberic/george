use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use serde_json::{Error, Value};

use comm::errors::entrances::{err_string, GeorgeResult};

use crate::engine::traits::TIndex;

/// 条件查询
///
/// 查询过程中不满足条件的记录将被移除出结果集
#[derive(Debug)]
struct Condition {
    /// 参数名，新插入的数据将会尝试将数据对象转成json，并将json中的`param`作为参数使用
    param: String,
    /// 条件 gt/lt/eq/diff 大于/小于/等于/不等
    cond: String,
    /// 比较对象，支持int、string、float和bool
    value: *mut dyn Any,
}

/// 排序方式
#[derive(Debug)]
struct Sort {
    /// 参数名，新插入的数据将会尝试将数据对象转成json，并将json中的`param`作为参数使用
    param: String,
    /// 是否升序
    asc: bool,
}

/// 检索选择器
///
/// 检索顺序 sort -> conditions -> skip -> limit
#[derive(Debug)]
pub struct Selector {
    /// 索引集合
    indexes: Arc<RwLock<HashMap<String, Arc<RwLock<dyn TIndex>>>>>,
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

impl Selector {
    /// 新建检索选择器
    ///
    /// selector_json_bytes 选择器字节数组，自定义转换策略
    ///
    /// indexes 索引集合
    ///
    /// delete 是否删除检索结果
    pub fn new(
        selector_json_bytes: Vec<u8>,
        indexes: Arc<RwLock<HashMap<String, Arc<RwLock<dyn TIndex>>>>>,
        delete: bool,
    ) -> GeorgeResult<Selector> {
        let mut selector = Selector {
            indexes,
            conditions: vec![],
            skip: 0,
            sort: None,
            limit: 1000,
            delete,
        };
        let result: Result<Value, Error> = serde_json::from_slice(selector_json_bytes.as_slice());
        match result {
            Ok(value) => {
                if value["Limit"].is_u64() {
                    selector.limit = value["Limit"].as_u64().unwrap();
                }
                if value["Skip"].is_u64() {
                    selector.skip = value["Skip"].as_u64().unwrap();
                }
                match Selector::sort(value.clone()) {
                    Some(s) => selector.sort = Some(s),
                    None => {}
                }
                selector.conditions = Selector::conditions(value);
                Ok(selector)
            }
            Err(err) => Err(err_string(err.to_string())),
        }
    }

    fn sort(value: Value) -> Option<Sort> {
        if value["Sort"].is_object() {
            if value["Sort"]["Param"].is_string() {
                let mut sort = Sort {
                    param: value["Sort"]["Param"].as_str().unwrap().to_string(),
                    asc: false,
                };
                if !value["Sort"]["Asc"].is_null() {
                    sort.asc = value["Sort"]["Asc"].as_bool().unwrap();
                }
                return Some(sort);
            }
        }
        None
    }

    fn conditions(value: Value) -> Vec<Condition> {
        let mut conditions: Vec<Condition> = vec![];
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
                conditions.push(Condition {
                    param: v["Param"].as_str().unwrap().to_string(),
                    cond: v["Cond"].as_str().unwrap().to_string(),
                    value: &mut v_res,
                })
            }
        }
        conditions
    }
}
