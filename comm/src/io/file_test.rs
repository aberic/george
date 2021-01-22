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
mod file {
    use crate::io::file::{Filer, FilerHandler};
    use crate::trans::trans_u64_2_string64;
    use std::fs;
    use std::io::{Read, Write};

    #[test]
    fn create_file_test() {
        match Filer::touch("src/test/test/dir") {
            Ok(_f) => println!("file_test success"),
            Err(err) => {
                println!("file_test err = {}", err);
            }
        }
        match Filer::touch("src/test/test/dir") {
            Ok(_f) => println!("file_test success"),
            Err(err) => {
                println!("file_test err = {}", err);
            }
        }
        match Filer::touch("src/test/test/file/a.txt") {
            Ok(_) => {
                // file.write_all("test".as_bytes()).unwrap();
                println!("file_test success")
            }
            Err(err) => {
                println!("file_test err = {}", err);
            }
        }
        match Filer::touch("src/test/test/file/a.txt") {
            Ok(_) => println!("file_test success"),
            Err(err) => {
                println!("file_test err = {}", err);
            }
        }
        match Filer::touch("src/test/test/file/b.txt") {
            Ok(_) => {
                // file.write_all("test".as_bytes()).unwrap();
                println!("file_test success")
            }
            Err(err) => {
                println!("file_test err = {}", err);
            }
        }
        match Filer::touch("src/test/test/file/b.txt") {
            Ok(_) => println!("file_test success"),
            Err(err) => {
                println!("file_test err = {}", err);
            }
        }
    }

    #[test]
    fn copy_test() {
        match Filer::touch("src/test/test/file/copy_from.txt") {
            Ok(_) => {
                // file.write_all("copy_from".as_bytes()).unwrap();
                println!("file_test success")
            }
            Err(err) => {
                println!("file_test err = {}", err);
            }
        }
        match Filer::cp(
            "src/test/test/file/copy_from.txt",
            "src/test/test/file/copy_to.txt",
        ) {
            Err(err) => println!("file_copy err = {}", err),
            _ => {}
        }
    }

    #[test]
    fn move_test() {
        match Filer::touch("src/test/test/file/move_from.txt") {
            Ok(_) => {
                // file.write_all("move_from".as_bytes()).unwrap();
                println!("file_test success")
            }
            Err(err) => {
                println!("file_test err = {}", err);
            }
        }
        match Filer::mv(
            "src/test/test/file/move_from.txt",
            "src/test/test/file/move_to.txt",
        ) {
            Err(err) => println!("file_move err = {}", err),
            _ => {}
        }
    }
}
