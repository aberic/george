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
// use std::collections::HashMap;
// use std::fs::{read_dir, ReadDir};
// use std::sync::{mpsc, Arc, RwLock};
// use std::thread;
//
// use chrono::{Duration, Local, NaiveDateTime};
//
// use comm::errors::children::IndexExistError;
// use comm::errors::{Errs, GeorgeError, GeorgeResult};
// use comm::io::file::FilerReader;
// use comm::io::Filer;
// use comm::strings::StringHandler;
// use comm::vectors::VectorHandler;
// use comm::Strings;
// use comm::Trans;
// use comm::Vector;
//
// use crate::task::engine::traits::{Pigeonhole, TForm, TIndex, TSeed};
// use crate::task::engine::DataReal;
// use crate::task::rich::{Condition, Expectation, Selector};
// use crate::task::Index as IndexDefault;
// use crate::task::Ledger;
// use crate::task::Seed;
// use crate::utils::comm::{
//     IndexKey, INDEX_BLOCK_HASH, INDEX_BLOCK_HEIGHT, INDEX_BLOCK_TX_HASH, INDEX_DISK, INDEX_TX_HASH,
// };
// use crate::utils::enums::{IndexType, KeyType};
// use crate::utils::store::{ContentBytes, Metadata, HD};
// use crate::utils::writer::Filed;
// use crate::utils::Paths;
//
// /// 新建视图
// ///
// /// 具体传参参考如下定义：<p><p>
// ///
// /// ###Params
// ///
// /// mem 是否为内存视图
// fn new_ledger(database_name: String, name: String) -> GeorgeResult<Ledger> {
//     let now: NaiveDateTime = Local::now().naive_local();
//     let create_time = Duration::nanoseconds(now.timestamp_nanos());
//     let filepath = Paths::ledger_filepath(database_name.clone(), name.clone());
//     let filepath_light = Paths::ledger_light_filepath(database_name.clone(), name.clone());
//     let filepath_merkle_light =
//         Paths::ledger_merkle_light_filepath(database_name.clone(), name.clone());
//     let filer = Filed::create(filepath.clone())?;
//     let filer_light = Filed::create(filepath_light.clone())?;
//     let filer_merkle_light = Filed::create(filepath_merkle_light.clone())?;
//     let metadata = Metadata::ledger();
//     let ledger = Ledger {
//         database_name,
//         name,
//         create_time,
//         metadata,
//         filepath,
//         filepath_light,
//         filepath_merkle_light,
//         filer,
//         filer_light,
//         filer_merkle_light,
//         indexes: Default::default(),
//     };
//     Ok(ledger)
// }
//
// /// 新建视图
// ///
// /// 具体传参参考如下定义：<p><p>
// ///
// /// ###Params
// ///
// /// mem 是否为内存视图
// fn mock_new_ledger(database_name: String, name: String) -> GeorgeResult<Ledger> {
//     let now: NaiveDateTime = Local::now().naive_local();
//     let create_time = Duration::nanoseconds(now.timestamp_nanos());
//     let filepath = Paths::ledger_filepath(database_name.clone(), name.clone());
//     let filepath_light = Paths::ledger_light_filepath(database_name.clone(), name.clone());
//     let filepath_merkle_light =
//         Paths::ledger_merkle_light_filepath(database_name.clone(), name.clone());
//     let filer = Filed::create(filepath.clone())?;
//     let filer_light = Filed::create(filepath_light.clone())?;
//     let filer_merkle_light = Filed::create(filepath_merkle_light.clone())?;
//     let metadata = Metadata::ledger();
//     let ledger = Ledger {
//         database_name,
//         name,
//         create_time,
//         metadata,
//         filepath,
//         filepath_light,
//         filepath_merkle_light,
//         filer,
//         filer_light,
//         filer_merkle_light,
//         indexes: Default::default(),
//     };
//     Ok(ledger)
// }
//
// impl Ledger {
//     pub(crate) fn create(database_name: String, name: String) -> GeorgeResult<Arc<RwLock<Ledger>>> {
//         let ledger_new = new_ledger(database_name, name)?;
//         let ledger = Arc::new(RwLock::new(ledger_new));
//         ledger.clone().read().unwrap().init()?;
//         // 区块世界状态存储索引
//         ledger.read().unwrap().create_index(
//             ledger.clone(),
//             INDEX_DISK.to_string(),
//             IndexType::Disk,
//             KeyType::String,
//             true,
//             true,
//             false,
//         )?;
//         // 区块高度存储索引，根据块高查询区块
//         ledger.read().unwrap().create_index(
//             ledger.clone(),
//             INDEX_BLOCK_HEIGHT.to_string(),
//             IndexType::Sequence,
//             KeyType::UInt,
//             false,
//             true,
//             false,
//         )?;
//         // 区块hash存储索引，根据块hash查询区块
//         ledger.read().unwrap().create_index(
//             ledger.clone(),
//             INDEX_BLOCK_HASH.to_string(),
//             IndexType::Disk,
//             KeyType::String,
//             true,
//             true,
//             false,
//         )?;
//         // 交易hash存储索引，根据交易hash查询区块、查询交易
//         ledger.read().unwrap().create_index(
//             ledger.clone(),
//             INDEX_TX_HASH.to_string(),
//             IndexType::Disk,
//             KeyType::String,
//             true,
//             true,
//             false,
//         )?;
//         // todo 补充溯源索引策略
//         Ok(ledger)
//     }
//
//     fn init(&self) -> GeorgeResult<()> {
//         let mut metadata_bytes = self.metadata_bytes();
//         let mut description = self.description();
//         // 初始化为32 + 8，即head长度加正文描述符长度
//         let mut before_description = ContentBytes::before(44, description.len() as u32);
//         metadata_bytes.append(&mut before_description);
//         metadata_bytes.append(&mut description);
//         self.append(metadata_bytes)?;
//         Ok(())
//     }
//
//     /// 创建时间
//     fn create_time(&self) -> Duration {
//         self.create_time.clone()
//     }
//
//     /// 文件信息
//     fn metadata(&self) -> Metadata {
//         self.metadata.clone()
//     }
//
//     fn filepath(&self) -> String {
//         self.filepath.clone()
//     }
//
//     fn filepath_light(&self) -> String {
//         self.filepath_light.clone()
//     }
//
//     fn filepath_merkle_light(&self) -> String {
//         self.filepath_merkle_light.clone()
//     }
//
//     /// 索引集合
//     fn index_map(&self) -> Arc<RwLock<HashMap<String, Arc<dyn TIndex>>>> {
//         self.indexes.clone()
//     }
//
//     /// 获取索引
//     fn index(&self, index_name: &str) -> GeorgeResult<Arc<dyn TIndex>> {
//         match self.index_map().read().unwrap().get(index_name) {
//             Some(idx) => Ok(idx.clone()),
//             None => Err(Errs::string(format!("index {} doesn't found", index_name))),
//         }
//     }
//
//     /// 文件字节信息
//     fn metadata_bytes(&self) -> Vec<u8> {
//         self.metadata.bytes()
//     }
//
//     /// 创建索引
//     ///
//     /// ###Params
//     /// * ledger 视图
//     /// * index_name 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入
//     /// * index_type 存储引擎类型
//     /// * key_type 索引值类型
//     /// * primary 是否主键，主键也是唯一索引，即默认列表依赖索引
//     /// * unique 是否唯一索引
//     /// * null 是否允许为空
//     fn create_index(
//         &self,
//         ledger: Arc<RwLock<Ledger>>,
//         index_name: String,
//         index_type: IndexType,
//         key_type: KeyType,
//         primary: bool,
//         unique: bool,
//         null: bool,
//     ) -> GeorgeResult<()> {
//         if self.exist_index(index_name.clone()) {
//             return Err(GeorgeError::from(IndexExistError));
//         }
//         self.index_map().write().unwrap().insert(
//             index_name.clone(),
//             IndexDefault::create(
//                 ledger, index_name, index_type, primary, unique, null, key_type,
//             )?,
//         );
//         Ok(())
//     }
//
//     /// 根据文件路径获取该文件追加写入的写对象
//     ///
//     /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
//     ///
//     /// #Return
//     ///
//     /// seek_end_before 写之前文件字节数据长度
//     fn append(&self, content: Vec<u8>) -> GeorgeResult<u64> {
//         self.filer.append(content)
//     }
//
//     fn read(&self, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
//         self.filer.read(start, last)
//     }
//
//     fn write(&self, seek: u64, content: Vec<u8>) -> GeorgeResult<()> {
//         self.filer.write(seek, content)
//     }
// }
//
// impl TForm for Ledger {
//     fn name(&self) -> String {
//         self.name.clone()
//     }
//
//     fn database_name(&self) -> String {
//         self.database_name.clone()
//     }
//
//     fn write_content(&self, value: Vec<u8>) -> GeorgeResult<Vec<u8>> {
//         // 内容持续长度(4字节)
//         let mut ledger_info_index = Trans::u32_2_bytes(value.len() as u32);
//         // 将数据存入ledger，返回数据在ledger中的起始坐标
//         let ledger_seek_start = self.append(value)?;
//         // 记录视图文件属性(版本号/数据归档/定位文件用2字节)+数据在表文件中起始偏移量p(6字节)
//         // 数据在视图文件中起始偏移量p(6字节)
//         let mut ledger_seek_start_bytes = Trans::u48_2_bytes(ledger_seek_start);
//         // 记录表文件属性(版本/数据归档/定位文件用2字节)+数据持续长度+数据在表文件中起始偏移量p(6字节)
//         // ledger_info_index = ledger持续长度(4字节) + ledger偏移量(6字节)
//         ledger_info_index.append(&mut ledger_seek_start_bytes);
//         Ok(ledger_info_index)
//     }
//
//     fn read_content(&self, version: u16, data_len: u32, seek: u64) -> GeorgeResult<Vec<u8>> {
//         Filer::read_sub(filepath, seek, data_len as usize)
//     }
//
//     fn read_content_by_info(&self, ledger_info_index: Vec<u8>) -> GeorgeResult<Vec<u8>> {
//         // 读取ledger持续长度(4字节)
//         let data_len = Trans::bytes_2_u32(Vector::sub(ledger_info_index.clone(), 2, 6)?)?;
//         // 读取ledger偏移量(6字节)
//         let seek = Trans::bytes_2_u48(Vector::sub(ledger_info_index.clone(), 6, 12)?)?;
//         Filer::read_sub(self.filepath(), seek, data_len as usize)
//     }
//
//     /// 检查值有效性
//     fn check(
//         &self,
//         conditions: Vec<Condition>,
//         delete: bool,
//         ledger_info_index: Vec<u8>,
//     ) -> GeorgeResult<(bool, Vec<u8>)> {
//         if Vector::is_empty(ledger_info_index.clone()) {
//             Ok((false, vec![]))
//         } else {
//             let real = DataReal::from(self.read_content_by_info(ledger_info_index)?)?;
//             let value_bytes = real.value();
//             if Condition::validate(conditions.clone(), value_bytes.clone()) {
//                 if delete {
//                     self.remove(real.key(), real.value())?;
//                 }
//                 Ok((true, value_bytes))
//             } else {
//                 Ok((false, vec![]))
//             }
//         }
//     }
// }
//
// /// db for disk
// impl Ledger {
//     /// 插入数据，如果存在则返回已存在<p><p>
//     ///
//     /// ###Params
//     ///
//     /// key string
//     ///
//     /// value 当前结果value信息<p><p>
//     ///
//     /// ###Return
//     ///
//     /// IndexResult<()>
//     pub(crate) fn put(&self, key: String, value: Vec<u8>) -> GeorgeResult<()> {
//         self.save(key, value, false)
//     }
//
//     /// 插入数据，无论存在与否都会插入或更新数据<p><p>
//     ///
//     /// ###Params
//     ///
//     /// key string
//     ///
//     /// value 当前结果value信息<p><p>
//     ///
//     /// ###Return
//     ///
//     /// IndexResult<()>
//     pub(crate) fn set(&self, key: String, value: Vec<u8>) -> GeorgeResult<()> {
//         self.save(key, value, true)
//     }
//
//     /// 获取数据，返回存储对象<p><p>
//     ///
//     /// ###Params
//     ///
//     /// index_name 索引名称
//     ///
//     /// key string
//     ///
//     /// ###Return
//     ///
//     /// Seed value信息
//     pub(crate) fn get(&self, index_name: &str, key: String) -> GeorgeResult<Vec<u8>> {
//         let index = self.index(index_name)?;
//         Ok(index.get(key.clone())?.value())
//     }
//
//     /// 删除数据<p><p>
//     ///
//     /// ###Params
//     ///
//     /// key string<p><p>
//     ///
//     /// ###Return
//     ///
//     /// GeorgeResult<()>
//     pub(crate) fn remove(&self, key: String, value: Vec<u8>) -> GeorgeResult<()> {
//         let real = self.index(INDEX_DISK)?.get(key.clone())?;
//         self.del(key, real.increment, value)
//     }
//
//     /// 条件检索
//     ///
//     /// selector_json_bytes 选择器字节数组，自定义转换策略
//     pub(crate) fn select(&self, constraint_json_bytes: Vec<u8>) -> GeorgeResult<Expectation> {
//         Selector::run(constraint_json_bytes, self.indexes.clone(), false)
//     }
//
//     /// 条件删除
//     ///
//     /// selector_json_bytes 选择器字节数组，自定义转换策略
//     pub(crate) fn delete(&self, constraint_json_bytes: Vec<u8>) -> GeorgeResult<Expectation> {
//         Selector::run(constraint_json_bytes, self.indexes.clone(), true)
//     }
// }
//
// impl Ledger {
//     /// 插入数据业务方法<p><p>
//     ///
//     /// ###Params
//     ///
//     /// key string
//     ///
//     /// value 当前结果value信息<p><p>
//     ///
//     /// force 如果存在原值，是否覆盖原结果<p><p>
//     ///
//     /// ###Return
//     ///
//     /// IndexResult<()>
//     fn save(&self, key: String, value: Vec<u8>, force: bool) -> GeorgeResult<()> {
//         let seed = Seed::create(Arc::new(self.clone()), key.clone(), value.clone());
//         let mut receives = Vec::new();
//         for (index_name, index) in self.index_map().read().unwrap().iter() {
//             let (sender, receive) = mpsc::channel();
//             receives.push(receive);
//             let index_name_clone = index_name.clone();
//             let index_clone = index.clone();
//             let key_clone = key.clone();
//             let value_clone = value.clone();
//             let seed_clone = seed.clone();
//             thread::spawn(move || match index_name_clone.as_str() {
//                 INDEX_DISK => sender.send(index_clone.put(key_clone, seed_clone, force)),
//                 INDEX_BLOCK_HASH => sender.send(index_clone.put(key_clone, seed_clone, force)),
//                 INDEX_BLOCK_HEIGHT => sender.send(index_clone.put(key_clone, seed_clone, force)),
//                 INDEX_TX_HASH => sender.send(index_clone.put(key_clone, seed_clone, force)),
//                 _ => match IndexKey::fetch(index_name_clone, value_clone) {
//                     Ok(res) => sender.send(index_clone.put(res, seed_clone, force)),
//                     Err(err) => {
//                         log::debug!("key fetch error: {}", err);
//                         sender.send(Ok(()))
//                     }
//                 },
//             });
//         }
//         for receive in receives.iter() {
//             let res = receive.recv();
//             match res {
//                 Ok(gr) => match gr {
//                     Err(err) => return Err(err),
//                     _ => {}
//                 },
//                 Err(err) => return Err(Errs::string(err.to_string())),
//             }
//         }
//         let seed_w = seed.write().unwrap();
//         seed_w.save()
//     }
//
//     /// 插入数据业务方法<p><p>
//     ///
//     /// ###Params
//     ///
//     /// key string
//     ///
//     /// value 当前结果value信息<p><p>
//     ///
//     /// force 如果存在原值，是否覆盖原结果<p><p>
//     ///
//     /// ###Return
//     ///
//     /// IndexResult<()>
//     fn del(&self, key: String, increment: u64, value: Vec<u8>) -> GeorgeResult<()> {
//         let seed = Seed::create_cus(Arc::new(self.clone()), key.clone(), increment, value.clone());
//         let mut receives = Vec::new();
//         for (index_name, index) in self.index_map().read().unwrap().iter() {
//             let (sender, receive) = mpsc::channel();
//             receives.push(receive);
//             let index_name_clone = index_name.clone();
//             let index_clone = index.clone();
//             let key_clone = key.clone();
//             let value_clone = value.clone();
//             let seed_clone = seed.clone();
//             thread::spawn(move || {
//                 log::debug!("thread del index {}", index_name_clone);
//                 match index_name_clone.as_str() {
//                     INDEX_DISK => sender.send(index_clone.del(key_clone, seed_clone)),
//                     INDEX_INCREMENT => sender.send(index_clone.del(key_clone, seed_clone)),
//                     _ => match IndexKey::fetch(index_name_clone, value_clone) {
//                         Ok(res) => sender.send(index_clone.del(res, seed_clone)),
//                         Err(err) => {
//                             log::debug!("key fetch error: {}", err);
//                             sender.send(Ok(()))
//                         }
//                     },
//                 }
//             });
//         }
//         for receive in receives.iter() {
//             let res = receive.recv();
//             match res {
//                 Ok(gr) => match gr {
//                     Err(err) => return Err(err),
//                     _ => {}
//                 },
//                 Err(err) => return Err(Errs::string(err.to_string())),
//             }
//         }
//         let seed_w = seed.write().unwrap();
//         seed_w.remove()
//     }
// }
//
// impl Ledger {
//     /// 生成文件描述
//     fn description(&self) -> Vec<u8> {
//         hex::encode(format!(
//             "{}:#?{}:#?{}",
//             self.name(),
//             self.create_time().num_nanoseconds().unwrap().to_string(),
//             self.pigeonhole().to_string()
//         ))
//         .into_bytes()
//     }
//
//     /// 通过文件描述恢复结构信息
//     pub(crate) fn recover(
//         database_name: String,
//         hd: HD,
//     ) -> GeorgeResult<(String, Arc<RwLock<Ledger>>)> {
//         let description_str = Strings::from_utf8(hd.description())?;
//         match hex::decode(description_str) {
//             Ok(vu8) => {
//                 let real = Strings::from_utf8(vu8)?;
//                 let mut split = real.split(":#?");
//                 let name = split.next().unwrap().to_string();
//                 let create_time = Duration::nanoseconds(
//                     split.next().unwrap().to_string().parse::<i64>().unwrap(),
//                 );
//                 let pigeonhole = Pigeonhole::from_string(split.next().unwrap().to_string())?;
//                 let filepath = Paths::ledger_filepath(database_name.clone(), name.clone());
//                 let ledger = Ledger {
//                     database_name: database_name.clone(),
//                     name,
//                     create_time,
//                     metadata: hd.metadata(),
//                     filer: Filed::recovery(filepath)?,
//                     indexes: Arc::new(Default::default()),
//                     pigeonhole,
//                 };
//                 log::info!(
//                     "recovery ledger {} from database {}",
//                     ledger.name(),
//                     database_name,
//                 );
//                 let ledger_bak = Arc::new(RwLock::new(ledger.clone()));
//                 match read_dir(Paths::ledger_path(database_name, ledger.name())) {
//                     // 恢复indexes数据
//                     Ok(paths) => {
//                         ledger_bak
//                             .read()
//                             .unwrap()
//                             .recovery_indexes(ledger_bak.clone(), paths)?;
//                         log::debug!(
//                             "ledger [db={}, name={}, create_time={}, pigeonhole={:#?}, {:#?}]",
//                             ledger.name(),
//                             ledger.name(),
//                             ledger.create_time().num_nanoseconds().unwrap().to_string(),
//                             ledger.pigeonhole(),
//                             hd.metadata()
//                         );
//                         Ok((ledger.name(), ledger_bak))
//                     }
//                     Err(err) => Err(Errs::strs("recovery ledger read dir", err)),
//                 }
//             }
//             Err(err) => Err(Errs::strs("recovery ledger decode", err)),
//         }
//     }
//
//     /// 恢复indexes数据
//     fn recovery_indexes(&self, ledger: Arc<RwLock<Ledger>>, paths: ReadDir) -> GeorgeResult<()> {
//         // 遍历ledger目录下文件
//         for path in paths {
//             match path {
//                 // 所有目录文件被默认为index根目录
//                 Ok(dir) => {
//                     if dir.path().is_dir() {
//                         let index_name = dir.file_name().to_str().unwrap().to_string();
//                         log::debug!("recovery index from {}", index_name);
//                         // 恢复index数据
//                         self.recovery_index(ledger.clone(), index_name.clone())?;
//                     }
//                 }
//                 Err(err) => return Err(Errs::strs("recovery indexes path", err)),
//             }
//         }
//         Ok(())
//     }
//
//     /// 恢复ledger数据
//     fn recovery_index(&self, ledger: Arc<RwLock<Ledger>>, index_name: String) -> GeorgeResult<()> {
//         let index_file_path =
//             Paths::index_filepath(self.database_name(), self.name(), index_name.clone());
//         let hd = ContentBytes::recovery(index_file_path.clone())?;
//         let metadata = hd.metadata();
//         let index;
//         // 恢复index数据
//         match hd.index_type() {
//             IndexType::None => return Err(Errs::str("index engine type error")),
//             _ => index = IndexDefault::recover(ledger, hd)?,
//         }
//         log::debug!(
//             "index [db={}, ledger={}, name={}, create_time={}, {:#?}]",
//             self.database_name(),
//             self.name(),
//             index_name.clone(),
//             index.create_time().num_nanoseconds().unwrap().to_string(),
//             metadata
//         );
//         // 如果已存在该ledger，则不处理
//         if !self.exist_index(index_name.clone()) {
//             self.index_map().write().unwrap().insert(index_name, index);
//         }
//         Ok(())
//     }
// }
//
// impl Ledger {
//     pub(crate) fn mock_create(
//         database_name: String,
//         name: String,
//     ) -> GeorgeResult<Arc<RwLock<Ledger>>> {
//         let ledger = mock_new_ledger(database_name, name)?;
//         let ledger_bak = Arc::new(RwLock::new(ledger));
//         ledger_bak.clone().read().unwrap().init()?;
//         Ok(ledger_bak)
//     }
//
//     pub(crate) fn mock_create_single(database_name: String, name: String) -> GeorgeResult<Ledger> {
//         let ledger = mock_new_ledger(database_name, name)?;
//         ledger.init()?;
//         Ok(ledger)
//     }
// }
