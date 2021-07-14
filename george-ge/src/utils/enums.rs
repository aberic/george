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

use crate::utils::{Enum, EnumHandler};
use serde::{Deserialize, Serialize};

impl EnumHandler for Enum {
    fn tag_u8(tag: Tag) -> u8 {
        tag_u8(tag)
    }

    fn tag(b: u8) -> Tag {
        tag(b)
    }
}

/// 文件类型标识符(1字节)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Tag {
    /// 占位
    None,
    /// 引导文件
    Bootstrap,
    /// 缓存页文件
    Page,
    /// 数据库文件
    Database,
    /// 表数据文件
    View,
    /// 索引数据文件
    Index,
    /// 表数据文件
    Ledger,
    /// 表数据文件
    Node,
}

/// 文件类型标识符转字节码
fn tag_u8(tag: Tag) -> u8 {
    match tag {
        Tag::None => 0x00,
        Tag::Bootstrap => 0x01,
        Tag::Database => 0x02,
        Tag::View => 0x03,
        Tag::Index => 0x04,
        Tag::Page => 0x05,
        Tag::Ledger => 0x06,
        Tag::Node => 0x07,
    }
}

/// 字节码转文件类型标识符
fn tag(b: u8) -> Tag {
    match b {
        0x01 => Tag::Bootstrap,
        0x02 => Tag::Database,
        0x03 => Tag::View,
        0x04 => Tag::Index,
        0x05 => Tag::Page,
        0x06 => Tag::Ledger,
        0x07 => Tag::Node,
        _ => Tag::None,
    }
}
