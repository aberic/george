use std::fs::File;
use std::ops::{Add, Sub};
use std::sync::{Arc, RwLock};

use comm::bytes::create_empty_bytes;
use comm::errors::children::{DataExistError, DataNoExistError, NoneError};
use comm::errors::entrances::{err_str, err_string, GeorgeError, GeorgeResult};
use comm::io::reader::{read_sub_bytes, read_sub_bytes_by_file, read_sub_file_bytes};
use comm::io::writer::write_seek_u8s;
use comm::trans::{trans_bytes_2_u64, trans_u64_2_bytes};
use comm::vectors::{find_eq_vec_bytes, find_last_eq_bytes};

use crate::engine::siam::doc32::seed::Seed;
use crate::engine::siam::traits::{DiskNode, TNode};
use crate::engine::traits::TSeed;
use crate::utils::store::Tag;
use crate::utils::writer::GLOBAL_WRITER;

/// 节点数组二分查找基本方法前置方法
///
/// node 所查找的集合根
///
/// match_index 在该集合中真实的下标位置
pub(super) fn binary_match_data_pre<N: TNode>(node: &N, match_index: u16) -> GeorgeResult<Arc<N>> {
    return match node.clone().nodes() {
        Some(arc_nodes) => {
            if arc_nodes.clone().read().unwrap().clone().len() > 0 {
                binary_match_data(arc_nodes.clone(), match_index)
            } else {
                Err(GeorgeError::DataNoExistError(DataNoExistError))
            }
        }
        None => Err(GeorgeError::NoneError(NoneError)),
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
fn binary_match_data<N: TNode>(
    nodes: Arc<RwLock<Vec<Arc<N>>>>,
    match_index: u16,
) -> GeorgeResult<Arc<N>> {
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
                return Err(GeorgeError::DataNoExistError(DataNoExistError));
            }
        } else if nodes[middle_index].degree_index() < match_index {
            // 在arr数组的右边找
            left_index = middle_index + 1
        } else if nodes[middle_index].degree_index() == match_index {
            return Ok(nodes.get(middle_index).unwrap().clone());
        }
    }
    Err(GeorgeError::DataNoExistError(DataNoExistError))
}

// Disk Node Exec After

/// 下一节点信息
#[derive(Debug, Clone)]
pub struct NodeBytes {
    /// 下一节点node_bytes
    pub bytes: Vec<u8>,
    /// 下一节点起始坐标seek
    pub seek: u64,
}

impl NodeBytes {
    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }
}

#[derive(Debug, Clone)]
pub struct QueryNodeData {
    nb: Option<NodeBytes>,
    nbs: Vec<NodeBytes>,
}

impl QueryNodeData {
    pub fn node_bytes(&self) -> Option<NodeBytes> {
        self.nb.clone()
    }
    pub fn node_bytes_list(&self) -> Vec<NodeBytes> {
        self.nbs.clone()
    }
}

/// 读取下一个节点的字节数组记录，如果不存在，则判断是否为插入操作，如果是插入操作，则新建下一个节点默认数组
///
/// node_bytes 当前操作节点的字节数组
///
/// next_node_seek 下一节点在文件中的真实起始位置
///
/// start 下一节点在node_bytes中的起始位置
///
/// root 是否根节点
///
/// new 是否插入操作
///
/// #return 下一节点状态
///
/// 下一节点node_bytes
///
/// 下一节点起始坐标seek
pub(super) fn read_next_nodes_bytes<N: DiskNode>(
    node: &N,
    node_bytes: Vec<u8>,
    index_id: String,
    next_node_seek: u64,
    start: u64,
    root: bool,
) -> GeorgeResult<NodeBytes> {
    let seek_start = start as usize;
    let seek_end = seek_start + 8;
    let u8s = node_bytes.as_slice()[seek_start..seek_end].to_vec();
    let next_node_bytes_seek = trans_bytes_2_u64(u8s);
    if next_node_bytes_seek == 0 {
        // 如果子项是32位node集合，在node集合中每一个node的默认字节长度是8，数量是256，即一次性读取2048个字节
        // 如果子项是64位node集合，在node集合中每一个node的默认字节长度是8，数量是65536，即一次性读取524288个字节
        let next_node_bytes = create_empty_bytes(2048);
        let seek = GLOBAL_WRITER.write_append_bytes(
            Tag::Index,
            index_id.clone(),
            next_node_bytes.clone(),
        )?;
        let seek_v = trans_u64_2_bytes(seek);
        write_seek_u8s(
            node.index_file_path(),
            start + next_node_seek,
            seek_v.clone().as_slice(),
        )?;
        if root {
            node.modify_node_bytes(seek_start, seek_v)
        }
        Ok(NodeBytes {
            bytes: next_node_bytes,
            seek,
        })
    } else {
        read_node_bytes(node.index_file_path(), next_node_bytes_seek)
    }
}

