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

use std::rc::Rc;
use std::sync::{Mutex, MutexGuard};

use crate::cryptos::hash::HashMD5Handler;
use crate::cryptos::Hash;
use crate::errors::{Errs, GeorgeResult};
use crate::merkle::{Node, NodeChild};

impl Node {
    pub(crate) fn new(hash: String, count: u32, child: Option<Rc<Mutex<NodeChild>>>) -> Node {
        return Node { hash, count, child };
    }

    pub fn hash(&self) -> String {
        self.hash.clone()
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn child(&self) -> Option<Rc<Mutex<NodeChild>>> {
        self.child.clone()
    }

    pub(crate) fn modify_child(&mut self, child: Option<Rc<Mutex<NodeChild>>>) {
        self.child = child
    }

    pub(crate) fn fit(&mut self, hash: String, count: u32, child: Option<Rc<Mutex<NodeChild>>>) {
        self.hash = hash;
        self.count = count;
        self.child = child;
    }

    /// 新增结点
    ///
    /// level 当前结点层数
    ///
    /// hash 新增结点hash
    pub(crate) fn add(&mut self, level: u32, hash: String) -> GeorgeResult<()> {
        // 判断当前结点子节点是否存在
        match self.child() {
            // 子结点如果存在，则继续判断子结点信息
            Some(nc) => {
                let mut child = nc.lock().unwrap();
                if level == 2 {
                    match child.left() {
                        // 如果存在，则继续判断右叶子结点
                        Some(_n) => match child.right() {
                            // 右叶子结点存在，不合规
                            Some(_n) => {
                                return Err(Errs::str(
                                    "merkle data invalid, node un match size, try to rebuild!",
                                ));
                            }
                            // 右叶子结点不存在，新建
                            None => child.modify_right(hash, 0, None),
                        },
                        // 如果不存在，则直接插入当前hash
                        None => child.modify_left(hash, 0, None),
                    }
                } else {
                    left_add(level - 1, hash, child)?;
                }
            }
            None => {
                if level == 2 {
                    // 子结点如不存在，则新建左叶子节点并插入
                    self.child = Some(Rc::new(Mutex::new(NodeChild::new(hash))))
                } else {
                    // 如不存在，则新建左子树根结点
                    let mut left_node = Node::new("".to_string(), 0, None);
                    match left_node.add(level - 1, hash) {
                        Err(err) => return Err(err),
                        Ok(()) => {
                            // 赋值左子树
                            self.child = Some(Rc::new(Mutex::new(NodeChild::new_left(left_node))));
                        }
                    }
                }
            }
        }
        // 叶子结点总数+1
        self.count += 1;
        // 重置当前结点hash
        let hash_left: String;
        let mut hash_right = String::new();
        let child = self.child().unwrap();
        let child_m = child.lock().unwrap();
        match child_m.left() {
            Some(left) => hash_left = left.lock().unwrap().hash(),
            None => return Err(Errs::str("node left hash is none")),
        }
        match child_m.right() {
            Some(right) => hash_right = right.lock().unwrap().hash(),
            _ => {}
        }
        match hash_result(hash_left, hash_right) {
            Ok(h) => {
                self.hash = h;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}

fn left_add(level: u32, hash: String, child: MutexGuard<NodeChild>) -> GeorgeResult<()> {
    // 判断左子树根结点是否存在
    match child.left() {
        // 如存在，则判断是否满载
        Some(left_node) => {
            let ln = left_node.clone();
            let mut ln_m = ln.lock().unwrap();
            // 当前子树可容纳叶子结点总数
            let leaf_count = 2_u32.pow(level - 1);
            // 如果不满载
            if leaf_count > ln_m.count() {
                // 左子树新增结点
                ln_m.add(level, hash)
                // 如果满载
            } else if leaf_count == ln_m.count() {
                right_add(level, hash, child)
                // 如果超载
            } else {
                Err(Errs::str(
                    "merkle data invalid, left add out of size, try to rebuild!",
                ))
            }
        }
        // 如不存在，该子树即存在子结点又不存在左子结点，既不合规
        None => Err(Errs::str(
            "merkle data invalid, left add wrong exist, try to rebuild!",
        )),
    }
}

fn right_add(level: u32, hash: String, mut child: MutexGuard<NodeChild>) -> GeorgeResult<()> {
    // 判断右子树根结点是否存在
    match child.right() {
        // 如存在，则判断是否满载
        Some(right_node) => {
            let rn = right_node.clone();
            let mut rn_m = rn.lock().unwrap();
            // 当前子树可容纳叶子结点总数
            let leaf_count = 2_u32.pow(level - 1);
            // 如果不满载
            if leaf_count > rn_m.count() {
                // 右子树新增结点
                rn_m.add(level, hash)
                // 如果满载或超载
            } else {
                Err(Errs::str(
                    "merkle data invalid, right add out of size, try to rebuild!",
                ))
            }
        }
        // 如不存在，新建右子树
        None => {
            // 新建右子树根结点
            child.modify_right("".to_string(), 0, None);
            // 右子树新增结点
            child.right().unwrap().lock().unwrap().add(level, hash)
        }
    }
}

fn hash_result(hash_left: String, hash_right: String) -> GeorgeResult<String> {
    if hash_left.is_empty() {
        Err(Errs::str("node left hash is none"))
    } else {
        if hash_right.is_empty() {
            Ok(hash_left)
        } else {
            Ok(Hash::md5(format!("{}{}", hash_left, hash_right)))
        }
    }
}
