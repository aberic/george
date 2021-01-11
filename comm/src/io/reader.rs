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

use std::fs::{read_to_string, File};
use std::io::{Read, Seek, SeekFrom};

use crate::errors::entrances::err_string;
use crate::errors::entrances::GeorgeResult;
use std::sync::{Arc, RwLock};

pub fn read_all(filepath: &str) -> GeorgeResult<String> {
    match read_to_string(filepath) {
        Ok(s) => Ok(s),
        Err(err) => Err(err_string(err.to_string())),
    }
}

pub fn read_all_string(filepath: String) -> GeorgeResult<String> {
    match read_to_string(filepath) {
        Ok(s) => Ok(s),
        Err(err) => Err(err_string(err.to_string())),
    }
}

/// 读取文件部分内容，从start开始，一直持续读取last长度
pub fn read_sub(filepath: &str, start: u64, last: usize) -> GeorgeResult<String> {
    match File::open(filepath) {
        Ok(file) => read_sub_file(file, start, last),
        Err(err) => Err(err_string(err.to_string())),
    }
}

/// 读取文件部分内容，从start开始，一直持续读取last长度
pub fn read_sub_file(mut file: File, start: u64, last: usize) -> GeorgeResult<String> {
    match file.seek(SeekFrom::Start(start)) {
        Ok(_u) => {
            let mut res = String::with_capacity(last);
            let mut position = 0;
            let mut buffer = [0u8; 1024];
            let mut err_fin: String = "".to_string();
            while position < last {
                match file.read(&mut buffer) {
                    Ok(_u) => match String::from_utf8(buffer.to_vec()) {
                        Ok(tmp_s) => {
                            let chs = tmp_s.chars();
                            for ch in chs.into_iter() {
                                if position < last {
                                    res.push(ch)
                                } else {
                                    break;
                                }
                                position += 1
                            }
                        }
                        Err(err) => {
                            err_fin = format!("read sub from utf8 failed! error is {}", err);
                            break;
                        }
                    },
                    Err(err) => {
                        err_fin = format!("read sub file read failed! error is {}", err);
                        break;
                    }
                }
            }
            if err_fin.is_empty() {
                Ok(res)
            } else {
                Err(err_string(err_fin))
            }
        }
        Err(err) => Err(err_string(err.to_string())),
    }
}

/// 读取文件部分内容，从start开始，一直持续读取last长度
pub fn read_sub_bytes(filepath: String, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
    match File::open(filepath) {
        Ok(file) => read_sub_file_bytes(file, start, last),
        Err(err) => Err(err_string(err.to_string())),
    }
}

/// 读取文件部分内容，从start开始，一直持续读取last长度
pub fn read_sub_bytes_by_file(
    file: Arc<RwLock<File>>,
    start: u64,
    last: usize,
) -> GeorgeResult<Vec<u8>> {
    read_sub_file_bytes(
        file.clone().read().unwrap().try_clone().unwrap(),
        start,
        last,
    )
}

/// 读取文件部分内容，从start开始，一直持续读取last长度
pub fn read_sub_file_bytes(mut file: File, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
    match file.seek(SeekFrom::Start(start)) {
        Ok(_u) => {
            let mut buf: Vec<u8> = vec![];
            let mut buffer = [0u8; 1024];
            let mut position = 0;
            while position < last {
                match file.read(&mut buffer) {
                    Ok(_u) => {
                        if last - position >= 1024 {
                            for b in buffer.iter() {
                                buf.push(*b);
                                position += 1
                            }
                        } else {
                            for b in buffer.iter() {
                                buf.push(*b);
                                position += 1;
                                if last - position <= 0 {
                                    break;
                                }
                            }
                        }
                    }
                    Err(err) => {
                        return Err(err_string(format!(
                            "read sub file read failed! error is {}",
                            err
                        )));
                    }
                }
            }
            Ok(buf)
        }
        Err(err) => Err(err_string(err.to_string())),
    }
}
