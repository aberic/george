use std::sync::{Arc, RwLock};

use chrono::{Duration, Local, NaiveDateTime};

use comm::errors::entrances::err_string;
use comm::errors::entrances::GeorgeResult;
use comm::io::file::create_file;
use comm::trans::{trans_bytes_2_u16, trans_u16_2_bytes};
use comm::vectors;

use crate::engine::siam::traits::TNode;
use crate::engine::traits::{TDescription, TIndex, TSeed};
use crate::utils::comm::{Category, IndexType, LevelType};
use crate::utils::path;
use crate::utils::store::{
    before_content_bytes, category, category_u8, head, level, level_u8, save, FileHeader, Tag,
};

/// Siam索引
///
/// 5位key及16位md5后key及5位起始seek和4位持续seek
#[derive(Debug)]
pub struct Index<N: TNode> {
    /// 库ID
    database_id: String,
    /// 视图ID
    view_id: String,
    /// 索引唯一ID
    id: String,
    /// 是否主键
    primary: bool,
    /// 按照规范结构组成的索引字段名称，由对象结构层级字段通过'.'组成，如'i','in.s'
    key_structure: String,
    /// 结点
    root: Arc<N>,
    /// 类型
    category: Category,
    /// 规模/级别
    level: LevelType,
    /// 创建时间
    create_time: Duration,
    /// 除此参数外的描述长度
    description_len: usize,
}

impl<N: TNode> TDescription for Index<N> {
    fn description(&mut self) -> Vec<u8> {
        let mut des: Vec<u8> = vec![];
        let mut des_front = hex::encode(format!(
            "{}/{}/{}/{}/{}/{}/{}/{}",
            self.database_id,
            self.view_id,
            self.id,
            self.primary,
            self.key_structure,
            category_u8(self.category),
            level_u8(self.level),
            self.create_time.num_nanoseconds().unwrap().to_string(),
        ))
        .into_bytes();
        let len = des_front.len();
        self.description_len = len + 2 + 38;
        let mut len_bytes = trans_u16_2_bytes(len as u16);
        des.append(&mut len_bytes);
        des.append(&mut des_front);
        des.append(&mut self.root.clone().node_bytes().read().unwrap().to_vec());
        des
    }

    fn recover(&mut self, mut description: Vec<u8>) -> GeorgeResult<()> {
        let des_len: Vec<u8> = vectors::sub(description.clone(), 0, 2);
        let len = trans_bytes_2_u16(des_len) as usize;
        let des_front = vectors::sub(description.clone(), 2, len + 2);
        let description_len = len + 2;
        let node_bytes = description.split_off(description_len);
        match String::from_utf8(des_front) {
            Ok(description_str) => match hex::decode(description_str) {
                Ok(vu8) => match String::from_utf8(vu8) {
                    Ok(real) => {
                        let mut split = real.split("/");
                        self.database_id = split.next().unwrap().to_string();
                        self.view_id = split.next().unwrap().to_string();
                        self.id = split.next().unwrap().to_string();
                        self.primary = split.next().unwrap().to_string().parse::<bool>().unwrap();
                        self.key_structure = split.next().unwrap().to_string();
                        self.category =
                            category(split.next().unwrap().to_string().parse::<u8>().unwrap());
                        self.level =
                            level(split.next().unwrap().to_string().parse::<u8>().unwrap());
                        self.create_time = Duration::nanoseconds(
                            split.next().unwrap().to_string().parse::<i64>().unwrap(),
                        );
                        self.description_len = description_len + 38;
                        self.root.set_node_bytes(node_bytes);
                        Ok(())
                    }
                    Err(err) => Err(err_string(format!(
                        "recovery index from utf8 failed! error is {}",
                        err
                    ))),
                },
                Err(err) => Err(err_string(format!(
                    "recovery index decode failed! error is {}",
                    err
                ))),
            },
            Err(err) => Err(err_string(err.to_string())),
        }
    }
}

/// 新建索引
///
/// 该索引需要定义ID，此外索引所表达的字段组成内容也是必须的，并通过primary判断索引类型，具体传参参考如下定义：<p><p>
///
/// ###Params
///
/// key_structure 按照规范结构组成的索引字段名称，由对象结构层级字段通过'.'组成，如'i','in.s'
///
/// primary 是否主键
fn new_index<N: TNode>(
    database_id: String,
    view_id: String,
    id: String,
    key_structure: String,
    primary: bool,
    root: Arc<N>,
    category: Category,
    level: LevelType,
) -> Index<N> {
    let now: NaiveDateTime = Local::now().naive_local();
    let create_time = Duration::nanoseconds(now.timestamp_nanos());
    return Index {
        database_id,
        view_id,
        id,
        primary,
        key_structure,
        root,
        category,
        level,
        create_time,
        description_len: 0,
    };
}

