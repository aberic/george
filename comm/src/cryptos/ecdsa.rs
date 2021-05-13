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

use openssl::ec::{EcGroup, EcKey, EcPoint, PointConversionForm};
use openssl::nid::Nid;
use openssl::pkey::{Private, Public};

use crate::cryptos::base64::{Base64, Base64Encoder, Basee64Decoder};
use crate::cryptos::hex::{Hex, HexDecoder, HexEncoder};
use crate::errors::entrances::err_strs;
use crate::errors::entrances::GeorgeResult;
use crate::io::file::{Filer, FilerWriter};
use crate::strings::{StringHandler, Strings};
use openssl::bn::{BigNum, BigNumContext};
use openssl::ecdsa::EcdsaSig;

pub struct ECDSA {
    sk: EcKey<Private>,
    pk: EcKey<Public>,
}

/// base method
impl ECDSA {
    /// 生成ECDSA对象，默认PRIME256V1
    pub fn new() -> GeorgeResult<ECDSA> {
        let (sk, pk) = generate()?;
        Ok(ECDSA { sk, pk })
    }

    /// 生成ECDSA对象
    ///
    /// nid OpenSSL对象的数字标识符。
    /// OpenSSL中的对象可以有短名称、长名称和数字标识符(NID)。为方便起见，对象通常在源代码中使用这些数字标识符表示。
    /// 用户通常不需要创建新的' Nid '。
    pub fn new_nid(nid: Nid) -> GeorgeResult<ECDSA> {
        let (sk, pk) = generate_nid(nid)?;
        Ok(ECDSA { sk, pk })
    }

    /// 生成ECDSA对象
    pub fn from(sk: EcKey<Private>) -> GeorgeResult<ECDSA> {
        let (sk, pk) = generate_pk_from_sk(sk)?;
        Ok(ECDSA { sk, pk })
    }

    /// 生成ECDSA对象
    pub fn from_key(sk: EcKey<Private>, pk: EcKey<Public>) -> ECDSA {
        ECDSA { sk, pk }
    }

    /// 生成ECDSA对象
    pub fn from_hex(sk: String, pk: String) -> GeorgeResult<ECDSA> {
        from_bytes(Hex::decode(sk)?, Hex::decode(pk)?)
    }

    /// 生成ECDSA对象
    pub fn from_hex_nid(sk: String, pk: String, nid: Nid) -> GeorgeResult<ECDSA> {
        from_bytes_nid(Hex::decode(sk)?, Hex::decode(pk)?, nid)
    }

    /// 生成ECDSA对象
    pub fn from_base64(sk: String, pk: String) -> GeorgeResult<ECDSA> {
        from_bytes(Base64::decode(sk)?, Base64::decode(pk)?)
    }

    /// 生成ECDSA对象
    pub fn from_base64_nid(sk: String, pk: String, nid: Nid) -> GeorgeResult<ECDSA> {
        from_bytes_nid(Base64::decode(sk)?, Base64::decode(pk)?, nid)
    }

    /// 生成ECDSA对象
    pub fn from_pem(sk: Vec<u8>, pk: Vec<u8>) -> GeorgeResult<ECDSA> {
        match EcKey::private_key_from_pem(&sk) {
            Ok(sk) => match EcKey::public_key_from_pem(&pk) {
                Ok(pk) => Ok(ECDSA { sk, pk }),
                Err(err) => Err(err_strs("EcKey public_key_from_pem", err)),
            },
            Err(err) => Err(err_strs("EcKey private_key_from_pem", err)),
        }
    }

    /// 生成ECDSA对象
    pub fn from_der(sk: Vec<u8>, pk: Vec<u8>) -> GeorgeResult<ECDSA> {
        match EcKey::private_key_from_der(&sk) {
            Ok(sk) => match EcKey::public_key_from_der(&pk) {
                Ok(pk) => Ok(ECDSA { sk, pk }),
                Err(err) => Err(err_strs("EcKey public_key_from_der", err)),
            },
            Err(err) => Err(err_strs("EcKey private_key_from_der", err)),
        }
    }

    pub fn sk(&self) -> EcKey<Private> {
        self.sk.clone()
    }

    pub fn pk(&self) -> EcKey<Public> {
        self.pk.clone()
    }
}

/// fmt method
impl ECDSA {
    pub fn sk_hex(&self) -> String {
        Hex::encode(self.sk.private_key().to_vec())
    }
    pub fn sk_base64(&self) -> String {
        Base64::encode(self.sk.private_key().to_vec())
    }

    pub fn sk_pem(&self) -> GeorgeResult<Vec<u8>> {
        match self.sk.private_key_to_pem() {
            Ok(res) => Ok(res),
            Err(err) => Err(err_strs("private_key_to_pem", err)),
        }
    }

