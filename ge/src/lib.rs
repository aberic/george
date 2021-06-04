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

use comm::errors::GeorgeResult;

use crate::ge::Metadata;
use crate::utils::enums::Tag;
use crate::utils::Filed;
use std::fmt::Debug;

mod description;
mod digest;
mod factory;
mod ge;
mod ge_test;
mod header;
mod metadata;
mod metadata_test;
pub mod utils;

/// 文件元数据信息，长度52字节
pub const METADATA_SIZE: u64 = 52;

/// 当前文件版本号(2字节)
pub const VERSION: [u8; 2] = [0x00, 0x00];
/// 文件版本1(2字节)
pub const VERSION_1: [u8; 2] = [0x00, 0x00];
/// 起始符(2字节)
const FRONT: [u8; 2] = [0x20, 0x19];
/// 截止符(2字节)
const END: [u8; 2] = [0x02, 0x19];

pub trait Ge: Send + Sync + Debug {
    /// 文件元数据信息，长度52字节
    fn inner(&self) -> Box<dyn Ge>;

    /// 文件元数据信息，长度52字节
    fn filepath(&self) -> String {
        self.inner().filepath()
    }

    /// 获取文件元数据信息
    fn metadata(&self) -> Metadata {
        self.inner().metadata()
    }

    /// 获取文件类型标识符
    fn tag(&self) -> Tag {
        self.inner().tag()
    }

    /// 文件版本号(2字节)，读取该文件时进行版本区分的编号，即ge文件版本发布号
    fn version(&self) -> GeorgeResult<u16> {
        self.inner().version()
    }

    /// 文件序号(2字节)，文件描述信息变更记录号，每当文件描述信息发生变更时，都会递增该序号
    fn sequence(&self) -> GeorgeResult<u16> {
        self.inner().sequence()
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
        self.inner()
            .rebuild(header_bytes, description_content_bytes_vc)
    }

    /// 变更描述信息
    ///
    /// ###Params
    /// * description 待变更的文件描述内容
    fn modify(&self, description_bytes: Vec<u8>) -> GeorgeResult<()> {
        self.inner().modify(description_bytes)
    }

    /// 文件描述变更记录
    fn history(&self) -> GeorgeResult<Vec<Vec<u8>>> {
        self.inner().history()
    }

    /// 文件描述最新内容字节数组
    fn description_content_bytes(&self) -> GeorgeResult<Vec<u8>> {
        self.inner().description_content_bytes()
    }

    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
    ///
    /// #Return
    ///
    /// seek_end_before 写之前文件字节数据长度
    fn append(&self, content: Vec<u8>) -> GeorgeResult<u64> {
        self.inner().append(content)
    }

    /// 写入的写对象到指定坐标
    ///
    /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
    fn write(&self, seek: u64, content: Vec<u8>) -> GeorgeResult<()> {
        self.inner().write(seek, content)
    }

    /// 读取文件部分内容，从start开始，一直持续读取last长度
    fn read(&self, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
        self.inner().read(start, last)
    }

    /// 读取文件部分内容，从start开始，一直持续读取last长度
    ///
    /// 如果无法读取该内容，即预期读取坐标超过实际内容长度，则返回期望读取长度的空字节数
    fn read_allow_none(&self, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
        self.inner().read_allow_none(start, last)
    }

    /// 获取文件长度
    fn len(&self) -> GeorgeResult<u64> {
        self.inner().len()
    }

    /// 整理归档
    ///
    /// archive_file_path 归档路径
    fn archive(&self, archive_file_path: String) -> GeorgeResult<()> {
        self.inner().archive(archive_file_path)
    }
}

/// `ge`文件对象
#[derive(Clone, Debug)]
pub struct GeImpl {
    /// `ge`文件地址
    filepath: String,
    /// 文件元数据信息，长度52字节
    metadata: Metadata,
    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 需要借助对象包裹，以便更新file，避免self为mut
    filed: Filed,
}

#[derive(Clone, Debug)]
pub struct GeFactory;
