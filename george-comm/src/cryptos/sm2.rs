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

use std::fs::read_to_string;
use std::path::Path;

use libsm::sm2::ecc::Point;
use libsm::sm2::signature::{SigCtx, Signature};

use crate::cryptos::base64::{Base64Decoder, Base64Encoder};
use crate::cryptos::hex::{HexDecoder, HexEncoder};
use crate::cryptos::Hex;
use crate::cryptos::{Base64, SM2};
use crate::errors::{Errs, GeorgeResult};
use crate::io::file::FilerWriter;
use crate::io::Filer;

pub trait SkNew {
    /// 生成非对称加密私钥，返回sk字节数组
    fn generate() -> Vec<u8>;

    /// 生成非对称加密私钥，返回sk字符串
    fn generate_hex() -> String;

    /// 生成非对称加密私钥，返回sk字符串
    fn generate_base64() -> String;
}

pub trait SkNewStore {
    /// 生成非对称加密私钥，返回sk字节数组
    ///
    /// 并将生成的私钥存储在sk指定文件中
    fn generate<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Vec<u8>>;

    /// 生成非对称加密私钥，返回sk字符串
    ///
    /// 并将生成的私钥存储在sk指定文件中
    fn generate_hex<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<String>;

    /// 生成非对称加密私钥，返回sk字符串
    ///
    /// 并将生成的私钥存储在sk指定文件中
    fn generate_base64<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<String>;
}

pub trait SKNew {
    /// 生成非对称加密公私钥，返回sk、pk字节数组
    fn generate() -> (Vec<u8>, Vec<u8>);

    /// 生成非对称加密公私钥，返回sk、pk字符串
    fn generate_hex() -> (String, String);

    /// 生成非对称加密公私钥，返回sk、pk字符串
    fn generate_base64() -> (String, String);
}

pub trait SKNewStore {
    /// 生成非对称加密公私钥，返回sk、pk字节数组
    ///
    /// 并将生成的公私钥存储在sk、pk指定文件中
    fn generate<P: AsRef<Path>>(sk_filepath: P, pk_filepath: P)
        -> GeorgeResult<(Vec<u8>, Vec<u8>)>;

    /// 生成非对称加密公私钥，返回sk、pk字符串
    ///
    /// 并将生成的公私钥存储在sk、pk指定文件中
    fn generate_hex<P: AsRef<Path>>(
        sk_filepath: P,
        pk_filepath: P,
    ) -> GeorgeResult<(String, String)>;

    /// 生成非对称加密公私钥，返回sk、pk字符串
    ///
    /// 并将生成的公私钥存储在sk、pk指定文件中
    fn generate_base64<P: AsRef<Path>>(
        sk_filepath: P,
        pk_filepath: P,
    ) -> GeorgeResult<(String, String)>;
}

pub trait SKPk {
    /// 根据私钥生成公钥
    fn generate_pk(sk: Vec<u8>) -> GeorgeResult<Vec<u8>>;

    /// 根据私钥hex字符串生成公钥
    fn generate_pk_by_hex(sk: String) -> GeorgeResult<Vec<u8>>;

    /// 根据私钥base64字符串生成公钥
    fn generate_pk_by_base64(sk: String) -> GeorgeResult<Vec<u8>>;

    /// 根据私钥hex字符串文件生成公钥
    fn generate_pk_by_hex_file<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Vec<u8>>;

    /// 根据私钥base64字符串文件生成公钥
    fn generate_pk_by_base64_file<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Vec<u8>>;
}

pub trait SKStoreKey<T> {
    /// 将公/私钥存储在指定文件中
    fn store<P: AsRef<Path>>(key: T, key_filepath: P) -> GeorgeResult<()>;

    /// 将公/私钥存储在指定文件中
    fn store_hex<P: AsRef<Path>>(key: T, key_filepath: P) -> GeorgeResult<()>;

    /// 将公/私钥存储在指定文件中
    fn store_base64<P: AsRef<Path>>(key: T, key_filepath: P) -> GeorgeResult<()>;
}

pub trait SKStore {
    /// 将公/私钥存储在指定文件中
    fn store<P: AsRef<Path>>(&self, sk_filepath: P, pk_filepath: P) -> GeorgeResult<()>;
}

pub trait SKLoadKey {
    /// 从指定文件中读取公/私钥
    fn load<P: AsRef<Path>>(sk_filepath: P, pk_filepath: P) -> GeorgeResult<SM2>;

    /// 从指定文件中读取公/私钥
    fn load_from_file<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<Vec<u8>>;

    /// 从指定文件中读取公/私钥
    fn load_string_from_file<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<String>;
}

pub trait SKSign<M, N> {
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
    fn sign_hex(msg: M, sk: N, pk: N) -> GeorgeResult<String>;

    /// 签名msg，返回签名结果字符串
    ///
    /// msg 待签名数据
    ///
    /// sk、pk 签名使用公私钥
    fn sign_base64(msg: M, sk: N, pk: N) -> GeorgeResult<String>;
}

pub trait SKSignPath<T> {
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
    fn sign_base64<P: AsRef<Path>>(msg: T, sk_filepath: P, pk_filepath: P) -> GeorgeResult<String>;
}

pub trait SKVerify<M, N, O> {
    /// 验签msg
    fn verify(msg: M, pk: N, der: O) -> GeorgeResult<bool>;
}

pub trait SKVerifyPath<M, N> {
    /// 验签msg
    fn verify<P: AsRef<Path>>(msg: M, sk_filepath: P, der: N) -> GeorgeResult<bool>;
}

impl SM2 {
    /// 生成非对称加密公私钥
    pub fn new() -> SM2 {
        let ctx = SigCtx::new();
        let (pk, sk) = ctx.new_keypair();
        SM2 { ctx, sk, pk }
    }

    pub fn new_pk(&self) -> Vec<u8> {
        self.ctx
            .serialize_pubkey(&self.ctx.pk_from_sk(&self.sk), true)
    }
    pub fn sk_bytes(&self) -> Vec<u8> {
        self.ctx.serialize_seckey(&self.sk)
    }

    pub fn pk_bytes(&self) -> Vec<u8> {
        self.ctx.serialize_pubkey(&self.pk, true)
    }

    pub fn sig(&self, msg: &[u8]) -> Vec<u8> {
        let sig = self.ctx.sign(msg, &self.sk, &self.pk);
        sig.der_encode()
    }

    pub fn sig_hex(&self, msg: &[u8]) -> String {
        let sig = self.ctx.sign(msg, &self.sk, &self.pk);
        Hex::encode(sig.der_encode())
    }

    pub fn sig_base64(&self, msg: &[u8]) -> String {
        let sig = self.ctx.sign(msg, &self.sk, &self.pk);
        Base64::encode(sig.der_encode())
    }

