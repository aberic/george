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

use crate::errors::entrances::GeorgeResult;
use crate::errors::entrances::{err_str, err_strs};
use crate::io::file::{Filer, FilerWriter};

pub struct SM2;

pub trait SM2New {
    /// 生成SM公私钥，返回sk、pk字节数组
    fn generate() -> (Vec<u8>, Vec<u8>);
    /// 生成SM公私钥，返回sk、pk字符串
    fn generate_string() -> (String, String);
}

pub trait SM2NewStore<T> {
    /// 生成SM公私钥，返回sk、pk字节数组
    ///
    /// 并将生成的公私钥存储在sk、pk指定文件中
    fn generate(sk_filepath: T, pk_filepath: T) -> GeorgeResult<(Vec<u8>, Vec<u8>)>;
    /// 生成SM公私钥，返回sk、pk字符串
    ///
    /// 并将生成的公私钥存储在sk、pk指定文件中
    fn generate_string(sk_filepath: T, pk_filepath: T) -> GeorgeResult<(String, String)>;
}

pub trait SM2PKV8s<T> {
    /// 根据私钥生成公钥
    fn generate_pk(sk: T) -> GeorgeResult<Vec<u8>>;
}

pub trait SM2PKString<T> {
    /// 根据私钥生成公钥
    fn generate_pk(sk: T) -> GeorgeResult<String>;
}

pub trait SM2PKV8sPath {
    /// 根据私钥文件生成公钥
    fn generate_pk<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Vec<u8>>;
}

pub trait SM2PKStringPath {
    /// 根据私钥文件生成公钥
    fn generate_pk<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<String>;
}

pub trait SM2KeyHex {
    fn key_encode(key: Vec<u8>) -> String;
    fn key_decode(key: String) -> GeorgeResult<Vec<u8>>;
}

pub trait SM2StoreKey<P> {
    fn store(key: &[u8], key_filepath: P) -> GeorgeResult<()>;
    fn store_bytes(key: Vec<u8>, key_filepath: P) -> GeorgeResult<()>;
    fn store_str(key: &str, key_filepath: P) -> GeorgeResult<()>;
    fn store_string(key: String, key_filepath: P) -> GeorgeResult<()>;
}

pub trait SM2LoadKey {
    fn load_from_file<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<Vec<u8>>;
    fn load_string_from_file<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<String>;
}

pub trait SM2Sign<M, N> {
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

pub trait SM2SignPath<T> {
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

pub trait SM2Verify<M, N, O> {
    /// 验签msg
    fn verify(msg: M, pk: N, der: O) -> GeorgeResult<bool>;
}

pub trait SM2VerifyPath<M, N> {
    fn verify<P: AsRef<Path>>(msg: M, sk_filepath: P, der: N) -> GeorgeResult<bool>;
}

////////// sm generate start //////////

impl SM2New for SM2 {
    fn generate() -> (Vec<u8>, Vec<u8>) {
        generate()
    }

    fn generate_string() -> (String, String) {
        generate_hex()
    }
}

impl SM2NewStore<String> for SM2 {
    fn generate(sk_filepath: String, pk_filepath: String) -> GeorgeResult<(Vec<u8>, Vec<u8>)> {
        generate_in_file(sk_filepath, pk_filepath)
    }

    fn generate_string(sk_filepath: String, pk_filepath: String) -> GeorgeResult<(String, String)> {
        generate_hex_in_file(sk_filepath, pk_filepath)
    }
}

impl SM2NewStore<&str> for SM2 {
    fn generate(sk_filepath: &str, pk_filepath: &str) -> GeorgeResult<(Vec<u8>, Vec<u8>)> {
        generate_in_file(sk_filepath.to_string(), pk_filepath.to_string())
    }

    fn generate_string(sk_filepath: &str, pk_filepath: &str) -> GeorgeResult<(String, String)> {
        generate_hex_in_file(sk_filepath.to_string(), pk_filepath.to_string())
    }
}

////////// sm generate end //////////

////////// sm generate pk from sk start //////////
impl SM2PKV8s<Vec<u8>> for SM2 {
    fn generate_pk(sk: Vec<u8>) -> GeorgeResult<Vec<u8>> {
        generate_pk_from_sk(sk)
    }
}

impl SM2PKV8s<String> for SM2 {
    fn generate_pk(sk: String) -> GeorgeResult<Vec<u8>> {
        generate_pk_from_sk_str(sk)
    }
}

impl SM2PKString<Vec<u8>> for SM2 {
    fn generate_pk(sk: Vec<u8>) -> GeorgeResult<String> {
        Ok(key_to_string(generate_pk_from_sk(sk)?))
    }
}

impl SM2PKString<String> for SM2 {
    fn generate_pk(sk: String) -> GeorgeResult<String> {
        Ok(key_to_string(generate_pk_from_sk_str(sk)?))
    }
}

impl SM2PKV8sPath for SM2 {
    fn generate_pk<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Vec<u8>> {
        generate_pk_from_sk_file(sk_filepath)
    }
}

impl SM2PKStringPath for SM2 {
    fn generate_pk<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<String> {
        Ok(key_to_string(generate_pk_from_sk_file(sk_filepath)?))
    }
}

////////// sm generate pk from sk end //////////

////////// sm store/load start //////////

impl SM2StoreKey<String> for SM2 {
    fn store(key: &[u8], key_filepath: String) -> GeorgeResult<()> {
        store_key(key_to_string(key.to_vec()), key_filepath)
    }