/// 封装方法函数
impl<N: TNode> TIndex for Index<N> {
    fn database_id(&self) -> String {
        self.database_id.clone()
    }
    fn view_id(&self) -> String {
        self.view_id.clone()
    }
    fn id(&self) -> String {
        self.id.clone()
    }
    fn is_primary(&self) -> bool {
        self.primary.clone()
    }
    fn key_structure(&self) -> String {
        self.key_structure.clone()
    }
    fn category(&self) -> Category {
        self.category
    }
    fn level(&self) -> LevelType {
        self.level
    }
    fn create_time(&self) -> Duration {
        self.create_time
    }
    fn put(&self, key: String, seed: Arc<RwLock<dyn TSeed>>, force: bool) -> GeorgeResult<()> {
        match self.category {
            Category::Memory => {
                self.root
                    .put(key, seed, force, "".to_string(), self.description_len)
            }
            Category::Document => self
                .root
                .put(key, seed, force, self.id(), self.description_len),
        }
    }
    fn get(&self, key: String) -> GeorgeResult<Vec<u8>> {
        match self.category {
            Category::Memory => self.root.get(key, "".to_string(), self.description_len),
            Category::Document => self.root.get(key, self.id(), self.description_len),
        }
    }
}

/// 封装方法函数
impl<N: TNode> Index<N> {
    /// 新建索引
    ///
    /// 该索引需要定义ID，此外索引所表达的字段组成内容也是必须的，并通过primary判断索引类型，具体传参参考如下定义：<p><p>
    ///
    /// ###Params
    ///
    /// key_structure 索引名称，可以自定义；<p>
    /// siam::Index按照规范结构组成的索引字段名称，由对象结构层级字段通过'.'组成，如'i','in.s'<p><p>
    ///
    /// primary 是否主键
    ///
    /// level 视图规模/级别
    pub(crate) fn init(
        database_id: String,
        view_id: String,
        id: String,
        key_structure: String,
        primary: bool,
        root: Arc<N>,
        category: Category,
        level: LevelType,
    ) -> GeorgeResult<Arc<RwLock<Index<N>>>> {
        let mut index = new_index(
            database_id.clone(),
            view_id.clone(),
            id.clone(),
            key_structure.clone(),
            primary,
            root,
            category,
            level,
        );
        let index_file_path = path::index_file_path(database_id, view_id, id.clone());
        let file = create_file(index_file_path.clone(), true)?;
        let mut head = head(FileHeader::create(
            Tag::Index,
            Category::Document,
            level,
            IndexType::Siam,
            0x00,
        ));
        let mut description = index.description();
        // 初始化为32 + 6，即head长度加正文描述符长度
        let mut before_description = before_content_bytes(38, description.len() as u16);
        head.append(&mut before_description);
        head.append(&mut description);
        save(Tag::Index, file, head, id, index_file_path, index)
    }

    /// 新建索引
    ///
    /// 该索引需要定义ID，此外索引所表达的字段组成内容也是必须的，并通过primary判断索引类型，具体传参参考如下定义：<p><p>
    ///
    /// ###Params
    ///
    /// key_structure 索引名称，可以自定义；<p>
    /// siam::Index按照规范结构组成的索引字段名称，由对象结构层级字段通过'.'组成，如'i','in.s'<p><p>
    ///
    /// primary 是否主键
    ///
    /// level 视图规模/级别
    pub(crate) fn create(
        database_id: String,
        view_id: String,
        id: String,
        key_structure: String,
        primary: bool,
        root: Arc<N>,
        category: Category,
        level: LevelType,
    ) -> Index<N> {
        new_index(
            database_id,
            view_id,
            id,
            key_structure.clone(),
            primary,
            root,
            category,
            level,
        )
    }

    /// 恢复index数据
    pub(crate) fn regain(
        description: Vec<u8>,
        root: Arc<N>,
    ) -> GeorgeResult<Arc<RwLock<Index<N>>>> {
        let mut index = Index {
            database_id: "".to_string(),
            view_id: "".to_string(),
            id: "".to_string(),
            primary: false,
            key_structure: "".to_string(),
            root,
            category: Category::Memory,
            level: LevelType::Large,
            create_time: Duration::nanoseconds(1),
            description_len: 0,
        };
        index.recover(description)?;
        println!(
            "index = {}, {}, {}, {}, {}, {:#?}, {:#?}, {}",
            index.database_id(),
            index.view_id(),
            index.id(),
            index.key_structure(),
            index.is_primary(),
            index.category(),
            index.level(),
            index.create_time().num_nanoseconds().unwrap().to_string(),
        );
        Ok(Arc::new(RwLock::new(index)))
    }
}
