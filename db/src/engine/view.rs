use std::collections::HashMap;
use std::fs::{read_dir, ReadDir};
use std::sync::{mpsc, Arc, RwLock};
use std::thread;

use chrono::{Duration, Local, NaiveDateTime};

use comm::cryptos::hash::md516;
use comm::errors::children::{DataNoExistError, IndexExistError};
use comm::errors::entrances::GeorgeResult;
use comm::errors::entrances::{err_string, GeorgeError};
use comm::io::file::create_file;

use crate::engine::siam::doc32::node::Node as Siam_Doc_Node;
use crate::engine::siam::doc32::seed::Seed as Doc32_Seed;
use crate::engine::siam::index::Index as Siam_Index;
use crate::engine::siam::mem::node::Node as Siam_Mem_Node;
use crate::engine::siam::mem::seed::Seed as Mem_Seed;
use crate::engine::siam::selector::{Expectation, Selector};
use crate::engine::traits::{TDescription, TIndex, TSeed};
use crate::utils::comm::{
    category, key_fetch, Category, IndexMold, IndexType, LevelType, INDEX_CATALOG,
};
use crate::utils::path::{index_file_path_yet, view_file_path, view_path};
use crate::utils::store;
use crate::utils::store::{
    before_content_bytes, category_u8, head, index_type_u8, level_u8, modify,
    recovery_before_content, save, store_index_id, store_view_id, FileHeader, Tag,
};
use crate::utils::writer::GLOBAL_WRITER;

/// 视图，类似表
pub(crate) struct View {
    /// 库ID
    database_id: String,
    /// 唯一ID
    id: String,
    /// 名称
    name: String,
    /// 描述
    comment: String,
    /// 索引类型
    index_type: IndexType,
    /// 类型
    category: Category,
    /// 规模/级别
    level: LevelType,
    /// 创建时间
    create_time: Duration,
    /// 索引集合
    indexes: Arc<RwLock<HashMap<String, Arc<RwLock<dyn TIndex>>>>>,
}

impl TDescription for View {
    fn description(&mut self) -> Vec<u8> {
        hex::encode(format!(
            "{}/{}/{}/{}/{}/{}/{}/{}",
            self.database_id,
            self.id,
            self.name,
            self.comment,
            index_type_u8(self.index_type),
            category_u8(self.category),
            level_u8(self.level),
            self.create_time.num_nanoseconds().unwrap().to_string()
        ))
        .into_bytes()
    }

    fn recover(&mut self, description: Vec<u8>) -> GeorgeResult<()> {
        match String::from_utf8(description) {
            Ok(description_str) => match hex::decode(description_str) {
                Ok(vu8) => match String::from_utf8(vu8) {
                    Ok(real) => {
                        let mut split = real.split("/");
                        self.database_id = split.next().unwrap().to_string();
                        self.id = split.next().unwrap().to_string();
                        self.name = split.next().unwrap().to_string();
                        self.comment = split.next().unwrap().to_string();
                        self.index_type = store::index_type(
                            split.next().unwrap().to_string().parse::<u8>().unwrap(),
                        );
                        self.category = store::category(
                            split.next().unwrap().to_string().parse::<u8>().unwrap(),
                        );
                        self.level =
                            store::level(split.next().unwrap().to_string().parse::<u8>().unwrap());
                        self.create_time = Duration::nanoseconds(
                            split.next().unwrap().to_string().parse::<i64>().unwrap(),
                        );
                        log::info!(
                            "recovery view {}({}.{})",
                            self.name(),
                            self.database_id(),
                            self.id()
                        );
                        match self.category() {
                            Category::Document => {
                                match read_dir(view_path(self.database_id(), self.id())) {
                                    // 恢复indexes数据
                                    Ok(paths) => {
                                        self.recovery_indexes(paths);
                                    }
                                    Err(err) => {
                                        panic!("recovery view read dir failed! error is {}", err)
                                    }
                                }
                            }
                            Category::Memory => match self.create_index(
                                self.database_id(),
                                INDEX_CATALOG.to_string(),
                                IndexMold::String,
                                true,
                            ) {
                                Err(err) => {
                                    panic!("recovery view read dir failed! error is {}", err)
                                }
                                _ => {}
                            },
                        }
                        Ok(())
                    }
                    Err(err) => Err(err_string(format!(
                        "recovery view from utf8 failed! error is {}",
                        err
                    ))),
                },
                Err(err) => Err(err_string(format!(
                    "recovery view decode failed! error is {}",
                    err
                ))),
            },
            Err(err) => Err(err_string(err.to_string())),
        }
    }
}