    pub fn sig_pk(&self, msg: &[u8], pk: &[u8]) -> GeorgeResult<Vec<u8>> {
        let pk_point: Point;
        match self.ctx.load_pubkey(pk) {
            Ok(pp) => pk_point = pp,
            Err(err) => return Err(Errs::string(format!("load pub key error! {:#?}", err))),
        }
        let sig = self.ctx.sign(msg, &self.sk, &pk_point);
        Ok(sig.der_encode())
    }

    pub fn verifies(&self, msg: &[u8], der: &[u8]) -> GeorgeResult<bool> {
        let sig: Signature;
        match Signature::der_decode(der) {
            Ok(s) => sig = s,
            Err(err) => return Err(Errs::strs("der decode", err)),
        }
        Ok(self.ctx.verify(msg, &self.pk, &sig))
    }

    pub fn verifies_pk(&self, msg: &[u8], der: &[u8], pk: &[u8]) -> GeorgeResult<bool> {
        let pk_point: Point;
        let sig: Signature;
        match self.ctx.load_pubkey(pk) {
            Ok(pp) => pk_point = pp,
            Err(err) => return Err(Errs::string(format!("load pub key error! {:#?}", err))),
        }
        match Signature::der_decode(der) {
            Ok(s) => sig = s,
            Err(err) => return Err(Errs::strs("der decode", err)),
        }
        Ok(self.ctx.verify(msg, &pk_point, &sig))
    }
}

////////// sm generate start //////////

impl SkNew for SM2 {
    fn generate() -> Vec<u8> {
        generate_sk()
    }

    fn generate_hex() -> String {
        Hex::encode(generate_sk())
    }

    fn generate_base64() -> String {
        Base64::encode(generate_sk())
    }
}

impl SkNewStore for SM2 {
    fn generate<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Vec<u8>> {
        generate_sk_in_file(sk_filepath)
    }

    fn generate_hex<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<String> {
        generate_sk_hex_in_file(sk_filepath)
    }

    fn generate_base64<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<String> {
        generate_sk_base64_in_file(sk_filepath)
    }
}

impl SKNew for SM2 {
    fn generate() -> (Vec<u8>, Vec<u8>) {
        generate()
    }

    fn generate_hex() -> (String, String) {
        generate_hex()
    }

    fn generate_base64() -> (String, String) {
        generate_base64()
    }
}

impl SKNewStore for SM2 {
    fn generate<P: AsRef<Path>>(
        sk_filepath: P,
        pk_filepath: P,
    ) -> GeorgeResult<(Vec<u8>, Vec<u8>)> {
        generate_in_file(sk_filepath, pk_filepath)
    }

    fn generate_hex<P: AsRef<Path>>(
        sk_filepath: P,
        pk_filepath: P,
    ) -> GeorgeResult<(String, String)> {
        generate_hex_in_file(sk_filepath, pk_filepath)
    }

    fn generate_base64<P: AsRef<Path>>(
        sk_filepath: P,
        pk_filepath: P,
    ) -> GeorgeResult<(String, String)> {
        generate_base64_in_file(sk_filepath, pk_filepath)
    }
}

////////// sm generate end //////////

////////// sm generate pk from sk start //////////
impl SKPk for SM2 {
    fn generate_pk(sk: Vec<u8>) -> GeorgeResult<Vec<u8>> {
        generate_pk_from_sk(sk)
    }

    fn generate_pk_by_hex(sk: String) -> GeorgeResult<Vec<u8>> {
        generate_pk_from_sk_hex(sk)
    }

    fn generate_pk_by_base64(sk: String) -> GeorgeResult<Vec<u8>> {
        generate_pk_from_sk_base64(sk)
    }

    fn generate_pk_by_hex_file<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Vec<u8>> {
        generate_pk_from_sk_hex_file(sk_filepath)
    }

    fn generate_pk_by_base64_file<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Vec<u8>> {
        generate_pk_from_sk_base64_file(sk_filepath)
    }
}

////////// sm generate pk from sk end //////////

////////// sm store/load start //////////

impl SKStoreKey<&[u8]> for SM2 {
    fn store<P: AsRef<Path>>(key: &[u8], key_filepath: P) -> GeorgeResult<()> {
        stores(key, key_filepath)
    }

    fn store_hex<P: AsRef<Path>>(key: &[u8], key_filepath: P) -> GeorgeResult<()> {
        store_hex_key(key, key_filepath)
    }

    fn store_base64<P: AsRef<Path>>(key: &[u8], key_filepath: P) -> GeorgeResult<()> {
        store_base64_key(key, key_filepath)
    }
}

impl SKStoreKey<Vec<u8>> for SM2 {
    fn store<P: AsRef<Path>>(key: Vec<u8>, key_filepath: P) -> GeorgeResult<()> {
        stores(key.as_slice(), key_filepath)
    }

    fn store_hex<P: AsRef<Path>>(key: Vec<u8>, key_filepath: P) -> GeorgeResult<()> {
        store_hex_bytes_key(key, key_filepath)
    }

    fn store_base64<P: AsRef<Path>>(key: Vec<u8>, key_filepath: P) -> GeorgeResult<()> {
        store_base64_bytes_key(key, key_filepath)
    }
}

impl SKStore for SM2 {
    fn store<P: AsRef<Path>>(&self, sk_filepath: P, pk_filepath: P) -> GeorgeResult<()> {
        store_key(Base64::encode(self.sk_bytes()), sk_filepath)?;
        store_key(Base64::encode(self.pk_bytes()), pk_filepath)
    }
}

impl SKLoadKey for SM2 {
    fn load<P: AsRef<Path>>(sk_filepath: P, pk_filepath: P) -> GeorgeResult<SM2> {
        let sk_bytes = load_key_from_file(sk_filepath)?;
        let pk_bytes = load_key_from_file(pk_filepath)?;
        let ctx = SigCtx::new();
        match ctx.load_pubkey(pk_bytes.as_slice()) {
            Ok(pk) => match ctx.load_seckey(sk_bytes.as_slice()) {
                Ok(sk) => Ok(SM2 { ctx, sk, pk }),
                Err(err) => return Err(Errs::string(format!("load pub key error! {:#?}", err))),
            },
            Err(err) => return Err(Errs::string(format!("load pub key error! {:#?}", err))),
        }
    }

    fn load_from_file<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<Vec<u8>> {
        load_key_from_file(key_filepath)
    }

    fn load_string_from_file<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<String> {
        load_key_string_from_file(key_filepath)
    }
}

////////// sm store/load end //////////

////////// sm sign start //////////

impl SKSign<&[u8], &[u8]> for SM2 {
    fn sign(msg: &[u8], sk: &[u8], pk: &[u8]) -> GeorgeResult<Vec<u8>> {
        sign(msg, sk, pk)
    }

    fn sign_hex(msg: &[u8], sk: &[u8], pk: &[u8]) -> GeorgeResult<String> {
        Ok(Hex::encode(sign(msg, sk, pk)?))
    }

    fn sign_base64(msg: &[u8], sk: &[u8], pk: &[u8]) -> GeorgeResult<String> {
        Ok(Base64::encode(sign(msg, sk, pk)?))
    }
}

