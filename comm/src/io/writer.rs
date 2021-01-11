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

use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use std::sync::{Arc, RwLock};

use crate::errors::entrances::{err_str_enhance, err_string};
use crate::errors::entrances::GeorgeResult;
use crate::io::file::create_file;

/// 在指定文件中追加数据
pub fn write_append_str(filepath: String, content: &str) -> GeorgeResult<()> {
    match OpenOptions::new().append(true).open(filepath) {
        Ok(mut file) => match file.write_all(content.as_bytes()) {
            Ok(()) => Ok(()),
            Err(err) => Err(err_string(err.to_string())),
        },
        Err(err) => Err(err_string(err.to_string())),
    }
}

/// 在指定文件中追加数据
pub fn write_append_string(filepath: String, content: String) -> GeorgeResult<()> {
    write_append_bytes(filepath, content.into_bytes())
}

/// 在指定文件中追加数据
pub fn write_append_bytes(filepath: String, content: Vec<u8>) -> GeorgeResult<()> {
    match OpenOptions::new().append(true).open(filepath) {
        Ok(file) => write_file_append_bytes(file, content),
        Err(err) => Err(err_string(err.to_string())),
    }
}

/// 在指定文件中写入数据
///
/// force 如果已存在，是否删除重写
pub fn write_bytes(filepath: String, content: Vec<u8>, force: bool) -> GeorgeResult<()> {
    match create_file(filepath, force) {
        Ok(mut file) => match file.write(content.as_slice()) {
            Ok(_) => Ok(()),
            Err(err) => Err(err_string(err.to_string())),
        },
        Err(err) => Err(err_string(err.to_string())),
    }
}

/// 在指定文件中写入数据
///
/// force 如果已存在，是否删除重写
pub fn write_bytes_str(filepath: &str, content: Vec<u8>, force: bool) -> GeorgeResult<()> {
    write_bytes(filepath.to_string(), content, force)
}

/// 在指定文件中指定位置后覆盖数据
pub fn write_seek_u8s(filepath: String, seek: u64, content: &[u8]) -> GeorgeResult<()> {
    match OpenOptions::new().write(true).open(filepath) {
        Ok(file) => write_file_seek_u8s(file, seek, content),
        Err(err) => Err(err_string(err.to_string())),
    }
}

/// 在指定文件中追加数据
pub fn write_file_append_bytes(mut file: File, content: Vec<u8>) -> GeorgeResult<()> {
    match file.write_all(content.as_slice()) {
        Ok(()) => Ok(()),
        Err(err) => Err(err_string(err.to_string())),
    }
}

pub fn write_all<T>(file: File, input: String, t: T) -> GeorgeResult<Arc<RwLock<T>>> {
    write_all_bytes(file, input.into_bytes(), t)
}

/// 在指定文件中追加数据
pub fn write_all_bytes<T>(
    mut file: File,
    content: Vec<u8>,
    t: T,
) -> GeorgeResult<Arc<RwLock<T>>> {
    match file.write_all(content.as_slice()) {
        Ok(()) => Ok(Arc::new(RwLock::new(t))),
        Err(err) => Err(err_string(err.to_string())),
    }
}

/// 在指定文件中指定位置后覆盖数据
pub fn write_file_seek_u8s(mut file: File, seek: u64, content: &[u8]) -> GeorgeResult<()> {
    match file.seek(SeekFrom::Start(seek)) {
        Ok(_s) => match file.write_all(content) {
            Ok(()) => Ok(()),
            Err(err) => Err(err_string(err.to_string())),
        },
        Err(err) => Err(err_string(err.to_string())),
    }
}

/// 在指定文件中写入数据
///
/// force 如果已存在，是否删除重写
pub fn write(filepath: String, content: Vec<u8>, force: bool) -> GeorgeResult<Vec<u8>> {
    match write_bytes(filepath, content.clone(), force) {
        Ok(()) => Ok(content),
        Err(err) => Err(err_str_enhance("write_bytes", err.to_string())),
    }
}
