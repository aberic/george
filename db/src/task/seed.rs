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

use comm::errors::entrances::{err_string, GeorgeResult};
use comm::io::writer::write_seek_u8s;
use comm::trans::trans_u64_2_bytes;
use serde::{Deserialize, Serialize};

use crate::task::engine::traits::TSeed;

/// B+Tree索引叶子结点内防hash碰撞数组结构中单体结构
///
/// 搭配Index使用
///
/// 叶子节点下真实存储数据的集合单体结构
#[derive(Debug)]
pub struct Seed {
    /// 获取当前结果原始key信息
    key: String,
    /// 索引操作策略集合
    policies: Vec<IndexPolicy>,
}

/// 待处理索引操作策略
#[derive(Debug, Clone, Serialize, Deserialize)]
struct IndexPolicy {
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
    fn exec(&self, value: Vec<u8>) -> GeorgeResult<()> {
        write_seek_u8s(self.index_file_path(), self.seek, value.as_slice())
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
    fn save(&mut self, view_seek_end: u64) -> GeorgeResult<()> {
        // todo 失败回滚
        if self.policies.len() == 0 {
            return Ok(());
        }
        let view_seek_end_bytes = trans_u64_2_bytes(view_seek_end);
        // 将在数据在view中的坐标存入各个index
        for policy in self.policies.to_vec() {
            policy.exec(view_seek_end_bytes.clone())?
        }
        Ok(())
    }

    fn remove(&mut self) -> GeorgeResult<()> {
        if self.policies.len() == 0 {
            return Ok(());
        }
        for policy in self.policies.to_vec() {
            policy.exec(vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])?
        }
        Ok(())
    }
}
