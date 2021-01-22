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

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use std::sync::{Arc, RwLock};

use once_cell::sync::Lazy;

use crate::utils::store::Tag;
use comm::errors::children::NoneError;
use comm::errors::entrances::GeorgeResult;
use comm::errors::entrances::{err_string, GeorgeError};
use comm::io::file::{Filer, FilerHandler};
use comm::io::writer::write_file_append_bytes;

#[derive(Debug, Clone)]
pub struct Filed {
    file_path: String,
    file_append: Arc<RwLock<File>>,
}

impl Filed {
    pub fn create(file_path: String) -> GeorgeResult<Arc<RwLock<Filed>>> {
        Filer::touch(file_path.clone())?;
        Filed::recovery(file_path)
    }
    pub fn recovery(file_path: String) -> GeorgeResult<Arc<RwLock<Filed>>> {
        let file_append = obtain_write_append_file(file_path.clone())?;
        return Ok(Arc::new(RwLock::new(Filed {
            file_path,
            file_append,
        })));
    }
    pub fn append(&mut self, file_path: String, content: Vec<u8>) -> GeorgeResult<u64> {
        let file_append = self.file_append.clone();
        let mut file_write = file_append.write().unwrap();
        match file_write.seek(SeekFrom::End(0)) {
            Ok(seek_end_before) => {
                match write_file_append_bytes(file_write.try_clone().unwrap(), content.clone()) {
                    Ok(()) => Ok(seek_end_before),
                    Err(_err) => {
                        self.file_append = obtain_write_append_file(file_path)?;
                        let file_write_again = self.file_append.write().unwrap();
                        write_file_append_bytes(file_write_again.try_clone().unwrap(), content)?;
                        Ok(seek_end_before)
                    }
                }
            }
            Err(_err) => {
                self.file_append = obtain_write_append_file(file_path)?;
                let mut file_write_again = self.file_append.write().unwrap();
                let seek_end_before_again = file_write_again.seek(SeekFrom::End(0)).unwrap();
                write_file_append_bytes(file_write_again.try_clone().unwrap(), content)?;
                Ok(seek_end_before_again)
            }
        }
    }
    fn file_path(&self) -> String {
        self.file_path.clone()
    }
    /// 整理归档
    ///
    /// archive_file_path 归档路径
    pub fn archive(&mut self, archive_file_path: String) -> GeorgeResult<()> {
        Filer::mv(self.file_path(), archive_file_path)?;
        Filer::touch(self.file_path())?;
        self.file_append = obtain_write_append_file(self.file_path())?;
        Ok(())
    }
}

/// 根据文件路径获取该文件追加写入的写对象
pub fn obtain_write_append_file(file_path: String) -> GeorgeResult<Arc<RwLock<File>>> {
    match OpenOptions::new().append(true).open(file_path) {
        Ok(file) => Ok(Arc::new(RwLock::new(file))),
        Err(err) => Err(err_string(err.to_string())),
    }
}
