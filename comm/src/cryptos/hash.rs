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

use crc32fast::Hasher;
use crypto::digest::Digest;
use crypto::md5::Md5;

use crate::errors::entrances::{err_strings, GeorgeResult};
use crate::strings::{StringHandler, Strings};
use std::ops::Add;

pub fn md5(comment: String) -> String {
    let mut md5_handler = Md5::new();
    md5_handler.input_str(comment.as_str());
    md5_handler.result_str()
}

pub fn md516(comment: String) -> String {
    Strings::subs(md5(comment), 8, 24)
}

pub fn hashcode32(comment: &[u8]) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(comment);
    hasher.finalize()
}

pub fn hashcode32_enhance(comment: String) -> u32 {
    return match comment.parse::<u32>() {
        Ok(su32) => su32,
        Err(_err) => hashcode32(comment.as_bytes()),
    };
}

pub fn hashcode64(comment: &[u8]) -> u64 {
    let mut c = crc64fast::Digest::new();
    c.write(comment);
    c.sum64()
}

pub fn hashcode64_str(comment: String) -> u64 {
    hashcode64(comment.as_bytes())
}

pub fn hashcode64_u64(comment: String) -> GeorgeResult<u64> {
    match comment.parse::<u64>() {
        Ok(su64) => Ok(su64),
        Err(err) => Err(err_strings(format!("{} parse to u64", comment), err)),
    }
}

pub fn hashcode64_i64(comment: String) -> GeorgeResult<u64> {
    match comment.parse::<i64>() {
        Ok(real) => Ok(hashcode64_i64_real(real)),
        Err(err) => Err(err_strings(format!("{} parse to i64", comment), err)),
    }
}

pub fn hashcode64_f64(comment: String) -> GeorgeResult<u64> {
    match comment.parse::<f64>() {
        Ok(real) => Ok(hashcode64_f64_real(real)),
        Err(err) => Err(err_strings(format!("{} parse to f64", comment), err)),
    }
}

pub fn hashcode64_bl(comment: String) -> GeorgeResult<u64> {
    match comment.parse::<bool>() {
        Ok(real) => Ok(hashcode64_bl_real(real)),
        Err(err) => Err(err_strings(format!("{} parse to bool", comment), err)),
    }
}

pub fn hashcode64_i64_real(real: i64) -> u64 {
    if real < 0 {
        real.add(9223372036854775807).add(1) as u64
    } else {
        (real as u64).add(9223372036854775807).add(1)
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
        (0, hashcode64_str(comment))
    }
}
