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

use crate::io::dir::{Dir, DirHandler};

#[test]
fn create_dir_test() {
    Dir::mk("src/test/test/dir").unwrap();
}

#[test]
fn copy_dir_test1() {
    let dir_from_path = String::from("src");
    let dir_to_path = String::from("src/test");
    match Dir::cp(dir_from_path, dir_to_path, false) {
        Ok(()) => println!("copy success!"),
        Err(err) => println!("err: {}", err),
    }
}

#[test]
fn copy_dir_test2() {
    let dir_from_path = String::from("hello");
    let dir_to_path = String::from("src/test");
    match Dir::cp(dir_from_path, dir_to_path, false) {
        Ok(()) => println!("copy success!"),
        Err(err) => println!("err: {}", err),
    }
}

#[test]
fn copy_dir_test3() {
    match Dir::cp("src", "src/mm/dir_create/create3", false) {
        Ok(()) => println!("copy success!"),
        Err(err) => println!("err: {}", err),
    }
}

#[test]
fn copy_dir_test4() {
    match Dir::cp("src", "src/io", false) {
        Ok(()) => println!("copy success!"),
        Err(err) => println!("err: {}", err),
    }
}

#[test]
fn copy_dir_test5() {
    match Dir::cp("src/test/crypto", "src/test/dir", false) {
        Ok(()) => println!("copy success!"),
        Err(err) => println!("err: {}", err),
    }
}

#[test]
fn move_dir_test1() {
    let dir_from_path = String::from("src");
    let dir_to_path = String::from("src/test");
    match Dir::mv(dir_from_path, dir_to_path, false) {
        Ok(()) => println!("move success!"),
        Err(err) => println!("err: {}", err),
    }
}

#[test]
fn move_dir_test2() {
    let dir_from_path = String::from("hello");
    let dir_to_path = String::from("src/test");
    match Dir::mv(dir_from_path, dir_to_path, false) {
        Ok(()) => println!("move success!"),
        Err(err) => println!("err: {}", err),
    }
}

#[test]
fn move_dir_test3() {
    match Dir::mv("src", "src/test", false) {
        Ok(()) => println!("move success!"),
        Err(err) => println!("err: {}", err),
    }
}

#[test]
fn move_dir_test4() {
    match Dir::mv("hello", "src/test", false) {
        Ok(()) => println!("move success!"),
        Err(err) => println!("err: {}", err),
    }
}

#[test]
fn move_dir_test5() {
    match Dir::mv("src/test/dir/crypto", "src/test/dirs", false) {
        Ok(()) => println!("move success!"),
        Err(err) => println!("err: {}", err),
    }
}
