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
use std::io::{Seek, SeekFrom};
use std::sync::{Arc, RwLock};

use comm::errors::entrances::GeorgeResult;
use comm::errors::entrances::{err_string, err_strs};
use comm::io::file::{Filer, FilerExecutor, FilerHandler};

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
    pub fn append(&mut self, content: Vec<u8>) -> GeorgeResult<u64> {
        let file_append = self.file_append.clone();
        let mut file_write = file_append.write().unwrap();
        match file_write.seek(SeekFrom::End(0)) {
            Ok(seek_end_before) => match file_write.try_clone() {
                Ok(f) => {
                    Filer::appends(f, content.clone())?;
                    Ok(seek_end_before)
                }
                Err(err) => Err(err_strs("write append file try clone2", err)),
            },
            Err(err) => Err(err_strs("write append file try clone1", err)),
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
