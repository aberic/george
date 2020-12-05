use std::collections::HashMap;
use std::fs::{read_dir, ReadDir};
use std::sync::{Arc, RwLock};

use chrono::{Duration, Local, NaiveDateTime};

use comm::cryptos::hash::md516;
use comm::errors::children::{DataNoExistError, ViewExistError, ViewNoExistError};
use comm::errors::entrances::GeorgeResult;
use comm::errors::entrances::{err_string, GeorgeError};
use comm::io::file::create_file;

use crate::engine::siam::selector::Expectation;
use crate::engine::traits::TDescription;
use crate::engine::view::View;
use crate::utils::comm::{Category, IndexType, LevelType};
use crate::utils::path::{database_file_path, database_path, view_file_path};
use crate::utils::store::{
    before_content_bytes, head, modify, recovery_before_content, save, FileHeader, Tag,
};
use crate::utils::writer::GLOBAL_WRITER;

/// 数据库
pub(crate) struct Database {
    /// 唯一ID
    id: String,
    /// 名称
    name: String,
    /// 描述
    comment: String,
    /// 创建时间
    create_time: Duration,
    /// 视图索引集合
    views: Arc<RwLock<HashMap<String, Arc<RwLock<View>>>>>,
}

impl TDescription for Database {
    fn description(&mut self) -> Vec<u8> {
        hex::encode(format!(
            "{}/{}/{}/{}",
            self.id,
            self.name,
            self.comment,
            self.create_time.num_nanoseconds().unwrap().to_string(),
        ))
        .into_bytes()
    }

    fn recover(&mut self, description: Vec<u8>) -> GeorgeResult<()> {
        match String::from_utf8(description) {
            Ok(description_str) => match hex::decode(description_str) {
                Ok(vu8) => match String::from_utf8(vu8) {
                    Ok(real) => {
                        let mut split = real.split("/");
                        self.id = split.next().unwrap().to_string();
                        self.name = split.next().unwrap().to_string();
                        self.comment = split.next().unwrap().to_string();
                        self.create_time = Duration::nanoseconds(
                            split.next().unwrap().to_string().parse::<i64>().unwrap(),
                        );
                        // 读取database目录下所有文件
                        match read_dir(database_path(self.id())) {
                            // 恢复views数据
                            Ok(paths) => {
                                self.recovery_views(paths);
                            }
                            Err(err) => {
                                panic!("recovery databases read dir failed! error is {}", err)
                            }
                        }
                        Ok(())
                    }
                    Err(err) => Err(err_string(format!(
                        "recovery database from utf8 failed! error is {}",
                        err
                    ))),
                },
                Err(err) => Err(err_string(format!(
                    "recovery database decode failed! error is {}",
                    err
                ))),
            },
            Err(err) => Err(err_string(err.to_string())),
        }
    }
}

/// 新建数据库
///
/// 具体传参参考如下定义：<p><p>
///
/// ###Params
///
/// id 数据库唯一ID
///
/// name 数据库名称
///
/// comment 数据库描述
fn new_database(name: String, comment: String) -> Database {
    let now: NaiveDateTime = Local::now().naive_local();
    let create_time = Duration::nanoseconds(now.timestamp_nanos());
    let id = md516(name.clone());
    return Database {
        id,
        name,
        comment,
        create_time,
        views: Arc::new(Default::default()),
    };
}

