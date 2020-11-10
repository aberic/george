use std::sync::{Arc, RwLock};

use chrono::Duration;
use serde::export::fmt::Debug;

use comm::errors::entrances::GeorgeResult;

use crate::utils::comm::{Category, LevelType};

/// B+Tree索引叶子结点内防hash碰撞数组对象中对象特性
///
/// 搭配Index使用
///
/// thread::spawn需要一个实现Send的闭包。Arc<T>只实现发送，需要实现发送和同步以支持多线程调用
///
/// 发送和同步是自动特性。可以添加自动特征到一个dyn特征类型
///
/// 如果要写dyn Trait + Send + Sync到处都是，那么需要声明Send和Sync是Trait的Super Traits
pub trait TSeed: Send + Sync + Debug {
    /// 获取当前结果原始key信息
    fn key(&self) -> String;
    /// value最终存储在文件中的持续长度
    fn value(&self) -> Option<Vec<u8>>;
    /// 修改value值
    fn modify(&mut self, value: Vec<u8>);
    /// 存储操作
    fn save(&mut self, value: Vec<u8>) -> GeorgeResult<()>;
}

/// 索引通用特性，遵循此特性创建索引可以更方便的针对icdb进行扩展
///
/// 该特性包含了索引的基本方法，理论上都需要进行实现才能使用
pub trait TIndex: TDescription + Send + Sync + Debug {
    fn database_id(&self) -> String;
    fn view_id(&self) -> String;
    /// 获取索引唯一ID
    fn id(&self) -> String;
    /// 当前索引是否为主键
    fn is_primary(&self) -> bool;
    /// 索引名称，可以自定义；<p>
    /// siam::Index 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`key_structure`作为索引存入<p><p>
    fn key_structure(&self) -> String;
    /// 视图类型
    fn category(&self) -> Category;
    /// 索引容量
    fn level(&self) -> LevelType;
    /// 创建时间
    fn create_time(&self) -> Duration;
    /// 插入数据<p><p>
    ///
    /// ###Params
    ///
    /// key string
    ///
    /// force 如果存在原值，是否覆盖原结果<p><p>
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
    /// Seed value信息
    fn get(&self, key: String) -> GeorgeResult<Vec<u8>>;
}

/// 所有生成sr文件的结构都需要实现该特征，如database、view及index
pub trait TDescription {
    /// 生成文件描述
    fn description(&mut self) -> Vec<u8>;
    /// 通过文件描述恢复结构信息
    fn recover(&mut self, description: Vec<u8>) -> GeorgeResult<()>;
}
