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

use crate::cryptos::hex::{Hex, HexEncoder};
use crate::errors::entrances::{Errs, GeorgeResult};
use crate::strings::{StringHandler, Strings};
use openssl::hash::{Hasher, MessageDigest};
use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Hash;

pub trait HashMD5Handler<T> {
    fn digest(md: MessageDigest, comment: T) -> GeorgeResult<String>;

    fn md5(comment: T) -> String;

    fn md516(comment: T) -> String;

    fn sm3(comment: T) -> String;

    fn sha1(comment: T) -> String;

    fn sha256(comment: T) -> String;
}

pub trait HashCRCHandler<T> {
    fn crc32(comment: T) -> u32;

    fn crc64(comment: T) -> u64;
}

pub trait HashCRCTypeHandler {
    fn crc32_string(comment: String) -> GeorgeResult<u32>;

    fn crc32_bool(comment: String) -> GeorgeResult<u32>;

    fn crc32_u32(comment: String) -> GeorgeResult<u32>;

    fn crc32_f32(comment: String) -> GeorgeResult<u32>;

    fn crc32_i32(comment: String) -> GeorgeResult<u32>;

    fn crc64_string(comment: String) -> GeorgeResult<u64>;

    fn crc64_bool(comment: String) -> GeorgeResult<u64>;

    fn crc64_u32(comment: String) -> GeorgeResult<u64>;

    fn crc64_u64(comment: String) -> GeorgeResult<u64>;

    fn crc64_f32(comment: String) -> GeorgeResult<u64>;

    fn crc64_f64(comment: String) -> GeorgeResult<u64>;

    fn crc64_i32(comment: String) -> GeorgeResult<u64>;

    fn crc64_i64(comment: String) -> GeorgeResult<u64>;
}

impl HashMD5Handler<&[u8]> for Hash {
    fn digest(md: MessageDigest, comment: &[u8]) -> GeorgeResult<String> {
        digest(md, comment)
    }

    fn md5(comment: &[u8]) -> String {
        md5_u8s(comment)
    }

    fn md516(comment: &[u8]) -> String {
        md516_u8s(comment)
    }

    fn sm3(comment: &[u8]) -> String {
        digest(MessageDigest::sm3(), comment).unwrap()
    }

    fn sha1(comment: &[u8]) -> String {
        digest(MessageDigest::sha1(), comment).unwrap()
    }

    fn sha256(comment: &[u8]) -> String {
        digest(MessageDigest::sha256(), comment).unwrap()
    }
}

impl HashMD5Handler<Vec<u8>> for Hash {
    fn digest(md: MessageDigest, comment: Vec<u8>) -> GeorgeResult<String> {
        digest(md, comment.as_slice())
    }

    fn md5(comment: Vec<u8>) -> String {
        md5_u8s(comment.as_slice())
    }

    fn md516(comment: Vec<u8>) -> String {
        md516_u8s(comment.as_slice())
    }

    fn sm3(comment: Vec<u8>) -> String {
        digest(MessageDigest::sm3(), comment.as_slice()).unwrap()
    }

    fn sha1(comment: Vec<u8>) -> String {
        digest(MessageDigest::sha1(), comment.as_slice()).unwrap()
    }

    fn sha256(comment: Vec<u8>) -> String {
        digest(MessageDigest::sha256(), comment.as_slice()).unwrap()
    }
}

impl HashMD5Handler<String> for Hash {
    fn digest(md: MessageDigest, comment: String) -> GeorgeResult<String> {
        digest(md, comment.as_bytes())
    }

    fn md5(comment: String) -> String {
        md5(comment)
    }

    fn md516(comment: String) -> String {
        md516(comment)
    }

    fn sm3(comment: String) -> String {
        digest(MessageDigest::sm3(), comment.as_bytes()).unwrap()
    }

    fn sha1(comment: String) -> String {
        digest(MessageDigest::sha1(), comment.as_bytes()).unwrap()
    }

    fn sha256(comment: String) -> String {
        digest(MessageDigest::sha256(), comment.as_bytes()).unwrap()
    }
}

impl HashCRCHandler<&[u8]> for Hash {
    fn crc32(comment: &[u8]) -> u32 {
        hashcode32(comment)
    }

