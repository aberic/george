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

use chrono::{Duration, Local, NaiveDateTime};

use comm::errors::entrances::{Errs, GeorgeResult};
use comm::strings::{StringHandler, Strings};

use crate::task::engine::memory::node::Node;
use crate::utils::path::Paths;
use crate::utils::store::{ContentBytes, Metadata, HD};
use crate::utils::writer::Filed;

#[derive(Debug, Clone)]
pub(crate) struct Page {
    /// 名称
    name: String,
    /// 描述
    comment: String,
    /// 可使用内存大小(单位：Mb，0：不限制大小)
    size: u64,
    /// 默认有效期(单位：秒)，如无设置，默认维300(0：永久有效)
    period: u32,
    /// 创建时间
    create_time: Duration,
    /// 文件信息
    metadata: Metadata,
    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 需要借助对象包裹，以便更新file，避免self为mut
    filer: Filed,
    /// 默认缓存页
    node: Arc<RwLock<Node>>,
}

/// 新建缓存页
///
/// 具体传参参考如下定义：<p><p>
///
/// ###Params
///
/// name 缓存页名称
///
/// comment 缓存页描述
///
/// size 可使用内存大小(单位：Mb)
///
/// period 默认有效期(单位：秒)，如无设置，默认维300
fn new_page(name: String, comment: String, size: u64, period: u32) -> GeorgeResult<Page> {
    let now: NaiveDateTime = Local::now().naive_local();
    let create_time = Duration::nanoseconds(now.timestamp_nanos());
    let filepath = Paths::page_filepath(name.clone());
    Ok(Page {
        name,
        comment,
        size,
        period,
        create_time,
        metadata: Metadata::page(),
        filer: Filed::create(filepath)?,
        node: Node::create(),
    })
}

impl Page {
    /// 新建缓存页
    ///
    /// 具体传参参考如下定义：<p><p>
    ///
    /// ###Params
    ///
    /// name 缓存页名称
    ///
    /// comment 缓存页描述
    ///
    /// size 可使用内存大小(单位：Mb)
    ///
    /// period 默认有效期(单位：秒)，如无设置，默认维300
    pub(crate) fn create(name: String, comment: String) -> GeorgeResult<Arc<RwLock<Page>>> {
        let page = new_page(name, comment, 0, 0)?;
        let mut metadata_bytes = page.metadata_bytes();
        let mut description = page.description();
        // 初始化为32 + 8，即head长度加正文描述符长度
        let mut before_description = ContentBytes::before(44, description.len() as u32);
        metadata_bytes.append(&mut before_description);
        metadata_bytes.append(&mut description);
        page.append(metadata_bytes)?;
        Ok(Arc::new(RwLock::new(page)))
    }

    /// 名称
    pub(crate) fn name(&self) -> String {
        self.name.clone()
    }

    /// 描述
    pub(crate) fn comment(&self) -> String {
        self.comment.clone()
    }

    /// 创建时间
    pub(crate) fn create_time(&self) -> Duration {
        self.create_time.clone()
    }

    /// 文件字节信息
    pub(crate) fn metadata_bytes(&self) -> Vec<u8> {
        self.metadata.bytes()
    }

    pub(super) fn node(&self) -> Arc<RwLock<Node>> {
        self.node.clone()
    }

    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
    ///
    /// #Return
    ///
    /// seek_end_before 写之前文件字节数据长度
    fn append(&self, content: Vec<u8>) -> GeorgeResult<u64> {
        self.filer.append(content)
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
    fn description(&self) -> Vec<u8> {
        hex::encode(format!(
            "{}:#?{}:#?{}:#?{}:#?{}",
            self.name(),
            self.comment(),
            self.size,
            self.period,
            self.create_time().num_nanoseconds().unwrap().to_string(),
        ))
        .into_bytes()
    }
    /// 通过文件描述恢复结构信息
    pub(crate) fn recover(hd: HD) -> GeorgeResult<Page> {
        let description_str = Strings::from_utf8(hd.description())?;
        match hex::decode(description_str) {
            Ok(vu8) => {
                let real = Strings::from_utf8(vu8)?;
                let mut split = real.split(":#?");
                let name = split.next().unwrap().to_string();
                let comment = split.next().unwrap().to_string();
                let size = split.next().unwrap().to_string().parse::<u64>().unwrap();
                let period = split.next().unwrap().to_string().parse::<u32>().unwrap();
                let create_time = Duration::nanoseconds(
                    split.next().unwrap().to_string().parse::<i64>().unwrap(),
                );
                let filepath = Paths::page_filepath(name.clone());
                let page = Page {
                    name,
                    comment,
                    size,
                    period,
                    create_time,
                    metadata: hd.metadata(),
                    filer: Filed::recovery(filepath)?,
                    node: Node::recovery(),
                };
                log::info!("recovery page {}", page.name());
                Ok(page)
            }
            Err(err) => Err(Errs::strs("recovery page decode", err)),
        }
    }
}
