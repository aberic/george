/*
 * Copyright (c) 2020. Aberic - All Rights Reserved.
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
use serde::export::fmt::Debug;

use comm::errors::entrances::GeorgeResult;

use crate::engine::siam::selector::{Constraint, Expectation};
use crate::utils::comm::{Category, IndexMold};

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
    /// 删除操作
    fn remove(&mut self) -> GeorgeResult<()>;
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
    /// siam::Index 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入<p><p>
    fn name(&self) -> String;
    /// 视图类型
    fn category(&self) -> Category;
    /// 索引值类型
    fn mold(&self) -> IndexMold;
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

/// 所有生成sr文件的结构都需要实现该特征，如database、view及index
pub trait TDescription {
    /// 生成文件描述
    fn description(&mut self) -> Vec<u8>;
    /// 通过文件描述恢复结构信息
    fn recover(&mut self, description: Vec<u8>) -> GeorgeResult<()>;
}
