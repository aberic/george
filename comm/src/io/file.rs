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
use std::path::Path;

use crate::errors::entrances::GeorgeResult;
use crate::errors::entrances::{err_string, err_strings};
use crate::io::dir::{Dir, DirHandler};
use std::fs::File;

pub trait FilerHandler<T>: Sized {
    fn exist(_: T) -> GeorgeResult<bool>;
    fn touch(_: T) -> GeorgeResult<()>;
    fn rm(_: T) -> GeorgeResult<()>;
    /// 指定路径下文件夹名称
    fn name(_: T) -> GeorgeResult<String>;
    /// 拷贝`from`文件至`to`目录下
    fn cp(_: T, _: T) -> GeorgeResult<()>;
    /// 移动`from`文件至`to`目录下
    fn mv(_: T, _: T) -> GeorgeResult<()>;
}

pub struct Filer {}

impl FilerHandler<String> for Filer {
    fn exist(path: String) -> GeorgeResult<bool> {
        file_exist(path)
    }
    fn touch(path: String) -> GeorgeResult<()> {
        file_touch(path)
    }
    fn rm(path: String) -> GeorgeResult<()> {
        file_remove(path)
    }
    fn name(path: String) -> GeorgeResult<String> {
        file_last_name(path)
    }
    fn cp(file_from_path: String, file_to_path: String) -> GeorgeResult<()> {
        file_copy(file_from_path, file_to_path)
    }
    fn mv(file_from_path: String, file_to_path: String) -> GeorgeResult<()> {
        file_move(file_from_path, file_to_path)
    }
}

impl FilerHandler<&str> for Filer {
    fn exist(path: &str) -> GeorgeResult<bool> {
        file_exist(path.to_string())
    }
    fn touch(path: &str) -> GeorgeResult<()> {
        file_touch(path.to_string())
    }
    fn rm(path: &str) -> GeorgeResult<()> {
        file_remove(path.to_string())
    }
    fn name(path: &str) -> GeorgeResult<String> {
        file_last_name(path.to_string())
    }
    fn cp(file_from_path: &str, file_to_path: &str) -> GeorgeResult<()> {
        file_copy(file_from_path.to_string(), file_to_path.to_string())
    }
    fn mv(file_from_path: &str, file_to_path: &str) -> GeorgeResult<()> {
        file_move(file_from_path.to_string(), file_to_path.to_string())
    }
}

/// 判断文件是否存在，如果为文件夹则报错，否则返回判断结果
fn file_exist(path: String) -> GeorgeResult<bool> {
    let path_check = Path::new(&path);
    if path_check.exists() {
        if path_check.is_dir() {
            Err(err_string(format!("path {} is dir", path)))
        } else {
            Ok(true)
        }
    } else {
        Ok(false)
    }
}

/// 创建文件
fn file_touch(path: String) -> GeorgeResult<()> {
    if file_exist(path.clone())? {
        Err(err_string(format!("file {} already exist!", path)))
    } else {
        let path_check = Path::new(&path);
        match path_check.parent() {
            Some(p) => {
                if !p.exists() {
                    Dir::mk(p.to_str().unwrap())?
                }
            }
            None => {}
        }
        match File::create(path.clone()) {
            Ok(_) => Ok(()),
            Err(err) => Err(err_strings(format!("path {} touch error: ", path), err)),
        }
    }
}

/// 删除目录
fn file_remove(path: String) -> GeorgeResult<()> {
    if file_exist(path.clone())? {
        match fs::remove_file(path.clone()) {
            Ok(()) => Ok(()),
            Err(err) => Err(err_strings(format!("path {} remove error: ", path), err)),
        }
    } else {
        Ok(())
    }
}

/// 获取path目录的绝对路径
///
/// 如果存在且为文件夹则报错
fn file_absolute(path: String) -> GeorgeResult<String> {
    if file_exist(path.clone())? {
        match fs::canonicalize(path.clone()) {
            Ok(path_buf) => Ok(path_buf.to_str().unwrap().to_string()),
            Err(err) => Err(err_strings(
                format!("fs {} canonicalize error: ", path),
                err,
            )),
        }
    } else {
        Err(err_string(format!("file {} doesn't exist!", path)))
    }
}

/// 判断目录是否存在，如果目录为文件夹则报错，否则返回判断结果
fn file_last_name(path: String) -> GeorgeResult<String> {
    if file_exist(path.clone())? {
        Ok(Path::new(&path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string())
    } else {
        Err(err_string(format!("path {} does't exist!", path)))
    }
}

/// 拷贝`from`至`to`
///
/// # Examples
///
/// ```no_run
/// use crate::io::file::{File, FileHandler};
///
/// fn main() -> std::io::Result<()> {
///     File::cp("foo.txt", "bar.txt")?;  // Copy foo.txt to bar.txt
///     Ok(())
/// }
/// ```
fn file_copy(file_from_path: String, file_to_path: String) -> GeorgeResult<()> {
    match fs::copy(file_from_path.clone(), file_to_path.clone()) {
        Ok(_) => Ok(()),
        Err(err) => Err(err_strings(
            format!("copy {} to {} error: ", file_from_path, file_to_path),
            err,
        )),
    }
}

/// 移动`from`至`to`
///
/// # Examples
///
/// ```no_run
/// use crate::io::file::{File, FileHandler};
///
/// fn main() -> std::io::Result<()> {
///     File::mv("foo.txt", "bar.txt")?;  // Copy foo.txt to bar.txt
///     Ok(())
/// }
/// ```
fn file_move(file_from_path: String, file_to_path: String) -> GeorgeResult<()> {
    file_copy(file_from_path.clone(), file_to_path)?;
    file_remove(file_from_path)
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
