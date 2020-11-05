use std::fmt::Debug;
use std::sync::{Arc, RwLock};

use comm::bytes::create_empty_bytes;
use comm::cryptos::hash::hashcode32_enhance;
use comm::errors::entrances::GeorgeResult;
use comm::vectors;

use crate::engine::siam::comm::{read_next_nodes_bytes, read_seed_bytes, write_seed_bytes};
use crate::engine::siam::traits::{DiskNode, TNode};
use crate::engine::traits::TSeed;
use crate::utils::comm::{level_distance_32, level_distance_64, LevelType};
use crate::utils::path::{index_file_path, view_file_path};

/// 索引B+Tree结点结构
///
/// 包含了索引的根结点、子结点以及叶子结点
///
/// 叶子结点中才会存在Link，其余结点Link为None
///
/// 持久化存储格式 {dataDir}/database/{dataName}/{formName}.form...
#[derive(Debug)]
pub(crate) struct Node {
    /// 库ID
    database_id: String,
    /// 视图ID
    view_id: String,
    /// 存储结点所属各子结点坐标顺序字符串
    ///
    /// 如果子项是node集合，在node集合中每一个node的默认字节长度是8，数量是256，即一次性读取2048个字节
    ///
    /// 如果子项是seed集合，在seed集合中每一个seed的默认字符长度是6，当前叶子node会存储叶子中首个出现hash碰撞的
    /// seed起始坐标，每一个seed都会存储出现hash碰撞的下一seed起始坐标
    node_bytes: Arc<RwLock<Vec<u8>>>,
}

/// 新建根结点
///
/// 该结点没有Links，也没有preNode，是B+Tree的创世结点
fn create_root_self(database_id: String, view_id: String) -> Node {
    return Node {
        database_id,
        view_id,
        node_bytes: Arc::new(RwLock::new(create_empty_bytes(2048))),
    };
}

fn create_empty() -> Node {
    return Node {
        database_id: "".to_string(),
        view_id: "".to_string(),
        node_bytes: Arc::new(Default::default()),
    };
}

impl Node {
    pub fn create_root(database_id: String, view_id: String) -> Arc<Self> {
        return Arc::new(create_root_self(database_id, view_id));
    }
}

/// 封装方法函数
impl TNode for Node {
    fn degree_index(&self) -> u16 {
        0
    }
    fn nodes(&self) -> Option<Arc<RwLock<Vec<Arc<Self>>>>> {
        None
    }
    fn seeds(&self) -> Option<Arc<RwLock<Vec<Arc<RwLock<dyn TSeed>>>>>> {
        None
    }
    fn node_bytes(&self) -> Arc<RwLock<Vec<u8>>> {
        self.node_bytes.clone()
    }
    fn set_node_bytes(&self, bytes: Vec<u8>) {
        let node_bytes = self.node_bytes();
        let mut nb_w = node_bytes.write().unwrap();
        nb_w.copy_from_slice(bytes.as_slice())
    }
    fn create_node(_degree_index: u16) -> Arc<Self> {
        return Arc::new(create_empty());
    }
    fn create_leaf(_degree_index: u16) -> Arc<Self> {
        return Arc::new(create_empty());
    }
    fn put(
        &self,
        key: String,
        seed: Arc<RwLock<dyn TSeed>>,
        force: bool,
        index_file_name: String,
        description_len: usize,
        level_type: LevelType,
    ) -> GeorgeResult<()>
    where
        Self: Sized,
    {
        let hash_key = hashcode32_enhance(key);
        let index_file_path =
            index_file_path(self.database_id(), self.view_id(), index_file_name.clone());
        let node_bytes = self.node_bytes().read().unwrap().to_vec();
        let view_file_path = view_file_path(self.database_id(), self.view_id());
        self.put_32_in_node(
            node_bytes,
            1,
            hash_key,
            seed,
            force,
            true,
            index_file_name,
            index_file_path,
            view_file_path,
            description_len as u64,
        )
    }
    fn get(
        &self,
        key: String,
        index_file_name: String,
        description_len: usize,
        level_type: LevelType,
    ) -> GeorgeResult<Vec<u8>>
    where
        Self: Sized,
    {
        let hash_key = hashcode32_enhance(key.clone());
        let index_file_path =
            index_file_path(self.database_id(), self.view_id(), index_file_name.clone());
        let node_bytes = self.node_bytes().read().unwrap().to_vec();
        let view_file_path = view_file_path(self.database_id(), self.view_id());
        self.get_32_in_node(
            node_bytes,
            1,
            hash_key,
            true,
            index_file_name,
            index_file_path,
            view_file_path,
            description_len as u64,
        )
    }
}

