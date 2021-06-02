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

use once_cell::sync::Lazy;

use comm::pool::ThreadPool;
use comm::{Env, Time};
use ge::Ge;

use crate::task::engine::memory::Node;
use crate::task::engine::traits::{Pigeonhole, TForm, TIndex, TNode};
use crate::task::engine::DataReal;
use crate::task::master::init_log;
use crate::task::seed::IndexPolicy;
use crate::utils::comm::GEORGE_DB_CONFIG;
use crate::utils::deploy::{init_config, GLOBAL_CONFIG};
use crate::utils::enums::{Engine, KeyType};

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

pub static GLOBAL_MASTER: Lazy<Arc<Master>> = Lazy::new(|| {
    init_config(Env::get(GEORGE_DB_CONFIG, "src/examples/conf.yaml"));
    init_log();
    log::info!("config & log init success!");
    GLOBAL_THREAD_POOL.init();
    log::info!("thread pool init success!");
    Master::generate()
});

pub(super) static GLOBAL_THREAD_POOL: Lazy<ThreadPool> = Lazy::new(|| {
    let config = GLOBAL_CONFIG.read().unwrap();
    let worker_threads = config.thread_count;
    log::info!("thread pool intent to start {} threads", worker_threads);
    ThreadPool::new(worker_threads).expect("thread pool new failed!")
});

/// 数据库
pub struct Master {
    /// 默认缓存页名称
    default_page_name: String,
    /// 缓存页集合
    pages: Arc<RwLock<HashMap<String, Arc<RwLock<Page>>>>>,
    /// 库集合
    databases: Arc<RwLock<HashMap<String, Arc<RwLock<Database>>>>>,
    /// 创建时间
    create_time: Time,
}

#[derive(Debug, Clone)]
pub(crate) struct Database {
    /// 名称
    name: String,
    /// 描述
    comment: String,
    /// 创建时间
    create_time: Time,
    /// ge文件对象
    ge: Ge,
    /// 视图集合
    views: Arc<RwLock<HashMap<String, Arc<RwLock<View>>>>>,
}

/// 视图，类似表
#[derive(Debug, Clone)]
pub(crate) struct View {
    /// 数据库名称
    database_name: String,
    /// 名称
    name: String,
    /// 描述
    comment: String,
    /// 创建时间
    create_time: Time,
    /// ge文件对象
    ge: Ge,
    /// 索引集合
    indexes: Arc<RwLock<HashMap<String, Arc<dyn TIndex>>>>,
    /// 当前归档版本信息
    pigeonhole: Pigeonhole,
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
    create_time: Time,
    /// ge文件对象
    ge: Ge,
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
    pub(crate) create_time: Time,
    /// 区块全数据记录文件
    ///
    /// 需要借助对象包裹，以便更新file，避免self为mut
    pub(crate) ge: Ge,
    /// 区块Header数据记录文件
    ///
    /// 需要借助对象包裹，以便更新file，避免self为mut
    pub(crate) ge_light: Ge,
    /// 区块Header数据以merkle形式进行存储的记录文件
    ///
    /// 需要借助对象包裹，以便更新file，避免self为mut
    pub(crate) ge_merkle_light: Ge,
    /// 区块高度存储索引，根据块高查询区块
    pub(crate) index_block_height: Arc<dyn TIndex>,
    /// 区块hash存储索引，根据块hash查询区块
    pub(crate) index_block_hash: Arc<dyn TIndex>,
    /// 交易hash存储索引，根据交易hash查询区块、查询交易
    pub(crate) index_tx_hash: Arc<dyn TIndex>,
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
    engine: Engine,
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
    /// 创建时间
    create_time: Time,
    /// ge文件对象
    ge: Ge,
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