impl SKSign<&[u8], Vec<u8>> for SM2 {
    fn sign(msg: &[u8], sk: Vec<u8>, pk: Vec<u8>) -> GeorgeResult<Vec<u8>> {
        sign(msg, sk.as_slice(), pk.as_slice())
    }

    fn sign_hex(msg: &[u8], sk: Vec<u8>, pk: Vec<u8>) -> GeorgeResult<String> {
        Ok(Hex::encode(sign(msg, sk.as_slice(), pk.as_slice())?))
    }

    fn sign_base64(msg: &[u8], sk: Vec<u8>, pk: Vec<u8>) -> GeorgeResult<String> {
        Ok(Base64::encode(sign(msg, sk.as_slice(), pk.as_slice())?))
    }
}

impl SKSign<Vec<u8>, Vec<u8>> for SM2 {
    fn sign(msg: Vec<u8>, sk: Vec<u8>, pk: Vec<u8>) -> GeorgeResult<Vec<u8>> {
        sign(msg.as_slice(), sk.as_slice(), pk.as_slice())
    }

    fn sign_hex(msg: Vec<u8>, sk: Vec<u8>, pk: Vec<u8>) -> GeorgeResult<String> {
        Ok(Hex::encode(sign(
            msg.as_slice(),
            sk.as_slice(),
            pk.as_slice(),
        )?))
    }

    fn sign_base64(msg: Vec<u8>, sk: Vec<u8>, pk: Vec<u8>) -> GeorgeResult<String> {
        Ok(Base64::encode(sign(
            msg.as_slice(),
            sk.as_slice(),
            pk.as_slice(),
        )?))
    }
}

impl SKSign<String, Vec<u8>> for SM2 {
    fn sign(msg: String, sk: Vec<u8>, pk: Vec<u8>) -> GeorgeResult<Vec<u8>> {
        sign(msg.as_bytes(), sk.as_slice(), pk.as_slice())
    }

    fn sign_hex(msg: String, sk: Vec<u8>, pk: Vec<u8>) -> GeorgeResult<String> {
        Ok(Hex::encode(sign(
            msg.as_bytes(),
            sk.as_slice(),
            pk.as_slice(),
        )?))
    }

    fn sign_base64(msg: String, sk: Vec<u8>, pk: Vec<u8>) -> GeorgeResult<String> {
        Ok(Base64::encode(sign(
            msg.as_bytes(),
            sk.as_slice(),
            pk.as_slice(),
        )?))
    }
}

impl SKSign<&str, Vec<u8>> for SM2 {
    fn sign(msg: &str, sk: Vec<u8>, pk: Vec<u8>) -> GeorgeResult<Vec<u8>> {
        sign(msg.as_bytes(), sk.as_slice(), pk.as_slice())
    }

    fn sign_hex(msg: &str, sk: Vec<u8>, pk: Vec<u8>) -> GeorgeResult<String> {
        Ok(Hex::encode(sign(
            msg.as_bytes(),
            sk.as_slice(),
            pk.as_slice(),
        )?))
    }

    fn sign_base64(msg: &str, sk: Vec<u8>, pk: Vec<u8>) -> GeorgeResult<String> {
        Ok(Base64::encode(sign(
            msg.as_bytes(),
            sk.as_slice(),
            pk.as_slice(),
        )?))
    }
}

impl SKSign<Vec<u8>, &[u8]> for SM2 {
    fn sign(msg: Vec<u8>, sk: &[u8], pk: &[u8]) -> GeorgeResult<Vec<u8>> {
        sign(msg.as_slice(), sk, pk)
    }

    fn sign_hex(msg: Vec<u8>, sk: &[u8], pk: &[u8]) -> GeorgeResult<String> {
        Ok(Hex::encode(sign(msg.as_slice(), sk, pk)?))
    }

    fn sign_base64(msg: Vec<u8>, sk: &[u8], pk: &[u8]) -> GeorgeResult<String> {
        Ok(Base64::encode(sign(msg.as_slice(), sk, pk)?))
    }
}

impl SKSign<String, &[u8]> for SM2 {
    fn sign(msg: String, sk: &[u8], pk: &[u8]) -> GeorgeResult<Vec<u8>> {
        sign(msg.as_bytes(), sk, pk)
    }

    fn sign_hex(msg: String, sk: &[u8], pk: &[u8]) -> GeorgeResult<String> {
        Ok(Hex::encode(sign(msg.as_bytes(), sk, pk)?))
    }

    fn sign_base64(msg: String, sk: &[u8], pk: &[u8]) -> GeorgeResult<String> {
        Ok(Base64::encode(sign(msg.as_bytes(), sk, pk)?))
    }
}

impl SKSign<&str, &[u8]> for SM2 {
    fn sign(msg: &str, sk: &[u8], pk: &[u8]) -> GeorgeResult<Vec<u8>> {
        sign(msg.as_bytes(), sk, pk)
    }

    fn sign_hex(msg: &str, sk: &[u8], pk: &[u8]) -> GeorgeResult<String> {
        Ok(Hex::encode(sign(msg.as_bytes(), sk, pk)?))
    }

    fn sign_base64(msg: &str, sk: &[u8], pk: &[u8]) -> GeorgeResult<String> {
        Ok(Base64::encode(sign(msg.as_bytes(), sk, pk)?))
    }
}

impl SKSign<&[u8], String> for SM2 {
    fn sign(msg: &[u8], sk: String, pk: String) -> GeorgeResult<Vec<u8>> {
        sign(
            msg,
            Base64::decode(sk)?.as_slice(),
            Base64::decode(pk)?.as_slice(),
        )
    }

    fn sign_hex(msg: &[u8], sk: String, pk: String) -> GeorgeResult<String> {
        Ok(Hex::encode(sign(
            msg,
            Base64::decode(sk)?.as_slice(),
            Base64::decode(pk)?.as_slice(),
        )?))
    }

    fn sign_base64(msg: &[u8], sk: String, pk: String) -> GeorgeResult<String> {
        Ok(Base64::encode(sign(
            msg,
            Base64::decode(sk)?.as_slice(),
            Base64::decode(pk)?.as_slice(),
        )?))
    }
}

impl SKSign<Vec<u8>, String> for SM2 {
    fn sign(msg: Vec<u8>, sk: String, pk: String) -> GeorgeResult<Vec<u8>> {
        sign(
            msg.as_slice(),
            Base64::decode(sk)?.as_slice(),
            Base64::decode(pk)?.as_slice(),
        )
    }

    fn sign_hex(msg: Vec<u8>, sk: String, pk: String) -> GeorgeResult<String> {
        Ok(Hex::encode(sign(
            msg.as_slice(),
            Base64::decode(sk)?.as_slice(),
            Base64::decode(pk)?.as_slice(),
        )?))
    }

