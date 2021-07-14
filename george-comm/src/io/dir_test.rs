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
mod dir {

    use crate::io::dir::DirHandler;
    use crate::io::Dir;

    #[test]
    fn create_dir_test() {
        Dir::mk_uncheck("src/test/test/dir").unwrap();
    }

    #[test]
    fn copy_dir_test1() {
        let dir_from_path = String::from("src/test/test");
        let dir_to_path = String::from("src/test/src");
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
        match Dir::cp("src/test/test/dir", "src/test/dir_create/create3", false) {
            Ok(()) => println!("copy success!"),
            Err(err) => println!("err: {}", err),
        }
    }

    #[test]
    fn move_dir_test1() {
        let dir_from_path = String::from("src/test/test/dir");
        let dir_to_path = String::from("src/test/file");
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
    fn rename_dir_test1() {
        match Dir::rename("src/test/src/test/dir", "src/test/dirss") {
            Ok(()) => println!("rename success!"),
            Err(err) => println!("err: {}", err),
        }
    }
}