    fn store_bytes(key: Vec<u8>, key_filepath: String) -> GeorgeResult<()> {
        store_key(key_to_string(key), key_filepath)
    }

    fn store_str(key: &str, key_filepath: String) -> GeorgeResult<()> {
        store_key(key.to_string(), key_filepath)
    }

    fn store_string(key: String, key_filepath: String) -> GeorgeResult<()> {
        store_key(key, key_filepath)
    }
}

impl SM2StoreKey<&str> for SM2 {
    fn store(key: &[u8], key_filepath: &str) -> GeorgeResult<()> {
        store_key(key_to_string(key.to_vec()), key_filepath.to_string())
    }

    fn store_bytes(key: Vec<u8>, key_filepath: &str) -> GeorgeResult<()> {
        store_key(key_to_string(key), key_filepath.to_string())
    }

    fn store_str(key: &str, key_filepath: &str) -> GeorgeResult<()> {
        store_key(key.to_string(), key_filepath.to_string())
    }

    fn store_string(key: String, key_filepath: &str) -> GeorgeResult<()> {
        store_key(key, key_filepath.to_string())
    }
}

impl SM2LoadKey for SM2 {
    fn load_from_file<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<Vec<u8>> {
        load_key_from_file(key_filepath)
    }

    fn load_string_from_file<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<String> {
        load_key_string_from_file(key_filepath)
    }
}

////////// sm store/load end //////////

////////// sm sign start //////////

impl SM2Sign<&[u8], &[u8]> for SM2 {
    fn sign(msg: &[u8], sk: &[u8], pk: &[u8]) -> GeorgeResult<Vec<u8>> {
        sign(msg, sk, pk)
    }

    fn sign_string(msg: &[u8], sk: &[u8], pk: &[u8]) -> GeorgeResult<String> {
        Ok(key_to_string(sign(msg, sk, pk)?))
    }
}

impl SM2Sign<&[u8], Vec<u8>> for SM2 {
    fn sign(msg: &[u8], sk: Vec<u8>, pk: Vec<u8>) -> GeorgeResult<Vec<u8>> {
        sign(msg, sk.as_slice(), pk.as_slice())
    }

    fn sign_string(msg: &[u8], sk: Vec<u8>, pk: Vec<u8>) -> GeorgeResult<String> {
        Ok(key_to_string(sign(msg, sk.as_slice(), pk.as_slice())?))
    }
}

impl SM2Sign<Vec<u8>, Vec<u8>> for SM2 {
    fn sign(msg: Vec<u8>, sk: Vec<u8>, pk: Vec<u8>) -> GeorgeResult<Vec<u8>> {
        sign(msg.as_slice(), sk.as_slice(), pk.as_slice())
    }

    fn sign_string(msg: Vec<u8>, sk: Vec<u8>, pk: Vec<u8>) -> GeorgeResult<String> {
        Ok(key_to_string(sign(
            msg.as_slice(),
            sk.as_slice(),
            pk.as_slice(),
        )?))
    }
}

impl SM2Sign<String, Vec<u8>> for SM2 {
    fn sign(msg: String, sk: Vec<u8>, pk: Vec<u8>) -> GeorgeResult<Vec<u8>> {
        sign(msg.as_bytes(), sk.as_slice(), pk.as_slice())
    }

    fn sign_string(msg: String, sk: Vec<u8>, pk: Vec<u8>) -> GeorgeResult<String> {
        Ok(key_to_string(sign(
            msg.as_bytes(),
            sk.as_slice(),
            pk.as_slice(),
        )?))
    }
}

impl SM2Sign<&str, Vec<u8>> for SM2 {
    fn sign(msg: &str, sk: Vec<u8>, pk: Vec<u8>) -> GeorgeResult<Vec<u8>> {
        sign(msg.as_bytes(), sk.as_slice(), pk.as_slice())
    }

    fn sign_string(msg: &str, sk: Vec<u8>, pk: Vec<u8>) -> GeorgeResult<String> {
        Ok(key_to_string(sign(
            msg.as_bytes(),
            sk.as_slice(),
            pk.as_slice(),
        )?))
    }
}

impl SM2Sign<Vec<u8>, &[u8]> for SM2 {
    fn sign(msg: Vec<u8>, sk: &[u8], pk: &[u8]) -> GeorgeResult<Vec<u8>> {
        sign(msg.as_slice(), sk, pk)
    }

    fn sign_string(msg: Vec<u8>, sk: &[u8], pk: &[u8]) -> GeorgeResult<String> {
        Ok(key_to_string(sign(msg.as_slice(), sk, pk)?))
    }
}

impl SM2Sign<String, &[u8]> for SM2 {
    fn sign(msg: String, sk: &[u8], pk: &[u8]) -> GeorgeResult<Vec<u8>> {
        sign(msg.as_bytes(), sk, pk)
    }

