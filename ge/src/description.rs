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

use std::fmt;
use std::sync::{Arc, RwLock};

use comm::errors::{Errs, GeorgeResult};
use comm::vectors::VectorHandler;
use comm::{Trans, Vector};

use crate::metadata::Description;
use crate::utils::Filed;

impl fmt::Debug for Description {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{start: {}, len: {}, modify: {}}}",
            self.start, self.len, self.modify
        )
    }
}

/// impl for new
impl Description {
    /// 文件描述起始值
    /// 文件描述由描述起始坐标(8字节) + 描述内容长度(4字节) + 变更后文件描述起始坐标(8字节)
    /// 起始坐标为52，即文件元数据信息(52字节)后开始计算
    /// * 默认新建modify为0
    /// * 默认新建modify_seek为`首部信息(32字节) + 文件描述由描述起始坐标(8字节) + 描述内容长度(4字节)`=44
    pub(crate) fn new(filed: Arc<RwLock<Filed>>, len: usize) -> Self {
        Description {
            start: 52,
            len,
            modify: 0,
            filed,
        }
    }
}

/// impl for fn
impl Description {
    /// ##生成`ge`文件描述信息，长度20字节
    /// 文件描述由描述起始坐标(8字节) + 描述内容长度(4字节) + 变更后文件描述起始坐标(8字节)
    ///
    /// ###Return
    ///
    /// 返回一个拼装完成的文件描述信息，长度20字节
    fn des_to_vec(start: u64, len: usize, modify: u64) -> Vec<u8> {
        let mut v8s: Vec<u8> = vec![];
        // 描述起始坐标(8字节)
        let mut start_bytes = Trans::u64_2_bytes(start);
        // 描述内容长度(4字节)
        let mut len_bytes = Trans::u32_2_bytes(len as u32);
        // 变更后文件描述起始坐标(8字节)
        let mut modify_bytes = Trans::u64_2_bytes(modify);
        v8s.append(&mut start_bytes);
        v8s.append(&mut len_bytes);
        v8s.append(&mut modify_bytes);
        v8s
    }

    /// 变更描述信息
    ///
    /// ###Params
    /// * description 待变更的文件描述内容
    pub(crate) fn modify(&mut self, mut description_bytes: Vec<u8>) -> GeorgeResult<()> {
        if self.modify > 0 {
            Err(Errs::string(format!(
                "ge file {} is invalid!",
                self.filed.read().unwrap().filepath()
            )))
        } else {
            let len = description_bytes.len();
            // 创建空文件描述字节数组
            let mut des = Vector::create_empty_bytes(20);
            des.append(&mut description_bytes);
            // 原modify记录下一文件描述坐标地址
            let modify = self.append(des)?;

            // 新文件描述内容
            // 描述起始坐标 = 文件描述坐标地址 + 文件描述(20字节)
            let des_new_start = modify + 20;
            let des_new_len = len;
            let des_new_bytes = Description::des_to_vec(des_new_start, des_new_len, 0);
            self.write(modify, des_new_bytes)?;

            // `文件描述`由`描述起始坐标(8字节) + 描述内容长度(4字节) + 变更后文件描述起始坐标(8字节)`
            // `self.start`后紧跟描述内容，`self.start - 8`即为记录`变更后文件描述起始坐标(8字节)`的坐标
            let modify_seek = self.start - 8;
            let modify_bytes = Trans::u64_2_bytes(modify);
            self.write(modify_seek, modify_bytes)?;
            self.start = des_new_start;
            self.len = len;
            Ok(())
        }
    }

    /// 文件描述变更记录
    pub(crate) fn history(&self) -> GeorgeResult<Vec<Vec<u8>>> {
        let mut des_vc: Vec<Vec<u8>> = vec![];
        let mut modify_start = 32;
        loop {
            let description_bytes = self.filed.read().unwrap().read(modify_start, 20)?;
            let start = Trans::bytes_2_u64(description_bytes[0..8].to_vec())?;
            let last = Trans::bytes_2_u32(description_bytes[8..12].to_vec())? as usize;
            modify_start = Trans::bytes_2_u64(description_bytes[12..20].to_vec())?;
            des_vc.push(self.filed.read().unwrap().read(start, last)?);
            if modify_start == 0 {
                break;
            }
        }
        Ok(des_vc)
    }

    /// ##生成`ge`文件描述信息，长度20字节
    /// 文件描述由描述起始坐标(8字节) + 描述内容长度(4字节) + 变更后文件描述起始坐标(8字节)
    ///
    /// ###Return
    ///
    /// 返回一个拼装完成的文件描述信息，长度20字节
    pub(crate) fn to_vec(&self) -> Vec<u8> {
        Description::des_to_vec(self.start, self.len, self.modify)
    }

    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
    ///
    /// #Return
    ///
    /// seek_end_before 写之前文件字节数据长度
    fn append(&self, content: Vec<u8>) -> GeorgeResult<u64> {
        self.filed.write().unwrap().append(content)
    }

    /// 写入的写对象到指定坐标
    ///
    /// 直接进行写操作，不提供对外获取方法，因为当库名称发生变更时会导致异常
    fn write(&self, seek: u64, content: Vec<u8>) -> GeorgeResult<()> {
        self.filed.write().unwrap().write(seek, content)
    }
}

/// impl for recovery
impl Description {
    /// ##恢复`ge`文件描述信息，长度20字节
    pub(crate) fn recovery(
        filed: Arc<RwLock<Filed>>,
        description_bytes: Vec<u8>,
    ) -> GeorgeResult<Description> {
        if description_bytes.len() != 20 {
            Err(Errs::str(
                "recovery description failed! description bytes len must be 20!",
            ))
        } else {
            let start = Trans::bytes_2_u64(description_bytes[0..8].to_vec())?;
            let len = Trans::bytes_2_u32(description_bytes[8..12].to_vec())? as usize;
            let modify = Trans::bytes_2_u64(description_bytes[12..20].to_vec())?;
            if modify > 0 {
                let description_bytes = filed.read().unwrap().read(modify, 20)?;
                Description::recovery(filed, description_bytes)
            } else {
                Ok(Description {
                    start,
                    len,
                    modify,
                    filed,
                })
            }
        }
    }
}
