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

use comm::errors::{Errs, GeorgeResult};
use comm::io::file::{FilerHandler, FilerReader};
use comm::io::Filer;

use crate::metadata::{Description, Header};
use crate::utils::enums::Tag;
use crate::utils::Filed;
use crate::{Ge, GeImpl};
use std::sync::{Arc, RwLock};

impl GeImpl {
    /// ##生成`ge`文件对象
    ///
    /// ###Params
    /// * filepath 文件所在路径
    /// * tag 文件类型标识符
    /// * description 文件描述内容
    ///
    /// ###Return
    ///
    /// 返回一个拼装完成的文件元数据信息，长度52字节
    pub fn mock_new<P: AsRef<Path>>(
        filepath: P,
        tag: Tag,
        description: Vec<u8>,
    ) -> GeorgeResult<GeImpl> {
        if Filer::exist(&filepath) {
            GeImpl::recovery(filepath)
        } else {
            GeImpl::new(filepath, tag, description)
        }
    }
}

/// impl for new
impl GeImpl {
    /// ##生成`ge`文件对象
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
    ) -> GeorgeResult<GeImpl> {
        let filed = Filed::create(&filepath)?;
        let filepath = Filer::absolute(filepath)?;
        let metadata = Metadata::new(tag, description.len());
        // 文件元数据信息，长度52字节
        let mut metadata_bytes = metadata.to_vec()?;
        metadata_bytes.append(&mut description);
        // 将metadata默认值即描述内容同步写入
        filed.append(metadata_bytes)?;
        Ok(GeImpl {
            filepath,
            metadata,
            filed,
        })
    }

    /// ##生成无`文件描述内容`属性的`ge`文件对象
    ///
    /// ###Params
    /// * filepath 文件所在路径
    /// * tag 文件类型标识符
    ///
    /// ###Return
    ///
    /// 返回一个拼装完成的文件元数据信息，长度52字节
    pub fn new_empty<P: AsRef<Path>>(filepath: P, tag: Tag) -> GeorgeResult<GeImpl> {
        let mut description = vec![];
        let filed = Filed::create(&filepath)?;
        let filepath = Filer::absolute(filepath)?;
        let metadata = Metadata::new(tag, description.len());
        // 文件元数据信息，长度52字节
        let mut metadata_bytes = metadata.to_vec()?;
        metadata_bytes.append(&mut description);
        // 将metadata默认值即描述内容同步写入
        filed.append(metadata_bytes)?;
        Ok(GeImpl {
            filepath,
            metadata,
            filed,
        })
    }
}

/// impl for fn
impl Ge for GeImpl {
    fn inner(&self) -> Box<dyn Ge> {
        Box::new(self.clone())
    }

    /// 文件元数据信息，长度52字节
    fn filepath(&self) -> String {
        self.filepath.clone()
    }

