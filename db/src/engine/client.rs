use std::collections::HashMap;
use std::fs::{read_dir, read_to_string, ReadDir};
use std::sync::{Arc, RwLock};

use chrono::{Duration, Local, NaiveDateTime};
use once_cell::sync::Lazy;

use comm::env;
use comm::errors::children::{DatabaseExistError, DatabaseNoExistError};
use comm::errors::entrances::GeorgeError;
use comm::errors::entrances::GeorgeResult;
use comm::io::file;
use comm::io::writer::write_append_bytes;
use logs::set_log;

use crate::engine::database::Database;
use crate::engine::siam::selector::Expectation;
use crate::engine::traits::TDescription;
use crate::engine::view::View;
use crate::utils::comm::{Category, IndexType, LevelType, GEORGE_DB_CONFIG, INDEX_CATALOG};
use crate::utils::deploy::init_config;
use crate::utils::path::{bootstrap_file_path, data_path, database_file_path};
use crate::utils::store::{head, recovery_before_content, FileHeader, Tag};

/// 数据库
pub(crate) struct Engine {
    /// 视图索引集合
    databases: Arc<RwLock<HashMap<String, Arc<RwLock<Database>>>>>,
    /// 创建时间
    create_time: Duration,
}

pub(crate) static GLOBAL_CLIENT: Lazy<Arc<Engine>> = Lazy::new(|| {
    let now: NaiveDateTime = Local::now().naive_local();
    let create_time = Duration::nanoseconds(now.timestamp_nanos());
    init_log();
    init_config(config_path());
    let engine = Engine {
        databases: Default::default(),
        create_time,
    };
    let arc_engine = Arc::new(engine);
    // 创建数据根目录
    match file::create_dir(data_path()) {
        Ok(_file) => println!("load data path success!"),
        Err(err) => panic!("create data path failed! error is {}", err),
    }
    // 创建引导文件
    match file::create_file(bootstrap_file_path(), false) {
        Ok(_f) => arc_engine.clone().init_or_recovery(),
        Err(err) => panic!("create bootstrap file failed! error is {}", err),
    }
    arc_engine
});

fn config_path() -> String {
    env::get(GEORGE_DB_CONFIG, "src/examples/conf.yaml")
}

fn init_log() {
    set_log(
        String::from("db"),
        String::from("src/test"),
        1024,
        7,
        String::from("trace"),
    );
}

impl Engine {
    /// 初始化sky或恢复sky数据
    fn init_or_recovery(&self) {
        let bootstrap_file = bootstrap_file_path();
        match read_to_string(bootstrap_file.clone()) {
            Ok(text) => {
                if text.is_empty() {
                    println!("initialize new data");
                    self.init()
                } else {
                    println!("recovery exist data from bootstrap file {}", bootstrap_file);
                    self.recovery()
                }
            }
            Err(err) => panic!("init_or_recovery failed! error is {}", err),
        }
    }

    /// 初始化sky
    fn init(&self) {
        match write_append_bytes(
            bootstrap_file_path(),
            head(FileHeader::create(
                Tag::Bootstrap,
                Category::Document,
                LevelType::Small,
                IndexType::Siam,
                0x00,
            )),
        ) {
            Err(err) => panic!("init failed! error is {}", err),
            _ => {}
        }
    }

    /// 恢复sky数据
    fn recovery(&self) {
        // 读取data目录下所有文件
        match read_dir(data_path()) {
            Ok(paths) => self.recovery_databases(paths),
            Err(err) => panic!("recovery failed! error is {}", err),
        }
    }

    /// 恢复databases数据
    fn recovery_databases(&self, paths: ReadDir) {
        // 遍历data目录下文件
        for path in paths {
            match path {
                // 所有目录文件被默认为database根目录
                Ok(dir) => {
                    if dir.path().is_dir() {
                        let database_dir_name = dir.file_name().to_str().unwrap().to_string();
                        println!("recovery database {}", database_dir_name);
                        self.recovery_database(database_dir_name.clone());
                    }
                }
                Err(err) => panic!("recovery databases failed! error is {}", err),
            }
        }
    }