    fn sign_base64(msg: Vec<u8>, sk: String, pk: String) -> GeorgeResult<String> {
        Ok(Base64::encode(sign(
            msg.as_slice(),
            Base64::decode(sk)?.as_slice(),
            Base64::decode(pk)?.as_slice(),
        )?))
    }
}

impl SKSign<String, String> for SM2 {
    fn sign(msg: String, sk: String, pk: String) -> GeorgeResult<Vec<u8>> {
        sign(
            msg.as_bytes(),
            Base64::decode(sk)?.as_slice(),
            Base64::decode(pk)?.as_slice(),
        )
    }

    fn sign_hex(msg: String, sk: String, pk: String) -> GeorgeResult<String> {
        Ok(Hex::encode(sign(
            msg.as_bytes(),
            Base64::decode(sk)?.as_slice(),
            Base64::decode(pk)?.as_slice(),
        )?))
    }

    fn sign_base64(msg: String, sk: String, pk: String) -> GeorgeResult<String> {
        Ok(Base64::encode(sign(
            msg.as_bytes(),
            Base64::decode(sk)?.as_slice(),
            Base64::decode(pk)?.as_slice(),
        )?))
    }
}

impl SKSign<&str, String> for SM2 {
    fn sign(msg: &str, sk: String, pk: String) -> GeorgeResult<Vec<u8>> {
        sign(
            msg.as_bytes(),
            Base64::decode(sk)?.as_slice(),
            Base64::decode(pk)?.as_slice(),
        )
    }

    fn sign_hex(msg: &str, sk: String, pk: String) -> GeorgeResult<String> {
        Ok(Hex::encode(sign(
            msg.as_bytes(),
            Base64::decode(sk)?.as_slice(),
            Base64::decode(pk)?.as_slice(),
        )?))
    }

    fn sign_base64(msg: &str, sk: String, pk: String) -> GeorgeResult<String> {
        Ok(Base64::encode(sign(
            msg.as_bytes(),
            Base64::decode(sk)?.as_slice(),
            Base64::decode(pk)?.as_slice(),
        )?))
    }
}

impl SKSign<&[u8], &str> for SM2 {
    fn sign(msg: &[u8], sk: &str, pk: &str) -> GeorgeResult<Vec<u8>> {
        sign(
            msg,
            Base64::decode(sk)?.as_slice(),
            Base64::decode(pk)?.as_slice(),
        )
    }

    fn sign_hex(msg: &[u8], sk: &str, pk: &str) -> GeorgeResult<String> {
        Ok(Hex::encode(sign(
            msg,
            Base64::decode(sk)?.as_slice(),
            Base64::decode(pk)?.as_slice(),
        )?))
    }

    fn sign_base64(msg: &[u8], sk: &str, pk: &str) -> GeorgeResult<String> {
        Ok(Base64::encode(sign(
            msg,
            Base64::decode(sk)?.as_slice(),
            Base64::decode(pk)?.as_slice(),
        )?))
    }
}

impl SKSign<Vec<u8>, &str> for SM2 {
    fn sign(msg: Vec<u8>, sk: &str, pk: &str) -> GeorgeResult<Vec<u8>> {
        sign(
            msg.as_slice(),
            Base64::decode(sk)?.as_slice(),
            Base64::decode(pk)?.as_slice(),
        )
    }

    fn sign_hex(msg: Vec<u8>, sk: &str, pk: &str) -> GeorgeResult<String> {
        Ok(Hex::encode(sign(
            msg.as_slice(),
            Base64::decode(sk)?.as_slice(),
            Base64::decode(pk)?.as_slice(),
        )?))
    }

    fn sign_base64(msg: Vec<u8>, sk: &str, pk: &str) -> GeorgeResult<String> {
        Ok(Base64::encode(sign(
            msg.as_slice(),
            Base64::decode(sk)?.as_slice(),
            Base64::decode(pk)?.as_slice(),
        )?))
    }
}

impl SKSign<String, &str> for SM2 {
    fn sign(msg: String, sk: &str, pk: &str) -> GeorgeResult<Vec<u8>> {
        sign(
            msg.as_bytes(),
            Base64::decode(sk)?.as_slice(),
            Base64::decode(pk)?.as_slice(),
        )
    }

    fn sign_hex(msg: String, sk: &str, pk: &str) -> GeorgeResult<String> {
        Ok(Hex::encode(sign(
            msg.as_bytes(),
            Base64::decode(sk)?.as_slice(),
            Base64::decode(pk)?.as_slice(),
        )?))
    }

    fn sign_base64(msg: String, sk: &str, pk: &str) -> GeorgeResult<String> {
        Ok(Base64::encode(sign(
            msg.as_bytes(),
            Base64::decode(sk)?.as_slice(),
            Base64::decode(pk)?.as_slice(),
        )?))
    }
}

impl SKSign<&str, &str> for SM2 {
    fn sign(msg: &str, sk: &str, pk: &str) -> GeorgeResult<Vec<u8>> {
        sign(
            msg.as_bytes(),
            Base64::decode(sk)?.as_slice(),
            Base64::decode(pk)?.as_slice(),
        )
    }

    fn sign_hex(msg: &str, sk: &str, pk: &str) -> GeorgeResult<String> {
        Ok(Hex::encode(sign(
            msg.as_bytes(),
            Base64::decode(sk)?.as_slice(),
            Base64::decode(pk)?.as_slice(),
        )?))
    }

    fn sign_base64(msg: &str, sk: &str, pk: &str) -> GeorgeResult<String> {
        Ok(Base64::encode(sign(
            msg.as_bytes(),
            Base64::decode(sk)?.as_slice(),
            Base64::decode(pk)?.as_slice(),
        )?))
    }
}

impl SKSignPath<&[u8]> for SM2 {
    fn sign<P: AsRef<Path>>(msg: &[u8], sk_filepath: P, pk_filepath: P) -> GeorgeResult<Vec<u8>> {
        sign(
            msg,
            load_key_from_file(sk_filepath)?.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
        )
    }

    fn sign_base64<P: AsRef<Path>>(
        msg: &[u8],
        sk_filepath: P,
        pk_filepath: P,
    ) -> GeorgeResult<String> {
        Ok(Base64::encode(sign(
            msg,
            load_key_from_file(sk_filepath)?.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
        )?))
    }
}

impl SKSignPath<Vec<u8>> for SM2 {
    fn sign<P: AsRef<Path>>(msg: Vec<u8>, sk_filepath: P, pk_filepath: P) -> GeorgeResult<Vec<u8>> {
        sign(
            msg.as_slice(),
            load_key_from_file(sk_filepath)?.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
        )
    }

    fn sign_base64<P: AsRef<Path>>(
        msg: Vec<u8>,
        sk_filepath: P,
        pk_filepath: P,
    ) -> GeorgeResult<String> {
        Ok(Base64::encode(sign(
            msg.as_slice(),
            load_key_from_file(sk_filepath)?.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
        )?))
    }
}

