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

use crate::errors::entrances::GeorgeResult;
use std::path::Path;

pub trait AESkNew {
    /// 生成非对称加密私钥，返回sk字节数组
    fn generate() -> Vec<u8>;
    /// 生成非对称加密私钥，返回sk字符串
    fn generate_string() -> String;
}

pub trait AESkNewStore<T> {
    /// 生成非对称加密私钥，返回sk字节数组
    ///
    /// 并将生成的私钥存储在sk指定文件中
    fn generate(sk_filepath: T) -> GeorgeResult<Vec<u8>>;
    /// 生成非对称加密私钥，返回sk字符串
    ///
    /// 并将生成的私钥存储在sk指定文件中
    fn generate_string(sk_filepath: T) -> GeorgeResult<String>;
}

pub trait AENew {
    /// 生成非对称加密公私钥，返回sk、pk字节数组
    fn generate() -> (Vec<u8>, Vec<u8>);
    /// 生成非对称加密公私钥，返回sk、pk字符串
    fn generate_string() -> (String, String);
}

pub trait AENewStore<T> {
    /// 生成非对称加密公私钥，返回sk、pk字节数组
    ///
    /// 并将生成的公私钥存储在sk、pk指定文件中
    fn generate(sk_filepath: T, pk_filepath: T) -> GeorgeResult<(Vec<u8>, Vec<u8>)>;
    /// 生成非对称加密公私钥，返回sk、pk字符串
    ///
    /// 并将生成的公私钥存储在sk、pk指定文件中
    fn generate_string(sk_filepath: T, pk_filepath: T) -> GeorgeResult<(String, String)>;
}

pub trait AEPkV8s<T> {
    /// 根据私钥生成公钥
    fn generate_pk(sk: T) -> GeorgeResult<Vec<u8>>;
}

pub trait AEPkString<T> {
    /// 根据私钥生成公钥
    fn generate_pk(sk: T) -> GeorgeResult<String>;
}

pub trait AEPkV8sPath {
    /// 根据私钥文件生成公钥
    fn generate_pk<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Vec<u8>>;
}

pub trait AEPkStringPath {
    /// 根据私钥文件生成公钥
    fn generate_pk<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<String>;
}

pub trait AEKeyHex {
    /// 将公/私钥字节数组格式化成字符串
    fn key_encode(key: Vec<u8>) -> String;
    /// 将公/私钥字符串格式化成字节数组
    fn key_decode(key: String) -> GeorgeResult<Vec<u8>>;
}

pub trait AEStoreKey<P> {
    /// 将公/私钥存储在指定文件中
    fn store(key: &[u8], key_filepath: P) -> GeorgeResult<()>;
    /// 将公/私钥存储在指定文件中
    fn store_bytes(key: Vec<u8>, key_filepath: P) -> GeorgeResult<()>;
    /// 将公/私钥存储在指定文件中
    fn store_str(key: &str, key_filepath: P) -> GeorgeResult<()>;
    /// 将公/私钥存储在指定文件中
    fn store_string(key: String, key_filepath: P) -> GeorgeResult<()>;
}

pub trait AELoadKey {
    /// 从指定文件中读取公/私钥
    fn load_from_file<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<Vec<u8>>;
    /// 从指定文件中读取公/私钥
    fn load_string_from_file<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<String>;
}

pub trait AESign<M, N> {
    /// 签名msg，返回签名结果字节数组
    ///
    /// msg 待签名数据
    ///
    /// sk、pk 签名使用公私钥
    fn sign(msg: M, sk: N, pk: N) -> GeorgeResult<Vec<u8>>;
    /// 签名msg，返回签名结果字符串
    ///
    /// msg 待签名数据
    ///
    /// sk、pk 签名使用公私钥
    fn sign_string(msg: M, sk: N, pk: N) -> GeorgeResult<String>;
}

pub trait AESignPath<T> {
    /// 签名msg，返回签名结果字节数组
    ///
    /// msg 待签名数据
    ///
    /// sk、pk 签名使用公私钥文件
    fn sign<P: AsRef<Path>>(msg: T, sk_filepath: P, pk_filepath: P) -> GeorgeResult<Vec<u8>>;
    /// 签名msg，返回签名结果字符串
    ///
    /// msg 待签名数据
    ///
    /// sk、pk 签名使用公私钥文件
    fn sign_string<P: AsRef<Path>>(msg: T, sk_filepath: P, pk_filepath: P) -> GeorgeResult<String>;
}

pub trait AEVerify<M, N, O> {
    /// 验签msg
    fn verify(msg: M, pk: N, der: O) -> GeorgeResult<bool>;
}

pub trait AEVerifyPath<M, N> {
    /// 验签msg
    fn verify<P: AsRef<Path>>(msg: M, sk_filepath: P, der: N) -> GeorgeResult<bool>;
}
