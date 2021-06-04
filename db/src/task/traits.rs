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
use std::sync::{Arc, RwLock};

use serde::__private::fmt::Debug;

use comm::errors::GeorgeResult;
use comm::Time;

use crate::task::rich::Expectation;
use crate::task::{Database, Page};
use crate::utils::enums::{Engine, KeyType};

pub trait TMaster {
    /// 缓存页集合
    fn page_map(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<Page>>>>>;

    /// 库集合
    fn database_map(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<Database>>>>>;

    /// 创建时间
    fn create_time(&self) -> Time;

    /// 创建缓存页
    ///
    /// ###Params
    ///
    /// * name 缓存页名称
    /// * comment 缓存页描述
    /// * size 可使用内存大小(单位：Mb)，为0则不限
    /// * period 默认有效期(单位：秒)，如为0，则默认为300
    fn create_page(
        &self,
        name: String,
        comment: String,
        size: u64,
        period: u32,
    ) -> GeorgeResult<()>;

    /// 删除缓存页
    fn remove_page(&self, page_name: String) -> GeorgeResult<()>;

    /// 修改缓存页
    fn modify_page(&self, page_name: String, page_new_name: String) -> GeorgeResult<()>;

    /// 根据缓存页name获取库
    fn page(&self, page_name: String) -> GeorgeResult<Arc<RwLock<Page>>>;

    /// 获取默认缓存页
    fn page_default(&self) -> GeorgeResult<Arc<RwLock<Page>>>;

    /// 创建数据库
    fn create_database(&self, database_name: String, database_comment: String) -> GeorgeResult<()>;

    /// 删除数据库
    fn remove_database(&self, database_name: String) -> GeorgeResult<()>;

    /// 修改数据库
    fn modify_database(
        &self,
        database_name: String,
        database_new_name: String,
        database_comment: String,
    ) -> GeorgeResult<()>;

    /// 根据库name获取库
    fn database(&self, database_name: String) -> GeorgeResult<Arc<RwLock<Database>>>;

    /// 创建视图
    ///
    /// mem 是否为内存视图
    fn create_view(
        &self,
        database_name: String,
        view_name: String,
        comment: String,
        with_sequence: bool,
    ) -> GeorgeResult<()>;

    /// 修改视图
    fn modify_view(
        &self,
        database_name: String,
        view_name: String,
        view_new_name: String,
        comment: String,
    ) -> GeorgeResult<()>;

    /// 整理归档
    ///
    /// archive_file_path 归档路径
    fn archive_view(
        &self,
        database_name: String,
        view_name: String,
        archive_file_path: String,
    ) -> GeorgeResult<()>;

    /// 指定归档版本信息
    ///
    /// version 版本号
    ///
    /// #return
    /// * filepath 当前归档版本文件所处路径
    /// * create_time 归档时间
    fn view_record(
        &self,
        database_name: String,
        view_name: String,
        version: u16,
    ) -> GeorgeResult<(String, Time)>;

    /// 在指定库及视图中创建索引
    ///
    /// 该索引需要定义ID，此外索引所表达的字段组成内容也是必须的，并通过primary判断索引类型，具体传参参考如下定义：<p><p>
    ///
    /// ###Params
    /// * database_name 数据库名
    /// * view_name 视图名
    /// * index_name 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入
    /// * index_type 存储引擎类型
    /// * key_type 索引值类型
    /// * primary 是否主键，主键也是唯一索引，即默认列表依赖索引
    /// * unique 是否唯一索引
    /// * null 是否允许为空
    fn create_index(
        &self,
        database_name: String,
        view_name: String,
        index_name: String,
        index_type: Engine,
        key_type: KeyType,
        primary: bool,
        unique: bool,
        null: bool,
    ) -> GeorgeResult<()>;

    /// 插入数据，如果存在则返回已存在<p><p>
    ///
    /// ###Params
    ///
    /// view_name 视图名称<p><p>
    ///
    /// key string
    ///
    /// value 当前结果value信息<p><p>
    ///
    /// ###Return
    ///
    /// IndexResult<()>
    fn put_disk(
        &self,
        database_name: String,
        view_name: String,
        key: String,
        value: Vec<u8>,
    ) -> GeorgeResult<()>;

    /// 插入数据，无论存在与否都会插入或更新数据<p><p>
    ///
    /// ###Params
    ///
    /// view_name 视图名称<p><p>
    ///
    /// key string
    ///
    /// value 当前结果value信息<p><p>
    ///
    /// ###Return
    ///
    /// IndexResult<()>
    fn set_disk(
        &self,
        database_name: String,
        view_name: String,
        key: String,
        value: Vec<u8>,
    ) -> GeorgeResult<()>;

    /// 获取数据，返回存储对象<p><p>
    ///
    /// ###Params
    ///
    /// view_name 视图名称
    ///
    /// key string
    ///
    /// ###Return
    ///
    /// Seed value信息
    fn get_disk(
        &self,
        database_name: String,
        view_name: String,
        key: String,
    ) -> GeorgeResult<Vec<u8>>;

    /// 获取数据，返回存储对象<p><p>
    ///
    /// ###Params
    ///
    /// view_name 视图名称
    ///
    /// index_name 索引名称
    ///
    /// key string
    ///
    /// ###Return
    ///
    /// Seed value信息
    fn get_disk_by_index(
        &self,
        database_name: String,
        view_name: String,
        index_name: String,
        key: String,
    ) -> GeorgeResult<Vec<u8>>;

    /// 删除数据<p><p>
    ///
    /// ###Params
    ///
    /// view_name 视图名称<p><p>
    ///
    /// key string
    ///
    /// ###Return
    ///
    /// IndexResult<()>
    fn remove_disk(
        &self,
        database_name: String,
        view_name: String,
        key: String,
    ) -> GeorgeResult<()>;

    /// 条件检索
    ///
    /// selector_json_bytes 选择器字节数组，自定义转换策略
    fn select_disk(
        &self,
        database_name: String,
        view_name: String,
        constraint_json_bytes: Vec<u8>,
    ) -> GeorgeResult<Expectation>;

    /// 条件删除
    ///
    /// selector_json_bytes 选择器字节数组，自定义转换策略
    fn delete_disk(
        &self,
        database_name: String,
        view_name: String,
        constraint_json_bytes: Vec<u8>,
    ) -> GeorgeResult<Expectation>;

    /// 插入数据，如果存在则返回已存在<p><p>
    ///
    /// ###Params
    ///
    /// key string
    ///
    /// value 当前结果value信息<p><p>
    ///
    /// ###Return
    ///
    /// IndexResult<()>
    fn put_memory_default(&self, key: String, value: Vec<u8>) -> GeorgeResult<()>;

    /// 插入数据，无论存在与否都会插入或更新数据<p><p>
    ///
    /// ###Params
    ///
    /// view_name 视图名称<p><p>
    ///
    /// key string
    ///
    /// value 当前结果value信息<p><p>
    ///
    /// ###Return
    ///
    /// IndexResult<()>
    fn set_memory_default(&self, key: String, value: Vec<u8>) -> GeorgeResult<()>;

    /// 获取数据，返回存储对象<p><p>
    ///
    /// ###Params
    ///
    /// view_name 视图名称
    ///
    /// key string
    ///
    /// ###Return
    ///
    /// Seed value信息
    fn get_memory_default(&self, key: String) -> GeorgeResult<Vec<u8>>;

    /// 删除数据<p><p>
    ///
    /// ###Params
    ///
    /// view_name 视图名称
    ///
    /// key string
    ///
    /// ###Return
    ///
    /// Seed value信息
    fn remove_memory_default(&self, key: String) -> GeorgeResult<()>;

    /// 插入数据，如果存在则返回已存在<p><p>
    ///
    /// ###Params
    ///
    /// key string
    ///
    /// value 当前结果value信息<p><p>
    ///
    /// ###Return
    ///
    /// IndexResult<()>
    fn put_memory(&self, page_name: String, key: String, value: Vec<u8>) -> GeorgeResult<()>;

    /// 插入数据，无论存在与否都会插入或更新数据<p><p>
    ///
    /// ###Params
    ///
    /// view_name 视图名称<p><p>
    ///
    /// key string
    ///
    /// value 当前结果value信息<p><p>
    ///
    /// ###Return
    ///
    /// IndexResult<()>
    fn set_memory(&self, page_name: String, key: String, value: Vec<u8>) -> GeorgeResult<()>;

    /// 获取数据，返回存储对象<p><p>
    ///
    /// ###Params
    ///
    /// view_name 视图名称
    ///
    /// key string
    ///
    /// ###Return
    ///
    /// Seed value信息
    fn get_memory(&self, page_name: String, key: String) -> GeorgeResult<Vec<u8>>;

    /// 删除数据<p><p>
    ///
    /// ###Params
    ///
    /// view_name 视图名称
    ///
    /// key string
    ///
    /// ###Return
    ///
    /// Seed value信息
    fn remove_memory(&self, page_name: String, key: String) -> GeorgeResult<()>;
}

/// 表通用特性，遵循此特性创建索引可以更方便的针对进行扩展
///
/// 该特性包含了索引的基本方法，理论上都需要进行实现才能使用
pub(crate) trait TForm: Send + Sync + Debug {
    /// 数据库名称
    fn database_name(&self) -> String;

    /// 名称
    fn name(&self) -> String;

    /// 介绍
    fn comment(&self) -> String;

    /// 组装写入视图的内容，即持续长度+该长度的原文内容
    ///
    /// 将数据存入view，返回数据在view中的起始偏移量坐标
    ///
    /// #param
    /// * value 待写入view中字节数组
    /// #return
    /// * view_info_index view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节)
    fn write_content(&self, value: Vec<u8>) -> GeorgeResult<Vec<u8>>;

    /// 读取已组装写入视图的内容，根据view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节)
    ///
    /// * version view版本号
    /// * data_len view数据持续长度
    /// * seek view数据偏移量
    fn read_content(&self, version: u16, data_len: u32, seek: u64) -> GeorgeResult<Vec<u8>>;

    /// 读取已组装写入视图的内容，根据view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节)
    ///
    /// * view_info_index 数据索引字节数组
    fn read_content_by_info(&self, view_info_index: Vec<u8>) -> GeorgeResult<Vec<u8>>;

    fn rm(&self, key: String, value: Vec<u8>) -> GeorgeResult<()>;
}
