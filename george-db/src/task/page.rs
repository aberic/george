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

use std::sync::{Arc, RwLock};

use chrono::Duration;

use george_comm::errors::{Errs, GeorgeResult};
use george_comm::strings::StringHandler;
use george_comm::{Strings, Time};
use george_ge::utils::enums::Tag;
use george_ge::GeFactory;

use crate::task::engine::memory::Node;
use crate::task::Page;
use crate::utils::Paths;

impl Page {
    /// 新建缓存页
    ///
    /// 具体传参参考如下定义：<p><p>
    ///
    /// ###Params
    ///
    /// * name 缓存页名称
    /// * comment 缓存页描述
    /// * size 可使用内存大小(单位：Mb)，为0则不限
    /// * period 默认有效期(单位：秒)，如为0，则默认为300
    pub(crate) fn create(
        name: String,
        comment: String,
        size: u64,
        period: u32,
    ) -> GeorgeResult<Arc<RwLock<Page>>> {
        let create_time = Time::now();
        let filepath = Paths::page_filepath(name.clone());
        let description = Some(Page::description(
            name.clone(),
            comment.clone(),
            size,
            period,
            create_time,
        ));
        Ok(Arc::new(RwLock::new(Page {
            name,
            comment,
            size,
            period,
            create_time,
            ge: GeFactory {}.create(Tag::Page, filepath, description)?,
            node: Node::create(),
        })))
    }

    /// 名称
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// 描述
    pub fn comment(&self) -> String {
        self.comment.clone()
    }

    /// 可使用内存大小(单位：Mb，0：不限制大小)
    pub fn size(&self) -> u64 {
        self.size
    }

    /// 默认有效期(单位：秒)，如无设置，默认维300(0：永久有效)
    pub fn period(&self) -> u32 {
        self.period
    }

    /// 创建时间
    pub fn create_time(&self) -> Time {
        self.create_time.clone()
    }

    pub(super) fn node(&self) -> Arc<RwLock<Node>> {
        self.node.clone()
    }
}

/// db for disk
impl Page {
    /// 插入数据，如果存在则返回已存在<p><p>
    ///
    /// ###Params
    ///
    /// key string
    ///
    /// value 当前结果value信息<p><p>
    ///
    /// ###Return
    ///
    /// IndexResult<()>
    pub(crate) fn put(&self, key: String, value: Vec<u8>) -> GeorgeResult<()> {
        self.node().read().unwrap().put(key, value, false)
    }

    /// 插入数据，无论存在与否都会插入或更新数据<p><p>
    ///
    /// ###Params
    ///
    /// key string
    ///
    /// value 当前结果value信息<p><p>
    ///
    /// ###Return
    ///
    /// IndexResult<()>
    pub(crate) fn set(&self, key: String, value: Vec<u8>) -> GeorgeResult<()> {
        self.node().read().unwrap().put(key, value, true)
    }

    /// 获取数据，返回存储对象<p><p>
    ///
    /// ###Params
    ///
    /// key string
    ///
    /// ###Return
    ///
    /// Seed value信息
    pub(crate) fn get(&self, key: String) -> GeorgeResult<Vec<u8>> {
        self.node().read().unwrap().get(key)
    }

    /// 删除数据<p><p>
    ///
    /// ###Params
    ///
    /// key string
    ///
    /// ###Return
    ///
    /// IndexResult<()>
    pub(crate) fn remove(&self, key: String) -> GeorgeResult<()> {
        self.node().read().unwrap().del(key)
    }
}

impl Page {
    /// 生成文件描述
    fn description(
        name: String,
        comment: String,
        size: u64,
        period: u32,
        create_time: Time,
    ) -> Vec<u8> {
        hex::encode(format!(
            "{}:#?{}:#?{}:#?{}:#?{}",
            name,
            comment,
            size,
            period,
            create_time.nano_string().unwrap(),
        ))
        .into_bytes()
    }

    /// 通过文件描述恢复结构信息
    pub(crate) fn recover(name: String) -> GeorgeResult<Page> {
        let filepath = Paths::page_filepath(name.clone());
        let ge = GeFactory {}.recovery(Tag::Page, filepath)?;
        let description_str = Strings::from_utf8(ge.description_content_bytes()?)?;
        match hex::decode(description_str) {
            Ok(vu8) => {
                let real = Strings::from_utf8(vu8)?;
                let mut split = real.split(":#?");
                let name = split.next().unwrap().to_string();
                let comment = split.next().unwrap().to_string();
                let size = split.next().unwrap().to_string().parse::<u64>().unwrap();
                let period = split.next().unwrap().to_string().parse::<u32>().unwrap();
                let duration = Duration::nanoseconds(
                    split.next().unwrap().to_string().parse::<i64>().unwrap(),
                );
                let page = Page {
                    name,
                    comment,
                    size,
                    period,
                    create_time: Time::from(duration),
                    ge,
                    node: Node::recovery(),
                };
                log::info!("recovery page {}", page.name());
                Ok(page)
            }
            Err(err) => Err(Errs::strs("recovery page decode", err)),
        }
    }
}
