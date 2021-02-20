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

use std::fmt;
use std::fs::File;

use comm::errors::entrances::GeorgeResult;
use comm::errors::entrances::{err_str, err_string};
use comm::io::file::{Filer, FilerNormal};
use comm::trans::{trans_bytes_2_u16, trans_bytes_2_u32, trans_u32_2_bytes};

use crate::utils::deploy::VERSION;
use crate::utils::enums::{Enum, EnumHandler, IndexType, Tag, ViewType};

/// 起始符
const FRONT: [u8; 2] = [0x20, 0x19];
/// 截止符
const END: [u8; 2] = [0x02, 0x19];

/// 文件信息
#[derive(Clone)]
pub struct Metadata {
    /// 标识符
    pub tag: Tag,
    /// 存储引擎类型
    pub index_type: IndexType,
    /// 视图引擎类型
    pub view_type: ViewType,
    /// 版本号
    pub version: [u8; 2],
    /// 序号
    pub sequence: u8,
}

impl fmt::Debug for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let version = vec![self.version[0], self.version[1]];
        write!(
            f,
            "tag = {:#?}, index_type = {:#?}, version = {:#?}, sequence = {:#?}",
            self.tag,
            self.index_type,
            trans_bytes_2_u16(version).unwrap(),
            self.sequence
        )
    }
}

impl Metadata {
    pub fn from_master(version: [u8; 2], sequence: u8) -> Metadata {
        Metadata {
            tag: Tag::Bootstrap,
            index_type: IndexType::None,
            view_type: ViewType::None,
            version,
            sequence,
        }
    }
    pub fn from_database(version: [u8; 2], sequence: u8) -> Metadata {
        Metadata {
            tag: Tag::Database,
            index_type: IndexType::None,
            view_type: ViewType::None,
            version,
            sequence,
        }
    }
    pub fn from_view(view_type: ViewType, version: [u8; 2], sequence: u8) -> Metadata {
        Metadata {
            tag: Tag::View,
            index_type: IndexType::None,
            view_type,
            version,
            sequence,
        }
    }
    pub fn from_index(index_type: IndexType, version: [u8; 2], sequence: u8) -> Metadata {
        Metadata {
            tag: Tag::Index,
            index_type,
            view_type: ViewType::None,
            version,
            sequence,
        }
    }
    pub fn database() -> Metadata {
        Metadata {
            tag: Tag::Database,
            index_type: IndexType::None,
            view_type: ViewType::None,
            version: VERSION,
            sequence: 0x00,
        }
    }
    pub fn view_disk() -> Metadata {
        Metadata {
            tag: Tag::View,
            index_type: IndexType::None,
            view_type: ViewType::Disk,
            version: VERSION,
            sequence: 0x00,
        }
    }
    pub fn view_mem() -> Metadata {
        Metadata {
            tag: Tag::View,
            index_type: IndexType::None,
            view_type: ViewType::Memory,
            version: VERSION,
            sequence: 0x00,
        }
    }
    pub fn index(index_type: IndexType) -> GeorgeResult<Metadata> {
        match index_type {
            IndexType::None => Err(err_str("unsupported engine type with none")),
            IndexType::Memory => Ok(Metadata {
                tag: Tag::Index,
                index_type,
                view_type: ViewType::None,
                version: VERSION,
                sequence: 0x00,
            }),
            IndexType::Dossier => Ok(Metadata {
                tag: Tag::Index,
                index_type,
                view_type: ViewType::None,
                version: VERSION,
                sequence: 0x00,
            }),
            IndexType::Library => Ok(Metadata {
                tag: Tag::Index,
                index_type,
                view_type: ViewType::None,
                version: VERSION,
                sequence: 0x00,
            }),
            IndexType::Block => Ok(Metadata {
                tag: Tag::Index,
                index_type,
                view_type: ViewType::None,
                version: VERSION,
                sequence: 0x00,
            }),
        }
    }
    fn from_bytes(head: Vec<u8>) -> GeorgeResult<Metadata> {
        if 0x20 != head.get(0).unwrap().clone() || 0x19 != head.get(1).unwrap().clone() {
            Err(err_str("recovery head failed! front is invalid!"))
        } else if 0x02 != head.get(30).unwrap().clone() || 0x19 != head.get(31).unwrap().clone() {
            Err(err_str("recovery head failed! end is invalid!"))
        } else {
            let tag = Enum::tag(head.get(2).unwrap().clone());
            match tag {
                Tag::Database => Ok(Metadata::from_database(
                    [head.get(4).unwrap().clone(), head.get(5).unwrap().clone()],
                    head.get(6).unwrap().clone(),
                )),
                Tag::View => Ok(Metadata::from_view(
                    Enum::view_type(head.get(3).unwrap().clone()),
                    [head.get(4).unwrap().clone(), head.get(5).unwrap().clone()],
                    head.get(6).unwrap().clone(),
                )),
                Tag::Index => Ok(Metadata::from_index(
                    Enum::index_type(head.get(3).unwrap().clone()),
                    [head.get(4).unwrap().clone(), head.get(5).unwrap().clone()],
                    head.get(6).unwrap().clone(),
                )),
                _ => Err(err_str("recovery head failed! tag doesn't support!")),
            }
        }
    }
    /// 标识符
    pub fn tag(&self) -> Tag {
        self.tag.clone()
    }
    /// 索引引擎类型
    pub fn index_type(&self) -> IndexType {
        self.index_type.clone()
    }
    /// 视图引擎类型
    pub fn view_type(&self) -> ViewType {
        self.view_type.clone()
    }
    /// 版本号
    pub fn version(&self) -> GeorgeResult<u16> {
        trans_bytes_2_u16(vec![self.version[0], self.version[1]])
    }
    /// 序号
    pub fn sequence(&self) -> u8 {
        self.sequence
    }
    /// 生成sr文件首部信息字符串，长度32个字节<p>
    ///
    /// 文件包括文件首部和正文两部分组成，文件首部告知了文件组成的所有有效信息，损坏将无法使用<p>
    ///
    /// ###Params
    ///
    /// tag 文件标识符，标识该文件是引导文件、库文件、表文件或是索引文件等，1字节<p>
    ///
    /// index_type 存储引擎类型，如内存类型Memory(0x00)/卷宗存储Dossier(0x01)/文库存储Library(0x02)/块存储Block(0x03)，该参数主要用于库、表和索引数据类型使用，1字节<p>
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
    pub fn bytes(&self) -> Vec<u8> {
        let mut type_u8: u8 = 0x00;
        match self.tag {
            Tag::View => type_u8 = Enum::view_type_u8(self.view_type()),
            Tag::Index => type_u8 = Enum::index_type_u8(self.index_type()),
            _ => {}
        }
        let head: [u8; 32] = [
            FRONT.get(0).unwrap().clone(),
            FRONT.get(1).unwrap().clone(),
            Enum::tag_u8(self.tag()),
            type_u8,
            self.version.get(0).unwrap().clone(),
            self.version.get(1).unwrap().clone(),
            self.sequence,
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
            0x00,
            0x00,
            END.get(0).unwrap().clone(),
            END.get(1).unwrap().clone(),
        ];
        head.to_vec()
    }
}

