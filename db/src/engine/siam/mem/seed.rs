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

use comm::errors::entrances::GeorgeResult;

use crate::engine::traits::TSeed;

/// B+Tree索引叶子结点内防hash碰撞数组结构中单体结构
///
/// 搭配Index使用
///
/// 叶子节点下真实存储数据的集合单体结构
#[derive(Debug)]
pub(crate) struct Seed {
    /// 当前结果原始key信息
    key: String,
    value: Option<Vec<u8>>,
}

/// 封装方法函数
impl Seed {
    /// 新建seed
    pub(crate) fn create(key: String) -> Seed {
        return Seed { key, value: None };
    }
}

/// 封装方法函数
impl TSeed for Seed {
    fn key(&self) -> String {
        self.key.clone()
    }
    fn value(&self) -> Option<Vec<u8>> {
        self.value.clone()
    }
    fn modify(&mut self, value: Vec<u8>) {
        self.value = Some(value);
    }
    fn save(&mut self, value: Vec<u8>) -> GeorgeResult<()> {
        self.value = Some(value);
        Ok(())
    }

    fn remove(&mut self) -> GeorgeResult<()> {
        self.value = None;
        Ok(())
    }
}
