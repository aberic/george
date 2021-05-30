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

use std::fs;
use std::ops::Add;
use std::path::Path;

use crate::errors::{Errs, GeorgeResult};
use crate::io::Dir;

pub trait DirHandler<T>: Sized {
    fn exist(_: T) -> GeorgeResult<bool>;

    fn mk(_: T) -> GeorgeResult<()>;

    fn mk_uncheck(_: T) -> GeorgeResult<()>;

    fn rm(_: T) -> GeorgeResult<()>;

    /// 指定路径下目录文件夹名称
    fn name(_: T) -> GeorgeResult<String>;

    /// 拷贝`from`目录下内容至`to`目录下
    ///
    /// force 是否强制更新`to`目录为空目录
    fn cp(_: T, _: T, force: bool) -> GeorgeResult<()>;

    /// 移动`from`目录下内容至`to`目录下
    ///
    /// force 是否强制更新`to`目录为空目录
    fn mv(_: T, _: T, force: bool) -> GeorgeResult<()>;
}

impl DirHandler<String> for Dir {
    fn exist(path: String) -> GeorgeResult<bool> {
        dir_exist(path)
    }

    fn mk(path: String) -> GeorgeResult<()> {
        dir_create(path)
    }

    fn mk_uncheck(path: String) -> GeorgeResult<()> {
        dir_create_uncheck(path)
    }

    fn rm(path: String) -> GeorgeResult<()> {
        dir_remove(path)
    }

    fn name(path: String) -> GeorgeResult<String> {
        dir_last_name(path)
    }

    fn cp(from_path: String, to_path: String, force: bool) -> GeorgeResult<()> {
        let from_dir_name = dir_last_name(from_path.clone())?;
        let to_path = to_path.add("/").add(&from_dir_name);
        dir_copy(from_path, to_path, force)
    }

    fn mv(from_path: String, to_path: String, force: bool) -> GeorgeResult<()> {
        let from_dir_name = dir_last_name(from_path.clone())?;
        let to_path = to_path.add("/").add(&from_dir_name);
        dir_move(from_path, to_path, force)
    }
}

impl DirHandler<&str> for Dir {
    fn exist(path: &str) -> GeorgeResult<bool> {
        dir_exist(path.to_string())
    }

    fn mk(path: &str) -> GeorgeResult<()> {
        dir_create(path.to_string())
    }

    fn mk_uncheck(path: &str) -> GeorgeResult<()> {
        dir_create_uncheck(path.to_string())
    }

    fn rm(path: &str) -> GeorgeResult<()> {
        dir_remove(path.to_string())
    }

    fn name(path: &str) -> GeorgeResult<String> {
        dir_last_name(path.to_string())
    }

    fn cp(from_path: &str, to_path: &str, force: bool) -> GeorgeResult<()> {
        let from_dir_name = dir_last_name(from_path.to_string())?;
        let to_path = to_path.to_string().add("/").add(&from_dir_name);
        dir_copy(from_path.to_string(), to_path, force)
    }

    fn mv(from_path: &str, to_path: &str, force: bool) -> GeorgeResult<()> {
        let from_dir_name = dir_last_name(from_path.to_string())?;
        let to_path = to_path.to_string().add("/").add(&from_dir_name);
        dir_move(from_path.to_string(), to_path, force)
    }
}

/// 判断目录是否存在，如果目录为文件则报错，否则返回判断结果
fn dir_exist(path: String) -> GeorgeResult<bool> {
    let path_check = Path::new(&path);
    if path_check.exists() {
        if path_check.is_file() {
            Err(Errs::string(format!("path {} is file", path)))
        } else {
            Ok(true)
        }
    } else {
        Ok(false)
    }
}

/// 创建目录
fn dir_create(path: String) -> GeorgeResult<()> {
    if dir_exist(path.clone())? {
        Err(Errs::dir_exist_error())
    } else {
        match fs::create_dir_all(path.clone()) {
            Ok(_) => Ok(()),
            Err(err) => Err(Errs::strings(format!("path {} create error: ", path), err)),
        }
    }
}