/// 新建视图
///
/// 具体传参参考如下定义：<p><p>
///
/// ###Params
///
/// id 视图唯一ID
///
/// name 视图名称
///
/// comment 视图描述
///
/// category 视图类型
///
/// level 视图规模/级别
fn new_view(
    database_id: String,
    name: String,
    comment: String,
    index_type: IndexType,
    category: Category,
    level: LevelType,
) -> View {
    let now: NaiveDateTime = Local::now().naive_local();
    let create_time = Duration::nanoseconds(now.timestamp_nanos());
    let id = md516(name.clone());
    return View {
        database_id,
        id,
        name,
        comment,
        index_type,
        category,
        level,
        create_time,
        indexes: Default::default(),
    };
}

/// 封装方法函数
impl View {
    pub(crate) fn init(
        database_id: String,
        name: String,
        comment: String,
        index_type: IndexType,
        category: Category,
        level: LevelType,
    ) -> GeorgeResult<Arc<RwLock<View>>> {
        let mut view = new_view(
            database_id.clone(),
            name,
            comment,
            index_type,
            category,
            level,
        );
        let view_id = view.id();
        let view_file_path = view_file_path(database_id.clone(), view_id.clone());
        let file = create_file(view_file_path.clone(), true)?;
        let mut head = head(FileHeader::create(
            Tag::View,
            category,
            level,
            index_type,
            0x00,
        ));
        let mut description = view.description();
        // 初始化为32 + 6，即head长度加正文描述符长度
        let mut before_description = before_content_bytes(38, description.len() as u16);
        head.append(&mut before_description);
        head.append(&mut description);
        save(
            Tag::View,
            file,
            head,
            store_view_id(database_id, view_id),
            view_file_path,
            view,
        )
    }
    pub(crate) fn create(
        database_id: String,
        name: String,
        comment: String,
        index_type: IndexType,
        category: Category,
        level: LevelType,
    ) -> View {
        return new_view(database_id, name, comment, index_type, category, level);
    }
    pub(crate) fn empty() -> View {
        return View {
            database_id: "".to_string(),
            id: "".to_string(),
            name: "".to_string(),
            comment: "".to_string(),
            index_type: IndexType::Siam,
            category: Category::Memory,
            level: LevelType::Small,
            create_time: Duration::nanoseconds(1),
            indexes: Arc::new(Default::default()),
        };
    }
    pub(crate) fn database_id(&self) -> String {
        self.database_id.clone()
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
    pub(crate) fn index_type(&self) -> IndexType {
        self.index_type
    }
    pub(crate) fn category(&self) -> Category {
        self.category
    }
    pub(crate) fn level(&self) -> LevelType {
        self.level
    }
    pub(crate) fn create_time(&self) -> Duration {
        self.create_time
    }
    pub(crate) fn modify(&mut self, name: String) -> GeorgeResult<()> {
        self.name = name;
        let description = self.description();
        modify(view_file_path(self.database_id(), self.id()), description)
    }
    /// 新建索引
    ///
    /// 该索引需要定义ID，此外索引所表达的字段组成内容也是必须的，并通过primary判断索引类型，具体传参参考如下定义：<p><p>
    ///
    /// ###Params
    ///
    /// index_name 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入
    ///
    /// primary 是否主键
    pub(crate) fn create_index(
        &self,
        database_id: String,
        index_name: String,
        index_mold: IndexMold,
        primary: bool,
    ) -> GeorgeResult<()> {
        let index_id = self.index_id(index_name.clone());
        if self.exist_index(index_id.clone()) {
            return Err(GeorgeError::IndexExistError(IndexExistError));
        }
        let index = self.index(
            database_id,
            index_id.clone(),
            index_name,
            index_mold,
            primary,
        )?;
        self.indexes
            .clone()
            .write()
            .unwrap()
            .insert(index_id, index);
        Ok(())
    }
    fn index_id(&self, index_name: String) -> String {
        md516(index_name.clone())
    }
    fn index(
        &self,
        database_id: String,
        index_id: String,
        index_name: String,
        index_mold: IndexMold,
        primary: bool,
    ) -> GeorgeResult<Arc<RwLock<dyn TIndex>>> {
        match self.index_type {
            IndexType::Siam => match self.category {
                Category::Memory => Ok(Arc::new(RwLock::new(Siam_Index::create(
                    database_id,
                    self.id(),
                    index_id,
                    index_name,
                    primary,
                    Siam_Mem_Node::create_root(),
                    category(self.category),
                    index_mold,
                )))),
                Category::Document => Ok(Siam_Index::init(
                    database_id,
                    self.id(),
                    index_id.clone(),
                    index_name,
                    primary,
                    Siam_Doc_Node::create_root(self.database_id(), self.id(), index_id),
                    category(self.category),
                    index_mold,
                )?),
            },
        }
    }
    fn exist_index(&self, index_name: String) -> bool {
        for res in self.indexes.clone().read().unwrap().iter() {
            if res.0.eq(&index_name) {
                return true;
            }
        }
        return false;
    }
    /// 插入数据，如果存在则返回已存在<p><p>
    ///
    /// ###Params
    ///
    /// key string
    ///
    /// value 当前结果value信息<p><p>
    ///
    /// ###Return
    ///
    /// IndexResult<()>
    pub(crate) fn put(&self, key: String, value: Vec<u8>) -> GeorgeResult<()> {
        self.save(key, value, false, false)
    }
    /// 插入数据，无论存在与否都会插入或更新数据<p><p>
    ///
    /// ###Params
    ///
    /// key string
    ///
    /// value 当前结果value信息<p><p>
    ///
    /// ###Return
    ///
    /// IndexResult<()>
    pub(crate) fn set(&self, key: String, value: Vec<u8>) -> GeorgeResult<()> {
        self.save(key, value, true, false)
    }
    /// 获取数据，返回存储对象<p><p>
    ///
    /// ###Params
    ///
    /// key string
    ///
    /// ###Return
    ///
    /// Seed value信息
    pub(crate) fn get(&self, key: String) -> GeorgeResult<Vec<u8>> {
        match self
            .indexes
            .clone()
            .read()
            .unwrap()
            .get(&self.index_id(INDEX_CATALOG.to_string()))
        {
            Some(index) => index.read().unwrap().get(key.clone()),
            None => Err(GeorgeError::DataNoExistError(DataNoExistError)),
        }
    }
    /// 删除数据<p><p>
    ///
    /// ###Params
    ///
    /// key string<p><p>
    ///
    /// ###Return
    ///
    /// IndexResult<()>
    pub(crate) fn remove(&self, key: String) -> GeorgeResult<()> {
        self.save(key, vec![], true, true)
    }
    /// 条件检索
    ///
    /// selector_json_bytes 选择器字节数组，自定义转换策略
    pub fn select(&self, constraint_json_bytes: Vec<u8>) -> GeorgeResult<Expectation> {
        Selector::run(constraint_json_bytes, self.indexes.clone(), false)
    }
    /// 条件删除
    ///
    /// selector_json_bytes 选择器字节数组，自定义转换策略
    pub fn delete(&self, constraint_json_bytes: Vec<u8>) -> GeorgeResult<Expectation> {
        Selector::run(constraint_json_bytes, self.indexes.clone(), true)
    }
    /// 插入数据业务方法<p><p>
    ///
    /// ###Params
    ///
    /// key string
    ///
    /// value 当前结果value信息<p><p>
    ///
    /// force 如果存在原值，是否覆盖原结果<p><p>
    ///
    /// ###Return
    ///
    /// IndexResult<()>
    fn save(&self, key: String, value: Vec<u8>, force: bool, remove: bool) -> GeorgeResult<()> {
        let seed: Arc<RwLock<dyn TSeed>>;
        match self.category {
            Category::Memory => {
                seed = Arc::new(RwLock::new(Mem_Seed::create(md516(key.clone()))));
            }
            Category::Document => {
                seed = Arc::new(RwLock::new(Doc32_Seed::create(
                    self.database_id(),
                    self.id(),
                    value.clone(),
                )));
            }
        }
        let mut receives = Vec::new();
        for index in self.indexes.clone().read().unwrap().iter() {
            let key_clone = key.clone();
            let seed_clone = seed.clone();
            let (sender, receive) = mpsc::channel();
            receives.push(receive);
            let index_clone = index.1.clone();
            let value_clone = value.clone();
            thread::spawn(move || {
                let index_r = index_clone.read().unwrap();
                let index_name = index_r.name();
                match index_name.as_str() {
                    INDEX_CATALOG => {
                        sender.send(index_r.put(key_clone.clone(), seed_clone.clone(), force))
                    }
                    _ => match key_fetch(index_r.name(), value_clone) {
                        Ok(res) => sender.send(index_r.put(res, seed_clone.clone(), force)),
                        Err(err) => {
                            log::debug!("error is {}", err);
                            sender.send(Ok(()))
                        }
                    },
                }
            });
        }
        for receive in receives.iter() {
            let res = receive.recv();
            match res {
                Ok(gr) => match gr {
                    Err(err) => return Err(err),
                    _ => {}
                },
                Err(err) => return Err(err_string(err.to_string())),
            }
        }
        if remove {
            seed.write().unwrap().remove()
        } else {
            // 执行真实存储操作，即索引将seed存入后，允许检索到该结果，但该结果值不存在，仅当所有索引存入都成功，才会执行本方法完成真实存储操作
            seed.write().unwrap().save(value)
        }
    }
}

impl View {
    /// 恢复indexes数据
    pub(super) fn recovery_indexes(&mut self, paths: ReadDir) {
        // 遍历data目录下文件
        for path in paths {
            match path {
                // 所有目录文件被默认为database根目录
                Ok(dir) => {
                    if dir.path().is_file() {
                        let index_file_name = dir.file_name().to_str().unwrap().to_string();
                        if index_file_name != "view.sr" {
                            log::debug!("recovery index from {}.sr", index_file_name);
                            // 恢复index数据
                            match self.recovery_index(self.database_id(), index_file_name.clone()) {
                                Ok(index) => {
                                    let idx = index.clone();
                                    let idx_r = idx.read().unwrap();
                                    let index_id = self.index_id(idx_r.name());
                                    // 如果已存在该view，则不处理
                                    if self.exist_index(index_id.clone()) {
                                        return;
                                    }
                                    self.indexes
                                        .clone()
                                        .write()
                                        .unwrap()
                                        .insert(index_id, index);
                                }
                                Err(err) => panic!("recovery_index failed while database is {} and index_file_name is {}, error: {}", self.database_id(), index_file_name, err),
                            }
                        }
                    }
                }
                Err(err) => panic!("recovery indexes path failed! error is {}", err),
            }
        }
    }
    /// 恢复index数据
    fn recovery_index(
        &self,
        database_id: String,
        index_file_name: String,
    ) -> GeorgeResult<Arc<RwLock<dyn TIndex>>> {
        let index_file_path = index_file_path_yet(database_id, self.id(), index_file_name.clone());
        let hd = recovery_before_content(Tag::Index, index_file_path.clone())?;
        log::trace!("head = {:#?}", hd.header);
        match self.index_type {
            IndexType::Siam => match self.category {
                Category::Memory => Ok(Siam_Index::regain(
                    hd.description,
                    Siam_Mem_Node::create_root(),
                )?),
                Category::Document => {
                    let index = Siam_Index::regain(
                        hd.description,
                        Siam_Doc_Node::create_root(
                            self.database_id(),
                            self.id(),
                            index_file_name
                                .clone()
                                .split(".")
                                .next()
                                .unwrap()
                                .to_string(),
                        ),
                    )?;
                    GLOBAL_WRITER.clone().insert_index(
                        store_index_id(
                            self.database_id(),
                            self.id(),
                            index.clone().read().unwrap().id(),
                        ),
                        index_file_path,
                    )?;
                    Ok(index)
                }
            },
        }
    }
}
