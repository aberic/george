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

use std::sync::{Arc, RwLock};

use comm::errors::{Errs, GeorgeResult};

use crate::header::Digest;
use crate::utils::enums::{Engine, Tag};
use crate::utils::Filed;
use crate::Metadata;

/// impl for new
impl Metadata {
    /// ##生成非`index`属性的`ge`文件文件元数据信息，长度52字节
    ///
    /// ###Params
    /// * digest 文件元数据中摘要信息，长度28字节
    ///
    /// ###Return
    ///
    /// 返回一个拼装完成的文件元数据信息，长度52字节
    pub(crate) fn new(filed: Arc<RwLock<Filed>>, tag: Tag, len: usize) -> Metadata {
        Metadata {
            header: Header::new(tag),
            description: Description::new(filed, len),
        }
    }

    /// ##生成`index`属性的`ge`文件文件元数据信息，长度52字节
    ///
    /// ###Params
    /// * digest 文件元数据中摘要信息，长度28字节
    ///
    /// ###Return
    ///
    /// 返回一个拼装完成的文件元数据信息，长度52字节
    pub(crate) fn new_4_index(filed: Arc<RwLock<Filed>>, engine: Engine, len: usize) -> Metadata {
        Metadata {
            header: Header::new_4_index(engine),
            description: Description::new(filed, len),
        }
    }
}

/// impl for fn
impl Metadata {
    /// ##生成ge文件元数据信息，长度52字节
    ///
    /// ###Params
    /// * digest 文件元数据中摘要信息，长度28字节
    ///
    /// ###Return
    ///
    /// 返回一个拼装完成的文件元数据信息，长度52字节
    pub(crate) fn to_vec(&self) -> GeorgeResult<Vec<u8>> {
        // 文件元数据信息，长度52字节
        let mut metadata_bytes: Vec<u8> = vec![];
        // 文件元数据中首部信息，长度32字节
        let mut header_bytes = self.header.to_vec()?;
        // 文件描述信息，长度20字节
        let mut des = self.description.to_vec();

        metadata_bytes.append(&mut header_bytes);
        metadata_bytes.append(&mut des);

        Ok(metadata_bytes)
    }
}

/// impl for recovery
impl Metadata {
    /// ##恢复`ge`文件元数据信息，长度52字节
    pub(crate) fn recovery(
        filed: Arc<RwLock<Filed>>,
        metadata_bytes: Vec<u8>,
    ) -> GeorgeResult<Metadata> {
        if metadata_bytes.len() != 52 {
            Err(Errs::str(
                "recovery metadata failed! metadata bytes len must be 52!",
            ))
        } else {
            Ok(Metadata {
                header: Header::recovery(metadata_bytes[0..32].to_vec())?,
                description: Description::recovery(filed, metadata_bytes[32..52].to_vec())?,
            })
        }
    }
}

/// ##文件元数据中首部信息，长度32字节
///
/// * `首部信息`长度为32字节，由`起始符(2字节) + 摘要(28字节) + 截止符(2字节)`组成
/// * `起始符`为固定标记[0x20, 0x19]
/// * `截止符`为固定标记[0x02, 0x19]
#[derive(Clone, Debug)]
pub struct Header {
    /// 文件元数据中摘要信息，长度28字节
    pub(crate) digest: Digest,
}

/// ##文件描述
/// * `文件描述`由`描述起始坐标(8字节) + 描述内容长度(4字节) + 变更后文件描述起始坐标(8字节)`
/// * `变更后文件描述起始坐标`为一个新的文件描述，是由于当前文件进行了描述变更，为了定位新的描述主题且能够追溯版本变更历史而做的追加
#[derive(Clone)]
pub struct Description {
    /// 描述起始坐标(8字节)
    pub(crate) start: u64,
    /// 描述内容长度(4字节)
    pub(crate) len: usize,
    /// 变更后文件描述起始坐标(8字节)
    pub(crate) modify: u64,
    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 需要借助对象包裹，以便更新file，避免self为mut
    pub(crate) filed: Arc<RwLock<Filed>>,
}
