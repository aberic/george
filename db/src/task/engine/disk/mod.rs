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

use ge::Ge;

use crate::task::engine::traits::TForm;
use crate::task::engine::RootBytes;
use crate::utils::enums::KeyType;

pub(crate) mod node;

/// 索引B+Tree结点结构
///
/// 包含了索引的根结点、子结点以及叶子结点
///
/// 叶子结点中才会存在Link，其余结点Link为None
///
/// 子项是64位node集合，在node集合中每一个node的默认字节长度是14(下一结点指针8字节 + 当前数据指针6字节)，数量是1170，即一次性读取16380个字节
///
/// 叶子结点没有下一结点指针，只有当前数据指针6字节，数量1170，即一次性读取7020个字节
///
/// record文件最大为2^(8*8)=16348PB
///
/// record存储固定长度的数据，长度为20，即view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节) + 链式后续数据(8字节)
/// 即view视图真实数据12+链式后续数据8，总计可存(2^64)/20条数据
#[derive(Debug, Clone)]
pub(crate) struct Node {
    form: Arc<RwLock<dyn TForm>>,
    index_name: String,
    key_type: KeyType,
    index_path: String,
    /// 是否唯一索引
    unique: bool,
    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 当有新的数据加入时，新数据存储地址在`node_file`中记录8字节
    ///
    /// 该项与`unique`和`record_filepath`组合使用
    ///
    /// 当`unique`为true时，则存储的8字节为view视图真实数据地址
    ///
    /// 当`unique`为false时，则与`record_file`搭配使用，启动碰撞链式结构
    ///
    /// 需要借助对象包裹，以便更新file，避免self为mut
    node_ge: Arc<dyn Ge>,
    /// 根据文件路径获取该文件追加写入的写对象
    /// * 用于记录重复索引链式结构信息
    /// * 当有新的数据加入时，新数据存储地址在`node_file`中记录8字节，为`数据地址`
    /// * `数据地址`指向`record_file`中起始偏移量，持续20字节。
    /// 由`view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节) + 下一数据地址(8字节)`组成
    /// * 当下一数据地址为空时，则表示当前链式结构已到尾部
    /// * 当`unique`为true时，该项不启用
    ///
    /// 需要借助对象包裹，以便更新file，避免self为mut
    record_ge: Arc<dyn Ge>,
    /// 存储根结点所属各子结点坐标顺序字节数组
    ///
    /// 子项是64位node集合，在node集合中每一个node的默认字节长度是14(下一结点指针8字节 + 当前数据指针6字节)，数量是1170，即一次性读取16380个字节
    root_bytes: Arc<RwLock<RootBytes>>,
}
