// /*
//  * Copyright (c) 2021. Aberic - All Rights Reserved.
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
// use std::sync::{Arc, RwLock};
//
// use num_integer::Integer;
//
// use comm::errors::{Errs, GeorgeResult};
// use comm::io::file::FilerHandler;
// use comm::io::Filer;
// use comm::vectors::VectorHandler;
// use comm::Trans;
// use comm::Vector;
//
// use crate::task::engine::block::Node;
// use crate::task::engine::traits::{TForm, TNode, TSeed};
// use crate::task::engine::{DataReal, RootBytes};
// use crate::task::rich::Condition;
// use crate::task::seed::IndexPolicy;
// use crate::task::{engine, View};
// use crate::utils::comm::{Distance, IndexKey};
// use crate::utils::enums::{IndexType, KeyType};
// use crate::utils::writer::Filed;
// use crate::utils::Paths;
//
// const BYTES_LEN_FOR_DISK: usize = 16380;
// const BYTES_LEN_FOR_DISK_LEAF: usize = 7020;
//
// impl Node {
//     /// 新建根结点
//     ///
//     /// 该结点没有Links，也没有preNode，是B+Tree的创世结点
//     pub fn create(
//         form: Arc<RwLock<dyn TForm>>,
//         index_name: String,
//         key_type: KeyType,
//         unique: bool,
//     ) -> GeorgeResult<Arc<Self>> {
//         let f_c = form.clone();
//         let f_r = f_c.read().unwrap();
//         let index_path = Paths::index_path(f_r.database_name(), f_r.name(), index_name.clone());
//         let node_filepath = Paths::node_filepath(index_path.clone(), String::from("disk"));
//         let node_filer = Filed::create(node_filepath.clone())?;
//         let record_filepath = Paths::record_filepath(index_path.clone());
//         let record_filer = Filed::create(record_filepath.clone())?;
//         record_filer.append(vec![0x86, 0x87])?;
//         let rb = RootBytes::create(BYTES_LEN_FOR_DISK);
//         node_filer.append(rb.bytes())?;
//         let root_bytes = Arc::new(RwLock::new(rb));
//         Ok(Arc::new(Node {
//             form,
//             index_name,
//             key_type,
//             index_path,
//             node_filepath,
//             record_filepath,
//             unique,
//             node_filer,
//             record_filer,
//             root_bytes,
//         }))
//     }
//
//     /// 恢复根结点
//     pub fn recovery(
//         form: Arc<RwLock<dyn TForm>>,
//         index_name: String,
//         key_type: KeyType,
//         unique: bool,
//     ) -> GeorgeResult<Arc<Self>> {
//         let v_c = form.clone();
//         let v_r = v_c.read().unwrap();
//         let index_path = Paths::index_path(v_r.database_name(), v_r.name(), index_name.clone());
//         let node_filepath = Paths::node_filepath(index_path.clone(), String::from("disk"));
//         let node_filer = Filed::recovery(node_filepath.clone())?;
//         let record_filepath = Paths::record_filepath(index_path.clone());
//         let record_filer = Filed::recovery(record_filepath.clone())?;
//         let rb = node_filer.read(0, BYTES_LEN_FOR_DISK)?;
//         let root_bytes = Arc::new(RwLock::new(RootBytes::recovery(rb, BYTES_LEN_FOR_DISK)?));
//         Ok(Arc::new(Node {
//             form,
//             index_name,
//             key_type,
//             index_path,
//             node_filepath,
//             record_filepath,
//             unique,
//             node_filer,
//             record_filer,
//             root_bytes,
//         }))
//     }
//
//     fn key_type(&self) -> KeyType {
//         self.key_type.clone()
//     }
//
//     fn node_bytes(&self) -> Vec<u8> {
//         self.root_bytes.read().unwrap().bytes()
//     }
//
//     fn node_filepath(&self) -> String {
//         self.node_filepath.clone()
//     }
//
//     /// 根据文件路径获取该文件追加写入的写对象
//     ///
//     /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
//     ///
//     /// #Return
//     ///
//     /// seek_end_before 写之前文件字节数据长度
//     fn node_append(&self, content: Vec<u8>) -> GeorgeResult<u64> {
//         self.node_filer.append(content)
//     }
//
//     fn node_read(&self, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
//         self.node_filer.clone().read(start, last)
//     }
//
//     fn node_write(&self, seek: u64, content: Vec<u8>) -> GeorgeResult<()> {
//         if seek < BYTES_LEN_FOR_DISK as u64 {
//             self.root_bytes
//                 .write()
//                 .unwrap()
//                 .modify(seek as usize, content.clone())
//         }
//         self.node_filer.write(seek, content)
//     }
//
//     fn record_filepath(&self) -> String {
//         self.record_filepath.clone()
//     }
//
//     /// 根据文件路径获取该文件追加写入的写对象
//     ///
//     /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
//     ///
//     /// #Return
//     ///
//     /// seek_end_before 写之前文件字节数据长度
//     fn record_append(&self, content: Vec<u8>) -> GeorgeResult<u64> {
//         self.record_filer.append(content)
//     }
//
//     /// 根据文件路径获取该文件追加写入的空数据
//     ///
//     /// 空数据由`block持续长度(4字节) + block偏移量(6字节) + 下一数据地址(8字节)`组成，共18字节
//     ///
//     /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
//     ///
//     /// #Return
//     ///
//     /// seek_end_before 写之前文件字节数据长度
//     fn record_append_empty(&self) -> GeorgeResult<u64> {
//         self.record_filer.append(Vector::create_empty_bytes(20))
//     }
//
//     fn record_read(&self, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
//         self.record_filer.clone().read(start, last)
//     }
//
//     fn record_write(&self, seek: u64, content: Vec<u8>) -> GeorgeResult<()> {
//         self.record_filer.write(seek, content)
//     }
// }
//
// /// 封装方法函数
// impl TNode for Node {
//     /// 插入数据<p><p>
//     ///
//     /// ###Params
//     ///
//     /// hash_key u64
//     ///
//     /// ###Return
//     ///
//     /// EngineResult<()>
//     fn put(&self, key: String, seed: Arc<RwLock<dyn TSeed>>, force: bool) -> GeorgeResult<()> {
//         let hash_key = IndexKey::hash(self.key_type(), key.clone())?;
//         self.put_in_node(0, self.node_bytes(), key, 1, hash_key, seed, force)
//     }
//
//     fn get(&self, key: String) -> GeorgeResult<DataReal> {
//         let hash_key = IndexKey::hash(self.key_type(), key.clone())?;
//         self.get_in_node(self.node_bytes(), key, 1, hash_key)
//     }
//
//     fn del(&self, key: String, seed: Arc<RwLock<dyn TSeed>>) -> GeorgeResult<()> {
//         let hash_key = IndexKey::hash(self.key_type(), key.clone())?;
//         self.del_in_node(self.node_bytes(), 0, key, 1, hash_key, seed)
//     }
//
//     fn select(
//         &self,
//         left: bool,
//         start: u64,
//         end: u64,
//         skip: u64,
//         limit: u64,
//         delete: bool,
//         conditions: Vec<Condition>,
//     ) -> GeorgeResult<(u64, u64, Vec<Vec<u8>>)> {
//         if left {
//             let (_, _, total, count, values) = self.left_query(
//                 self.node_bytes(),
//                 1,
//                 start,
//                 end,
//                 conditions,
//                 skip,
//                 limit,
//                 delete,
//             )?;
//             Ok((total, count, values))
//         } else {
//             let (_, _, total, count, values) = self.right_query(
//                 self.node_bytes(),
//                 1,
//                 start,
//                 end,
//                 conditions,
//                 skip,
//                 limit,
//                 delete,
//             )?;
//             Ok((total, count, values))
//         }
//     }
// }
//
// impl Node {
//     /// 存储数据真实操作
//     ///
//     /// * node_bytes_seek 当前操作结点的字节数组起始坐标
//     /// * node_bytes 当前操作结点的字节数组
//     /// * key 使用当前索引的原始key
//     /// * level 当前操作结点层
//     /// * flexible_key 下一级最左最小树所对应真实key
//     /// * Seed value信息
//     /// * force 是否强制插入
//     fn put_in_node(
//         &self,
//         node_bytes_seek: u64,
//         node_bytes: Vec<u8>,
//         key: String,
//         level: u8,
//         flexible_key: u64,
//         seed: Arc<RwLock<dyn TSeed>>,
//         force: bool,
//     ) -> GeorgeResult<()>
//     where
//         Self: Sized,
//     {
//         // 如果当前层高为7，则达到最底层，否则递归下一层逻辑
//         if level == 7 {
//             // 由view视图执行save操作时反写进record文件中value起始seek
//             let record_info_seek: u64;
//             // 相对当前结点字节数组，下一结点在字节数组中的偏移量
//             let next_node_start = flexible_key * 6;
//             // 记录在record中有关view数据的字节数组记录坐标，即视图文件中存放数据数组起始坐标
//             let record_seek_bytes = Vector::sub_last(node_bytes, next_node_start as usize, 6)?;
//             // 记录在record中有关view数据的真实坐标
//             let record_seek: u64;
//             // 如果存在坐标值，则继续，否则新建
//             if Vector::is_fill(record_seek_bytes.clone()) {
//                 // 索引执行插入真实坐标6字节
//                 record_seek = Trans::bytes_2_u48(record_seek_bytes)?;
//                 // 已存在该索引值，需要继续判断插入可行性
//                 // 如果唯一且非强制覆盖，返回数据已存在
//                 if self.unique {
//                     if force {
//                         record_info_seek = self.record_view_info_seek_put(
//                             key.clone(),
//                             record_seek,
//                             seed.clone(),
//                             force,
//                         )?;
//                     } else {
//                         return Err(Errs::data_exist_error());
//                     }
//                 } else {
//                     // 如果非唯一，则需要判断hash碰撞，hash碰撞未发生才会继续进行强制性判断
//                     record_info_seek = self.record_view_info_seek_put(
//                         key.clone(),
//                         record_seek,
//                         seed.clone(),
//                         force,
//                     )?;
//                 }
//             } else {
//                 // 不存在下一坐标值，新建
//                 // record追加新链式子结构
//                 record_info_seek = self.record_append_empty()?;
//                 // record新追加链式子结构坐标字节数组
//                 let record_info_seek_bytes = Trans::u48_2_bytes(record_info_seek);
//                 // record起始链式结构在node文件中真实坐标
//                 record_seek = node_bytes_seek + next_node_start;
//                 // 将record新追加链式子结构坐标字节数组写入record起始链式结构在node文件中真实坐标
//                 self.node_write(record_seek, record_info_seek_bytes)?;
//             }
//             seed.write().unwrap().modify_4_put(IndexPolicy::create(
//                 key,
//                 IndexType::Disk,
//                 self.record_filepath(),
//                 record_info_seek,
//             ));
//             Ok(())
//         } else {
//             // 通过当前树下一层高获取结点间间隔数量，即每一度中存在的元素数量
//             let distance = Distance::level_64s(level);
//             // 通过当前层真实key除以下一层间隔数获取结点处在下一层的度数和模
//             let (next_degree, rem) = flexible_key.div_rem(&distance);
//             // 相对当前结点字节数组，下一结点在字节数组中的偏移量
//             let next_node_start = next_degree * 14;
//             // 如果模为0，则表示在当前层对应度节点可获取该数据
//             if rem == 0 {
//                 // 由view视图执行save操作时反写进record文件中value起始seek
//                 let record_info_seek: u64;
//                 // 获取当前数据指针在结点中记录的字节数组起始坐标(下一结点指针8字节 + 当前数据指针6字节)
//                 let next_node_record_start = (next_node_start + 8) as usize;
//                 // 记录在record中有关view数据的字节数组记录坐标，即视图文件中存放数据数组起始坐标
//                 let record_seek_bytes =
//                     Vector::sub_last(node_bytes.clone(), next_node_record_start, 6)?;
//                 // 记录在record中有关view数据的真实坐标
//                 let record_seek: u64;
//                 // 如果存在坐标值，则继续，否则新建
//                 if Vector::is_fill(record_seek_bytes.clone()) {
//                     // 索引执行插入真实坐标6字节
//                     record_seek = Trans::bytes_2_u48(record_seek_bytes)?;
//                     // 已存在该索引值，需要继续判断插入可行性
//                     // 如果唯一且非强制覆盖，返回数据已存在
//                     if self.unique {
//                         if force {
//                             record_info_seek = self.record_view_info_seek_put(
//                                 key.clone(),
//                                 record_seek,
//                                 seed.clone(),
//                                 force,
//                             )?;
//                         } else {
//                             return Err(Errs::data_exist_error());
//                         }
//                     } else {
//                         // 如果非唯一，则需要判断hash碰撞，hash碰撞未发生才会继续进行强制性判断
//                         record_info_seek = self.record_view_info_seek_put(
//                             key.clone(),
//                             record_seek,
//                             seed.clone(),
//                             force,
//                         )?;
//                     }
//                 } else {
//                     // 不存在下一坐标值，新建
//                     // record追加新链式子结构
//                     record_info_seek = self.record_append_empty()?;
//                     // record新追加链式子结构坐标字节数组
//                     let mut record_seek_bytes = Trans::u48_2_bytes(record_info_seek);
//                     // record起始链式结构在node文件中真实坐标，即数据指针6字节
//                     record_seek = node_bytes_seek + next_node_start;
//                     // 记录在结点中的字节数组(下一结点指针8字节 + 当前数据指针6字节)
//                     let mut content = Vector::sub_last(node_bytes, next_node_start as usize, 8)?;
//                     content.append(&mut record_seek_bytes);
//                     // 将record新追加链式子结构坐标字节数组写入record起始链式结构在node文件中真实坐标
//                     self.node_write(record_seek, content)?;
//                 }
//                 seed.write().unwrap().modify_4_put(IndexPolicy::create(
//                     key,
//                     IndexType::Disk,
//                     self.record_filepath(),
//                     record_info_seek,
//                 ));
//                 Ok(())
//             } else {
//                 // 下一结点的真实坐标
//                 let next_node_seek: u64;
//                 // 通过当前层真实key减去下一层的度数与间隔数的乘积获取结点所在下一层的真实key
//                 let next_flexible_key = flexible_key - next_degree * distance;
//                 // 下一结点字节数组起始坐标
//                 let mut next_node_seek_bytes =
//                     Vector::sub_last(node_bytes.clone(), next_node_start as usize, 8)?;
//                 // 下一结点字节数组
//                 let next_node_bytes: Vec<u8>;
//                 // 如果存在坐标值，则继续，否则新建
//                 if Vector::is_fill(next_node_seek_bytes.clone()) {
//                     next_node_seek = Trans::bytes_2_u64(next_node_seek_bytes)?;
//                     if level == 6 {
//                         next_node_bytes =
//                             self.node_read(next_node_seek, BYTES_LEN_FOR_DISK_LEAF)?;
//                     } else {
//                         next_node_bytes = self.node_read(next_node_seek, BYTES_LEN_FOR_DISK)?;
//                     }
//                 } else {
//                     if level == 6 {
//                         // 创建新的结点字节数组
//                         next_node_bytes = Vector::create_empty_bytes(BYTES_LEN_FOR_DISK_LEAF);
//                         // 将新的结点字节数组写入node_file并返回写入前的起始坐标
//                         next_node_seek = self.node_append(next_node_bytes.clone())?;
//                         next_node_seek_bytes = Trans::u64_2_bytes(next_node_seek);
//                     } else {
//                         // 创建新的结点字节数组
//                         next_node_bytes = Vector::create_empty_bytes(BYTES_LEN_FOR_DISK);
//                         // 将新的结点字节数组写入node_file并返回写入前的起始坐标
//                         next_node_seek = self.node_append(next_node_bytes.clone())?;
//                         next_node_seek_bytes = Trans::u64_2_bytes(next_node_seek);
//                         // record起始链式结构在node文件中真实坐标，即数据指针6字节
//                         let mut record_seek_bytes = Vector::create_empty_bytes(6);
//                         // 记录在结点中的字节数组(下一结点指针8字节 + 当前数据指针6字节)
//                         next_node_seek_bytes.append(&mut record_seek_bytes);
//                     }
//                     // 下一结点坐标记录在文件中的坐标
//                     let next_node_seek_real_seek = node_bytes_seek + next_node_start;
//                     self.node_write(next_node_seek_real_seek, next_node_seek_bytes)?;
//                 }
//                 self.put_in_node(
//                     next_node_seek,
//                     next_node_bytes,
//                     key,
//                     level + 1,
//                     next_flexible_key,
//                     seed,
//                     force,
//                 )
//             }
//         }
//     }
//
//     /// 获取由view视图执行save操作时反写进record文件中value起始seek
//     fn record_view_info_seek_put(
//         &self,
//         key: String,
//         record_seek: u64,
//         seed: Arc<RwLock<dyn TSeed>>,
//         force: bool,
//     ) -> GeorgeResult<u64> {
//         // 读取record中该坐标值
//         let res = self.record_read(record_seek, 20)?;
//         // record存储固定长度的数据，长度为20，即view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节) + 链式后续数据(8字节)
//         // 读取view版本号(2字节)
//         let view_version = Trans::bytes_2_u16(Vector::sub(res.clone(), 0, 2)?)?;
//         // 读取view持续长度(4字节)
//         let view_data_len = Trans::bytes_2_u32(Vector::sub(res.clone(), 2, 6)?)?;
//         // 读取view偏移量(6字节)
//         let view_data_seek = Trans::bytes_2_u48(Vector::sub(res.clone(), 6, 12)?)?;
//         // 如果view视图真实数据坐标为空
//         // 处理因断点、宕机等意外导致后续索引数据写入成功而视图数据写入失败的问题
//         if view_data_seek > 0 {
//             // 从view视图中读取真实数据内容
//             let info = self.form.read().unwrap().read_content(
//                 view_version,
//                 view_data_len,
//                 view_data_seek,
//             )?;
//             // 将字节数组内容转换为可读kv
//             let date = DataReal::from(info)?;
//             // 因为hash key指向同一碰撞，对比key是否相同
//             if date.key == seed.read().unwrap().key() {
//                 // 如果key相同，则判断是否强制覆盖
//                 if force {
//                     // 如果强制覆盖，则返回当前待覆盖坐标
//                     Ok(record_seek)
//                 } else {
//                     // 如果不能覆盖，则返回数据已存在
//                     Err(Errs::data_exist_error())
//                 }
//                 // 如果key不同，则发生hash碰撞，开启索引链式结构循环坐标定位
//             } else {
//                 // record存储固定长度的数据，长度为20，即view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节) + 链式后续数据(8字节)
//                 // 读取链式后续数据坐标
//                 let record_next_seek_bytes = Vector::sub_last(res, 12, 8)?;
//                 // 如果链式后续数据有值，则进入下一轮判定
//                 if Vector::is_fill(record_next_seek_bytes.clone()) {
//                     let record_next_seek = Trans::bytes_2_u64(record_next_seek_bytes)?;
//                     self.record_view_info_seek_put(key, record_next_seek, seed, force)
//                     // 如果链式后续数据无值，则插入新数据
//                 } else {
//                     // record追加新链式子结构
//                     let record_next_seek = self.record_append(Vector::create_empty_bytes(20))?;
//                     // record新追加链式子结构坐标字节数组
//                     let record_next_seek_bytes = Trans::u64_2_bytes(record_next_seek);
//                     // 当前record中链式子结构坐标在record文件中记录的坐标位置
//                     let record_next_seek_seek = record_seek + 12;
//                     // 将下一record的坐标写入当前record链式子结构字节数组中
//                     self.record_write(record_next_seek_seek, record_next_seek_bytes)?;
//                     Ok(record_next_seek)
//                 }
//             }
//         } else {
//             // 处理因断点、宕机等意外导致后续索引数据写入成功而视图数据写入失败的问题
//             Ok(record_seek)
//         }
//     }
//
//     /// 读取数据真实操作
//     ///
//     /// * node_bytes 当前操作结点的字节数组
//     /// * key 使用当前索引的原始key
//     /// * level 当前操作结点层
//     /// * flexible_key 下一级最左最小树所对应真实key
//     fn get_in_node(
//         &self,
//         node_bytes: Vec<u8>,
//         key: String,
//         level: u8,
//         flexible_key: u64,
//     ) -> GeorgeResult<DataReal> {
//         // 如果当前层高为7，则达到最底层，否则递归下一层逻辑
//         if level == 7 {
//             // 相对当前结点字节数组，下一结点在字节数组中的偏移量
//             let next_node_start = flexible_key * 6;
//             // 记录在record中有关view数据的字节数组记录坐标，即视图文件中存放数据数组起始坐标
//             let record_seek_bytes = Vector::sub_last(node_bytes, next_node_start as usize, 6)?;
//             self.judge_seek_bytes(key, record_seek_bytes)
//         } else {
//             // 通过当前树下一层高获取结点间间隔数量，即每一度中存在的元素数量
//             let distance = Distance::level_64s(level);
//             // 通过当前层真实key除以下一层间隔数获取结点处在下一层的度数和模
//             let (next_degree, rem) = flexible_key.div_rem(&distance);
//             // 相对当前结点字节数组，下一结点在字节数组中的偏移量
//             let next_node_start = next_degree * 14;
//             // 如果模为0，则表示在当前层对应度节点可获取该数据
//             if rem == 0 {
//                 // 获取当前数据指针在结点中记录的字节数组起始坐标(下一结点指针8字节 + 当前数据指针6字节)
//                 let next_node_record_start = (next_node_start + 8) as usize;
//                 // 记录在record中有关view数据的字节数组记录坐标，即视图文件中存放数据数组起始坐标
//                 let record_seek_bytes = Vector::sub_last(node_bytes, next_node_record_start, 6)?;
//                 self.judge_seek_bytes(key, record_seek_bytes)
//             } else {
//                 // 下一结点字节数组起始坐标
//                 let next_node_seek_bytes =
//                     Vector::sub_last(node_bytes, next_node_start as usize, 8)?;
//                 // 如果存在坐标值，则继续，否则新建
//                 if Vector::is_fill(next_node_seek_bytes.clone()) {
//                     // 下一结点的真实坐标
//                     let next_node_seek = Trans::bytes_2_u64(next_node_seek_bytes)?;
//                     // 下一结点字节数组
//                     let next_node_bytes: Vec<u8>;
//                     if level == 6 {
//                         next_node_bytes =
//                             self.node_read(next_node_seek, BYTES_LEN_FOR_DISK_LEAF)?;
//                     } else {
//                         next_node_bytes = self.node_read(next_node_seek, BYTES_LEN_FOR_DISK)?;
//                     }
//                     // 通过当前层真实key减去下一层的度数与间隔数的乘积获取结点所在下一层的真实key
//                     let next_flexible_key = flexible_key - next_degree * distance;
//                     self.get_in_node(next_node_bytes, key, level + 1, next_flexible_key)
//                 } else {
//                     // 如果为空，则返回无此数据
//                     Err(Errs::data_no_exist_error())
//                 }
//             }
//         }
//     }
//
//     /// 期望根据`下一结点偏移量字节数组`获取由view视图执行save操作时反写进record文件中value起始seek
//     ///
//     /// 如果存在坐标值，则继续，否则返回无此数据
//     fn judge_seek_bytes(&self, key: String, record_seek_bytes: Vec<u8>) -> GeorgeResult<DataReal> {
//         // 如果存在坐标值，则继续，否则返回无此数据
//         if Vector::is_fill(record_seek_bytes.clone()) {
//             // 索引执行插入真实坐标
//             let record_seek = Trans::bytes_2_u48(record_seek_bytes)?;
//             self.record_view_info_seek_get(key, record_seek)
//         } else {
//             // 如果为空，则返回无此数据
//             Err(Errs::data_no_exist_error())
//         }
//     }
//
//     /// 获取由view视图执行save操作时反写进record文件中value起始seek
//     fn record_view_info_seek_get(&self, key: String, record_seek: u64) -> GeorgeResult<DataReal> {
//         // 读取record中该坐标值
//         // record存储固定长度的数据，长度为20，即view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节) + 链式后续数据(8字节)
//         let res = self.record_read(record_seek, 20)?;
//         // 读取view版本号(2字节)
//         let view_version = Trans::bytes_2_u16(Vector::sub(res.clone(), 0, 2)?)?;
//         // 读取view持续长度(4字节)
//         let view_data_len = Trans::bytes_2_u32(Vector::sub(res.clone(), 2, 6)?)?;
//         // 读取view偏移量(6字节)
//         let view_data_seek = Trans::bytes_2_u48(Vector::sub(res.clone(), 6, 12)?)?;
//         // 如果view视图真实数据坐标为空
//         // 处理因断点、宕机等意外导致后续索引数据写入成功而视图数据写入失败的问题
//         if view_data_seek > 0 {
//             // 从view视图中读取真实数据内容
//             let info = self.form.read().unwrap().read_content(
//                 view_version,
//                 view_data_len,
//                 view_data_seek,
//             )?;
//             // 将字节数组内容转换为可读kv
//             let date = DataReal::from(info)?;
//             // 因为hash key指向同一碰撞，对比key是否相同
//             if date.key == key {
//                 Ok(date)
//             } else {
//                 // 如果key不同，则需要进一步判断是否唯一
//                 // 如果唯一，则不存在hash碰撞
//                 if self.unique {
//                     Err(Errs::data_no_exist_error())
//                 } else {
//                     // 不唯一则可能发生hash碰撞，开启索引链式结构循环坐标定位
//                     // record存储固定长度的数据，长度为20，即view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节) + 链式后续数据(8字节)
//                     // 读取链式后续数据坐标
//                     let record_seek_bytes = Vector::sub_last(res, 12, 8)?;
//                     self.judge_seek_bytes(key, record_seek_bytes)
//                 }
//             }
//         } else {
//             // record存储固定长度的数据，长度为20，即view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节) + 链式后续数据(8字节)
//             // 读取链式后续数据坐标
//             let record_seek_bytes = Vector::sub_last(res, 12, 8)?;
//             self.judge_seek_bytes(key, record_seek_bytes)
//         }
//     }
//
//     fn del_in_node(
//         &self,
//         node_bytes: Vec<u8>,
//         node_real_seek: u64,
//         key: String,
//         level: u8,
//         flexible_key: u64,
//         seed: Arc<RwLock<dyn TSeed>>,
//     ) -> GeorgeResult<()> {
//         // 如果当前层高为7，则达到最底层，否则递归下一层逻辑
//         if level == 7 {
//             // 相对当前结点字节数组，下一结点在字节数组中的偏移量
//             let next_node_start = flexible_key * 6;
//             // 如果唯一，直接删除
//             if self.unique {
//                 seed.write().unwrap().modify_4_del(IndexPolicy::create(
//                     key,
//                     IndexType::Disk,
//                     self.node_filepath(),
//                     node_real_seek + next_node_start as u64,
//                 ));
//                 Ok(())
//             } else {
//                 // 记录在record中有关view数据的字节数组记录坐标，即视图文件中存放数据数组起始坐标
//                 let record_seek_bytes = Vector::sub_last(node_bytes, next_node_start as usize, 6)?;
//                 self.judge_seek_bytes_for_del(key, record_seek_bytes, seed)
//             }
//         } else {
//             // 通过当前树下一层高获取结点间间隔数量，即每一度中存在的元素数量
//             let distance = Distance::level_64s(level);
//             // 通过当前层真实key除以下一层间隔数获取结点处在下一层的度数和模
//             let (next_degree, rem) = flexible_key.div_rem(&distance);
//             // 相对当前结点字节数组，下一结点在字节数组中的偏移量
//             let next_node_start = next_degree * 14;
//             // 如果模为0，则表示在当前层对应度节点可获取该数据
//             if rem == 0 {
//                 // 获取当前数据指针在结点中记录的字节数组起始坐标(下一结点指针8字节 + 当前数据指针6字节)
//                 let next_node_record_start = (next_node_start + 8) as usize;
//                 // 记录在record中有关view数据的字节数组记录坐标，即视图文件中存放数据数组起始坐标
//                 let record_seek_bytes = Vector::sub_last(node_bytes, next_node_record_start, 6)?;
//                 self.judge_seek_bytes_for_del(key, record_seek_bytes, seed)
//             } else {
//                 // 下一结点字节数组起始坐标
//                 let next_node_seek_bytes =
//                     Vector::sub_last(node_bytes, next_node_start as usize, 8)?;
//                 // 如果存在坐标值，则继续，否则新建
//                 if Vector::is_fill(next_node_seek_bytes.clone()) {
//                     // 下一结点的真实坐标
//                     let next_node_real_seek = Trans::bytes_2_u64(next_node_seek_bytes)?;
//                     // 下一结点字节数组
//                     let next_node_bytes: Vec<u8>;
//                     if level == 6 {
//                         next_node_bytes =
//                             self.node_read(next_node_real_seek, BYTES_LEN_FOR_DISK_LEAF)?;
//                     } else {
//                         next_node_bytes =
//                             self.node_read(next_node_real_seek, BYTES_LEN_FOR_DISK)?;
//                     }
//                     // 通过当前层真实key减去下一层的度数与间隔数的乘积获取结点所在下一层的真实key
//                     let next_flexible_key = flexible_key - next_degree * distance;
//                     self.del_in_node(
//                         next_node_bytes,
//                         next_node_real_seek,
//                         key,
//                         level + 1,
//                         next_flexible_key,
//                         seed,
//                     )
//                 } else {
//                     // 如果为空，则返回无此数据
//                     Ok(())
//                 }
//             }
//         }
//     }
//
//     /// 期望根据`下一结点偏移量字节数组`获取由view视图执行save操作时反写进record文件中value起始seek，用于删除
//     ///
//     /// 如果存在坐标值，则继续，否则返回无此数据
//     fn judge_seek_bytes_for_del(
//         &self,
//         key: String,
//         record_seek_bytes: Vec<u8>,
//         seed: Arc<RwLock<dyn TSeed>>,
//     ) -> GeorgeResult<()> {
//         // 如果存在坐标值，则继续，否则返回无此数据
//         if Vector::is_fill(record_seek_bytes.clone()) {
//             // 索引执行插入真实坐标
//             let record_seek = Trans::bytes_2_u48(record_seek_bytes)?;
//             self.record_view_info_seek_del(key, record_seek, seed)
//         } else {
//             // 如果为空，则什么也不做
//             Ok(())
//         }
//     }
//
//     /// 获取由view视图执行save操作时反写进record文件中value起始seek，用于删除
//     fn record_view_info_seek_del(
//         &self,
//         key: String,
//         record_seek: u64,
//         seed: Arc<RwLock<dyn TSeed>>,
//     ) -> GeorgeResult<()> {
//         // 读取record中该坐标值
//         // record存储固定长度的数据，长度为20，即view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节) + 链式后续数据(8字节)
//         let res = self.record_read(record_seek, 20)?;
//         // 读取view版本号(2字节)
//         let view_version = Trans::bytes_2_u16(Vector::sub(res.clone(), 0, 2)?)?;
//         // 读取view持续长度(4字节)
//         let view_data_len = Trans::bytes_2_u32(Vector::sub(res.clone(), 2, 6)?)?;
//         // 读取view偏移量(6字节)
//         let view_data_seek = Trans::bytes_2_u48(Vector::sub(res.clone(), 6, 12)?)?;
//         // 如果view视图真实数据坐标为空
//         // 处理因断点、宕机等意外导致后续索引数据写入成功而视图数据写入失败的问题
//         if view_data_seek > 0 {
//             // 从view视图中读取真实数据内容
//             let info = self.form.read().unwrap().read_content(
//                 view_version,
//                 view_data_len,
//                 view_data_seek,
//             )?;
//             // 将字节数组内容转换为可读kv
//             let date = DataReal::from(info)?;
//             // 因为hash key指向同一碰撞，对比key是否相同
//             if date.key == key {
//                 // 可能存在hash碰撞，将后续索引链式结构循环坐标读取出来
//                 let next_record_seek_bytes = Vector::sub_last(res, 12, 8)?;
//                 // 如果后续坐标内容为空，则不存在后续数据，直接将待删除内容替换为空字节数组即可
//                 if Vector::is_empty(next_record_seek_bytes.clone()) {
//                     seed.write().unwrap().modify_4_del(IndexPolicy::create(
//                         key,
//                         IndexType::Disk,
//                         self.record_filepath(),
//                         record_seek,
//                     ));
//                 } else {
//                     // 如果存在后续坐标内容
//                     // 获取下一个索引执行插入真实坐标
//                     let next_record_seek = Trans::bytes_2_u64(next_record_seek_bytes)?;
//                     // 获取下一个record存储固定长度的数据，长度为20，即view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节) + 链式后续数据(8字节)
//                     let next_record_bytes = self.record_read(next_record_seek, 20)?;
//                     // 将下一个record记录写入当前记录，以此实现删除
//                     seed.write()
//                         .unwrap()
//                         .modify_4_del(IndexPolicy::create_custom(
//                             key,
//                             self.record_filepath(),
//                             record_seek,
//                             next_record_bytes,
//                         ));
//                 }
//                 Ok(())
//             } else {
//                 // 不唯一则可能发生hash碰撞，开启索引链式结构循环坐标定位
//                 // record存储固定长度的数据，长度为20，即view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节) + 链式后续数据(8字节)
//                 // 读取链式后续数据坐标
//                 let next_record_seek_bytes = Vector::sub_last(res, 12, 8)?;
//                 self.judge_seek_bytes_for_del(key, next_record_seek_bytes, seed)
//             }
//         } else {
//             // record存储固定长度的数据，长度为20，即view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节) + 链式后续数据(8字节)
//             // 读取链式后续数据坐标
//             let next_record_seek_bytes = Vector::sub_last(res, 12, 8)?;
//             self.judge_seek_bytes_for_del(key, next_record_seek_bytes, seed)
//         }
//     }
//
//     /// 通过左查询约束获取数据集
//     ///
//     /// ###Params
//     ///
//     /// * level 当前查询树层数
//     /// * node_bytes 当前操作结点的字节数组
//     /// * start 查询起始坐标，如为0则表示前置数据没有起始符
//     /// * end 查询终止坐标，如为0则表示后续数据没有终止符
//     /// * conditions 条件集合
//     /// * skip 结果集跳过数量
//     /// * limit 结果集限制数量
//     /// * delete 是否删除检索结果
//     ///
//     /// ###Return
//     ///
//     /// * skip 结果集跳过数量
//     /// * limit 结果集限制数量
//     /// * total 检索过程中遍历的总条数（也表示文件读取次数，文件描述符次数远小于该数，一般文件描述符数为1，即共用同一文件描述符）
//     /// * count 检索结果过程中遍历的总条数
//     /// * values 检索结果集合
//     fn left_query(
//         &self,
//         node_bytes: Vec<u8>,
//         level: u8,
//         start: u64,
//         end: u64,
//         conditions: Vec<Condition>,
//         mut skip: u64,
//         mut limit: u64,
//         delete: bool,
//     ) -> GeorgeResult<(u64, u64, u64, u64, Vec<Vec<u8>>)> {
//         let mut total: u64 = 0;
//         let mut count: u64 = 0;
//         let mut values: Vec<Vec<u8>> = vec![];
//
//         // 如果当前层高为7，则达到最底层，否则递归下一层逻辑
//         if level == 7 {
//             // 待查询结果索引字节数组
//             let node_seek_bytes = self.node_seek_bytes(node_bytes, start, end)?;
//             // 生成待查询结果索引数组，将待查询结果索引字节数组以每6个字节为一组进行重新组合
//             let vs_res = Vector::find_eq_vec_bytes(node_seek_bytes, 6)?;
//             // 遍历待查询结果索引数组
//             for res in vs_res {
//                 // 如果存在坐标值，则继续，否则返回无此数据
//                 if Vector::is_fill(res.clone()) {
//                     // 索引执行插入真实坐标
//                     let record_seek = Trans::bytes_2_u48(res)?;
//                     let (s, l, t, c, mut v) = self.record_view_info_seek_valid(
//                         record_seek,
//                         conditions.clone(),
//                         skip,
//                         limit,
//                         delete,
//                     )?;
//                     skip = s;
//                     limit = l;
//                     total += t;
//                     count += c;
//                     values.append(&mut v);
//                     // 判断是否已经达到limit要求，如果达到要求，则直接返回数据，否则进入循环查询
//                     if limit <= 0 {
//                         break;
//                     }
//                 } else {
//                     // 如果为空，则继续循环
//                     continue;
//                 }
//             }
//         } else {
//             // 通过当前树下一层高获取结点间间隔数量，即每一度中存在的元素数量
//             let distance = Distance::level_64s(level);
//             // 通过当前层真实`start key`除以下一层间隔数获取结点处在下一层的起始度数
//             let (next_start_degree, rem_start) = start.div_rem(&distance);
//             // 通过当前层真实`end key`除以下一层间隔数获取结点处在下一层的截至度数
//             let (next_end_degree, rem_end) = end.div_rem(&distance);
//             // 相对当前结点字节数组，下一结点在字节数组中的偏移量
//             let next_node_start = (next_start_degree * 14) as usize;
//
//             // 优先获取当前在读字节可匹配到的数据
//             // 如果模为0，则表示在当前层对应度节点可获取该数据
//             if rem_start == 0 {
//                 // 获取当前数据指针在结点中记录的字节数组起始坐标(下一结点指针8字节 + 当前数据指针6字节)
//                 let next_node_record_start = (next_node_start + 8) as usize;
//                 // 记录在record中有关view数据的字节数组记录坐标，即视图文件中存放数据数组起始坐标
//                 let record_seek_bytes =
//                     Vector::sub_last(node_bytes.clone(), next_node_record_start as usize, 6)?;
//                 // 如果存在坐标值，则继续，否则返回无此数据
//                 if Vector::is_fill(record_seek_bytes.clone()) {
//                     // 索引执行插入真实坐标
//                     let record_seek = Trans::bytes_2_u48(record_seek_bytes)?;
//                     let (s, l, t, c, mut v) = self.record_view_info_seek_valid(
//                         record_seek,
//                         conditions.clone(),
//                         skip,
//                         limit,
//                         delete,
//                     )?;
//                     skip = s;
//                     limit = l;
//                     total += t;
//                     count += c;
//                     values.append(&mut v);
//                     // 判断是否已经达到limit要求，如果达到要求，则直接返回数据，否则进入循环查询
//                     if limit <= 0 {
//                         return Ok((skip, limit, total, count, values));
//                     }
//                 }
//             }
//
//             let disk_bytes_len: usize;
//             if level == 6 {
//                 disk_bytes_len = BYTES_LEN_FOR_DISK_LEAF;
//             } else {
//                 disk_bytes_len = BYTES_LEN_FOR_DISK;
//             }
//
//             // 如果下一层的起始度数与下一层的截至度数相同，则表示操作未分层，继续进行左查询
//             if next_start_degree == next_end_degree {
//                 // 通过当前层真实key减去下一层的度数与间隔数的乘积获取结点所在下一层的真实key
//                 let next_start_key = start - next_start_degree * distance;
//                 let next_end_key = end - next_end_degree * distance;
//                 // 下一结点字节数组起始坐标(下一结点指针8字节 + 下一结点数据指针6字节)
//                 let next_node_seek_bytes =
//                     Vector::sub_last(node_bytes.clone(), next_node_start, 8)?;
//                 // 如果存在坐标值，则继续，否则新建
//                 if Vector::is_fill(next_node_seek_bytes.clone()) {
//                     let (s, l, t, c, mut v) = self.next_left_query(
//                         level + 1,
//                         next_start_key,
//                         next_end_key,
//                         conditions.clone(),
//                         next_node_seek_bytes,
//                         disk_bytes_len,
//                         skip,
//                         limit,
//                         delete,
//                     )?;
//                     skip = s;
//                     limit = l;
//                     total += t;
//                     count += c;
//                     values.append(&mut v);
//                     // 判断是否已经达到limit要求，如果达到要求，则直接返回数据，否则进入循环查询
//                     if limit == 0 {
//                         return Ok((skip, limit, total, count, values));
//                     }
//                 }
//             } else {
//                 // 如果下一层的起始度数与下一层的截至度数不同，则表示操作已分层，需要循环左查询
//                 // 需要循环左查询首尾两次为特殊查询
//                 // 首次查询的起始坐标由start确定，终止坐标为0
//                 // 末次查询的终止坐标由end确定，起始坐标为0
//
//                 // 首次查询开始
//                 // 通过当前层真实key减去下一层的度数与间隔数的乘积获取结点所在下一层的真实key
//                 let next_start_key = start - next_start_degree * distance;
//                 // 下一结点字节数组起始坐标
//                 let next_node_seek_bytes =
//                     Vector::sub_last(node_bytes.clone(), next_node_start, 8)?;
//                 // 如果存在坐标值，则继续，否则新建
//                 if Vector::is_fill(next_node_seek_bytes.clone()) {
//                     let (s, l, t, c, mut v) = self.next_left_query(
//                         level + 1,
//                         next_start_key,
//                         0,
//                         conditions.clone(),
//                         next_node_seek_bytes,
//                         disk_bytes_len,
//                         skip,
//                         limit,
//                         delete,
//                     )?;
//                     skip = s;
//                     limit = l;
//                     total += t;
//                     count += c;
//                     values.append(&mut v);
//                     // 判断是否已经达到limit要求，如果达到要求，则直接返回数据，否则进入循环查询
//                     if limit == 0 {
//                         return Ok((skip, limit, total, count, values));
//                     }
//                 }
//                 // 首次查询结束
//
//                 // 循环查询开始
//                 // 待检查起始坐标
//                 let mut check_start_degree = next_start_degree;
//                 while check_start_degree < next_end_degree {
//                     // 相对当前结点字节数组，下一结点在字节数组中的偏移量
//                     let next_node_start = (check_start_degree * 8) as usize;
//                     // 下一结点字节数组起始坐标
//                     let next_node_seek_bytes =
//                         Vector::sub_last(node_bytes.clone(), next_node_start, 8)?;
//                     // 如果存在坐标值，则继续，否则新建
//                     if Vector::is_fill(next_node_seek_bytes.clone()) {
//                         let (s, l, t, c, mut v) = self.next_left_query(
//                             level + 1,
//                             0,
//                             0,
//                             conditions.clone(),
//                             next_node_seek_bytes,
//                             disk_bytes_len,
//                             skip,
//                             limit,
//                             delete,
//                         )?;
//                         skip = s;
//                         limit = l;
//                         total += t;
//                         count += c;
//                         values.append(&mut v);
//                         // 判断是否已经达到limit要求，如果达到要求，则直接返回数据，否则进入循环查询
//                         if limit == 0 {
//                             return Ok((skip, limit, total, count, values));
//                         }
//                     }
//                     // 待检查起始坐标递增1，继续下一轮循环左查询
//                     check_start_degree += 1;
//                 }
//                 // 循环查询结束
//
//                 // 末次查询开始
//                 // 相对当前结点字节数组，下一结点在字节数组中的偏移量
//                 let next_node_end = (next_end_degree * 14) as usize;
//                 // 通过当前层真实key减去下一层的度数与间隔数的乘积获取结点所在下一层的真实key
//                 let next_end_key = end - next_end_degree * distance;
//                 // 下一结点字节数组起始坐标
//                 let next_node_seek_bytes = Vector::sub_last(node_bytes.clone(), next_node_end, 8)?;
//                 // 如果存在坐标值，则继续，否则新建
//                 if Vector::is_fill(next_node_seek_bytes.clone()) {
//                     let (s, l, t, c, mut v) = self.next_left_query(
//                         level + 1,
//                         0,
//                         next_end_key,
//                         conditions.clone(),
//                         next_node_seek_bytes,
//                         disk_bytes_len,
//                         skip,
//                         limit,
//                         delete,
//                     )?;
//                     skip = s;
//                     limit = l;
//                     total += t;
//                     count += c;
//                     values.append(&mut v);
//                     // 判断是否已经达到limit要求，如果达到要求，则直接返回数据，否则进入循环查询
//                     if limit == 0 {
//                         return Ok((skip, limit, total, count, values));
//                     }
//                 }
//                 // 末次查询结束
//             }
//
//             // 如果下一层的起始度数与下一层的截至度数相同，则表示操作未分层，继续进行左查询
//             if rem_end == 0 {
//                 // 相对当前结点字节数组，下一结点在字节数组中的偏移量
//                 let next_node_end = (next_end_degree * 14) as usize;
//                 // 获取当前数据指针在结点中记录的字节数组起始坐标(下一结点指针8字节 + 当前数据指针6字节)
//                 let next_node_record_end = (next_node_end + 8) as usize;
//                 // 记录在record中有关view数据的字节数组记录坐标，即视图文件中存放数据数组起始坐标
//                 let record_seek_bytes =
//                     Vector::sub_last(node_bytes.clone(), next_node_record_end as usize, 6)?;
//                 // 如果存在坐标值，则继续，否则返回无此数据
//                 if Vector::is_fill(record_seek_bytes.clone()) {
//                     // 索引执行插入真实坐标
//                     let record_seek = Trans::bytes_2_u48(record_seek_bytes)?;
//                     let (s, l, t, c, mut v) = self.record_view_info_seek_valid(
//                         record_seek,
//                         conditions.clone(),
//                         skip,
//                         limit,
//                         delete,
//                     )?;
//                     skip = s;
//                     limit = l;
//                     total += t;
//                     count += c;
//                     values.append(&mut v);
//                 }
//             }
//         }
//         Ok((skip, limit, total, count, values))
//     }
//
//     /// 获取下一左查询结果数据集
//     ///
//     /// ###Params
//     ///
//     /// * level 下一查询树层数
//     /// * start 查询起始坐标，如为0则表示前置数据没有起始符
//     /// * end 查询终止坐标，如为0则表示后续数据没有终止符
//     /// * conditions 条件集合
//     /// * next_node_seek_bytes 下一结点字节数组起始坐标
//     /// * disk_bytes 下一结点数组长度
//     /// * skip 结果集跳过数量
//     /// * limit 结果集限制数量
//     /// * delete 是否删除检索结果
//     ///
//     /// ###Return
//     ///
//     /// * skip 结果集跳过数量
//     /// * limit 结果集限制数量
//     /// * total 检索过程中遍历的总条数（也表示文件读取次数，文件描述符次数远小于该数，一般文件描述符数为1，即共用同一文件描述符）
//     /// * count 检索结果过程中遍历的总条数
//     /// * values 检索结果集合
//     fn next_left_query(
//         &self,
//         level: u8,
//         start: u64,
//         end: u64,
//         conditions: Vec<Condition>,
//         next_node_seek_bytes: Vec<u8>,
//         disk_bytes_len: usize,
//         mut skip: u64,
//         mut limit: u64,
//         delete: bool,
//     ) -> GeorgeResult<(u64, u64, u64, u64, Vec<Vec<u8>>)> {
//         let mut total: u64 = 0;
//         let mut count: u64 = 0;
//         let mut values: Vec<Vec<u8>> = vec![];
//         // 下一结点的真实坐标
//         let next_node_seek = Trans::bytes_2_u64(next_node_seek_bytes)?;
//         // 下一结点字节数组
//         let next_node_bytes = self.node_read(next_node_seek, disk_bytes_len)?;
//         let (s, l, t, c, mut v) = self.left_query(
//             next_node_bytes,
//             level,
//             start,
//             end,
//             conditions,
//             skip,
//             limit,
//             delete,
//         )?;
//         skip = s;
//         limit = l;
//         total += t;
//         count += c;
//         values.append(&mut v);
//         Ok((skip, limit, total, count, values))
//     }
//
//     /// 获取由view视图执行save操作时反写进record文件中value
//     fn record_view_info_seek_valid(
//         &self,
//         record_seek: u64,
//         conditions: Vec<Condition>,
//         mut skip: u64,
//         mut limit: u64,
//         delete: bool,
//     ) -> GeorgeResult<(u64, u64, u64, u64, Vec<Vec<u8>>)> {
//         let mut total: u64 = 0;
//         let mut count: u64 = 0;
//         let mut values: Vec<Vec<u8>> = vec![];
//
//         // 读取record中该坐标值
//         // record存储固定长度的数据，长度为20，即view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节) + 链式后续数据(8字节)
//         let res = self.record_read(record_seek, 20)?;
//         let view_info_index = Vector::sub(res.clone(), 0, 12)?;
//         let (valid, value_bytes) = engine::check(
//             self.form.clone(),
//             conditions.clone(),
//             delete,
//             view_info_index,
//         )?;
//         if valid {
//             if skip <= 0 {
//                 limit -= 1;
//                 count += 1;
//                 values.push(value_bytes)
//             } else {
//                 skip -= 1;
//             }
//         }
//         total += 1;
//
//         // 判断是否唯一索引
//         // 如果唯一，则略过
//         // 如果非唯一，则需要继续检查碰撞数据
//         // 同步判断是否已经达到limit要求，如果达到要求，则直接返回数据，否则继续
//         if !self.unique && limit > 0 {
//             // record存储固定长度的数据，长度为20，即view版本号(2字节) + view持续长度(4字节) + view偏移量(6字节) + 链式后续数据(8字节)
//             // 读取链式后续数据坐标
//             let record_next_seek_bytes = Vector::sub_last(res, 12, 8)?;
//             // 如果存在坐标值，则继续，否则返回结果
//             if Vector::is_fill(record_next_seek_bytes.clone()) {
//                 // 索引执行插入真实坐标
//                 let record_seek = Trans::bytes_2_u64(record_next_seek_bytes)?;
//                 let (s, l, t, c, mut v) =
//                     self.record_view_info_seek_valid(record_seek, conditions, skip, limit, delete)?;
//                 skip = s;
//                 limit = l;
//                 total += t;
//                 count += c;
//                 values.append(&mut v);
//             }
//         }
//         Ok((skip, limit, total, count, values))
//     }
//
//     /// 根据查询起止条件获取待查询结果索引字节数组
//     /// * 最底层范围查询分区间查询、左区间查询、右区间查询和全量查询4种
//     /// * 区间查询即start不为0，end不为0，需要取包含start到end及之间的有效数据
//     /// * 左区间查询即start不为0，end为0，需要取包含start到末端及之间的有效数据
//     /// * 右区间查询即start为0，end不为0，需要取包含首端到end及之间的有效数据
//     /// * 全量查询start为0，end为0，需要取首尾两端及之间的有效数据
//     ///
//     /// ###Params
//     ///
//     /// * node_bytes 当前操作结点的字节数组
//     /// * start 查询起始坐标，如为0则表示前置数据没有起始符
//     /// * end 查询终止坐标，如为0则表示后续数据没有终止符
//     ///
//     /// ###Return
//     ///
//     /// * node_seek_bytes 待查询结果索引字节数组
//     fn node_seek_bytes(&self, node_bytes: Vec<u8>, start: u64, end: u64) -> GeorgeResult<Vec<u8>> {
//         // 待查询结果索引字节数组
//         let node_seek_bytes: Vec<u8>;
//         // 最底层范围查询分区间查询、左区间查询、右区间查询和全量查询4种
//         // 区间查询即start不为0，end不为0，需要取包含start到end及之间的有效数据
//         // 左区间查询即start不为0，end为0，需要取包含start到末端及之间的有效数据
//         // 右区间查询即start为0，end不为0，需要取包含首端到end及之间的有效数据
//         // 全量查询start为0，end为0，需要取首尾两端及之间的有效数据
//         if start > 0 && end > 0 {
//             // start不为0，end不为0
//             // 通过当前层真实`start key`除以下一层间隔数获取结点处在下一层的起始度数
//             let next_start_degree = start;
//             // 通过当前层真实`end key`除以下一层间隔数获取结点处在下一层的截至度数
//             let next_end_degree = end + 1;
//             // 相对当前结点字节数组，下一结点在字节数组中的偏移量
//             let next_node_start = (next_start_degree * 6) as usize;
//             let next_node_end = (next_end_degree * 6) as usize;
//             // 待查询结果索引字节数组为包含start到end及之间的有效数据
//             node_seek_bytes = Vector::sub(node_bytes, next_node_start, next_node_end)?;
//         } else if start > 0 && end == 0 {
//             // start不为0，end为0
//             // 通过当前层真实`start key`除以下一层间隔数获取结点处在下一层的起始度数
//             let next_start_degree = start;
//             // 相对当前结点字节数组，下一结点在字节数组中的偏移量
//             let next_node_start = (next_start_degree * 6) as usize;
//             // 待查询结果索引字节数组为包含start到末端及之间的有效数据
//             node_seek_bytes = Vector::sub(node_bytes, next_node_start, 0)?;
//         } else if start == 0 && end > 0 {
//             // start为0，end不为0
//             // 通过当前层真实`end key`除以下一层间隔数获取结点处在下一层的截至度数
//             let next_end_degree = end + 1;
//             // 相对当前结点字节数组，下一结点在字节数组中的偏移量
//             let next_node_end = (next_end_degree * 6) as usize;
//             // 待查询结果索引字节数组为包含包含首端到end及之间的有效数据
//             node_seek_bytes = Vector::sub(node_bytes, 0, next_node_end)?;
//         } else {
//             // start为0，end为0
//             // 待查询结果索引字节数组为包含首尾两端及之间的有效数据
//             node_seek_bytes = node_bytes;
//         }
//         Ok(node_seek_bytes)
//     }
//
//     /// 通过右查询约束获取数据集
//     ///
//     /// ###Params
//     ///
//     /// * level 当前查询树层数
//     /// * node_bytes 当前操作结点的字节数组
//     /// * start 查询起始坐标，如为0则表示前置数据没有起始符
//     /// * end 查询终止坐标，如为0则表示后续数据没有终止符
//     /// * conditions 条件集合
//     /// * skip 结果集跳过数量
//     /// * limit 结果集限制数量
//     /// * delete 是否删除检索结果
//     ///
//     /// ###Return
//     ///
//     /// * skip 结果集跳过数量
//     /// * limit 结果集限制数量
//     /// * total 检索过程中遍历的总条数（也表示文件读取次数，文件描述符次数远小于该数，一般文件描述符数为1，即共用同一文件描述符）
//     /// * count 检索结果过程中遍历的总条数
//     /// * values 检索结果集合
//     fn right_query(
//         &self,
//         node_bytes: Vec<u8>,
//         level: u8,
//         start: u64,
//         end: u64,
//         conditions: Vec<Condition>,
//         mut skip: u64,
//         mut limit: u64,
//         delete: bool,
//     ) -> GeorgeResult<(u64, u64, u64, u64, Vec<Vec<u8>>)> {
//         let mut total: u64 = 0;
//         let mut count: u64 = 0;
//         let mut values: Vec<Vec<u8>> = vec![];
//
//         // 如果当前层高为7，则达到最底层，否则递归下一层逻辑
//         if level == 7 {
//             let node_seek_bytes = self.node_seek_bytes(node_bytes, start, end)?;
//             // 生成待查询结果索引数组，将待查询结果索引字节数组以每6个字节为一组进行重新组合
//             let vs_res = Vector::find_eq_vec_bytes(node_seek_bytes, 6)?;
//             let mut len = vs_res.len();
//             while len.gt(&0) {
//                 // 判断是否已经达到limit要求，如果达到要求，则直接返回数据，否则进入循环查询
//                 if limit <= 0 {
//                     break;
//                 }
//                 match vs_res.get(len - 1) {
//                     Some(res) => {
//                         // 如果存在坐标值，则继续，否则返回无此数据
//                         if Vector::is_fill(res.clone()) {
//                             // 索引执行插入真实坐标
//                             let record_seek = Trans::bytes_2_u48(res.clone())?;
//                             let (s, l, t, c, mut v) = self.record_view_info_seek_valid(
//                                 record_seek,
//                                 conditions.clone(),
//                                 skip,
//                                 limit,
//                                 delete,
//                             )?;
//                             skip = s;
//                             limit = l;
//                             total += t;
//                             count += c;
//                             values.append(&mut v);
//                         }
//                     }
//                     None => return Err(Errs::str("select bytes get none error")),
//                 }
//                 len -= 1;
//             }
//         } else {
//             // 通过当前树下一层高获取结点间间隔数量，即每一度中存在的元素数量
//             let distance = Distance::level_64s(level);
//             // 通过当前层真实`start key`除以下一层间隔数获取结点处在下一层的起始度数
//             let (next_start_degree, rem_start) = start.div_rem(&distance);
//             // 通过当前层真实`end key`除以下一层间隔数获取结点处在下一层的截至度数
//             let (next_end_degree, rem_end) = end.div_rem(&distance);
//             // 相对当前结点字节数组，下一结点在字节数组中的偏移量
//             let next_node_start = (next_start_degree * 14) as usize;
//
//             // 优先获取当前在读字节可匹配到的数据
//             // 如果下一层的起始度数与下一层的截至度数相同，则表示操作未分层，继续进行右查询
//             if rem_end == 0 {
//                 // 相对当前结点字节数组，下一结点在字节数组中的偏移量
//                 let next_node_end = (next_start_degree * 14) as usize;
//                 // 获取当前数据指针在结点中记录的字节数组起始坐标(下一结点指针8字节 + 当前数据指针6字节)
//                 let next_node_record_end = (next_node_end + 8) as usize;
//                 // 记录在record中有关view数据的字节数组记录坐标，即视图文件中存放数据数组起始坐标
//                 let record_seek_bytes =
//                     Vector::sub_last(node_bytes.clone(), next_node_record_end as usize, 6)?;
//                 // 如果存在坐标值，则继续，否则返回无此数据
//                 if Vector::is_fill(record_seek_bytes.clone()) {
//                     // 索引执行插入真实坐标
//                     let record_seek = Trans::bytes_2_u48(record_seek_bytes)?;
//                     let (s, l, t, c, mut v) = self.record_view_info_seek_valid(
//                         record_seek,
//                         conditions.clone(),
//                         skip,
//                         limit,
//                         delete,
//                     )?;
//                     skip = s;
//                     limit = l;
//                     total += t;
//                     count += c;
//                     values.append(&mut v);
//                 }
//             }
//
//             let disk_bytes_len: usize;
//             if level == 6 {
//                 disk_bytes_len = BYTES_LEN_FOR_DISK_LEAF;
//             } else {
//                 disk_bytes_len = BYTES_LEN_FOR_DISK;
//             }
//
//             // 如果下一层的起始度数与下一层的截至度数相同，则表示操作未分层，继续进行右查询
//             if next_start_degree == next_end_degree {
//                 // 通过当前层真实key减去下一层的度数与间隔数的乘积获取结点所在下一层的真实key
//                 let next_start_key = start - next_start_degree * distance;
//                 let next_end_key = end - next_end_degree * distance;
//                 // 下一结点字节数组起始坐标(下一结点指针8字节 + 下一结点数据指针6字节)
//                 let next_node_seek_bytes =
//                     Vector::sub_last(node_bytes.clone(), next_node_start, 8)?;
//                 // 如果存在坐标值，则继续，否则新建
//                 if Vector::is_fill(next_node_seek_bytes.clone()) {
//                     let (s, l, t, c, mut v) = self.next_right_query(
//                         level + 1,
//                         next_start_key,
//                         next_end_key,
//                         conditions.clone(),
//                         next_node_seek_bytes,
//                         disk_bytes_len,
//                         skip,
//                         limit,
//                         delete,
//                     )?;
//                     skip = s;
//                     limit = l;
//                     total += t;
//                     count += c;
//                     values.append(&mut v);
//                     // 判断是否已经达到limit要求，如果达到要求，则直接返回数据，否则进入循环查询
//                     if limit == 0 {
//                         return Ok((skip, limit, total, count, values));
//                     }
//                 }
//             } else {
//                 // 如果下一层的起始度数与下一层的截至度数不同，则表示操作已分层，需要循环右查询
//                 // 需要循环右查询首尾两次为特殊查询
//                 // 首次查询的起始坐标由start确定，终止坐标为0
//                 // 末次查询的终止坐标由end确定，起始坐标为0
//
//                 // 首次查询开始
//                 // 相对当前结点字节数组，下一结点在字节数组中的偏移量
//                 let next_node_end = (next_end_degree * 14) as usize;
//                 // 通过当前层真实key减去下一层的度数与间隔数的乘积获取结点所在下一层的真实key
//                 let next_end_key = end - next_end_degree * distance;
//                 // 下一结点字节数组起始坐标
//                 let next_node_seek_bytes = Vector::sub_last(node_bytes.clone(), next_node_end, 8)?;
//                 // 如果存在坐标值，则继续，否则新建
//                 if Vector::is_fill(next_node_seek_bytes.clone()) {
//                     let (s, l, t, c, mut v) = self.next_right_query(
//                         level + 1,
//                         0,
//                         next_end_key,
//                         conditions.clone(),
//                         next_node_seek_bytes,
//                         disk_bytes_len,
//                         skip,
//                         limit,
//                         delete,
//                     )?;
//                     skip = s;
//                     limit = l;
//                     total += t;
//                     count += c;
//                     values.append(&mut v);
//                     // 判断是否已经达到limit要求，如果达到要求，则直接返回数据，否则进入循环查询
//                     if limit <= 0 {
//                         return Ok((skip, limit, total, count, values));
//                     }
//                 }
//                 // 首次查询结束
//
//                 // 循环查询开始
//                 // 待检查起始坐标
//                 let mut check_end_degree = next_end_degree;
//                 while check_end_degree > next_start_degree {
//                     // 相对当前结点字节数组，下一结点在字节数组中的偏移量
//                     let next_node_start = (check_end_degree * 8) as usize;
//                     // 下一结点字节数组起始坐标
//                     let next_node_seek_bytes =
//                         Vector::sub_last(node_bytes.clone(), next_node_start, 8)?;
//                     // 如果存在坐标值，则继续，否则新建
//                     if Vector::is_fill(next_node_seek_bytes.clone()) {
//                         let (s, l, t, c, mut v) = self.next_right_query(
//                             level + 1,
//                             0,
//                             0,
//                             conditions.clone(),
//                             next_node_seek_bytes,
//                             disk_bytes_len,
//                             skip,
//                             limit,
//                             delete,
//                         )?;
//                         skip = s;
//                         limit = l;
//                         total += t;
//                         count += c;
//                         values.append(&mut v);
//                         // 判断是否已经达到limit要求，如果达到要求，则直接返回数据，否则进入循环查询
//                         if limit == 0 {
//                             return Ok((skip, limit, total, count, values));
//                         }
//                     }
//                     // 待检查起始坐标递减1，继续下一轮循环右查询
//                     check_end_degree -= 1;
//                 }
//                 // 循环查询结束
//
//                 // 末次查询开始
//                 // 通过当前层真实key减去下一层的度数与间隔数的乘积获取结点所在下一层的真实key
//                 let next_start_key = start - next_start_degree * distance;
//                 // 下一结点字节数组起始坐标
//                 let next_node_seek_bytes =
//                     Vector::sub_last(node_bytes.clone(), next_node_start, 8)?;
//                 // 如果存在坐标值，则继续，否则新建
//                 if Vector::is_fill(next_node_seek_bytes.clone()) {
//                     let (s, l, t, c, mut v) = self.next_right_query(
//                         level + 1,
//                         next_start_key,
//                         0,
//                         conditions.clone(),
//                         next_node_seek_bytes,
//                         disk_bytes_len,
//                         skip,
//                         limit,
//                         delete,
//                     )?;
//                     skip = s;
//                     limit = l;
//                     total += t;
//                     count += c;
//                     values.append(&mut v);
//                     // 判断是否已经达到limit要求，如果达到要求，则直接返回数据，否则进入循环查询
//                     if limit == 0 {
//                         return Ok((skip, limit, total, count, values));
//                     }
//                 }
//                 // 末次查询结束
//             }
//
//             // 如果模为0，则表示在当前层对应度节点可获取该数据
//             if rem_start == 0 {
//                 // 获取当前数据指针在结点中记录的字节数组起始坐标(下一结点指针8字节 + 当前数据指针6字节)
//                 let next_node_record_start = (next_node_start + 8) as usize;
//                 // 记录在record中有关view数据的字节数组记录坐标，即视图文件中存放数据数组起始坐标
//                 let record_seek_bytes =
//                     Vector::sub_last(node_bytes.clone(), next_node_record_start as usize, 6)?;
//                 // 如果存在坐标值，则继续，否则返回无此数据
//                 if Vector::is_fill(record_seek_bytes.clone()) {
//                     // 索引执行插入真实坐标
//                     let record_seek = Trans::bytes_2_u48(record_seek_bytes)?;
//                     let (s, l, t, c, mut v) = self.record_view_info_seek_valid(
//                         record_seek,
//                         conditions.clone(),
//                         skip,
//                         limit,
//                         delete,
//                     )?;
//                     skip = s;
//                     limit = l;
//                     total += t;
//                     count += c;
//                     values.append(&mut v);
//                     // 判断是否已经达到limit要求，如果达到要求，则直接返回数据，否则进入循环查询
//                     if limit <= 0 {
//                         return Ok((skip, limit, total, count, values));
//                     }
//                 }
//             }
//         }
//         Ok((skip, limit, total, count, values))
//     }
//
//     /// 获取下一左查询结果数据集
//     ///
//     /// ###Params
//     ///
//     /// * level 下一查询树层数
//     /// * start 查询起始坐标，如为0则表示前置数据没有起始符
//     /// * end 查询终止坐标，如为0则表示后续数据没有终止符
//     /// * conditions 条件集合
//     /// * next_node_seek_bytes 下一结点字节数组起始坐标
//     /// * disk_bytes 下一结点数组长度
//     /// * skip 结果集跳过数量
//     /// * limit 结果集限制数量
//     /// * delete 是否删除检索结果
//     ///
//     /// ###Return
//     ///
//     /// * skip 结果集跳过数量
//     /// * limit 结果集限制数量
//     /// * total 检索过程中遍历的总条数（也表示文件读取次数，文件描述符次数远小于该数，一般文件描述符数为1，即共用同一文件描述符）
//     /// * count 检索结果过程中遍历的总条数
//     /// * values 检索结果集合
//     fn next_right_query(
//         &self,
//         level: u8,
//         start: u64,
//         end: u64,
//         conditions: Vec<Condition>,
//         next_node_seek_bytes: Vec<u8>,
//         disk_bytes_len: usize,
//         mut skip: u64,
//         mut limit: u64,
//         delete: bool,
//     ) -> GeorgeResult<(u64, u64, u64, u64, Vec<Vec<u8>>)> {
//         let mut total: u64 = 0;
//         let mut count: u64 = 0;
//         let mut values: Vec<Vec<u8>> = vec![];
//         // 下一结点的真实坐标
//         let next_node_seek = Trans::bytes_2_u64(next_node_seek_bytes)?;
//         // 下一结点字节数组
//         let next_node_bytes = self.node_read(next_node_seek, disk_bytes_len)?;
//         let (s, l, t, c, mut v) = self.right_query(
//             next_node_bytes,
//             level,
//             start,
//             end,
//             conditions,
//             skip,
//             limit,
//             delete,
//         )?;
//         skip = s;
//         limit = l;
//         total += t;
//         count += c;
//         values.append(&mut v);
//         Ok((skip, limit, total, count, values))
//     }
// }
//
// impl Node {
//     pub fn mock_recovery(
//         view: Arc<RwLock<View>>,
//         index_name: String,
//         key_type: KeyType,
//         unique: bool,
//     ) -> GeorgeResult<Arc<Self>> {
//         let v_c = view.clone();
//         let v_r = v_c.read().unwrap();
//         let index_path = Paths::index_path(v_r.database_name(), v_r.name(), index_name.clone());
//         let node_filepath = Paths::node_filepath(index_path.clone(), String::from("disk"));
//         let node_filer = Filed::mock(node_filepath.clone())?;
//         let record_filepath = Paths::record_filepath(index_path.clone());
//         let record_filer: Filed;
//         if Filer::exist(record_filepath.clone()) {
//             record_filer = Filed::mock(record_filepath.clone())?;
//         } else {
//             record_filer = Filed::mock(record_filepath.clone())?;
//             record_filer.append(vec![0x86, 0x87])?;
//         }
//         let root_bytes: Arc<RwLock<RootBytes>>;
//         match node_filer.read(0, BYTES_LEN_FOR_DISK) {
//             Ok(rb) => {
//                 root_bytes = Arc::new(RwLock::new(RootBytes::recovery(rb, BYTES_LEN_FOR_DISK)?))
//             }
//             Err(_) => {
//                 let rb = RootBytes::create(BYTES_LEN_FOR_DISK);
//                 node_filer.append(rb.bytes())?;
//                 root_bytes = Arc::new(RwLock::new(rb))
//             }
//         }
//         Ok(Arc::new(Node {
//             form: view,
//             index_name,
//             key_type,
//             index_path,
//             node_filepath,
//             record_filepath,
//             unique,
//             node_filer,
//             record_filer,
//             root_bytes,
//         }))
//     }
// }
