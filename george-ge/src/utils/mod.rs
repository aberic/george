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

use crate::utils::enums::Tag;
use crate::utils::filed::FiledExec;

pub mod enums;
pub mod filed;

/// `ge`文件枚举工具
pub struct Enum {}

/// `ge`文件枚举方法
pub trait EnumHandler {
    /// 文件类型标识符转字节码
    fn tag_u8(tag: Tag) -> u8;
    /// 字节码转文件类型标识符
    fn tag(b: u8) -> Tag;
}

#[derive(Debug, Clone)]
pub struct Filed {
    filepath: String,
    exec: Arc<RwLock<FiledExec>>,
}