    fn crc64(comment: &[u8]) -> u64 {
        hashcode64(comment)
    }
}

impl HashCRCHandler<Vec<u8>> for Hash {
    fn crc32(comment: Vec<u8>) -> u32 {
        hashcode32(comment.as_slice())
    }

    fn crc64(comment: Vec<u8>) -> u64 {
        hashcode64(comment.as_slice())
    }
}

impl HashCRCHandler<String> for Hash {
    fn crc32(comment: String) -> u32 {
        hashcode32_string(comment)
    }

    fn crc64(comment: String) -> u64 {
        hashcode64_string(comment)
    }
}

impl HashCRCHandler<bool> for Hash {
    fn crc32(comment: bool) -> u32 {
        hashcode32_bool_real(comment)
    }

    fn crc64(comment: bool) -> u64 {
        hashcode64_bool_real(comment)
    }
}

impl HashCRCHandler<i32> for Hash {
    fn crc32(comment: i32) -> u32 {
        hashcode32_i32_real(comment)
    }

    fn crc64(comment: i32) -> u64 {
        hashcode64_i64_real(comment as i64)
    }
}

impl HashCRCHandler<i64> for Hash {
    fn crc32(_comment: i64) -> u32 {
        0
    }

    fn crc64(comment: i64) -> u64 {
        hashcode64_i64_real(comment)
    }
}

impl HashCRCHandler<f32> for Hash {
    fn crc32(comment: f32) -> u32 {
        hashcode32_f32_real(comment)
    }

    fn crc64(comment: f32) -> u64 {
        hashcode64_f64_real(comment as f64)
    }
}

impl HashCRCHandler<f64> for Hash {
    fn crc32(_comment: f64) -> u32 {
        0
    }

    fn crc64(comment: f64) -> u64 {
        hashcode64_f64_real(comment)
    }
}

impl HashCRCTypeHandler for Hash {
    fn crc32_string(comment: String) -> GeorgeResult<u32> {
        Ok(hashcode32_string(comment))
    }

    fn crc32_bool(comment: String) -> GeorgeResult<u32> {
        hashcode32_bl(comment)
    }

    fn crc32_u32(comment: String) -> GeorgeResult<u32> {
        hashcode32_u32(comment)
    }

    fn crc32_f32(comment: String) -> GeorgeResult<u32> {
        hashcode32_f32(comment)
    }

    fn crc32_i32(comment: String) -> GeorgeResult<u32> {
        hashcode32_i32(comment)
    }

    fn crc64_string(comment: String) -> GeorgeResult<u64> {
        Ok(hashcode64_string(comment))
    }

    fn crc64_bool(comment: String) -> GeorgeResult<u64> {
        hashcode64_bl(comment)
    }

    fn crc64_u32(comment: String) -> GeorgeResult<u64> {
        hashcode64_u64(comment)
    }

    fn crc64_u64(comment: String) -> GeorgeResult<u64> {
        hashcode64_u64(comment)
    }

    fn crc64_f32(comment: String) -> GeorgeResult<u64> {
        hashcode64_f64(comment)
    }

    fn crc64_f64(comment: String) -> GeorgeResult<u64> {
        hashcode64_f64(comment)
    }

    fn crc64_i32(comment: String) -> GeorgeResult<u64> {
        hashcode64_i64(comment)
    }

    fn crc64_i64(comment: String) -> GeorgeResult<u64> {
        hashcode64_i64(comment)
    }
}

fn digest(md: MessageDigest, comment: &[u8]) -> GeorgeResult<String> {
    match Hasher::new(md) {
        Ok(mut hasher) => match hasher.update(comment) {
            Ok(()) => match hasher.finish() {
                Ok(d_bytes) => Ok(Hex::encode(d_bytes.to_vec())),
                Err(err) => Err(Errs::strs("hasher finish", err)),
            },
            Err(err) => Err(Errs::strs("hasher update", err)),
        },
        Err(err) => Err(Errs::strs("hasher new", err)),
    }
}

fn md5(comment: String) -> String {
    md5_u8s(comment.as_bytes())
}

fn md5_u8s(comment: &[u8]) -> String {
    let mut hash = Hasher::new(MessageDigest::md5()).unwrap();
    hash.update(comment).unwrap();
    let res = hash.finish().unwrap();
    Hex::encode(res.to_vec())
}

