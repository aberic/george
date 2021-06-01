/*
 * Copyright (c) 2021. Aberic - All Rights Reserved.
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

use std::path::Path;
use std::sync::{Arc, RwLock};

use comm::errors::{Errs, GeorgeResult};
use comm::io::file::{FilerHandler, FilerReader};
use comm::io::Filer;

use crate::metadata::{Description, Header};
use crate::utils::enums::{Engine, Tag};
use crate::utils::Filed;
use crate::Ge;

/// impl for new
impl Ge {
    /// ##生成非`index`属性的`ge`文件对象
    ///
    /// ###Params
    /// * filepath 文件所在路径
    /// * tag 文件类型标识符
    /// * description 文件描述内容
    ///
    /// ###Return
    ///
    /// 返回一个拼装完成的文件元数据信息，长度52字节
    pub(crate) fn new_mock<P: AsRef<Path>>(
        filepath: P,
        tag: Tag,
        description: Vec<u8>,
    ) -> GeorgeResult<Ge> {
        match tag {
            Tag::Index => Err(Errs::str(
                "index engine must be assigned, try new_4_index instead!",
            )),
            _ => {
                if Filer::exist(&filepath) {
                    Ge::recovery(filepath)
                } else {
                    Ge::new(filepath, tag, description)
                }
            }
        }
    }
    /// ##生成非`index`属性的`ge`文件对象
    ///
    /// ###Params
    /// * filepath 文件所在路径
    /// * tag 文件类型标识符
    /// * description 文件描述内容
    ///
    /// ###Return
    ///
    /// 返回一个拼装完成的文件元数据信息，长度52字节
    pub fn new<P: AsRef<Path>>(
        filepath: P,
        tag: Tag,
        mut description: Vec<u8>,
    ) -> GeorgeResult<Ge> {
        match tag {
            Tag::Index => Err(Errs::str(
                "index engine must be assigned, try new_4_index instead!",
            )),
            _ => {
                let filed = Arc::new(RwLock::new(Filed::create(&filepath)?));
                let filepath = Filer::absolute(filepath)?;
                let metadata = Metadata::new(filed.clone(), tag, description.len());
                // 文件元数据信息，长度52字节
                let mut metadata_bytes = metadata.to_vec()?;
                metadata_bytes.append(&mut description);
                let filed_c = filed.clone();
                let filed_w = filed_c.write().unwrap();
                // 将metadata默认值即描述内容同步写入
                filed_w.append(metadata_bytes)?;
                Ok(Ge {
                    filepath,
                    metadata,
                    filed,
                })
            }
        }
    }

    /// ##生成`index`属性的`ge`文件对象
    ///
    /// ###Params
    /// * filepath 文件所在路径
    /// * engine 存储引擎类型
    /// * description 文件描述内容
    ///
    /// ###Return
    ///
    /// 返回一个拼装完成的文件元数据信息，长度52字节
    pub fn new_4_index<P: AsRef<Path>>(
        filepath: P,
        engine: Engine,
        mut description: Vec<u8>,
    ) -> GeorgeResult<Ge> {
        let filed = Arc::new(RwLock::new(Filed::create(&filepath)?));
        let filepath = Filer::absolute(filepath)?;
        let metadata = Metadata::new_4_index(filed.clone(), engine, description.len());
        // 文件元数据信息，长度52字节
        let mut metadata_bytes = metadata.to_vec()?;
        metadata_bytes.append(&mut description);
        let filed_c = filed.clone();
        let filed_w = filed_c.write().unwrap();
        // 将metadata默认值即描述内容同步写入
        filed_w.append(metadata_bytes)?;
        Ok(Ge {
            filepath,
            metadata,
            filed,
        })
    }
}

/// impl for fn
impl Ge {
    /// 文件元数据信息，长度52字节
    pub fn filepath(&self) -> String {
        self.filepath.clone()
    }

    /// 获取文件类型标识符
    pub fn tag(&self) -> Tag {
        self.metadata.header.digest.tag()
    }

    /// 获取存储引擎类型
    pub fn engine(&self) -> Engine {
        self.metadata.header.digest.engine()
    }

    /// 文件版本号(2字节)，读取该文件时进行版本区分的编号，即ge文件版本发布号
    pub fn version(&self) -> GeorgeResult<u16> {
        self.metadata.header.digest.version()
    }

    /// 文件序号(2字节)，文件描述信息变更记录号，每当文件描述信息发生变更时，都会递增该序号
    pub fn sequence(&self) -> GeorgeResult<u16> {
        self.metadata.header.digest.sequence()
    }

    /// 变更描述信息
    ///
    /// ###Params
    /// * description 待变更的文件描述内容
    pub fn modify(&mut self, description_bytes: Vec<u8>) -> GeorgeResult<()> {
        match self.tag() {
            Tag::Index => Err(Errs::str("ge file with index tag can not be modify!")),
            _ => match self.metadata.description.modify(description_bytes) {
                Ok(()) => {
                    let sequence_bytes = self.metadata.header.digest.sequence_add()?;
                    self.write(6, sequence_bytes)
                }
                Err(err) => Err(err),
            },
        }
    }

    /// 文件描述变更记录
    pub fn history(&self) -> GeorgeResult<Vec<Vec<u8>>> {
        self.metadata.description.history()
    }

    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
    ///
    /// #Return
    ///
    /// seek_end_before 写之前文件字节数据长度
    pub fn append(&self, content: Vec<u8>) -> GeorgeResult<u64> {
        self.filed.write().unwrap().append(content)
    }

    /// 写入的写对象到指定坐标
    ///
    /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
    pub fn write(&self, seek: u64, content: Vec<u8>) -> GeorgeResult<()> {
        self.filed.write().unwrap().write(seek, content)
    }
}

/// impl for recovery
impl Ge {
    /// ##生成非`index`属性的`ge`文件对象
    ///
    /// ###Params
    /// * filepath 文件所在路径
    ///
    /// ###Return
    ///
    /// 返回一个拼装完成的文件元数据信息，长度52字节
    pub fn recovery<P: AsRef<Path>>(filepath: P) -> GeorgeResult<Ge> {
        let filepath = Filer::absolute(filepath)?;
        let filed = Arc::new(RwLock::new(Filed::recovery(&filepath)?));
        let metadata_bytes = Filer::read_sub(&filepath, 0, 52)?;
        let metadata = Metadata::recovery(filed.clone(), metadata_bytes)?;
        Ok(Ge {
            filepath,
            metadata,
            filed,
        })
    }
}

/// #文件元数据信息，长度52字节
///
/// * 文件信息包括`元数据 + 文件正文`，元数据用于描述文件属性，文件正文是于读取有意义的存储内容
/// * `元数据`由`首部信息(32字节) + 文件描述(20字节)`组成
/// ---
/// ##首部信息
/// * `首部信息`长度为32字节，由`起始符(2字节) + 摘要(28字节) + 截止符(2字节)`组成
/// * `起始符`为固定标记[0x20, 0x19]
/// * `截止符`为固定标记[0x02, 0x19]
/// * `摘要`由`有意义字符 + 占位符组成`
/// ---
/// ##文件描述
/// * 文件描述内容为不固定长度字节数组，为了更好的恢复文件可读性并能精准定位文件变更记录，文件描述设计为可追溯的20字节码
/// * 首个`文件描述`在`首部信息`后追加，与`首部信息`共同组成了初始版本的`元数据`
/// * `文件描述`由`描述起始坐标(8字节) + 描述内容长度(4字节) + 变更后文件描述起始坐标(8字节)`
/// * `变更后文件描述起始坐标`为一个新的文件描述，是由于当前文件进行了描述变更，为了定位新的描述主题且能够追溯版本变更历史而做的追加
/// ---
/// ##20210531版
/// * `摘要`由`文件类型标识符(1字节) + 存储引擎类型符(1字节) + 文件版本号(2字节) + 文件序号(2字节) + 占位符(22字节)`组成
/// * 版本文件序号为[0x00, 0x00]
#[derive(Clone, Debug)]
pub struct Metadata {
    /// 文件元数据中首部信息(32字节)
    pub(crate) header: Header,
    /// 文件描述(20字节)
    pub(crate) description: Description,
}
