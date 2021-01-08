use std::fmt::Debug;
use std::sync::{Arc, RwLock};

use comm::cryptos::hash::{hashcode64_enhance, md516};
use comm::errors::entrances::{err_str, GeorgeError, GeorgeResult};

use crate::engine::siam::comm::binary_match_data_pre;
use crate::engine::siam::selector::Constraint;
use crate::engine::siam::traits::TNode;
use crate::engine::traits::TSeed;
use crate::utils::comm::{level_distance_64, IndexMold};
use comm::errors::children::{DataExistError, DataNoExistError};

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
    seeds: Option<Arc<RwLock<Vec<Arc<RwLock<dyn TSeed>>>>>>,
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
    let seeds: Option<Arc<RwLock<Vec<Arc<RwLock<dyn TSeed>>>>>> =
        Some(Arc::new(RwLock::new(Vec::new())));
    return Node {
        degree_index,
        nodes: None,
        seeds,
    };
}

impl Node {
    /// 新建根结点
    ///
    /// 该结点没有Seeds，也没有preNode，是B+Tree的创世结点
    pub(crate) fn create_root() -> Arc<Self> {
        return Arc::new(create_root_self());
    }
}

/// 封装方法函数
impl TNode for Node {
    fn degree_index(&self) -> u16 {
        self.degree_index
    }
    fn nodes(&self) -> Option<Arc<RwLock<Vec<Arc<Self>>>>> {
        self.nodes.clone()
    }
    fn seeds(&self) -> Option<Arc<RwLock<Vec<Arc<RwLock<dyn TSeed>>>>>> {
        self.seeds.clone()
    }
    fn node_bytes(&self) -> Arc<RwLock<Vec<u8>>> {
        Arc::new(RwLock::new(vec![0x00]))
    }
    fn set_node_bytes(&self, _bytes: Vec<u8>) {}
    fn put(
        &self,
        key: String,
        seed: Arc<RwLock<dyn TSeed>>,
        force: bool,
        _description_len: usize,
    ) -> GeorgeResult<()>
    where
        Self: Sized,
    {
        self.put_in_node(1, hashcode64_enhance(key), seed, force)
    }
    fn get(&self, key: String) -> GeorgeResult<Vec<u8>> {
        self.get_in_node(1, md516(key.clone()), hashcode64_enhance(key.clone()))
    }
    fn remove(&self, _key: String) -> GeorgeResult<Vec<u8>> {
        // todo
        unimplemented!()
    }
    fn get_last(&self) -> GeorgeResult<Vec<u8>>
    where
        Self: Sized,
    {
        Err(err_str("unimplemented!"))
    }
    fn select(
        &self,
        _mold: IndexMold,
        _left: bool,
        _start: u64,
        _end: u64,
        _constraint: Constraint,
    ) -> GeorgeResult<(u64, u64, Vec<Vec<u8>>)> {
        Err(err_str("unimplemented!"))
    }
    fn delete(
        &self,
        _mold: IndexMold,
        _left: bool,
        _start: u64,
        _end: u64,
        _constraint: Constraint,
    ) -> GeorgeResult<(u64, u64)> {
        Err(err_str("unimplemented!"))
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
        seed: Arc<RwLock<dyn TSeed>>,
        force: bool,
    ) -> GeorgeResult<()> {
        let next_flexible_key: u64;
        let next_degree: u16;
        let distance = level_distance_64(level);
        next_degree = (flexible_key / distance) as u16;
        next_flexible_key = flexible_key - next_degree as u64 * distance;
        let node_next: Arc<Node>;
        let node_next_level = level + 1;
        if level == 4 {
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
        return match binary_match_data_pre(self, index) {
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
    fn put_seed(&self, seed: Arc<RwLock<dyn TSeed>>, force: bool) -> GeorgeResult<()> {
        // 获取seed叶子，如果存在，则判断版本号，如果不存在，则新建一个空并返回
        return if force {
            self.exist_seed_save_force(seed.clone());
            self.seeds().unwrap().write().unwrap().push(seed.clone());
            Ok(())
        } else {
            if self.exist_seed(seed.read().unwrap().key()) {
                Err(GeorgeError::DataExistError(DataExistError))
            } else {
                self.seeds().unwrap().write().unwrap().push(seed.clone());
                Ok(())
            }
        };
    }
    /// 指定节点中是否存在匹配md516_key的seed
    ///
    /// 该方法用于put类型，即如果存在，则返回已存在值，没有额外操作
    fn exist_seed(&self, md516_key: String) -> bool {
        let arc = self.seeds().clone().unwrap().clone();
        let seeds = arc.read().unwrap();
        let mut seeds_rm_position = vec![];
        let mut position: usize = 0;
        let mut res = false;
        for seed in seeds.clone().iter() {
            let seed_r = seed.read().unwrap();
            if seed_r.value().is_none() {
                seeds_rm_position.push(position);
                position += 1;
                continue;
            }
            if seed_r.key().eq(&md516_key) {
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
    fn exist_seed_save_force(&self, seed_new: Arc<RwLock<dyn TSeed>>) {
        let arc = self.seeds().clone().unwrap().clone();
        let seeds = arc.read().unwrap();
        let mut seeds_rm_position = vec![];
        let mut position: usize = 0;
        for seed in seeds.clone().iter_mut() {
            let seed_r = seed.read().unwrap();
            if seed_r.value().is_none() {
                seeds_rm_position.push(position);
                position += 1;
                continue;
            }
            let mut seed_new_w = seed_new.write().unwrap();
            if seed_r.key().eq(&seed_new_w.key()) {
                if !seed_r.value().eq(&seed_new_w.value()) {
                    seed_new_w.modify(seed_r.value().unwrap());
                }
                seeds_rm_position.push(position);
            }
            position += 1;
        }
        for position in seeds_rm_position {
            seeds.clone().remove(position);
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
    pub(super) fn get_in_node(
        &self,
        level: u8,
        md516_key: String,
        flexible_key: u64,
    ) -> GeorgeResult<Vec<u8>> {
        let next_flexible_key: u64;
        let next_degree: u16;
        if level.lt(&5) {
            let distance = level_distance_64(level);
            next_degree = (flexible_key / distance) as u16;
            next_flexible_key = flexible_key - next_degree as u64 * distance;
        } else {
            // 获取seed叶子，如果存在，则判断版本号，如果不存在，则新建一个空并返回
            return self.get_seed_value(md516_key);
        };
        let node_next = binary_match_data_pre(self, next_degree)?;
        node_next.get_in_node(level + 1, md516_key, next_flexible_key)
    }

    /// 指定节点中是否存在匹配md516_key的seed
    ///
    /// 该方法用于get类型，在检索的同时会删除已发现的空seed
    fn get_seed_value(&self, md516_key: String) -> GeorgeResult<Vec<u8>> {
        let arc = self.seeds().clone().unwrap().clone();
        let seeds = arc.read().unwrap();
        let mut seeds_rm_position = vec![];
        let mut position: usize = 0;
        let mut exist = false;
        let mut vu8 = vec![];
        for seed in seeds.iter() {
            let seed_r = seed.read().unwrap();
            if seed_r.value().is_none() {
                seeds_rm_position.push(position);
                position += 1;
                continue;
            }
            if seed_r.key().eq(&md516_key) {
                exist = true;
                vu8 = seed_r.value().unwrap();
            }
            position += 1;
        }
        for position in seeds_rm_position {
            seeds.clone().remove(position);
        }
        if exist {
            Ok(vu8)
        } else {
            Err(GeorgeError::DataNoExistError(DataNoExistError))
        }
    }
}
