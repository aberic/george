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

use crate::task::engine::memory::Node;
use crate::task::engine::traits::{Pigeonhole, TForm, TIndex, TNode};
use crate::task::engine::DataReal;
use crate::task::seed::IndexPolicy;
use crate::utils::enums::{IndexType, KeyType};
use crate::utils::store::Metadata;
use crate::utils::writer::Filed;
use chrono::Duration;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// 数据库
pub(super) struct Master {
    /// 默认缓存页名称
    default_page_name: String,
    /// 缓存页集合
    pages: Arc<RwLock<HashMap<String, Arc<RwLock<Page>>>>>,
    /// 库集合
    databases: Arc<RwLock<HashMap<String, Arc<RwLock<Database>>>>>,
    /// 创建时间
    create_time: Duration,
}

#[derive(Debug, Clone)]
pub(crate) struct Database {
    /// 名称
    name: String,
    /// 描述
    comment: String,
    /// 创建时间
    create_time: Duration,
    /// 文件信息
    metadata: Metadata,
    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 需要借助对象包裹，以便更新file，避免self为mut
    filer: Filed,
    /// 视图集合
    views: Arc<RwLock<HashMap<String, Arc<RwLock<View>>>>>,
}

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

#[derive(Debug, Clone)]
pub(crate) struct Page {
    /// 名称
    name: String,
    /// 描述
    comment: String,
    /// 可使用内存大小(单位：Mb，0：不限制大小)
    size: u64,
    /// 默认有效期(单位：秒)，如无设置，默认维300(0：永久有效)
    period: u32,
    /// 创建时间
    create_time: Duration,
    /// 文件信息
    metadata: Metadata,
    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 需要借助对象包裹，以便更新file，避免self为mut
    filer: Filed,
    /// 默认缓存页
    node: Arc<RwLock<Node>>,
}

/// 账本
#[derive(Debug, Clone)]
pub(crate) struct Ledger {
    /// 数据库名称
    pub(crate) database_name: String,
    /// 名称
    pub(crate) name: String,
    /// 创建时间
    pub(crate) create_time: Duration,
    /// 文件信息
    pub(crate) metadata: Metadata,
    pub(crate) filepath: String,
    pub(crate) filepath_light: String,
    pub(crate) filepath_merkle_light: String,
    /// 区块全数据记录文件
    ///
    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 需要借助对象包裹，以便更新file，避免self为mut
    pub(crate) filer: Filed,
    /// 区块Header数据记录文件
    ///
    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 需要借助对象包裹，以便更新file，避免self为mut
    pub(crate) filer_light: Filed,
    /// 区块Header数据以merkle形式进行存储的记录文件
    ///
    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 需要借助对象包裹，以便更新file，避免self为mut
    pub(crate) filer_merkle_light: Filed,
    /// 索引集合
    pub(crate) indexes: Arc<RwLock<HashMap<String, Arc<dyn TIndex>>>>,
}

/// Siam索引
///
/// 5位key及16位md5后key及5位起始seek和4位持续seek
#[derive(Debug)]
pub(crate) struct Index {
    form: Arc<RwLock<dyn TForm>>,
    /// 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入
    name: String,
    /// 存储引擎类型
    index_type: IndexType,
    /// 是否主键，主键也是唯一索引，即默认列表依赖索引
    primary: bool,
    /// 是否唯一索引
    unique: bool,
    /// 是否允许为空
    null: bool,
    /// 索引值类型
    key_type: KeyType,
    /// 结点
    root: Arc<dyn TNode>,
    /// 文件信息
    metadata: Metadata,
    /// 创建时间
    create_time: Duration,
    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 需要借助对象包裹，以便更新file，避免self为mut
    filer: Filed,
}

/// B+Tree索引叶子结点内防hash碰撞数组结构中单体结构
///
/// 搭配Index使用
///
/// 叶子节点下真实存储数据的集合单体结构
#[derive(Debug)]
pub(crate) struct Seed {
    real: DataReal,
    /// 除主键索引外的其它索引操作策略集合
    policies: Vec<IndexPolicy>,
    form: Arc<dyn TForm>,
}