    fn sign_string(msg: String, sk: &[u8], pk: &[u8]) -> GeorgeResult<String> {
        Ok(key_to_string(sign(msg.as_bytes(), sk, pk)?))
    }
}

impl SM2Sign<&str, &[u8]> for SM2 {
    fn sign(msg: &str, sk: &[u8], pk: &[u8]) -> GeorgeResult<Vec<u8>> {
        sign(msg.as_bytes(), sk, pk)
    }

    fn sign_string(msg: &str, sk: &[u8], pk: &[u8]) -> GeorgeResult<String> {
        Ok(key_to_string(sign(msg.as_bytes(), sk, pk)?))
    }
}

impl SM2Sign<&[u8], String> for SM2 {
    fn sign(msg: &[u8], sk: String, pk: String) -> GeorgeResult<Vec<u8>> {
        sign(
            msg,
            &key_from_string(sk)?.as_slice(),
            &key_from_string(pk)?.as_slice(),
        )
    }

    fn sign_string(msg: &[u8], sk: String, pk: String) -> GeorgeResult<String> {
        Ok(key_to_string(sign(
            msg,
            key_from_string(sk)?.as_slice(),
            key_from_string(pk)?.as_slice(),
        )?))
    }
}

impl SM2Sign<Vec<u8>, String> for SM2 {
    fn sign(msg: Vec<u8>, sk: String, pk: String) -> GeorgeResult<Vec<u8>> {
        sign(
            msg.as_slice(),
            &key_from_string(sk)?.as_slice(),
            &key_from_string(pk)?.as_slice(),
        )
    }

    fn sign_string(msg: Vec<u8>, sk: String, pk: String) -> GeorgeResult<String> {
        Ok(key_to_string(sign(
            msg.as_slice(),
            key_from_string(sk)?.as_slice(),
            key_from_string(pk)?.as_slice(),
        )?))
    }
}

impl SM2Sign<String, String> for SM2 {
    fn sign(msg: String, sk: String, pk: String) -> GeorgeResult<Vec<u8>> {
        sign(
            msg.as_bytes(),
            &key_from_string(sk)?.as_slice(),
            &key_from_string(pk)?.as_slice(),
        )
    }

    fn sign_string(msg: String, sk: String, pk: String) -> GeorgeResult<String> {
        Ok(key_to_string(sign(
            msg.as_bytes(),
            key_from_string(sk)?.as_slice(),
            key_from_string(pk)?.as_slice(),
        )?))
    }
}

impl SM2Sign<&str, String> for SM2 {
    fn sign(msg: &str, sk: String, pk: String) -> GeorgeResult<Vec<u8>> {
        sign(
            msg.as_bytes(),
            &key_from_string(sk)?.as_slice(),
            &key_from_string(pk)?.as_slice(),
        )
    }

    fn sign_string(msg: &str, sk: String, pk: String) -> GeorgeResult<String> {
        Ok(key_to_string(sign(
            msg.as_bytes(),
            key_from_string(sk)?.as_slice(),
            key_from_string(pk)?.as_slice(),
        )?))
    }
}

impl SM2Sign<&[u8], &str> for SM2 {
    fn sign(msg: &[u8], sk: &str, pk: &str) -> GeorgeResult<Vec<u8>> {
        sign(
            msg,
            &key_from_string(sk.to_string())?.as_slice(),
            &key_from_string(pk.to_string())?.as_slice(),
        )
    }

    fn sign_string(msg: &[u8], sk: &str, pk: &str) -> GeorgeResult<String> {
        Ok(key_to_string(sign(
            msg,
            key_from_string(sk.to_string())?.as_slice(),
            key_from_string(pk.to_string())?.as_slice(),
        )?))
    }
}

impl SM2Sign<Vec<u8>, &str> for SM2 {
    fn sign(msg: Vec<u8>, sk: &str, pk: &str) -> GeorgeResult<Vec<u8>> {
        sign(
            msg.as_slice(),
            &key_from_string(sk.to_string())?.as_slice(),
            &key_from_string(pk.to_string())?.as_slice(),
        )
    }

    fn sign_string(msg: Vec<u8>, sk: &str, pk: &str) -> GeorgeResult<String> {
        Ok(key_to_string(sign(
            msg.as_slice(),
            key_from_string(sk.to_string())?.as_slice(),
            key_from_string(pk.to_string())?.as_slice(),
        )?))
    }
}

impl SM2Sign<String, &str> for SM2 {
    fn sign(msg: String, sk: &str, pk: &str) -> GeorgeResult<Vec<u8>> {
        sign(
            msg.as_bytes(),
            &key_from_string(sk.to_string())?.as_slice(),
            &key_from_string(pk.to_string())?.as_slice(),
        )
    }

