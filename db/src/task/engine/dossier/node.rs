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

use crate::task::engine::traits::TSeed;
use comm::bytes::create_empty_bytes;
use comm::cryptos::hash::hashcode32_enhance;
use comm::errors::entrances::GeorgeResult;

/// 索引B+Tree结点结构
///
/// 包含了索引的根结点、子结点以及叶子结点
///
/// 叶子结点中才会存在Link，其余结点Link为None
#[derive(Debug, Clone)]
pub(crate) struct Node {
    /// 存储结点所属各子结点坐标顺序字符串
    ///
    /// 如果子项是32位node集合，在node集合中每一个node的默认字节长度是8，数量是256，即一次性读取2048个字节
    ///
    /// 如果子项是seed集合，在seed集合中每一个seed的默认字符长度是6，当前叶子node会存储叶子中首个出现hash碰撞的
    /// seed起始坐标，每一个seed都会存储出现hash碰撞的下一seed起始坐标
    node_bytes: Arc<RwLock<Vec<u8>>>,
}

fn create_empty() -> Node {
    return Node {
        node_bytes: Arc::new(Default::default()),
    };
}

impl Node {
    /// 新建根结点
    ///
    /// 该结点没有Links，也没有preNode，是B+Tree的创世结点
    pub fn create_root() -> Arc<RwLock<Self>> {
        return Arc::new(RwLock::new(Node {
            node_bytes: Arc::new(RwLock::new(create_empty_bytes(2048))),
        }));
    }
    /// 恢复根结点
    pub fn recovery_root(v8s: Vec<u8>) -> Arc<RwLock<Self>> {
        return Arc::new(RwLock::new(Node {
            node_bytes: Arc::new(RwLock::new(v8s)),
        }));
    }
}

/// 封装方法函数
impl Node {
    /// 存储结点所属各子结点坐标顺序字符串
    ///
    /// 如果子项是node集合，在node集合中每一个node的默认字节长度是8，数量是65536，即一次性读取524288个字节
    pub(crate) fn node_bytes(&self) -> Arc<RwLock<Vec<u8>>> {
        self.node_bytes.clone()
    }
    /// 插入数据<p><p>
    ///
    /// ###Params
    ///
    /// key u64
    ///
    /// ###Return
    ///
    /// EngineResult<()>
    pub(crate) fn put(&self, key: u64, seed: Arc<RwLock<dyn TSeed>>) -> GeorgeResult<()> {
        let node_bytes = self.node_bytes().read().unwrap().to_vec();
        self.put_in_node(node_bytes, 1, key, seed, true)
    }
    pub(crate) fn get(&self, key: String) -> GeorgeResult<Vec<u8>> {
        Ok("test".as_bytes().to_vec())
    }
}

impl Node {
    /// 存储数据真实操作
    ///
    /// node_bytes 当前操作结点的字节数组
    ///
    /// level 当前操作结点层
    ///
    /// flexible_key 下一级最左最小树所对应真实key
    ///
    /// Seed value信息
    ///
    /// root 是否根结点
    ///
    /// node_seek 当前操作结点在文件中的真实起始位置
    fn put_in_node(
        &self,
        node_bytes: Vec<u8>,
        level: u8,
        flexible_key: u64,
        seed: Arc<RwLock<dyn TSeed>>,
        root: bool,
    ) -> GeorgeResult<()>
    where
        Self: Sized,
    {
        Ok(())
    }
}