/// 读取下一个节点的字节数组记录，如果不存在，则判断是否为插入操作，如果是插入操作，则新建下一个节点默认数组
///
/// node_bytes 当前操作节点的字节数组
///
/// next_node_seek 下一节点在文件中的真实起始位置
///
/// start 下一节点在node_bytes中的起始位置
///
/// root 是否根节点
///
/// new 是否插入操作
///
/// #return 下一节点状态
///
/// 下一节点node_bytes
///
/// 下一节点起始坐标seek
pub(super) fn try_read_next_nodes_bytes<N: DiskNode>(
    node: &N,
    node_bytes: Vec<u8>,
    start: u64,
) -> GeorgeResult<NodeBytes> {
    let seek_start = start as usize;
    let seek_end = seek_start + 8;
    let u8s = node_bytes.as_slice()[seek_start..seek_end].to_vec();
    let next_node_bytes_seek = trans_bytes_2_u64(u8s);
    if next_node_bytes_seek == 0 {
        Err(GeorgeError::DataNoExistError(DataNoExistError))
    } else {
        read_node_bytes(node.index_file_path(), next_node_bytes_seek)
    }
}

/// 读取最右叶子节点的最右字节数组记录
///
/// node_bytes 当前操作节点的字节数组
///
/// next_node_seek 下一节点在文件中的真实起始位置
///
/// start 下一节点在node_bytes中的起始位置
///
/// root 是否根节点
///
/// new 是否插入操作
///
/// #return 下一节点状态
///
/// 下一节点node_bytes
///
/// 下一节点起始坐标seek
pub(super) fn read_last_nodes_bytes(
    node_bytes: Vec<u8>,
    index_file_path: String,
) -> GeorgeResult<NodeBytes> {
    let u8s = find_last_eq_bytes(node_bytes, 8)?;
    let next_node_bytes_seek = trans_bytes_2_u64(u8s);
    read_node_bytes(index_file_path, next_node_bytes_seek)
}

/// 读取最右叶子节点的字节数组集合记录
///
/// node_bytes 当前操作节点的字节数组
///
/// next_node_seek 下一节点在文件中的真实起始位置
///
/// start 下一节点在node_bytes中的起始位置
///
/// root 是否根节点
///
/// new 是否插入操作
///
/// #return 下一节点状态
///
/// 下一节点node_bytes
///
/// 下一节点起始坐标seek
pub(super) fn read_next_and_all_nodes_bytes_by_file(
    node_bytes: Vec<u8>,
    index_file: Arc<RwLock<File>>,
    start: usize,
    end: usize,
) -> GeorgeResult<Vec<NodeBytes>> {
    let mut nbs: Vec<NodeBytes> = vec![];
    let last_bytes: Vec<u8>;
    let u82s: Vec<Vec<u8>>;

    let seek_start = start * 8;
    if end == 0 {
        last_bytes = node_bytes.as_slice()[seek_start..].to_vec();
    } else {
        let seek_end = end * 8;
        last_bytes = node_bytes.as_slice()[seek_start..seek_end].to_vec();
    }
    u82s = find_eq_vec_bytes(last_bytes, 8)?;
    for u8s in u82s {
        let next_node_bytes_seek = trans_bytes_2_u64(u8s);
        nbs.push(read_node_bytes_by_file(
            index_file.clone(),
            next_node_bytes_seek,
        )?);
    }
    Ok(nbs)
}

