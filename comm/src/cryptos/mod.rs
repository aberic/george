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

use libsm::sm2::ecc::Point;
use libsm::sm2::signature::SigCtx;
use libsm::sm4::Cipher;
use num_bigint::BigUint;
use openssl::ec::EcKey;
use openssl::pkey::{PKey, Private, Public};
use openssl::rsa::Rsa;
use openssl::x509::X509;

pub mod base64;
mod base64_test;
pub mod ca;
mod ca_test;
pub mod ecdsa;
mod ecdsa_test;
pub mod hash;
mod hash_test;
pub mod hex;
mod hex_test;
pub mod homomorphic;
mod homomorphic_test;
pub mod key;
pub mod rsa;
mod rsa_test;
mod rust_tls;
pub mod sm2;
mod sm2_test;
pub mod sm4;
mod sm4_test;

#[derive(Debug, Clone)]
pub struct Base64;

#[derive(Debug, Clone)]
pub struct Hex;

#[derive(Debug, Clone)]
pub struct Hash;

pub struct Key;

pub struct RSA {
    // /// 私钥位数
    // bits: u32,
    // /// 指定的密码算法
    // ///
    // /// Cipher Represents a particular cipher algorithm.
    // ///
    // /// See OpenSSL doc at [`EVP_EncryptInit`] for more information on each algorithms.
    // ///
    // /// [`EVP_EncryptInit`]: https://www.openssl.org/docs/man1.1.0/crypto/EVP_EncryptInit.html
    // cipher: Cipher,
    sk: PKey<Private>,
    pk: PKey<Public>,
    rsa_sk: Rsa<Private>,
    rsa_pk: Rsa<Public>,
}

pub struct ECDSA {
    sk: PKey<Private>,
    pk: PKey<Public>,
    sk_ec: EcKey<Private>,
    pk_ec: EcKey<Public>,
}

/// 字节数组与字符串通过Base64转换
pub struct SM2 {
    ctx: SigCtx,
    sk: BigUint,
    pk: Point,
}

pub struct SM4 {
    key: [u8; 16],
    iv: [u8; 16],
    sm4_cipher_mode: Cipher,
}

pub struct Cert {
    pub x509: X509,
}
