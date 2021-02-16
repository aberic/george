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
use serde::export::fmt::Debug;

use comm::errors::entrances::GeorgeResult;

use crate::task::view::View;
use crate::utils::enums::IndexMold;
use crate::utils::store::Metadata;

/// 索引通用特性，遵循此特性创建索引可以更方便的针对icdb进行扩展
///
/// 该特性包含了索引的基本方法，理论上都需要进行实现才能使用
pub(crate) trait TIndex: Send + Sync + Debug {
    /// 索引名称，可以自定义；<p>
    /// siam::Index 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入<p><p>
    fn name(&self) -> String;
    /// 索引值类型
    fn mold(&self) -> IndexMold;
    /// 存储引擎类型
    fn metadata(&self) -> Metadata;
    /// 创建时间
    fn create_time(&self) -> Duration;
    /// 插入数据<p><p>
    ///
    /// ###Params
    ///
    /// key string
    ///
    /// ###Return
    ///
    /// EngineResult<()>
    fn put(&self, key: String, seed: Arc<RwLock<dyn TSeed>>) -> GeorgeResult<()>;
    /// 获取数据，返回存储对象<p><p>
    ///
    /// ###Params
    ///
    /// key strings
    ///
    /// ###Return
    ///
    /// Seed value信息
    fn get(&self, key: String) -> GeorgeResult<Vec<u8>>;
    /// 删除数据<p><p>
    ///
    /// ###Params
    ///
    /// key strings
    ///
    /// ###Return
    ///
    /// Seed value信息
    fn del(&self, key: String) -> GeorgeResult<()>;
}

/// 结点通用特性，遵循此特性创建结点可以更方便的针对db进行扩展
///
/// 该特性包含了结点的基本方法，理论上都需要进行实现才能使用
pub(crate) trait TNode: Send + Sync + Debug {
    /// 存储结点所属各子结点坐标顺序字符串
    ///
    /// 如果子项是node集合，在node集合中每一个node的默认字节长度是8，数量是256，即一次性读取2048个字节
    ///
    /// 如果子项是seed集合，在seed集合中每一个seed的默认字符长度是6，当前叶子node会存储叶子中首个出现hash碰撞的
    /// seed起始坐标，每一个seed都会存储出现hash碰撞的下一seed起始坐标
    fn node_bytes(&self) -> Arc<RwLock<Vec<u8>>>;
    /// 插入数据<p><p>
    ///
    /// ###Params
    ///
    /// key string
    ///
    /// force 如果存在原值，是否覆盖原结果
    ///
    /// description_len 描述长度
    ///
    /// ###Return
    ///
    /// EngineResult<()>
    fn put(
        &self,
        original_key: String,
        database_name: String,
        view_name: String,
        index_name: String,
        key: u64,
        seed: Arc<RwLock<dyn TSeed>>,
    ) -> GeorgeResult<()>;
    /// 获取数据，返回存储对象<p><p>
    ///
    /// ###Params
    ///
    /// key string
    ///
    /// ###Return
    ///
    /// Seed value信息
    fn get(
        &self,
        database_name: String,
        view_name: String,
        index_name: String,
        key: u64,
    ) -> GeorgeResult<Vec<u8>>;
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
    /// 值是否为空
    fn is_none(&self) -> bool;
    /// 修改value值
    fn modify(&mut self, value: Vec<u8>) -> GeorgeResult<()>;
    /// 存储操作
    ///
    /// view View 视图<p><p>
    ///
    /// force 是否强制覆盖
    fn save(&mut self, database_name: String, view: View, force: bool) -> GeorgeResult<()>;
    /// 删除操作
    ///
    /// view View 视图
    fn remove(&mut self, database_name: String, view: View) -> GeorgeResult<()>;
}
