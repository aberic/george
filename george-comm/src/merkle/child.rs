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

use std::sync::{Arc, RwLock};

use crate::merkle::{Node, NodeChild};

impl NodeChild {
    pub(crate) fn new(hash: String) -> NodeChild {
        NodeChild {
            0: Some(Arc::new(RwLock::new(Node::new(hash, 0, None)))),
            1: None,
        }
    }

    pub(crate) fn new_left(node: Node) -> NodeChild {
        NodeChild {
            0: Some(Arc::new(RwLock::new(node))),
            1: None,
        }
    }

    pub fn left(&self) -> Option<Arc<RwLock<Node>>> {
        self.0.clone()
    }

    pub fn right(&self) -> Option<Arc<RwLock<Node>>> {
        self.1.clone()
    }

    pub(crate) fn modify_left(
        &mut self,
        hash: String,
        count: u32,
        child: Option<Arc<RwLock<NodeChild>>>,
    ) {
        match self.left() {
            Some(n) => {
                let mut n_m = n.write().unwrap();
                n_m.fit(hash, count, child);
            }
            None => self.0 = Some(Arc::new(RwLock::new(Node::new(hash, count, child)))),
        }
    }

    pub(crate) fn modify_right(
        &mut self,
        hash: String,
        count: u32,
        child: Option<Arc<RwLock<NodeChild>>>,
    ) {
        match self.right() {
            Some(n) => {
                let mut n_m = n.write().unwrap();
                n_m.fit(hash, count, child);
            }
            None => self.1 = Some(Arc::new(RwLock::new(Node::new(hash, count, child)))),
        }
    }

    pub(crate) fn none_right(&mut self) {
        self.1 = None
    }
}
