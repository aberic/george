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

use crate::task::engine::traits::{TNode, TSeed};
use crate::task::rich::Condition;
use crate::task::seed::IndexPolicy;
use crate::task::view::View;
use crate::utils::comm::level_distance_64;
use crate::utils::enums::IndexType;
use crate::utils::path::{index_path, node_filepath};
use comm::errors::entrances::GeorgeResult;
use comm::io::file::{Filer, FilerReader};
use comm::strings::{StringHandler, Strings};
use std::ops::Add;

/// 索引B+Tree结点结构
///
/// 包含了索引的根结点、子结点以及叶子结点
///
/// 叶子结点中才会存在Link，其余结点Link为None
#[derive(Debug, Clone)]
pub(crate) struct Node {
    index_name: String,
}

impl Node {
    /// 新建根结点
    ///
    /// 该结点没有Links，也没有preNode，是B+Tree的创世结点
    pub fn create(index_name: String) -> Arc<RwLock<Self>> {
        return Arc::new(RwLock::new(Node { index_name }));
    }
    /// 恢复根结点
    pub fn recovery(index_name: String) -> Arc<RwLock<Self>> {
        return Arc::new(RwLock::new(Node { index_name }));
    }
    fn index_name(&self) -> String {
        self.index_name.clone()
    }
}

/// 封装方法函数
impl TNode for Node {
    fn modify(&mut self) -> GeorgeResult<()> {
        unimplemented!()
    }
    /// 插入数据<p><p>
    ///
    /// ###Params
    ///
    /// hash_key u64
    ///
    /// ###Return
    ///
    /// EngineResult<()>
    fn put(&self, hash_key: u64, seed: Arc<RwLock<dyn TSeed>>, force: bool) -> GeorgeResult<()> {
        unimplemented!()
    }
    fn get(&self, key: String, hash_key: u64) -> GeorgeResult<Vec<u8>> {
        unimplemented!()
    }
    fn del(&self, key: String, hash_key: u64) -> GeorgeResult<()> {
        unimplemented!()
    }
    fn select(
        &self,
        _left: bool,
        _start: u64,
        _end: u64,
        _skip: u64,
        _limit: u64,
        _delete: bool,
        _conditions: Vec<Condition>,
    ) -> GeorgeResult<(u64, u64, Vec<Vec<u8>>)> {
        unimplemented!()
    }
}
