use std::fmt::Debug;
use std::sync::{Arc, RwLock};

use comm::bytes::create_empty_bytes;
use comm::cryptos::hash::{hashcode32_enhance, hashcode64_enhance};
use comm::errors::entrances::GeorgeResult;
use comm::trans::trans_bytes_2_u64;
use comm::vectors;
use comm::vectors::find_last_eq_bytes;

use crate::engine::siam::comm::{
    read_last_nodes_bytes, read_next_nodes_bytes, read_seed_bytes, read_seed_bytes_from_view,
    write_seed_bytes,
};
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
    /// 索引ID
    index_id: String,
    /// 视图文件路径
    view_file_path: String,
    /// 索引文件路径
    index_file_path: String,
    /// 存储结点所属各子结点坐标顺序字符串
    ///
    /// 如果子项是32位node集合，在node集合中每一个node的默认字节长度是8，数量是256，即一次性读取2048个字节
    ///
    /// 如果子项是64位node集合，在node集合中每一个node的默认字节长度是8，数量是65536，即一次性读取524288个字节
    ///
    /// 如果子项是seed集合，在seed集合中每一个seed的默认字符长度是6，当前叶子node会存储叶子中首个出现hash碰撞的
    /// seed起始坐标，每一个seed都会存储出现hash碰撞的下一seed起始坐标
    node_bytes: Arc<RwLock<Vec<u8>>>,
}

/// 新建根结点
///
/// 该结点没有Links，也没有preNode，是B+Tree的创世结点
fn create_root_self(
    database_id: String,
    view_id: String,
    index_id: String,
    level_type: LevelType,
) -> Node {
    let view_file_path = view_file_path(database_id.clone(), view_id.clone());
    let index_file_path = index_file_path(database_id.clone(), view_id.clone(), index_id.clone());
    match level_type {
        LevelType::Small => Node {
            database_id,
            view_id,
            index_id,
            view_file_path,
            index_file_path,
            node_bytes: Arc::new(RwLock::new(create_empty_bytes(2048))),
        },
        LevelType::Large => Node {
            database_id,
            view_id,
            index_id,
            view_file_path,
            index_file_path,
            node_bytes: Arc::new(RwLock::new(create_empty_bytes(524288))),
        },
    }
}

fn create_empty() -> Node {
    return Node {
        database_id: "".to_string(),
        view_id: "".to_string(),
        index_id: "".to_string(),
        view_file_path: "".to_string(),
        index_file_path: "".to_string(),
        node_bytes: Arc::new(Default::default()),
    };
}

impl Node {
    pub fn create_root(
        database_id: String,
        view_id: String,
        index_id: String,
        level_type: LevelType,
    ) -> Arc<Self> {
        return Arc::new(create_root_self(database_id, view_id, index_id, level_type));
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
        description_len: usize,
        level_type: LevelType,
    ) -> GeorgeResult<()>
    where
        Self: Sized,
    {
        let node_bytes = self.node_bytes().read().unwrap().to_vec();
        match level_type {
            LevelType::Small => self.put_32_in_node(
                node_bytes,
                1,
                hashcode32_enhance(key),
                seed,
                force,
                true,
                description_len as u64,
                level_type,
            ),
            LevelType::Large => self.put_64_in_node(
                node_bytes,
                1,
                hashcode64_enhance(key),
                seed,
                force,
                true,
                description_len as u64,
                level_type,
            ),
        }
    }
    fn get(
        &self,
        key: String,
        description_len: usize,
        level_type: LevelType,
    ) -> GeorgeResult<Vec<u8>> {
        let node_bytes = self.node_bytes().read().unwrap().to_vec();
        match level_type {
            LevelType::Small => self.get_32_in_node(
                node_bytes,
                1,
                hashcode32_enhance(key),
                true,
                description_len as u64,
                level_type,
            ),
            LevelType::Large => self.get_64_in_node(
                node_bytes,
                1,
                hashcode64_enhance(key),
                true,
                description_len as u64,
                level_type,
            ),
        }
    }
}

