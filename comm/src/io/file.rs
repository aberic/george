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

use std::fs;
use std::fs::File;
use std::path::Path;

use crate::errors::entrances::err_string;
use crate::errors::entrances::GeorgeResult;

/// 创建目录
pub fn create_dir_str(dir_path: &str) -> GeorgeResult<()> {
    create_dir(dir_path.to_string())
}

/// 创建目录
pub fn create_dir(dir_path: String) -> GeorgeResult<()> {
    // println!("create filepath = {}", dir_path);
    let path = Path::new(&dir_path);
    if path.exists() && path.is_dir() {
        // println!("file path exists = {}", dir_path);
        Ok(())
    } else {
        // println!("file create path = {}", dir_path);
        match fs::create_dir_all(dir_path) {
            Ok(_) => Ok(()),
            Err(err) => Err(err_string(err.to_string())),
        }
    }
}

/// 创建文件
///
/// filepath 文件路径
///
/// force 如果存在旧文件，是否删除并新建
pub fn create_file_str(filepath: &str, force: bool) -> GeorgeResult<File> {
    create_file(filepath.to_string(), force)
}

/// 创建文件
///
/// filepath 文件路径
///
/// force 如果存在旧文件，是否删除并新建
pub fn create_file(filepath: String, force: bool) -> GeorgeResult<File> {
    // println!("create filepath = {}", filepath);
    let path = Path::new(&filepath);
    match path.parent() {
        Some(p) => {
            if !p.exists() {
                match fs::create_dir_all(p) {
                    Err(err) => {
                        return Err(err_string(err.to_string()));
                    }
                    _ => {}
                }
            }
        }
        None => {}
    }
    if path.exists() && path.is_file() {
        // println!("file exists = {}", filepath);
        if force {
            create_file_real(filepath)
        } else {
            match File::open(filepath) {
                Ok(file) => Ok(file),
                Err(err) => Err(err_string(err.to_string())),
            }
        }
    } else {
        // println!("file path = {}", filepath);
        create_file_real(filepath)
    }
}

fn create_file_real(filepath: String) -> GeorgeResult<File> {
    match File::create(filepath) {
        Ok(file) => Ok(file),
        Err(err) => Err(err_string(err.to_string())),
    }
}
