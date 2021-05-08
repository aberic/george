/*
 * Copyright (c) 2020. Aberic - All Rights Reserved.
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

use std::fmt::Debug;
use std::sync::{Arc, RwLock};

use comm::errors::entrances::{GeorgeError, GeorgeResult};

use crate::task::engine::memory::seed::Seed;
use crate::task::rich::Condition;
use crate::utils::comm::{hash_key_64, level_distance_64};
use crate::utils::enums::KeyType;
use comm::errors::children::{DataExistError, DataNoExistError, MethodNoSupportError, NoneError};

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

/// 新建根结点
///
/// 该结点没有Links，也没有preNode，是B+Tree的创世结点
fn create_root_self() -> Node {
    let nodes: Option<Arc<RwLock<Vec<Arc<Node>>>>> = Some(Arc::new(RwLock::new(Vec::new())));
    return Node {
        degree_index: 0,
        nodes,
        seeds: None,
    };
}

/// 新建普通结点
///
/// 该结点需要定义层和度
///
/// 该结点需要指定上一结点，该方法不判断上一结点是否为None，但在检索等操作时可能会造成该结果丢失
///
/// 该结点下真实存储数据的集合必然为None
fn create_node_self(degree_index: u16) -> Node {
    let nodes: Option<Arc<RwLock<Vec<Arc<Node>>>>> = Some(Arc::new(RwLock::new(Vec::new())));
    return Node {
        degree_index,
        nodes,
        seeds: None,
    };
}

/// 新建叶子结点
///
/// 该结点需要定义层和度
///
/// 该结点需要指定上一结点，该方法不判断上一结点是否为None，但在检索等操作时可能会造成该结果丢失
///
/// 该结点的子结点集合必然为None
fn create_leaf_self(degree_index: u16) -> Node {
    let seeds: Option<Arc<RwLock<Vec<Arc<RwLock<Seed>>>>>> =
        Some(Arc::new(RwLock::new(Vec::new())));
    return Node {
        degree_index,
        nodes: None,
        seeds,
    };
}

impl Node {
    /// 新建根结点s
    ///
    /// 该结点没有Seeds，也没有preNode，是B+Tree的创世结点
    pub(crate) fn create() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(create_root_self()))
    }

    /// 恢复根结点
    pub(crate) fn recovery() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(create_root_self()))
    }
}

/// 封装方法函数
impl Node {
    /// 当前结点所在集合中的索引下标，该坐标不一定在数组中的正确位置，但一定是逻辑正确的
    fn degree_index(&self) -> u16 {
        self.degree_index
    }

    /// 子结点集合Vec，允许为空Option，多线程共享数据Arc，支持并发操作RWLock，集合内存储指针Box，指针类型为Node
    fn nodes(&self) -> Option<Arc<RwLock<Vec<Arc<Self>>>>> {
        self.nodes.clone()
    }

    /// 叶子结点下真实存储数据的集合，该集合主要目的在于解决Hash碰撞，允许为空Option，多线程共享数据Arc，
    /// 支持并发操作RWLock，集合内存储指针Box，指针类型为Seed
    fn seeds(&self) -> Option<Arc<RwLock<Vec<Arc<RwLock<Seed>>>>>> {
        self.seeds.clone()
    }
}

/// 封装方法函数
impl Node {
    pub(crate) fn put(&self, key: String, value: Vec<u8>, force: bool) -> GeorgeResult<()> {
        let flexible_key = hash_key_64(KeyType::String, key.clone())?;
        let seed = Arc::new(RwLock::new(Seed::create(key, value)));
        self.put_in_node(1, flexible_key, seed, force)
    }

    pub(crate) fn get(&self, key: String) -> GeorgeResult<Vec<u8>> {
        let flexible_key = hash_key_64(KeyType::String, key.clone())?;
        self.get_in_node(1, key, flexible_key)
    }

    pub(crate) fn del(&self, key: String) -> GeorgeResult<()> {
        let flexible_key = hash_key_64(KeyType::String, key.clone())?;
        self.del_in_node(1, key, flexible_key)
    }
}

impl Node {
    /// 插入数据<p><p>
    ///
    /// ###Params
    ///
    /// hash_key 索引key，可通过hash转换string生成，如果是自增ID则自动生成，长度为无符号64位整型，是数据存放于
    /// 索引树中的坐标
    ///
    /// md516_key 有一部分是通过将hash_key字符串化后再经过md5处理并取8-24位字符获取；也有可能是在hash_key取得
    /// 基础值后，还没有得到最终值前的过渡值经过md5处理并取8-24位字符获取；获取方式不定，但总归是通过hash_key进行
    /// 处理获取。该值用于解决B+Tree中hash碰撞的问题，在每一个叶子结点处都会进行数组对象存储，数组中存储的对象结构
    /// 如下所示：
    ///
    /// ```
    /// struct Seed {
    ///     key: String, // 当前结果原始key信息
    ///     value: Vec<u8>, // 当前结果value信息
    /// }
    /// ```
    ///
    /// flexible_key 下一级最左最小树所对应真实key<p><p>
    ///
    /// value 当前结果value信息<p><p>
    ///
    /// ###Return
    ///
    /// IndexResult<()>
    fn put_in_node(
        &self,
        level: u8,
        flexible_key: u64,
        seed: Arc<RwLock<Seed>>,
        force: bool,
    ) -> GeorgeResult<()> {
        let next_flexible_key: u64;
        let distance = level_distance_64(level);
        let next_degree = (flexible_key / distance) as u16;
        next_flexible_key = flexible_key - next_degree as u64 * distance;
        let node_next: Arc<Node>;
        let node_next_level = level + 1;
        if level.eq(&4) {
            // 创建或获取下一个叶子节点
            node_next = self.create_or_take_leaf(next_degree);
            node_next.put_seed(seed, force)
        } else {
            // 创建或获取下一个子节点
            node_next = self.create_or_take_node(next_degree);
            node_next.put_in_node(node_next_level, next_flexible_key, seed, force)
        }
    }

    /// 新建普通结点
    ///
    /// 该结点需要定义层和度
    ///
    /// 该结点需要指定上一结点，该方法不判断上一结点是否为None，但在检索等操作时可能会造成该结果丢失
    ///
    /// 该结点下真实存储数据的集合必然为None
    fn create_node(&self, degree_index: u16) -> Arc<Self> {
        return Arc::new(create_node_self(degree_index));
    }

    /// 新建叶子结点
    ///
    /// 该结点需要定义层和度
    ///
    /// 该结点需要指定上一结点，该方法不判断上一结点是否为None，但在检索等操作时可能会造成该结果丢失
    ///
    /// 该结点的子结点集合必然为None
    fn create_leaf(&self, degree_index: u16) -> Arc<Self> {
        return Arc::new(create_leaf_self(degree_index));
    }

    fn create_or_take_node(&self, index: u16) -> Arc<Node> {
        self.create_or_take(index, false)
    }

    fn create_or_take_leaf(&self, index: u16) -> Arc<Node> {
        self.create_or_take(index, true)
    }

    /// 从指定父节点中尝试取出一个子节点，如果子节点不存在则创建一个并返回
    ///
    /// node 指定父节点
    ///
    /// leaf 是否叶子节点
    fn create_or_take(&self, index: u16, leaf: bool) -> Arc<Node> {
        return match self.binary_match_data_pre(index) {
            Ok(next_node) => {
                // 节点存在，且返回节点在集合中的正确下标
                next_node.clone()
            }
            _ => {
                // 节点不存在，新建节点并返回
                let node_next: Arc<Node>;
                if leaf {
                    node_next = self.create_leaf(index);
                } else {
                    node_next = self.create_node(index);
                }
                // 子节点集合不可能不存在，如果不存在，则抛出系统异常
                let nodes = self.nodes().unwrap();
                let mut nodes_w = nodes.write().unwrap();
                nodes_w.push(node_next.clone());
                nodes_w.sort_by(|a, b| a.degree_index().cmp(&b.degree_index()));
                // add_child_node(self, node_next.clone());
                node_next
            }
        };
    }

    /// 节点数组二分查找基本方法前置方法
    ///
    /// node 所查找的集合根
    ///
    /// match_index 在该集合中真实的下标位置
    pub(super) fn binary_match_data_pre(&self, match_index: u16) -> GeorgeResult<Arc<Node>> {
        return match self.nodes() {
            Some(arc_nodes) => {
                if arc_nodes.clone().read().unwrap().clone().len() > 0 {
                    self.binary_match_data(arc_nodes.clone(), match_index)
                } else {
                    Err(GeorgeError::from(DataNoExistError))
                }
            }
            None => Err(GeorgeError::from(NoneError)),
        };
    }

    /// 节点数组二分查找基本方法<p><p>
    ///
    /// ###Params
    ///
    /// match_index 要查找的值<p><p>
    ///
    /// ###Return
    ///
    /// real_index 返回查找到的真实的元素下标，该下标是对应数组内的下标，并非树中节点数组原型的下标
    ///
    /// 如果没找到，则返回err
    fn binary_match_data(
        &self,
        nodes: Arc<RwLock<Vec<Arc<Node>>>>,
        match_index: u16,
    ) -> GeorgeResult<Arc<Node>> {
        let nodes = nodes.read().unwrap();
        let mut left_index: usize = 0;
        let mut middle_index: usize;
        let mut right_index: usize = nodes.len() - 1;
        while left_index <= right_index {
            middle_index = (left_index + right_index) / 2;
            // 如果要找的数比midVal大
            if nodes[middle_index].degree_index() > match_index {
                // 在arr数组的左边找
                if middle_index > 0 {
                    right_index = middle_index - 1
                } else {
                    return Err(GeorgeError::from(DataNoExistError));
                }
            } else if nodes[middle_index].degree_index() < match_index {
                // 在arr数组的右边找
                left_index = middle_index + 1
            } else if nodes[middle_index].degree_index() == match_index {
                return Ok(nodes.get(middle_index).unwrap().clone());
            }
        }
        Err(GeorgeError::from(DataNoExistError))
    }

    fn put_seed(&self, seed: Arc<RwLock<Seed>>, force: bool) -> GeorgeResult<()> {
        // 获取seed叶子，如果存在，则判断版本号，如果不存在，则新建一个空并返回
        return if force {
            self.exist_seed_save_force(seed.clone());
            self.seeds().unwrap().write().unwrap().push(seed.clone());
            Ok(())
        } else {
            if self.exist_seed(seed.read().unwrap().key()) {
                Err(GeorgeError::from(DataExistError))
            } else {
                self.seeds().unwrap().write().unwrap().push(seed.clone());
                Ok(())
            }
        };
    }

    /// 指定节点中是否存在匹配key的seed
    ///
    /// 该方法用于put类型，即如果存在，则返回已存在值，没有额外操作
    fn exist_seed(&self, key: String) -> bool {
        let arc = self.seeds().clone().unwrap().clone();
        let seeds = arc.read().unwrap();
        let mut seeds_rm_position = vec![];
        let mut position: usize = 0;
        let mut res = false;
        for seed in seeds.clone().iter() {
            let seed_r = seed.read().unwrap();
            if seed_r.is_none() {
                seeds_rm_position.push(position);
                position += 1;
                continue;
            }
            if seed_r.key().eq(&key) {
                res = true;
            }
            position += 1;
        }
        for position in seeds_rm_position {
            seeds.clone().remove(position);
        }
        res
    }

    /// 指定节点中是否存在匹配md516_key的seed
    ///
    /// 该方法用于set类型，即如果存在，则将待插入seed内容变更为已存在内容，同时删除已存在seed
    ///
    /// 这一步操作是为了方便前置传参方法更新seed索引数据，已达到真实存储的目的
    fn exist_seed_save_force(&self, seed_new: Arc<RwLock<Seed>>) {
        let arc = self.seeds().clone().unwrap().clone();
        let mut seeds = arc.write().unwrap();
        let mut seeds_rm_position = vec![];
        for (position, seed) in seeds.clone().iter_mut().enumerate() {
            let seed_r = seed.read().unwrap();
            if seed_r.is_none() {
                seeds_rm_position.push(position);
                continue;
            }
            let seed_new_w = seed_new.write().unwrap();
            if seed_r.key().eq(&seed_new_w.key()) {
                seeds_rm_position.push(position);
            }
        }
        for position in seeds_rm_position {
            seeds.remove(position);
        }
    }

    /// 获取数据，返回存储对象<p><p>
    ///
    /// ###Params
    ///
    /// hash_key 索引key，可通过hash转换string生成，如果是自增ID则自动生成，长度为无符号64位整型，是数据存放于
    /// 索引树中的坐标
    ///
    /// md516_key 有一部分是通过将hash_key字符串化后再经过md5处理并取8-24位字符获取；也有可能是在hash_key取得
    /// 基础值后，还没有得到最终值前的过渡值经过md5处理并取8-24位字符获取；获取方式不定，但总归是通过hash_key进行
    /// 处理获取。该值用于解决B+Tree中hash碰撞的问题，在每一个叶子结点处都会进行数组对象存储，数组中存储的对象结构
    /// 如下所示：
    ///
    /// ```
    /// struct Seed {
    ///     key: String, // 当前结果原始key信息
    ///     value: Vec<u8>, // 当前结果value信息
    /// }
    /// ```
    ///
    /// flexible_key 下一级最左最小树所对应真实key<p><p>
    ///
    /// ###Return
    ///
    /// Seed value信息
    fn get_in_node(&self, level: u8, key: String, flexible_key: u64) -> GeorgeResult<Vec<u8>> {
        let next_flexible_key: u64;
        let next_degree: u16;
        if level.lt(&5) {
            let distance = level_distance_64(level);
            next_degree = (flexible_key / distance) as u16;
            next_flexible_key = flexible_key - next_degree as u64 * distance;
        } else {
            // 获取seed叶子，如果存在，则判断版本号，如果不存在，则新建一个空并返回
            return self.get_seed_value(key);
        };
        let node_next = self.binary_match_data_pre(next_degree)?;
        node_next.get_in_node(level + 1, key, next_flexible_key)
    }

    /// 指定节点中是否存在匹配md516_key的seed
    ///
    /// 该方法用于get类型，在检索的同时会删除已发现的空seed
    fn get_seed_value(&self, key: String) -> GeorgeResult<Vec<u8>> {
        let arc = self.seeds().clone().unwrap().clone();
        let mut seeds = arc.write().unwrap();
        let mut seeds_rm_position = vec![];
        let mut exist = false;
        let mut vu8 = vec![];
        for (position, seed) in seeds.iter().enumerate() {
            let seed_r = seed.read().unwrap();
            if seed_r.is_none() {
                seeds_rm_position.push(position);
                continue;
            }
            if seed_r.key().eq(&key) {
                exist = true;
                vu8 = seed_r.value()?;
            }
        }
        for position in seeds_rm_position {
            seeds.remove(position);
        }
        if exist {
            Ok(vu8)
        } else {
            Err(GeorgeError::from(DataNoExistError))
        }
    }

    /// 删除数据<p><p>
    ///
    /// ###Params
    ///
    /// hash_key 索引key，可通过hash转换string生成，如果是自增ID则自动生成，长度为无符号64位整型，是数据存放于
    /// 索引树中的坐标
    ///
    /// md516_key 有一部分是通过将hash_key字符串化后再经过md5处理并取8-24位字符获取；也有可能是在hash_key取得
    /// 基础值后，还没有得到最终值前的过渡值经过md5处理并取8-24位字符获取；获取方式不定，但总归是通过hash_key进行
    /// 处理获取。该值用于解决B+Tree中hash碰撞的问题，在每一个叶子结点处都会进行数组对象存储，数组中存储的对象结构
    /// 如下所示：
    ///
    /// ```
    /// struct Seed {
    ///     key: String, // 当前结果原始key信息
    ///     value: Vec<u8>, // 当前结果value信息
    /// }
    /// ```
    ///
    /// flexible_key 下一级最左最小树所对应真实key<p><p>
    ///
    /// ###Return
    ///
    /// Seed value信息
    fn del_in_node(&self, level: u8, key: String, flexible_key: u64) -> GeorgeResult<()> {
        let next_flexible_key: u64;
        let next_degree: u16;
        if level.lt(&5) {
            let distance = level_distance_64(level);
            next_degree = (flexible_key / distance) as u16;
            next_flexible_key = flexible_key - next_degree as u64 * distance;
        } else {
            // 获取seed叶子，如果存在，则判断版本号，如果不存在，则新建一个空并返回
            return self.remove_seed_value(key);
        };
        match self.binary_match_data_pre(next_degree) {
            Ok(node_next) => node_next.del_in_node(level + 1, key, next_flexible_key),
            _ => Ok(()),
        }
    }

    /// 删除指定节点中存在匹配md516_key的seed
    fn remove_seed_value(&self, key: String) -> GeorgeResult<()> {
        let arc = self.seeds().clone().unwrap().clone();
        let mut seeds = arc.write().unwrap();
        let mut seeds_rm_position = vec![];
        for (position, seed) in seeds.iter().enumerate() {
            let seed_r = seed.read().unwrap();
            if seed_r.is_none() {
                seeds_rm_position.push(position);
                continue;
            }
            if seed_r.key().eq(&key) {
                seeds_rm_position.push(position);
                break;
            }
        }
        for position in seeds_rm_position {
            seeds.remove(position);
        }
        Ok(())
    }
}
