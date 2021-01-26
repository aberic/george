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
    use crate::io::file::{Filer, FilerHandler, FilerReader, FilerWriter};
    use crate::trans::{trans_bytes_2_u64, trans_u64_2_string64};
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

    #[test]
    fn writer_append_test() {
        Filer::touch("src/test/file/g.txt").unwrap();
        match Filer::append(
            "src/test/file/g.txt",
            vec![
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
            ],
        ) {
            Ok(()) => {
                let vs: Vec<u8> = vec![0x0b, 0x0c, 0x0d, 0x0e];
                match Filer::write_seek("src/test/file/g.txt", 3, vs) {
                    Err(err) => println!("err = {}", err),
                    _ => {}
                }
            }
            Err(err) => println!("err = {}", err),
        }
    }

    #[test]
    fn writer_seek_test() {
        Filer::touch("src/test/file/seek.txt").unwrap();
        let u8s1 = "hello world!".as_bytes();
        match Filer::write_seek("src/test/file/seek.txt", 100000000, u8s1) {
            Err(err) => println!("err = {}", err),
            _ => {}
        }
        let u8s2 = "success!".as_bytes();
        match Filer::write_seek("src/test/file/seek.txt", 300000000, u8s2) {
            Err(err) => println!("err = {}", err),
            _ => {}
        }
        let u8s3 = "failed!".as_bytes();
        match Filer::write_seek("src/test/file/seek.txt", 150000000, u8s3) {
            Err(err) => println!("err = {}", err),
            _ => {}
        }

        let x1 = Filer::read_sub("src/test/file/seek.txt", 150000000, 7).unwrap();
        println!("x = {}", String::from_utf8(x1).unwrap());
    }

    #[test]
    fn reader_test1() {
        let x1 = Filer::read_sub("src/test/file/seek.txt", 150000000, 7).unwrap();
        println!("x1 is empty = {}", is_bytes_fill(x1.clone()));
        println!("x1 = {}", String::from_utf8(x1).unwrap());

        let x2 = Filer::read_sub("src/test/file/seek.txt", 160000000, 8).unwrap();
        println!("x2 is empty = {}", is_bytes_fill(x2.clone()));
        println!("x2 = {}", String::from_utf8(x2).unwrap());
    }

    /// 检查字节数组是否已有数据，即不为空且每一个字节都不是0x00
    pub fn is_bytes_fill(bs: Vec<u8>) -> bool {
        let bs_len = bs.len();
        let mut i = 0;
        while i < bs_len {
            if bs[i].ne(&0x00) {
                return true;
            }
            i += 1;
        }
        false
    }

    #[test]
    fn reader_test2() {
        let s = Filer::read("src/examples/conf.yaml");
        println!("s = {:#?}", s);
    }

    #[test]
    fn read_sub_bytes_test() {
        println!(
            "res1 = {:#?}",
            Filer::read_sub("src/examples/29f459a44fee58c7.ge".to_string(), 448, 8,).unwrap()
        );
        println!(
            "res2 = {:#?}",
            Filer::read_sub("src/examples/29f459a44fee58c7.ge".to_string(), 0, 2048,).unwrap()
        );
    }
}
