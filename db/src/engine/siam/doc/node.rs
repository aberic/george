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
use std::fs::File;
use std::ops::AddAssign;
use std::sync::{Arc, RwLock};

use comm::bytes::create_empty_bytes;
use comm::cryptos::hash::hashcode32_enhance;
use comm::errors::entrances::{err_str, err_string_enhance, GeorgeResult};
use comm::trans::trans_bytes_2_u64;
use comm::vectors;
use comm::vectors::find_last_eq_bytes;

use crate::engine::siam::comm::{
    read_before_and_all_nodes_bytes_by_file, read_before_nodes_and_all_bytes_by_file,
    read_last_nodes_bytes, read_next_and_all_nodes_bytes_by_file,
    read_next_nodes_and_all_bytes_by_file, read_next_nodes_bytes, read_seed_bytes,
    read_seed_bytes_from_view, read_seed_bytes_from_view_file, try_read_next_nodes_bytes,
    write_seed_bytes,
};
use crate::engine::siam::selector::{Condition, Constraint};
use crate::engine::siam::traits::{DiskNode, TNode};
use crate::engine::traits::TSeed;
use crate::utils::comm::{level_distance_32, IndexMold};
use crate::utils::path::{index_file_path, view_file_path};
use crate::utils::store::store_index_id;

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
fn create_root_self(database_id: String, view_id: String, index_id: String) -> Node {
    Node {
        database_id: database_id.clone(),
        view_id: view_id.clone(),
        index_id: index_id.clone(),
        view_file_path: view_file_path(database_id.clone(), view_id.clone()),
        index_file_path: index_file_path(database_id.clone(), view_id.clone(), index_id.clone()),
        node_bytes: Arc::new(RwLock::new(create_empty_bytes(2048))),
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
    pub fn create_root(database_id: String, view_id: String, index_id: String) -> Arc<Self> {
        return Arc::new(create_root_self(database_id, view_id, index_id));
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
    fn put(
        &self,
        key: String,
        seed: Arc<RwLock<dyn TSeed>>,
        force: bool,
        description_len: usize,
    ) -> GeorgeResult<()>
    where
        Self: Sized,
    {
        let node_bytes = self.node_bytes().read().unwrap().to_vec();
        self.put_in_node(
            node_bytes,
            1,
            hashcode32_enhance(key),
            seed,
            force,
            true,
            description_len as u64,
        )
    }
    fn get(&self, key: String) -> GeorgeResult<Vec<u8>> {
        let node_bytes = self.node_bytes().read().unwrap().to_vec();
        self.get_in_node(node_bytes, 1, hashcode32_enhance(key))
    }
    fn get_last(&self) -> GeorgeResult<Vec<u8>>
    where
        Self: Sized,
    {
        let node_bytes = self.node_bytes().read().unwrap().to_vec();
        self.get_last_in_node(node_bytes, 1)
    }
    fn select(
        &self,
        mold: IndexMold,
        left: bool,
        start: u64,
        end: u64,
        constraint: Constraint,
    ) -> GeorgeResult<(u64, u64, Vec<Vec<u8>>)> {
        let node_bytes = self.node_bytes().read().unwrap().to_vec();
        match File::open(self.index_file_path()) {
            Ok(index_file_real) => match File::open(self.view_file_path()) {
                Ok(view_file_real) => {
                    let index_file = Arc::new(RwLock::new(index_file_real));
                    let view_file = Arc::new(RwLock::new(view_file_real));
                    let level = 1;
                    let conditions = constraint.conditions();
                    log::debug!("conditions length = {}", conditions.len());
                    let skip = constraint.skip();
                    let limit = constraint.limit();
                    let delete = constraint.delete();
                    let query: (u64, u64, u64, u64, Vec<Vec<u8>>);
                    if left {
                        query = self.left_query(
                            mold, index_file, view_file, node_bytes, start, end, level, conditions,
                            skip, limit, delete,
                        )?
                    } else {
                        query = self.right_query(
                            mold, index_file, view_file, node_bytes, start, end, level, conditions,
                            skip, limit, delete,
                        )?
                    }
                    Ok((query.0, query.1, query.4))
                }
                Err(err) => Err(err_string_enhance(
                    format!(
                        "select view file whit path {} error, ",
                        self.view_file_path()
                    ),
                    err.to_string(),
                )),
            },
            Err(err) => Err(err_string_enhance(
                format!(
                    "select index file whit path {} error, ",
                    self.index_file_path()
                ),
                err.to_string(),
            )),
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
    fn put_in_node(
        &self,
        node_bytes: Vec<u8>,
        level: u8,
        flexible_key: u32,
        seed: Arc<RwLock<dyn TSeed>>,
        force: bool,
        root: bool,
        next_node_seek: u64,
    ) -> GeorgeResult<()>
    where
        Self: Sized,
    {
        // 通过当前树下一层高获取结点间间隔数量，即每一度中存在的元素数量
        let distance = level_distance_32(level);
        // 通过当前层真实key除以下一层间隔数获取结点处在下一层的度数
        let next_degree = flexible_key / distance;
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
            let nbs = read_next_nodes_bytes(
                self,
                node_bytes,
                store_index_id(self.database_id(), self.view_id(), self.index_id()),
                next_node_seek,
                (next_degree * 8) as u64, // 在当前操作结点的字节数组的起始位置
                root,
            )?;
            // 通过当前层真实key减去下一层的度数与间隔数的乘机获取结点所在下一层的真实key
            let next_flexible_key = flexible_key - next_degree * distance;
            self.put_in_node(
                nbs.bytes(),
                level + 1,
                next_flexible_key,
                seed,
                force,
                false,
                nbs.seek,
            )
        }
    }
    fn get_in_node(
        &self,
        node_bytes: Vec<u8>,
        level: u8,
        flexible_key: u32,
    ) -> GeorgeResult<Vec<u8>> {
        let distance = level_distance_32(level);
        let next_degree = flexible_key / distance;
        if level == 4 {
            read_seed_bytes(node_bytes, self.view_file_path(), next_degree as usize * 8)
        } else {
            // 下一结点状态
            // 下一结点node_bytes
            // 下一结点起始坐标seek
            // 在node集合中每一个node的默认字节长度是8，数量是256，即一次性读取2048个字节
            let nbs = try_read_next_nodes_bytes(self, node_bytes, next_degree as usize * 8)?;
            let next_flexible_key = flexible_key - next_degree * distance;
            self.get_in_node(nbs.bytes, level + 1, next_flexible_key)
        }
    }
    fn get_last_in_node(&self, node_bytes: Vec<u8>, level: u8) -> GeorgeResult<Vec<u8>> {
        if level == 4 {
            let u8s = find_last_eq_bytes(node_bytes, 8)?;
            let seek = trans_bytes_2_u64(u8s);
            read_seed_bytes_from_view(self.view_file_path(), seek)
        } else {
            // 下一结点状态
            // 下一结点node_bytes
            // 下一结点起始坐标seek
            // 在node集合中每一个node的默认字节长度是8，数量是256，即一次性读取2048个字节
            let nbs = read_last_nodes_bytes(node_bytes, self.index_file_path())?;
            self.get_last_in_node(nbs.bytes, level + 1)
        }
    }
    fn left_query(
        &self,
        mold: IndexMold,
        index_file: Arc<RwLock<File>>,
        view_file: Arc<RwLock<File>>,
        node_bytes: Vec<u8>,
        start_key: u64,
        end_key: u64,
        level: u8,
        conditions: Vec<Condition>,
        mut skip: u64,
        mut limit: u64,
        delete: bool,
    ) -> GeorgeResult<(u64, u64, u64, u64, Vec<Vec<u8>>)> {
        // todo delete
        let mut total: u64 = 0;
        let mut count: u64 = 0;
        let mut res: Vec<Vec<u8>> = vec![];
        if level == 4 {
            let nbs_arr = read_next_and_all_nodes_bytes_by_file(
                node_bytes,
                index_file.clone(),
                start_key as usize,
                end_key as usize,
            )?;
            total += nbs_arr.len() as u64;
            for nbs in nbs_arr {
                if limit <= 0 {
                    break;
                }
                let bytes = read_seed_bytes_from_view_file(view_file.clone(), nbs.seek)?;
                total += 1;
                if Condition::validate(mold, conditions.clone(), bytes.clone()) {
                    if skip <= 0 {
                        limit -= 1;
                        count += 1;
                        res.push(bytes)
                    } else {
                        skip -= 1;
                    }
                }
            }
        // if delete {
        //     for nbs in nbs_arr {
        //         if limit <= 0 {
        //             break;
        //         }
        //         let bytes = read_seed_bytes_from_view_file(view_file.clone(), nbs.seek)?;
        //         total += 1;
        //         if Condition::validate(mold, conditions.clone(), bytes.clone()) {
        //             if skip <= 0 {
        //                 limit -= 1;
        //                 count += 1;
        //                 res.push(bytes)
        //             } else {
        //                 skip -= 1;
        //             }
        //         }
        //     }
        // } else {
        //     for nbs in nbs_arr {
        //         if limit <= 0 {
        //             break;
        //         }
        //         let bytes = read_seed_bytes_from_view_file(view_file.clone(), nbs.seek)?;
        //         total += 1;
        //         if Condition::validate(mold, conditions.clone(), bytes.clone()) {
        //             if skip <= 0 {
        //                 limit -= 1;
        //                 count += 1;
        //                 res.push(bytes)
        //             } else {
        //                 skip -= 1;
        //             }
        //         }
        //     }
        // }
        } else {
            let distance = level_distance_32(level) as u64;
            let next_start_degree = start_key / distance;
            let next_end_degree = end_key / distance;
            // 下一结点状态
            // 下一结点node_bytes
            // 下一结点起始坐标seek
            // 在node集合中每一个node的默认字节长度是8，数量是256，即一次性读取2048个字节
            let qnd = read_next_nodes_and_all_bytes_by_file(
                node_bytes,
                index_file.clone(),
                next_start_degree as u64 * 8,
                next_end_degree as u64 * 8,
            )?;
            total.add_assign(1 + qnd.node_bytes_list().len() as u64);
            let next_start_key = start_key - next_start_degree * distance;
            let next_end_key = end_key - next_end_degree * distance;
            match qnd.node_bytes() {
                Some(nbs) => {
                    let mut nek = next_end_key;
                    if next_start_degree != next_end_degree {
                        nek = 0;
                    }
                    let mut temp = self.left_query(
                        mold,
                        index_file.clone(),
                        view_file.clone(),
                        nbs.bytes(),
                        next_start_key,
                        nek,
                        level + 1,
                        conditions.clone(),
                        skip,
                        limit,
                        delete,
                    )?;
                    total += temp.0;
                    count += temp.1;
                    skip = temp.2;
                    limit = temp.3;
                    res.append(&mut temp.4);
                }
                _ => {}
            }
            if limit > 0 {
                let mut node_bytes_list_len_check = 0;
                let node_bytes_list_len = qnd.node_bytes_list().len();
                if node_bytes_list_len > 0 {
                    node_bytes_list_len_check = node_bytes_list_len - 1;
                }
                for (pos, nbs) in qnd.node_bytes_list().iter().enumerate() {
                    let mut nek = 0;
                    if node_bytes_list_len_check == pos {
                        nek = next_end_key;
                    }
                    let mut temp = self.left_query(
                        mold,
                        index_file.clone(),
                        view_file.clone(),
                        nbs.bytes(),
                        0,
                        nek,
                        level + 1,
                        conditions.clone(),
                        skip,
                        limit,
                        delete,
                    )?;
                    res.append(&mut temp.4);
                    total += temp.0;
                    count += temp.1;
                    skip = temp.2;
                    limit = temp.3;
                    if limit <= 0 {
                        break;
                    }
                }
            }
        }
        Ok((total, count, skip, limit, res))
    }
    fn right_query(
        &self,
        mold: IndexMold,
        index_file: Arc<RwLock<File>>,
        view_file: Arc<RwLock<File>>,
        node_bytes: Vec<u8>,
        start_key: u64,
        end_key: u64,
        level: u8,
        conditions: Vec<Condition>,
        mut skip: u64,
        mut limit: u64,
        delete: bool,
    ) -> GeorgeResult<(u64, u64, u64, u64, Vec<Vec<u8>>)> {
        // todo delete
        let mut total: u64 = 0;
        let mut count: u64 = 0;
        let mut res: Vec<Vec<u8>> = vec![];
        if level == 4 {
            let nbs_arr = read_before_and_all_nodes_bytes_by_file(
                node_bytes,
                index_file.clone(),
                start_key as usize,
                end_key as usize,
            )?;
            total += nbs_arr.len() as u64;
            let mut len = nbs_arr.len();
            while len > 0 {
                if limit <= 0 {
                    break;
                }
                match nbs_arr.get(len - 1) {
                    Some(nbs) => {
                        let bytes = read_seed_bytes_from_view_file(view_file.clone(), nbs.seek)?;
                        total += 1;
                        if Condition::validate(mold, conditions.clone(), bytes.clone()) {
                            if skip > 0 {
                                skip -= 1;
                            } else {
                                limit -= 1;
                                count += 1;
                                res.push(bytes)
                            }
                        }
                        len -= 1;
                    }
                    None => return Err(err_str("select bytes get none error")),
                }
            }
        } else {
            let distance = level_distance_32(level) as u64;
            let next_start_degree = start_key / distance;
            let next_end_degree = end_key / distance;
            // 下一结点状态
            // 下一结点node_bytes
            // 下一结点起始坐标seek
            // 在node集合中每一个node的默认字节长度是8，数量是256，即一次性读取2048个字节
            let qnd = read_before_nodes_and_all_bytes_by_file(
                node_bytes,
                index_file.clone(),
                next_start_degree * 8,
                next_end_degree * 8,
            )?;
            total.add_assign(1 + qnd.node_bytes_list().len() as u64);
            let next_start_key = start_key - next_start_degree * distance;
            let next_end_key = end_key - next_end_degree * distance;
            match qnd.node_bytes() {
                Some(nbs) => {
                    let mut nsk = next_start_key;
                    if next_start_degree != next_end_degree {
                        nsk = 0;
                    }
                    let mut temp = self.right_query(
                        mold,
                        index_file.clone(),
                        view_file.clone(),
                        nbs.bytes(),
                        nsk,
                        next_end_key,
                        level + 1,
                        conditions.clone(),
                        skip,
                        limit,
                        delete,
                    )?;
                    total += temp.0;
                    count += temp.1;
                    skip = temp.2;
                    limit = temp.3;
                    res.append(&mut temp.4);
                }
                _ => {}
            }
            if limit > 0 {
                let mut node_bytes_list_len_check = 0;
                let node_bytes_list_len = qnd.node_bytes_list().len();
                if node_bytes_list_len > 0 {
                    node_bytes_list_len_check = node_bytes_list_len - 1;
                }
                for (pos, nbs) in qnd.node_bytes_list().iter().enumerate() {
                    let mut nsk = 0;
                    if node_bytes_list_len_check == pos {
                        nsk = next_start_key;
                    }
                    let mut temp = self.right_query(
                        mold,
                        index_file.clone(),
                        view_file.clone(),
                        nbs.bytes(),
                        nsk,
                        0,
                        level + 1,
                        conditions.clone(),
                        skip,
                        limit,
                        delete,
                    )?;
                    total += temp.0;
                    count += temp.1;
                    skip = temp.2;
                    limit = temp.3;
                    res.append(&mut temp.4);
                    if limit <= 0 {
                        break;
                    }
                }
            }
        }
        Ok((total, count, skip, limit, res))
    }
}