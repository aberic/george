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

use crate::task::engine::library::node::Node;
use crate::task::engine::traits::TNode;
use crate::task::seed::Seed;
use logs::set_log_test;
use std::sync::{Arc, RwLock};

fn node() -> Arc<RwLock<Node>> {
    match Node::create_root(
        "database_name".to_string(),
        "view_name".to_string(),
        "index_name".to_string(),
        false,
    ) {
        Ok(node) => node,
        _ => Node::recovery_root(
            "database_name".to_string(),
            "view_name".to_string(),
            "index_name".to_string(),
            false,
        )
        .unwrap(),
    }
}

#[test]
fn put_in_node_test() {
    set_log_test();
    let seed = Seed::create("".to_string(), vec![], false);
    let node = node();
    let node_w = node.write().unwrap();
    match node_w.put(65536, seed, false) {
        Err(err) => println!("put err = {}", err.to_string()),
        _ => {}
    }
}