    fn sign_string(msg: String, sk: &str, pk: &str) -> GeorgeResult<String> {
        Ok(key_to_string(sign(
            msg.as_bytes(),
            key_from_string(sk.to_string())?.as_slice(),
            key_from_string(pk.to_string())?.as_slice(),
        )?))
    }
}

impl SM2Sign<&str, &str> for SM2 {
    fn sign(msg: &str, sk: &str, pk: &str) -> GeorgeResult<Vec<u8>> {
        sign(
            msg.as_bytes(),
            &key_from_string(sk.to_string())?.as_slice(),
            &key_from_string(pk.to_string())?.as_slice(),
        )
    }

    fn sign_string(msg: &str, sk: &str, pk: &str) -> GeorgeResult<String> {
        Ok(key_to_string(sign(
            msg.as_bytes(),
            key_from_string(sk.to_string())?.as_slice(),
            key_from_string(pk.to_string())?.as_slice(),
        )?))
    }
}

impl SM2SignPath<&[u8]> for SM2 {
    fn sign<P: AsRef<Path>>(msg: &[u8], sk_filepath: P, pk_filepath: P) -> GeorgeResult<Vec<u8>> {
        sign(
            msg,
            load_key_from_file(sk_filepath)?.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
        )
    }

    fn sign_string<P: AsRef<Path>>(
        msg: &[u8],
        sk_filepath: P,
        pk_filepath: P,
    ) -> GeorgeResult<String> {
        Ok(key_to_string(sign(
            msg,
            load_key_from_file(sk_filepath)?.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
        )?))
    }
}

impl SM2SignPath<Vec<u8>> for SM2 {
    fn sign<P: AsRef<Path>>(msg: Vec<u8>, sk_filepath: P, pk_filepath: P) -> GeorgeResult<Vec<u8>> {
        sign(
            msg.as_slice(),
            load_key_from_file(sk_filepath)?.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
        )
    }

    fn sign_string<P: AsRef<Path>>(
        msg: Vec<u8>,
        sk_filepath: P,
        pk_filepath: P,
    ) -> GeorgeResult<String> {
        Ok(key_to_string(sign(
            msg.as_slice(),
            load_key_from_file(sk_filepath)?.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
        )?))
    }
}

impl SM2SignPath<String> for SM2 {
    fn sign<P: AsRef<Path>>(msg: String, sk_filepath: P, pk_filepath: P) -> GeorgeResult<Vec<u8>> {
        sign(
            msg.as_bytes(),
            load_key_from_file(sk_filepath)?.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
        )
    }

    fn sign_string<P: AsRef<Path>>(
        msg: String,
        sk_filepath: P,
        pk_filepath: P,
    ) -> GeorgeResult<String> {
        Ok(key_to_string(sign(
            msg.as_bytes(),
            load_key_from_file(sk_filepath)?.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
        )?))
    }
}

impl SM2SignPath<&str> for SM2 {
    fn sign<P: AsRef<Path>>(msg: &str, sk_filepath: P, pk_filepath: P) -> GeorgeResult<Vec<u8>> {
        sign(
            msg.as_bytes(),
            load_key_from_file(sk_filepath)?.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
        )
    }

