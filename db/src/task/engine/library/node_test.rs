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

#[cfg(test)]
mod library_node_test {
    use crate::task::engine::library::node::Node;
    use crate::task::view::View;
    use crate::utils::enums::KeyType;
    use comm::errors::entrances::GeorgeResult;
    use std::sync::Arc;

    fn create_node(
        database_name: String,
        view_name: String,
        index_name: String,
        key_type: KeyType,
        unique: bool,
    ) -> GeorgeResult<Arc<Node>> {
        let view = View::mock_create(database_name, view_name).unwrap();
        Node::mock_recovery(view, index_name, key_type, unique)
    }

    #[test]
    fn create_root_test() {
        let node = create_node(
            "db".to_string(),
            "view".to_string(),
            "index".to_string(),
            KeyType::String,
            true,
        )
        .unwrap();
        println!("node create success! {:#?}", node);
    }

    #[cfg(test)]
    mod unique_test {
        use crate::task::engine::library::node_test::library_node_test::create_node;
        use crate::task::engine::traits::{TNode, TSeed};
        use crate::task::seed::Seed;
        use crate::task::view::View;
        use crate::utils::enums::KeyType;

        #[test]
        fn put_test() {
            let view = View::mock_create_single("db".to_string(), "view".to_string()).unwrap();
            let node = create_node(
                "db".to_string(),
                "view".to_string(),
                "library".to_string(),
                KeyType::String,
                true,
            )
            .unwrap();
            let seed = Seed::create(view, "yes".to_string(), "no".to_string().into_bytes());
            match node.put("yes".to_string(), seed.clone(), false) {
                Ok(()) => {
                    let seed_w = seed.write().unwrap();
                    match seed_w.save() {
                        Ok(()) => println!("put success!"),
                        Err(err) => println!("seed save error! error is {}", err),
                    }
                }
                Err(err) => println!("put error! error is {}", err),
            }
        }

        #[test]
        fn get_test() {
            View::mock_create_single("db".to_string(), "view".to_string()).unwrap();
            let node = create_node(
                "db".to_string(),
                "view".to_string(),
                "library".to_string(),
                KeyType::String,
                true,
            )
            .unwrap();
            match node.get("yes".to_string()) {
                Ok(v8s) => println!("res = {:#?}", String::from_utf8(v8s).unwrap().as_str()),
                Err(err) => println!("get error! error is {}", err),
            }
        }

        #[test]
        fn del_test() {
            let view = View::mock_create_single("db".to_string(), "view".to_string()).unwrap();
            let node = create_node(
                "db".to_string(),
                "view".to_string(),
                "library".to_string(),
                KeyType::String,
                true,
            )
            .unwrap();
            let seed = Seed::create(view, "yes".to_string(), "no".to_string().into_bytes());
            match node.del("yes".to_string(), seed.clone()) {
                Ok(v8s) => {
                    let seed_w = seed.write().unwrap();
                    match seed_w.remove() {
                        Ok(()) => println!("del success!"),
                        Err(err) => println!("seed save error! error is {}", err),
                    }
                }
                Err(err) => println!("del error! error is {}", err),
            }
        }
    }

    #[cfg(test)]
    mod un_unique_test {
        use crate::task::engine::library::node_test::library_node_test::create_node;
        use crate::task::engine::traits::{TNode, TSeed};
        use crate::task::seed::Seed;
        use crate::task::view::View;
        use crate::utils::enums::KeyType;

        #[test]
        fn put_test() {
            let view = View::mock_create_single("db".to_string(), "view".to_string()).unwrap();
            let node = create_node(
                "db".to_string(),
                "view".to_string(),
                "library1".to_string(),
                KeyType::String,
                false,
            )
            .unwrap();
            let seed = Seed::create(view, "yes".to_string(), "no".to_string().into_bytes());
            match node.put("yes".to_string(), seed.clone(), false) {
                Ok(()) => {
                    let seed_w = seed.write().unwrap();
                    match seed_w.save() {
                        Ok(()) => println!("put success!"),
                        Err(err) => println!("seed save error! error is {}", err),
                    }
                }
                Err(err) => println!("put error! error is {}", err),
            }
        }

        #[test]
        fn get_test() {
            View::mock_create_single("db".to_string(), "view".to_string()).unwrap();
            let node = create_node(
                "db".to_string(),
                "view".to_string(),
                "library1".to_string(),
                KeyType::String,
                false,
            )
            .unwrap();
            match node.get("yes".to_string()) {
                Ok(v8s) => println!("res = {:#?}", String::from_utf8(v8s).unwrap().as_str()),
                Err(err) => println!("get error! error is {}", err),
            }
        }

        #[test]
        fn del_test() {
            let view = View::mock_create_single("db".to_string(), "view".to_string()).unwrap();
            let node = create_node(
                "db".to_string(),
                "view".to_string(),
                "library1".to_string(),
                KeyType::String,
                false,
            )
            .unwrap();
            let seed = Seed::create(view, "yes".to_string(), "no".to_string().into_bytes());
            match node.del("yes".to_string(), seed.clone()) {
                Ok(v8s) => {
                    let seed_w = seed.write().unwrap();
                    match seed_w.remove() {
                        Ok(()) => println!("del success!"),
                        Err(err) => println!("seed save error! error is {}", err),
                    }
                }
                Err(err) => println!("del error! error is {}", err),
            }
        }
    }
}
