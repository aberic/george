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

#[cfg(test)]
mod node_test {
    use crate::engine::siam::doc::node::Node;
    use crate::engine::siam::traits::TNode;
    use std::sync::Arc;

    #[test]
    fn create_root_test() {
        let n: Arc<Node> = Node::create_root(
            "database".to_string(),
            "view".to_string(),
            "index".to_string(),
        );
        println!("node is {:#?}", n);
        println!("node degree_index = {}", n.degree_index());
        println!("node nodes = {:#?}", n.nodes());
        println!();
    }
}
