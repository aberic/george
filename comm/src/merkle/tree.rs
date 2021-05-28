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
use std::sync::Mutex;

use crate::errors::{Errs, GeorgeResult};
use crate::merkle::{Node, NodeChild, Tree};

/// 新建默克尔树
fn new_string(hash: String) -> Tree {
    return Tree {
        level: 2,
        root: Rc::new(Mutex::new(Node::new(
            hash.clone(),
            1,
            Some(Rc::new(Mutex::new(NodeChild::new(hash)))),
        ))),
    };
}

impl Tree {
    /// 新建默克尔树
    pub fn new(hash: &str) -> Tree {
        new_string(hash.to_string())
    }

    /// 新建默克尔树
    pub fn new_string(hash: String) -> Tree {
        new_string(hash)
    }
    pub fn level(&self) -> u32 {
        self.level
    }
    pub fn root(&self) -> Rc<Mutex<Node>> {
        self.root.clone()
    }
    pub fn hash(&self) -> String {
        self.root.lock().unwrap().hash()
    }
    /// 新增结点
    pub fn add(&mut self, hash: &str) -> GeorgeResult<()> {
        self.add_string(hash.to_string())
    }
    /// 新增结点
    fn add_string(&mut self, hash: String) -> GeorgeResult<()> {
        // 当前默克尔树可容纳叶子结点总数
        let root_count = 2_u32.pow(self.level - 1);
        let mut root = self.root.lock().unwrap();
        // 如果默克尔树可容纳叶子结点总数大于当前叶子结点数，则说明当前树未满载，允许新结点右插入
        if root_count > root.count() {
            // 新结点右插入
            root.add(self.level, hash)
        // 如果默克尔树可容纳叶子结点总数等于当前叶子结点数，则说明当前树满载，默克尔树除了根结点外整体下沉并成为根结点的左子树，新增结点从根结点右子树中寻找空叶子结点位
        } else if root_count == root.count() {
            match root.child() {
                // 子结点如果存在，则继续判断子结点信息
                Some(nc) => {
                    let mut child_new = NodeChild::new("".to_string());
                    // 变更根结点左子树
                    child_new.modify_left(root.hash(), root.count(), Some(nc));
                    // 变更根结点右子树
                    child_new.none_right();
                    root.modify_child(Some(Rc::new(Mutex::new(child_new))));
                }
                None => {
                    return Err(Errs::str(
                        "merkle data invalid, root child do not exist, try to rebuild!",
                    ));
                }
            }
            // 默克尔树层高+1
            self.level += 1;
            root.add(self.level, hash)
        // 如果默克尔树可容纳叶子结点总数小于当前叶子结点数，则说明当前树异常，需要重建
        } else {
            Err(Errs::str("merkle data invalid, try to rebuild!"))
        }
    }
}
