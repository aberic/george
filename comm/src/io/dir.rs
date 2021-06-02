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

pub trait DirHandler: Sized {
    fn exist<P: AsRef<Path>>(path: P) -> GeorgeResult<bool>;

    fn mk<P: AsRef<Path>>(path: P) -> GeorgeResult<()>;

    fn mk_uncheck<P: AsRef<Path>>(path: P) -> GeorgeResult<()>;

    fn rm<P: AsRef<Path>>(path: P) -> GeorgeResult<()>;

    /// 指定路径下目录文件夹名称
    fn name<P: AsRef<Path>>(path: P) -> GeorgeResult<String>;

    /// 拷贝`from`目录下内容至`to`目录下
    ///
    /// force 是否强制更新`to`目录为空目录
    fn cp<P: AsRef<Path>>(from: P, to: P, force: bool) -> GeorgeResult<()>;

    /// 移动`from`目录下内容至`to`目录下
    ///
    /// force 是否强制更新`to`目录为空目录
    fn mv<P: AsRef<Path>>(from: P, to: P, force: bool) -> GeorgeResult<()>;

    /// 重命名
    fn rename<P: AsRef<Path>>(from: P, to: P) -> GeorgeResult<()>;
}

impl DirHandler for Dir {
    fn exist<P: AsRef<Path>>(path: P) -> GeorgeResult<bool> {
        dir_exist(path)
    }

    fn mk<P: AsRef<Path>>(path: P) -> GeorgeResult<()> {
        dir_create(path)
    }

    fn mk_uncheck<P: AsRef<Path>>(path: P) -> GeorgeResult<()> {
        dir_create_uncheck(path)
    }

    fn rm<P: AsRef<Path>>(path: P) -> GeorgeResult<()> {
        dir_remove(path)
    }

    fn name<P: AsRef<Path>>(path: P) -> GeorgeResult<String> {
        dir_last_name(path)
    }

    fn cp<P: AsRef<Path>>(from: P, to: P, force: bool) -> GeorgeResult<()> {
        let from_dir_name = dir_last_name(&from)?;
        let to = to
            .as_ref()
            .to_str()
            .unwrap()
            .to_string()
            .add("/")
            .add(&from_dir_name);
        dir_copy(from.as_ref().to_str().unwrap().to_string(), to, force)
    }

    fn mv<P: AsRef<Path>>(from: P, to: P, force: bool) -> GeorgeResult<()> {
        let from_dir_name = dir_last_name(&from)?;
        let to = to
            .as_ref()
            .to_str()
            .unwrap()
            .to_string()
            .add("/")
            .add(&from_dir_name);
        dir_move(from.as_ref().to_str().unwrap().to_string(), to, force)
    }

    fn rename<P: AsRef<Path>>(from: P, to: P) -> GeorgeResult<()> {
        dir_rename(from, to)
    }
}

/// 判断目录是否存在，如果目录为文件则报错，否则返回判断结果
fn dir_exist<P: AsRef<Path>>(path: P) -> GeorgeResult<bool> {
    let path_check = Path::new(path.as_ref());
    if path_check.exists() {
        if path_check.is_file() {
            Err(Errs::string(format!(
                "path {} is file",
                path.as_ref().to_str().unwrap()
            )))
        } else {
            Ok(true)
        }
    } else {
        Ok(false)
    }
}

/// 创建目录
fn dir_create<P: AsRef<Path>>(path: P) -> GeorgeResult<()> {
    if dir_exist(&path)? {
        Err(Errs::dir_exist_error())
    } else {
        match fs::create_dir_all(&path) {
            Ok(_) => Ok(()),
            Err(err) => Err(Errs::strings(
                format!("path {} create error: ", path.as_ref().to_str().unwrap()),
                err,
            )),
        }
    }
}

/// 创建目录
fn dir_create_uncheck<P: AsRef<Path>>(path: P) -> GeorgeResult<()> {
    if dir_exist(&path)? {
        Ok(())
    } else {
        match fs::create_dir_all(&path) {
            Ok(_) => Ok(()),
            Err(err) => Err(Errs::strings(
                format!("path {} create error: ", path.as_ref().to_str().unwrap()),
                err,
            )),
        }
    }
}

/// 删除目录
fn dir_remove<P: AsRef<Path>>(path: P) -> GeorgeResult<()> {
    match fs::remove_dir_all(&path) {
        Ok(()) => Ok(()),
        Err(err) => Err(Errs::strings(
            format!("path {} remove error: ", path.as_ref().to_str().unwrap()),
            err,
        )),
    }
}

/// 获取path目录的绝对路径
///
/// 如果存在且为文件则报错
///
/// 如果存在并且是目录，则根据force来判断是否强制清空该目录
///
/// force 是否强制更新该目录为空目录
fn dir_absolute<P: AsRef<Path>>(path: P, force: bool) -> GeorgeResult<String> {
    if dir_exist(&path)? {
        if force {
            match fs::remove_dir_all(&path) {
                Ok(()) => dir_create_uncheck(&path)?,
                Err(err) => {
                    return Err(Errs::strings(
                        format!("remove dir {} error: ", path.as_ref().to_str().unwrap()),
                        err,
                    ))
                }
            }
        }
    } else {
        dir_create_uncheck(&path)?;
    }
    match fs::canonicalize(&path) {
        Ok(path_buf) => Ok(path_buf.to_str().unwrap().to_string()),
        Err(err) => Err(Errs::strings(
            format!(
                "fs {} canonicalize error: ",
                path.as_ref().to_str().unwrap()
            ),
            err,
        )),
    }
}

/// 判断目录是否存在，如果目录为文件夹则报错，否则返回判断结果
fn dir_last_name<P: AsRef<Path>>(path: P) -> GeorgeResult<String> {
    if dir_exist(&path)? {
        Ok(Path::new(path.as_ref())
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string())
    } else {
        Err(Errs::string(format!(
            "path {} does't exist!",
            path.as_ref().to_str().unwrap()
        )))
    }
}

/// 拷贝`from`目录至`to`目录下
///
/// force 是否强制更新`to`目录为空目录
fn dir_copy<P: AsRef<Path>>(from: P, to: P, force: bool) -> GeorgeResult<()> {
    let from_absolute_path_str = dir_absolute(&from, false)?;
    let to_absolute_path_str = dir_absolute(&to, force)?;
    if to_absolute_path_str.contains(&from_absolute_path_str) {
        Err(Errs::string(format!(
            "to path {} is a sub project of path {}",
            to_absolute_path_str, from_absolute_path_str
        )))
    } else {
        match fs::read_dir(from) {
            Ok(read_dir) => {
                // 遍历database目录下文件
                for path in read_dir {
                    match path {
                        // 所有目录文件被默认为view根目录
                        Ok(dir) => {
                            let dir_path = dir.path();
                            let now_from_path = dir_path.to_str().unwrap();
                            let dir_name = dir.file_name().to_string_lossy().to_string();
                            let now_to_path = to
                                .as_ref()
                                .to_str()
                                .unwrap()
                                .to_string()
                                .add("/")
                                .add(&dir_name);
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
fn dir_move<P: AsRef<Path>>(from: P, to: P, force: bool) -> GeorgeResult<()> {
    dir_copy(&from, &to, force)?;
    dir_remove(from)
}

fn dir_rename<P: AsRef<Path>>(from: P, to: P) -> GeorgeResult<()> {
    match std::fs::rename(from, to) {
        Ok(_) => Ok(()),
        Err(err) => Err(Errs::strs("dir rename failed", err.to_string())),
    }
}
