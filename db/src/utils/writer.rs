use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use std::sync::{Arc, RwLock};

use once_cell::sync::Lazy;

use comm::errors::children::NoneError;
use comm::errors::entrances::GeorgeResult;
use comm::errors::entrances::{err_string, GeorgeError};

use crate::utils::store::Tag;

/// 视图及索引写对象
pub struct Writer {
    pub views: Arc<RwLock<HashMap<String, Arc<RwLock<File>>>>>,
    pub indexes: Arc<RwLock<HashMap<String, Arc<RwLock<File>>>>>,
}

/// 视图及索引写全局单例对象
pub(crate) static GLOBAL_WRITER: Lazy<Arc<Writer>> = Lazy::new(|| {
    let writer = Writer {
        views: Arc::new(Default::default()),
        indexes: Arc::new(Default::default()),
    };
    Arc::new(writer)
});

impl Writer {
    /// 视图及索引写对象新增视图管理
    pub fn insert_view(&self, view_id: String, view_file_path: String) -> GeorgeResult<()> {
        match OpenOptions::new().append(true).open(view_file_path) {
            Ok(file) => {
                self.views
                    .clone()
                    .write()
                    .unwrap()
                    .insert(view_id, Arc::new(RwLock::new(file)));
                Ok(())
            }
            Err(err) => Err(err_string(err.to_string())),
        }
    }

    /// 视图及索引写对象新增索引管理
    pub fn insert_index(&self, index_id: String, index_file_path: String) -> GeorgeResult<()> {
        match OpenOptions::new().append(true).open(index_file_path) {
            Ok(file) => {
                self.indexes
                    .clone()
                    .write()
                    .unwrap()
                    .insert(index_id, Arc::new(RwLock::new(file)));
                Ok(())
            }
            Err(err) => Err(err_string(err.to_string())),
        }
    }

    /// 获取视图或索引写文件对象
    fn file(&self, tag: Tag, id: String) -> GeorgeResult<Arc<RwLock<File>>> {
        return match tag {
            Tag::View => match self.views.clone().read().unwrap().get(&id) {
                Some(f) => Ok(f.clone()),
                None => Err(GeorgeError::NoneError(NoneError)),
            },
            Tag::Index => match self.indexes.clone().read().unwrap().get(&id) {
                Some(f) => Ok(f.clone()),
                None => Err(GeorgeError::NoneError(NoneError)),
            },
            _ => Err(GeorgeError::NoneError(NoneError)),
        };
    }

    /// 在指定文件中追加数据
    ///
    /// 如果是view，则存储id为“database_id+view_id”<p>
    /// 参考方法`store_view_id(database_id: String, view_id: String) -> String`<p>
    ///
    /// 如果是index，则存储id为“database_id+view_id+index_id”<p>
    /// 参考方法`store_index_id(database_id: String, view_id: String, index_id: String) -> String`
    pub fn write_append_bytes(&self, tag: Tag, id: String, content: Vec<u8>) -> GeorgeResult<u64> {
        self.write_append_u8s(tag, id, content.as_slice())
    }

    /// 在指定文件中追加数据
    ///
    /// 如果是view，则存储id为“database_id+view_id”<p>
    /// 参考方法`store_view_id(database_id: String, view_id: String) -> String`<p>
    ///
    /// 如果是index，则存储id为“database_id+view_id+index_id”<p>
    /// 参考方法`store_index_id(database_id: String, view_id: String, index_id: String) -> String`
    pub fn write_append_u8s(&self, tag: Tag, id: String, content: &[u8]) -> GeorgeResult<u64> {
        match self.file(tag, id) {
            Ok(file_arc) => {
                let file = file_arc.clone();
                let mut file_w = file.write().unwrap();
                // 获取当前文件总长度，并将其作为写的偏移量
                let seek_start = file_w.metadata().unwrap().len();
                match file_w.write_all(content) {
                    Ok(()) => Ok(seek_start),
                    Err(err) => Err(err_string(err.to_string())),
                }
            }
            Err(err) => Err(err),
        }
    }

    /// 在指定文件中指定位置后覆盖数据
    ///
    /// 如果是view，则存储id为“database_id+view_id”<p>
    /// 参考方法`store_view_id(database_id: String, view_id: String) -> String`<p>
    ///
    /// 如果是index，则存储id为“database_id+view_id+index_id”<p>
    /// 参考方法`store_index_id(database_id: String, view_id: String, index_id: String) -> String`
    pub fn write_seek_u8s(
        &self,
        tag: Tag,
        id: String,
        seek: u64,
        content: &[u8],
    ) -> GeorgeResult<()> {
        match self.file(tag, id) {
            Ok(file_arc) => {
                let file = file_arc.clone();
                let mut file_w = file.write().unwrap();
                match file_w.seek(SeekFrom::Start(seek)) {
                    Ok(_s) => match file_w.write_all(content) {
                        Ok(()) => Ok(()),
                        Err(err) => Err(err_string(err.to_string())),
                    },
                    Err(err) => Err(err_string(err.to_string())),
                }
            }
            Err(err) => Err(err),
        }
    }
}
