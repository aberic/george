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

use std::fs::read;

use openssl::ec::{EcGroup, EcKey};
use openssl::nid::Nid;
use openssl::pkey::{Private, Public};

use crate::errors::entrances::err_str_enhance;
use crate::errors::entrances::GeorgeResult;
use crate::io::writer::write;

/// 生成ECDSA私钥，默认PRIME256V1
pub fn generate_sk() -> GeorgeResult<Vec<u8>> {
    match EcGroup::from_curve_name(Nid::X9_62_PRIME256V1) {
        Ok(group) => match EcKey::generate(&group) {
            Ok(key) => match key.private_key_to_pem() {
                Ok(v8s) => Ok(v8s),
                Err(err) => Err(err_str_enhance("private_key_to_pem", err.to_string())),
            },
            Err(err) => Err(err_str_enhance("generate", err.to_string())),
        },
        Err(err) => Err(err_str_enhance("from_curve_name", err.to_string())),
    }
}

/// 生成ECDSA私钥并将私钥存储指定文件
///
/// force 如果已存在，是否删除重写
pub fn generate_sk_in_file(filepath: String, force: bool) -> GeorgeResult<Vec<u8>> {
    match generate_sk() {
        Ok(u8s) => write(filepath, u8s.clone(), force),
        Err(err) => Err(err_str_enhance("generate_sk", err.to_string())),
    }
}

/// 生成ECDSA私钥并将私钥存储指定文件
///
/// force 如果已存在，是否删除重写
pub fn generate_sk_in_files(filepath: &str, force: bool) -> GeorgeResult<Vec<u8>> {
    generate_sk_in_file(filepath.to_string(), force)
}

/// 读取ECDSA私钥
pub fn load_sk(sk: Vec<u8>) -> GeorgeResult<EcKey<Private>> {
    match EcKey::private_key_from_pem(sk.as_slice()) {
        Ok(key) => Ok(key),
        Err(err) => Err(err_str_enhance("private_key_from_pem", err.to_string())),
    }
}

/// 读取ECDSA私钥
pub fn load_sk_file(filepath: String) -> GeorgeResult<EcKey<Private>> {
    match read(filepath) {
        Ok(u8s) => load_sk(u8s),
        Err(err) => Err(err_str_enhance("read", err.to_string())),
    }
}

/// 生成ECDSA公钥
pub fn generate_pk_from_sk(sk: EcKey<Private>) -> GeorgeResult<Vec<u8>> {
    match sk.public_key_to_pem() {
        Ok(u8s) => Ok(u8s),
        Err(err) => Err(err_str_enhance("public_key_to_pem", err.to_string())),
    }
}

/// 生成ECDSA公钥
pub fn generate_pk_from_sk_bytes(sk: Vec<u8>) -> GeorgeResult<Vec<u8>> {
    match load_sk(sk) {
        Ok(key) => generate_pk_from_sk(key),
        Err(err) => Err(err_str_enhance("load_sk", err.to_string())),
    }
}

/// 生成ECDSA公钥
pub fn generate_pk_from_sk_file(filepath: String) -> GeorgeResult<Vec<u8>> {
    match load_sk_file(filepath) {
        Ok(key) => generate_pk_from_sk(key),
        Err(err) => Err(err_str_enhance("load_sk_file", err.to_string())),
    }
}

/// 生成ECDSA公钥并将私钥存储指定文件
///
/// force 如果已存在，是否删除重写
pub fn generate_pk_in_file_from_sk(
    sk: EcKey<Private>,
    filepath: String,
    force: bool,
) -> GeorgeResult<Vec<u8>> {
    match generate_pk_from_sk(sk) {
        Ok(u8s) => write(filepath, u8s.clone(), force),
        Err(err) => Err(err_str_enhance("generate_pk_from_sk", err.to_string())),
    }
}

/// 生成ECDSA公钥并将私钥存储指定文件
///
/// force 如果已存在，是否删除重写
pub fn generate_pk_in_file_from_sk_bytes(
    sk: Vec<u8>,
    filepath: String,
    force: bool,
) -> GeorgeResult<Vec<u8>> {
    match generate_pk_from_sk_bytes(sk) {
        Ok(u8s) => write(filepath, u8s.clone(), force),
        Err(err) => Err(err_str_enhance(
            "generate_pk_from_sk_bytes",
            err.to_string(),
        )),
    }
}

/// 生成ECDSA公钥并将私钥存储指定文件
///
/// force 如果已存在，是否删除重写
pub fn generate_pk_in_file_from_sk_file(
    sk_filepath: String,
    pk_filepath: String,
    force: bool,
) -> GeorgeResult<Vec<u8>> {
    match generate_pk_from_sk_file(sk_filepath) {
        Ok(u8s) => write(pk_filepath, u8s.clone(), force),
        Err(err) => Err(err_str_enhance("generate_pk_from_sk_file", err.to_string())),
    }
}

/// 读取ECDSA公钥
pub fn load_pk(pk: Vec<u8>) -> GeorgeResult<EcKey<Public>> {
    match EcKey::public_key_from_pem(pk.as_slice()) {
        Ok(key) => Ok(key),
        Err(err) => Err(err_str_enhance("private_key_from_pem", err.to_string())),
    }
}

/// 读取ECDSA公钥
pub fn load_pk_file(filepath: String) -> GeorgeResult<EcKey<Public>> {
    match read(filepath) {
        Ok(u8s) => load_pk(u8s),
        Err(err) => Err(err_str_enhance("read", err.to_string())),
    }
}