impl SKSignPath<String> for SM2 {
    fn sign<P: AsRef<Path>>(msg: String, sk_filepath: P, pk_filepath: P) -> GeorgeResult<Vec<u8>> {
        sign(
            msg.as_bytes(),
            load_key_from_file(sk_filepath)?.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
        )
    }

    fn sign_base64<P: AsRef<Path>>(
        msg: String,
        sk_filepath: P,
        pk_filepath: P,
    ) -> GeorgeResult<String> {
        Ok(Base64::encode(sign(
            msg.as_bytes(),
            load_key_from_file(sk_filepath)?.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
        )?))
    }
}

impl SKSignPath<&str> for SM2 {
    fn sign<P: AsRef<Path>>(msg: &str, sk_filepath: P, pk_filepath: P) -> GeorgeResult<Vec<u8>> {
        sign(
            msg.as_bytes(),
            load_key_from_file(sk_filepath)?.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
        )
    }

    fn sign_base64<P: AsRef<Path>>(
        msg: &str,
        sk_filepath: P,
        pk_filepath: P,
    ) -> GeorgeResult<String> {
        Ok(Base64::encode(sign(
            msg.as_bytes(),
            load_key_from_file(sk_filepath)?.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
        )?))
    }
}

////////// sm sign end //////////

////////// sm verify start //////////

impl SKVerify<&[u8], &[u8], &[u8]> for SM2 {
    fn verify(msg: &[u8], pk: &[u8], der: &[u8]) -> GeorgeResult<bool> {
        verify(msg, pk, der)
    }
}

impl SKVerify<&[u8], &[u8], Vec<u8>> for SM2 {
    fn verify(msg: &[u8], pk: &[u8], der: Vec<u8>) -> GeorgeResult<bool> {
        verify(msg, pk, der.as_slice())
    }
}

impl SKVerify<&[u8], &[u8], String> for SM2 {
    fn verify(msg: &[u8], pk: &[u8], der: String) -> GeorgeResult<bool> {
        verify(msg, pk, Base64::decode(der)?.as_slice())
    }
}

impl SKVerify<&[u8], &[u8], &str> for SM2 {
    fn verify(msg: &[u8], pk: &[u8], der: &str) -> GeorgeResult<bool> {
        verify(msg, pk, Base64::decode(der.to_string())?.as_slice())
    }
}

impl SKVerify<&[u8], Vec<u8>, &[u8]> for SM2 {
    fn verify(msg: &[u8], pk: Vec<u8>, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg, pk.as_slice(), der)
    }
}

impl SKVerify<&[u8], Vec<u8>, Vec<u8>> for SM2 {
    fn verify(msg: &[u8], pk: Vec<u8>, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(msg, pk.as_slice(), der.as_slice())
    }
}

impl SKVerify<&[u8], Vec<u8>, String> for SM2 {
    fn verify(msg: &[u8], pk: Vec<u8>, der: String) -> GeorgeResult<bool> {
        verify(msg, pk.as_slice(), Base64::decode(der)?.as_slice())
    }
}

impl SKVerify<&[u8], Vec<u8>, &str> for SM2 {
    fn verify(msg: &[u8], pk: Vec<u8>, der: &str) -> GeorgeResult<bool> {
        verify(
            msg,
            pk.as_slice(),
            Base64::decode(der.to_string())?.as_slice(),
        )
    }
}

impl SKVerify<&[u8], String, &[u8]> for SM2 {
    fn verify(msg: &[u8], pk: String, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg, &Base64::decode(pk)?.as_slice(), der)
    }
}

impl SKVerify<&[u8], String, Vec<u8>> for SM2 {
    fn verify(msg: &[u8], pk: String, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(msg, &Base64::decode(pk)?.as_slice(), der.as_slice())
    }
}

impl SKVerify<&[u8], String, String> for SM2 {
    fn verify(msg: &[u8], pk: String, der: String) -> GeorgeResult<bool> {
        verify(
            msg,
            &Base64::decode(pk)?.as_slice(),
            &Base64::decode(der)?.as_slice(),
        )
    }
}

impl SKVerify<&[u8], String, &str> for SM2 {
    fn verify(msg: &[u8], pk: String, der: &str) -> GeorgeResult<bool> {
        verify(
            msg,
            &Base64::decode(pk)?.as_slice(),
            Base64::decode(der.to_string())?.as_slice(),
        )
    }
}

impl SKVerify<&[u8], &str, &[u8]> for SM2 {
    fn verify(msg: &[u8], pk: &str, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg, &Base64::decode(pk.to_string())?.as_slice(), der)
    }
}

impl SKVerify<&[u8], &str, Vec<u8>> for SM2 {
    fn verify(msg: &[u8], pk: &str, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg,
            &Base64::decode(pk.to_string())?.as_slice(),
            der.as_slice(),
        )
    }
}

impl SKVerify<&[u8], &str, String> for SM2 {
    fn verify(msg: &[u8], pk: &str, der: String) -> GeorgeResult<bool> {
        verify(
            msg,
            &Base64::decode(pk.to_string())?.as_slice(),
            Base64::decode(der)?.as_slice(),
        )
    }
}

impl SKVerify<&[u8], &str, &str> for SM2 {
    fn verify(msg: &[u8], pk: &str, der: &str) -> GeorgeResult<bool> {
        verify(
            msg,
            &Base64::decode(pk.to_string())?.as_slice(),
            &Base64::decode(der.to_string())?.as_slice(),
        )
    }
}

impl SKVerify<Vec<u8>, &[u8], &[u8]> for SM2 {
    fn verify(msg: Vec<u8>, pk: &[u8], der: &[u8]) -> GeorgeResult<bool> {
        verify(msg.as_slice(), pk, der)
    }
}

impl SKVerify<Vec<u8>, &[u8], Vec<u8>> for SM2 {
    fn verify(msg: Vec<u8>, pk: &[u8], der: Vec<u8>) -> GeorgeResult<bool> {
        verify(msg.as_slice(), pk, der.as_slice())
    }
}

impl SKVerify<Vec<u8>, &[u8], String> for SM2 {
    fn verify(msg: Vec<u8>, pk: &[u8], der: String) -> GeorgeResult<bool> {
        verify(msg.as_slice(), pk, Base64::decode(der)?.as_slice())
    }
}

impl SKVerify<Vec<u8>, &[u8], &str> for SM2 {
    fn verify(msg: Vec<u8>, pk: &[u8], der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            pk,
            Base64::decode(der.to_string())?.as_slice(),
        )
    }
}

impl SKVerify<Vec<u8>, Vec<u8>, &[u8]> for SM2 {
    fn verify(msg: Vec<u8>, pk: Vec<u8>, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg.as_slice(), pk.as_slice(), der)
    }
}

impl SKVerify<Vec<u8>, Vec<u8>, Vec<u8>> for SM2 {
    fn verify(msg: Vec<u8>, pk: Vec<u8>, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(msg.as_slice(), pk.as_slice(), der.as_slice())
    }
}