/// 读取下一个节点的字节数组记录及其后续字节数组
///
/// node_bytes 当前操作节点的字节数组
///
/// next_node_seek 下一节点在文件中的真实起始位置
///
/// start 下一节点在node_bytes中的起始位置
///
/// root 是否根节点
///
/// new 是否插入操作
///
/// #return 下一节点状态
///
/// 下一节点node_bytes
///
/// 下一节点起始坐标seek
pub(super) fn read_next_nodes_and_all_bytes_by_file(
    node_bytes: Vec<u8>,
    index_file: Arc<RwLock<File>>,
    start: u64,
    end: u64,
) -> GeorgeResult<QueryNodeData> {
    let qnd: QueryNodeData;

    let last_bytes: Vec<u8>;

    let seek_start = start as usize;
    let seek_last_start = seek_start + 8;

    if end > 0 {
        let seek_end = end as usize + 8;
        last_bytes = node_bytes.as_slice()[seek_last_start..seek_end].to_vec();
    } else {
        last_bytes = node_bytes.as_slice()[seek_last_start..].to_vec();
    }

    let mut nbs: Vec<NodeBytes> = vec![];
    let u82s = find_eq_vec_bytes(last_bytes, 8)?;
    for u8s in u82s {
        let next_node_bytes_seek = trans_bytes_2_u64(u8s);
        nbs.push(read_node_bytes_by_file(
            index_file.clone(),
            next_node_bytes_seek,
        )?);
    }

    let u8s = node_bytes.as_slice()[seek_start..seek_last_start].to_vec();
    let next_node_bytes_seek = trans_bytes_2_u64(u8s);
    if next_node_bytes_seek == 0 {
        qnd = QueryNodeData { nb: None, nbs }
    } else {
        let nb = read_node_bytes_by_file(index_file.clone(), next_node_bytes_seek)?;
        qnd = QueryNodeData { nb: Some(nb), nbs }
    }

    Ok(qnd)
}

/// 读取下一个节点的字节数组记录及其之前字节数组
///
/// node_bytes 当前操作节点的字节数组
///
/// next_node_seek 下一节点在文件中的真实起始位置
///
/// start 下一节点在node_bytes中的起始位置
///
/// root 是否根节点
///
/// new 是否插入操作
///
/// #return 下一节点状态
///
/// 下一节点node_bytes
///
/// 下一节点起始坐标seek
pub(super) fn read_before_and_all_nodes_bytes_by_file(
    node_bytes: Vec<u8>,
    index_file: Arc<RwLock<File>>,
    start: usize,
    end: usize,
) -> GeorgeResult<Vec<NodeBytes>> {
    let mut nbs: Vec<NodeBytes> = vec![];
    let before_bytes: Vec<u8>;
    let u82s: Vec<Vec<u8>>;

    let seek_start = start * 8;

    if end > 0 {
        let seek_end = end * 8;
        let seek_before_end = seek_end + 8;
        before_bytes = node_bytes.as_slice()[seek_start..seek_before_end].to_vec();
    } else {
        before_bytes = node_bytes.as_slice()[seek_start..].to_vec();
    }

    u82s = find_eq_vec_bytes(before_bytes, 8)?;
    for u8s in u82s {
        let next_node_bytes_seek = trans_bytes_2_u64(u8s);
        nbs.push(read_node_bytes_by_file(
            index_file.clone(),
            next_node_bytes_seek,
        )?);
    }
    Ok(nbs)
}

