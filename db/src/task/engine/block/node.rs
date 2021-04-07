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
use crate::utils::enums::KeyType;
use comm::errors::entrances::GeorgeResult;

/// 索引B+Tree结点结构
///
/// 包含了索引的根结点、子结点以及叶子结点
///
/// 叶子结点中才会存在Link，其余结点Link为None
#[derive(Debug, Clone)]
pub(crate) struct Node {
    index_name: String,
    key_type: KeyType,
}

impl Node {
    /// 新建根结点
    ///
    /// 该结点没有Links，也没有preNode，是B+Tree的创世结点
    pub fn create(index_name: String, key_type: KeyType) -> Arc<Self> {
        return Arc::new(Node {
            index_name,
            key_type,
        });
    }
    /// 恢复根结点
    pub fn recovery(index_name: String, key_type: KeyType) -> Arc<Self> {
        return Arc::new(Node {
            index_name,
            key_type,
        });
    }
    fn index_name(&self) -> String {
        self.index_name.clone()
    }
    fn key_type(&self) -> KeyType {
        self.key_type.clone()
    }
}

/// 封装方法函数
impl TNode for Node {
    /// 插入数据<p><p>
    ///
    /// ###Params
    ///
    /// hash_key u64
    ///
    /// ###Return
    ///
    /// EngineResult<()>
    fn put(&self, _key: String, _seed: Arc<RwLock<dyn TSeed>>, _force: bool) -> GeorgeResult<()> {
        unimplemented!()
    }
    fn get(&self, _key: String) -> GeorgeResult<Vec<u8>> {
        unimplemented!()
    }
    fn del(&self, _key: String, _seed: Arc<RwLock<dyn TSeed>>) -> GeorgeResult<()> {
        unimplemented!()
    }
    fn select(
        &self,
        _left: bool,
        _start_bytes: Vec<u8>,
        _end_bytes: Vec<u8>,
        _skip: u64,
        _limit: u64,
        _delete: bool,
        _conditions: Vec<Condition>,
    ) -> GeorgeResult<(u64, u64, Vec<Vec<u8>>)> {
        unimplemented!()
    }
}
