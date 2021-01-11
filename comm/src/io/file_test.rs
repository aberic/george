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
    use std::io::{Read, Write};

    use crate::io::file::{create_dir_str, create_file_str};
    use crate::trans::trans_u64_2_string64;
    use std::fs::{copy, File};

    #[test]
    fn create_dir_test() {
        create_dir_str("src/test/test/dir").unwrap();
    }

    #[test]
    fn create_file_test() {
        match create_file_str("src/test/test/dir", true) {
            Ok(_f) => println!("file_test success"),
            Err(err) => {
                println!("file_test err = {}", err);
            }
        }
        match create_file_str("src/test/test/dir", false) {
            Ok(_f) => println!("file_test success"),
            Err(err) => {
                println!("file_test err = {}", err);
            }
        }
        match create_file_str("src/test/test/file/a.txt", false) {
            Ok(mut file) => {
                file.write_all("test".as_bytes()).unwrap();
                println!("file_test success")
            }
            Err(err) => {
                println!("file_test err = {}", err);
            }
        }
        match create_file_str("src/test/test/file/a.txt", false) {
            Ok(_file) => println!("file_test success"),
            Err(err) => {
                println!("file_test err = {}", err);
            }
        }
        match create_file_str("src/test/test/file/b.txt", false) {
            Ok(mut file) => {
                file.write_all("test".as_bytes()).unwrap();
                println!("file_test success")
            }
            Err(err) => {
                println!("file_test err = {}", err);
            }
        }
        match create_file_str("src/test/test/file/b.txt", true) {
            Ok(_file) => println!("file_test success"),
            Err(err) => {
                println!("file_test err = {}", err);
            }
        }
    }

    #[test]
    fn u8_test() {
        let x: u8 = 0x20;
        match create_file_str("src/test/test/file/c.txt", true) {
            Ok(mut file) => {
                let buf_w = [x];
                file.write_all(&buf_w).unwrap();
                println!("file_test success");
                let mut buf_r = [0u8; 1];
                match file.read(&mut buf_r) {
                    Ok(size) => {
                        println!("size = {}, buf_r_str = {}", size, buf_r[0].to_string());
                    }
                    Err(err) => println!("file_test err = {}", err),
                }
            }
            Err(err) => println!("file_test err = {}", err),
        }
    }

    #[test]
    fn u8_arr_test1() {
        let x: [u8; 4] = [0x20, 0x19, 0x02, 0x19];
        match create_file_str("src/test/test/file/d.txt", true) {
            Ok(mut file) => {
                file.write_all(&x).unwrap();
                println!("write success");
            }
            Err(err) => println!("file create err = {}", err),
        }
        match File::open("src/test/test/file/d.txt") {
            Ok(mut file) => {
                let mut buf_r = [0u8; 4];
                match file.read(&mut buf_r) {
                    Ok(size) => {
                        println!("size = {}, buf_r_str = {}", size, hex::encode(buf_r));
                    }
                    Err(err) => println!("file read err = {}", err),
                }
            }
            Err(err) => println!("file open err = {}", err),
        }
    }

    #[test]
    fn u8_arr_test2() {
        let x: u64 = 18446744073709551615;
        match create_file_str("src/test/test/file/e.txt", true) {
            Ok(mut file) => {
                let xu = &x.to_be_bytes();
                println!("xu = {:#?}", xu);
                file.write_all(xu).unwrap();
                println!("write success");
            }
            Err(err) => println!("file create err = {}", err),
        }

        let x: u64 = 18446744073709551615;
        match create_file_str("src/test/test/file/f.txt", true) {
            Ok(mut file) => {
                let xs = trans_u64_2_string64(x);
                let xu = xs.as_bytes();
                println!("xu = {:#?}", xu);
                file.write_all(xu).unwrap();
                println!("write success");
            }
            Err(err) => println!("file create err = {}", err),
        }
    }

    #[test]
    fn copy_test() {
        match create_file_str("src/test/test/file/copy_from.txt", false) {
            Ok(mut file) => {
                file.write_all("copy_from".as_bytes()).unwrap();
                println!("file_test success")
            }
            Err(err) => {
                println!("file_test err = {}", err);
            }
        }
        match copy(
            "src/test/test/file/copy_from.txt",
            "src/test/test/file/copy_to.txt",
        ) {
            Err(err) => println!("file_copy err = {}", err),
            _ => {}
        }
    }
}