/// 读取下一个节点的字节数组记录及其之前字节数组
///
/// node_bytes 当前操作节点的字节数组
///
/// next_node_seek 下一节点在文件中的真实起始位置
///
/// start 下一节点在node_bytes中的起始位置
///
/// root 是否根节点
///
/// new 是否插入操作
///
/// #return 下一节点状态
///
/// 下一节点node_bytes
///
/// 下一节点起始坐标seek
pub(super) fn read_before_nodes_and_all_bytes_by_file(
    node_bytes: Vec<u8>,
    index_file: Arc<RwLock<File>>,
    start: u64,
    end: u64,
) -> GeorgeResult<QueryNodeData> {
    let qnd: QueryNodeData;

    let seek_start = start as usize;
    let seek_end = end as usize;
    let seek_before_end = seek_end + 8;

    let before_bytes = node_bytes.as_slice()[seek_start..seek_end].to_vec();
    let mut nbs: Vec<NodeBytes> = vec![];
    let u82s = find_eq_vec_bytes(before_bytes, 8)?;
    let mut len = u82s.len();
    while len > 0 {
        match u82s.get(len - 1) {
            Some(u8s) => {
                let next_node_bytes_seek = trans_bytes_2_u64(u8s.clone());
                nbs.push(read_node_bytes_by_file(
                    index_file.clone(),
                    next_node_bytes_seek,
                )?);
                len -= 1;
            }
            None => {
                return Err(err_str(
                    "read before nodes and all bytes by file get none error",
                ));
            }
        }
    }

    let u8s = node_bytes.as_slice()[seek_end..seek_before_end].to_vec();
    let next_node_bytes_seek = trans_bytes_2_u64(u8s);
    if next_node_bytes_seek != 0 {
        let nb = read_node_bytes_by_file(index_file.clone(), next_node_bytes_seek)?;
        qnd = QueryNodeData { nb: Some(nb), nbs }
    } else {
        qnd = QueryNodeData { nb: None, nbs }
    }

    Ok(qnd)
}

/// 读取结点字节数组及该数组的起始偏移量
///
/// 如果子项是32位node集合，在node集合中每一个node的默认字节长度是8，数量是256，即一次性读取2048个字节
///
/// 如果子项是64位node集合，在node集合中每一个node的默认字节长度是8，数量是65536，即一次性读取524288个字节
fn read_node_bytes(index_file_path: String, next_node_bytes_seek: u64) -> GeorgeResult<NodeBytes> {
    Ok(NodeBytes {
        bytes: read_sub_bytes(index_file_path, next_node_bytes_seek, 2048)?,
        seek: next_node_bytes_seek,
    })
}

/// 读取结点字节数组及该数组的起始偏移量
///
/// 如果子项是32位node集合，在node集合中每一个node的默认字节长度是8，数量是256，即一次性读取2048个字节
///
/// 如果子项是64位node集合，在node集合中每一个node的默认字节长度是8，数量是65536，即一次性读取524288个字节
fn read_node_bytes_by_file(
    index_file: Arc<RwLock<File>>,
    next_node_bytes_seek: u64,
) -> GeorgeResult<NodeBytes> {
    let next_node_bytes = read_sub_file_bytes(
        index_file.clone().read().unwrap().try_clone().unwrap(),
        next_node_bytes_seek,
        2048,
    )?;
    Ok(NodeBytes {
        bytes: next_node_bytes,
        seek: next_node_bytes_seek,
    })
}