    /// 恢复database数据
    fn recovery_database(&self, database_dir_name: String) {
        match recovery_before_content(Tag::Database, database_file_path(database_dir_name.clone()))
        {
            Ok(hd) => {
                // println!("head = {:#?}", hd.header);
                // 恢复database数据
                let mut db = Database::empty();
                match db.recover(hd.description) {
                    Ok(()) => {
                        let db_name = db.name();
                        println!(
                            "db [id={}, name={}, create time ={}]",
                            db.id(),
                            db_name,
                            db.create_time().num_nanoseconds().unwrap().to_string(),
                        );
                        // 如果已存在该database，则不处理
                        if self.exist_database(db_name.clone()) {
                            return;
                        }
                        self.databases
                            .clone()
                            .write()
                            .unwrap()
                            .insert(db_name.clone(), Arc::new(RwLock::new(db)));
                    }
                    Err(err) => panic!("recovery database failed! error is {}", err),
                }
            }
            Err(err) => panic!("{}", err),
        }
    }
}

impl Engine {
    pub(crate) fn databases(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<Database>>>>> {
        self.databases.clone()
    }
    pub(crate) fn create_time(&self) -> Duration {
        self.create_time
    }
    /// 创建数据库
    pub(crate) fn create_database(
        &self,
        database_name: String,
        database_comment: String,
    ) -> GeorgeResult<()> {
        if self.exist_database(database_name.clone()) {
            return Err(GeorgeError::DatabaseExistError(DatabaseExistError));
        }
        let db = Database::init(database_name.clone(), database_comment)?;
        self.databases
            .clone()
            .write()
            .unwrap()
            .insert(database_name, db.clone());
        Ok(())
    }
    pub(crate) fn modify_database(
        &self,
        database_old_name: String,
        database_new_name: String,
    ) -> GeorgeResult<()> {
        if !self.exist_database(database_old_name.clone()) {
            return Err(GeorgeError::DatabaseNoExistError(DatabaseNoExistError));
        }
        if self.exist_database(database_new_name.clone()) {
            return Err(GeorgeError::DatabaseExistError(DatabaseExistError));
        }
        let databases = self.databases.clone();
        let mut databases_w = databases.write().unwrap();
        let database = databases_w.get(&database_old_name).unwrap().clone();
        databases_w.remove(&database_old_name);
        databases_w.insert(database_new_name.clone(), database.clone());
        database
            .clone()
            .write()
            .unwrap()
            .modify(database_new_name.clone())
    }
    /// 根据库name获取库
    pub(crate) fn database(&self, database_name: String) -> GeorgeResult<Arc<RwLock<Database>>> {
        match self.databases.clone().read().unwrap().get(&database_name) {
            Some(v) => Ok(v.clone()),
            None => Err(GeorgeError::DatabaseNoExistError(DatabaseNoExistError)),
        }
    }
    fn exist_database(&self, database_name: String) -> bool {
        return match self
            .databases
            .clone()
            .read()
            .unwrap()
            .get(database_name.as_str())
        {
            Some(_) => true,
            None => false,
        };
    }
    /// 创建视图
    pub(crate) fn create_view(
        &self,
        database_name: String,
        view_name: String,
        view_comment: String,
        index_type: IndexType,
        view_category: Category,
        view_level: LevelType,
    ) -> GeorgeResult<()> {
        match self.databases.clone().read().unwrap().get(&database_name) {
            Some(database_lock) => {
                let database = database_lock.read().unwrap();
                database.create_view(
                    view_name.clone(),
                    view_comment,
                    index_type,
                    view_category,
                    view_level,
                )?;
            }
            None => return Err(GeorgeError::DatabaseNoExistError(DatabaseNoExistError)),
        }
        self.create_index(database_name, view_name, INDEX_CATALOG.to_string(), true)
    }
    /// 获取数据库集合
    pub(crate) fn db_array(&self) -> Vec<Arc<RwLock<Database>>> {
        let mut arr_dbs = Vec::new();
        let dbs = self.databases.read().unwrap();
        for db in dbs.iter() {
            arr_dbs.push(db.1.clone())
        }
        return arr_dbs;
    }
    pub(crate) fn view_array(&self, database_name: String) -> GeorgeResult<Vec<Arc<RwLock<View>>>> {
        let database = self.database(database_name)?;
        let views = database.read().unwrap().views();
        let mut vs = Vec::new();
        for v in views.clone().read().unwrap().iter() {
            vs.push(v.1.clone())
        }
        Ok(vs)
    }
    pub(crate) fn views(
        &self,
        database_name: String,
    ) -> GeorgeResult<Arc<RwLock<HashMap<String, Arc<RwLock<View>>>>>> {
        let database = self.database(database_name)?;
        let vs = database.read().unwrap().views();
        Ok(vs)
    }
    pub(crate) fn view(
        &self,
        database_name: String,
        view_name: String,
    ) -> GeorgeResult<Arc<RwLock<View>>> {
        self.database(database_name)?
            .read()
            .unwrap()
            .view(view_name)
    }
    /// 在指定库及视图中创建索引
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
        database_name: String,
        view_name: String,
        key_structure: String,
        primary: bool,
    ) -> GeorgeResult<()> {
        let database = self.database(database_name)?;
        let db = database.clone();
        let db_r = db.read().unwrap();
        let view = db_r.view(view_name)?;
        let v = view.clone();
        let v_r = v.read().unwrap();
        v_r.create_index(db_r.id(), key_structure, primary)
    }
    pub(crate) fn modify_view(
        &self,
        database_name: String,
        view_old_name: String,
        view_new_name: String,
    ) -> GeorgeResult<()> {
        return match self.databases.clone().read().unwrap().get(&database_name) {
            Some(database_lock) => {
                let database = database_lock.read().unwrap();
                database.modify_view(view_old_name, view_new_name)
            }
            None => Err(GeorgeError::DatabaseNoExistError(DatabaseNoExistError)),
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
    pub(crate) fn put(
        &self,
        database_name: String,
        view_name: String,
        key: String,
        value: Vec<u8>,
    ) -> GeorgeResult<()> {
        self.database(database_name)?
            .read()
            .unwrap()
            .put(view_name, key, value)
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
    pub(crate) fn set(
        &self,
        database_name: String,
        view_name: String,
        key: String,
        value: Vec<u8>,
    ) -> GeorgeResult<()> {
        self.database(database_name)?
            .read()
            .unwrap()
            .set(view_name, key, value)
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
    pub(crate) fn get(
        &self,
        database_name: String,
        view_name: String,
        key: String,
    ) -> GeorgeResult<Vec<u8>> {
        self.database(database_name)?
            .read()
            .unwrap()
            .get(view_name, key)
    }
    /// 条件检索
    ///
    /// selector_json_bytes 选择器字节数组，自定义转换策略
    pub fn select(
        &self,
        database_name: String,
        view_name: String,
        constraint_json_bytes: Vec<u8>,
    ) -> GeorgeResult<Expectation> {
        self.database(database_name)?
            .read()
            .unwrap()
            .select(view_name, constraint_json_bytes)
    }
    /// 条件删除
    ///
    /// selector_json_bytes 选择器字节数组，自定义转换策略
    pub fn delete(
        &self,
        database_name: String,
        view_name: String,
        constraint_json_bytes: Vec<u8>,
    ) -> GeorgeResult<Expectation> {
        self.database(database_name)?
            .read()
            .unwrap()
            .delete(view_name, constraint_json_bytes)
    }
}
