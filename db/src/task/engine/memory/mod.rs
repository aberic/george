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

pub(crate) mod node;
mod node_test;
pub(crate) mod seed;

/// 索引B+Tree结点结构
///
/// 包含了索引的根结点、子结点以及叶子结点
///
/// 叶子结点中才会存在Link，其余结点Link为None
///
/// 持久化存储格式 {dataDir}/database/{dataName}/{formName}.form...
#[derive(Debug)]
pub(crate) struct Node {
    /// 当前结点所在集合中的索引下标，该坐标不一定在数组中的正确位置，但一定是逻辑正确的
    degree_index: u16,
    /// 子结点集合Vec，允许为空Option，多线程共享数据Arc，支持并发操作RWLock，集合内存储指针Box，指针类型为Node
    nodes: Option<Arc<RwLock<Vec<Arc<Node>>>>>,
    /// 叶子结点下真实存储数据的集合，该集合主要目的在于解决Hash碰撞，允许为空Option，多线程共享数据Arc，
    /// 支持并发操作RWLock，集合内存储指针Box，指针类型为Link
    seeds: Option<Arc<RwLock<Vec<Arc<RwLock<Seed>>>>>>,
}

/// B+Tree索引叶子结点内防hash碰撞数组结构中单体结构
///
/// 搭配Index使用
///
/// 叶子节点下真实存储数据的集合单体结构
#[derive(Debug)]
pub(crate) struct Seed {
    /// 当前结果原始key信息
    key: String,
    value: Option<Vec<u8>>,
    old_value: Option<Vec<u8>>,
}
