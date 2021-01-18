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
// use crate::task::engine::traits::{TIndex, TNode, TSeed};
// use crate::utils::comm::EngineType;
// use crate::utils::path::index_file_path;
// use crate::utils::store::{
//     before_content_bytes, capacity, capacity_u8, engine_type, engine_type_u8, index_type,
//     index_type_u8, metadata_2_bytes, tag, tag_u8, Metadata, HD,
// };
// use crate::utils::writer::obtain_write_append_file;
// use chrono::{Duration, Local, NaiveDateTime};
// use comm::errors::entrances::GeorgeResult;
// use comm::io::file::create_file;
// use comm::io::writer::write_file_append_bytes;
// use std::fs::File;
// use std::io::{Seek, SeekFrom};
// use std::sync::{Arc, RwLock};
//
// /// Siam索引
// ///
// /// 5位key及16位md5后key及5位起始seek和4位持续seek
// #[derive(Debug)]
// pub struct Index<N: TNode> {
//     /// 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入
//     name: String,
//     /// 是否主键
//     primary: bool,
//     /// 结点
//     root: Arc<N>,
//     /// 文件信息
//     metadata: Metadata,
//     /// 创建时间
//     create_time: Duration,
//     /// 根据文件路径获取该文件追加写入的写对象
//     file_append: Arc<RwLock<File>>,
// }
//
// /// 新建索引
// ///
// /// 该索引需要定义ID，此外索引所表达的字段组成内容也是必须的，并通过primary判断索引类型，具体传参参考如下定义：<p><p>
// ///
// /// ###Params
// ///
// /// index_name 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入
// ///
// /// primary 是否主键
// ///
// /// root 根结点
// ///
// /// metadata 文件信息
// fn new_index<N: TNode>(
//     database_name: String,
//     view_name: String,
//     name: String,
//     primary: bool,
//     root: Arc<N>,
//     metadata: Metadata,
// ) -> GeorgeResult<Index<N>> {
//     let now: NaiveDateTime = Local::now().naive_local();
//     let create_time = Duration::nanoseconds(now.timestamp_nanos());
//     let file_path = index_file_path(database_name, view_name, name.clone());
//     let file_append = obtain_write_append_file(file_path)?;
//     let index = Index {
//         primary,
//         name,
//         root,
//         metadata,
//         create_time,
//         file_append,
//     };
//     Ok(index)
// }
//
// impl<N: TNode> Index<N> {
//     pub(super) fn create(
//         database_name: String,
//         view_name: String,
//         name: String,
//         primary: bool,
//         root: Arc<N>,
//         metadata: Metadata,
//     ) -> GeorgeResult<Arc<RwLock<Self>>> {
//         create_file(
//             index_file_path(database_name.clone(), view_name.clone(), name.clone()),
//             true,
//         )?;
//         let mut index = new_index(
//             database_name.clone(),
//             view_name.clone(),
//             name,
//             primary,
//             root,
//             metadata,
//         )?;
//         let mut metadata_bytes = metadata_2_bytes(index.metadata());
//         let mut description = index.description();
//         // 初始化为32 + 8，即head长度加正文描述符长度
//         let mut before_description = before_content_bytes(40, description.len() as u32);
//         metadata_bytes.append(&mut before_description);
//         metadata_bytes.append(&mut description);
//         index.file_append(database_name, view_name, metadata_bytes)?;
//         Ok(Arc::new(RwLock::new(index)))
//     }
//     /// 根据文件路径获取该文件追加写入的写对象
//     ///
//     /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
//     ///
//     /// #Return
//     ///
//     /// seek_end_before 写之前文件字节数据长度
//     pub(super) fn file_append(
//         &mut self,
//         database_name: String,
//         view_name: String,
//         content: Vec<u8>,
//     ) -> GeorgeResult<u64> {
//         let file_append = self.file_append.clone();
//         let mut file_write = file_append.write().unwrap();
//         let seek_end_before = file_write.seek(SeekFrom::End(0)).unwrap();
//         match write_file_append_bytes(file_write.try_clone().unwrap(), content.clone()) {
//             Ok(()) => Ok(seek_end_before),
//             Err(_err) => {
//                 let file_path = index_file_path(database_name, view_name, self.name());
//                 self.file_append = obtain_write_append_file(file_path)?;
//                 let file_again = self.file_append.write().unwrap();
//                 write_file_append_bytes(file_again.try_clone().unwrap(), content)?;
//                 Ok(seek_end_before)
//             }
//         }
//     }
// }
//
// /// 封装方法函数
// impl<N: TNode> TIndex for Index<N> {
//     fn name(&self) -> String {
//         self.name.clone()
//     }
//     fn is_primary(&self) -> bool {
//         self.primary.clone()
//     }
//     fn metadata(&self) -> Metadata {
//         self.metadata.clone()
//     }
//     fn create_time(&self) -> Duration {
//         self.create_time
//     }
//     fn put(&self, _key: String, _seed: Arc<RwLock<dyn TSeed>>, _force: bool) -> GeorgeResult<()> {
//         unimplemented!()
//     }
//     fn get(&self, _key: String) -> GeorgeResult<Vec<u8>> {
//         unimplemented!()
//     }
// }
//
// impl<N: TNode> Index<N> {
//     fn description(&self) -> Vec<u8> {
//         let metadata = self.metadata();
//         hex::encode(format!(
//             "{}/{}/{}/{}/{}/{}/{}/{}/{}/{}",
//             self.name,
//             self.primary,
//             tag_u8(metadata.tag),
//             engine_type_u8(metadata.engine_type),
//             capacity_u8(metadata.capacity),
//             index_type_u8(metadata.index_type),
//             metadata.version.get(0).unwrap(),
//             metadata.version.get(1).unwrap(),
//             metadata.sequence,
//             self.create_time().num_nanoseconds().unwrap().to_string(),
//         ))
//         .into_bytes()
//     }
//
//     fn recover(database_name: String, view_name: String, hd: HD) -> GeorgeResult<()> {
//         match String::from_utf8(hd.description) {
//             Ok(description_str) => match hex::decode(description_str) {
//                 Ok(vu8) => match String::from_utf8(vu8) {
//                     Ok(real) => {
//                         let mut split = real.split("/");
//                         let name = split.next().unwrap().to_string();
//                         let primary = split.next().unwrap().to_string().parse::<bool>().unwrap();
//                         let tag = tag(split.next().unwrap().to_string().parse::<u8>().unwrap());
//                         let engine_type =
//                             engine_type(split.next().unwrap().to_string().parse::<u8>().unwrap());
//                         let capacity =
//                             capacity(split.next().unwrap().to_string().parse::<u8>().unwrap());
//                         let index_type =
//                             index_type(split.next().unwrap().to_string().parse::<u8>().unwrap());
//                         let mut version: [u8; 2] = [0x00, 0x00];
//                         version[0] = split.next().unwrap().to_string().parse::<u8>().unwrap();
//                         version[1] = split.next().unwrap().to_string().parse::<u8>().unwrap();
//                         let sequence = split.next().unwrap().to_string().parse::<u8>().unwrap();
//                         let create_time = Duration::nanoseconds(
//                             split.next().unwrap().to_string().parse::<i64>().unwrap(),
//                         );
//                         let file_path =
//                             index_file_path(database_name.clone(), view_name.clone(), name.clone());
//                         let file_append = obtain_write_append_file(file_path)?;
//                         let index = Index {
//                             name,
//                             primary,
//                             create_time,
//                             metadata: hd.metadata,
//                             file_append,
//                             root: Arc::new(Default::default()),
//                         };
//                         log::info!(
//                             "recovery index {} from database.view {}.{}",
//                             index.name(),
//                             database_name,
//                             view_name
//                         );
//                         match read_dir(view_path(database_name, view.name())) {
//                             // 恢复indexes数据
//                             Ok(paths) => view.recovery_indexes(paths),
//                             Err(err) => panic!("recovery view read dir failed! error is {}", err),
//                         }
//                         Ok(view)
//                     }
//                     Err(err) => Err(err_string(format!(
//                         "recovery view from utf8 failed! error is {}",
//                         err
//                     ))),
//                 },
//                 Err(err) => Err(err_string(format!(
//                     "recovery view decode failed! error is {}",
//                     err
//                 ))),
//             },
//             Err(err) => Err(err_string(err.to_string())),
//         }
//     }
// }
