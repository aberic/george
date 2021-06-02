// /*
//  * Copyright (c) 2020. Aberic - All Rights Reserved.
//  *
//  * Licensed under the Apache License, Version 2.0 (the "License");
//  * you may not use this file except in compliance with the License.
//  * You may obtain a copy of the License at
//  * http://www.apache.org/licenses/LICENSE-2.0
//  * Unless required by applicable law or agreed to in writing, software
//  * distributed under the License is distributed on an "AS IS" BASIS,
//  * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  * See the License for the specific language governing permissions and
//  * limitations under the License.
//  */
//
// use crate::task::engine::traits::TForm;
// use crate::task::engine::RootBytes;
// use crate::utils::enums::KeyType;
// use crate::utils::writer::Filed;
// use std::sync::{Arc, RwLock};
//
// /// 默克尔树
// ///
// /// 默克尔树在ge中的定义如下：
// ///
// /// 无法凭空创建一棵默克尔树，即默克尔树的存在条件是必须拥有叶子结点
// ///
// /// 创建默克尔树的唯一方法是传入一个叶子结点开始
// ///
// /// 默克尔树的hash传递是非计算必要的，即通过左右叶子结点计算上层hash值时，如果右叶子结点为空，那么上层hash值直接继承左叶子节点hash值
// ///
// /// 默克尔树的叶子结点永远不会是根结点
// ///
// /// 默克尔树最小层数为2，如果仅存在一个叶子结点，那么是一棵两层树，且为树的左叶子结点
// ///
// /// 如果是空树，则遵循上述要求，创建两层树，并赋值底层左叶子结点
// ///
// /// 默克尔树每层都能容纳固定数量的叶子结点，且总数量等于2^(level-1)
// ///
// /// 设总数量为N，层数为L，当默克尔树层数为2时，即L=2，N=2^(2-1)=2，表示当前两层默克尔树能容纳2个叶子结点；同理当L=5，N=2^(5-1)=16
// ///
// /// 默克尔树允许存在孤儿叶子结点以及孤儿中间结点，但不会存在孤儿根节点，根节点永远都会有两个子节点，且允许右叶子结点为空
// ///
// /// 默克尔树的左子树永远是满载的，即新增结点时总会向右子树进行寻道，并且是递归寻道，寻道最底层，即L=1时，优先放置在左叶子节点
// ///
// /// 当新增结点发现当前默克尔树已经满载，则整棵树除了根结点外整体下沉并成为根结点的左子树，新增结点从根结点右子树中寻找空叶子结点位
// pub struct Node {
//     form: Arc<RwLock<dyn TForm>>,
//     index_name: String,
//     key_type: KeyType,
//     index_path: String,
//     /// 当前层高
//     level: u32,
//     /// 当前结点hash
//     hash: String,
//     /// 当前子结点数量
//     count: u32,
//     /// 索引文件路径
//     ///
//     /// 当有新的数据加入时，新数据存储地址在`node_file`中记录8字节
//     node_filepath: String,
//     /// 根据文件路径获取该文件追加写入的写对象
//     ///
//     /// 需要借助对象包裹，以便更新file，避免self为mut
//     node_filer: Filed,
//     /// 存储根结点所属各子结点坐标顺序字节数组
//     ///
//     /// 子项是64位node集合，在node集合中每一个node的默认字节长度是14(下一结点指针8字节 + 当前数据指针6字节)，数量是1170，即一次性读取16380个字节
//     root_bytes: Arc<RwLock<RootBytes>>,
// }