    pub fn sk_pem_str(&self) -> GeorgeResult<String> {
        match self.sk.private_key_to_pem() {
            Ok(res) => Strings::from_utf8(res),
            Err(err) => Err(err_strs("private_key_to_pem", err)),
        }
    }

    pub fn sk_pem_hex(&self) -> GeorgeResult<String> {
        match self.sk.private_key_to_pem() {
            Ok(res) => Ok(Hex::encode(res)),
            Err(err) => Err(err_strs("private_key_to_pem", err)),
        }
    }

    pub fn sk_pem_base64(&self) -> GeorgeResult<String> {
        match self.sk.private_key_to_pem() {
            Ok(res) => Ok(Base64::encode(res)),
            Err(err) => Err(err_strs("private_key_to_pem", err)),
        }
    }

    pub fn sk_der(&self) -> GeorgeResult<Vec<u8>> {
        match self.sk.private_key_to_der() {
            Ok(res) => Ok(res),
            Err(err) => Err(err_strs("private_key_to_pem", err)),
        }
    }

    pub fn sk_der_hex(&self) -> GeorgeResult<String> {
        match self.sk.private_key_to_der() {
            Ok(res) => Ok(Hex::encode(res)),
            Err(err) => Err(err_strs("private_key_to_pem", err)),
        }
    }

    pub fn sk_der_base64(&self) -> GeorgeResult<String> {
        match self.sk.private_key_to_der() {
            Ok(res) => Ok(Base64::encode(res)),
            Err(err) => Err(err_strs("private_key_to_pem", err)),
        }
    }

    pub fn pk_hex(&self) -> GeorgeResult<String> {
        let mut ctx = BigNumContext::new().unwrap();
        match self.pk.public_key().to_bytes(
            &self.sk.group(),
            PointConversionForm::COMPRESSED,
            &mut ctx,
        ) {
            Ok(res) => Ok(Hex::encode(res)),
            Err(err) => Err(err_strs("public_key to_bytes", err)),
        }
    }

    pub fn pk_base64(&self) -> GeorgeResult<String> {
        let mut ctx = BigNumContext::new().unwrap();
        match self.pk.public_key().to_bytes(
            &self.sk.group(),
            PointConversionForm::COMPRESSED,
            &mut ctx,
        ) {
            Ok(res) => Ok(Base64::encode(res)),
            Err(err) => Err(err_strs("public_key to_bytes", err)),
        }
    }

    pub fn pk_pem(&self) -> GeorgeResult<Vec<u8>> {
        match self.pk.public_key_to_pem() {
            Ok(res) => Ok(res),
            Err(err) => Err(err_strs("public_key_to_pem", err)),
        }
    }

    pub fn pk_pem_str(&self) -> GeorgeResult<String> {
        match self.pk.public_key_to_pem() {
            Ok(res) => Strings::from_utf8(res),
            Err(err) => Err(err_strs("public_key_to_pem", err)),
        }
    }

    pub fn pk_pem_hex(&self) -> GeorgeResult<String> {
        match self.pk.public_key_to_pem() {
            Ok(res) => Ok(Hex::encode(res)),
            Err(err) => Err(err_strs("public_key_to_pem", err)),
        }
    }

    pub fn pk_pem_base64(&self) -> GeorgeResult<String> {
        match self.pk.public_key_to_pem() {
            Ok(res) => Ok(Base64::encode(res)),
            Err(err) => Err(err_strs("public_key_to_pem", err)),
        }
    }

    pub fn pk_der(&self) -> GeorgeResult<Vec<u8>> {
        match self.pk.public_key_to_der() {
            Ok(res) => Ok(res),
            Err(err) => Err(err_strs("public_key_to_der", err)),
        }
    }

    pub fn pk_der_hex(&self) -> GeorgeResult<String> {
        match self.pk.public_key_to_der() {
            Ok(res) => Ok(Hex::encode(res)),
            Err(err) => Err(err_strs("public_key_to_der", err)),
        }
    }

    pub fn pk_der_base64(&self) -> GeorgeResult<String> {
        match self.pk.public_key_to_der() {
            Ok(res) => Ok(Base64::encode(res)),
            Err(err) => Err(err_strs("public_key_to_der", err)),
        }
    }
}

/// sign method
impl ECDSA {
    pub fn sign(&self, data: &[u8]) -> GeorgeResult<Vec<u8>> {
        match EcdsaSig::sign(data, &self.sk) {
            Ok(sig) => match sig.to_der() {
                Ok(res) => Ok(res),
                Err(err) => Err(err_strs("EcdsaSig to_der", err)),
            },
            Err(err) => Err(err_strs("EcdsaSig sign", err)),
        }
    }

