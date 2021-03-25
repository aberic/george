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
// use std::fs::File;
//
// use serde::{Deserialize, Serialize};
//
// use comm::errors::children::DataExistError;
// use comm::errors::entrances::{err_str, err_string, err_strs, GeorgeError, GeorgeResult};
// use comm::io::file::{Filer, FilerExecutor, FilerHandler, FilerNormal};
// use comm::trans::{
//     trans_bytes_2_u16, trans_bytes_2_u32, trans_bytes_2_u64, trans_u16_2_bytes, trans_u32_2_bytes,
//     trans_u48_2_bytes,
// };
// use comm::vectors::{Vector, VectorHandler};
//
// use crate::task::engine::traits::TSeed;
// use crate::task::view::View;
// use crate::utils::comm::{is_bytes_fill, VALUE_TYPE_NORMAL};
// use crate::utils::enums::IndexType;
// use comm::strings::{StringHandler, Strings};
//
// /// B+Tree索引叶子结点内防hash碰撞数组结构中单体结构
// ///
// /// 搭配Index使用
// ///
// /// 叶子节点下真实存储数据的集合单体结构
// #[derive(Debug)]
// pub(crate) struct Seed {
//     /// 获取当前结果原始key信息，用于内存版索引
//     key: String,
//     value: Vec<u8>,
//     /// 除主键索引外的其它索引操作策略集合
//     policies: Vec<IndexPolicy>,
// }
//
// /// 视图索引内容
// #[derive(Debug, Clone)]
// pub(crate) struct IndexData {
//     /// 视图文件属性，版本号(2字节)
//     version: u16,
//     /// 数据在表文件中起始偏移量p(6字节)
//     data_seek: u64,
//     /// 使用当前索引的原始key
//     original_key: String,
//     /// 真实存储数据内容
//     value: Vec<u8>,
// }
//
// impl IndexData {
//     /// 创建视图索引内容
//     ///
//     /// view_index_info 循环定位记录使用文件属性
//     pub(crate) fn create(view: View, view_info_index: Vec<u8>) -> GeorgeResult<IndexData> {
//         let version = trans_bytes_2_u16(Vector::sub(view_info_index.clone(), 0, 2)?)?;
//         let path = view.path(version)?;
//         let original_key_len = trans_bytes_2_u64(Vector::sub(view_info_index.clone(), 8, 10)?)?;
//         let original_key = Strings::from_utf8(Vector::sub(
//             view_info_index.clone(),
//             10,
//             10 + original_key_len as usize,
//         )?)?;
//         let data_seek = trans_bytes_2_u64(Vector::sub(view_info_index.clone(), 2, 8)?)?;
//         // 当前数据所在文件对象
//         let file = Filer::reader(path)?;
//         let file_value_len: File;
//         let file_value: File;
//         match file.try_clone() {
//             Ok(f) => file_value_len = f,
//             Err(err) => return Err(err_strs("index data create file try clone", err)),
//         }
//         match file.try_clone() {
//             Ok(f) => file_value = f,
//             Err(err) => return Err(err_strs("index data create file try clone", err)),
//         }
//         let value_len = trans_bytes_2_u32(Filer::read_subs(file_value_len, data_seek, 4)?)?;
//         let value = Filer::read_subs(file_value, data_seek + 4, value_len as usize)?;
//         Ok(IndexData {
//             version,
//             data_seek,
//             original_key,
//             value,
//         })
//     }
//     pub(crate) fn equal_key(&self, key: String) -> bool {
//         key.eq(&self.original_key)
//     }
//     pub(crate) fn value(&self) -> Vec<u8> {
//         self.value.clone()
//     }
//     /// 匹配视图索引key
//     ///
//     /// view_index_info 循环定位记录使用文件属性
//     pub(crate) fn key_exist(key: String, view_index_info: Vec<u8>) -> GeorgeResult<bool> {
//         let original_key_len = trans_bytes_2_u64(Vector::sub(view_index_info.clone(), 8, 10)?)?;
//         let original_key = Strings::from_utf8(Vector::sub(
//             view_index_info.clone(),
//             10,
//             original_key_len as usize,
//         )?)?;
//         Ok(key.eq(&original_key))
//     }
// }
//
// /// 待处理索引操作策略
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub(crate) struct IndexPolicy {
//     index_type: IndexType,
//     /// 使用当前索引的原始key
//     original_key: String,
//     /// 待处理索引文件路径
//     node_file_path: String,
//     /// 待写入索引内容起始偏移量
//     seek: u64,
// }
//
// impl IndexPolicy {
//     fn from(v8s: Vec<u8>) -> GeorgeResult<IndexPolicy> {
//         match serde_json::from_slice(v8s.as_slice()) {
//             Ok(index_policy) => Ok(index_policy),
//             Err(err) => Err(err_string(err.to_string())),
//         }
//     }
//     pub fn bytes(
//         index_type: IndexType,
//         node_file_path: String,
//         seek: u64,
//     ) -> GeorgeResult<Vec<u8>> {
//         let policy = IndexPolicy {
//             index_type,
//             original_key: "".to_string(),
//             node_file_path,
//             seek,
//         };
//         match serde_json::to_vec(&policy) {
//             Ok(v8s) => Ok(v8s),
//             Err(err) => Err(err_string(err.to_string())),
//         }
//     }
//     fn node_file_path(&self) -> String {
//         self.node_file_path.clone()
//     }
//     fn original_key(&self) -> String {
//         self.original_key.clone()
//     }
//     /// 检出索引记录表文档属性
//     ///
//     /// view_version_bytes 视图文件属性，版本号(2字节)
//     pub(super) fn check_out_bytes(&self, mut view_version_bytes: Vec<u8>) -> GeorgeResult<Vec<u8>> {
//         Filer::try_touch(self.node_file_path())?;
//         let file = Filer::reader_writer(self.node_file_path())?;
//         // 表内容索引(8字节)，记录表文件属性(数据归档/定位文件用2字节)+数据在表文件中起始偏移量p(6字节)
//         let check_out_bytes = Filer::read_subs(file, self.seek, 8)?;
//         // 如果读取到不为空，则表明该数据已经存在
//         if is_bytes_fill(check_out_bytes.clone()) {
//             Ok(check_out_bytes)
//         } else {
//             // 如果读取到为空，则表明该数据为首次插入
//             view_version_bytes.append(&mut Vector::create_empty_bytes(6));
//             Ok(view_version_bytes)
//         }
//     }
//     /// 执行索引落库操作
//     ///
//     /// view 视图对象
//     ///
//     /// view_version_bytes 视图文件属性，版本号(2字节)
//     ///
//     /// view_index_info 表内容索引(8字节)，记录表文件属性(数据归档/定位文件用2字节)+数据在表文件中起始偏移量p(6字节)
//     fn exec(
//         &self,
//         view: View,
//         view_version_bytes: Vec<u8>,
//         view_index_info: Vec<u8>,
//         force: bool,
//     ) -> GeorgeResult<()> {
//         Filer::try_touch(self.node_file_path())?;
//         let file = Filer::reader_writer(self.node_file_path())?;
//         // 表内容索引(8字节)，记录表文件属性(数据归档/定位文件用2字节)+数据在表文件中起始偏移量p(6字节)
//         let check_out_bytes;
//         match file.try_clone() {
//             Ok(file) => check_out_bytes = Filer::read_subs(file, self.seek, 8)?,
//             Err(err) => return Err(err_strs("seed exec file try clone1", err)),
//         }
//         // 如果读取到不为空，则表明该数据已经存在
//         if is_bytes_fill(check_out_bytes) {
//             // todo 先读取视图数据，比对是否为碰撞数据
//
//             if force {}
//             Err(GeorgeError::DataExistError(DataExistError))
//         } else {
//             // 如果读取到为空，则表明该数据为首次插入
//             self.record_normal(
//                 file,
//                 self.original_key(),
//                 view,
//                 view_version_bytes,
//                 view_index_info,
//             )
//         }
//     }
//     /// 生成表内容索引(8字节)+原始key长度+原始key
//     ///
//     /// view_seek_start_bytes 数据在视图文件中起始偏移量p(8字节)
//     fn view_info_index_data(&self, key: String, mut view_index_info: Vec<u8>) -> Vec<u8> {
//         // 原始key字节数组
//         let mut key_bytes = key.into_bytes();
//         // 原始key字节数组长度
//         let mut key_bytes_len_bytes = trans_u16_2_bytes(key_bytes.len() as u16);
//         // 视图内容索引(8字节)+原始key长度
//         view_index_info.append(&mut key_bytes_len_bytes);
//         // 视图内容索引(8字节)+原始key长度+原始key
//         view_index_info.append(&mut key_bytes);
//         view_index_info
//     }
//     /// 执行索引落库操作
//     ///
//     /// index_class 索引属性：主键溯源；主键不溯源；普通索引
//     ///
//     /// view 视图对象
//     ///
//     /// view_version_bytes 当前视图文件属性，版本号(2字节)
//     ///
//     /// view_index_info 表内容索引(8字节)，记录表文件属性(数据归档/定位文件用2字节)+数据在表文件中起始偏移量p(6字节)
//     fn record_normal(
//         &self,
//         file: File,
//         key: String,
//         view: View,
//         view_version_bytes: Vec<u8>,
//         view_index_info: Vec<u8>,
//     ) -> GeorgeResult<()> {
//         let mut view_info_index_data = self.view_info_index_data(key, view_index_info);
//         // 定位文件持续长度(4字节)
//         let mut pos_data_len_bytes = trans_u32_2_bytes(view_info_index_data.len() as u32);
//         // 定位文件首次插入数据类型为正常数据类型
//         let mut pos_data_bytes = vec![VALUE_TYPE_NORMAL];
//         // 生成定位数据字节数组=数据类型(1字节)+持续长度(4字节)
//         pos_data_bytes.append(&mut pos_data_len_bytes);
//         // 生成完整存储数据字节数组=数据类型(1字节)+持续长度(4字节)+数据字节数组
//         pos_data_bytes.append(&mut view_info_index_data);
//         // 执行视图存储操作，得到定位数据起始偏移量
//         let pos_seek_start = view.write_content(pos_data_bytes)?;
//         // 记录表文件属性(数据归档/定位文件用2字节)+数据在表文件中起始偏移量p(6字节)
//         let mut index_info_index = view_version_bytes;
//         // 记录表文件属性为当前视图版本号(数据归档/定位文件用2字节)
//         // 生成数据在表文件中起始偏移量p(6字节)
//         let mut pos_seek_start_bytes = trans_u48_2_bytes(pos_seek_start);
//         // 生成存储在索引中的视图文件坐标信息(8字节)
//         index_info_index.append(&mut pos_seek_start_bytes);
//         // 将视图索引偏移量记录在索引文件指定位置
//         Filer::write_seeks(file, self.seek, index_info_index)
//     }
// }
//
// /// 封装方法函数
// impl Seed {
//     /// 新建seed
//     pub fn create(key: String, value: Vec<u8>) -> Seed {
//         return Seed {
//             key,
//             value,
//             policies: Vec::new(),
//         };
//     }
//     /// 获取当前结果原始key信息
//     fn key(&self) -> String {
//         self.key.clone()
//     }
// }
//
// /// 封装方法函数
// impl TSeed for Seed {
//     fn key(&self) -> String {
//         self.key.clone()
//     }
//     fn value(&self) -> Vec<u8> {
//         self.value.clone()
//     }
//     fn is_none(&self) -> bool {
//         self.value.is_empty()
//     }
//     fn modify(&mut self, value: Vec<u8>) -> GeorgeResult<()> {
//         let mut index_policy = IndexPolicy::from(value)?;
//         index_policy.original_key = self.key();
//         self.policies.push(index_policy);
//         Ok(())
//     }
//     fn save(&mut self, view: View) -> GeorgeResult<()> {
//         // todo 失败回滚
//         if self.policies.len() == 0 {
//             // return Err(err_string(format!(
//             //     "no index found in this view {}",
//             //     view.name()
//             // )));
//             return Ok(());
//         }
//         let mut value = self.value();
//         let value_len = value.len() as u32;
//         let mut value_bytes = trans_u32_2_bytes(value_len);
//         value_bytes.append(&mut value);
//         // 执行真实存储操作，即索引将seed存入后，允许检索到该结果，但该结果值不存在，仅当所有索引存入都成功，才会执行本方法完成真实存储操作
//         let view_seek_start = view.write_content(value_bytes)?;
//         // 记录视图文件属性(版本号/数据归档/定位文件用2字节)+数据在表文件中起始偏移量p(6字节)
//         // 数据在视图文件中起始偏移量p(6字节)
//         let mut view_seek_start_bytes = trans_u48_2_bytes(view_seek_start);
//         // 生成视图文件属性，版本号(2字节)
//         let view_version_bytes = trans_u16_2_bytes(view.version());
//         // 循环定位记录使用文件属性
//         let mut view_info_index = view_version_bytes.clone();
//         // 记录表文件属性(版本/数据归档/定位文件用2字节)+数据在表文件中起始偏移量p(6字节)
//         view_info_index.append(&mut view_seek_start_bytes);
//
//         // 将在数据在view中的坐标存入各个index
//         for policy in self.policies.to_vec() {
//             match policy.index_type {
//                 IndexType::None => return Err(err_str("index type none is not support!")),
//                 IndexType::Memory => return Err(err_str("index type memory is not support!")),
//                 IndexType::Dossier => return Err(err_str("dossier index need to be complete!")),
//                 _ => policy.exec(
//                     view.clone(),
//                     view_version_bytes.clone(),
//                     view_info_index.clone(),
//                     false,
//                 )?,
//             }
//         }
//         Ok(())
//     }
//     fn remove(&mut self) -> GeorgeResult<()> {
//         // if self.policies.len() == 0 {
//         //     return Ok(());
//         // }
//         // // 生成视图文件属性，版本号(2字节)
//         // let view_version_bytes = trans_u16_2_bytes(view.version());
//         // // 循环定位记录使用文件属性
//         // let mut view_info_index = view_version_bytes.clone();
//         // let mut view_seek_start_bytes = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
//         // // 记录表文件属性(版本/数据归档/定位文件用2字节)+数据在表文件中起始偏移量p(6字节)
//         // view_info_index.append(&mut view_seek_start_bytes);
//         // for policy in self.policies.to_vec() {
//         //     // todo 设计碰撞模型
//         //     policy.exec(
//         //         view.clone(),
//         //         view_version_bytes.clone(),
//         //         view_info_index.clone(),
//         //         true,
//         //     )?
//         // }
//         Ok(())
//     }
// }