fn md516(comment: String) -> String {
    Strings::subs(md5(comment), 8, 24)
}

fn md516_u8s(comment: &[u8]) -> String {
    Strings::subs(md5_u8s(comment), 8, 24)
}

fn hashcode32(comment: &[u8]) -> u32 {
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(comment);
    hasher.finalize()
}

fn hashcode32_string(comment: String) -> u32 {
    hashcode32(comment.as_bytes())
}

fn hashcode64(comment: &[u8]) -> u64 {
    let mut c = crc64fast::Digest::new();
    c.write(comment);
    c.sum64().add(1)
}

fn hashcode64_string(comment: String) -> u64 {
    hashcode64(comment.as_bytes())
}

fn hashcode32_u32(comment: String) -> GeorgeResult<u32> {
    match comment.parse::<u32>() {
        Ok(real) => Ok(real),
        Err(err) => Err(Errs::strings(format!("{} parse to u32", comment), err)),
    }
}

fn hashcode64_u64(comment: String) -> GeorgeResult<u64> {
    match comment.parse::<u64>() {
        Ok(real) => Ok(real.add(1)),
        Err(err) => Err(Errs::strings(format!("{} parse to u64", comment), err)),
    }
}

fn hashcode32_i32(comment: String) -> GeorgeResult<u32> {
    match comment.parse::<i32>() {
        Ok(real) => Ok(hashcode32_i32_real(real)),
        Err(err) => Err(Errs::strings(format!("{} parse to i32", comment), err)),
    }
}

fn hashcode64_i64(comment: String) -> GeorgeResult<u64> {
    match comment.parse::<i64>() {
        Ok(real) => Ok(hashcode64_i64_real(real)),
        Err(err) => Err(Errs::strings(format!("{} parse to i64", comment), err)),
    }
}

fn hashcode32_f32(comment: String) -> GeorgeResult<u32> {
    match comment.parse::<f32>() {
        Ok(real) => Ok(hashcode32_f32_real(real)),
        Err(err) => Err(Errs::strings(format!("{} parse to f32", comment), err)),
    }
}

fn hashcode64_f64(comment: String) -> GeorgeResult<u64> {
    match comment.parse::<f64>() {
        Ok(real) => Ok(hashcode64_f64_real(real)),
        Err(err) => Err(Errs::strings(format!("{} parse to f64", comment), err)),
    }
}

fn hashcode32_bl(comment: String) -> GeorgeResult<u32> {
    match comment.parse::<bool>() {
        Ok(real) => Ok(hashcode32_bool_real(real)),
        Err(err) => Err(Errs::strings(format!("{} parse to bool", comment), err)),
    }
}

fn hashcode64_bl(comment: String) -> GeorgeResult<u64> {
    match comment.parse::<bool>() {
        Ok(real) => Ok(hashcode64_bool_real(real)),
        Err(err) => Err(Errs::strings(format!("{} parse to bool", comment), err)),
    }
}

fn hashcode32_i32_real(real: i32) -> u32 {
    if real < 0 {
        real.add(2147483647).add(1) as u32
    } else {
        (real as u32).add(2147483648)
    }
}

fn hashcode64_i64_real(real: i64) -> u64 {
    hashcode64_i64_trans(real).add(1)
}

fn hashcode64_i64_trans(real: i64) -> u64 {
    if real < 0 {
        real.add(9223372036854775807).add(1) as u64
    } else {
        (real as u64).add(9223372036854775807).add(1)
    }
}

fn hashcode32_f32_real(real: f32) -> u32 {
    if real > 0.0 {
        real.to_bits().add(2147483648)
    } else if real < 0.0 {
        2147483648 - real.to_bits() + 2147483648
    } else {
        2147483648
    }
}

fn hashcode64_f64_real(real: f64) -> u64 {
    if real > 0.0 {
        real.to_bits().add(9223372036854775809)
    } else if real < 0.0 {
        18446744073709551615 - real.to_bits() + 2
    } else {
        9223372036854775809
    }
}

fn hashcode32_bool_real(real: bool) -> u32 {
    if real {
        1
    } else {
        0
    }
}

fn hashcode64_bool_real(real: bool) -> u64 {
    if real {
        2
    } else {
        1
    }
}