    pub fn verify(&self, data: &[u8], der: &[u8]) -> GeorgeResult<bool> {
        match EcdsaSig::from_der(der) {
            Ok(sig) => match sig.verify(data, &self.pk) {
                Ok(res) => Ok(res),
                Err(err) => Err(err_strs("EcdsaSig verify", err)),
            },
            Err(err) => Err(err_strs("EcdsaSig from_der", err)),
        }
    }
}

/// 生成ECDSA私钥，默认PRIME256V1
fn generate() -> GeorgeResult<(EcKey<Private>, EcKey<Public>)> {
    generate_nid(Nid::X9_62_PRIME256V1)
}

/// 生成ECDSA私钥
///
/// nid OpenSSL对象的数字标识符。
/// OpenSSL中的对象可以有短名称、长名称和数字标识符(NID)。为方便起见，对象通常在源代码中使用这些数字标识符表示。
/// 用户通常不需要创建新的' Nid '。
fn generate_nid(nid: Nid) -> GeorgeResult<(EcKey<Private>, EcKey<Public>)> {
    match EcGroup::from_curve_name(nid) {
        Ok(group) => match EcKey::generate(&group) {
            Ok(sk) => {
                let ec_point_ref = sk.public_key();
                match EcKey::from_public_key(&group, ec_point_ref) {
                    Ok(pk) => Ok((sk, pk)),
                    Err(err) => Err(err_strs("from_public_key", err)),
                }
            }
            Err(err) => Err(err_strs("generate", err)),
        },
        Err(err) => Err(err_strs("from_curve_name", err)),
    }
}

/// 生成ECDSA私钥，默认PRIME256V1
fn generate_sk() -> GeorgeResult<EcKey<Private>> {
    generate_sk_nid(Nid::X9_62_PRIME256V1)
}

/// 生成ECDSA私钥
///
/// nid OpenSSL对象的数字标识符。
/// OpenSSL中的对象可以有短名称、长名称和数字标识符(NID)。为方便起见，对象通常在源代码中使用这些数字标识符表示。
/// 用户通常不需要创建新的' Nid '。
fn generate_sk_nid(nid: Nid) -> GeorgeResult<EcKey<Private>> {
    match EcGroup::from_curve_name(nid) {
        Ok(group) => match EcKey::generate(&group) {
            Ok(key) => Ok(key),
            Err(err) => Err(err_strs("generate", err)),
        },
        Err(err) => Err(err_strs("from_curve_name", err)),
    }
}

/// 生成ECDSA私钥
///
/// nid OpenSSL对象的数字标识符。
/// OpenSSL中的对象可以有短名称、长名称和数字标识符(NID)。为方便起见，对象通常在源代码中使用这些数字标识符表示。
/// 用户通常不需要创建新的' Nid '。
fn generate_pk_from_sk(sk: EcKey<Private>) -> GeorgeResult<(EcKey<Private>, EcKey<Public>)> {
    let ec_point_ref = sk.public_key();
    match EcKey::from_public_key(sk.group(), ec_point_ref) {
        Ok(pk) => Ok((sk, pk)),
        Err(err) => Err(err_strs("from_public_key", err)),
    }
}

/// 生成ECDSA对象
fn from_bytes(sk_bytes: Vec<u8>, pk_bytes: Vec<u8>) -> GeorgeResult<ECDSA> {
    from_bytes_nid(sk_bytes, pk_bytes, Nid::X9_62_PRIME256V1)
}

/// 生成ECDSA对象
fn from_bytes_nid(sk_bytes: Vec<u8>, pk_bytes: Vec<u8>, nid: Nid) -> GeorgeResult<ECDSA> {
    let group = EcGroup::from_curve_name(nid).unwrap();
    let mut ctx = BigNumContext::new().unwrap();
    let public_key = EcPoint::from_bytes(&group, &pk_bytes, &mut ctx).unwrap();
    let pk = EcKey::from_public_key(&group, &public_key).unwrap();

    match BigNum::from_slice(&sk_bytes) {
        Ok(bn) => match EcKey::from_private_components(&group, &bn, &public_key) {
            Ok(sk) => Ok(ECDSA { sk, pk }),
            Err(err) => Err(err_strs("EcKey from_private_components", err)),
        },
        Err(err) => Err(err_strs("BigNum from_slice", err)),
    }
}

//////////////////////////////////////////////////////////////////////////////

/// 生成ECDSA私钥，默认PRIME256V1
fn generate_sk1() -> GeorgeResult<Vec<u8>> {
    match EcGroup::from_curve_name(Nid::X9_62_PRIME256V1) {
        Ok(group) => match EcKey::generate(&group) {
            Ok(key) => match key.private_key_to_pem() {
                Ok(v8s) => Ok(v8s),
                Err(err) => Err(err_strs("private_key_to_pem", err)),
            },
            Err(err) => Err(err_strs("generate", err)),
        },
        Err(err) => Err(err_strs("from_curve_name", err)),
    }
}

