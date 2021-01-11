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
    use std::sync::{Arc, RwLock};

    use crate::engine::siam::mem::node::Node;
    use crate::engine::siam::mem::seed::Seed;
    use crate::engine::siam::traits::TNode;
    use crate::engine::traits::TSeed;
    use comm::cryptos::hash::md516;
    use std::error::Error;

    #[test]
    fn create_root_test() {
        let n: Arc<Node> = Node::create_root();
        println!("node is {:#?}", n);
        println!("node degree_index = {}", n.degree_index());
        println!("node nodes = {:#?}", n.nodes());
        println!();
    }

    #[test]
    fn put_get_32() {
        let root: Arc<Node> = Node::create_root();
        let key = "test".to_string();
        let seed = Arc::new(RwLock::new(Seed::create(md516(key.clone()))));
        seed.write().unwrap().save("1".as_bytes().to_vec()).unwrap();
        root.put(key.clone(), seed, false, 0).unwrap();
        let irg = root.get(key.clone());
        match irg {
            Ok(seed) => println!("u is {:#?}", seed),
            Err(ie) => println!("res is {:#?}", ie.source().unwrap().to_string()),
        }
        let irg = root.get(key.clone());
        match irg {
            Ok(seed) => println!("u is {:#?}", seed),
            Err(ie) => println!("res is {:#?}", ie.source().unwrap().to_string()),
        }
    }

    #[test]
    fn put_get_64() {
        let root: Arc<Node> = Node::create_root();
        let key = "test".to_string();
        let seed = Arc::new(RwLock::new(Seed::create(md516(key.clone()))));
        seed.write().unwrap().save("1".as_bytes().to_vec()).unwrap();
        root.put(key.clone(), seed, false, 0).unwrap();
        let irg = root.get(key.clone());
        match irg {
            Ok(seed) => println!("u is {:#?}", seed),
            Err(ie) => println!("res is {:#?}", ie.source().unwrap().to_string()),
        }
        let irg = root.get(key.clone());
        match irg {
            Ok(seed) => println!("u is {:#?}", seed),
            Err(ie) => println!("res is {:#?}", ie.source().unwrap().to_string()),
        }
    }
}