/// 读取下一个节点的字节数组记录，如果不存在，则判断是否为插入操作，如果是插入操作，则新建下一个节点默认数组
///
/// node_bytes 当前操作节点的字节数组
///
/// next_node_seek 下一节点在文件中的真实起始位置
///
/// start 下一节点在node_bytes中的起始位置
///
/// new 是否插入操作
///
/// force 如果存在原值，是否覆盖原结果
pub(super) fn write_seed_bytes(
    node_bytes: Vec<u8>,
    index_file_path: String,
    view_file_path: String,
    next_node_seek: u64,
    start: u64,
    force: bool,
    seed: Arc<RwLock<dyn TSeed>>,
) -> GeorgeResult<()> {
    let seek_start = start as usize;
    let seek_end = seek_start + 8;
    let u8s = node_bytes.as_slice()[seek_start..seek_end].to_vec();
    let seed_seek = trans_bytes_2_u64(u8s);
    if seed_seek == 0 {
        let seed_u8s = Seed::u8s(index_file_path, next_node_seek, start)?;
        seed.write().unwrap().modify(seed_u8s);
        Ok(())
    } else {
        // 先读取seed的长度
        let seed_len_bytes = read_sub_bytes(view_file_path.clone(), seed_seek, 8)?;
        let seed_len = trans_bytes_2_u64(seed_len_bytes);
        if seed_len == 0 {
            let seed_u8s = Seed::u8s(index_file_path, next_node_seek, start)?;
            seed.write().unwrap().modify(seed_u8s);
            Ok(())
        } else {
            if force {
                let seed_bytes = read_sub_bytes(view_file_path, seed_seek + 8, seed_len as usize)?;
                let seed_value_bytes = seed.clone().read().unwrap().value().unwrap();
                if seed_bytes.as_slice().eq(seed_value_bytes.as_slice()) {
                    Ok(())
                } else {
                    let seed_u8s = Seed::u8s(index_file_path, next_node_seek, start)?;
                    seed.write().unwrap().modify(seed_u8s);
                    Ok(())
                }
            } else {
                Err(GeorgeError::DataExistError(DataExistError))
            }
        }
    }
}

/// 读取下一个节点的字节数组记录，如果不存在，则判断是否为插入操作，如果是插入操作，则新建下一个节点默认数组
///
/// node_bytes 当前操作节点的字节数组
///
/// start 下一节点在node_bytes中的起始位置
///
/// force 如果存在原值，是否覆盖原结果
pub(super) fn read_seed_bytes(
    node_bytes: Vec<u8>,
    view_file_path: String,
    start: u64,
) -> GeorgeResult<Vec<u8>> {
    let seek_start = start as usize;
    let seek_end = seek_start + 8;
    let u8s = node_bytes.as_slice()[seek_start..seek_end].to_vec();
    let seed_seek = trans_bytes_2_u64(u8s);
    if seed_seek == 0 {
        Err(GeorgeError::DataNoExistError(DataNoExistError))
    } else {
        read_seed_bytes_from_view(view_file_path, seed_seek)
    }
}

pub(super) fn read_seed_bytes_from_view(
    view_file_path: String,
    seed_seek: u64,
) -> GeorgeResult<Vec<u8>> {
    match File::open(view_file_path) {
        Ok(file) => {
            let file_rw = Arc::new(RwLock::new(file));
            // 先读取seed的长度
            let seed_len_bytes = read_sub_bytes_by_file(file_rw.clone(), seed_seek, 8)?;
            let seed_len = trans_bytes_2_u64(seed_len_bytes);
            let seed_bytes =
                read_sub_bytes_by_file(file_rw.clone(), seed_seek + 8, seed_len as usize)?;
            Ok(seed_bytes)
        }
        Err(err) => Err(err_string(err.to_string())),
    }
}

pub(super) fn read_seed_bytes_from_view_file(
    view_file: Arc<RwLock<File>>,
    seed_seek: u64,
) -> GeorgeResult<Vec<u8>> {
    // 先读取seed的长度
    let seed_len_bytes = read_sub_bytes_by_file(view_file.clone(), seed_seek, 8)?;
    let seed_len = trans_bytes_2_u64(seed_len_bytes);
    let seed_bytes = read_sub_bytes_by_file(view_file.clone(), seed_seek + 8, seed_len as usize)?;
    Ok(seed_bytes)
}

pub fn i64_2_u64(res: i64) -> u64 {
    if res >= 0 {
        (res as u64).add(9223372036854775809)
    } else {
        (res as u64).sub(9223372036854775807)
    }
}

pub fn i32_2_u64(res: i32) -> u64 {
    if res >= 0 {
        (res as u64).add(2147483649)
    } else {
        (res as u64).sub(2147483647)
    }
}
