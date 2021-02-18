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

use std::fmt::Debug;
use std::sync::{Arc, RwLock};

use chrono::{Duration, Local, NaiveDateTime};

use comm::errors::entrances::GeorgeResult;

use crate::task::engine::memory::node::Node;
use crate::task::engine::traits::{TIndex, TSeed};
use crate::utils::enums::{EngineType, IndexMold};
use crate::utils::store::Metadata;

/// Siam索引
///
/// 5位key及16位md5后key及5位起始seek和4位持续seek
#[derive(Debug)]
pub struct Index {
    /// 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入
    name: String,
    /// 文件信息
    metadata: Metadata,
    /// 结点
    root: Arc<Node>,
    /// 创建时间
    create_time: Duration,
}

/// 新建索引
///
/// 该索引需要定义ID，此外索引所表达的字段组成内容也是必须的，并通过primary判断索引类型，具体传参参考如下定义：<p><p>
///
/// ###Params
///
/// index_name 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入
///
/// primary 是否主键
fn new_index(name: String, metadata: Metadata) -> GeorgeResult<Index> {
    let now: NaiveDateTime = Local::now().naive_local();
    let create_time = Duration::nanoseconds(now.timestamp_nanos());
    return Ok(Index {
        name,
        root: Node::create_root(),
        metadata,
        create_time,
    });
}

/// 封装方法函数
impl Index {
    /// 新建索引
    ///
    /// 该索引需要定义ID，此外索引所表达的字段组成内容也是必须的，并通过primary判断索引类型，具体传参参考如下定义：<p><p>
    pub(crate) fn create(name: String) -> GeorgeResult<Arc<RwLock<dyn TIndex>>> {
        let index = new_index(name, Metadata::index(EngineType::Memory)?)?;
        Ok(Arc::new(RwLock::new(index)))
    }
}

/// 封装方法函数
impl TIndex for Index {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn mold(&self) -> IndexMold {
        IndexMold::String
    }
    fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }
    fn metadata_bytes(&self) -> Vec<u8> {
        self.metadata.bytes()
    }
    fn create_time(&self) -> Duration {
        self.create_time.clone()
    }
    fn modify(&mut self, _dn: String, _vn: String) {
        unimplemented!()
    }
    fn put(&self, key: String, seed: Arc<RwLock<dyn TSeed>>) -> GeorgeResult<()> {
        self.root.put(key, seed)
    }
    fn get(&self, key: String) -> GeorgeResult<Vec<u8>> {
        self.root.get(key)
    }
    fn del(&self, key: String) -> GeorgeResult<()> {
        self.root.del(key)
    }
}