impl Database {
    pub(crate) fn init(name: String, comment: String) -> GeorgeResult<Arc<RwLock<Database>>> {
        let mut db = new_database(name, comment);
        let file = create_file(database_file_path(db.id()), true)?;
        let mut head = head(FileHeader::create(
            Tag::Database,
            Category::Document,
            LevelType::Small,
            IndexType::Siam,
            0x00,
        ));
        let mut description = db.description();
        // 初始化为32 + 6，即head长度加正文描述符长度
        let mut before_description = before_content_bytes(38, description.len() as u16);
        head.append(&mut before_description);
        head.append(&mut description);
        save(Tag::Database, file, head, db.id(), "".to_string(), db)
    }
    pub(crate) fn id(&self) -> String {
        self.id.clone()
    }
    pub(crate) fn name(&self) -> String {
        self.name.clone()
    }
    pub(crate) fn comment(&self) -> String {
        self.comment.clone()
    }
    pub(crate) fn create_time(&self) -> Duration {
        self.create_time
    }
    pub(crate) fn views(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<View>>>>> {
        self.views.clone()
    }
    pub(crate) fn modify(&mut self, name: String) -> GeorgeResult<()> {
        self.name = name;
        let description = self.description();
        modify(database_file_path(self.id()), description)
    }
    pub(crate) fn create(name: String, comment: String) -> Database {
        return new_database(name, comment);
    }
    pub(crate) fn empty() -> Database {
        return Database {
            id: "".to_string(),
            name: "".to_string(),
            comment: "".to_string(),
            create_time: Duration::nanoseconds(1),
            views: Arc::new(Default::default()),
        };
    }
    /// 创建视图
    pub(crate) fn create_view(
        &self,
        name: String,
        comment: String,
        index_type: IndexType,
        category: Category,
        level: LevelType,
    ) -> GeorgeResult<()> {
        if self.exist_view(name.clone()) {
            return Err(GeorgeError::ViewExistError(ViewExistError));
        }
        let view = View::init(
            self.id.clone(),
            name.clone(),
            comment,
            index_type,
            category,
            level,
        )?;
        self.views
            .clone()
            .write()
            .unwrap()
            .insert(name, view.clone());
        Ok(())
    }
    pub(crate) fn insert_view(&self, view_name: String, view: Arc<RwLock<View>>) {
        self.views
            .clone()
            .write()
            .unwrap()
            .insert(view_name, view.clone());
    }
    /// 根据视图name获取视图
    pub(crate) fn view(&self, view_name: String) -> GeorgeResult<Arc<RwLock<View>>> {
        let views = self.views.clone();
        let view_r = views.read().unwrap();
        let res = view_r.get(&view_name);
        match res {
            Some(v) => Ok(v.clone()),
            None => Err(GeorgeError::ViewNoExistError(ViewNoExistError)),
        }
    }
    pub(crate) fn modify_view(
        &self,
        view_old_name: String,
        view_new_name: String,
    ) -> GeorgeResult<()> {
        if !self.exist_view(view_old_name.clone()) {
            return Err(GeorgeError::ViewNoExistError(ViewNoExistError));
        }
        if self.exist_view(view_new_name.clone()) {
            return Err(GeorgeError::ViewExistError(ViewExistError));
        }
        let views = self.views.clone();
        let mut views_w = views.write().unwrap();
        let view = views_w.get(&view_old_name).unwrap().clone();
        views_w.remove(&view_old_name);
        views_w.insert(view_new_name.clone(), view.clone());
        view.clone().write().unwrap().modify(view_new_name.clone())
    }
    pub(crate) fn exist_view(&self, view_name: String) -> bool {
        for res in self.views.clone().read().unwrap().iter() {
            if res.0.eq(&view_name) {
                return true;
            }
        }
        return false;
    }
    /// 在指定视图中创建索引
    ///
    /// 该索引需要定义ID，此外索引所表达的字段组成内容也是必须的，并通过primary判断索引类型，具体传参参考如下定义：<p><p>
    ///
    /// ###Params
    ///
    /// key_structure 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`key_structure`作为索引存入
    ///
    /// primary 是否主键
    pub(crate) fn create_index(
        &self,
        view_name: String,
        key_structure: String,
        primary: bool,
    ) -> GeorgeResult<()> {
        return match self.views.clone().read().unwrap().get(&view_name) {
            Some(view) => view
                .write()
                .unwrap()
                .create_index(self.id(), key_structure, primary),
            _ => Err(GeorgeError::ViewNoExistError(ViewNoExistError)),
        };
    }
    /// 插入数据，如果存在则返回已存在<p><p>
    ///
    /// ###Params
    ///
    /// view_name 视图名称<p><p>
    ///
    /// key string
    ///
    /// value 当前结果value信息<p><p>
    ///
    /// ###Return
    ///
    /// IndexResult<()>
    pub(crate) fn put(&self, view_name: String, key: String, value: Vec<u8>) -> GeorgeResult<()> {
        return match self.views.clone().read().unwrap().get(&view_name) {
            Some(view) => view.read().unwrap().put(key, value),
            _ => Err(GeorgeError::ViewNoExistError(ViewNoExistError)),
        };
    }
    /// 插入数据，无论存在与否都会插入或更新数据<p><p>
    ///
    /// ###Params
    ///
    /// view_name 视图名称<p><p>
    ///
    /// key string
    ///
    /// value 当前结果value信息<p><p>
    ///
    /// ###Return
    ///
    /// IndexResult<()>
    pub(crate) fn set(&self, view_name: String, key: String, value: Vec<u8>) -> GeorgeResult<()> {
        return match self.views.clone().read().unwrap().get(&view_name) {
            Some(view) => view.read().unwrap().set(key, value),
            _ => Err(GeorgeError::DataNoExistError(DataNoExistError)),
        };
    }
    /// 获取数据，返回存储对象<p><p>
    ///
    /// ###Params
    ///
    /// view_name 视图名称<p><p>
    ///
    /// key string
    ///
    /// ###Return
    ///
    /// Seed value信息
    pub(crate) fn get(&self, view_name: String, key: String) -> GeorgeResult<Vec<u8>> {
        return match self.views.clone().read().unwrap().get(&view_name) {
            Some(view) => view.read().unwrap().get(key),
            _ => Err(GeorgeError::ViewNoExistError(ViewNoExistError)),
        };
    }
    /// 条件检索
    ///
    /// selector_json_bytes 选择器字节数组，自定义转换策略
    pub fn select(
        &self,
        view_name: String,
        constraint_json_bytes: Vec<u8>,
    ) -> GeorgeResult<Expectation> {
        return match self.views.clone().read().unwrap().get(&view_name) {
            Some(view) => view.read().unwrap().select(constraint_json_bytes),
            _ => Err(GeorgeError::ViewNoExistError(ViewNoExistError)),
        };
    }
    /// 条件删除
    ///
    /// selector_json_bytes 选择器字节数组，自定义转换策略
    pub fn delete(
        &self,
        view_name: String,
        constraint_json_bytes: Vec<u8>,
    ) -> GeorgeResult<Expectation> {
        return match self.views.clone().read().unwrap().get(&view_name) {
            Some(view) => view.read().unwrap().delete(constraint_json_bytes),
            _ => Err(GeorgeError::ViewNoExistError(ViewNoExistError)),
        };
    }
}

impl Database {
    /// 恢复views数据
    pub(super) fn recovery_views(&self, paths: ReadDir) {
        // 遍历data目录下文件
        for path in paths {
            match path {
                // 所有目录文件被默认为database根目录
                Ok(dir) => {
                    if dir.path().is_dir() {
                        let view_dir_name = dir.file_name().to_str().unwrap().to_string();
                        println!("recovery view {}", view_dir_name);
                        // 恢复view数据
                        self.recovery_view(view_dir_name.clone());
                    }
                }
                Err(err) => panic!("recovery views path failed! error is {}", err),
            }
        }
    }

