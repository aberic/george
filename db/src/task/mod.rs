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
mod database;
pub mod engine;
mod index;
mod ledger;
pub mod master;
mod master_test;
mod page;
mod rich;
mod seed;
mod view;

use crate::task::engine::traits::{Pigeonhole, TIndex};
use crate::utils::path::Paths;
use crate::utils::store::Metadata;
use crate::utils::writer::Filed;
use chrono::{Duration, Local, NaiveDateTime};
use comm::errors::entrances::GeorgeResult;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// 视图，类似表
#[derive(Debug, Clone)]
pub(crate) struct View {
    /// 数据库名称
    pub(crate) database_name: String,
    /// 名称
    pub(crate) name: String,
    /// 创建时间
    pub(crate) create_time: Duration,
    /// 文件信息
    pub(crate) metadata: Metadata,
    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 需要借助对象包裹，以便更新file，避免self为mut
    pub(crate) filer: Filed,
    /// 索引集合
    pub(crate) indexes: Arc<RwLock<HashMap<String, Arc<dyn TIndex>>>>,
    /// 当前归档版本信息
    pub(crate) pigeonhole: Pigeonhole,
}

/// 表数据元
#[derive(Debug, Clone)]
pub(crate) struct Form;

impl Form {
    /// 新建视图
    ///
    /// 具体传参参考如下定义：<p><p>
    ///
    /// ###Params
    ///
    /// mem 是否为内存视图
    pub(crate) fn new_view(database_name: String, name: String) -> GeorgeResult<View> {
        let now: NaiveDateTime = Local::now().naive_local();
        let create_time = Duration::nanoseconds(now.timestamp_nanos());
        let filepath = Paths::view_filepath(database_name.clone(), name.clone());
        let metadata = Metadata::view();
        let view = View {
            database_name: database_name.clone(),
            name,
            create_time,
            metadata,
            filer: Filed::create(filepath.clone())?,
            indexes: Default::default(),
            pigeonhole: Pigeonhole::create(0, filepath, create_time),
        };
        Ok(view)
    }

    /// 新建视图
    ///
    /// 具体传参参考如下定义：<p><p>
    ///
    /// ###Params
    ///
    /// mem 是否为内存视图
    pub(crate) fn mock_new_view(database_name: String, name: String) -> GeorgeResult<View> {
        let now: NaiveDateTime = Local::now().naive_local();
        let create_time = Duration::nanoseconds(now.timestamp_nanos());
        let filepath = Paths::view_filepath(database_name.clone(), name.clone());
        let metadata = Metadata::view();
        let view = View {
            database_name: database_name.clone(),
            name,
            create_time,
            metadata,
            filer: Filed::mock(filepath.clone())?,
            indexes: Default::default(),
            pigeonhole: Pigeonhole::create(0, filepath, create_time),
        };
        Ok(view)
    }
}
