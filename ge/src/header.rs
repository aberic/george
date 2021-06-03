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

use comm::errors::{Errs, GeorgeResult};
use comm::vectors::VectorHandler;
use comm::Vector;

use crate::metadata::Header;
use crate::utils::enums::Tag;
use crate::{END, FRONT};
use std::sync::{Arc, RwLock};

/// impl for new
impl Header {
    /// ##生成非`index`属性的`ge`文件默认摘要
    ///
    /// ###Params
    /// * tag 文件类型标识符(1字节)
    ///
    /// ###Return
    ///
    /// 返回一个拼装完成的文件元数据中摘要信息
    pub(crate) fn new(tag: Tag) -> Self {
        Header {
            digest: Arc::new(RwLock::new(Digest::new(tag))),
        }
    }
}

/// impl for fn
impl Header {
    /// 获取文件元数据中摘要信息
    pub fn digest(&self) -> Arc<RwLock<Digest>> {
        self.digest.clone()
    }

    /// ##生成ge文件元数据中首部信息，长度32字节
    ///
    /// ###Return
    ///
    /// 返回一个拼装完成的文件元数据中首部信息，长度32字节
    pub fn to_vec(&self) -> GeorgeResult<Vec<u8>> {
        // 首2字节 + 摘要28字节 + 尾2字节 = 32字节
        let mut header_bytes: Vec<u8> = vec![];
        // 首2字节
        header_bytes.push(FRONT.get(0).unwrap().clone());
        header_bytes.push(FRONT.get(1).unwrap().clone());
        // 摘要28字节 = 已知5字节 + 占位23字节
        // 摘要28字节 - 已知5字节
        let mut digest_bytes = self.digest.read().unwrap().to_vec()?;
        header_bytes.append(&mut digest_bytes);
        // 摘要28字节 - 占位23字节
        let mut mid_bytes = Vector::create_empty_bytes(23);
        header_bytes.append(&mut mid_bytes);
        // 尾2字节
        header_bytes.push(END.get(0).unwrap().clone());
        header_bytes.push(END.get(1).unwrap().clone());

        Ok(header_bytes)
    }
}

/// impl for recovery
impl Header {
    /// ##恢复`ge`文件元数据中首部信息，长度32字节
    pub(crate) fn recovery(filepath: String, header_bytes: Vec<u8>) -> GeorgeResult<Header> {
        if 0x20 != header_bytes[0] || 0x19 != header_bytes[1] {
            Err(Errs::string(format!(
                "recovery header failed! front is invalid while file {}!",
                filepath
            )))
        } else if 0x02 != header_bytes[30] || 0x19 != header_bytes[31] {
            Err(Errs::string(format!(
                "recovery header failed! end is invalid while file {}!",
                filepath
            )))
        } else {
            let digest_bytes = header_bytes[2..].to_vec();
            Ok(Header {
                digest: Arc::new(RwLock::new(Digest::recovery(digest_bytes))),
            })
        }
    }
}

/// ##文件元数据中摘要信息，长度28字节
///
/// * `元数据`长度为32字节，由`起始符(2字节) + 摘要(28字节) + 截止符(2字节)`组成
/// * `摘要`由`文件类型标识符(1字节) + 文件版本号(2字节) + 文件序号(2字节) + 占位符(23字节)`组成
/// * 文件版本号(2字节)，读取该文件时进行版本区分的编号，即`ge`文件版本发布号
/// * 文件序号(2字节)，文件描述信息变更记录号，每当文件描述信息发生变更时，都会递增该序号
#[derive(Clone)]
pub struct Digest {
    /// 文件类型标识符(1字节)
    pub(crate) tag: Tag,
    /// 文件版本号(2字节)，读取该文件时进行版本区分的编号，即`ge`文件版本发布号
    pub(crate) version: [u8; 2],
    /// 文件序号(2字节)，文件描述信息变更记录号，每当文件描述信息发生变更时，都会递增该序号
    /// 该序号无法实时保证递增有效性，即有可能当版本发生了变更，但序号没有变
    /// 上述情况一般发生在版本信息更新成功后出现宕机等不可逆操作，导致序号尚未更新
    /// 但通过`history`接口读取版本变更记录的时候，该序号会得到正确的修复
    pub(crate) sequence: [u8; 2],
}
