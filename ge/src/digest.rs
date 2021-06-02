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

use std::fmt;

use comm::errors::{Errs, GeorgeResult};
use comm::Trans;

use crate::header::Digest;
use crate::utils::enums::{Engine, Tag};
use crate::utils::{Enum, EnumHandler};
use crate::VERSION;

impl fmt::Debug for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{tag: {:#?}, engine: {:#?}, version: {:#?}, sequence: {:#?}}}",
            self.tag,
            self.engine,
            self.version().unwrap(),
            self.sequence().unwrap(),
        )
    }
}

/// impl for new
impl Digest {
    /// ##生成非`index`属性的`ge`文件默认摘要
    ///
    /// ###Params
    /// * tag 文件类型标识符(1字节)
    ///
    /// ###Return
    ///
    /// 返回一个拼装完成的文件元数据中摘要信息
    pub(crate) fn new(tag: Tag) -> Self {
        Digest {
            tag,
            engine: Engine::None,
            version: VERSION,
            sequence: [0x00, 0x00],
        }
    }

    /// 索引数据文件默认摘要
    pub(crate) fn new_4_index(engine: Engine) -> Self {
        Digest {
            tag: Tag::Index,
            engine,
            version: VERSION,
            sequence: [0x00, 0x00],
        }
    }
}

/// impl for fn
impl Digest {
    /// 获取文件类型标识符
    pub(crate) fn tag(&self) -> Tag {
        self.tag.clone()
    }

    /// 获取存储引擎类型
    pub(crate) fn engine(&self) -> Engine {
        self.engine.clone()
    }

    /// 文件版本号(2字节)，读取该文件时进行版本区分的编号，即ge文件版本发布号
    pub(crate) fn version(&self) -> GeorgeResult<u16> {
        Trans::bytes_2_u16(self.version.to_vec())
    }

    /// 文件序号(2字节)，文件描述信息变更记录号，每当文件描述信息发生变更时，都会递增该序号
    pub(crate) fn sequence(&self) -> GeorgeResult<u16> {
        Trans::bytes_2_u16(self.sequence.to_vec())
    }

    /// 文件序号递增
    pub(crate) fn sequence_add(&mut self) -> GeorgeResult<Vec<u8>> {
        let mut sequence = Trans::bytes_2_u16(self.sequence.to_vec())?;
        sequence += 1;
        let sequence_bytes = Trans::u16_2_bytes(sequence);
        self.sequence = [sequence_bytes[0], sequence_bytes[1]];
        Ok(sequence_bytes)
    }

    /// ##生成ge文件摘要已知信息，长度6字节
    ///
    /// ###Return
    ///
    /// 返回一个拼装完成的文件已知摘要信息，长度6字节
    pub fn to_vec(&self) -> GeorgeResult<Vec<u8>> {
        let engine: u8;
        let tag = Enum::tag_u8(self.tag());
        match self.tag() {
            Tag::Index => match self.engine() {
                Engine::None => return Err(Errs::str("index engine not support none!")),
                _ => engine = Enum::engine_u8(self.engine()),
            },
            _ => engine = Enum::engine_u8(Engine::None),
        }
        // 文件元数据中摘要信息，长度28字节
        let mut digest_bytes: Vec<u8> = vec![];
        // 已知6字节
        digest_bytes.push(tag);
        digest_bytes.push(engine);
        digest_bytes.push(self.version.get(0).unwrap().clone());
        digest_bytes.push(self.version.get(1).unwrap().clone());
        digest_bytes.push(self.sequence.get(0).unwrap().clone());
        digest_bytes.push(self.sequence.get(1).unwrap().clone());

        Ok(digest_bytes)
    }
}

/// impl for recovery
impl Digest {
    /// ##恢复`ge`文件默认摘要已知内容，长度6字节
    pub(crate) fn recovery(digest_bytes: Vec<u8>) -> GeorgeResult<Digest> {
        if digest_bytes.len() != 6 {
            Err(Errs::str(
                "recovery digest failed! digest bytes len must be 6!",
            ))
        } else {
            // 文件类型标识符(1字节)
            let tag = Enum::tag(digest_bytes[0]);
            // 存储引擎类型符(1字节)
            let engine = Enum::engine(digest_bytes[1]);
            // 文件版本号(2字节)
            let version: [u8; 2] = [digest_bytes[2], digest_bytes[3]];
            // 文件序号(2字节)
            let sequence: [u8; 2] = [digest_bytes[4], digest_bytes[5]];
            Ok(Digest {
                tag,
                engine,
                version,
                sequence,
            })
        }
    }
}
