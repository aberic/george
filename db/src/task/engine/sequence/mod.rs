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

use crate::task::engine::traits::TForm;
use crate::utils::enums::KeyType;
use crate::utils::writer::Filed;
use std::sync::{Arc, RwLock};

pub(crate) mod node;

/// 索引B+Tree结点结构
///
/// 包含了索引的根结点、子结点以及叶子结点
///
/// 叶子结点中才会存在Link，其余结点Link为None
#[derive(Debug, Clone)]
pub(crate) struct Node {
    form: Arc<RwLock<dyn TForm>>,
    index_name: String,
    key_type: KeyType,
    /// 索引文件路径
    ///
    /// * 当有新的数据加入时，新数据存储地址在`node_file`中记录12字节。
    /// 由`view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节)`组成
    node_filepath: String,
    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 需要借助对象包裹，以便更新file，避免self为mut
    filer: Filed,
}
