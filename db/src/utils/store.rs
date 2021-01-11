use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::{Arc, RwLock};

use comm::errors::children::NoneError;
use comm::errors::entrances::GeorgeResult;
use comm::errors::entrances::{err_str, err_string, GeorgeError};
use comm::io::reader::read_sub_file_bytes;
use comm::io::writer::{write_all_bytes, write_seek_u8s};
use comm::trans::{trans_bytes_2_u16, trans_bytes_2_u32, trans_u16_2_bytes, trans_u32_2_bytes};

use crate::utils::comm::{Category, IndexMold, IndexType, LevelType};
use crate::utils::deploy::VERSION;
use crate::utils::writer::GLOBAL_WRITER;
use std::ops::Add;

/// 标识符
#[derive(Debug)]
pub enum Tag {
    /// 引导文件
    Bootstrap,
    /// 数据库文件
    Database,
    /// 表数据文件
    View,
    /// 索引数据文件
    Index,
}

/// 起始符
const FRONT: [u8; 2] = [0x20, 0x19];
/// 截止符
const END: [u8; 2] = [0x02, 0x19];

/// 文件信息
#[derive(Debug)]
pub struct FileHeader {
    /// 标识符
    pub tag: Tag,
    /// 存储类型
    pub category: Category,
    /// 存储容量
    pub level: LevelType,
    /// 索引类型
    pub index_type: IndexType,
    /// 序号
    pub sequence: u8,
}

impl FileHeader {
    pub fn create(
        tag: Tag,
        category: Category,
        level: LevelType,
        index_type: IndexType,
        sequence: u8,
    ) -> FileHeader {
        FileHeader {
            tag,
            category,
            level,
            index_type,
            sequence,
        }
    }
    fn from_bytes(head: Vec<u8>) -> GeorgeResult<FileHeader> {
        if 0x20 != head.get(0).unwrap().clone() || 0x19 != head.get(1).unwrap().clone() {
            Err(err_str("recovery head failed! because front is invalid!"))
        } else if 0x02 != head.get(30).unwrap().clone() || 0x19 != head.get(31).unwrap().clone() {
            Err(err_str("recovery head failed! because end is invalid!"))
        } else {
            Ok(FileHeader::create(
                tag(head.get(2).unwrap().clone()),
                category(head.get(3).unwrap().clone()),
                level(head.get(4).unwrap().clone()),
                index_type(head.get(5).unwrap().clone()),
                head.get(8).unwrap().clone(),
            ))
        }
    }
}

pub fn tag_u8(tag: Tag) -> u8 {
    match tag {
        Tag::Bootstrap => 0x00,
        Tag::Database => 0x01,
        Tag::View => 0x02,
        Tag::Index => 0x03,
    }
}

pub fn category_u8(category: Category) -> u8 {
    match category {
        Category::Memory => 0x00,
        Category::Document => 0x01,
    }
}

pub fn level_u8(level_type: LevelType) -> u8 {
    match level_type {
        LevelType::Small => 0x00,
        LevelType::Large => 0x01,
    }
}

pub fn mold_u8(mold: IndexMold) -> u8 {
    match mold {
        IndexMold::String => 0x00,
        IndexMold::U64 => 0x01,
        IndexMold::I64 => 0x02,
        IndexMold::U32 => 0x03,
        IndexMold::I32 => 0x04,
        IndexMold::F64 => 0x05,
    }
}

pub fn index_type_u8(index_type: IndexType) -> u8 {
    match index_type {
        IndexType::Siam => 0x00,
    }
}

pub fn tag(b: u8) -> Tag {
    match b {
        0x00 => Tag::Bootstrap,
        0x01 => Tag::Database,
        0x02 => Tag::View,
        0x03 => Tag::Index,
        _ => Tag::Bootstrap,
    }
}

pub fn category(b: u8) -> Category {
    match b {
        0x00 => Category::Memory,
        0x01 => Category::Document,
        _ => Category::Memory,
    }
}

pub fn mold(b: u8) -> IndexMold {
    match b {
        0x00 => IndexMold::String,
        0x01 => IndexMold::U64,
        0x02 => IndexMold::I64,
        0x03 => IndexMold::U32,
        0x04 => IndexMold::I32,
        0x05 => IndexMold::F64,
        _ => IndexMold::String,
    }
}