    /// 恢复view数据
    fn recovery_view(&self, view_dir_name: String) {
        let view_file_path = view_file_path(self.id(), view_dir_name.clone());
        match recovery_before_content(Tag::View, view_file_path.clone()) {
            Ok(hd) => {
                // println!("head = {:#?}", hd.header);
                // 恢复view数据
                let mut view = View::empty();
                match view.recover(hd.description) {
                    Ok(()) => {
                        let view_name = view.name();
                        println!(
                            "view [dbID={}, id={}, name={}, category={:#?}, level={:#?}, create_time={}]",
                            view.database_id(),
                            view.id(),
                            view_name,
                            view.category(),
                            view.level(),
                            view.create_time().num_nanoseconds().unwrap().to_string()
                        );
                        // 如果已存在该view，则不处理
                        if self.exist_view(view_name.clone()) {
                            return;
                        }
                        self.insert_view(view_name, Arc::new(RwLock::new(view)));
                        match GLOBAL_WRITER
                            .clone()
                            .insert_view(view_dir_name, view_file_path)
                        {
                            Ok(()) => {}
                            Err(err) => panic!(
                                "recovery view when writer insert view failed! error is {}",
                                err
                            ),
                        }
                    }
                    Err(err) => panic!("recovery view failed! error is {}", err),
                }
            }
            Err(err) => panic!(
                "recovery view when recovery before content failed! error is {}",
                err
            ),
        }
    }
}
