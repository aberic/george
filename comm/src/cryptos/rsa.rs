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

use openssl::pkey::{PKey, Private, Public};
use openssl::rsa::{Padding, Rsa};

use crate::errors::entrances::err_str_enhance;
use crate::errors::entrances::GeorgeResult;
use crate::io::writer::write;

/// 生成RSA私钥
///
/// bits 私钥位数，默认提供PKCS8
///
/// Serializes the private key to a PEM-encoded PKCS#8 EncryptedPrivateKeyInfo structure.
///
/// The output will have a header of `-----BEGIN ENCRYPTED PRIVATE KEY-----`.
///
/// This corresponds to [`PEM_write_bio_PKCS8PrivateKey`].
///
/// [`PEM_write_bio_PKCS8PrivateKey`]: https://www.openssl.org/docs/man1.0.2/crypto/PEM_write_bio_PKCS8PrivateKey.html
/// <p>
///
/// # Return
/// bytes，可以通过string(bytes)的方式查阅
pub fn generate_sk(bits: u32) -> GeorgeResult<Vec<u8>> {
    match Rsa::generate(bits) {
        Ok(rsa) => match PKey::from_rsa(rsa) {
            Ok(key) => match key.private_key_to_pem_pkcs8() {
                Ok(res) => Ok(res),
                Err(err) => Err(err_str_enhance("private_key_to_pem_pkcs8", err.to_string())),
            },
            Err(err) => Err(err_str_enhance("from_rsa", err.to_string())),
        },
        Err(err) => Err(err_str_enhance("generate", err.to_string())),
    }
}

/// 生成RSA私钥并将私钥存储指定文件
///
/// bits 私钥位数，默认提供PKCS8
///
/// force 如果已存在，是否删除重写
pub fn generate_sk_in_file(bits: u32, filepath: String, force: bool) -> GeorgeResult<Vec<u8>> {
    match generate_sk(bits) {
        Ok(u8s) => write(filepath, u8s.clone(), force),
        Err(err) => Err(err_str_enhance("generate_sk", err.to_string())),
    }
}

/// 生成RSA私钥并将私钥存储指定文件
///
/// bits 私钥位数，默认提供PKCS8
///
/// force 如果已存在，是否删除重写
pub fn generate_sk_in_files(bits: u32, filepath: &str, force: bool) -> GeorgeResult<Vec<u8>> {
    generate_sk_in_file(bits, filepath.to_string(), force)
}

/// 读取RSA私钥
pub fn load_sk(sk: Vec<u8>) -> GeorgeResult<PKey<Private>> {
    match PKey::private_key_from_pem(sk.as_slice()) {
        Ok(key) => Ok(key),
        Err(err) => Err(err_str_enhance("private_key_from_pem", err.to_string())),
    }
}

/// 读取RSA私钥
pub fn load_sk_file(filepath: String) -> GeorgeResult<PKey<Private>> {
    match read(filepath) {
        Ok(u8s) => load_sk(u8s),
        Err(err) => Err(err_str_enhance("read", err.to_string())),
    }
}

/// 生成RSA公钥
pub fn generate_pk_from_sk(sk: PKey<Private>) -> GeorgeResult<Vec<u8>> {
    match sk.public_key_to_pem() {
        Ok(u8s) => Ok(u8s),
        Err(err) => Err(err_str_enhance("public_key_to_pem", err.to_string())),
    }
}

/// 生成RSA公钥
pub fn generate_pk_from_sk_bytes(sk: Vec<u8>) -> GeorgeResult<Vec<u8>> {
    match load_sk(sk) {
        Ok(key) => generate_pk_from_sk(key),
        Err(err) => Err(err_str_enhance("load_sk", err.to_string())),
    }
}

/// 生成RSA公钥
pub fn generate_pk_from_sk_file(filepath: String) -> GeorgeResult<Vec<u8>> {
    match load_sk_file(filepath) {
        Ok(key) => generate_pk_from_sk(key),
        Err(err) => Err(err_str_enhance("load_sk_file", err.to_string())),
    }
}

