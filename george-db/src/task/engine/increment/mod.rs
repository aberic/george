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

use std::sync::atomic::AtomicU64;
use std::sync::{Arc, RwLock};

use george_ge::Ge;

use crate::task::traits::TForm;

pub(crate) mod node;

/// 索引B+Tree结点结构
///
/// 包含了索引的根结点、子结点以及叶子结点
///
/// 叶子结点中才会存在Link，其余结点Link为None
#[derive(Debug, Clone)]
pub(crate) struct Node {
    form: Arc<RwLock<dyn TForm>>,
    atomic_key: Arc<AtomicU64>,
    index_name: String,
    /// ge文件
    ///
    /// * 当有新的数据加入时，新数据存储地址在`node_file`中记录12字节。
    /// 由`view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节)`组成
    ge: Arc<dyn Ge>,
}