    fn sign_string<P: AsRef<Path>>(
        msg: &str,
        sk_filepath: P,
        pk_filepath: P,
    ) -> GeorgeResult<String> {
        Ok(key_to_string(sign(
            msg.as_bytes(),
            load_key_from_file(sk_filepath)?.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
        )?))
    }
}

////////// sm sign end //////////

////////// sm verify start //////////

impl SM2Verify<&[u8], &[u8], &[u8]> for SM2 {
    fn verify(msg: &[u8], pk: &[u8], der: &[u8]) -> GeorgeResult<bool> {
        verify(msg, pk, der)
    }
}

impl SM2Verify<&[u8], &[u8], Vec<u8>> for SM2 {
    fn verify(msg: &[u8], pk: &[u8], der: Vec<u8>) -> GeorgeResult<bool> {
        verify(msg, pk, der.as_slice())
    }
}

impl SM2Verify<&[u8], &[u8], String> for SM2 {
    fn verify(msg: &[u8], pk: &[u8], der: String) -> GeorgeResult<bool> {
        verify(msg, pk, key_from_string(der)?.as_slice())
    }
}

impl SM2Verify<&[u8], &[u8], &str> for SM2 {
    fn verify(msg: &[u8], pk: &[u8], der: &str) -> GeorgeResult<bool> {
        verify(msg, pk, key_from_string(der.to_string())?.as_slice())
    }
}

impl SM2Verify<&[u8], Vec<u8>, &[u8]> for SM2 {
    fn verify(msg: &[u8], pk: Vec<u8>, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg, pk.as_slice(), der)
    }
}

impl SM2Verify<&[u8], Vec<u8>, Vec<u8>> for SM2 {
    fn verify(msg: &[u8], pk: Vec<u8>, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(msg, pk.as_slice(), der.as_slice())
    }
}

impl SM2Verify<&[u8], Vec<u8>, String> for SM2 {
    fn verify(msg: &[u8], pk: Vec<u8>, der: String) -> GeorgeResult<bool> {
        verify(msg, pk.as_slice(), key_from_string(der)?.as_slice())
    }
}

impl SM2Verify<&[u8], Vec<u8>, &str> for SM2 {
    fn verify(msg: &[u8], pk: Vec<u8>, der: &str) -> GeorgeResult<bool> {
        verify(
            msg,
            pk.as_slice(),
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl SM2Verify<&[u8], String, &[u8]> for SM2 {
    fn verify(msg: &[u8], pk: String, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg, &key_from_string(pk)?.as_slice(), der)
    }
}

impl SM2Verify<&[u8], String, Vec<u8>> for SM2 {
    fn verify(msg: &[u8], pk: String, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(msg, &key_from_string(pk)?.as_slice(), der.as_slice())
    }
}

impl SM2Verify<&[u8], String, String> for SM2 {
    fn verify(msg: &[u8], pk: String, der: String) -> GeorgeResult<bool> {
        verify(
            msg,
            &key_from_string(pk)?.as_slice(),
            &key_from_string(der)?.as_slice(),
        )
    }
}

impl SM2Verify<&[u8], String, &str> for SM2 {
    fn verify(msg: &[u8], pk: String, der: &str) -> GeorgeResult<bool> {
        verify(
            msg,
            &key_from_string(pk)?.as_slice(),
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl SM2Verify<&[u8], &str, &[u8]> for SM2 {
    fn verify(msg: &[u8], pk: &str, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg, &key_from_string(pk.to_string())?.as_slice(), der)
    }
}

impl SM2Verify<&[u8], &str, Vec<u8>> for SM2 {
    fn verify(msg: &[u8], pk: &str, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg,
            &key_from_string(pk.to_string())?.as_slice(),
            der.as_slice(),
        )
    }
}

impl SM2Verify<&[u8], &str, String> for SM2 {
    fn verify(msg: &[u8], pk: &str, der: String) -> GeorgeResult<bool> {
        verify(
            msg,
            &key_from_string(pk.to_string())?.as_slice(),
            key_from_string(der)?.as_slice(),
        )
    }
}

impl SM2Verify<&[u8], &str, &str> for SM2 {
    fn verify(msg: &[u8], pk: &str, der: &str) -> GeorgeResult<bool> {
        verify(
            msg,
            &key_from_string(pk.to_string())?.as_slice(),
            &key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl SM2Verify<Vec<u8>, &[u8], &[u8]> for SM2 {
    fn verify(msg: Vec<u8>, pk: &[u8], der: &[u8]) -> GeorgeResult<bool> {
        verify(msg.as_slice(), pk, der)
    }
}

impl SM2Verify<Vec<u8>, &[u8], Vec<u8>> for SM2 {
    fn verify(msg: Vec<u8>, pk: &[u8], der: Vec<u8>) -> GeorgeResult<bool> {
        verify(msg.as_slice(), pk, der.as_slice())
    }
}

impl SM2Verify<Vec<u8>, &[u8], String> for SM2 {
    fn verify(msg: Vec<u8>, pk: &[u8], der: String) -> GeorgeResult<bool> {
        verify(msg.as_slice(), pk, key_from_string(der)?.as_slice())
    }
}

impl SM2Verify<Vec<u8>, &[u8], &str> for SM2 {
    fn verify(msg: Vec<u8>, pk: &[u8], der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            pk,
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl SM2Verify<Vec<u8>, Vec<u8>, &[u8]> for SM2 {
    fn verify(msg: Vec<u8>, pk: Vec<u8>, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg.as_slice(), pk.as_slice(), der)
    }
}

impl SM2Verify<Vec<u8>, Vec<u8>, Vec<u8>> for SM2 {
    fn verify(msg: Vec<u8>, pk: Vec<u8>, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(msg.as_slice(), pk.as_slice(), der.as_slice())
    }
}

impl SM2Verify<Vec<u8>, Vec<u8>, String> for SM2 {
    fn verify(msg: Vec<u8>, pk: Vec<u8>, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            pk.as_slice(),
            key_from_string(der)?.as_slice(),
        )
    }
}

impl SM2Verify<Vec<u8>, Vec<u8>, &str> for SM2 {
    fn verify(msg: Vec<u8>, pk: Vec<u8>, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            pk.as_slice(),
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl SM2Verify<Vec<u8>, String, &[u8]> for SM2 {
    fn verify(msg: Vec<u8>, pk: String, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg.as_slice(), &key_from_string(pk)?.as_slice(), der)
    }
}

impl SM2Verify<Vec<u8>, String, Vec<u8>> for SM2 {
    fn verify(msg: Vec<u8>, pk: String, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            &key_from_string(pk)?.as_slice(),
            der.as_slice(),
        )
    }
}

impl SM2Verify<Vec<u8>, String, String> for SM2 {
    fn verify(msg: Vec<u8>, pk: String, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            &key_from_string(pk)?.as_slice(),
            &key_from_string(der)?.as_slice(),
        )
    }
}

impl SM2Verify<Vec<u8>, String, &str> for SM2 {
    fn verify(msg: Vec<u8>, pk: String, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            &key_from_string(pk)?.as_slice(),
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl SM2Verify<Vec<u8>, &str, &[u8]> for SM2 {
    fn verify(msg: Vec<u8>, pk: &str, der: &[u8]) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            &key_from_string(pk.to_string())?.as_slice(),
            der,
        )
    }
}

impl SM2Verify<Vec<u8>, &str, Vec<u8>> for SM2 {
    fn verify(msg: Vec<u8>, pk: &str, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            &key_from_string(pk.to_string())?.as_slice(),
            der.as_slice(),
        )
    }
}

impl SM2Verify<Vec<u8>, &str, String> for SM2 {
    fn verify(msg: Vec<u8>, pk: &str, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            &key_from_string(pk.to_string())?.as_slice(),
            key_from_string(der)?.as_slice(),
        )
    }
}

impl SM2Verify<Vec<u8>, &str, &str> for SM2 {
    fn verify(msg: Vec<u8>, pk: &str, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            &key_from_string(pk.to_string())?.as_slice(),
            &key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl SM2Verify<String, &[u8], &[u8]> for SM2 {
    fn verify(msg: String, pk: &[u8], der: &[u8]) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk, der)
    }
}

impl SM2Verify<String, &[u8], Vec<u8>> for SM2 {
    fn verify(msg: String, pk: &[u8], der: Vec<u8>) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk, der.as_slice())
    }
}

impl SM2Verify<String, &[u8], String> for SM2 {
    fn verify(msg: String, pk: &[u8], der: String) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk, key_from_string(der)?.as_slice())
    }
}