/// 生成RSA公钥并将私钥存储指定文件
///
/// force 如果已存在，是否删除重写
pub fn generate_pk_in_file_from_sk(
    sk: PKey<Private>,
    filepath: String,
    force: bool,
) -> GeorgeResult<Vec<u8>> {
    match generate_pk_from_sk(sk) {
        Ok(u8s) => write(filepath, u8s.clone(), force),
        Err(err) => Err(err_str_enhance("generate_pk_from_sk", err.to_string())),
    }
}

/// 生成RSA公钥并将私钥存储指定文件
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

/// 生成RSA公钥并将私钥存储指定文件
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

/// 读取RSA公钥
pub fn load_pk(pk: Vec<u8>) -> GeorgeResult<PKey<Public>> {
    match PKey::public_key_from_pem(pk.as_slice()) {
        Ok(key) => Ok(key),
        Err(err) => Err(err_str_enhance("private_key_from_pem", err.to_string())),
    }
}

/// 读取RSA公钥
pub fn load_pk_file(filepath: String) -> GeorgeResult<PKey<Public>> {
    match read(filepath) {
        Ok(u8s) => load_pk(u8s),
        Err(err) => Err(err_str_enhance("read", err.to_string())),
    }
}

pub fn encrypt_sk(sk: Rsa<Private>, data: &[u8]) -> GeorgeResult<Vec<u8>> {
    let mut emesg = vec![0; sk.size() as usize];
    match sk.private_encrypt(data, &mut emesg, Padding::PKCS1) {
        Ok(_) => Ok(emesg),
        Err(err) => Err(err_str_enhance("private_encrypt", err.to_string())),
    }
}

pub fn decrypt_sk(sk: Rsa<Private>, data: &[u8]) -> GeorgeResult<Vec<u8>> {
    let mut emesg = vec![0; sk.size() as usize];
    match sk.private_decrypt(data, &mut emesg, Padding::PKCS1) {
        Ok(_) => Ok(emesg),
        Err(err) => Err(err_str_enhance("private_decrypt", err.to_string())),
    }
}

pub fn encrypt_sk_bytes(sk_bytes: Vec<u8>, data: String) -> GeorgeResult<Vec<u8>> {
    match load_sk(sk_bytes) {
        Ok(sk_key) => match sk_key.rsa() {
            Ok(sk) => encrypt_sk(sk, data.as_bytes()),
            Err(err) => Err(err_str_enhance("rsa", err.to_string())),
        },
        Err(err) => Err(err_str_enhance("load_sk", err.to_string())),
    }
}

pub fn encrypt_sk_file(filepath: String, data: String) -> GeorgeResult<Vec<u8>> {
    match load_sk_file(filepath) {
        Ok(sk_key) => match sk_key.rsa() {
            Ok(sk) => encrypt_sk(sk, data.as_bytes()),
            Err(err) => Err(err_str_enhance("rsa", err.to_string())),
        },
        Err(err) => Err(err_str_enhance("load_sk_file", err.to_string())),
    }
}

pub fn encrypt_pk(pk: Rsa<Public>, data: &[u8]) -> GeorgeResult<Vec<u8>> {
    let mut emesg = vec![0; pk.size() as usize];
    match pk.public_encrypt(data, &mut emesg, Padding::PKCS1) {
        Ok(_) => Ok(emesg),
        Err(err) => Err(err_str_enhance("public_encrypt", err.to_string())),
    }
}

pub fn encrypt_pk_bytes(pk_bytes: Vec<u8>, data: String) -> GeorgeResult<Vec<u8>> {
    match load_pk(pk_bytes) {
        Ok(pk_key) => match pk_key.rsa() {
            Ok(pk) => encrypt_pk(pk, data.as_bytes()),
            Err(err) => Err(err_str_enhance("rsa", err.to_string())),
        },
        Err(err) => Err(err_str_enhance("load_pk", err.to_string())),
    }
}

pub fn encrypt_pk_file(filepath: String, data: String) -> GeorgeResult<Vec<u8>> {
    match load_pk_file(filepath) {
        Ok(pk_key) => match pk_key.rsa() {
            Ok(pk) => encrypt_pk(pk, data.as_bytes()),
            Err(err) => Err(err_str_enhance("rsa", err.to_string())),
        },
        Err(err) => Err(err_str_enhance("load_pk_file", err.to_string())),
    }
}