/// 正文前所有信息，包括头部信息和正文描述信息
///
/// start 正文描述起始位置，初始化为32 + 8，即head长度加正文描述符长度
///
/// description_len 正文描述内容持续长度
pub fn before_content_bytes(start: u32, description_len: u32) -> Vec<u8> {
    let mut start_bytes = trans_u32_2_bytes(start);
    let mut last_bytes = trans_u32_2_bytes(description_len);
    start_bytes.append(&mut last_bytes);
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

#[derive(Debug, Clone)]
pub struct HD {
    pub metadata: Metadata,
    pub description: Vec<u8>,
}

impl HD {
    pub fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }
    pub fn index_type(&self) -> IndexType {
        self.metadata().index_type.clone()
    }
    pub fn description(&self) -> Vec<u8> {
        self.description.clone()
    }
}

/// 恢复首部信息和正文描述信息，即正文内容之前的所有信息
pub fn recovery_before_content(filepath: String) -> GeorgeResult<HD> {
    match File::open(filepath.clone()) {
        Ok(file) => {
            match file.try_clone() {
                Ok(file_clone) => {
                    // before_content包括head以及正文描述信息
                    // head长度已知32，正文描述长度已知8，总长度40
                    let content = Filer::read_subs(file_clone, 0, 40)?;
                    let mut metadata_bytes: Vec<u8> = vec![];
                    let mut start_bytes: Vec<u8> = vec![];
                    let mut last_bytes: Vec<u8> = vec![];
                    let mut position = 0;
                    for b in content {
                        if position < 32 {
                            metadata_bytes.push(b)
                        } else if position >= 36 {
                            last_bytes.push(b)
                        } else {
                            start_bytes.push(b)
                        }
                        position += 1
                    }
                    let start = trans_bytes_2_u32(start_bytes.clone())? as u64;
                    let last = trans_bytes_2_u32(last_bytes.clone())? as usize;
                    let metadata = Metadata::from_bytes(metadata_bytes)?;
                    // 读取正文描述
                    let description = Filer::read_subs(file, start, last)?;
                    log::debug!(
                        "{:#?} recovery before content from file {}",
                        metadata.tag,
                        filepath
                    );
                    Ok(HD {
                        metadata,
                        description,
                    })
                }
                Err(err) => Err(err_string(format!(
                    "recovery before content file {} try clone failed! error is {}",
                    filepath, err
                ))),
            }
        }
        Err(err) => Err(err_string(format!(
            "recovery from path {} open failed! error is {}",
            filepath, err
        ))),
    }
}
