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

use crypto::digest::Digest;
use crypto::md5::Md5;

use crate::errors::entrances::{err_str, err_strings, GeorgeResult};
use crate::strings::{StringHandler, Strings};
use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Hash;

pub trait HashHandler<T> {
    fn md5(comment: T) -> String;
    fn md516(comment: T) -> String;
    fn crc32(comment: T) -> u32;
    fn crc64(comment: T) -> u64;
}

/// Hash支持类型
#[derive(Debug, Clone)]
pub enum HashType {
    /// 字符串
    String,
    /// 无符号64位整型
    U64,
    /// 有符号64位整型
    I64,
    /// 无符号64位整型
    U32,
    /// 有符号64位整型
    I32,
    /// 有符号64位浮点类型
    F64,
    /// 有符号32位浮点类型
    F32,
    /// bool类型
    Bool,
    /// 不支持类型
    None,
}

pub trait CRCTypeHandler {
    fn crc32(tp: HashType, comment: String) -> GeorgeResult<u32>;
    fn crc64(tp: HashType, comment: String) -> GeorgeResult<u64>;
}

impl HashHandler<&[u8]> for Hash {
    fn md5(comment: &[u8]) -> String {
        md5_u8s(comment)
    }
    fn md516(comment: &[u8]) -> String {
        md516_u8s(comment)
    }
    fn crc32(comment: &[u8]) -> u32 {
        hashcode32_u8s(comment)
    }
    fn crc64(comment: &[u8]) -> u64 {
        hashcode64_u8s(comment)
    }
}

impl HashHandler<String> for Hash {
    fn md5(comment: String) -> String {
        md5(comment)
    }
    fn md516(comment: String) -> String {
        md516(comment)
    }
    fn crc32(comment: String) -> u32 {
        hashcode32(comment)
    }
    fn crc64(comment: String) -> u64 {
        hashcode64(comment)
    }
}

impl CRCTypeHandler for Hash {
    fn crc32(tp: HashType, comment: String) -> GeorgeResult<u32> {
        let mut hash_key: u32 = 0;
        match tp {
            HashType::String => hash_key = hashcode32(comment),
            HashType::Bool => hash_key = hashcode32_bl(comment)?,
            HashType::U32 => hash_key = hashcode32_u32(comment)?,
            HashType::F32 => hash_key = hashcode32_f32(comment)?,
            HashType::I32 => hash_key = hashcode32_i32(comment)?,
            _ => return Err(err_str("hash type not support!")),
        }
        Ok(hash_key)
    }
    fn crc64(tp: HashType, comment: String) -> GeorgeResult<u64> {
        let mut hash_key: u64 = 0;
        match tp {
            HashType::String => hash_key = hashcode64(comment),
            HashType::Bool => hash_key = hashcode64_bl(comment)?,
            HashType::U32 => hash_key = hashcode64_u64(comment)?,
            HashType::U64 => hash_key = hashcode64_u64(comment)?,
            HashType::F32 => hash_key = hashcode64_f64(comment)?,
            HashType::F64 => hash_key = hashcode64_f64(comment)?,
            HashType::I32 => hash_key = hashcode64_i64(comment)?,
            HashType::I64 => hash_key = hashcode64_i64(comment)?,
            _ => return Err(err_str("hash type not support!")),
        }
        Ok(hash_key)
    }
}

pub fn md5(comment: String) -> String {
    let mut md5_handler = Md5::new();
    md5_handler.input_str(comment.as_str());
    md5_handler.result_str()
}

fn md5_u8s(comment: &[u8]) -> String {
    let mut md5_handler = Md5::new();
    md5_handler.input(comment);
    md5_handler.result_str()
}

pub fn md516(comment: String) -> String {
    Strings::subs(md5(comment), 8, 24)
}

fn md516_u8s(comment: &[u8]) -> String {
    Strings::subs(md5_u8s(comment), 8, 24)
}

pub fn hashcode32(comment: String) -> u32 {
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(comment.as_bytes());
    hasher.finalize()
}

pub fn hashcode32_u8s(comment: &[u8]) -> u32 {
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(comment);
    hasher.finalize()
}

