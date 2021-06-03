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

use std::path::Path;
use std::sync::Arc;

use comm::errors::{Errs, GeorgeResult};

use crate::utils::enums::Tag;
use crate::{Ge, GeFactory, GeImpl};

mod bootstrap;
mod database;
mod index;
mod ledger;
mod node;
mod page;
mod view;

impl GeFactory {
    /// ##生成`ge`文件对象
    ///
    /// ###Params
    /// * filepath 文件所在路径
    /// * description 文件描述内容
    ///
    /// ###Return
    ///
    /// 返回一个拼装完成的文件元数据信息，长度52字节
    pub fn create<P: AsRef<Path>>(
        &self,
        tag: Tag,
        filepath: P,
        description: Option<Vec<u8>>,
    ) -> GeorgeResult<Arc<dyn Ge>> {
        if tag == Tag::Node {
            Ok(Arc::new(Node::new(filepath)?))
        } else {
            let des: Vec<u8>;
            match description {
                Some(res) => des = res,
                None => des = vec![],
            }
            match tag {
                Tag::Bootstrap => Ok(Arc::new(Bootstrap::new(filepath, des)?)),
                Tag::Page => Ok(Arc::new(Page::new(filepath, des)?)),
                Tag::Database => Ok(Arc::new(Database::new(filepath, des)?)),
                Tag::View => Ok(Arc::new(View::new(filepath, des)?)),
                Tag::Ledger => Ok(Arc::new(Ledger::new(filepath, des)?)),
                Tag::Index => Ok(Arc::new(Index::new(filepath, des)?)),
                _ => Err(Errs::str("ge tag none do not support!")),
            }
        }
    }

    /// ##恢复`ge`文件对象
    ///
    /// ###Params
    /// * filepath 文件所在路径
    ///
    /// ###Return
    ///
    /// 返回一个拼装完成的文件元数据信息，长度52字节
    pub fn recovery<P: AsRef<Path>>(&self, tag: Tag, filepath: P) -> GeorgeResult<Arc<dyn Ge>> {
        match tag {
            Tag::Bootstrap => Ok(Arc::new(Bootstrap::recovery(filepath)?)),
            Tag::Page => Ok(Arc::new(Page::recovery(filepath)?)),
            Tag::Database => Ok(Arc::new(Database::recovery(filepath)?)),
            Tag::View => Ok(Arc::new(View::recovery(filepath)?)),
            Tag::Ledger => Ok(Arc::new(Ledger::recovery(filepath)?)),
            Tag::Index => Ok(Arc::new(Index::recovery(filepath)?)),
            Tag::Node => Ok(Arc::new(Node::recovery(filepath)?)),
            _ => Err(Errs::str("ge tag none do not support!")),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Bootstrap {
    ge: GeImpl,
}

#[derive(Clone, Debug)]
pub struct Page {
    ge: GeImpl,
}

#[derive(Clone, Debug)]
pub struct Database {
    ge: GeImpl,
}

#[derive(Clone, Debug)]
pub struct View {
    ge: GeImpl,
}

#[derive(Clone, Debug)]
pub struct Ledger {
    ge: GeImpl,
}

#[derive(Clone, Debug)]
pub struct Index {
    ge: GeImpl,
}

#[derive(Clone, Debug)]
pub struct Node {
    ge: GeImpl,
}
