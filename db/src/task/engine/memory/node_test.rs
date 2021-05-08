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
mod node_test {
    use crate::task::engine::memory::node::Node;

    #[test]
    fn put_get_test() {
        let root = Node::create();
        let root_w = root.write().unwrap();

        let key = String::from("test");
        let value1 = String::from("v1").into_bytes();
        let value2 = String::from("v2").into_bytes();
        match root_w.put(key.clone(), value1, false) {
            Ok(()) => println!("put success!"),
            Err(err) => println!("put error! error is {}", err),
        }
        match root_w.put(key.clone(), value2, false) {
            Ok(()) => println!("put success!"),
            Err(err) => println!("put error! error is {}", err),
        }
        match root_w.get(key) {
            Ok(v8s) => println!("res = {:#?}", String::from_utf8(v8s).unwrap().as_str()),
            Err(err) => println!("get error! error is {}", err),
        }
    }
}