impl DiskNode for Node {
    fn database_id(&self) -> String {
        self.database_id.clone()
    }
    fn view_id(&self) -> String {
        self.view_id.clone()
    }
    fn index_id(&self) -> String {
        self.index_id.clone()
    }
    fn view_file_path(&self) -> String {
        self.view_file_path.clone()
    }
    fn index_file_path(&self) -> String {
        self.index_file_path.clone()
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
        next_node_seek: u64,
        level_type: LevelType,
    ) -> GeorgeResult<()>
    where
        Self: Sized,
    {
        // 通过当前树下一层高获取结点间间隔数量，即每一度中存在的元素数量
        let distance = level_distance_32(level);
        // 通过当前层真实key除以下一层间隔数获取结点处在下一层的度数
        let next_degree = (flexible_key / distance) as u16;
        // 如果当前层高为4，则达到最底层，否则递归下一层逻辑
        if level == 4 {
            write_seed_bytes(
                node_bytes,
                self.index_file_path(),
                self.view_file_path(),
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
                self.index_id(),
                next_node_seek,
                (next_degree * 8) as u64, // 在当前操作结点的字节数组的起始位置
                root,
                true,
                level_type,
            )?;
            // 通过当前层真实key减去下一层的度数与间隔数的乘机获取结点所在下一层的真实key
            let next_flexible_key = flexible_key - next_degree as u32 * distance;
            self.put_32_in_node(
                node_bytes,
                level + 1,
                next_flexible_key,
                seed,
                force,
                false,
                seek,
                level_type,
            )
        }
    }
    fn get_32_in_node(
        &self,
        node_bytes: Vec<u8>,
        level: u8,
        flexible_key: u32,
        root: bool,
        node_seek: u64,
        level_type: LevelType,
    ) -> GeorgeResult<Vec<u8>> {
        let distance = level_distance_32(level);
        let next_degree = (flexible_key / distance) as u16;
        if level == 4 {
            read_seed_bytes(node_bytes, self.view_file_path(), (next_degree * 8) as u64)
        } else {
            // 下一结点状态
            // 下一结点node_bytes
            // 下一结点起始坐标seek
            // 在node集合中每一个node的默认字节长度是8，数量是256，即一次性读取2048个字节
            let (node_bytes, seek) = read_next_nodes_bytes(
                self,
                node_bytes,
                self.index_id(),
                node_seek,
                (next_degree * 8) as u64,
                root,
                false,
                level_type,
            )?;
            // 通过当前层真实key减去下一层的度数与间隔数的乘机获取结点所在下一层的真实key
            let next_flexible_key = flexible_key - next_degree as u32 * distance;
            self.get_32_in_node(
                node_bytes,
                level + 1,
                next_flexible_key,
                false,
                seek,
                level_type,
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
        next_node_seek: u64,
        level_type: LevelType,
    ) -> GeorgeResult<()>
    where
        Self: Sized,
    {
        // 通过当前树下一层高获取结点间间隔数量，即每一度中存在的元素数量
        let distance = level_distance_64(level);
        // 通过当前层真实key除以下一层间隔数获取结点处在下一层的度数
        let next_degree = flexible_key / distance;
        // 如果当前层高为4，则达到最底层，否则递归下一层逻辑
        if level == 4 {
            write_seed_bytes(
                node_bytes,
                self.index_file_path(),
                self.view_file_path(),
                next_node_seek,
                next_degree * 8, // 在当前操作结点的字节数组的起始位置
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
                self.index_id(),
                next_node_seek,
                next_degree * 8, // 在当前操作结点的字节数组的起始位置
                root,
                true,
                level_type,
            )?;
            // 通过当前层真实key减去下一层的度数与间隔数的乘机获取结点所在下一层的真实key
            let next_flexible_key = flexible_key - next_degree * distance;
            self.put_64_in_node(
                node_bytes,
                level + 1,
                next_flexible_key,
                seed,
                force,
                false,
                seek,
                level_type,
            )
        }
    }
    fn get_64_in_node(
        &self,
        node_bytes: Vec<u8>,
        level: u8,
        flexible_key: u64,
        root: bool,
        node_seek: u64,
        level_type: LevelType,
    ) -> GeorgeResult<Vec<u8>> {
        let distance = level_distance_64(level);
        let next_degree = flexible_key / distance;
        if level == 4 {
            read_seed_bytes(node_bytes, self.view_file_path(), next_degree * 8)
        } else {
            // 下一结点状态
            // 下一结点node_bytes
            // 下一结点起始坐标seek
            // 在node集合中每一个node的默认字节长度是8，数量是256，即一次性读取2048个字节
            let (node_bytes, seek) = read_next_nodes_bytes(
                self,
                node_bytes,
                self.index_id(),
                node_seek,
                next_degree * 8,
                root,
                false,
                level_type,
            )?;
            let next_flexible_key = flexible_key - next_degree as u64 * distance;
            self.get_64_in_node(
                node_bytes,
                level + 1,
                next_flexible_key,
                false,
                seek,
                level_type,
            )
        }
    }
    fn get_last_in_node(
        &self,
        node_bytes: Vec<u8>,
        level: u8,
        level_type: LevelType,
    ) -> GeorgeResult<Vec<u8>> {
        if level == 4 {
            let u8s = find_last_eq_bytes(node_bytes, 8)?;
            let seek = trans_bytes_2_u64(u8s);
            read_seed_bytes_from_view(self.view_file_path(), seek)
        } else {
            // 下一结点状态
            // 下一结点node_bytes
            // 下一结点起始坐标seek
            // 在node集合中每一个node的默认字节长度是8，数量是256，即一次性读取2048个字节
            let (node_bytes, _seek) =
                read_last_nodes_bytes(node_bytes, self.index_file_path(), level_type)?;
            self.get_last_in_node(node_bytes, level + 1, level_type)
        }
    }
}
