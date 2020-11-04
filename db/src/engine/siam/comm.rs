use std::sync::{Arc, RwLock};

use comm::bytes::create_empty_bytes;
use comm::errors::children::{DataExistError, DataNoExistError, NoneError};
use comm::errors::entrances::{GeorgeError, GeorgeResult};
use comm::io::reader::read_sub_bytes;
use comm::io::writer::write_seek_u8s;
use comm::trans::{trans_bytes_2_u64, trans_u64_2_bytes};

use crate::engine::siam::traits::{DiskNode, TNode};
use crate::engine::traits::TSeed;
use crate::utils::comm::{level_distance_32, level_distance_64};
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

/// 快速排序
fn quick_sort<N: TNode>(node: &N, nodes: &mut Vec<Arc<N>>, left: usize, right: usize) {
    if left >= right {
        return;
    }
    let mut l = left;
    let mut r = right;
    while l < r {
        while l < r && nodes[r].degree_index() >= nodes[left].degree_index() {
            r -= 1;
        }
        while l < r && nodes[l].degree_index() <= nodes[left].degree_index() {
            l += 1;
        }
        nodes.swap(l, r);
    }
    nodes.swap(left, l);
    if l > 1 {
        quick_sort(node, nodes, left, l - 1);
    }
    quick_sort(node, nodes, r + 1, right);
}

/// 新增子节点
///
/// 该操作会导致子节点数组内进行一次排序
///
/// 排序按照degree_index从小到大
pub(super) fn add_child_node<N: TNode>(node_own: &N, node: Arc<N>) {
    match node_own.nodes() {
        Some(ns) => {
            ns.write().unwrap().push(node);
            let ns_lock_w = ns.clone();
            let mut nodes = ns_lock_w.write().unwrap();
            let len = nodes.len() - 1;
            quick_sort(node_own, &mut nodes, 0, len)
        }
        _ => {}
    }
}