impl SKVerify<Vec<u8>, Vec<u8>, String> for SM2 {
    fn verify(msg: Vec<u8>, pk: Vec<u8>, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            pk.as_slice(),
            Base64::decode(der)?.as_slice(),
        )
    }
}

impl SKVerify<Vec<u8>, Vec<u8>, &str> for SM2 {
    fn verify(msg: Vec<u8>, pk: Vec<u8>, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            pk.as_slice(),
            Base64::decode(der.to_string())?.as_slice(),
        )
    }
}

impl SKVerify<Vec<u8>, String, &[u8]> for SM2 {
    fn verify(msg: Vec<u8>, pk: String, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg.as_slice(), &Base64::decode(pk)?.as_slice(), der)
    }
}

impl SKVerify<Vec<u8>, String, Vec<u8>> for SM2 {
    fn verify(msg: Vec<u8>, pk: String, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            &Base64::decode(pk)?.as_slice(),
            der.as_slice(),
        )
    }
}

impl SKVerify<Vec<u8>, String, String> for SM2 {
    fn verify(msg: Vec<u8>, pk: String, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            &Base64::decode(pk)?.as_slice(),
            &Base64::decode(der)?.as_slice(),
        )
    }
}

impl SKVerify<Vec<u8>, String, &str> for SM2 {
    fn verify(msg: Vec<u8>, pk: String, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            &Base64::decode(pk)?.as_slice(),
            Base64::decode(der.to_string())?.as_slice(),
        )
    }
}

impl SKVerify<Vec<u8>, &str, &[u8]> for SM2 {
    fn verify(msg: Vec<u8>, pk: &str, der: &[u8]) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            &Base64::decode(pk.to_string())?.as_slice(),
            der,
        )
    }
}

impl SKVerify<Vec<u8>, &str, Vec<u8>> for SM2 {
    fn verify(msg: Vec<u8>, pk: &str, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            &Base64::decode(pk.to_string())?.as_slice(),
            der.as_slice(),
        )
    }
}

impl SKVerify<Vec<u8>, &str, String> for SM2 {
    fn verify(msg: Vec<u8>, pk: &str, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            &Base64::decode(pk.to_string())?.as_slice(),
            Base64::decode(der)?.as_slice(),
        )
    }
}

impl SKVerify<Vec<u8>, &str, &str> for SM2 {
    fn verify(msg: Vec<u8>, pk: &str, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            &Base64::decode(pk.to_string())?.as_slice(),
            &Base64::decode(der.to_string())?.as_slice(),
        )
    }
}

impl SKVerify<String, &[u8], &[u8]> for SM2 {
    fn verify(msg: String, pk: &[u8], der: &[u8]) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk, der)
    }
}

impl SKVerify<String, &[u8], Vec<u8>> for SM2 {
    fn verify(msg: String, pk: &[u8], der: Vec<u8>) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk, der.as_slice())
    }
}

impl SKVerify<String, &[u8], String> for SM2 {
    fn verify(msg: String, pk: &[u8], der: String) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk, Base64::decode(der)?.as_slice())
    }
}

impl SKVerify<String, &[u8], &str> for SM2 {
    fn verify(msg: String, pk: &[u8], der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            pk,
            Base64::decode(der.to_string())?.as_slice(),
        )
    }
}

impl SKVerify<String, Vec<u8>, &[u8]> for SM2 {
    fn verify(msg: String, pk: Vec<u8>, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk.as_slice(), der)
    }
}

impl SKVerify<String, Vec<u8>, Vec<u8>> for SM2 {
    fn verify(msg: String, pk: Vec<u8>, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk.as_slice(), der.as_slice())
    }
}

impl SKVerify<String, Vec<u8>, String> for SM2 {
    fn verify(msg: String, pk: Vec<u8>, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            pk.as_slice(),
            Base64::decode(der)?.as_slice(),
        )
    }
}

impl SKVerify<String, Vec<u8>, &str> for SM2 {
    fn verify(msg: String, pk: Vec<u8>, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            pk.as_slice(),
            Base64::decode(der.to_string())?.as_slice(),
        )
    }
}

impl SKVerify<String, String, &[u8]> for SM2 {
    fn verify(msg: String, pk: String, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), &Base64::decode(pk)?.as_slice(), der)
    }
}

impl SKVerify<String, String, Vec<u8>> for SM2 {
    fn verify(msg: String, pk: String, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &Base64::decode(pk)?.as_slice(),
            der.as_slice(),
        )
    }
}

impl SKVerify<String, String, String> for SM2 {
    fn verify(msg: String, pk: String, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &Base64::decode(pk)?.as_slice(),
            &Base64::decode(der)?.as_slice(),
        )
    }
}

impl SKVerify<String, String, &str> for SM2 {
    fn verify(msg: String, pk: String, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &Base64::decode(pk)?.as_slice(),
            Base64::decode(der.to_string())?.as_slice(),
        )
    }
}

impl SKVerify<String, &str, &[u8]> for SM2 {
    fn verify(msg: String, pk: &str, der: &[u8]) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &Base64::decode(pk.to_string())?.as_slice(),
            der,
        )
    }
}

impl SKVerify<String, &str, Vec<u8>> for SM2 {
    fn verify(msg: String, pk: &str, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &Base64::decode(pk.to_string())?.as_slice(),
            der.as_slice(),
        )
    }
}

impl SKVerify<String, &str, String> for SM2 {
    fn verify(msg: String, pk: &str, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &Base64::decode(pk.to_string())?.as_slice(),
            Base64::decode(der)?.as_slice(),
        )
    }
}

impl SKVerify<String, &str, &str> for SM2 {
    fn verify(msg: String, pk: &str, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &Base64::decode(pk.to_string())?.as_slice(),
            &Base64::decode(der.to_string())?.as_slice(),
        )
    }
}

impl SKVerify<&str, &[u8], &[u8]> for SM2 {
    fn verify(msg: &str, pk: &[u8], der: &[u8]) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk, der)
    }
}

impl SKVerify<&str, &[u8], Vec<u8>> for SM2 {
    fn verify(msg: &str, pk: &[u8], der: Vec<u8>) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk, der.as_slice())
    }
}

impl SKVerify<&str, &[u8], String> for SM2 {
    fn verify(msg: &str, pk: &[u8], der: String) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk, Base64::decode(der)?.as_slice())
    }
}

impl SKVerify<&str, &[u8], &str> for SM2 {
    fn verify(msg: &str, pk: &[u8], der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            pk,
            Base64::decode(der.to_string())?.as_slice(),
        )
    }
}

impl SKVerify<&str, Vec<u8>, &[u8]> for SM2 {
    fn verify(msg: &str, pk: Vec<u8>, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk.as_slice(), der)
    }
}

impl SKVerify<&str, Vec<u8>, Vec<u8>> for SM2 {
    fn verify(msg: &str, pk: Vec<u8>, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk.as_slice(), der.as_slice())
    }
}