impl SM2Verify<String, &[u8], &str> for SM2 {
    fn verify(msg: String, pk: &[u8], der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            pk,
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl SM2Verify<String, Vec<u8>, &[u8]> for SM2 {
    fn verify(msg: String, pk: Vec<u8>, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk.as_slice(), der)
    }
}

impl SM2Verify<String, Vec<u8>, Vec<u8>> for SM2 {
    fn verify(msg: String, pk: Vec<u8>, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk.as_slice(), der.as_slice())
    }
}

impl SM2Verify<String, Vec<u8>, String> for SM2 {
    fn verify(msg: String, pk: Vec<u8>, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            pk.as_slice(),
            key_from_string(der)?.as_slice(),
        )
    }
}

impl SM2Verify<String, Vec<u8>, &str> for SM2 {
    fn verify(msg: String, pk: Vec<u8>, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            pk.as_slice(),
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl SM2Verify<String, String, &[u8]> for SM2 {
    fn verify(msg: String, pk: String, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), &key_from_string(pk)?.as_slice(), der)
    }
}

impl SM2Verify<String, String, Vec<u8>> for SM2 {
    fn verify(msg: String, pk: String, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk)?.as_slice(),
            der.as_slice(),
        )
    }
}

impl SM2Verify<String, String, String> for SM2 {
    fn verify(msg: String, pk: String, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk)?.as_slice(),
            &key_from_string(der)?.as_slice(),
        )
    }
}

impl SM2Verify<String, String, &str> for SM2 {
    fn verify(msg: String, pk: String, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk)?.as_slice(),
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl SM2Verify<String, &str, &[u8]> for SM2 {
    fn verify(msg: String, pk: &str, der: &[u8]) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk.to_string())?.as_slice(),
            der,
        )
    }
}

impl SM2Verify<String, &str, Vec<u8>> for SM2 {
    fn verify(msg: String, pk: &str, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk.to_string())?.as_slice(),
            der.as_slice(),
        )
    }
}

impl SM2Verify<String, &str, String> for SM2 {
    fn verify(msg: String, pk: &str, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk.to_string())?.as_slice(),
            key_from_string(der)?.as_slice(),
        )
    }
}

impl SM2Verify<String, &str, &str> for SM2 {
    fn verify(msg: String, pk: &str, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk.to_string())?.as_slice(),
            &key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl SM2Verify<&str, &[u8], &[u8]> for SM2 {
    fn verify(msg: &str, pk: &[u8], der: &[u8]) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk, der)
    }
}

impl SM2Verify<&str, &[u8], Vec<u8>> for SM2 {
    fn verify(msg: &str, pk: &[u8], der: Vec<u8>) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk, der.as_slice())
    }
}

impl SM2Verify<&str, &[u8], String> for SM2 {
    fn verify(msg: &str, pk: &[u8], der: String) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk, key_from_string(der)?.as_slice())
    }
}

impl SM2Verify<&str, &[u8], &str> for SM2 {
    fn verify(msg: &str, pk: &[u8], der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            pk,
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl SM2Verify<&str, Vec<u8>, &[u8]> for SM2 {
    fn verify(msg: &str, pk: Vec<u8>, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk.as_slice(), der)
    }
}

impl SM2Verify<&str, Vec<u8>, Vec<u8>> for SM2 {
    fn verify(msg: &str, pk: Vec<u8>, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk.as_slice(), der.as_slice())
    }
}