pub fn level(b: u8) -> LevelType {
    match b {
        0x00 => LevelType::Small,
        0x01 => LevelType::Large,
        _ => LevelType::Large,
    }
}

pub fn index_type(b: u8) -> IndexType {
    match b {
        0x00 => IndexType::Siam,
        _ => IndexType::Siam,
    }
}

pub fn mold_str(mold: IndexMold) -> String {
    match mold {
        IndexMold::String => String::from("string"),
        IndexMold::U64 => String::from("u64"),
        IndexMold::I64 => String::from("i64"),
        IndexMold::U32 => String::from("u32"),
        IndexMold::I32 => String::from("i32"),
        IndexMold::F64 => String::from("f32"),
    }
}

/// 生成sr文件首部信息字符串，长度32个字节<p>
///
/// 文件包括文件首部和正文两部分组成，文件首部告知了文件组成的所有有效信息，损坏将无法使用<p>
///
/// ###Params
///
/// tag 文件标识符，标识该文件是引导文件、库文件、表文件或是索引文件等，1字节<p>
///
/// category 文件存储类型，如内存类型Memory(0x00)或文档类型Document(0x01)，该参数主要用于库、表和索引数据类型使用，1字节<p>
///
/// level 文件存储容量，如Small(0x00)表示2^32，以及Large(0x01)表示2^64结点个数，该参数主要用于库、表和索引数据类型使用，1字节<p>
///
/// index_type 文件索引类型，如Siam(0x00)。该参数主要用于库、表和索引数据类型使用，1字节<p>
///
/// version 文件创建时版本号，2字节<p>
///
/// sequence 文件序号，第一个字节表示当前文件所持顺序，第二个字节表示是否存在后续文件，0x00无，0x01有。该参数主要用于
/// 表数据类型使用，2字节<p>
///
/// 自version=[0x00, 0x00]起始生效<p>
///
/// ###Return
///
/// 返回一个拼装完成的文件首部字符串
pub fn head(header: FileHeader) -> Vec<u8> {
    let head: [u8; 32] = [
        FRONT.get(0).unwrap().clone(),
        FRONT.get(1).unwrap().clone(),
        tag_u8(header.tag),
        category_u8(header.category),
        level_u8(header.level),
        index_type_u8(header.index_type),
        VERSION.get(0).unwrap().clone(),
        VERSION.get(1).unwrap().clone(),
        header.sequence,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        END.get(0).unwrap().clone(),
        END.get(1).unwrap().clone(),
    ];
    head.to_vec()
}

/// 正文前所有信息，包括头部信息和正文描述信息
///
/// start 正文描述起始位置，初始化为32 + 6，即head长度加正文描述符长度
///
/// description_len 正文描述内容持续长度
pub fn before_content_bytes(start: u32, description_len: u16) -> Vec<u8> {
    let mut start_bytes = trans_u32_2_bytes(start);
    let mut last_bytes = trans_u16_2_bytes(description_len);
    // println!(
    //     "start_bytes = {:#?}, last_bytes = {:#?}",
    //     start_bytes, last_bytes
    // );
    start_bytes.append(&mut last_bytes);
    // println!("start_bytes = {:#?}", start_bytes);
    start_bytes
}

/// index 正文前所有信息，包括头部信息和正文描述信息
///
/// start 正文描述起始位置，初始化为32 + 8，即head长度加正文描述符长度
///
/// description_len 正文描述内容持续长度
pub fn before_content_bytes_for_index(start: u32, description_len: u32) -> Vec<u8> {
    let mut start_bytes = trans_u32_2_bytes(start);
    let mut last_bytes = trans_u32_2_bytes(description_len);
    // println!(
    //     "start_bytes = {:#?}, last_bytes = {:#?}",
    //     start_bytes, last_bytes
    // );
    start_bytes.append(&mut last_bytes);
    // println!("start_bytes = {:#?}", start_bytes);
    start_bytes
}

// pub fn parse_before_content_bytes(bytes: Vec<u8>) -> Vec<u8> {}

#[derive(Debug)]
pub struct HD {
    pub header: FileHeader,
    pub description: Vec<u8>,
}