impl SKVerify<&str, Vec<u8>, String> for SM2 {
    fn verify(msg: &str, pk: Vec<u8>, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            pk.as_slice(),
            Base64::decode(der)?.as_slice(),
        )
    }
}

impl SKVerify<&str, Vec<u8>, &str> for SM2 {
    fn verify(msg: &str, pk: Vec<u8>, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            pk.as_slice(),
            Base64::decode(der.to_string())?.as_slice(),
        )
    }
}

impl SKVerify<&str, String, &[u8]> for SM2 {
    fn verify(msg: &str, pk: String, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), &Base64::decode(pk)?.as_slice(), der)
    }
}

impl SKVerify<&str, String, Vec<u8>> for SM2 {
    fn verify(msg: &str, pk: String, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &Base64::decode(pk)?.as_slice(),
            der.as_slice(),
        )
    }
}

impl SKVerify<&str, String, String> for SM2 {
    fn verify(msg: &str, pk: String, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &Base64::decode(pk)?.as_slice(),
            &Base64::decode(der)?.as_slice(),
        )
    }
}

impl SKVerify<&str, String, &str> for SM2 {
    fn verify(msg: &str, pk: String, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &Base64::decode(pk)?.as_slice(),
            Base64::decode(der.to_string())?.as_slice(),
        )
    }
}

impl SKVerify<&str, &str, &[u8]> for SM2 {
    fn verify(msg: &str, pk: &str, der: &[u8]) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &Base64::decode(pk.to_string())?.as_slice(),
            der,
        )
    }
}

impl SKVerify<&str, &str, Vec<u8>> for SM2 {
    fn verify(msg: &str, pk: &str, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &Base64::decode(pk.to_string())?.as_slice(),
            der.as_slice(),
        )
    }
}

impl SKVerify<&str, &str, String> for SM2 {
    fn verify(msg: &str, pk: &str, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &Base64::decode(pk.to_string())?.as_slice(),
            Base64::decode(der)?.as_slice(),
        )
    }
}

impl SKVerify<&str, &str, &str> for SM2 {
    fn verify(msg: &str, pk: &str, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &Base64::decode(pk.to_string())?.as_slice(),
            &Base64::decode(der.to_string())?.as_slice(),
        )
    }
}

impl SKVerifyPath<&[u8], &[u8]> for SM2 {
    fn verify<P: AsRef<Path>>(msg: &[u8], pk_filepath: P, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg, load_key_from_file(pk_filepath)?.as_slice(), der)
    }
}

impl SKVerifyPath<&[u8], Vec<u8>> for SM2 {
    fn verify<P: AsRef<Path>>(msg: &[u8], pk_filepath: P, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg,
            load_key_from_file(pk_filepath)?.as_slice(),
            der.as_slice(),
        )
    }
}

impl SKVerifyPath<&[u8], String> for SM2 {
    fn verify<P: AsRef<Path>>(msg: &[u8], pk_filepath: P, der: String) -> GeorgeResult<bool> {
        verify(
            msg,
            load_key_from_file(pk_filepath)?.as_slice(),
            Base64::decode(der)?.as_slice(),
        )
    }
}

impl SKVerifyPath<&[u8], &str> for SM2 {
    fn verify<P: AsRef<Path>>(msg: &[u8], pk_filepath: P, der: &str) -> GeorgeResult<bool> {
        verify(
            msg,
            load_key_from_file(pk_filepath)?.as_slice(),
            Base64::decode(der.to_string())?.as_slice(),
        )
    }
}

impl SKVerifyPath<Vec<u8>, &[u8]> for SM2 {
    fn verify<P: AsRef<Path>>(msg: Vec<u8>, pk_filepath: P, der: &[u8]) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
            der,
        )
    }
}

impl SKVerifyPath<Vec<u8>, Vec<u8>> for SM2 {
    fn verify<P: AsRef<Path>>(msg: Vec<u8>, pk_filepath: P, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
            der.as_slice(),
        )
    }
}

impl SKVerifyPath<Vec<u8>, String> for SM2 {
    fn verify<P: AsRef<Path>>(msg: Vec<u8>, pk_filepath: P, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
            Base64::decode(der)?.as_slice(),
        )
    }
}

impl SKVerifyPath<Vec<u8>, &str> for SM2 {
    fn verify<P: AsRef<Path>>(msg: Vec<u8>, pk_filepath: P, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
            Base64::decode(der.to_string())?.as_slice(),
        )
    }
}

impl SKVerifyPath<String, &[u8]> for SM2 {
    fn verify<P: AsRef<Path>>(msg: String, pk_filepath: P, der: &[u8]) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            load_key_from_file(pk_filepath)?.as_slice(),
            der,
        )
    }
}

impl SKVerifyPath<String, Vec<u8>> for SM2 {
    fn verify<P: AsRef<Path>>(msg: String, pk_filepath: P, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            load_key_from_file(pk_filepath)?.as_slice(),
            der.as_slice(),
        )
    }
}

impl SKVerifyPath<String, String> for SM2 {
    fn verify<P: AsRef<Path>>(msg: String, pk_filepath: P, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            load_key_from_file(pk_filepath)?.as_slice(),
            Base64::decode(der)?.as_slice(),
        )
    }
}

impl SKVerifyPath<String, &str> for SM2 {
    fn verify<P: AsRef<Path>>(msg: String, pk_filepath: P, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            load_key_from_file(pk_filepath)?.as_slice(),
            Base64::decode(der.to_string())?.as_slice(),
        )
    }
}

impl SKVerifyPath<&str, &[u8]> for SM2 {
    fn verify<P: AsRef<Path>>(msg: &str, pk_filepath: P, der: &[u8]) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            load_key_from_file(pk_filepath)?.as_slice(),
            der,
        )
    }
}

impl SKVerifyPath<&str, Vec<u8>> for SM2 {
    fn verify<P: AsRef<Path>>(msg: &str, pk_filepath: P, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            load_key_from_file(pk_filepath)?.as_slice(),
            der.as_slice(),
        )
    }
}

impl SKVerifyPath<&str, String> for SM2 {
    fn verify<P: AsRef<Path>>(msg: &str, pk_filepath: P, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            load_key_from_file(pk_filepath)?.as_slice(),
            Base64::decode(der)?.as_slice(),
        )
    }
}

impl SKVerifyPath<&str, &str> for SM2 {
    fn verify<P: AsRef<Path>>(msg: &str, pk_filepath: P, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            load_key_from_file(pk_filepath)?.as_slice(),
            Base64::decode(der.to_string())?.as_slice(),
        )
    }
}

////////// sm verify end //////////

fn stores<P: AsRef<Path>>(key: &[u8], key_filepath: P) -> GeorgeResult<()> {
    match Filer::write_force(key_filepath, key) {
        Ok(_) => Ok(()),
        Err(err) => Err(Errs::strs("store key", err)),
    }
}

