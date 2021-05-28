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
    use crate::io::file::{FilerHandler, FilerNormal, FilerReader, FilerWriter};
    use crate::io::Filer;
    use crate::vectors::VectorHandler;
    use crate::Vector;

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
        match Filer::absolute("src/test/test/file/move_from.txt") {
            Ok(res) => {
                // file.write_all("move_from".as_bytes()).unwrap();
                println!("file absolute = {}", res)
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
    fn writer_test() {
        match Filer::write("src/test/file/x.txt", vec![0x0b, 0x0c, 0x0d, 0x0e]) {
            Ok(s) => println!("write success with s = {}", s),
            Err(err) => println!("file write err = {}", err),
        }
        match Filer::write_force("src/test/file/y.txt", vec![0x0b, 0x0c, 0x0d, 0x0e]) {
            Ok(s) => println!("write success with s = {}", s),
            Err(err) => println!("file write err = {}", err),
        }
        match Filer::write_force("src/test/file/y.txt", vec![0x01, 0x02, 0x03]) {
            Ok(s) => println!("write success with s = {}", s),
            Err(err) => println!("file write err = {}", err),
        }
    }

    #[test]
    fn writer_append_test() {
        Filer::try_touch("src/test/file/g.txt").unwrap();
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
        match Filer::append(
            "src/test/file/h.txt",
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
        Filer::try_touch("src/test/file/seek.txt").unwrap();
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
        Filer::try_touch("src/test/file/reader.txt").unwrap();
        let u8s1 = "hello world!".as_bytes();
        match Filer::write_seek("src/test/file/reader.txt", 100000000, u8s1) {
            Err(err) => println!("err = {}", err),
            _ => {}
        }
        let x1 = Filer::read_sub_allow_none("src/test/file/reader.txt", 150000000, 7).unwrap();
        println!("x1 is empty = {}", Vector::is_fill(x1.clone()));
        println!("x1 = {}", String::from_utf8(x1).unwrap());

        let x2 = Filer::read_sub_allow_none("src/test/file/reader.txt", 160000000, 8).unwrap();
        println!("x2 is empty = {}", Vector::is_fill(x2.clone()));
        println!("x2 = {}", String::from_utf8(x2).unwrap());

        match Filer::try_touch("src/test/file/read_sub.txt") {
            Err(err) => println!("try_touch err = {}", err.to_string()),
            _ => {}
        }
        match Filer::read_sub("src/test/file/read_sub.txt", 150000000, 7) {
            Ok(x3) => println!("x3 = {}", String::from_utf8(x3).unwrap()),
            Err(err) => println!("read_sub err = {}", err.to_string()),
        }
    }

    #[test]
    fn reader_test2() {
        let s = Filer::read("src/examples/conf.yaml");
        println!("s = {:#?}", s);
    }

    #[test]
    fn reader_test3() {
        let s1 = Filer::read("src/cryptos/mod.rs").unwrap();
        println!("s = {}", s1);
        let file = Filer::reader("src/cryptos/mod.rs").unwrap();
        let s2 = Filer::read_file(file).unwrap();
        println!("s = {}", s2);
        assert_eq!(s1, s2);
    }

    #[test]
    fn read_sub_bytes_test1() {
        println!(
            "res1 = {:#?}",
            Filer::read_sub("src/cryptos/mod.rs".to_string(), 448, 8,).unwrap()
        );
        println!(
            "res2 = {:#?}",
            Filer::read_sub("src/cryptos/mod.rs".to_string(), 0, 2048,).unwrap()
        );
    }

    #[test]
    fn read_sub_bytes_test2() {
        println!(
            "res1 = {:#?}",
            Filer::read_sub("src/cryptos/mod.rs", 448, 8).unwrap()
        );
        let file = Filer::reader("src/cryptos/mod.rs").unwrap();
        println!("res2 = {:#?}", Filer::read_file_sub(file, 448, 8).unwrap());
    }

    #[test]
    fn file_len_test() {
        println!("len1 = {:#?}", Filer::len("src/cryptos/mod.rs").unwrap());
        let file = Filer::reader("src/cryptos/mod.rs").unwrap();
        println!("len2 = {:#?}", Filer::len_file(file).unwrap());
    }
}
