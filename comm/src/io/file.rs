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

use crate::errors::entrances::{err_string, err_strings};
use crate::errors::entrances::{err_strs, GeorgeResult};
use crate::io::dir::{Dir, DirHandler};
use std::fs::{read_to_string, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

pub trait FilerNormal {
    fn read_subs(file: File, start: u64, last: usize) -> GeorgeResult<Vec<u8>>;
    fn reader(filepath: String) -> GeorgeResult<File>;
    fn writer(filepath: String) -> GeorgeResult<File>;
    fn appender(filepath: String) -> GeorgeResult<File>;
    fn reader_writer(filepath: String) -> GeorgeResult<File>;
    fn reader_appender(filepath: String) -> GeorgeResult<File>;
}

pub trait FilerHandler<T>: Sized {
    fn exist(path: T) -> GeorgeResult<bool>;
    fn touch(path: T) -> GeorgeResult<()>;
    fn try_touch(path: T) -> GeorgeResult<()>;
    fn rm(path: T) -> GeorgeResult<()>;
    /// 指定路径下文件夹名称
    fn name(path: T) -> GeorgeResult<String>;
    /// 拷贝`from`文件至`to`目录下
    fn cp(file_from_path: T, file_to_path: T) -> GeorgeResult<()>;
    /// 移动`from`文件至`to`目录下
    fn mv(file_from_path: T, file_to_path: T) -> GeorgeResult<()>;
}

pub trait FilerExecutor<T>: Sized {
    fn appends(file: File, content: T) -> GeorgeResult<()>;
    fn write_seeks(file: File, seek: u64, content: T) -> GeorgeResult<()>;
}

pub trait FilerWriter<M, N>: Sized {
    fn write(filepath: M, content: N) -> GeorgeResult<usize>;
    fn append(filepath: M, content: N) -> GeorgeResult<()>;
    fn write_seek(filepath: M, seek: u64, content: N) -> GeorgeResult<()>;
}

pub trait FilerReader<T>: Sized {
    fn read(filepath: T) -> GeorgeResult<String>;
    fn read_sub(filepath: T, start: u64, last: usize) -> GeorgeResult<Vec<u8>>;
}

pub struct Filer {}

impl FilerNormal for Filer {
    fn read_subs(file: File, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
        read_subs(file, start, last)
    }
    fn reader(filepath: String) -> GeorgeResult<File> {
        r_file(filepath)
    }
    fn writer(filepath: String) -> GeorgeResult<File> {
        w_file(filepath)
    }
    fn appender(filepath: String) -> GeorgeResult<File> {
        a_file(filepath)
    }
    fn reader_writer(filepath: String) -> GeorgeResult<File> {
        rw_file(filepath)
    }
    fn reader_appender(filepath: String) -> GeorgeResult<File> {
        ra_file(filepath)
    }
}

impl FilerHandler<String> for Filer {
    fn exist(path: String) -> GeorgeResult<bool> {
        file_exist(path)
    }
    fn touch(path: String) -> GeorgeResult<()> {
        file_touch(path)
    }
    fn try_touch(path: String) -> GeorgeResult<()> {
        file_try_touch(path)
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
    fn try_touch(path: &str) -> GeorgeResult<()> {
        file_try_touch(path.to_string())
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

impl FilerExecutor<&[u8]> for Filer {
    fn appends(file: File, content: &[u8]) -> GeorgeResult<()> {
        file_appends(file, content)
    }
    fn write_seeks(file: File, seek: u64, content: &[u8]) -> GeorgeResult<()> {
        file_write_seeks(file, seek, content)
    }
}

impl FilerExecutor<Vec<u8>> for Filer {
    fn appends(file: File, content: Vec<u8>) -> GeorgeResult<()> {
        file_appends(file, content.as_slice())
    }
    fn write_seeks(file: File, seek: u64, content: Vec<u8>) -> GeorgeResult<()> {
        file_write_seeks(file, seek, content.as_slice())
    }
}

impl FilerExecutor<String> for Filer {
    fn appends(file: File, content: String) -> GeorgeResult<()> {
        file_appends(file, content.as_bytes())
    }
    fn write_seeks(file: File, seek: u64, content: String) -> GeorgeResult<()> {
        file_write_seeks(file, seek, content.as_bytes())
    }
}

impl FilerExecutor<&str> for Filer {
    fn appends(file: File, content: &str) -> GeorgeResult<()> {
        file_appends(file, content.as_bytes())
    }
    fn write_seeks(file: File, seek: u64, content: &str) -> GeorgeResult<()> {
        file_write_seeks(file, seek, content.as_bytes())
    }
}

impl FilerWriter<String, &[u8]> for Filer {
    fn write(filepath: String, content: &[u8]) -> GeorgeResult<usize> {
        file_write(filepath, content)
    }
    fn append(filepath: String, content: &[u8]) -> GeorgeResult<()> {
        file_append(filepath, content)
    }
    fn write_seek(filepath: String, seek: u64, content: &[u8]) -> GeorgeResult<()> {
        file_write_seek(filepath, seek, content)
    }
}

impl FilerWriter<String, Vec<u8>> for Filer {
    fn write(filepath: String, content: Vec<u8>) -> GeorgeResult<usize> {
        file_write(filepath, content.as_slice())
    }
    fn append(filepath: String, content: Vec<u8>) -> GeorgeResult<()> {
        file_append(filepath, content.as_slice())
    }
    fn write_seek(filepath: String, seek: u64, content: Vec<u8>) -> GeorgeResult<()> {
        file_write_seek(filepath, seek, content.as_slice())
    }
}

impl FilerWriter<String, String> for Filer {
    fn write(filepath: String, content: String) -> GeorgeResult<usize> {
        file_write(filepath, content.as_bytes())
    }
    fn append(filepath: String, content: String) -> GeorgeResult<()> {
        file_append(filepath, content.as_bytes())
    }
    fn write_seek(filepath: String, seek: u64, content: String) -> GeorgeResult<()> {
        file_write_seek(filepath, seek, content.as_bytes())
    }
}

impl FilerWriter<String, &str> for Filer {
    fn write(filepath: String, content: &str) -> GeorgeResult<usize> {
        file_write(filepath, content.as_bytes())
    }
    fn append(filepath: String, content: &str) -> GeorgeResult<()> {
        file_append(filepath, content.as_bytes())
    }
    fn write_seek(filepath: String, seek: u64, content: &str) -> GeorgeResult<()> {
        file_write_seek(filepath, seek, content.as_bytes())
    }
}

impl FilerWriter<&str, &[u8]> for Filer {
    fn write(filepath: &str, content: &[u8]) -> GeorgeResult<usize> {
        file_write(filepath.to_string(), content)
    }
    fn append(filepath: &str, content: &[u8]) -> GeorgeResult<()> {
        file_append(filepath.to_string(), content)
    }
    fn write_seek(filepath: &str, seek: u64, content: &[u8]) -> GeorgeResult<()> {
        file_write_seek(filepath.to_string(), seek, content)
    }
}

impl FilerWriter<&str, Vec<u8>> for Filer {
    fn write(filepath: &str, content: Vec<u8>) -> GeorgeResult<usize> {
        file_write(filepath.to_string(), content.as_slice())
    }
    fn append(filepath: &str, content: Vec<u8>) -> GeorgeResult<()> {
        file_append(filepath.to_string(), content.as_slice())
    }
    fn write_seek(filepath: &str, seek: u64, content: Vec<u8>) -> GeorgeResult<()> {
        file_write_seek(filepath.to_string(), seek, content.as_slice())
    }
}

impl FilerWriter<&str, String> for Filer {
    fn write(filepath: &str, content: String) -> GeorgeResult<usize> {
        file_write(filepath.to_string(), content.as_bytes())
    }
    fn append(filepath: &str, content: String) -> GeorgeResult<()> {
        file_append(filepath.to_string(), content.as_bytes())
    }
    fn write_seek(filepath: &str, seek: u64, content: String) -> GeorgeResult<()> {
        file_write_seek(filepath.to_string(), seek, content.as_bytes())
    }
}

impl FilerWriter<&str, &str> for Filer {
    fn write(filepath: &str, content: &str) -> GeorgeResult<usize> {
        file_write(filepath.to_string(), content.as_bytes())
    }
    fn append(filepath: &str, content: &str) -> GeorgeResult<()> {
        file_append(filepath.to_string(), content.as_bytes())
    }
    fn write_seek(filepath: &str, seek: u64, content: &str) -> GeorgeResult<()> {
        file_write_seek(filepath.to_string(), seek, content.as_bytes())
    }
}

impl FilerReader<String> for Filer {
    fn read(filepath: String) -> GeorgeResult<String> {
        file_read(filepath)
    }
    fn read_sub(filepath: String, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
        file_read_sub(filepath, start, last)
    }
}

impl FilerReader<&str> for Filer {
    fn read(filepath: &str) -> GeorgeResult<String> {
        file_read(filepath.to_string())
    }
    fn read_sub(filepath: &str, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
        file_read_sub(filepath.to_string(), start, last)
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

/// 尝试创建文件，如果存在该文件，则复用该文件
fn file_try_touch(path: String) -> GeorgeResult<()> {
    if file_exist(path.clone())? {
        Ok(())
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

/// 在指定文件中写入数据
///
/// 返回写入的字节长度
pub fn file_write(filepath: String, content: &[u8]) -> GeorgeResult<usize> {
    if !file_exist(filepath.clone())? {
        file_touch(filepath.clone())?;
    }
    match OpenOptions::new().write(true).open(filepath) {
        Ok(mut file) => match file.write(content) {
            Ok(size) => Ok(size),
            Err(err) => Err(err_strs("file write all", err)),
        },
        Err(err) => Err(err_strs("file open when write", err)),
    }
}

/// 在指定文件中追加数据
fn file_append(filepath: String, content: &[u8]) -> GeorgeResult<()> {
    match OpenOptions::new().append(true).open(filepath) {
        Ok(file) => file_appends(file, content),
        Err(err) => Err(err_strs("file open when append", err)),
    }
}

/// 在指定文件中追加数据
fn file_appends(mut file: File, content: &[u8]) -> GeorgeResult<()> {
    match file.write_all(content) {
        Ok(()) => Ok(()),
        Err(err) => Err(err_strs("file write all", err)),
    }
}

/// 在指定文件中指定位置后覆盖数据
fn file_write_seek(filepath: String, seek: u64, content: &[u8]) -> GeorgeResult<()> {
    match OpenOptions::new().write(true).open(filepath) {
        Ok(file) => file_write_seeks(file, seek, content),
        Err(err) => Err(err_strs("file open when write seek", err)),
    }
}

/// 在指定文件中指定位置后覆盖数据
fn file_write_seeks(mut file: File, seek: u64, content: &[u8]) -> GeorgeResult<()> {
    match file.seek(SeekFrom::Start(seek)) {
        Ok(_s) => match file.write_all(content) {
            Ok(()) => Ok(()),
            Err(err) => Err(err_strs("file write all", err)),
        },
        Err(err) => Err(err_strs("file open when write seek", err)),
    }
}

fn file_read(filepath: String) -> GeorgeResult<String> {
    match read_to_string(filepath) {
        Ok(s) => Ok(s),
        Err(err) => Err(err_strs("file read to string", err)),
    }
}

/// 读取文件部分内容，从start开始，一直持续读取last长度
fn file_read_sub(filepath: String, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
    match File::open(filepath) {
        Ok(file) => read_subs(file, start, last),
        Err(err) => Err(err_string(err.to_string())),
    }
}

/// 读取文件部分内容，从start开始，一直持续读取last长度
fn read_subs(mut file: File, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
    let file_len = file.try_clone().unwrap().seek(SeekFrom::End(0)).unwrap();
    if file_len < start + last as u64 {
        Ok(vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
    } else {
        match file.seek(SeekFrom::Start(start)) {
            Ok(_u) => {
                if last.eq(&8) {
                    let mut buffer = [0u8; 8];
                    let mut buf: Vec<u8> = vec![];
                    let mut position = 0;
                    while position < last {
                        match file.read(&mut buffer) {
                            Ok(_u) => {
                                if last - position >= 8 {
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
                } else {
                    let mut buffer = [0u8; 1024];
                    let mut buf: Vec<u8> = vec![];
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
            }
            Err(err) => Err(err_string(err.to_string())),
        }
    }
}

fn rw_file(filepath: String) -> GeorgeResult<File> {
    match OpenOptions::new().read(true).write(true).open(filepath) {
        Ok(file) => Ok(file),
        Err(err) => Err(err_strs("open read&write file", err)),
    }
}

fn ra_file(filepath: String) -> GeorgeResult<File> {
    match OpenOptions::new().read(true).append(true).open(filepath) {
        Ok(file) => Ok(file),
        Err(err) => Err(err_strs("open read&write file", err)),
    }
}

fn r_file(filepath: String) -> GeorgeResult<File> {
    match OpenOptions::new().read(true).open(filepath) {
        Ok(file) => Ok(file),
        Err(err) => Err(err_strs("open read file", err)),
    }
}

fn w_file(filepath: String) -> GeorgeResult<File> {
    match OpenOptions::new().write(true).open(filepath) {
        Ok(file) => Ok(file),
        Err(err) => Err(err_strs("open write file", err)),
    }
}

fn a_file(filepath: String) -> GeorgeResult<File> {
    match OpenOptions::new().append(true).open(filepath) {
        Ok(file) => Ok(file),
        Err(err) => Err(err_strs("open append file", err)),
    }
}
