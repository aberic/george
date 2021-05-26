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

use crate::merkle::node;
use crate::merkle::node::Node;

pub(super) struct NodeChild(Option<Rc<Mutex<Node>>>, Option<Rc<Mutex<Node>>>);

pub(super) fn new(hash: String) -> NodeChild {
    NodeChild {
        0: Some(Rc::new(Mutex::new(node::new(hash, 0, None)))),
        1: None,
    }
}

pub(super) fn new_left(node: Node) -> NodeChild {
    NodeChild {
        0: Some(Rc::new(Mutex::new(node))),
        1: None,
    }
}

impl NodeChild {
    pub(super) fn left(&self) -> Option<Rc<Mutex<Node>>> {
        self.0.clone()
    }
    pub(super) fn right(&self) -> Option<Rc<Mutex<Node>>> {
        self.1.clone()
    }
    pub(super) fn modify_left(
        &mut self,
        hash: String,
        count: u32,
        child: Option<Rc<Mutex<NodeChild>>>,
    ) {
        match self.left() {
            Some(n) => {
                let mut n_m = n.lock().unwrap();
                n_m.fit(hash, count, child);
            }
            None => self.0 = Some(Rc::new(Mutex::new(node::new(hash, count, child)))),
        }
    }
    pub(super) fn modify_right(
        &mut self,
        hash: String,
        count: u32,
        child: Option<Rc<Mutex<NodeChild>>>,
    ) {
        match self.right() {
            Some(n) => {
                let mut n_m = n.lock().unwrap();
                n_m.fit(hash, count, child);
            }
            None => self.1 = Some(Rc::new(Mutex::new(node::new(hash, count, child)))),
        }
    }
    pub(super) fn none_right(&mut self) {
        self.1 = None
    }
}
