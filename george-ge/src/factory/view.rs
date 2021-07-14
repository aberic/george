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

use george_comm::errors::GeorgeResult;

use crate::factory::View;
use crate::utils::enums::Tag;
use crate::{Ge, GeImpl};

impl View {
    /// ##生成`ge`文件对象
    ///
    /// ###Params
    /// * filepath 文件所在路径
    /// * description 文件描述内容
    ///
    /// ###Return
    ///
    /// 返回一个拼装完成的文件元数据信息，长度52字节
    pub(crate) fn new<P: AsRef<Path>>(filepath: P, description: Vec<u8>) -> GeorgeResult<Self>
    where
        Self: Sized,
    {
        Ok(View {
            ge: GeImpl::new(filepath, Tag::View, description)?,
        })
    }

    /// ##恢复`ge`文件对象
    ///
    /// ###Params
    /// * filepath 文件所在路径
    ///
    /// ###Return
    ///
    /// 返回一个拼装完成的文件元数据信息，长度52字节
    pub(crate) fn recovery<P: AsRef<Path>>(filepath: P) -> GeorgeResult<Self>
    where
        Self: Sized,
    {
        Ok(View {
            ge: GeImpl::recovery(filepath)?,
        })
    }
}

impl Ge for View {
    fn inner(&self) -> Box<dyn Ge> {
        Box::new(self.ge.clone())
    }
}
