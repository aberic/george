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

use comm::bytes::create_empty_bytes;
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
    pub(crate) fn node_bytes(&self) -> Arc<RwLock<Vec<u8>>> {
        self.node_bytes.clone()
    }
    fn set_node_bytes(&self, bytes: Vec<u8>) {
        let node_bytes = self.node_bytes();
        let mut nb_w = node_bytes.write().unwrap();
        nb_w.copy_from_slice(bytes.as_slice())
    }
    // fn put(
    //     &self,
    //     key: String,
    //     seed: Arc<RwLock<dyn TSeed>>,
    //     force: bool,
    //     description_len: usize,
    // ) -> GeorgeResult<()>
    //     where
    //         Self: Sized,
    // {
    //     let node_bytes = self.node_bytes().read().unwrap().to_vec();
    //     self.put_in_node(
    //         node_bytes,
    //         1,
    //         hashcode32_enhance(key),
    //         seed,
    //         force,
    //         true,
    //         description_len as u64,
    //     )
    // }
    // fn get(&self, key: String) -> GeorgeResult<Vec<u8>> {
    //     let node_bytes = self.node_bytes().read().unwrap().to_vec();
    //     self.get_in_node(node_bytes, 1, hashcode32_enhance(key))
    // }
    // fn get_last(&self) -> GeorgeResult<Vec<u8>>
    //     where
    //         Self: Sized,
    // {
    //     let node_bytes = self.node_bytes().read().unwrap().to_vec();
    //     self.get_last_in_node(node_bytes, 1)
    // }
    // fn select(
    //     &self,
    //     mold: IndexMold,
    //     left: bool,
    //     start: u64,
    //     end: u64,
    //     constraint: Constraint,
    // ) -> GeorgeResult<(u64, u64, Vec<Vec<u8>>)> {
    //     let node_bytes = self.node_bytes().read().unwrap().to_vec();
    //     match File::open(self.index_file_path()) {
    //         Ok(index_file_real) => match File::open(self.view_file_path()) {
    //             Ok(view_file_real) => {
    //                 let index_file = Arc::new(RwLock::new(index_file_real));
    //                 let view_file = Arc::new(RwLock::new(view_file_real));
    //                 let level = 1;
    //                 let conditions = constraint.conditions();
    //                 log::debug!("conditions length = {}", conditions.len());
    //                 let skip = constraint.skip();
    //                 let limit = constraint.limit();
    //                 let delete = constraint.delete();
    //                 let query: (u64, u64, u64, u64, Vec<Vec<u8>>);
    //                 if left {
    //                     query = self.left_query(
    //                         mold, index_file, view_file, node_bytes, start, end, level, conditions,
    //                         skip, limit, delete,
    //                     )?
    //                 } else {
    //                     query = self.right_query(
    //                         mold, index_file, view_file, node_bytes, start, end, level, conditions,
    //                         skip, limit, delete,
    //                     )?
    //                 }
    //                 Ok((query.0, query.1, query.4))
    //             }
    //             Err(err) => Err(err_string_enhance(
    //                 format!(
    //                     "select view file whit path {} error, ",
    //                     self.view_file_path()
    //                 ),
    //                 err.to_string(),
    //             )),
    //         },
    //         Err(err) => Err(err_string_enhance(
    //             format!(
    //                 "select index file whit path {} error, ",
    //                 self.index_file_path()
    //             ),
    //             err.to_string(),
    //         )),
    //     }
    // }
}