impl DiskNode for Node {
    fn database_id(&self) -> String {
        self.database_id.clone()
    }
    fn view_id(&self) -> String {
        self.view_id.clone()
    }
    fn modify_node_bytes(&self, start: usize, vs: Vec<u8>) {
        let nb = self.node_bytes();
        let mut nb_w = nb.write().unwrap();
        let nb_n = vectors::modify(nb_w.to_vec(), vs, start);
        nb_w.copy_from_slice(nb_n.as_slice())
    }
    fn put_32_in_node(
        &self,
        node_bytes: Vec<u8>,
        level: u8,
        flexible_key: u32,
        seed: Arc<RwLock<dyn TSeed>>,
        force: bool,
        root: bool,
        index_file_name: String,
        index_file_path: String,
        view_file_path: String,
        next_node_seek: u64,
    ) -> GeorgeResult<()>
    where
        Self: Sized,
    {
        // 通过当前树下一层高获取结点间间隔数量，即每一度中存在的元素数量
        let distance = level_distance_32(level);
        // 通过当前层真实key除以下一层间隔数获取结点处在下一层的度数
        let next_degree = (flexible_key / distance) as u16;
        // 通过当前层真实key减去下一层的度数与间隔数的乘机获取结点所在下一层的真实key
        let next_flexible_key = flexible_key - next_degree as u32 * distance;
        // 如果当前层高为4，则达到最底层，否则递归下一层逻辑
        if level == 4 {
            write_seed_bytes(
                node_bytes,
                index_file_path,
                view_file_path,
                next_node_seek,
                (next_degree * 8) as u64, // 在当前操作结点的字节数组的起始位置
                force,
                seed,
            )
        } else {
            // 下一结点状态
            // 下一结点node_bytes
            // 下一结点起始坐标seek
            // 在node集合中每一个node的默认字节长度是8，数量是256，即一次性读取2048个字节
            let (node_bytes, seek) = read_next_nodes_bytes(
                self,
                node_bytes,
                index_file_name.clone(),
                index_file_path.clone(),
                next_node_seek,
                (next_degree * 8) as u64, // 在当前操作结点的字节数组的起始位置
                root,
                true,
            )?;
            self.put_32_in_node(
                node_bytes,
                level + 1,
                next_flexible_key,
                seed,
                force,
                false,
                index_file_name,
                index_file_path,
                view_file_path,
                seek,
            )
        }
    }
    fn get_32_in_node(
        &self,
        node_bytes: Vec<u8>,
        level: u8,
        flexible_key: u32,
        root: bool,
        index_file_name: String,
        index_file_path: String,
        view_file_path: String,
        node_seek: u64,
    ) -> GeorgeResult<Vec<u8>>
    where
        Self: Sized,
    {
        let distance = level_distance_32(level);
        let next_degree = (flexible_key / distance) as u16;
        let next_flexible_key = flexible_key - next_degree as u32 * distance;
        if level == 4 {
            read_seed_bytes(node_bytes, view_file_path, (next_degree * 8) as u64)
        } else {
            // 下一结点状态
            // 下一结点node_bytes
            // 下一结点起始坐标seek
            // 在node集合中每一个node的默认字节长度是8，数量是256，即一次性读取2048个字节
            let (node_bytes, seek) = read_next_nodes_bytes(
                self,
                node_bytes,
                index_file_name.clone(),
                index_file_path.clone(),
                node_seek,
                (next_degree * 8) as u64,
                root,
                false,
            )?;
            self.get_32_in_node(
                node_bytes,
                level + 1,
                next_flexible_key,
                false,
                index_file_name,
                index_file_path,
                view_file_path,
                seek,
            )
        }
    }
    fn put_64_in_node(
        &self,
        node_bytes: Vec<u8>,
        level: u8,
        flexible_key: u64,
        seed: Arc<RwLock<dyn TSeed>>,
        force: bool,
        root: bool,
        index_file_name: String,
        index_file_path: String,
        view_file_path: String,
        next_node_seek: u64,
    ) -> GeorgeResult<()>
    where
        Self: Sized,
    {
        // 通过当前树下一层高获取结点间间隔数量，即每一度中存在的元素数量
        let distance = level_distance_64(level);
        // 通过当前层真实key除以下一层间隔数获取结点处在下一层的度数
        let next_degree = (flexible_key / distance) as u16;
        // 通过当前层真实key减去下一层的度数与间隔数的乘机获取结点所在下一层的真实key
        let next_flexible_key = flexible_key - next_degree as u64 * distance;
        // 如果当前层高为4，则达到最底层，否则递归下一层逻辑
        if level == 4 {
            write_seed_bytes(
                node_bytes,
                index_file_path,
                view_file_path,
                next_node_seek,
                (next_degree * 8) as u64, // 在当前操作结点的字节数组的起始位置
                force,
                seed,
            )
        } else {
            // 下一结点状态
            // 下一结点node_bytes
            // 下一结点起始坐标seek
            // 在node集合中每一个node的默认字节长度是8，数量是256，即一次性读取2048个字节
            let (node_bytes, seek) = read_next_nodes_bytes(
                self,
                node_bytes,
                index_file_name.clone(),
                index_file_path.clone(),
                next_node_seek,
                (next_degree * 8) as u64, // 在当前操作结点的字节数组的起始位置
                root,
                true,
            )?;
            self.put_64_in_node(
                node_bytes,
                level + 1,
                next_flexible_key,
                seed,
                force,
                false,
                index_file_name,
                index_file_path,
                view_file_path,
                seek,
            )
        }
    }
    fn get_64_in_node(
        &self,
        node_bytes: Vec<u8>,
        level: u8,
        flexible_key: u64,
        root: bool,
        index_file_name: String,
        index_file_path: String,
        view_file_path: String,
        node_seek: u64,
    ) -> GeorgeResult<Vec<u8>>
    where
        Self: Sized,
    {
        let distance = level_distance_64(level);
        let next_degree = (flexible_key / distance) as u16;
        let next_flexible_key = flexible_key - next_degree as u64 * distance;
        if level == 4 {
            read_seed_bytes(node_bytes, view_file_path, (next_degree * 8) as u64)
        } else {
            // 下一结点状态
            // 下一结点node_bytes
            // 下一结点起始坐标seek
            // 在node集合中每一个node的默认字节长度是8，数量是256，即一次性读取2048个字节
            let (node_bytes, seek) = read_next_nodes_bytes(
                self,
                node_bytes,
                index_file_name.clone(),
                index_file_path.clone(),
                node_seek,
                (next_degree * 8) as u64,
                root,
                false,
            )?;
            self.get_64_in_node(
                node_bytes,
                level + 1,
                next_flexible_key,
                false,
                index_file_name,
                index_file_path,
                view_file_path,
                seek,
            )
        }
    }
}
