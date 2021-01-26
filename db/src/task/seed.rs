/*
 * Copyright (c) 2021. Aberic - All Rights Reserved.
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

use comm::errors::entrances::{err_string, err_strs, GeorgeError, GeorgeResult};
use comm::trans::{trans_u16_2_bytes, trans_u32_2_bytes, trans_u64_2_bytes};
use serde::{Deserialize, Serialize};

use crate::task::engine::traits::TSeed;
use crate::task::view::{Pigeonhole, View};
use crate::utils::comm::{is_bytes_fill, VALUE_TYPE_NORMAL};
use comm::errors::children::DataExistError;
use comm::io::file::{Filer, FilerExecutor, FilerHandler, FilerNormal, FilerWriter};
use comm::vectors::{Vector, VectorHandler};
use std::fs::{File, OpenOptions};

/// B+Tree索引叶子结点内防hash碰撞数组结构中单体结构
///
/// 搭配Index使用
///
/// 叶子节点下真实存储数据的集合单体结构
#[derive(Debug)]
pub(crate) struct Seed {
    /// 获取当前结果原始key信息
    key: String,
    /// 索引操作策略集合
    policies: Vec<IndexPolicy>,
}

/// 待处理索引操作策略
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct IndexPolicy {
    /// 待处理索引文件路径
    index_file_path: String,
    /// 待写入索引内容起始偏移量
    seek: u64,
}

impl IndexPolicy {
    fn from(v8s: Vec<u8>) -> GeorgeResult<IndexPolicy> {
        match serde_json::from_slice(v8s.as_slice()) {
            Ok(t) => Ok(t),
            Err(err) => Err(err_string(err.to_string())),
        }
    }
    pub fn bytes(index_file_path: String, seek: u64) -> GeorgeResult<Vec<u8>> {
        let policy = IndexPolicy {
            index_file_path,
            seek,
        };
        match serde_json::to_vec(&policy) {
            Ok(v8s) => Ok(v8s),
            Err(err) => Err(err_string(err.to_string())),
        }
    }
    fn index_file_path(&self) -> String {
        self.index_file_path.clone()
    }
    /// 执行索引落库操作
    ///
    /// view 视图对象
    ///
    /// view_version_bytes 视图文件属性，版本号(2字节)
    ///
    /// view_seek_start_bytes 视图内容在视图文件中的起始偏移量(8字节)
    fn exec(
        &self,
        db_name: String,
        key: String,
        view: View,
        version_bytes: Vec<u8>,
        seek_start_bytes: Vec<u8>,
        force: bool,
    ) -> GeorgeResult<()> {
        Filer::try_touch(self.index_file_path())?;
        match OpenOptions::new()
            .write(true)
            .read(true)
            .open(self.index_file_path())
        {
            Ok(file) => {
                let check = Filer::read_subs(file.try_clone().unwrap(), self.seek, 8)?;
                // 如果读取到不为空，则表明该数据已经存在
                if is_bytes_fill(check) {
                    // todo 先读取视图数据，比对是否为碰撞数据

                    if force {}
                    Err(GeorgeError::DataExistError(DataExistError))
                } else {
                    match file.try_clone() {
                        // 如果读取到为空，则表明该数据为首次插入
                        Ok(file) => {
                            self.record(file, db_name, key, view, version_bytes, seek_start_bytes)
                        }
                        Err(err) => Err(err_strs("seed exec file try clone", err)),
                    }
                }
            }
            Err(err) => Err(err_strs("seed file open when write seek", err)),
        }
    }
    /// 执行索引落库操作
    ///
    /// view 视图对象
    ///
    /// view_version_bytes 视图文件属性，版本号(2字节)
    ///
    /// view_seek_start_bytes 视图内容在视图文件中的起始偏移量(8字节)
    fn record(
        &self,
        file: File,
        db_name: String,
        key: String,
        view: View,
        mut version_bytes: Vec<u8>,
        mut seek_start_bytes: Vec<u8>,
    ) -> GeorgeResult<()> {
        // 首次插入数据类型为正常数据类型
        let mut value_type_bytes = vec![VALUE_TYPE_NORMAL];
        // 生成表内容索引(8字节)+原始key长度+原始key
        // 原始key字节数组
        let mut key_bytes = key.into_bytes();
        // 原始key字节数组长度
        let mut key_bytes_len_bytes = trans_u16_2_bytes(key_bytes.len() as u16);
        // 视图内容索引(8字节)+原始key长度
        seek_start_bytes.append(&mut key_bytes_len_bytes);
        // 视图内容索引(8字节)+原始key长度+原始key
        seek_start_bytes.append(&mut key_bytes);
        // 持续长度(4字节)
        let mut data_len_bytes = trans_u32_2_bytes(seek_start_bytes.len() as u32);
        // 生成定位数据字节数组=数据类型(1字节)+持续长度(4字节)
        value_type_bytes.append(&mut data_len_bytes);
        // 执行视图存储操作，得到定位数据起始偏移量
        let pos_seek_start = view.write_content(db_name, value_type_bytes)?;
        // 视图内容索引(8字节)
        let mut pos_seek_start_bytes = trans_u64_2_bytes(pos_seek_start);
        // 记录表文件属性(数据归档/定位文件用2字节)+数据在表文件中起始偏移量p(6字节)
        // 记录表文件属性为当前视图版本号(数据归档/定位文件用2字节)
        // 生成数据在表文件中起始偏移量p(6字节)
        pos_seek_start_bytes = Vector::sub(pos_seek_start_bytes, 2, 8);
        // 生成存储在索引中的视图文件坐标信息(8字节)
        version_bytes.append(&mut pos_seek_start_bytes);
        // 将视图索引偏移量记录在索引文件指定位置
        Filer::write_seeks(file, self.seek, version_bytes)
    }
}

/// 封装方法函数
impl Seed {
    /// 新建seed
    pub fn create(key: String) -> Seed {
        return Seed {
            key,
            policies: Vec::new(),
        };
    }
}

/// 封装方法函数
impl TSeed for Seed {
    fn key(&self) -> String {
        self.key.clone()
    }
    fn modify(&mut self, value: Vec<u8>) -> GeorgeResult<()> {
        self.policies.push(IndexPolicy::from(value)?);
        Ok(())
    }
    fn save(
        &self,
        database_name: String,
        view: View,
        value: Vec<u8>,
        force: bool,
    ) -> GeorgeResult<()> {
        // todo 失败回滚
        if self.policies.len() == 0 {
            return Ok(());
        }
        // 执行真实存储操作，即索引将seed存入后，允许检索到该结果，但该结果值不存在，仅当所有索引存入都成功，才会执行本方法完成真实存储操作
        let view_seek_start = view.write_content(database_name.clone(), value)?;
        // 视图内容索引(8字节)
        let view_seek_start_bytes = trans_u64_2_bytes(view_seek_start);
        // 生成视图文件属性，版本号(2字节)
        let view_version_bytes = trans_u16_2_bytes(view.version());

        // 将在数据在view中的坐标存入各个index
        for policy in self.policies.to_vec() {
            policy.exec(
                database_name.clone(),
                self.key(),
                view.clone(),
                view_version_bytes.clone(),
                view_seek_start_bytes.clone(),
                force,
            )?
        }
        Ok(())
    }

    fn remove(&self, database_name: String, view: View) -> GeorgeResult<()> {
        if self.policies.len() == 0 {
            return Ok(());
        }
        // 生成视图文件属性，版本号(2字节)
        let view_version_bytes = trans_u16_2_bytes(view.version());
        for policy in self.policies.to_vec() {
            // todo 设计碰撞模型
            policy.exec(
                database_name.clone(),
                self.key(),
                view.clone(),
                view_version_bytes.clone(),
                vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
                true,
            )?
        }
        Ok(())
    }
}
