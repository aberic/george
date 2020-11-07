use std::fmt::Debug;
use std::sync::{Arc, RwLock};

use comm::cryptos::hash::{hashcode32_enhance, hashcode64_enhance, md516};
use comm::errors::entrances::{err_str, GeorgeResult};

use crate::engine::siam::comm::{
    get_in_node_u32, get_in_node_u64, put_in_node_u32, put_in_node_u64,
};
use crate::engine::siam::traits::TNode;
use crate::engine::traits::TSeed;
use crate::utils::comm::LevelType;

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
    fn create_node(degree_index: u16) -> Arc<Self> {
        return Arc::new(create_node_self(degree_index));
    }
    fn create_leaf(degree_index: u16) -> Arc<Self> {
        return Arc::new(create_leaf_self(degree_index));
    }
    fn put(
        &self,
        key: String,
        seed: Arc<RwLock<dyn TSeed>>,
        force: bool,
        _description_len: usize,
        level_type: LevelType,
    ) -> GeorgeResult<()>
        where
            Self: Sized,
    {
        match level_type {
            LevelType::Small => put_in_node_u32(self, 1, hashcode32_enhance(key), seed, force),
            LevelType::Large => put_in_node_u64(self, 1, hashcode64_enhance(key), seed, force),
        }
    }
    fn get(
        &self,
        key: String,
        _description_len: usize,
        level_type: LevelType,
    ) -> GeorgeResult<Vec<u8>> {
        match level_type {
            LevelType::Small => {
                get_in_node_u32(self, 1, md516(key.clone()), hashcode32_enhance(key.clone()))
            }
            LevelType::Large => {
                get_in_node_u64(self, 1, md516(key.clone()), hashcode64_enhance(key.clone()))
            }
        }
    }
    fn get_last(&self, _level_type: LevelType) -> GeorgeResult<Vec<u8>>
        where
            Self: Sized,
    {
        Err(err_str("unimplemented!"))
    }
}