impl SM2Verify<&str, Vec<u8>, String> for SM2 {
    fn verify(msg: &str, pk: Vec<u8>, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            pk.as_slice(),
            key_from_string(der)?.as_slice(),
        )
    }
}

impl SM2Verify<&str, Vec<u8>, &str> for SM2 {
    fn verify(msg: &str, pk: Vec<u8>, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            pk.as_slice(),
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl SM2Verify<&str, String, &[u8]> for SM2 {
    fn verify(msg: &str, pk: String, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), &key_from_string(pk)?.as_slice(), der)
    }
}

impl SM2Verify<&str, String, Vec<u8>> for SM2 {
    fn verify(msg: &str, pk: String, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk)?.as_slice(),
            der.as_slice(),
        )
    }
}

impl SM2Verify<&str, String, String> for SM2 {
    fn verify(msg: &str, pk: String, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk)?.as_slice(),
            &key_from_string(der)?.as_slice(),
        )
    }
}

impl SM2Verify<&str, String, &str> for SM2 {
    fn verify(msg: &str, pk: String, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk)?.as_slice(),
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl SM2Verify<&str, &str, &[u8]> for SM2 {
    fn verify(msg: &str, pk: &str, der: &[u8]) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk.to_string())?.as_slice(),
            der,
        )
    }
}

impl SM2Verify<&str, &str, Vec<u8>> for SM2 {
    fn verify(msg: &str, pk: &str, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk.to_string())?.as_slice(),
            der.as_slice(),
        )
    }
}

impl SM2Verify<&str, &str, String> for SM2 {
    fn verify(msg: &str, pk: &str, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk.to_string())?.as_slice(),
            key_from_string(der)?.as_slice(),
        )
    }
}

impl SM2Verify<&str, &str, &str> for SM2 {
    fn verify(msg: &str, pk: &str, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk.to_string())?.as_slice(),
            &key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl SM2VerifyPath<&[u8], &[u8]> for SM2 {
    fn verify<P: AsRef<Path>>(msg: &[u8], pk_filepath: P, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg, load_key_from_file(pk_filepath)?.as_slice(), der)
    }
}

impl SM2VerifyPath<&[u8], Vec<u8>> for SM2 {
    fn verify<P: AsRef<Path>>(msg: &[u8], pk_filepath: P, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg,
            load_key_from_file(pk_filepath)?.as_slice(),
            der.as_slice(),
        )
    }
}

impl SM2VerifyPath<&[u8], String> for SM2 {
    fn verify<P: AsRef<Path>>(msg: &[u8], pk_filepath: P, der: String) -> GeorgeResult<bool> {
        verify(
            msg,
            load_key_from_file(pk_filepath)?.as_slice(),
            key_from_string(der)?.as_slice(),
        )
    }
}

impl SM2VerifyPath<&[u8], &str> for SM2 {
    fn verify<P: AsRef<Path>>(msg: &[u8], pk_filepath: P, der: &str) -> GeorgeResult<bool> {
        verify(
            msg,
            load_key_from_file(pk_filepath)?.as_slice(),
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl SM2VerifyPath<Vec<u8>, &[u8]> for SM2 {
    fn verify<P: AsRef<Path>>(msg: Vec<u8>, pk_filepath: P, der: &[u8]) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
            der,
        )
    }
}

impl SM2VerifyPath<Vec<u8>, Vec<u8>> for SM2 {
    fn verify<P: AsRef<Path>>(msg: Vec<u8>, pk_filepath: P, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
            der.as_slice(),
        )
    }
}

impl SM2VerifyPath<Vec<u8>, String> for SM2 {
    fn verify<P: AsRef<Path>>(msg: Vec<u8>, pk_filepath: P, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
            key_from_string(der)?.as_slice(),
        )
    }
}

impl SM2VerifyPath<Vec<u8>, &str> for SM2 {
    fn verify<P: AsRef<Path>>(msg: Vec<u8>, pk_filepath: P, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl SM2VerifyPath<String, &[u8]> for SM2 {
    fn verify<P: AsRef<Path>>(msg: String, pk_filepath: P, der: &[u8]) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            load_key_from_file(pk_filepath)?.as_slice(),
            der,
        )
    }
}

impl SM2VerifyPath<String, Vec<u8>> for SM2 {
    fn verify<P: AsRef<Path>>(msg: String, pk_filepath: P, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            load_key_from_file(pk_filepath)?.as_slice(),
            der.as_slice(),
        )
    }
}

impl SM2VerifyPath<String, String> for SM2 {
    fn verify<P: AsRef<Path>>(msg: String, pk_filepath: P, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            load_key_from_file(pk_filepath)?.as_slice(),
            key_from_string(der)?.as_slice(),
        )
    }
}

impl SM2VerifyPath<String, &str> for SM2 {
    fn verify<P: AsRef<Path>>(msg: String, pk_filepath: P, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            load_key_from_file(pk_filepath)?.as_slice(),
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl SM2VerifyPath<&str, &[u8]> for SM2 {
    fn verify<P: AsRef<Path>>(msg: &str, pk_filepath: P, der: &[u8]) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            load_key_from_file(pk_filepath)?.as_slice(),
            der,
        )
    }
}