/// 创建目录
fn dir_create_uncheck(path: String) -> GeorgeResult<()> {
    if dir_exist(path.clone())? {
        Ok(())
    } else {
        match fs::create_dir_all(path.clone()) {
            Ok(_) => Ok(()),
            Err(err) => Err(Errs::strings(format!("path {} create error: ", path), err)),
        }
    }
}

/// 删除目录
fn dir_remove(path: String) -> GeorgeResult<()> {
    match fs::remove_dir_all(path.clone()) {
        Ok(()) => Ok(()),
        Err(err) => Err(Errs::strings(format!("path {} remove error: ", path), err)),
    }
}

/// 获取path目录的绝对路径
///
/// 如果存在且为文件则报错
///
/// 如果存在并且是目录，则根据force来判断是否强制清空该目录
///
/// force 是否强制更新该目录为空目录
fn dir_absolute(path: String, force: bool) -> GeorgeResult<String> {
    if dir_exist(path.clone())? {
        if force {
            match fs::remove_dir_all(path.clone()) {
                Ok(()) => dir_create_uncheck(path.clone())?,
                Err(err) => return Err(Errs::strings(format!("remove dir {} error: ", path), err)),
            }
        }
    } else {
        dir_create_uncheck(path.clone())?;
    }
    match fs::canonicalize(path.clone()) {
        Ok(path_buf) => Ok(path_buf.to_str().unwrap().to_string()),
        Err(err) => Err(Errs::strings(
            format!("fs {} canonicalize error: ", path),
            err,
        )),
    }
}

/// 判断目录是否存在，如果目录为文件夹则报错，否则返回判断结果
fn dir_last_name(path: String) -> GeorgeResult<String> {
    if dir_exist(path.clone())? {
        Ok(Path::new(&path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string())
    } else {
        Err(Errs::string(format!("path {} does't exist!", path)))
    }
}

/// 拷贝`from`目录至`to`目录下
///
/// force 是否强制更新`to`目录为空目录
fn dir_copy(from_path: String, to_path: String, force: bool) -> GeorgeResult<()> {
    let from_absolute_path_str = dir_absolute(from_path.clone(), false)?;
    let to_absolute_path_str = dir_absolute(to_path.clone(), force)?;
    if to_absolute_path_str.contains(&from_absolute_path_str) {
        Err(Errs::string(format!(
            "to path {} is a sub project of path {}",
            to_absolute_path_str, from_absolute_path_str
        )))
    } else {
        match fs::read_dir(from_path) {
            Ok(read_dir) => {
                // 遍历database目录下文件
                for path in read_dir {
                    match path {
                        // 所有目录文件被默认为view根目录
                        Ok(dir) => {
                            let dir_path = dir.path();
                            let now_from_path = dir_path.to_str().unwrap();
                            let dir_name = dir.file_name().to_string_lossy().to_string();
                            let now_to_path = to_path.clone().add("/").add(&dir_name);
                            if dir.path().is_dir() {
                                match dir_create_uncheck(now_to_path.clone()) {
                                    Ok(()) => {
                                        dir_copy(now_from_path.to_string(), now_to_path, true)?
                                    }
                                    Err(err) => {
                                        return Err(Errs::strings(
                                            format!("create dir {} error: ", now_to_path),
                                            err,
                                        ));
                                    }
                                }
                            } else if dir.path().is_file() {
                                match fs::copy(now_from_path.clone(), now_to_path.clone()) {
                                    Err(err) => {
                                        return Err(Errs::strings(
                                            format!(
                                                "file copy from {} to {} error: ",
                                                now_from_path, now_to_path
                                            ),
                                            err,
                                        ));
                                    }
                                    _ => {}
                                }
                            } else {
                                return Err(Errs::str("unsupported path type error!"));
                            }
                        }
                        Err(err) => {
                            return Err(Errs::strs("dir entry error: ", err));
                        }
                    }
                }
                Ok(())
            }
            Err(err) => return Err(Errs::strs("read dir error: ", err)),
        }
    }
}

/// 移动`from`目录至`to`目录下
///
/// force 是否强制更新`to`目录为空目录
fn dir_move(from_path: String, to_path: String, force: bool) -> GeorgeResult<()> {
    dir_copy(from_path.clone(), to_path, force)?;
    dir_remove(from_path)
}
