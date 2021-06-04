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
// use std::sync::{Arc, RwLock};
//
// use chrono::{Duration, Local, NaiveDateTime};
// use tokio::sync::mpsc::Sender;
//
// use comm::errors::{Errs, GeorgeResult};
// use comm::io::file::FilerReader;
// use comm::io::Filer;
// use comm::strings::StringHandler;
// use comm::vectors::VectorHandler;
// use comm::Strings;
// use comm::Trans;
// use comm::Vector;
// use protocols::impls::chain::block::Block;
//
// use crate::task::engine::traits::{Pigeonhole, TIndex, TSeed};
// use crate::task::engine::DataReal;
// use crate::task::rich::{Condition, Expectation, Selector};
// use crate::task::Ledger;
// use crate::task::Seed;
// use crate::task::{Index as IndexDefault, GLOBAL_THREAD_POOL};
// use crate::utils::comm::{
//     IndexKey, INDEX_BLOCK_HASH, INDEX_BLOCK_HEIGHT, INDEX_DISK, INDEX_INCREMENT, INDEX_TX_HASH,
// };
// use crate::utils::enums::{IndexType, KeyType};
// use crate::utils::store::{ContentBytes, Metadata, HD};
// use crate::utils::writer::Filed;
// use crate::utils::Paths;
//
// /// 新建视图
// fn new_ledger(database_name: String, name: String) -> GeorgeResult<Ledger> {
//     let now: NaiveDateTime = Local::now().naive_local();
//     let create_time = Duration::nanoseconds(now.timestamp_nanos());
//     let filepath = Paths::ledger_filepath(database_name.clone(), name.clone());
//     let filepath_light = Paths::ledger_light_filepath(database_name.clone(), name.clone());
//     let filepath_merkle_light =
//         Paths::ledger_merkle_light_filepath(database_name.clone(), name.clone());
//     let metadata = Metadata::ledger();
//     Ok(Ledger {
//         database_name,
//         name,
//         create_time,
//         metadata,
//         filer: Filed::create(filepath)?,
//         filer_light: Filed::create(filepath_light)?,
//         filer_merkle_light: Filed::create(filepath_merkle_light)?,
//         index_block_height: Arc::new(()),
//         index_block_hash: Arc::new(()),
//         index_tx_hash: Arc::new(()),
//         indexes: Default::default(),
//     })
// }
//
// /// 新建视图
// fn mock_new_ledger(database_name: String, name: String) -> GeorgeResult<Ledger> {
//     let now: NaiveDateTime = Local::now().naive_local();
//     let create_time = Duration::nanoseconds(now.timestamp_nanos());
//     let filepath = Paths::ledger_filepath(database_name.clone(), name.clone());
//     let filepath_light = Paths::ledger_light_filepath(database_name.clone(), name.clone());
//     let filepath_merkle_light =
//         Paths::ledger_merkle_light_filepath(database_name.clone(), name.clone());
//     let metadata = Metadata::ledger();
//     Ok(Ledger {
//         database_name,
//         name,
//         create_time,
//         metadata,
//         filer: Filed::create(filepath)?,
//         filer_light: Filed::create(filepath_light)?,
//         filer_merkle_light: Filed::create(filepath_merkle_light)?,
//         index_block_height: Arc::new(()),
//         index_block_hash: Arc::new(()),
//         index_tx_hash: Arc::new(()),
//         indexes: Default::default(),
//     })
// }
//
// impl Ledger {
//     pub(crate) fn create(database_name: String, name: String) -> GeorgeResult<Arc<RwLock<Ledger>>> {
//         let ledger_new = new_ledger(database_name, name)?;
//         let ledger = Arc::new(RwLock::new(ledger_new));
//         ledger.clone().read().unwrap().init()?;
//         let mut ledger_w = ledger.write().unwrap();
//         ledger_w.set_index_block_height(IndexDefault::create(
//             ledger.clone(),
//             INDEX_BLOCK_HEIGHT.to_string(),
//             IndexType::Sequence,
//             false,
//             true,
//             false,
//             KeyType::UInt,
//         )?);
//         ledger_w.set_index_block_hash(IndexDefault::create(
//             ledger.clone(),
//             INDEX_BLOCK_HASH.to_string(),
//             IndexType::Disk,
//             false,
//             true,
//             false,
//             KeyType::String,
//         )?);
//         ledger_w.set_index_tx_hash(IndexDefault::create(
//             ledger.clone(),
//             INDEX_TX_HASH.to_string(),
//             IndexType::Disk,
//             false,
//             true,
//             false,
//             KeyType::String,
//         )?);
//         // 区块世界状态存储索引
//         ledger_w.create_index(
//             ledger.clone(),
//             INDEX_DISK.to_string(),
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
//     fn set_index_block_height(&mut self, index_block_height: Arc<dyn TIndex>) {
//         self.index_block_height = index_block_height
//     }
//
//     fn set_index_block_hash(&mut self, index_block_hash: Arc<dyn TIndex>) {
//         self.index_block_hash = index_block_hash
//     }
//
//     fn set_index_tx_hash(&mut self, index_tx_hash: Arc<dyn TIndex>) {
//         self.index_tx_hash = index_tx_hash
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
//             return Err(Errs::index_exist_error());
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
//     /// 追加写入的写对象
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
//     /// 读取`start`起始且持续`last`长度的数据
//     fn read(&self, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
//         self.filer.read(start, last)
//     }
//
//     /// 写入的写对象到指定坐标
//     ///
//     /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
//     fn write(&self, seek: u64, content: Vec<u8>) -> GeorgeResult<()> {
//         self.filer.write(seek, content)
//     }
//
//     /// 追加写入的写对象
//     ///
//     /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
//     ///
//     /// #Return
//     ///
//     /// seek_end_before 写之前文件字节数据长度
//     fn append_light(&self, content: Vec<u8>) -> GeorgeResult<u64> {
//         self.filer_light.append(content)
//     }
//
//     /// 读取`start`起始且持续`last`长度的数据
//     fn read_light(&self, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
//         self.filer_light.read(start, last)
//     }
//
//     /// 写入的写对象到指定坐标
//     ///
//     /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
//     fn write_light(&self, seek: u64, content: Vec<u8>) -> GeorgeResult<()> {
//         self.filer_light.write(seek, content)
//     }
//
//     /// 追加写入的写对象
//     ///
//     /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
//     ///
//     /// #Return
//     ///
//     /// seek_end_before 写之前文件字节数据长度
//     fn append_merkle_light(&self, content: Vec<u8>) -> GeorgeResult<u64> {
//         self.filer_merkle_light.append(content)
//     }
//
//     /// 读取`start`起始且持续`last`长度的数据
//     fn read_merkle_light(&self, start: u64, last: usize) -> GeorgeResult<Vec<u8>> {
//         self.filer_merkle_light.read(start, last)
//     }
//
//     /// 写入的写对象到指定坐标
//     ///
//     /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
//     fn write_merkle_light(&self, seek: u64, content: Vec<u8>) -> GeorgeResult<()> {
//         self.filer_merkle_light.write(seek, content)
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
//     fn read_content(&self, _version: u16, data_len: u32, seek: u64) -> GeorgeResult<Vec<u8>> {
//         Filer::read_sub(filepath, seek, data_len as usize)
//     }
//
//     fn read_content_by_info(&self, ledger_info_index: Vec<u8>) -> GeorgeResult<Vec<u8>> {
//         // 读取ledger持续长度(4字节)
//         let data_len = Trans::bytes_2_u32(Vector::sub(ledger_info_index.clone(), 2, 6)?)?;
//         // 读取ledger偏移量(6字节)
//         let seek = Trans::bytes_2_u48(Vector::sub(ledger_info_index.clone(), 6, 12)?)?;
//         Filer::read_sub(self.filer.filepath(), seek, data_len as usize)
//     }
//
//     fn rm(&self, key: String, value: Vec<u8>) -> GeorgeResult<()> {
//         self.remove(key, value)
//     }
// }
//
// /// db for block
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
//     pub(crate) fn put(&self, block: Block) -> GeorgeResult<()> {
//         GLOBAL_THREAD_POOL.task_block_on(self.save(key, value, false))
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
//         GLOBAL_THREAD_POOL.task_block_on(self.save(key, value, false))
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
//         GLOBAL_THREAD_POOL.task_block_on(self.save(key, value, true))
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
//         GLOBAL_THREAD_POOL.task_block_on(self.del(key, real.increment, value))
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
//     async fn save(&self, key: String, value: Vec<u8>, force: bool) -> GeorgeResult<()> {
//         let seed = Seed::create(Arc::new(self.clone()), key.clone(), value.clone());
//         let mut receives = Vec::new();
//         for (index_name, index) in self.index_map().read().unwrap().iter() {
//             let (sender, receive) = tokio::sync::mpsc::channel(32);
//             receives.push(receive);
//             GLOBAL_THREAD_POOL.spawn(self.clone().index_put_exec(
//                 index_name.clone(),
//                 index.clone(),
//                 key.clone(),
//                 value.clone(),
//                 seed.clone(),
//                 force,
//                 sender,
//             ));
//         }
//         for receive in receives.iter_mut() {
//             let message = receive.recv().await;
//             match message {
//                 Some(res) => match res {
//                     Err(err) => return Err(err),
//                     _ => {}
//                 },
//                 _ => {}
//             }
//         }
//         let seed_w = seed.write().unwrap();
//         seed_w.save()
//     }
//
//     async fn index_put_exec(
//         self,
//         index_name: String,
//         index: Arc<dyn TIndex>,
//         key: String,
//         value: Vec<u8>,
//         seed: Arc<RwLock<Seed>>,
//         force: bool,
//         sender: Sender<GeorgeResult<()>>,
//     ) {
//         match index_name.as_str() {
//             INDEX_DISK => {
//                 self.send_put(index_name, index, key, seed, force, sender)
//                     .await
//             }
//             INDEX_INCREMENT => {
//                 self.send_put(index_name, index, key, seed, force, sender)
//                     .await
//             }
//             _ => match IndexKey::fetch(index_name.clone(), value) {
//                 Ok(res) => {
//                     self.send_put(index_name, index, res, seed, force, sender)
//                         .await
//                 }
//                 Err(err) => {
//                     log::warn!("key fetch error: {}", err);
//                     match sender.send(Ok(())).await {
//                         Err(err) => {
//                             log::error!(
//                                 "sender send put error in database {} ledger {} index {} while exec key {} {}",
//                                 self.database_name(),
//                                 self.name(),
//                                 index_name,
//                                 key,
//                                 err
//                             );
//                         }
//                         _ => {}
//                     }
//                 }
//             },
//         }
//     }
//
//     async fn send_put(
//         self,
//         index_name: String,
//         index: Arc<dyn TIndex>,
//         key: String,
//         seed: Arc<RwLock<Seed>>,
//         force: bool,
//         sender: Sender<GeorgeResult<()>>,
//     ) {
//         match sender.send(index.put(key.clone(), seed, force)).await {
//             Err(err) => {
//                 log::error!(
//                     "sender send put error in database {} ledger {} index {} while exec key {} {}",
//                     self.database_name(),
//                     self.name(),
//                     index_name,
//                     key,
//                     err
//                 );
//             }
//             _ => {}
//         }
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
//     async fn del(&self, key: String, increment: u64, value: Vec<u8>) -> GeorgeResult<()> {
//         let seed = Seed::create_cus(
//             Arc::new(self.clone()),
//             key.clone(),
//             increment,
//             value.clone(),
//         );
//         let mut receives = Vec::new();
//         for (index_name, index) in self.index_map().read().unwrap().iter() {
//             let (sender, receive) = tokio::sync::mpsc::channel(32);
//             receives.push(receive);
//             GLOBAL_THREAD_POOL.spawn(self.clone().index_del_exec(
//                 index_name.clone(),
//                 index.clone(),
//                 key.clone(),
//                 value.clone(),
//                 seed.clone(),
//                 sender,
//             ));
//         }
//         for receive in receives.iter_mut() {
//             let message = receive.recv().await;
//             match message {
//                 Some(res) => match res {
//                     Err(err) => return Err(err),
//                     _ => {}
//                 },
//                 _ => {}
//             }
//         }
//         let seed_w = seed.write().unwrap();
//         seed_w.remove()
//     }
//
//     async fn index_del_exec(
//         self,
//         index_name: String,
//         index: Arc<dyn TIndex>,
//         key: String,
//         value: Vec<u8>,
//         seed: Arc<RwLock<Seed>>,
//         sender: Sender<GeorgeResult<()>>,
//     ) {
//         match index_name.as_str() {
//             INDEX_DISK => self.send_del(index_name, index, key, seed, sender).await,
//             INDEX_INCREMENT => self.send_del(index_name, index, key, seed, sender).await,
//             _ => match IndexKey::fetch(index_name.clone(), value) {
//                 Ok(res) => self.send_del(index_name, index, res, seed, sender).await,
//                 Err(err) => {
//                     log::warn!("key fetch error: {}", err);
//                     match sender.send(Ok(())).await {
//                         Err(err) => {
//                             log::error!(
//                                 "sender send del error in database {} ledger {} index {} while exec key {} {}",
//                                 self.database_name(),
//                                 self.name(),
//                                 index_name,
//                                 key,
//                                 err
//                             );
//                         }
//                         _ => {}
//                     }
//                 }
//             },
//         }
//     }
//
//     async fn send_del(
//         self,
//         index_name: String,
//         index: Arc<dyn TIndex>,
//         key: String,
//         seed: Arc<RwLock<Seed>>,
//         sender: Sender<GeorgeResult<()>>,
//     ) {
//         match sender.send(index.del(key.clone(), seed)).await {
//             Err(err) => {
//                 log::error!(
//                     "sender send del error in database {} ledger {} index {} while exec key {} {}",
//                     self.database_name(),
//                     self.name(),
//                     index_name,
//                     key,
//                     err
//                 );
//             }
//             _ => {}
//         }
//     }
// }
//
// impl Ledger {
//     /// 生成文件描述
//     fn description(&self) -> Vec<u8> {
//         hex::encode(format!(
//             "{}:#?{}",
//             self.name(),
//             self.create_time().num_nanoseconds().unwrap().to_string(),
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
//                 let filepath = Paths::ledger_filepath(database_name.clone(), name.clone());
//                 let filepath_light =
//                     Paths::ledger_light_filepath(database_name.clone(), name.clone());
//                 let filepath_merkle_light =
//                     Paths::ledger_merkle_light_filepath(database_name.clone(), name.clone());
//                 let ledger_new = Ledger {
//                     database_name: database_name.clone(),
//                     name,
//                     create_time,
//                     metadata: hd.metadata(),
//                     filer: Filed::recovery(filepath)?,
//                     filer_light: Filed::recovery(filepath_light)?,
//                     filer_merkle_light: Filed::recovery(filepath_merkle_light)?,
//                     index_block_height: Arc::new(()),
//                     index_block_hash: Arc::new(()),
//                     index_tx_hash: Arc::new(()),
//                     indexes: Arc::new(Default::default()),
//                 };
//                 log::info!(
//                     "recovery ledger {} from database {}",
//                     ledger_new.name(),
//                     database_name,
//                 );
//                 let ledger = Arc::new(RwLock::new(ledger_new.clone()));
//                 match read_dir(Paths::ledger_path(database_name, ledger_new.name())) {
//                     // 恢复indexes数据
//                     Ok(paths) => {
//                         // todo 先恢复默认索引
//                         ledger
//                             .read()
//                             .unwrap()
//                             .recovery_indexes(ledger.clone(), paths)?;
//                         log::debug!(
//                             "ledger [db={}, name={}, create_time={}, {:#?}]",
//                             ledger_new.database_name(),
//                             ledger_new.name(),
//                             ledger_new
//                                 .create_time()
//                                 .num_nanoseconds()
//                                 .unwrap()
//                                 .to_string(),
//                             hd.metadata()
//                         );
//                         Ok((ledger_new.name(), ledger))
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
//         // 遍历ledger目录下文件 todo 过滤默认索引
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
