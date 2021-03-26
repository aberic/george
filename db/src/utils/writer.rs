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

use comm::errors::entrances::GeorgeResult;
use comm::errors::entrances::{err_string, err_strs};
use comm::io::file::{Filer, FilerExecutor, FilerHandler, FilerNormal};

#[derive(Debug, Clone)]
pub struct Filed {
    filepath: String,
    reader: Arc<File>,
    writer: Arc<RwLock<File>>,
    appender: Arc<RwLock<File>>,
}

impl Filed {
    pub fn create_self(filepath: String) -> GeorgeResult<Filed> {
        Filer::touch(filepath.clone())?;
        Filed::recovery_self(filepath)
    }
    pub fn create(filepath: String) -> GeorgeResult<Arc<RwLock<Filed>>> {
        Filer::touch(filepath.clone())?;
        Filed::recovery(filepath)
    }
    pub fn recovery_self(filepath: String) -> GeorgeResult<Filed> {
        Ok(Filed {
            filepath: filepath.clone(),
            reader: Arc::new(Filer::reader(filepath.clone())?),
            writer: Arc::new(RwLock::new(Filer::writer(filepath.clone())?)),
            appender: Arc::new(RwLock::new(Filer::appender(filepath)?)),
        })
    }
    pub fn recovery(filepath: String) -> GeorgeResult<Arc<RwLock<Filed>>> {
        Ok(Arc::new(RwLock::new(Filed {
            filepath: filepath.clone(),
            reader: Arc::new(Filer::reader(filepath.clone())?),
            writer: Arc::new(RwLock::new(Filer::appender(filepath.clone())?)),
            appender: Arc::new(RwLock::new(Filer::appender(filepath)?)),
        })))
    }
    pub fn read(&self, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
        match self.reader.clone().try_clone() {
            Ok(file) => Filer::read_subs(file, start, last),
            Err(err) => Err(err_strs("filed read", err)),
        }
    }
    pub fn write(&mut self, seek: u64, content: Vec<u8>) -> GeorgeResult<()> {
        let writer = self.writer.clone();
        let mut file_write = writer.write().unwrap();
        match file_write.seek(SeekFrom::Start(seek)) {
            Ok(_s) => match file_write.write_all(content.as_slice()) {
                Ok(()) => Ok(()),
                Err(err) => Err(err_strs("filed write while write all", err)),
            },
            Err(err) => Err(err_strs("filed write while seek", err)),
        }
    }
    pub fn append(&mut self, content: Vec<u8>) -> GeorgeResult<u64> {
        let appender = self.appender.clone();
        let mut file_append = appender.write().unwrap();
        match file_append.seek(SeekFrom::End(0)) {
            Ok(seek_end_before) => match file_append.try_clone() {
                Ok(f) => {
                    Filer::appends(f, content.clone())?;
                    Ok(seek_end_before)
                }
                Err(err) => Err(err_strs("write append file try clone2", err)),
            },
            Err(err) => Err(err_strs("write append file try clone1", err)),
        }
    }
    fn filepath(&self) -> String {
        self.filepath.clone()
    }
    /// 整理归档
    ///
    /// archive_file_path 归档路径
    pub fn archive(&mut self, archive_filepath: String) -> GeorgeResult<()> {
        Filer::mv(self.filepath(), archive_filepath)?;
        Filer::touch(self.filepath())?;
        self.appender = obtain_append_file(self.filepath())?;
        Ok(())
    }
}

/// 根据文件路径获取该文件追加写入的写对象
pub fn obtain_append_file(filepath: String) -> GeorgeResult<Arc<RwLock<File>>> {
    Ok(Arc::new(RwLock::new(Filer::appender(filepath)?)))
}