/// 指定节点中是否存在匹配md516_key的seed
///
/// 该方法用于get类型，在检索的同时会删除已发现的空seed
pub fn get_seed_value<N: TNode>(node: &N, md516_key: String) -> GeorgeResult<Vec<u8>> {
    let arc = node.clone().seeds().clone().unwrap().clone();
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

/// 指定节点中是否存在匹配md516_key的seed
///
/// 该方法用于put类型，即如果存在，则返回已存在值，没有额外操作
fn exist_seed<N: TNode>(node: &N, md516_key: String) -> bool {
    let arc = node.clone().seeds().clone().unwrap().clone();
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
fn exist_seed_save_force<N: TNode>(node: &N, seed_new: Arc<RwLock<dyn TSeed>>) {
    let arc = node.clone().seeds().clone().unwrap().clone();
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

pub fn create_or_take_node<N: TNode>(node: &N, index: u16) -> Arc<N> {
    create_or_take(node, index, false)
}

pub fn create_or_take_leaf<N: TNode>(node: &N, index: u16) -> Arc<N> {
    create_or_take(node, index, true)
}

/// 从指定父节点中尝试取出一个子节点，如果子节点不存在则创建一个并返回
///
/// node 指定父节点
///
/// leaf 是否叶子节点
fn create_or_take<N: TNode>(node: &N, index: u16, leaf: bool) -> Arc<N> {
    return match binary_match_data_pre(node, index) {
        Ok(next_node) => {
            // 节点存在，且返回节点在集合中的正确下标
            next_node.clone()
        }
        _ => {
            // 节点不存在，新建节点并返回
            let node_next: Arc<N>;
            if leaf {
                node_next = N::create_leaf(index);
            } else {
                node_next = N::create_node(index);
            }
            // 子节点集合不可能不存在，如果不存在，则抛出系统异常
            add_child_node(node, node_next.clone());
            node_next
        }
    };
}

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
pub(super) fn put_in_node_u64<N: TNode>(
    node: &N,
    level: u8,
    flexible_key: u64,
    seed: Arc<RwLock<dyn TSeed>>,
    force: bool,
) -> GeorgeResult<()> {
    let next_flexible_key: u64;
    let next_degree: u16;
    let distance: u64;
    distance = level_distance_64(level);
    next_degree = (flexible_key / distance) as u16;
    next_flexible_key = flexible_key - next_degree as u64 * distance;
    let node_next: Arc<N>;
    let node_next_level = level + 1;
    if level == 4 {
        // 创建或获取下一个叶子节点
        node_next = create_or_take_leaf(node, next_degree);
        put_seed(&*node_next, seed, force)
    } else {
        // 创建或获取下一个子节点
        node_next = create_or_take_node(node, next_degree);
        put_in_node_u64(&*node_next, node_next_level, next_flexible_key, seed, force)
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
pub(super) fn get_in_node_u64<N: TNode>(
    node: &N,
    level: u8,
    md516_key: String,
    flexible_key: u64,
) -> GeorgeResult<Vec<u8>> {
    let next_flexible_key: u64;
    let next_degree: u16;
    let distance: u64;
    if level.lt(&5) {
        distance = level_distance_64(level);
        next_degree = (flexible_key / distance) as u16;
        next_flexible_key = flexible_key - next_degree as u64 * distance;
    } else {
        // 获取seed叶子，如果存在，则判断版本号，如果不存在，则新建一个空并返回
        return get_seed_value(node, md516_key);
    };
    let node_next = binary_match_data_pre(node, next_degree)?;
    get_in_node_u64(node_next.as_ref(), level + 1, md516_key, next_flexible_key)
}

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
///
/// flexible_key 下一级最左最小树所对应真实key<p><p>
///
/// value 当前结果value信息<p><p>
///
/// ###Return
///
/// IndexResult<()>
pub(super) fn put_in_node_u32<N: TNode>(
    node: &N,
    level: u8,
    hash_key: u32,
    flexible_key: u32,
    seed: Arc<RwLock<dyn TSeed>>,
    force: bool,
) -> GeorgeResult<()> {
    let next_flexible_key: u32;
    let next_degree: u16;
    let distance: u32;
    distance = level_distance_32(level);
    next_degree = (flexible_key / distance) as u16;
    next_flexible_key = flexible_key - next_degree as u32 * distance;
    let node_next: Arc<N>;
    let node_next_level = level + 1;
    if level == 4 {
        // 创建或获取下一个叶子节点
        node_next = create_or_take_leaf(node, next_degree);
        put_seed(&*node_next, seed, force)
    } else {
        // 创建或获取下一个子节点
        node_next = create_or_take_node(node, next_degree);
        put_in_node_u32(
            &*node_next,
            node_next_level,
            hash_key,
            next_flexible_key,
            seed,
            force,
        )
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
pub(super) fn get_in_node_u32<N: TNode>(
    node: &N,
    level: u8,
    hash_key: u32,
    md516_key: String,
    flexible_key: u32,
) -> GeorgeResult<Vec<u8>> {
    let next_flexible_key: u32;
    let next_degree: u16;
    let distance: u32;
    if level.lt(&5) {
        distance = level_distance_32(level);
        next_degree = (flexible_key / distance) as u16;
        next_flexible_key = flexible_key - next_degree as u32 * distance;
    } else {
        // 获取seed叶子，如果存在，则判断版本号，如果不存在，则新建一个空并返回
        return get_seed_value(node, md516_key);
    };
    let node_next = binary_match_data_pre(node, next_degree)?;
    get_in_node_u32(
        node_next.as_ref(),
        level + 1,
        hash_key,
        md516_key,
        next_flexible_key,
    )
}

pub fn put_seed<N: TNode>(node: &N, seed: Arc<RwLock<dyn TSeed>>, force: bool) -> GeorgeResult<()> {
    // 获取seed叶子，如果存在，则判断版本号，如果不存在，则新建一个空并返回
    return if force {
        exist_seed_save_force(node, seed.clone());
        node.seeds().unwrap().write().unwrap().push(seed.clone());
        Ok(())
    } else {
        if exist_seed(node, seed.read().unwrap().key()) {
            Err(GeorgeError::DataExistError(DataExistError))
        } else {
            node.seeds().unwrap().write().unwrap().push(seed.clone());
            Ok(())
        }
    };
}

// Disk Node Exec After

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
pub fn read_next_nodes_bytes<N: DiskNode>(
    node: &N,
    node_bytes: Vec<u8>,
    index_file_name: String,
    index_file_path: String,
    next_node_seek: u64,
    start: u64,
    root: bool,
    new: bool,
) -> GeorgeResult<(Vec<u8>, u64)> {
    let seek_start = start as usize;
    let seek_end = seek_start + 8;
    let u8s = node_bytes.as_slice()[seek_start..seek_end].to_vec();
    let next_node_bytes_seek = trans_bytes_2_u64(u8s);
    if next_node_bytes_seek == 0 {
        if new {
            let next_node_bytes = create_empty_bytes(2048);
            let seek = GLOBAL_WRITER.write_append_bytes(
                Tag::Index,
                index_file_name.clone(),
                next_node_bytes.clone(),
            )?;
            let seek_v = trans_u64_2_bytes(seek);
            write_seek_u8s(
                index_file_path.clone(),
                start + next_node_seek,
                seek_v.clone().as_slice(),
            )?;
            if root {
                node.modify_node_bytes(seek_start, seek_v)
            }
            Ok((next_node_bytes, seek))
        } else {
            Err(GeorgeError::DataNoExistError(DataNoExistError))
        }
    } else {
        // 在node集合中每一个node的默认字节长度是8，数量是256，即一次性读取2048个字节
        let next_node_bytes = read_sub_bytes(index_file_path, next_node_bytes_seek, 2048)?;
        Ok((next_node_bytes, next_node_bytes_seek))
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
/// new 是否插入操作
///
/// force 如果存在原值，是否覆盖原结果
pub fn write_seed_bytes<N: DiskNode>(
    node: &N,
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
        write_seed(node, index_file_path, next_node_seek, start, seed)
    } else {
        if force {
            // 先读取seed的长度
            let seed_len_bytes = read_sub_bytes(view_file_path.clone(), seed_seek, 8)?;
            let seed_len = trans_bytes_2_u64(seed_len_bytes);
            let seed_bytes = read_sub_bytes(view_file_path, seed_seek + 8, seed_len as usize)?;
            if seed_bytes
                .as_slice()
                .eq(seed.clone().read().unwrap().value().unwrap().as_slice())
            {
                Ok(())
            } else {
                write_seed(node, index_file_path, next_node_seek, start, seed)
            }
        } else {
            Err(GeorgeError::DataExistError(DataExistError))
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
pub fn read_seed_bytes(
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
        // 先读取seed的长度
        let seed_len_bytes = read_sub_bytes(view_file_path.clone(), seed_seek, 8)?;
        let seed_len = trans_bytes_2_u64(seed_len_bytes);
        let seed_bytes = read_sub_bytes(view_file_path, seed_seek + 8, seed_len as usize)?;
        Ok(seed_bytes)
    }
}

pub fn write_seed<N: DiskNode>(
    node: &N,
    index_file_path: String,
    next_node_seek: u64,
    start: u64,
    seed: Arc<RwLock<dyn TSeed>>,
) -> GeorgeResult<()> {
    let mut seed_bytes = seed.clone().read().unwrap().value().unwrap();
    let mut seed_bytes_len_bytes = trans_u64_2_bytes(seed_bytes.len() as u64);
    seed_bytes_len_bytes.append(&mut seed_bytes);
    let seek = GLOBAL_WRITER.write_append_bytes(
        Tag::View,
        node.view_id(),
        seed_bytes_len_bytes.clone(),
    )?;
    let seek_v = trans_u64_2_bytes(seek);
    write_seek_u8s(
        index_file_path.clone(),
        start + next_node_seek,
        seek_v.clone().as_slice(),
    )?;
    Ok(())
}