    /// 获取文件元数据信息
    fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }

    /// 获取文件类型标识符
    fn tag(&self) -> Tag {
        self.metadata.header.digest.read().unwrap().tag()
    }

    /// 文件版本号(2字节)，读取该文件时进行版本区分的编号，即ge文件版本发布号
    fn version(&self) -> GeorgeResult<u16> {
        self.metadata.header.digest.read().unwrap().version()
    }

    /// 文件序号(2字节)，文件描述信息变更记录号，每当文件描述信息发生变更时，都会递增该序号
    fn sequence(&self) -> GeorgeResult<u16> {
        self.metadata.header.digest.read().unwrap().sequence()
    }

    /// ##初始化`ge`文件对象，一般在文件进行归档等操作后需要新的文件进行后续操作，新文件需要初始化
    ///
    /// 初始化结构为：`header_bytes(32字节) + [description_bytes(20字节) + description_content_bytes(len字节) … 循环]`
    ///
    /// ###Params
    /// * header_bytes ge文件元数据中首部信息，长度32字节
    /// * description_content_bytes_vc 文件描述变更记录
    /// * description 文件描述内容
    fn rebuild(
        &self,
        header_bytes: Vec<u8>,
        description_content_bytes_vc: Vec<Vec<u8>>,
    ) -> GeorgeResult<()> {
        // 将32字节首部信息写入文件
        self.append(header_bytes)?;
        // 起始坐标为52，即文件元数据信息(52字节)后开始计算
        let mut start = 52;
        let vc_len = description_content_bytes_vc.len();
        let pos = 1;
        for description_content_bytes in description_content_bytes_vc {
            let len = description_content_bytes.len();
            if pos < vc_len {
                let modify = start + (len as u64);
                // 生成文件描述信息，长度20字节
                let description_bytes = Description { start, len, modify }.to_vec();
                // 将20字节文件描述信息写入文件
                self.append(description_bytes)?;
                self.append(description_content_bytes)?;
                // 后续写入的文件描述内容的起始坐标为`modify + 20`
                start = modify + 20;
            } else {
                // 生成文件描述信息，长度20字节
                let description_bytes = Description {
                    start,
                    len,
                    modify: 0,
                }
                .to_vec();
                // 将20字节文件描述信息写入文件
                self.append(description_bytes)?;
                self.append(description_content_bytes)?;
            }
        }
        Ok(())
    }

    /// 变更描述信息
    ///
    /// ###Params
    /// * description 待变更的文件描述内容
    fn modify(&self, description_bytes: Vec<u8>) -> GeorgeResult<()> {
        match self.tag() {
            Tag::Index => Err(Errs::str("ge file with index tag can not be modify!")),
            _ => match self
                .metadata
                .description
                .write()
                .unwrap()
                .modify(description_bytes, &self.filed)
            {
                Ok(()) => {
                    let sequence_bytes = self
                        .metadata
                        .header
                        .digest
                        .write()
                        .unwrap()
                        .sequence_add()?;
                    self.write(6, sequence_bytes)
                }
                Err(err) => Err(err),
            },
        }
    }

    /// 文件描述变更记录
    fn history(&self) -> GeorgeResult<Vec<Vec<u8>>> {
        self.metadata
            .description
            .read()
            .unwrap()
            .history(&self.filed)
    }

    /// 文件描述最新内容字节数组
    fn description_content_bytes(&self) -> GeorgeResult<Vec<u8>> {
        self.metadata
            .description
            .read()
            .unwrap()
            .content_bytes(&self.filed)
    }

    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
    ///
    /// #Return
    ///
    /// seek_end_before 写之前文件字节数据长度
    fn append(&self, content: Vec<u8>) -> GeorgeResult<u64> {
        self.filed.append(content)
    }

    /// 写入的写对象到指定坐标
    ///
    /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
    fn write(&self, seek: u64, content: Vec<u8>) -> GeorgeResult<()> {
        self.filed.write(seek, content)
    }

    /// 读取文件部分内容，从start开始，一直持续读取last长度
    fn read(&self, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
        self.filed.read(start, last)
    }

    /// 读取文件部分内容，从start开始，一直持续读取last长度
    ///
    /// 如果无法读取该内容，即预期读取坐标超过实际内容长度，则返回期望读取长度的空字节数
    fn read_allow_none(&self, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
        self.filed.read_allow_none(start, last)
    }

    /// 获取文件长度
    fn len(&self) -> GeorgeResult<u64> {
        self.filed.len()
    }

    /// 整理归档
    ///
    /// archive_file_path 归档路径
    fn archive(&self, archive_file_path: String) -> GeorgeResult<()> {
        self.filed.archive(archive_file_path)
    }
}

/// impl for recovery
impl GeImpl {
    /// ##生成非`index`属性的`ge`文件对象
    ///
    /// ###Params
    /// * filepath 文件所在路径
    ///
    /// ###Return
    ///
    /// 返回一个拼装完成的文件元数据信息，长度52字节
    pub fn recovery<P: AsRef<Path>>(filepath: P) -> GeorgeResult<GeImpl> {
        let filepath = Filer::absolute(filepath)?;
        let filed = Filed::recovery(&filepath)?;
        let metadata_bytes = Filer::read_sub(&filepath, 0, 52)?;
        let metadata = Metadata::recovery(&filed, metadata_bytes)?;
        Ok(GeImpl {
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
/// * `摘要`由`文件类型标识符(1字节) + 文件版本号(2字节) + 文件序号(2字节) + 占位符(23字节)`组成
/// * 版本文件序号为[0x00, 0x00]
#[derive(Clone, Debug)]
pub struct Metadata {
    /// 文件元数据中首部信息(32字节)
    pub(crate) header: Header,
    /// 文件描述(20字节)
    pub(crate) description: Arc<RwLock<Description>>,
}