impl SM2VerifyPath<&str, Vec<u8>> for SM2 {
    fn verify<P: AsRef<Path>>(msg: &str, pk_filepath: P, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            load_key_from_file(pk_filepath)?.as_slice(),
            der.as_slice(),
        )
    }
}

impl SM2VerifyPath<&str, String> for SM2 {
    fn verify<P: AsRef<Path>>(msg: &str, pk_filepath: P, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            load_key_from_file(pk_filepath)?.as_slice(),
            key_from_string(der)?.as_slice(),
        )
    }
}

impl SM2VerifyPath<&str, &str> for SM2 {
    fn verify<P: AsRef<Path>>(msg: &str, pk_filepath: P, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            load_key_from_file(pk_filepath)?.as_slice(),
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

////////// sm verify end //////////

fn store_key(key: String, key_filepath: String) -> GeorgeResult<()> {
    match Filer::write_force(key_filepath, key) {
        Ok(_) => Ok(()),
        Err(err) => Err(err_strs("store key", err)),
    }
}

fn load_key_string_from_file<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<String> {
    match read_to_string(key_filepath) {
        Ok(res) => Ok(res),
        Err(err) => Err(err_strs("read", err)),
    }
}

fn load_key_from_file<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<Vec<u8>> {
    match read_to_string(key_filepath) {
        Ok(res) => Ok(key_from_string(res)?),
        Err(err) => Err(err_strs("read", err)),
    }
}

fn key_to_string(key: Vec<u8>) -> String {
    hex::encode(key)
}

fn key_from_string(key: String) -> GeorgeResult<Vec<u8>> {
    match hex::decode(key) {
        Ok(res) => Ok(res),
        Err(err) => Err(err_strs("hex decode", err)),
    }
}

fn generate() -> (Vec<u8>, Vec<u8>) {
    let ctx = SigCtx::new();
    let (pk, sk) = ctx.new_keypair();
    (ctx.serialize_seckey(&sk), ctx.serialize_pubkey(&pk, true))
}

fn generate_hex() -> (String, String) {
    let (sk, pk) = generate();
    (key_to_string(sk), key_to_string(pk))
}

fn generate_pk_from_sk(sk: Vec<u8>) -> GeorgeResult<Vec<u8>> {
    let ctx = SigCtx::new();
    match ctx.load_seckey(sk.as_slice()) {
        Ok(p) => Ok(ctx.serialize_pubkey(&ctx.pk_from_sk(&p), true)),
        Err(()) => Err(err_str("unknown")),
    }
}

fn generate_pk_from_sk_str(sk: String) -> GeorgeResult<Vec<u8>> {
    match hex::decode(sk) {
        Ok(sk_bytes) => generate_pk_from_sk(sk_bytes),
        Err(err) => Err(err_strs("hex decode", err)),
    }
}

fn generate_pk_from_sk_file<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Vec<u8>> {
    match read_to_string(sk_filepath) {
        Ok(sk) => generate_pk_from_sk_str(sk),
        Err(err) => Err(err_strs("read to string", err)),
    }
}

fn generate_in_file(sk_filepath: String, pk_filepath: String) -> GeorgeResult<(Vec<u8>, Vec<u8>)> {
    let (sk_bytes, pk_bytes) = generate();
    store_key(key_to_string(sk_bytes.clone()), sk_filepath)?;
    store_key(key_to_string(pk_bytes.clone()), pk_filepath)?;
    Ok((sk_bytes, pk_bytes))
}

fn generate_hex_in_file(
    sk_filepath: String,
    pk_filepath: String,
) -> GeorgeResult<(String, String)> {
    let (sk_str, pk_str) = generate_hex();
    store_key(sk_str.clone(), sk_filepath)?;
    store_key(pk_str.clone(), pk_filepath)?;
    Ok((sk_str, pk_str))
}

fn sign(msg: &[u8], sk: &[u8], pk: &[u8]) -> GeorgeResult<Vec<u8>> {
    let ctx = SigCtx::new();
    let pk_point: Point;
    let sig: Signature;
    match ctx.load_pubkey(pk) {
        Ok(pp) => pk_point = pp,
        Err(()) => return Err(err_str("load pub key error!")),
    }
    match ctx.load_seckey(sk) {
        Ok(sk_bu) => sig = ctx.sign(msg, &sk_bu, &pk_point),
        Err(()) => return Err(err_str("load pub key error!")),
    }
    Ok(sig.der_encode())
}

fn verify(msg: &[u8], pk: &[u8], der: &[u8]) -> GeorgeResult<bool> {
    let ctx = SigCtx::new();
    let pk_point: Point;
    let sig: Signature;
    match ctx.load_pubkey(pk) {
        Ok(pp) => pk_point = pp,
        Err(()) => return Err(err_str("load pub key error!")),
    }
    match Signature::der_decode(der) {
        Ok(s) => sig = s,
        Err(err) => return Err(err_strs("der decode", err)),
    }
    Ok(ctx.verify(msg, &pk_point, &sig))
}
