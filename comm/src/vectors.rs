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

use crate::errors::entrances::{Errs, GeorgeResult};

pub trait VectorHandler {
    /// 变更数组内容
    ///
    /// source 原始数组
    ///
    /// target 变更内容
    ///
    /// start 起始下标
    fn modify<T: Clone>(source: Vec<T>, target: Vec<T>, start: usize) -> Vec<T>;
    /// 截取数组
    ///
    /// source 原始数组
    ///
    /// start 截取起始下标
    ///
    /// end 截取终止下标
    fn sub<T: Clone>(source: Vec<T>, start: usize, end: usize) -> GeorgeResult<Vec<T>>;
    /// 截取数组
    ///
    /// source 原始数组
    ///
    /// start 截取起始下标
    ///
    /// 持续读取个数
    fn sub_last<T: Clone>(source: Vec<T>, start: usize, last: usize) -> GeorgeResult<Vec<T>>;
    /// 从可被`eq`整除的bytes长度的字节数组中查找最后不为0的`eq`个字节组成新的数组
    fn find_last_eq_bytes(bytes: Vec<u8>, eq: usize) -> GeorgeResult<Vec<u8>>;
    /// 从可被`eq`整除的bytes长度的字节数组中查找所有与`eq`长度相同的不为0的字节数组集合
    fn find_eq_vec_bytes(bytes: Vec<u8>, eq: usize) -> GeorgeResult<Vec<Vec<u8>>>;
    /// 创建长度为len且字节均为0x00的字节数组
    fn create_empty_bytes(len: usize) -> Vec<u8>;
    /// 检查字节数组是否被填充，即数组中任意字节不为`0x00`
    fn is_fill(bytes: Vec<u8>) -> bool;
    /// 检查字节数组是否为空或都不为`0x00`
    fn is_empty(bytes: Vec<u8>) -> bool;
}

pub struct Vector {}

impl VectorHandler for Vector {
    fn modify<T: Clone>(source: Vec<T>, target: Vec<T>, start: usize) -> Vec<T> {
        vector_modify(source, target, start)
    }
    fn sub<T: Clone>(source: Vec<T>, start: usize, end: usize) -> GeorgeResult<Vec<T>> {
        vector_sub(source, start, end)
    }
    fn sub_last<T: Clone>(source: Vec<T>, start: usize, last: usize) -> GeorgeResult<Vec<T>> {
        vector_sub(source, start, start + last)
    }
    fn find_last_eq_bytes(bytes: Vec<u8>, eq: usize) -> GeorgeResult<Vec<u8>> {
        vector_find_last_eq_bytes(bytes, eq)
    }
    fn find_eq_vec_bytes(bytes: Vec<u8>, eq: usize) -> GeorgeResult<Vec<Vec<u8>>> {
        vector_find_eq_vec_bytes(bytes, eq)
    }
    fn create_empty_bytes(len: usize) -> Vec<u8> {
        create_empty_bytes(len)
    }

    fn is_fill(bytes: Vec<u8>) -> bool {
        let bs_len = bytes.len();
        let mut i = 0;
        while i < bs_len {
            if bytes[i].ne(&0x00) {
                return true;
            }
            i += 1;
        }
        false
    }

    fn is_empty(bytes: Vec<u8>) -> bool {
        let bs_len = bytes.len();
        let mut i = 0;
        while i < bs_len {
            if bytes[i].ne(&0x00) {
                return false;
            }
            i += 1;
        }
        true
    }
}

/// 变更数组内容
///
/// source 原始数组
///
/// target 变更内容
///
/// start 起始下标
fn vector_modify<T: Clone>(mut source: Vec<T>, target: Vec<T>, mut start: usize) -> Vec<T> {
    let len = target.len();
    let mut position = 0;
    while position < len {
        source.remove(start);
        source.insert(start, target.get(position).unwrap().clone());
        start += 1;
        position += 1
    }
    source
}

/// 截取数组
///
/// source 原始数组
///
/// start 截取起始下标
///
/// end 截取终止下标，如果为0，则取start之后所有数据
fn vector_sub<T: Clone>(source: Vec<T>, start: usize, end: usize) -> GeorgeResult<Vec<T>> {
    let source_len = source.len();
    if source_len < end {
        Err(Errs::str("source array type out of bounds"))
    } else {
        let mut s1 = source.to_vec();
        let mut s2 = s1.split_off(start);
        if end > 0 {
            let _x = s2.split_off(end - start);
        }
        Ok(s2)
    }
}

/// 从可被`eq`整除的bytes长度的字节数组中查找最后不为0的`eq`个字节组成新的数组
fn vector_find_last_eq_bytes(bytes: Vec<u8>, eq: usize) -> GeorgeResult<Vec<u8>> {
    let mut res: Vec<u8> = vec![];
    let mut temp: Vec<u8> = vec![];
    let mut position = 0;
    let mut valid = false;
    for b in bytes {
        if position < eq {
            if valid || b > 0x00 {
                valid = true;
            }
            temp.push(b);
            position += 1
        } else {
            if temp.len().ne(&eq) {
                return Err(Errs::str("temp length out of 8"));
            }
            if valid {
                res = temp.to_vec();
            }
            temp.clear();
            position = 0;
            if b > 0x00 {
                valid = true;
            } else {
                valid = false;
            }
            temp.push(b);
            position += 1
        }
    }
    Ok(res)
}

/// 从可被`eq`整除的bytes长度的字节数组中查找所有与`eq`长度相同的不为0的字节数组集合
fn vector_find_eq_vec_bytes(mut bytes: Vec<u8>, eq: usize) -> GeorgeResult<Vec<Vec<u8>>> {
    if bytes.len() % eq != 0 {
        return Err(Errs::string(format!("bytes length can't % by {}", eq)));
    }
    // 此步确保能够遍历完成最后一组
    bytes.push(0x00);
    let mut res: Vec<Vec<u8>> = vec![];
    let mut temp: Vec<u8> = vec![];
    let mut position = 0;
    let mut valid = false;
    for b in bytes {
        if position < eq {
            if valid || b > 0x00 {
                valid = true;
            }
            temp.push(b);
            position += 1
        } else {
            if temp.len().ne(&eq) {
                return Err(Errs::str("temp length out of 8"));
            }
            if valid {
                res.push(temp.to_vec())
            }
            temp.clear();
            position = 0;
            if b > 0x00 {
                valid = true;
            } else {
                valid = false;
            }
            temp.push(b);
            position += 1
        }
    }
    Ok(res)
}

/// 创建长度为len且字节均为0x00的字节数组
fn create_empty_bytes(len: usize) -> Vec<u8> {
    let mut res: Vec<u8> = vec![];
    let mut position = 0;
    while position < len {
        res.push(0x00);
        position += 1
    }
    res
}
