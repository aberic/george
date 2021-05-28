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
mod merkle {
    use crate::merkle::{Node, Tree};
    use std::rc::Rc;
    use std::sync::Mutex;

    #[test]
    fn tree_test() {
        let mut tree = Tree::new("1");
        match tree.add("2") {
            Err(err) => println!("err 2 = {}", err),
            _ => {}
        }
        match tree.add("3") {
            Err(err) => println!("err 3 = {}", err),
            _ => {}
        }
        match tree.add("4") {
            Err(err) => println!("err 4 = {}", err),
            _ => {}
        }
        match tree.add("5") {
            Err(err) => println!("err 5 = {}", err),
            _ => {}
        }
        match tree.add("6") {
            Err(err) => println!("err 6 = {}", err),
            _ => {}
        }
        println!("tree hash = {}", tree.hash());
        tree_println(tree.level(), tree.root());
    }

    fn tree_println(level: u32, node: Rc<Mutex<Node>>) {
        let n_m = node.lock().unwrap();
        println!(
            "level = {}, count = {}, hash = {}",
            level,
            n_m.count(),
            n_m.hash()
        );
        match n_m.child() {
            Some(child) => {
                let c_m = child.lock().unwrap();
                match c_m.left() {
                    Some(left_node) => {
                        tree_println(level - 1, left_node);
                        match c_m.right() {
                            Some(right_node) => {
                                tree_println(level - 1, right_node);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
