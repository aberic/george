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
use crate::vectors::{Vector, VectorHandler};
use std::fs::{read, read_to_string, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

pub trait FilerNormal {
    /// 获取读`File`
    fn reader<P: AsRef<Path>>(filepath: P) -> GeorgeResult<File>;
    /// 获取写`File`
    fn writer<P: AsRef<Path>>(filepath: P) -> GeorgeResult<File>;
    /// 获取追加写`File`
    fn appender<P: AsRef<Path>>(filepath: P) -> GeorgeResult<File>;
    /// 获取读写`File`
    fn reader_writer<P: AsRef<Path>>(filepath: P) -> GeorgeResult<File>;
    /// 获取读和追加写`File`
    fn reader_appender<P: AsRef<Path>>(filepath: P) -> GeorgeResult<File>;
}

pub trait FilerHandler: Sized {
    /// 判断文件是否存在
    fn exist<P: AsRef<Path>>(filepath: P) -> bool;
    /// 创建新文件
    fn touch<P: AsRef<Path>>(filepath: P) -> GeorgeResult<()>;
    /// 尝试创建新文件，如果存在则返回成功，如果不存在则新建
    fn try_touch<P: AsRef<Path>>(filepath: P) -> GeorgeResult<()>;
    /// 删除文件，如果不存在该文件则直接返回成功
    fn rm<P: AsRef<Path>>(filepath: P) -> GeorgeResult<()>;
    /// 指定路径下文件夹名称
    fn name<P: AsRef<Path>>(filepath: P) -> GeorgeResult<String>;
    /// 拷贝`from`文件至`to`目录下
    fn cp<P: AsRef<Path>>(file_from_path: P, file_to_path: P) -> GeorgeResult<()>;
    /// 移动`from`文件至`to`目录下
    fn mv<P: AsRef<Path>>(file_from_path: P, file_to_path: P) -> GeorgeResult<()>;
    /// 获取path目录的绝对路径
    ///
    /// 如果存在且为文件夹则报错
    fn absolute<P: AsRef<Path>>(filepath: P) -> GeorgeResult<String>;
}

pub trait FilerExecutor<T>: Sized {
    /// 向`File`中追加`content`
    fn appends(file: File, content: T) -> GeorgeResult<()>;
    /// 将`content`在指定`seek`处写入
    fn write_seeks(file: File, seek: u64, content: T) -> GeorgeResult<()>;
}

pub trait FilerWriter<T>: Sized {
    /// 向file_obj(filepath/file)中写入content，如果file_obj不存在则报错
    fn write<P: AsRef<Path>>(filepath: P, content: T) -> GeorgeResult<usize>;
    /// 向file_obj(filepath/file)中写入content，如果file_obj不存在则新建
    fn write_force<P: AsRef<Path>>(filepath: P, content: T) -> GeorgeResult<usize>;
    /// 向file_obj(filepath/file)中追加写content，如果file_obj不存在则报错
    fn append<P: AsRef<Path>>(filepath: P, content: T) -> GeorgeResult<()>;
    /// 向file_obj(filepath/file)中追加写content，如果file_obj不存在则新建
    fn append_force<P: AsRef<Path>>(filepath: P, content: T) -> GeorgeResult<()>;
    fn write_seek<P: AsRef<Path>>(filepath: P, seek: u64, content: T) -> GeorgeResult<()>;
    /// 向file_obj(filepath/file)中写入content，如果file_obj不存在则报错
    fn write_file(file: File, content: T) -> GeorgeResult<usize>;
    /// 向file_obj(filepath/file)中写入content，如果file_obj不存在则新建
    fn write_file_force(file: File, content: T) -> GeorgeResult<usize>;
    /// 向file_obj(filepath/file)中追加写content，如果file_obj不存在则报错
    fn append_file(file: File, content: T) -> GeorgeResult<()>;
    /// 向file_obj(filepath/file)中追加写content，如果file_obj不存在则新建
    fn append_file_force(file: File, content: T) -> GeorgeResult<()>;
    fn write_file_seek(file: File, seek: u64, content: T) -> GeorgeResult<()>;
}

pub trait FilerWriterPath<T>: Sized {}

pub trait FilerReader: Sized {
    fn read<P: AsRef<Path>>(filepath: P) -> GeorgeResult<String>;
    fn read_bytes<P: AsRef<Path>>(filepath: P) -> GeorgeResult<Vec<u8>>;
    fn read_sub<P: AsRef<Path>>(filepath: P, start: u64, last: usize) -> GeorgeResult<Vec<u8>>;
    fn read_sub_allow_none<P: AsRef<Path>>(
        filepath: P,
        start: u64,
        last: usize,
    ) -> GeorgeResult<Vec<u8>>;
    fn len<P: AsRef<Path>>(filepath: P) -> GeorgeResult<u64>;
    fn read_file(file: File) -> GeorgeResult<String>;
    fn read_file_bytes(file: File) -> GeorgeResult<Vec<u8>>;
    fn read_file_sub(file: File, start: u64, last: usize) -> GeorgeResult<Vec<u8>>;
    fn read_file_sub_allow_none(file: File, start: u64, last: usize) -> GeorgeResult<Vec<u8>>;
    fn len_file(file: File) -> GeorgeResult<u64>;
}

pub struct Filer;

impl FilerNormal for Filer {
    fn reader<P: AsRef<Path>>(filepath: P) -> GeorgeResult<File> {
        r_file(filepath)
    }

    fn writer<P: AsRef<Path>>(filepath: P) -> GeorgeResult<File> {
        w_file(filepath)
    }

    fn appender<P: AsRef<Path>>(filepath: P) -> GeorgeResult<File> {
        a_file(filepath)
    }

    fn reader_writer<P: AsRef<Path>>(filepath: P) -> GeorgeResult<File> {
        rw_file(filepath)
    }

    fn reader_appender<P: AsRef<Path>>(filepath: P) -> GeorgeResult<File> {
        ra_file(filepath)
    }
}

impl FilerHandler for Filer {
    fn exist<P: AsRef<Path>>(filepath: P) -> bool {
        file_exist(&filepath)
    }

    fn touch<P: AsRef<Path>>(filepath: P) -> GeorgeResult<()> {
        file_touch(&filepath)
    }

    fn try_touch<P: AsRef<Path>>(filepath: P) -> GeorgeResult<()> {
        file_try_touch(filepath)
    }

    fn rm<P: AsRef<Path>>(filepath: P) -> GeorgeResult<()> {
        file_remove(filepath)
    }

    fn name<P: AsRef<Path>>(filepath: P) -> GeorgeResult<String> {
        file_last_name(filepath)
    }

    fn cp<P: AsRef<Path>>(file_from_path: P, file_to_path: P) -> GeorgeResult<()> {
        file_copy(file_from_path, file_to_path)
    }

    fn mv<P: AsRef<Path>>(file_from_path: P, file_to_path: P) -> GeorgeResult<()> {
        file_move(file_from_path, file_to_path)
    }

    fn absolute<P: AsRef<Path>>(filepath: P) -> GeorgeResult<String> {
        file_absolute(filepath)
    }
}

impl FilerExecutor<&[u8]> for Filer {
    fn appends(file: File, content: &[u8]) -> GeorgeResult<()> {
        file_append(file, content)
    }

    fn write_seeks(file: File, seek: u64, content: &[u8]) -> GeorgeResult<()> {
        file_write_seek(file, seek, content)
    }
}

impl FilerExecutor<Vec<u8>> for Filer {
    fn appends(file: File, content: Vec<u8>) -> GeorgeResult<()> {
        file_append(file, content.as_slice())
    }

    fn write_seeks(file: File, seek: u64, content: Vec<u8>) -> GeorgeResult<()> {
        file_write_seek(file, seek, content.as_slice())
    }
}

impl FilerExecutor<String> for Filer {
    fn appends(file: File, content: String) -> GeorgeResult<()> {
        file_append(file, content.as_bytes())
    }

    fn write_seeks(file: File, seek: u64, content: String) -> GeorgeResult<()> {
        file_write_seek(file, seek, content.as_bytes())
    }
}

impl FilerExecutor<&str> for Filer {
    fn appends(file: File, content: &str) -> GeorgeResult<()> {
        file_append(file, content.as_bytes())
    }

    fn write_seeks(file: File, seek: u64, content: &str) -> GeorgeResult<()> {
        file_write_seek(file, seek, content.as_bytes())
    }
}

impl FilerWriter<&[u8]> for Filer {
    fn write<P: AsRef<Path>>(filepath: P, content: &[u8]) -> GeorgeResult<usize> {
        filepath_write(filepath, content)
    }

    fn write_force<P: AsRef<Path>>(filepath: P, content: &[u8]) -> GeorgeResult<usize> {
        filepath_write_force(filepath, content)
    }

    fn append<P: AsRef<Path>>(filepath: P, content: &[u8]) -> GeorgeResult<()> {
        filepath_append(filepath, content)
    }

    fn append_force<P: AsRef<Path>>(filepath: P, content: &[u8]) -> GeorgeResult<()> {
        filepath_append_force(filepath, content)
    }

    fn write_seek<P: AsRef<Path>>(filepath: P, seek: u64, content: &[u8]) -> GeorgeResult<()> {
        filepath_write_seek(filepath, seek, content)
    }

    fn write_file(file: File, content: &[u8]) -> GeorgeResult<usize> {
        file_write(file, content)
    }

    fn write_file_force(file: File, content: &[u8]) -> GeorgeResult<usize> {
        file_write(file, content)
    }

    fn append_file(file: File, content: &[u8]) -> GeorgeResult<()> {
        file_append(file, content)
    }

    fn append_file_force(file: File, content: &[u8]) -> GeorgeResult<()> {
        file_append(file, content)
    }

    fn write_file_seek(file: File, seek: u64, content: &[u8]) -> GeorgeResult<()> {
        file_write_seek(file, seek, content)
    }
}

impl FilerWriter<Vec<u8>> for Filer {
    fn write<P: AsRef<Path>>(filepath: P, content: Vec<u8>) -> GeorgeResult<usize> {
        filepath_write(filepath, content.as_slice())
    }

    fn write_force<P: AsRef<Path>>(filepath: P, content: Vec<u8>) -> GeorgeResult<usize> {
        filepath_write_force(filepath, content.as_slice())
    }

    fn append<P: AsRef<Path>>(filepath: P, content: Vec<u8>) -> GeorgeResult<()> {
        filepath_append(filepath, content.as_slice())
    }

    fn append_force<P: AsRef<Path>>(filepath: P, content: Vec<u8>) -> GeorgeResult<()> {
        filepath_append_force(filepath, content.as_slice())
    }

    fn write_seek<P: AsRef<Path>>(filepath: P, seek: u64, content: Vec<u8>) -> GeorgeResult<()> {
        filepath_write_seek(filepath, seek, content.as_slice())
    }

    fn write_file(file: File, content: Vec<u8>) -> GeorgeResult<usize> {
        file_write(file, content.as_slice())
    }

    fn write_file_force(file: File, content: Vec<u8>) -> GeorgeResult<usize> {
        file_write(file, content.as_slice())
    }

    fn append_file(file: File, content: Vec<u8>) -> GeorgeResult<()> {
        file_append(file, content.as_slice())
    }

    fn append_file_force(file: File, content: Vec<u8>) -> GeorgeResult<()> {
        file_append(file, content.as_slice())
    }

    fn write_file_seek(file: File, seek: u64, content: Vec<u8>) -> GeorgeResult<()> {
        file_write_seek(file, seek, content.as_slice())
    }
}

impl FilerWriter<String> for Filer {
    fn write<P: AsRef<Path>>(filepath: P, content: String) -> GeorgeResult<usize> {
        filepath_write(filepath, content.as_bytes())
    }

    fn write_force<P: AsRef<Path>>(filepath: P, content: String) -> GeorgeResult<usize> {
        filepath_write_force(filepath, content.as_bytes())
    }

    fn append<P: AsRef<Path>>(filepath: P, content: String) -> GeorgeResult<()> {
        filepath_append(filepath, content.as_bytes())
    }

    fn append_force<P: AsRef<Path>>(filepath: P, content: String) -> GeorgeResult<()> {
        filepath_append_force(filepath, content.as_bytes())
    }

    fn write_seek<P: AsRef<Path>>(filepath: P, seek: u64, content: String) -> GeorgeResult<()> {
        filepath_write_seek(filepath, seek, content.as_bytes())
    }

    fn write_file(file: File, content: String) -> GeorgeResult<usize> {
        file_write(file, content.as_bytes())
    }

    fn write_file_force(file: File, content: String) -> GeorgeResult<usize> {
        file_write(file, content.as_bytes())
    }

    fn append_file(file: File, content: String) -> GeorgeResult<()> {
        file_append(file, content.as_bytes())
    }

    fn append_file_force(file: File, content: String) -> GeorgeResult<()> {
        file_append(file, content.as_bytes())
    }

    fn write_file_seek(file: File, seek: u64, content: String) -> GeorgeResult<()> {
        file_write_seek(file, seek, content.as_bytes())
    }
}

impl FilerWriter<&str> for Filer {
    fn write<P: AsRef<Path>>(filepath: P, content: &str) -> GeorgeResult<usize> {
        filepath_write(filepath, content.as_bytes())
    }

    fn write_force<P: AsRef<Path>>(filepath: P, content: &str) -> GeorgeResult<usize> {
        filepath_write_force(filepath, content.as_bytes())
    }

    fn append<P: AsRef<Path>>(filepath: P, content: &str) -> GeorgeResult<()> {
        filepath_append(filepath, content.as_bytes())
    }

    fn append_force<P: AsRef<Path>>(filepath: P, content: &str) -> GeorgeResult<()> {
        filepath_append_force(filepath, content.as_bytes())
    }

    fn write_seek<P: AsRef<Path>>(filepath: P, seek: u64, content: &str) -> GeorgeResult<()> {
        filepath_write_seek(filepath, seek, content.as_bytes())
    }

    fn write_file(file: File, content: &str) -> GeorgeResult<usize> {
        file_write(file, content.as_bytes())
    }

    fn write_file_force(file: File, content: &str) -> GeorgeResult<usize> {
        file_write(file, content.as_bytes())
    }

    fn append_file(file: File, content: &str) -> GeorgeResult<()> {
        file_append(file, content.as_bytes())
    }

    fn append_file_force(file: File, content: &str) -> GeorgeResult<()> {
        file_append(file, content.as_bytes())
    }

    fn write_file_seek(file: File, seek: u64, content: &str) -> GeorgeResult<()> {
        file_write_seek(file, seek, content.as_bytes())
    }
}

impl FilerReader for Filer {
    fn read<P: AsRef<Path>>(filepath: P) -> GeorgeResult<String> {
        filepath_read(filepath)
    }

    fn read_bytes<P: AsRef<Path>>(filepath: P) -> GeorgeResult<Vec<u8>> {
        filepath_reads(filepath)
    }

    fn read_sub<P: AsRef<Path>>(filepath: P, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
        filepath_read_sub(filepath, start, last)
    }

    fn read_sub_allow_none<P: AsRef<Path>>(
        filepath: P,
        start: u64,
        last: usize,
    ) -> GeorgeResult<Vec<u8>> {
        filepath_read_sub_allow_none(filepath, start, last)
    }

    fn len<P: AsRef<Path>>(filepath: P) -> GeorgeResult<u64> {
        filepath_len(filepath)
    }

    fn read_file(file: File) -> GeorgeResult<String> {
        file_read(file)
    }

    fn read_file_bytes(file: File) -> GeorgeResult<Vec<u8>> {
        file_read_bytes(file)
    }

    fn read_file_sub(file: File, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
        file_read_sub(file, start, last)
    }

    fn read_file_sub_allow_none(file: File, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
        file_read_sub_allow_none(file, start, last)
    }

    fn len_file(file: File) -> GeorgeResult<u64> {
        file_len(file)
    }
}

/// 判断文件是否存在，如果为文件夹则报错，否则返回判断结果
fn file_exist<P: AsRef<Path>>(filepath: P) -> bool {
    let path_check = Path::new(filepath.as_ref().as_os_str());
    if path_check.exists() {
        if path_check.is_dir() {
            false
        } else {
            true
        }
    } else {
        false
    }
}

/// 创建文件
fn file_touch<P: AsRef<Path>>(filepath: P) -> GeorgeResult<()> {
    if file_exist(&filepath) {
        Err(err_string(format!(
            "file {} already exist!",
            filepath.as_ref().to_str().unwrap()
        )))
    } else {
        let path_check = Path::new(filepath.as_ref().as_os_str());
        match path_check.parent() {
            Some(p) => {
                if !p.exists() {
                    Dir::mk_uncheck(p.to_str().unwrap())?
                }
            }
            None => {}
        }
        match File::create(&filepath) {
            Ok(_) => Ok(()),
            Err(err) => Err(err_strings(
                format!("path {} touch error: ", filepath.as_ref().to_str().unwrap()),
                err,
            )),
        }
    }
}

/// 尝试创建文件，如果存在该文件，则复用该文件
fn file_try_touch<P: AsRef<Path>>(filepath: P) -> GeorgeResult<()> {
    if file_exist(&filepath) {
        Ok(())
    } else {
        let path_check = Path::new(filepath.as_ref().as_os_str());
        match path_check.parent() {
            Some(p) => {
                if !p.exists() {
                    Dir::mk_uncheck(p.to_str().unwrap())?
                }
            }
            None => {}
        }
        match File::create(&filepath) {
            Ok(_) => Ok(()),
            Err(err) => Err(err_strings(
                format!("path {} touch error: ", filepath.as_ref().to_str().unwrap()),
                err,
            )),
        }
    }
}

/// 删除目录
fn file_remove<P: AsRef<Path>>(filepath: P) -> GeorgeResult<()> {
    if file_exist(&filepath) {
        match fs::remove_file(&filepath) {
            Ok(()) => Ok(()),
            Err(err) => Err(err_strings(
                format!(
                    "path {} remove error: ",
                    filepath.as_ref().to_str().unwrap()
                ),
                err,
            )),
        }
    } else {
        Ok(())
    }
}

/// 获取path目录的绝对路径
///
/// 如果存在且为文件夹则报错
fn file_absolute<P: AsRef<Path>>(filepath: P) -> GeorgeResult<String> {
    if file_exist(&filepath) {
        match fs::canonicalize(&filepath) {
            Ok(path_buf) => Ok(path_buf.to_str().unwrap().to_string()),
            Err(err) => Err(err_strings(
                format!(
                    "fs {} canonicalize error: ",
                    filepath.as_ref().to_str().unwrap()
                ),
                err,
            )),
        }
    } else {
        Err(err_string(format!(
            "file {} doesn't exist!",
            filepath.as_ref().to_str().unwrap()
        )))
    }
}

/// 判断目录是否存在，如果目录为文件夹则报错，否则返回判断结果
fn file_last_name<P: AsRef<Path>>(filepath: P) -> GeorgeResult<String> {
    if file_exist(&filepath) {
        Ok(Path::new(filepath.as_ref().as_os_str())
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string())
    } else {
        Err(err_string(format!(
            "path {} does't exist!",
            filepath.as_ref().to_str().unwrap()
        )))
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
fn file_copy<P: AsRef<Path>>(file_from_path: P, file_to_path: P) -> GeorgeResult<()> {
    match fs::copy(&file_from_path, &file_to_path) {
        Ok(_) => Ok(()),
        Err(err) => Err(err_strings(
            format!(
                "copy {} to {} error: ",
                file_from_path.as_ref().to_str().unwrap(),
                file_to_path.as_ref().to_str().unwrap()
            ),
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
fn file_move<P: AsRef<Path>>(file_from_path: P, file_to_path: P) -> GeorgeResult<()> {
    file_copy(&file_from_path, &file_to_path)?;
    file_remove(file_from_path)
}

/// 在指定文件中写入数据
///
/// 返回写入的字节长度
pub fn file_write(mut file: File, content: &[u8]) -> GeorgeResult<usize> {
    match file.write(content) {
        Ok(size) => Ok(size),
        Err(err) => Err(err_strs("file write all", err)),
    }
}

/// 在指定文件中写入数据
///
/// 返回写入的字节长度
pub fn filepath_write<P: AsRef<Path>>(filepath: P, content: &[u8]) -> GeorgeResult<usize> {
    match OpenOptions::new().write(true).open(filepath) {
        Ok(file) => file_write(file, content),
        Err(err) => Err(err_strs("file open when write", err)),
    }
}

/// 在指定文件中写入数据
///
/// 返回写入的字节长度
pub fn filepath_write_force<P: AsRef<Path>>(filepath: P, content: &[u8]) -> GeorgeResult<usize> {
    if !file_exist(&filepath) {
        file_touch(&filepath)?;
    }
    filepath_write(filepath, content)
}

/// 在指定文件中追加数据
fn filepath_append<P: AsRef<Path>>(filepath: P, content: &[u8]) -> GeorgeResult<()> {
    match OpenOptions::new().append(true).open(filepath) {
        Ok(file) => file_append(file, content),
        Err(err) => Err(err_strs("file open when append", err)),
    }
}

/// 在指定文件中追加数据
fn filepath_append_force<P: AsRef<Path>>(filepath: P, content: &[u8]) -> GeorgeResult<()> {
    if !file_exist(&filepath) {
        file_touch(&filepath)?;
    }
    filepath_append(filepath, content)
}

/// 在指定文件中追加数据
fn file_append(mut file: File, content: &[u8]) -> GeorgeResult<()> {
    match file.write_all(content) {
        Ok(()) => Ok(()),
        Err(err) => Err(err_strs("file write all", err)),
    }
}

/// 在指定文件中指定位置后覆盖数据
fn filepath_write_seek<P: AsRef<Path>>(filepath: P, seek: u64, content: &[u8]) -> GeorgeResult<()> {
    match OpenOptions::new().write(true).open(filepath) {
        Ok(file) => file_write_seek(file, seek, content),
        Err(err) => Err(err_strs("file open when write seek", err)),
    }
}

/// 在指定文件中指定位置后覆盖数据
fn file_write_seek(mut file: File, seek: u64, content: &[u8]) -> GeorgeResult<()> {
    match file.seek(SeekFrom::Start(seek)) {
        Ok(_s) => match file.write_all(content) {
            Ok(()) => Ok(()),
            Err(err) => Err(err_strs("file write all", err)),
        },
        Err(err) => Err(err_strs("file open when write seek", err)),
    }
}

fn filepath_read<P: AsRef<Path>>(filepath: P) -> GeorgeResult<String> {
    match read_to_string(filepath) {
        Ok(s) => Ok(s),
        Err(err) => Err(err_strs("file read to string", err)),
    }
}

fn file_read(mut file: File) -> GeorgeResult<String> {
    let mut string = String::with_capacity(initial_buffer_size(&file));
    match file.read_to_string(&mut string) {
        Ok(_usize) => Ok(string),
        Err(err) => Err(err_strs("file read to string", err)),
    }
}

/// Indicates how large a buffer to pre-allocate before reading the entire file.
fn initial_buffer_size(file: &File) -> usize {
    // Allocate one extra byte so the buffer doesn't need to grow before the
    // final `read` call at the end of the file.  Don't worry about `usize`
    // overflow because reading will fail regardless in that case.
    file.metadata().map(|m| m.len() as usize + 1).unwrap_or(0)
}

fn filepath_reads<P: AsRef<Path>>(filepath: P) -> GeorgeResult<Vec<u8>> {
    match read(filepath) {
        Ok(u8s) => Ok(u8s),
        Err(err) => Err(err_strs("file read to string", err)),
    }
}

fn file_read_bytes(mut file: File) -> GeorgeResult<Vec<u8>> {
    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Ok(_usize) => Ok(buffer),
        Err(err) => Err(err_strs("file read to string", err)),
    }
}

/// 读取文件部分内容，从start开始，一直持续读取last长度
fn filepath_read_sub<P: AsRef<Path>>(
    filepath: P,
    start: u64,
    last: usize,
) -> GeorgeResult<Vec<u8>> {
    match File::open(filepath) {
        Ok(file) => file_read_sub(file, start, last),
        Err(err) => Err(err_string(err.to_string())),
    }
}

/// 读取文件部分内容，从start开始，一直持续读取last长度
fn filepath_read_sub_allow_none<P: AsRef<Path>>(
    filepath: P,
    start: u64,
    last: usize,
) -> GeorgeResult<Vec<u8>> {
    match File::open(filepath) {
        Ok(file) => file_read_sub_allow_none(file, start, last),
        Err(err) => Err(err_string(err.to_string())),
    }
}

fn filepath_len<P: AsRef<Path>>(filepath: P) -> GeorgeResult<u64> {
    file_len(r_file(filepath)?)
}

fn file_len(mut file: File) -> GeorgeResult<u64> {
    match file.seek(SeekFrom::End(0)) {
        Ok(res) => Ok(res),
        Err(err) => Err(err_string(err.to_string())),
    }
}

/// 读取文件部分内容，从start开始，一直持续读取last长度
fn file_read_sub(mut file: File, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
    let file_len = file.seek(SeekFrom::End(0)).unwrap();
    if file_len < start + last as u64 {
        Err(err_string(format!(
            "read sub file read failed! file_len is {} while start {} and last {}",
            file_len, start, last
        )))
    } else {
        file_read_subs_helper(file, start, last)
    }
}

/// 读取文件部分内容，从start开始，一直持续读取last长度
///
/// 如果无法读取该内容，即预期读取坐标超过实际内容长度，则返回期望读取长度的空字节数组
fn file_read_sub_allow_none(mut file: File, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
    let file_len = file.seek(SeekFrom::End(0)).unwrap();
    if file_len < start + last as u64 {
        Ok(Vector::create_empty_bytes(last))
    } else {
        file_read_subs_helper(file, start, last)
    }
}

/// 读取文件部分内容，从start开始，一直持续读取last长度
fn file_read_subs_helper(mut file: File, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
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

fn rw_file<P: AsRef<Path>>(filepath: P) -> GeorgeResult<File> {
    match OpenOptions::new().read(true).write(true).open(filepath) {
        Ok(file) => Ok(file),
        Err(err) => Err(err_strs("open read&write file", err)),
    }
}

fn ra_file<P: AsRef<Path>>(filepath: P) -> GeorgeResult<File> {
    match OpenOptions::new().read(true).append(true).open(filepath) {
        Ok(file) => Ok(file),
        Err(err) => Err(err_strs("open read&write file", err)),
    }
}

fn r_file<P: AsRef<Path>>(filepath: P) -> GeorgeResult<File> {
    match OpenOptions::new().read(true).open(filepath) {
        Ok(file) => Ok(file),
        Err(err) => Err(err_strs("open read file", err)),
    }
}

fn w_file<P: AsRef<Path>>(filepath: P) -> GeorgeResult<File> {
    match OpenOptions::new().write(true).open(filepath) {
        Ok(file) => Ok(file),
        Err(err) => Err(err_strs("open write file", err)),
    }
}

fn a_file<P: AsRef<Path>>(filepath: P) -> GeorgeResult<File> {
    match OpenOptions::new().append(true).open(filepath) {
        Ok(file) => Ok(file),
        Err(err) => Err(err_strs("open append file", err)),
    }
}