/// 恢复正文内容外的所有首部信息
pub fn recovery_before_content(tag: Tag, filepath: String) -> GeorgeResult<HD> {
    let td: &str;
    match tag {
        Tag::Bootstrap => td = "bootstrap",
        Tag::Database => td = "database",
        Tag::View => td = "view",
        Tag::Index => td = "index",
    }
    // println!("{} recovery before content", td);
    match File::open(filepath) {
        Ok(file) => {
            match file.try_clone() {
                Ok(f) => {
                    // before_content包括head以及正文描述信息
                    // other head长度已知32，正文描述长度已知6，总长度38
                    // index head长度已知32，正文描述长度已知8，总长度40
                    // 从0开始读取，一直读取80个字符
                    let content: Vec<u8>;
                    match tag {
                        Tag::Index => content = read_sub_file_bytes(f, 0, 40)?,
                        _ => content = read_sub_file_bytes(f, 0, 38)?,
                    }
                    let mut head_bytes: Vec<u8> = vec![];
                    let mut start_bytes: Vec<u8> = vec![];
                    let mut last_bytes: Vec<u8> = vec![];
                    let mut position = 0;
                    for b in content {
                        if position < 32 {
                            head_bytes.push(b)
                        } else if position >= 36 {
                            last_bytes.push(b)
                        } else {
                            start_bytes.push(b)
                        }
                        position += 1
                    }
                    let start = trans_bytes_2_u32(start_bytes.clone()) as u64;
                    let last: usize;
                    match tag {
                        Tag::Index => last = trans_bytes_2_u32(last_bytes.clone()) as usize,
                        _ => last = trans_bytes_2_u16(last_bytes.clone()) as usize,
                    }
                    // println!(
                    //     "head_bytes = {:#?}start_bytes = {:#?}\nlast_bytes = {:#?}\nstart = {}\nlast = {}",
                    //     head_bytes, start_bytes, last_bytes, start, last
                    // );
                    let header = FileHeader::from_bytes(head_bytes)?;
                    // 读取正文描述
                    let description = read_sub_file_bytes(file, start, last)?;
                    Ok(HD {
                        header,
                        description,
                    })
                }
                Err(err) => Err(err_string(format!(
                    "recovery {} before content file try clone failed! error is {}",
                    td, err
                ))),
            }
        }
        Err(err) => Err(err_string(format!(
            "recovery {} open failed! error is {}",
            td, err
        ))),
    }
}

pub fn store_view_id(database_id: String, view_id: String) -> String {
    database_id.add(&view_id)
}

pub fn store_index_id(database_id: String, view_id: String, index_id: String) -> String {
    database_id.add(&view_id).add(&index_id)
}

/// 存储对应head及文件内容描述<p>
///
/// 如果是view，则存储id为“database_id+view_id”<p>
/// 参考方法`store_view_id(database_id: String, view_id: String) -> String`<p>
///
/// 如果是index，则存储id为“database_id+view_id+index_id”<p>
/// 参考方法`store_index_id(database_id: String, view_id: String, index_id: String) -> String`
pub fn save<T>(
    tag: Tag,
    file: File,
    head: Vec<u8>,
    id: String,
    path: String,
    t: T,
) -> GeorgeResult<Arc<RwLock<T>>> {
    match file.try_clone() {
        Ok(f) => match write_all_bytes(f, head, t) {
            Ok(i) => match tag {
                Tag::Database => Ok(i),
                Tag::View => match GLOBAL_WRITER.clone().insert_view(id, path) {
                    Ok(()) => Ok(i),
                    Err(err) => Err(err),
                },
                Tag::Index => match GLOBAL_WRITER.clone().insert_index(id, path) {
                    Ok(()) => Ok(i),
                    Err(err) => Err(err),
                },
                _ => Err(GeorgeError::NoneError(NoneError)),
            },
            Err(err) => Err(err),
        },
        Err(err) => Err(err_string(err.to_string())),
    }
}

pub fn modify(filepath: String, description: Vec<u8>) -> GeorgeResult<()> {
    match OpenOptions::new().append(true).open(filepath.clone()) {
        Ok(mut file) => {
            let seek = file.metadata().unwrap().len();
            let before_description = before_content_bytes(seek as u32, description.len() as u16);
            match file.write_all(description.as_slice()) {
                Ok(()) => {
                    // 初始化head为32，描述起始4字节，长度2字节
                    write_seek_u8s(filepath, 32, before_description.as_slice())
                }
                Err(err) => Err(err_string(err.to_string())),
            }
        }
        Err(err) => Err(err_string(err.to_string())),
    }
}
