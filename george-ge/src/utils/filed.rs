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

use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;
use std::sync::{Arc, RwLock};

use george_comm::errors::Errs;
use george_comm::errors::GeorgeResult;
use george_comm::io::file::{FilerExecutor, FilerHandler, FilerNormal, FilerReader};
use george_comm::io::Filer;

use crate::utils::Filed;

impl Filed {
    pub fn create<P: AsRef<Path>>(filepath: P) -> GeorgeResult<Filed> {
        Filer::touch(&filepath)?;
        Filed::recovery(filepath)
    }

    pub fn recovery<P: AsRef<Path>>(filepath: P) -> GeorgeResult<Filed> {
        let writer = Filer::writer(&filepath)?;
        let appender = Filer::appender(&filepath)?;
        Ok(Filed {
            filepath: Filer::absolute(filepath)?,
            exec: Arc::new(RwLock::new(FiledExec { writer, appender })),
        })
    }

    /// 获取文件长度
    pub fn len(&self) -> GeorgeResult<u64> {
        self.exec.read().unwrap().len(self.filepath())
    }

    /// 读取文件部分内容，从start开始，一直持续读取last长度
    pub fn read(&self, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
        self.exec.read().unwrap().read(self.filepath(), start, last)
    }

    /// 读取文件部分内容，从start开始，一直持续读取last长度
    ///
    /// 如果无法读取该内容，即预期读取坐标超过实际内容长度，则返回期望读取长度的空字节数
    pub fn read_allow_none(&self, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
        self.exec
            .read()
            .unwrap()
            .read_allow_none(self.filepath(), start, last)
    }

    /// 在指定坐标后写入content
    pub fn write(&self, seek: u64, content: Vec<u8>) -> GeorgeResult<()> {
        self.exec.write().unwrap().write(seek, content)
    }

    /// 向File中追加content
    pub fn append(&self, content: Vec<u8>) -> GeorgeResult<u64> {
        self.exec.write().unwrap().append(content)
    }

    pub fn filepath(&self) -> String {
        self.filepath.clone()
    }

    /// 整理归档
    ///
    /// archive_file_path 归档路径
    pub fn archive(&self, archive_filepath: String) -> GeorgeResult<()> {
        Filer::mv(self.filepath(), archive_filepath)?;
        Filer::touch(self.filepath())?;
        self.exec.write().unwrap().recovery(self.filepath())
    }
}

#[derive(Debug)]
pub(super) struct FiledExec {
    writer: File,
    appender: File,
}

impl FiledExec {
    fn recovery<P: AsRef<Path>>(&mut self, filepath: P) -> GeorgeResult<()> {
        self.writer = Filer::writer(&filepath)?;
        self.appender = Filer::appender(filepath)?;
        Ok(())
    }

    /// 获取文件长度
    fn len<P: AsRef<Path>>(&self, filepath: P) -> GeorgeResult<u64> {
        Filer::len(filepath)
    }

    /// 读取文件部分内容，从start开始，一直持续读取last长度
    fn read<P: AsRef<Path>>(&self, filepath: P, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
        Filer::read_sub(filepath, start, last)
    }

    /// 读取文件部分内容，从start开始，一直持续读取last长度
    ///
    /// 如果无法读取该内容，即预期读取坐标超过实际内容长度，则返回期望读取长度的空字节数
    fn read_allow_none<P: AsRef<Path>>(
        &self,
        filepath: P,
        start: u64,
        last: usize,
    ) -> GeorgeResult<Vec<u8>> {
        Filer::read_sub_allow_none(filepath, start, last)
    }

    /// 在指定坐标后写入content
    fn write(&self, seek: u64, content: Vec<u8>) -> GeorgeResult<()> {
        match self.writer.try_clone() {
            Ok(mut file) => match file.seek(SeekFrom::Start(seek)) {
                Ok(_s) => match file.write_all(content.as_slice()) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(Errs::strs("filed write while write all", err)),
                },
                Err(err) => Err(Errs::strs("filed write while seek", err)),
            },
            Err(err) => Err(Errs::strs("filed read", err)),
        }
    }

    /// 向File中追加content
    fn append(&self, content: Vec<u8>) -> GeorgeResult<u64> {
        match self.appender.try_clone() {
            Ok(mut file) => match file.seek(SeekFrom::End(0)) {
                Ok(seek_end_before) => {
                    Filer::appends(file, content.clone())?;
                    Ok(seek_end_before)
                }
                Err(err) => Err(Errs::strs("write append file try clone1", err)),
            },
            Err(err) => Err(Errs::strs("filed read", err)),
        }
    }
}

/// 根据文件路径获取该文件追加写入的写对象
pub fn obtain_append_file<P: AsRef<Path>>(filepath: P) -> GeorgeResult<Arc<RwLock<File>>> {
    Ok(Arc::new(RwLock::new(Filer::appender(filepath)?)))
}