fn store_hex_key<P: AsRef<Path>>(key: &[u8], key_filepath: P) -> GeorgeResult<()> {
    match Filer::write_force(key_filepath, Hex::encode(key)) {
        Ok(_) => Ok(()),
        Err(err) => Err(Errs::strs("store key", err)),
    }
}

fn store_hex_bytes_key<P: AsRef<Path>>(key: Vec<u8>, key_filepath: P) -> GeorgeResult<()> {
    match Filer::write_force(key_filepath, Hex::encode(key)) {
        Ok(_) => Ok(()),
        Err(err) => Err(Errs::strs("store key", err)),
    }
}

fn store_base64_key<P: AsRef<Path>>(key: &[u8], key_filepath: P) -> GeorgeResult<()> {
    match Filer::write_force(key_filepath, Base64::encode(key)) {
        Ok(_) => Ok(()),
        Err(err) => Err(Errs::strs("store key", err)),
    }
}

fn store_base64_bytes_key<P: AsRef<Path>>(key: Vec<u8>, key_filepath: P) -> GeorgeResult<()> {
    match Filer::write_force(key_filepath, Base64::encode(key)) {
        Ok(_) => Ok(()),
        Err(err) => Err(Errs::strs("store key", err)),
    }
}

fn store_key<P: AsRef<Path>>(key: String, key_filepath: P) -> GeorgeResult<()> {
    match Filer::write_force(key_filepath, key) {
        Ok(_) => Ok(()),
        Err(err) => Err(Errs::strs("store key", err)),
    }
}

fn load_key_string_from_file<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<String> {
    match read_to_string(key_filepath) {
        Ok(res) => Ok(res),
        Err(err) => Err(Errs::strs("read", err)),
    }
}

fn load_key_from_file<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<Vec<u8>> {
    match read_to_string(key_filepath) {
        Ok(res) => Ok(Base64::decode(res)?),
        Err(err) => Err(Errs::strs("read", err)),
    }
}

fn generate() -> (Vec<u8>, Vec<u8>) {
    let ctx = SigCtx::new();
    let (pk, sk) = ctx.new_keypair();
    (ctx.serialize_seckey(&sk), ctx.serialize_pubkey(&pk, true))
}

fn generate_hex() -> (String, String) {
    let (sk, pk) = generate();
    (Hex::encode(sk), Hex::encode(pk))
}

fn generate_base64() -> (String, String) {
    let (sk, pk) = generate();
    (Base64::encode(sk), Base64::encode(pk))
}

fn generate_sk() -> Vec<u8> {
    let ctx = SigCtx::new();
    let (_pk, sk) = ctx.new_keypair();
    ctx.serialize_seckey(&sk)
}

fn generate_pk_from_sk(sk: Vec<u8>) -> GeorgeResult<Vec<u8>> {
    let ctx = SigCtx::new();
    match ctx.load_seckey(sk.as_slice()) {
        Ok(p) => Ok(ctx.serialize_pubkey(&ctx.pk_from_sk(&p), true)),
        Err(err) => Err(Errs::string(format!("unknown {:#?}", err))),
    }
}

fn generate_pk_from_sk_hex(sk: String) -> GeorgeResult<Vec<u8>> {
    generate_pk_from_sk(Hex::decode(sk)?)
}

fn generate_pk_from_sk_base64(sk: String) -> GeorgeResult<Vec<u8>> {
    generate_pk_from_sk(Base64::decode(sk)?)
}

fn generate_pk_from_sk_hex_file<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Vec<u8>> {
    match read_to_string(sk_filepath) {
        Ok(sk) => generate_pk_from_sk_hex(sk),
        Err(err) => Err(Errs::strs("read to string", err)),
    }
}

fn generate_pk_from_sk_base64_file<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Vec<u8>> {
    match read_to_string(sk_filepath) {
        Ok(sk) => generate_pk_from_sk_base64(sk),
        Err(err) => Err(Errs::strs("read to string", err)),
    }
}

fn generate_in_file<P: AsRef<Path>>(
    sk_filepath: P,
    pk_filepath: P,
) -> GeorgeResult<(Vec<u8>, Vec<u8>)> {
    let (sk_bytes, pk_bytes) = generate();
    store_base64_bytes_key(sk_bytes.clone(), sk_filepath)?;
    store_base64_bytes_key(pk_bytes.clone(), pk_filepath)?;
    Ok((sk_bytes, pk_bytes))
}

fn generate_hex_in_file<P: AsRef<Path>>(
    sk_filepath: P,
    pk_filepath: P,
) -> GeorgeResult<(String, String)> {
    let (sk_str, pk_str) = generate_hex();
    store_key(sk_str.clone(), sk_filepath)?;
    store_key(pk_str.clone(), pk_filepath)?;
    Ok((sk_str, pk_str))
}

fn generate_base64_in_file<P: AsRef<Path>>(
    sk_filepath: P,
    pk_filepath: P,
) -> GeorgeResult<(String, String)> {
    let (sk_str, pk_str) = generate_base64();
    store_key(sk_str.clone(), sk_filepath)?;
    store_key(pk_str.clone(), pk_filepath)?;
    Ok((sk_str, pk_str))
}

fn generate_sk_in_file<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Vec<u8>> {
    let (sk_bytes, _pk_bytes) = generate();
    store_base64_bytes_key(sk_bytes.clone(), sk_filepath)?;
    Ok(sk_bytes)
}

fn generate_sk_hex_in_file<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<String> {
    let (sk_str, _pk_str) = generate_hex();
    store_key(sk_str.clone(), sk_filepath)?;
    Ok(sk_str)
}

fn generate_sk_base64_in_file<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<String> {
    let (sk_str, _pk_str) = generate_base64();
    store_key(sk_str.clone(), sk_filepath)?;
    Ok(sk_str)
}

fn sign(msg: &[u8], sk: &[u8], pk: &[u8]) -> GeorgeResult<Vec<u8>> {
    let ctx = SigCtx::new();
    let pk_point: Point;
    let sig: Signature;
    match ctx.load_pubkey(pk) {
        Ok(pp) => pk_point = pp,
        Err(err) => return Err(Errs::string(format!("load pub key error! {:#?}", err))),
    }
    match ctx.load_seckey(sk) {
        Ok(sk_bu) => sig = ctx.sign(msg, &sk_bu, &pk_point),
        Err(err) => return Err(Errs::string(format!("load pub key error! {:#?}", err))),
    }
    Ok(sig.der_encode())
}

fn verify(msg: &[u8], pk: &[u8], der: &[u8]) -> GeorgeResult<bool> {
    let ctx = SigCtx::new();
    let pk_point: Point;
    let sig: Signature;
    match ctx.load_pubkey(pk) {
        Ok(pp) => pk_point = pp,
        Err(err) => return Err(Errs::string(format!("load pub key error! {:#?}", err))),
    }
    match Signature::der_decode(der) {
        Ok(s) => sig = s,
        Err(err) => return Err(Errs::strs("der decode", err)),
    }
    Ok(ctx.verify(msg, &pk_point, &sig))
}