/// 生成ECDSA私钥并将私钥存储指定文件
///
/// 如果已存在，删除重写
pub fn generate_sk_in_file(filepath: String) -> GeorgeResult<Vec<u8>> {
    match generate_sk1() {
        Ok(u8s) => {
            Filer::write_force(filepath, u8s.clone())?;
            Ok(u8s)
        }
        Err(err) => Err(err_strs("generate_sk", err)),
    }
}

/// 生成ECDSA私钥并将私钥存储指定文件
///
/// 如果已存在，删除重写
pub fn generate_sk_in_files(filepath: &str) -> GeorgeResult<Vec<u8>> {
    generate_sk_in_file(filepath.to_string())
}

/// 读取ECDSA私钥
pub fn load_sk(sk: Vec<u8>) -> GeorgeResult<EcKey<Private>> {
    match EcKey::private_key_from_pem(sk.as_slice()) {
        Ok(key) => Ok(key),
        Err(err) => Err(err_strs("private_key_from_pem", err)),
    }
}

/// 读取ECDSA私钥
pub fn load_sk_file(filepath: String) -> GeorgeResult<EcKey<Private>> {
    match read(filepath) {
        Ok(u8s) => load_sk(u8s),
        Err(err) => Err(err_strs("read", err)),
    }
}

/// 生成ECDSA公钥
pub fn generate_pk_from_sk1(sk: EcKey<Private>) -> GeorgeResult<Vec<u8>> {
    match sk.public_key_to_pem() {
        Ok(u8s) => Ok(u8s),
        Err(err) => Err(err_strs("public_key_to_pem", err)),
    }
}

/// 生成ECDSA公钥
pub fn generate_pk_from_sk_bytes(sk: Vec<u8>) -> GeorgeResult<Vec<u8>> {
    match load_sk(sk) {
        Ok(key) => generate_pk_from_sk1(key),
        Err(err) => Err(err_strs("load_sk", err)),
    }
}

/// 生成ECDSA公钥
pub fn generate_pk_from_sk_file(filepath: String) -> GeorgeResult<Vec<u8>> {
    match load_sk_file(filepath) {
        Ok(key) => generate_pk_from_sk1(key),
        Err(err) => Err(err_strs("load_sk_file", err)),
    }
}

/// 生成ECDSA公钥并将私钥存储指定文件
///
/// 如果已存在，删除重写
pub fn generate_pk_in_file_from_sk(sk: EcKey<Private>, filepath: String) -> GeorgeResult<Vec<u8>> {
    match generate_pk_from_sk1(sk) {
        Ok(u8s) => {
            Filer::write_force(filepath, u8s.clone())?;
            Ok(u8s)
        }
        Err(err) => Err(err_strs("generate_pk_from_sk", err)),
    }
}

/// 生成ECDSA公钥并将私钥存储指定文件
///
/// 如果已存在，删除重写
pub fn generate_pk_in_file_from_sk_bytes(sk: Vec<u8>, filepath: String) -> GeorgeResult<Vec<u8>> {
    match generate_pk_from_sk_bytes(sk) {
        Ok(u8s) => {
            Filer::write_force(filepath, u8s.clone())?;
            Ok(u8s)
        }
        Err(err) => Err(err_strs("generate_pk_from_sk_bytes", err)),
    }
}

/// 生成ECDSA公钥并将私钥存储指定文件
///
/// 如果已存在，删除重写
pub fn generate_pk_in_file_from_sk_file(
    sk_filepath: String,
    pk_filepath: String,
) -> GeorgeResult<Vec<u8>> {
    match generate_pk_from_sk_file(sk_filepath) {
        Ok(u8s) => {
            Filer::write_force(pk_filepath, u8s.clone())?;
            Ok(u8s)
        }
        Err(err) => Err(err_strs("generate_pk_from_sk_file", err)),
    }
}

/// 读取ECDSA公钥
pub fn load_pk(pk: Vec<u8>) -> GeorgeResult<EcKey<Public>> {
    match EcKey::public_key_from_pem(pk.as_slice()) {
        Ok(key) => Ok(key),
        Err(err) => Err(err_strs("private_key_from_pem", err)),
    }
}

/// 读取ECDSA公钥
pub fn load_pk_file(filepath: String) -> GeorgeResult<EcKey<Public>> {
    match read(filepath) {
        Ok(u8s) => load_pk(u8s),
        Err(err) => Err(err_strs("read", err)),
    }
}