pub fn hashcode32_enhance(comment: String) -> u32 {
    return match comment.parse::<u32>() {
        Ok(su32) => su32,
        Err(_err) => hashcode32_u8s(comment.as_bytes()),
    };
}

pub fn hashcode64_u8s(comment: &[u8]) -> u64 {
    let mut c = crc64fast::Digest::new();
    c.write(comment);
    c.sum64()
}

pub fn hashcode64(comment: String) -> u64 {
    hashcode64_u8s(comment.as_bytes())
}

pub fn hashcode32_u32(comment: String) -> GeorgeResult<u32> {
    match comment.parse::<u32>() {
        Ok(real) => Ok(real),
        Err(err) => Err(err_strings(format!("{} parse to u32", comment), err)),
    }
}

pub fn hashcode64_u64(comment: String) -> GeorgeResult<u64> {
    match comment.parse::<u64>() {
        Ok(real) => Ok(real),
        Err(err) => Err(err_strings(format!("{} parse to u64", comment), err)),
    }
}

pub fn hashcode32_i32(comment: String) -> GeorgeResult<u32> {
    match comment.parse::<i32>() {
        Ok(real) => Ok(hashcode32_i32_real(real)),
        Err(err) => Err(err_strings(format!("{} parse to i32", comment), err)),
    }
}

pub fn hashcode64_i64(comment: String) -> GeorgeResult<u64> {
    match comment.parse::<i64>() {
        Ok(real) => Ok(hashcode64_i64_real(real)),
        Err(err) => Err(err_strings(format!("{} parse to i64", comment), err)),
    }
}

pub fn hashcode32_f32(comment: String) -> GeorgeResult<u32> {
    match comment.parse::<f32>() {
        Ok(real) => Ok(hashcode32_f32_real(real)),
        Err(err) => Err(err_strings(format!("{} parse to f32", comment), err)),
    }
}

pub fn hashcode64_f64(comment: String) -> GeorgeResult<u64> {
    match comment.parse::<f64>() {
        Ok(real) => Ok(hashcode64_f64_real(real)),
        Err(err) => Err(err_strings(format!("{} parse to f64", comment), err)),
    }
}

pub fn hashcode32_bl(comment: String) -> GeorgeResult<u32> {
    match comment.parse::<bool>() {
        Ok(real) => Ok(hashcode32_bl_real(real)),
        Err(err) => Err(err_strings(format!("{} parse to bool", comment), err)),
    }
}

pub fn hashcode64_bl(comment: String) -> GeorgeResult<u64> {
    match comment.parse::<bool>() {
        Ok(real) => Ok(hashcode64_bl_real(real)),
        Err(err) => Err(err_strings(format!("{} parse to bool", comment), err)),
    }
}

pub fn hashcode32_i32_real(real: i32) -> u32 {
    if real < 0 {
        real.add(2147483647).add(1) as u32
    } else {
        (real as u32).add(2147483648)
    }
}

pub fn hashcode64_i64_real(real: i64) -> u64 {
    if real < 0 {
        real.add(9223372036854775807).add(1) as u64
    } else {
        (real as u64).add(9223372036854775807).add(1)
    }
}

pub fn hashcode32_f32_real(real: f32) -> u32 {
    if real > 0.0 {
        real.to_bits().add(2147483648)
    } else if real < 0.0 {
        2147483648 - real.to_bits() + 2147483648
    } else {
        2147483648
    }
}

pub fn hashcode64_f64_real(real: f64) -> u64 {
    if real > 0.0 {
        real.to_bits().add(9223372036854775808)
    } else if real < 0.0 {
        9223372036854775807 + 9223372036854775807 - real.to_bits() + 2
    } else {
        9223372036854775808
    }
}

pub fn hashcode32_bl_real(real: bool) -> u32 {
    if real {
        1
    } else {
        0
    }
}

pub fn hashcode64_bl_real(real: bool) -> u64 {
    if real {
        1
    } else {
        0
    }
}

pub fn hashcode_enhance(u32: bool, comment: String) -> (u32, u64) {
    if u32 {
        (hashcode32_enhance(comment), 0)
    } else {
        (0, hashcode64(comment))
    }
}
