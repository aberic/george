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
use std::fs::read_to_string;
use std::path::Path;

use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private, Public};
use openssl::rsa::{Padding, Rsa};
use openssl::sign::{Signer, Verifier};
use openssl::symm::Cipher;

use crate::cryptos::base64::{Base64Decoder, Base64Encoder};
use crate::cryptos::hex::{HexDecoder, HexEncoder};
use crate::cryptos::Hex;
use crate::cryptos::{Base64, RSA};
use crate::errors::Errs;
use crate::errors::GeorgeResult;
use crate::io::file::FilerWriter;
use crate::io::Filer;
use crate::strings::StringHandler;
use crate::Strings;

pub trait RSANew {
    /// 生成非对称加密私钥，返回sk字节数组
    ///
    /// bits 私钥位数
    ///
    /// Serializes the private key to a PEM-encoded PKCS#1 RSAPrivateKey structure.
    ///
    /// The output will have a header of `-----BEGIN RSA PRIVATE KEY-----`.
    ///
    /// This corresponds to [`PEM_write_bio_RSAPrivateKey`].
    ///
    /// [`PEM_write_bio_RSAPrivateKey`]: https://www.openssl.org/docs/man1.1.0/crypto/PEM_write_bio_RSAPrivateKey.html
    /// <p>
    ///
    /// # Return
    /// bytes，可以通过string(bytes)的方式查阅
    fn generate_pkcs1_pem(bits: u32) -> GeorgeResult<Vec<u8>>;

    /// 生成非对称加密私钥，返回sk字节数组
    ///
    /// bits 私钥位数
    ///
    /// Serializes the private key to a PEM-encoded PKCS#8 PrivateKeyInfo structure.
    ///
    /// The output will have a header of `-----BEGIN PRIVATE KEY-----`.
    ///
    /// This corresponds to [`PEM_write_bio_PKCS8PrivateKey`].
    ///
    /// [`PEM_write_bio_PKCS8PrivateKey`]: https://www.openssl.org/docs/man1.0.2/crypto/PEM_write_bio_PKCS8PrivateKey.html
    /// <p>
    ///
    /// # Return
    /// bytes，可以通过string(bytes)的方式查阅
    fn generate_pkcs8_pem(bits: u32) -> GeorgeResult<Vec<u8>>;

    /// 生成非对称加密私钥，返回sk字节数组
    ///
    /// bits 私钥位数
    ///
    /// Serializes the private key to a DER-encoded PKCS#1 RSAPrivateKey structure.
    ///
    /// This corresponds to [`i2d_RSAPrivateKey`].
    ///
    /// [`i2d_RSAPrivateKey`]: https://www.openssl.org/docs/man1.0.2/crypto/i2d_RSAPrivateKey.html
    /// <p>
    ///
    /// # Return
    /// bytes，可以通过string(bytes)的方式查阅
    fn generate_pkcs1_der(bits: u32) -> GeorgeResult<Vec<u8>>;

    /// 生成非对称加密私钥，返回sk字节数组
    ///
    /// bits 私钥位数
    ///
    /// Serializes the private key to a DER-encoded key type specific format.
    ///
    /// This corresponds to [`i2d_PrivateKey`].
    ///
    /// [`i2d_PrivateKey`]: https://www.openssl.org/docs/man1.0.2/crypto/i2d_PrivateKey.html
    /// <p>
    ///
    /// # Return
    /// bytes，可以通过string(bytes)的方式查阅
    fn generate_pkcs8_der(bits: u32) -> GeorgeResult<Vec<u8>>;

    /// 生成非对称加密私钥，返回sk字符串
    ///
    /// bits 私钥位数
    ///
    /// Serializes the private key to a PEM-encoded PKCS#1 RSAPrivateKey structure.
    ///
    /// The output will have a header of `-----BEGIN RSA PRIVATE KEY-----`.
    ///
    /// This corresponds to [`PEM_write_bio_RSAPrivateKey`].
    ///
    /// [`PEM_write_bio_RSAPrivateKey`]: https://www.openssl.org/docs/man1.1.0/crypto/PEM_write_bio_RSAPrivateKey.html
    /// <p>
    ///
    /// # Return
    /// bytes，可以通过string(bytes)的方式查阅
    fn generate_pkcs1_pem_string(bits: u32) -> GeorgeResult<String>;

    /// 生成非对称加密私钥，返回sk字符串
    ///
    /// bits 私钥位数
    ///
    /// Serializes the private key to a PEM-encoded PKCS#8 PrivateKeyInfo structure.
    ///
    /// The output will have a header of `-----BEGIN PRIVATE KEY-----`.
    ///
    /// This corresponds to [`PEM_write_bio_PKCS8PrivateKey`].
    ///
    /// [`PEM_write_bio_PKCS8PrivateKey`]: https://www.openssl.org/docs/man1.0.2/crypto/PEM_write_bio_PKCS8PrivateKey.html
    /// <p>
    ///
    /// # Return
    /// bytes，可以通过string(bytes)的方式查阅
    fn generate_pkcs8_pem_string(bits: u32) -> GeorgeResult<String>;

    /// 生成非对称加密私钥，返回sk字符串
    ///
    /// bits 私钥位数
    ///
    /// Serializes the private key to a DER-encoded PKCS#1 RSAPrivateKey structure.
    ///
    /// This corresponds to [`i2d_RSAPrivateKey`].
    ///
    /// [`i2d_RSAPrivateKey`]: https://www.openssl.org/docs/man1.0.2/crypto/i2d_RSAPrivateKey.html
    /// <p>
    ///
    /// # Return
    /// bytes，可以通过string(bytes)的方式查阅
    fn generate_pkcs1_der_base64(bits: u32) -> GeorgeResult<String>;

    /// 生成非对称加密私钥，返回sk字符串
    ///
    /// bits 私钥位数
    ///
    /// Serializes the private key to a DER-encoded key type specific format.
    ///
    /// This corresponds to [`i2d_PrivateKey`].
    ///
    /// [`i2d_PrivateKey`]: https://www.openssl.org/docs/man1.0.2/crypto/i2d_PrivateKey.html
    /// <p>
    ///
    /// # Return
    /// bytes，可以通过string(bytes)的方式查阅
    fn generate_pkcs8_der_base64(bits: u32) -> GeorgeResult<String>;

    /// 生成非对称加密私钥，返回sk字符串
    ///
    /// bits 私钥位数
    ///
    /// Serializes the private key to a DER-encoded PKCS#1 RSAPrivateKey structure.
    ///
    /// This corresponds to [`i2d_RSAPrivateKey`].
    ///
    /// [`i2d_RSAPrivateKey`]: https://www.openssl.org/docs/man1.0.2/crypto/i2d_RSAPrivateKey.html
    /// <p>
    ///
    /// # Return
    /// bytes，可以通过string(bytes)的方式查阅
    fn generate_pkcs1_der_hex(bits: u32) -> GeorgeResult<String>;

    /// 生成非对称加密私钥，返回sk字符串
    ///
    /// bits 私钥位数
    ///
    /// Serializes the private key to a DER-encoded key type specific format.
    ///
    /// This corresponds to [`i2d_PrivateKey`].
    ///
    /// [`i2d_PrivateKey`]: https://www.openssl.org/docs/man1.0.2/crypto/i2d_PrivateKey.html
    /// <p>
    ///
    /// # Return
    /// bytes，可以通过string(bytes)的方式查阅
    fn generate_pkcs8_der_hex(bits: u32) -> GeorgeResult<String>;
}

pub trait RSANewPass<T> {
    /// 生成非对称加密私钥，返回sk字节数组
    ///
    /// bits 私钥位数
    ///
    /// Cipher Represents a particular cipher algorithm.
    ///
    /// See OpenSSL doc at [`EVP_EncryptInit`] for more information on each algorithms.
    ///
    /// [`EVP_EncryptInit`]: https://www.openssl.org/docs/man1.1.0/crypto/EVP_EncryptInit.html
    ///
    /// Serializes the private key to a PEM-encoded encrypted PKCS#1 RSAPrivateKey structure.
    ///
    /// The output will have a header of `-----BEGIN RSA PRIVATE KEY-----`.
    ///
    /// This corresponds to [`PEM_write_bio_RSAPrivateKey`].
    ///
    /// [`PEM_write_bio_RSAPrivateKey`]: https://www.openssl.org/docs/man1.1.0/crypto/PEM_write_bio_RSAPrivateKey.html
    /// <p>
    ///
    /// # Return
    /// bytes，可以通过string(bytes)的方式查阅
    fn generate_pkcs1_pem_pass(
        bits: u32,
        cipher: openssl::symm::Cipher,
        passphrase: T,
    ) -> GeorgeResult<Vec<u8>>;

    /// 生成非对称加密私钥，返回sk字节数组
    ///
    /// bits 私钥位数
    ///
    /// Cipher Represents a particular cipher algorithm.
    ///
    /// See OpenSSL doc at [`EVP_EncryptInit`] for more information on each algorithms.
    ///
    /// [`EVP_EncryptInit`]: https://www.openssl.org/docs/man1.1.0/crypto/EVP_EncryptInit.html
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
    fn generate_pkcs8_pem_pass(
        bits: u32,
        cipher: openssl::symm::Cipher,
        passphrase: T,
    ) -> GeorgeResult<Vec<u8>>;

    /// 生成非对称加密私钥，返回sk字符串
    ///
    /// bits 私钥位数
    ///
    /// Cipher Represents a particular cipher algorithm.
    ///
    /// See OpenSSL doc at [`EVP_EncryptInit`] for more information on each algorithms.
    ///
    /// [`EVP_EncryptInit`]: https://www.openssl.org/docs/man1.1.0/crypto/EVP_EncryptInit.html
    ///
    /// Serializes the private key to a PEM-encoded encrypted PKCS#1 RSAPrivateKey structure.
    ///
    /// The output will have a header of `-----BEGIN RSA PRIVATE KEY-----`.
    ///
    /// This corresponds to [`PEM_write_bio_RSAPrivateKey`].
    ///
    /// [`PEM_write_bio_RSAPrivateKey`]: https://www.openssl.org/docs/man1.1.0/crypto/PEM_write_bio_RSAPrivateKey.html
    /// <p>
    ///
    /// # Return
    /// bytes，可以通过string(bytes)的方式查阅
    fn generate_pkcs1_pem_pass_string(
        bits: u32,
        cipher: openssl::symm::Cipher,
        passphrase: T,
    ) -> GeorgeResult<String>;

    /// 生成非对称加密私钥，返回sk字符串
    ///
    /// bits 私钥位数
    ///
    /// Cipher Represents a particular cipher algorithm.
    ///
    /// See OpenSSL doc at [`EVP_EncryptInit`] for more information on each algorithms.
    ///
    /// [`EVP_EncryptInit`]: https://www.openssl.org/docs/man1.1.0/crypto/EVP_EncryptInit.html
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
    fn generate_pkcs8_pem_pass_string(
        bits: u32,
        cipher: openssl::symm::Cipher,
        passphrase: T,
    ) -> GeorgeResult<String>;
}

pub trait RSANewStore {
    /// 生成非对称加密私钥，返回sk字节数组
    ///
    /// 并将生成的私钥存储在sk指定文件中
    fn generate_pkcs1_pem<P: AsRef<Path>>(bits: u32, sk_filepath: P) -> GeorgeResult<Vec<u8>>;

    /// 生成非对称加密私钥，返回sk字节数组
    ///
    /// 并将生成的私钥存储在sk指定文件中
    fn generate_pkcs8_pem<P: AsRef<Path>>(bits: u32, sk_filepath: P) -> GeorgeResult<Vec<u8>>;

    /// 生成非对称加密私钥，返回sk字符串
    ///
    /// 并将生成的私钥存储在sk指定文件中
    fn generate_pkcs1_pem_string<P: AsRef<Path>>(bits: u32, sk_filepath: P)
        -> GeorgeResult<String>;

    /// 生成非对称加密私钥，返回sk字符串
    ///
    /// 并将生成的私钥存储在sk指定文件中
    fn generate_pkcs8_pem_string<P: AsRef<Path>>(bits: u32, sk_filepath: P)
        -> GeorgeResult<String>;

    /// 生成非对称加密私钥，返回sk字节数组
    ///
    /// 并将生成的私钥存储在sk指定文件中
    fn generate_pkcs1_der<P: AsRef<Path>>(bits: u32, sk_filepath: P) -> GeorgeResult<Vec<u8>>;

    /// 生成非对称加密私钥，返回sk字节数组
    ///
    /// 并将生成的私钥存储在sk指定文件中
    fn generate_pkcs8_der<P: AsRef<Path>>(bits: u32, sk_filepath: P) -> GeorgeResult<Vec<u8>>;

    /// 生成非对称加密私钥，返回sk字符串
    ///
    /// 并将生成的私钥存储在sk指定文件中
    fn generate_pkcs1_der_base64<P: AsRef<Path>>(bits: u32, sk_filepath: P)
        -> GeorgeResult<String>;

    /// 生成非对称加密私钥，返回sk字符串
    ///
    /// 并将生成的私钥存储在sk指定文件中
    fn generate_pkcs8_der_base64<P: AsRef<Path>>(bits: u32, sk_filepath: P)
        -> GeorgeResult<String>;

    /// 生成非对称加密私钥，返回sk字符串
    ///
    /// 并将生成的私钥存储在sk指定文件中
    fn generate_pkcs1_der_hex<P: AsRef<Path>>(bits: u32, sk_filepath: P) -> GeorgeResult<String>;

    /// 生成非对称加密私钥，返回sk字符串
    ///
    /// 并将生成的私钥存储在sk指定文件中
    fn generate_pkcs8_der_hex<P: AsRef<Path>>(bits: u32, sk_filepath: P) -> GeorgeResult<String>;
}

pub trait RSANewPassStore<M> {
    /// 生成非对称加密私钥，返回sk字节数组
    ///
    /// 并将生成的私钥存储在sk指定文件中
    fn generate_pkcs1_pem_pass<P: AsRef<Path>>(
        bits: u32,
        cipher: openssl::symm::Cipher,
        passphrase: M,
        sk_filepath: P,
    ) -> GeorgeResult<Vec<u8>>;

    /// 生成非对称加密私钥，返回sk字节数组
    ///
    /// 并将生成的私钥存储在sk指定文件中
    fn generate_pkcs8_pem_pass<P: AsRef<Path>>(
        bits: u32,
        cipher: openssl::symm::Cipher,
        passphrase: M,
        sk_filepath: P,
    ) -> GeorgeResult<Vec<u8>>;

    /// 生成非对称加密私钥，返回sk字符串
    ///
    /// 并将生成的私钥存储在sk指定文件中
    fn generate_pkcs1_pem_pass_string<P: AsRef<Path>>(
        bits: u32,
        cipher: openssl::symm::Cipher,
        passphrase: M,
        sk_filepath: P,
    ) -> GeorgeResult<String>;

    /// 生成非对称加密私钥，返回sk字符串
    ///
    /// 并将生成的私钥存储在sk指定文件中
    fn generate_pkcs8_pem_pass_string<P: AsRef<Path>>(
        bits: u32,
        cipher: openssl::symm::Cipher,
        passphrase: M,
        sk_filepath: P,
    ) -> GeorgeResult<String>;
}

pub trait RSAPkV8s<T> {
    /// 根据私钥生成公钥
    fn generate_pk(sk: T) -> GeorgeResult<Vec<u8>>;
}

pub trait RSAPk2String<T> {
    /// 根据私钥生成公钥
    fn generate_pk(sk: T) -> GeorgeResult<String>;
}

pub trait RSAPkKey<T> {
    /// 根据私钥生成公钥
    fn generate_pk(sk: T) -> GeorgeResult<PKey<Public>>;
}

pub trait RSAPk<T> {
    /// 根据私钥生成公钥
    fn generate_pk_pkcs1(sk: T) -> GeorgeResult<Rsa<Public>>;

    /// 根据私钥生成公钥
    fn generate_pk_pkcs8(sk: T) -> GeorgeResult<Rsa<Public>>;
}

pub trait RSAPkString<T> {
    /// 根据私钥生成公钥
    fn generate_pk_pkcs1_pem(sk: T) -> GeorgeResult<Rsa<Public>>;

    /// 根据私钥生成公钥
    fn generate_pk_pkcs8_pem(sk: T) -> GeorgeResult<Rsa<Public>>;

    /// 根据私钥生成公钥
    fn generate_pk_pkcs1_hex(sk: T) -> GeorgeResult<Rsa<Public>>;

    /// 根据私钥生成公钥
    fn generate_pk_pkcs8_hex(sk: T) -> GeorgeResult<Rsa<Public>>;

    /// 根据私钥生成公钥
    fn generate_pk_pkcs1_base64(sk: T) -> GeorgeResult<Rsa<Public>>;

    /// 根据私钥生成公钥
    fn generate_pk_pkcs8_base64(sk: T) -> GeorgeResult<Rsa<Public>>;
}

pub trait RSAPkString2String<T> {
    /// 根据私钥生成公钥
    fn generate_pk_pkcs1_pem(sk: T) -> GeorgeResult<String>;

    /// 根据私钥生成公钥
    fn generate_pk_pkcs8_pem(sk: T) -> GeorgeResult<String>;

    /// 根据私钥生成公钥
    fn generate_pk_pkcs1_hex(sk: T) -> GeorgeResult<String>;

    /// 根据私钥生成公钥
    fn generate_pk_pkcs8_hex(sk: T) -> GeorgeResult<String>;

    /// 根据私钥生成公钥
    fn generate_pk_pkcs1_base64(sk: T) -> GeorgeResult<String>;

    /// 根据私钥生成公钥
    fn generate_pk_pkcs8_base64(sk: T) -> GeorgeResult<String>;
}

pub trait RSAPkKeyString2String<T> {
    /// 根据私钥生成公钥
    fn generate_pk_pkey_pem(sk: T) -> GeorgeResult<String>;

    /// 根据私钥生成公钥
    fn generate_pk_pkey_hex(sk: T) -> GeorgeResult<String>;

    /// 根据私钥生成公钥
    fn generate_pk_pkey_base64(sk: T) -> GeorgeResult<String>;
}

pub trait RSAPkKeyString<T> {
    /// 根据私钥生成公钥
    fn generate_pk_pkey_pem(sk: T) -> GeorgeResult<PKey<Public>>;

    /// 根据私钥生成公钥
    fn generate_pk_pkey_hex(sk: T) -> GeorgeResult<PKey<Public>>;

    /// 根据私钥生成公钥
    fn generate_pk_pkey_base64(sk: T) -> GeorgeResult<PKey<Public>>;
}

pub trait RSAPkKeyPath {
    /// 根据私钥文件生成公钥
    fn generate_pk<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<PKey<Public>>;
}

pub trait RSAPkPath {
    /// 根据私钥文件生成公钥
    fn generate_pk_pkcs1<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Rsa<Public>>;

    /// 根据私钥文件生成公钥
    fn generate_pk_pkcs8<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Rsa<Public>>;
}

pub trait RSAPkV8sPath {
    /// 根据私钥文件生成公钥
    fn generate_pk_pkcs1_pem<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Vec<u8>>;

    /// 根据私钥文件生成公钥
    fn generate_pk_pkcs8_pem<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Vec<u8>>;

    /// 根据私钥文件生成公钥
    fn generate_pk_pkcs1_der<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Vec<u8>>;

    /// 根据私钥文件生成公钥
    fn generate_pk_pkcs8_der<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Vec<u8>>;
}

pub trait RSAPkStringPath {
    /// 根据私钥文件生成公钥
    fn generate_pk_pkcs1_pem<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<String>;

    /// 根据私钥文件生成公钥
    fn generate_pk_pkcs8_pem<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<String>;

    /// 根据私钥文件生成公钥
    fn generate_pk_pkcs1_der_hex<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<String>;

    /// 根据私钥文件生成公钥
    fn generate_pk_pkcs8_der_hex<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<String>;

    /// 根据私钥文件生成公钥
    fn generate_pk_pkcs1_der_base64<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<String>;

    /// 根据私钥文件生成公钥
    fn generate_pk_pkcs8_der_base64<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<String>;
}

pub trait RSAStoreKey<M> {
    /// 将公/私钥存储在指定文件中
    fn store<P: AsRef<Path>>(key: M, key_filepath: P) -> GeorgeResult<()>;
}

pub trait RSALoadKey {
    /// 从指定文件中读取公/私钥字节数组
    fn load_bytes<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<Vec<u8>>;

    /// 从指定文件中读取公/私钥字符串
    fn load_string<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<String>;

    /// 从指定文件中读取Pkey私钥
    fn load_sk<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<PKey<Private>>;

    /// 从指定文件中读取Pkey公钥
    fn load_pk<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<PKey<Public>>;

    /// 从指定文件中读取Rsa私钥
    fn load_rsa_sk<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<Rsa<Private>>;

    /// 从指定文件中读取Rsa公钥
    fn load_rsa_pk<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<Rsa<Public>>;
}

/// base method
impl RSA {
    /// 生成RSA对象
    pub fn new(bits: u32) -> GeorgeResult<RSA> {
        let rsa_sk = generate(bits)?;
        let rsa_pk = generate_pk_rsa_pkcs1_from_rsa_sk(rsa_sk.clone())?;
        let sk = generate_pkey(rsa_sk.clone())?;
        let pk = generate_pkey(rsa_pk.clone())?;
        Ok(RSA {
            sk,
            pk,
            rsa_sk,
            rsa_pk,
        })
    }

    /// 生成RSA对象
    pub fn from(rsa_sk: Rsa<Private>) -> GeorgeResult<RSA> {
        let rsa_pk = generate_pk_rsa_pkcs1_from_rsa_sk(rsa_sk.clone())?;
        let sk = generate_pkey(rsa_sk.clone())?;
        let pk = generate_pkey(rsa_pk.clone())?;
        Ok(RSA {
            sk,
            pk,
            rsa_sk,
            rsa_pk,
        })
    }

    /// 生成RSA对象
    pub fn from_pkey(sk: PKey<Private>) -> GeorgeResult<RSA> {
        let pk = generate_pk_pkey_from_pkey_sk(sk.clone())?;
        let rsa_sk = generate_rsa(sk.clone())?;
        let rsa_pk = generate_pk_rsa_pkcs1_from_rsa_sk(rsa_sk.clone())?;
        Ok(RSA {
            sk,
            pk,
            rsa_sk,
            rsa_pk,
        })
    }

    /// 生成RSA对象
    pub fn from_bytes(sk: Vec<u8>) -> GeorgeResult<RSA> {
        RSA::from_pkey(load_sk_pkey(sk)?)
    }

    /// 通过私钥文件生成RSA对象
    pub fn load<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<RSA> {
        let sk = load_sk_pkey_file(sk_filepath)?;
        RSA::from_pkey(sk)
    }

    /// 通过公私钥文件生成RSA对象
    pub fn load_all<P: AsRef<Path>>(sk_filepath: P, pk_filepath: P) -> GeorgeResult<RSA> {
        let sk = load_sk_pkey_file(sk_filepath)?;
        let pk = load_pk_pkey_file(pk_filepath)?;
        if !sk.public_eq(&pk) {
            Err(Errs::str("sk public_eq false"))
        } else {
            let rsa_sk = generate_rsa(sk.clone())?;
            let rsa_pk = generate_rsa(pk.clone())?;
            Ok(RSA {
                sk,
                pk,
                rsa_sk,
                rsa_pk,
            })
        }
    }

    // /// 通过公私钥文件生成RSA对象
    // pub fn store_pkcs1_pem<P: AsRef<Path>>(
    //     &self,
    //     sk_filepath: P,
    //     pk_filepath: P,
    // ) -> GeorgeResult<RSA> {
    //     let _ = Filer::write_force(sk_filepath, self.sk_pkcs1_pem()?)?;
    //     RSA::store(self.sk_pkcs1_pem()?, sk_filepath)?;
    //     RSA::store(self.pk_pkcs1_pem()?, pk_filepath)
    // }

    pub fn sk(&self) -> PKey<Private> {
        self.sk.clone()
    }

    pub fn pk(&self) -> PKey<Public> {
        self.pk.clone()
    }

    pub fn rsa_sk(&self) -> Rsa<Private> {
        self.rsa_sk.clone()
    }

    pub fn rsa_pk(&self) -> Rsa<Public> {
        self.rsa_pk.clone()
    }
}

/// pem method
impl RSA {
    pub fn sk_pkcs1_pem(&self) -> GeorgeResult<Vec<u8>> {
        match self.rsa_sk.private_key_to_pem() {
            Ok(res) => Ok(res),
            Err(err) => Err(Errs::strs("private_key_to_pem", err)),
        }
    }

    pub fn pk_pkcs1_pem(&self) -> GeorgeResult<Vec<u8>> {
        match self.rsa_pk.public_key_to_pem_pkcs1() {
            Ok(res) => Ok(res),
            Err(err) => Err(Errs::strs("private_key_to_pem", err)),
        }
    }

    pub fn sk_pkcs8_pem(&self) -> GeorgeResult<Vec<u8>> {
        match self.sk.private_key_to_pem_pkcs8() {
            Ok(res) => Ok(res),
            Err(err) => Err(Errs::strs("private_key_to_pem", err)),
        }
    }

    pub fn pk_pkcs8_pem(&self) -> GeorgeResult<Vec<u8>> {
        match self.pk.public_key_to_pem() {
            Ok(res) => Ok(res),
            Err(err) => Err(Errs::strs("private_key_to_pem", err)),
        }
    }

    pub fn sk_pkcs1_pem_str(&self) -> GeorgeResult<String> {
        Strings::from_utf8(self.sk_pkcs1_pem()?)
    }

    pub fn pk_pkcs1_pem_str(&self) -> GeorgeResult<String> {
        Strings::from_utf8(self.pk_pkcs1_pem()?)
    }

    pub fn sk_pkcs8_pem_str(&self) -> GeorgeResult<String> {
        Strings::from_utf8(self.sk_pkcs8_pem()?)
    }

    pub fn pk_pkcs8_pem_str(&self) -> GeorgeResult<String> {
        Strings::from_utf8(self.pk_pkcs8_pem()?)
    }

    pub fn sk_pkcs1_pem_hex(&self) -> GeorgeResult<String> {
        Ok(Hex::encode(self.sk_pkcs1_pem()?))
    }

    pub fn pk_pkcs1_pem_hex(&self) -> GeorgeResult<String> {
        Ok(Hex::encode(self.pk_pkcs1_pem()?))
    }

    pub fn sk_pkcs8_pem_hex(&self) -> GeorgeResult<String> {
        Ok(Hex::encode(self.sk_pkcs8_pem()?))
    }

    pub fn pk_pkcs8_pem_hex(&self) -> GeorgeResult<String> {
        Ok(Hex::encode(self.pk_pkcs8_pem()?))
    }

    pub fn sk_pkcs1_pem_base64(&self) -> GeorgeResult<String> {
        Ok(Base64::encode(self.sk_pkcs1_pem()?))
    }

    pub fn pk_pkcs1_pem_base64(&self) -> GeorgeResult<String> {
        Ok(Base64::encode(self.pk_pkcs1_pem()?))
    }

    pub fn sk_pkcs8_pem_base64(&self) -> GeorgeResult<String> {
        Ok(Base64::encode(self.sk_pkcs8_pem()?))
    }

    pub fn pk_pkcs8_pem_base64(&self) -> GeorgeResult<String> {
        Ok(Base64::encode(self.pk_pkcs8_pem()?))
    }
}

/// der method
impl RSA {
    pub fn sk_pkcs1_der(&self) -> GeorgeResult<Vec<u8>> {
        match self.rsa_sk.private_key_to_der() {
            Ok(res) => Ok(res),
            Err(err) => Err(Errs::strs("private_key_to_pem", err)),
        }
    }

    pub fn pk_pkcs1_der(&self) -> GeorgeResult<Vec<u8>> {
        match self.rsa_pk.public_key_to_der() {
            Ok(res) => Ok(res),
            Err(err) => Err(Errs::strs("private_key_to_pem", err)),
        }
    }

    pub fn sk_pkcs8_der(&self) -> GeorgeResult<Vec<u8>> {
        match self.sk.private_key_to_der() {
            Ok(res) => Ok(res),
            Err(err) => Err(Errs::strs("private_key_to_pem", err)),
        }
    }

    pub fn pk_pkcs8_der(&self) -> GeorgeResult<Vec<u8>> {
        match self.pk.public_key_to_der() {
            Ok(res) => Ok(res),
            Err(err) => Err(Errs::strs("private_key_to_pem", err)),
        }
    }

    pub fn sk_pkcs1_der_hex(&self) -> GeorgeResult<String> {
        Ok(Hex::encode(self.sk_pkcs1_der()?))
    }

    pub fn pk_pkcs1_der_hex(&self) -> GeorgeResult<String> {
        Ok(Hex::encode(self.pk_pkcs1_der()?))
    }

    pub fn sk_pkcs8_der_hex(&self) -> GeorgeResult<String> {
        Ok(Hex::encode(self.sk_pkcs8_der()?))
    }

    pub fn pk_pkcs8_der_hex(&self) -> GeorgeResult<String> {
        Ok(Hex::encode(self.pk_pkcs8_der()?))
    }

    pub fn sk_pkcs1_der_base64(&self) -> GeorgeResult<String> {
        Ok(Base64::encode(self.sk_pkcs1_der()?))
    }

    pub fn pk_pkcs1_der_base64(&self) -> GeorgeResult<String> {
        Ok(Base64::encode(self.pk_pkcs1_der()?))
    }

    pub fn sk_pkcs8_der_base64(&self) -> GeorgeResult<String> {
        Ok(Base64::encode(self.sk_pkcs8_der()?))
    }

    pub fn pk_pkcs8_der_base64(&self) -> GeorgeResult<String> {
        Ok(Base64::encode(self.pk_pkcs8_der()?))
    }
}

/// sign method
impl RSA {
    pub fn sign(&self, msg: &[u8]) -> GeorgeResult<Vec<u8>> {
        let mut signer: Signer;
        match Signer::new(MessageDigest::sha256(), &self.sk) {
            Ok(sig) => signer = sig,
            Err(err) => return Err(Errs::strs("signer new", err)),
        }
        match signer.set_rsa_padding(Padding::PKCS1) {
            Err(err) => return Err(Errs::strs("signer set_rsa_padding", err)),
            _ => {}
        }
        match signer.update(msg) {
            Err(err) => return Err(Errs::strs("signer update", err)),
            _ => {}
        }
        match signer.sign_to_vec() {
            Ok(res) => Ok(res),
            Err(err) => Err(Errs::strs("signer sign_to_vec", err)),
        }
    }

    pub fn sign_cus(
        &self,
        msg: &[u8],
        digest: MessageDigest,
        padding: Padding,
    ) -> GeorgeResult<Vec<u8>> {
        let mut signer: Signer;
        match Signer::new(digest, &self.sk) {
            Ok(sig) => signer = sig,
            Err(err) => return Err(Errs::strs("signer new", err)),
        }
        match signer.set_rsa_padding(padding) {
            Err(err) => return Err(Errs::strs("signer set_rsa_padding", err)),
            _ => {}
        }
        match signer.update(msg) {
            Err(err) => return Err(Errs::strs("signer update", err)),
            _ => {}
        }
        match signer.sign_to_vec() {
            Ok(res) => Ok(res),
            Err(err) => Err(Errs::strs("signer sign_to_vec", err)),
        }
    }

    pub fn verify(&self, msg: &[u8], der: &[u8]) -> GeorgeResult<bool> {
        let mut verifier: Verifier;
        match Verifier::new(MessageDigest::sha256(), &self.pk) {
            Ok(ver) => verifier = ver,
            Err(err) => return Err(Errs::strs("verifier new", err)),
        }
        match verifier.update(msg) {
            Err(err) => return Err(Errs::strs("verifier update", err)),
            _ => {}
        }
        match verifier.verify(der) {
            Ok(res) => Ok(res),
            Err(err) => Err(Errs::strs("verifier verify", err)),
        }
    }

    pub fn verify_cus(
        &self,
        msg: &[u8],
        der: &[u8],
        digest: MessageDigest,
        padding: Padding,
    ) -> GeorgeResult<bool> {
        let mut verifier: Verifier;
        match Verifier::new(digest, &self.pk) {
            Ok(ver) => verifier = ver,
            Err(err) => return Err(Errs::strs("verifier update", err)),
        }
        match verifier.set_rsa_padding(padding) {
            Err(err) => return Err(Errs::strs("verifier set_rsa_padding", err)),
            _ => {}
        }
        match verifier.update(msg) {
            Err(err) => return Err(Errs::strs("verifier update", err)),
            _ => {}
        }
        match verifier.verify(der) {
            Ok(res) => Ok(res),
            Err(err) => Err(Errs::strs("verifier verify", err)),
        }
    }
}

/// crypt method
impl RSA {
    pub fn encrypt_sk(&self, data: &[u8]) -> GeorgeResult<Vec<u8>> {
        let mut emesg = vec![0; self.rsa_sk.size() as usize];
        match self
            .rsa_sk
            .private_encrypt(data, &mut emesg, Padding::PKCS1)
        {
            Ok(_) => Ok(emesg),
            Err(err) => Err(Errs::strs("private_encrypt", err)),
        }
    }

    pub fn decrypt_sk(&self, data: &[u8]) -> GeorgeResult<Vec<u8>> {
        let mut emesg = vec![0; self.rsa_sk.size() as usize];
        match self
            .rsa_sk
            .private_decrypt(data, &mut emesg, Padding::PKCS1)
        {
            Ok(_) => Ok(emesg),
            Err(err) => Err(Errs::strs("private_decrypt", err)),
        }
    }

    pub fn encrypt_pk(&self, data: &[u8]) -> GeorgeResult<Vec<u8>> {
        let mut emesg = vec![0; self.rsa_pk.size() as usize];
        match self.rsa_pk.public_encrypt(data, &mut emesg, Padding::PKCS1) {
            Ok(_) => Ok(emesg),
            Err(err) => Err(Errs::strs("public_encrypt", err)),
        }
    }

    pub fn decrypt_pk(&self, data: &[u8]) -> GeorgeResult<Vec<u8>> {
        let mut emesg = vec![0; self.rsa_pk.size() as usize];
        match self.rsa_pk.public_decrypt(data, &mut emesg, Padding::PKCS1) {
            Ok(_) => Ok(emesg),
            Err(err) => Err(Errs::strs("public_decrypt", err)),
        }
    }

    pub fn encrypt_sk_padding(&self, data: &[u8], padding: Padding) -> GeorgeResult<Vec<u8>> {
        let mut emesg = vec![0; self.rsa_sk.size() as usize];
        match self.rsa_sk.private_encrypt(data, &mut emesg, padding) {
            Ok(_) => Ok(emesg),
            Err(err) => Err(Errs::strs("private_encrypt", err)),
        }
    }

    pub fn decrypt_sk_padding(&self, data: &[u8], padding: Padding) -> GeorgeResult<Vec<u8>> {
        let mut emesg = vec![0; self.rsa_sk.size() as usize];
        match self.rsa_sk.private_decrypt(data, &mut emesg, padding) {
            Ok(_) => Ok(emesg),
            Err(err) => Err(Errs::strs("private_decrypt", err)),
        }
    }

    pub fn encrypt_pk_padding(&self, data: &[u8], padding: Padding) -> GeorgeResult<Vec<u8>> {
        let mut emesg = vec![0; self.rsa_pk.size() as usize];
        match self.rsa_pk.public_encrypt(data, &mut emesg, padding) {
            Ok(_) => Ok(emesg),
            Err(err) => Err(Errs::strs("public_encrypt", err)),
        }
    }

    pub fn decrypt_pk_padding(&self, data: &[u8], padding: Padding) -> GeorgeResult<Vec<u8>> {
        let mut emesg = vec![0; self.rsa_pk.size() as usize];
        match self.rsa_pk.public_decrypt(data, &mut emesg, padding) {
            Ok(_) => Ok(emesg),
            Err(err) => Err(Errs::strs("public_decrypt", err)),
        }
    }
}

////////// generate pk start //////////

impl RSANew for RSA {
    fn generate_pkcs1_pem(bits: u32) -> GeorgeResult<Vec<u8>> {
        generate_pkcs1_sk_pem(bits)
    }

    fn generate_pkcs8_pem(bits: u32) -> GeorgeResult<Vec<u8>> {
        generate_pkcs8_sk_pem(bits)
    }

    fn generate_pkcs1_der(bits: u32) -> GeorgeResult<Vec<u8>> {
        generate_pkcs1_sk_der(bits)
    }

    fn generate_pkcs8_der(bits: u32) -> GeorgeResult<Vec<u8>> {
        generate_pkcs8_sk_der(bits)
    }

    fn generate_pkcs1_pem_string(bits: u32) -> GeorgeResult<String> {
        generate_pkcs1_sk_pem_string(bits)
    }

    fn generate_pkcs8_pem_string(bits: u32) -> GeorgeResult<String> {
        generate_pkcs8_sk_pem_string(bits)
    }

    fn generate_pkcs1_der_base64(bits: u32) -> GeorgeResult<String> {
        generate_pkcs1_sk_der_base64_string(bits)
    }

    fn generate_pkcs8_der_base64(bits: u32) -> GeorgeResult<String> {
        generate_pkcs8_sk_der_base64_string(bits)
    }

    fn generate_pkcs1_der_hex(bits: u32) -> GeorgeResult<String> {
        generate_pkcs1_sk_der_hex_string(bits)
    }

    fn generate_pkcs8_der_hex(bits: u32) -> GeorgeResult<String> {
        generate_pkcs8_sk_der_hex_string(bits)
    }
}

impl RSANewPass<&[u8]> for RSA {
    fn generate_pkcs1_pem_pass(
        bits: u32,
        cipher: Cipher,
        passphrase: &[u8],
    ) -> GeorgeResult<Vec<u8>> {
        generate_pkcs1_sk_pem_pass(bits, cipher, passphrase)
    }

    fn generate_pkcs8_pem_pass(
        bits: u32,
        cipher: Cipher,
        passphrase: &[u8],
    ) -> GeorgeResult<Vec<u8>> {
        generate_pkcs8_sk_pem_pass(bits, cipher, passphrase)
    }

    fn generate_pkcs1_pem_pass_string(
        bits: u32,
        cipher: Cipher,
        passphrase: &[u8],
    ) -> GeorgeResult<String> {
        generate_pkcs1_sk_pem_pass_string(bits, cipher, passphrase)
    }

    fn generate_pkcs8_pem_pass_string(
        bits: u32,
        cipher: Cipher,
        passphrase: &[u8],
    ) -> GeorgeResult<String> {
        generate_pkcs8_sk_pem_pass_string(bits, cipher, passphrase)
    }
}

impl RSANewPass<Vec<u8>> for RSA {
    fn generate_pkcs1_pem_pass(
        bits: u32,
        cipher: Cipher,
        passphrase: Vec<u8>,
    ) -> GeorgeResult<Vec<u8>> {
        generate_pkcs1_sk_pem_pass(bits, cipher, passphrase.as_slice())
    }

    fn generate_pkcs8_pem_pass(
        bits: u32,
        cipher: Cipher,
        passphrase: Vec<u8>,
    ) -> GeorgeResult<Vec<u8>> {
        generate_pkcs8_sk_pem_pass(bits, cipher, passphrase.as_slice())
    }

    fn generate_pkcs1_pem_pass_string(
        bits: u32,
        cipher: Cipher,
        passphrase: Vec<u8>,
    ) -> GeorgeResult<String> {
        generate_pkcs1_sk_pem_pass_string(bits, cipher, passphrase.as_slice())
    }

    fn generate_pkcs8_pem_pass_string(
        bits: u32,
        cipher: Cipher,
        passphrase: Vec<u8>,
    ) -> GeorgeResult<String> {
        generate_pkcs8_sk_pem_pass_string(bits, cipher, passphrase.as_slice())
    }
}

impl RSANewPass<&str> for RSA {
    fn generate_pkcs1_pem_pass(
        bits: u32,
        cipher: Cipher,
        passphrase: &str,
    ) -> GeorgeResult<Vec<u8>> {
        generate_pkcs1_sk_pem_pass(bits, cipher, passphrase.as_bytes())
    }

    fn generate_pkcs8_pem_pass(
        bits: u32,
        cipher: Cipher,
        passphrase: &str,
    ) -> GeorgeResult<Vec<u8>> {
        generate_pkcs8_sk_pem_pass(bits, cipher, passphrase.as_bytes())
    }

    fn generate_pkcs1_pem_pass_string(
        bits: u32,
        cipher: Cipher,
        passphrase: &str,
    ) -> GeorgeResult<String> {
        generate_pkcs1_sk_pem_pass_string(bits, cipher, passphrase.as_bytes())
    }

    fn generate_pkcs8_pem_pass_string(
        bits: u32,
        cipher: Cipher,
        passphrase: &str,
    ) -> GeorgeResult<String> {
        generate_pkcs8_sk_pem_pass_string(bits, cipher, passphrase.as_bytes())
    }
}

impl RSANewPass<String> for RSA {
    fn generate_pkcs1_pem_pass(
        bits: u32,
        cipher: Cipher,
        passphrase: String,
    ) -> GeorgeResult<Vec<u8>> {
        generate_pkcs1_sk_pem_pass(bits, cipher, passphrase.as_bytes())
    }

    fn generate_pkcs8_pem_pass(
        bits: u32,
        cipher: Cipher,
        passphrase: String,
    ) -> GeorgeResult<Vec<u8>> {
        generate_pkcs8_sk_pem_pass(bits, cipher, passphrase.as_bytes())
    }

    fn generate_pkcs1_pem_pass_string(
        bits: u32,
        cipher: Cipher,
        passphrase: String,
    ) -> GeorgeResult<String> {
        generate_pkcs1_sk_pem_pass_string(bits, cipher, passphrase.as_bytes())
    }

    fn generate_pkcs8_pem_pass_string(
        bits: u32,
        cipher: Cipher,
        passphrase: String,
    ) -> GeorgeResult<String> {
        generate_pkcs8_sk_pem_pass_string(bits, cipher, passphrase.as_bytes())
    }
}

impl RSANewStore for RSA {
    fn generate_pkcs1_pem<P: AsRef<Path>>(bits: u32, sk_filepath: P) -> GeorgeResult<Vec<u8>> {
        generate_pkcs1_sk_pem_file(bits, sk_filepath)
    }

    fn generate_pkcs8_pem<P: AsRef<Path>>(bits: u32, sk_filepath: P) -> GeorgeResult<Vec<u8>> {
        generate_pkcs8_sk_pem_file(bits, sk_filepath)
    }

    fn generate_pkcs1_pem_string<P: AsRef<Path>>(
        bits: u32,
        sk_filepath: P,
    ) -> GeorgeResult<String> {
        generate_pkcs1_sk_pem_file_string(bits, sk_filepath)
    }

    fn generate_pkcs8_pem_string<P: AsRef<Path>>(
        bits: u32,
        sk_filepath: P,
    ) -> GeorgeResult<String> {
        generate_pkcs8_sk_pem_file_string(bits, sk_filepath)
    }

    fn generate_pkcs1_der<P: AsRef<Path>>(bits: u32, sk_filepath: P) -> GeorgeResult<Vec<u8>> {
        generate_pkcs1_sk_der_file(bits, sk_filepath)
    }

    fn generate_pkcs8_der<P: AsRef<Path>>(bits: u32, sk_filepath: P) -> GeorgeResult<Vec<u8>> {
        generate_pkcs8_sk_der_file(bits, sk_filepath)
    }

    fn generate_pkcs1_der_base64<P: AsRef<Path>>(
        bits: u32,
        sk_filepath: P,
    ) -> GeorgeResult<String> {
        generate_pkcs1_sk_der_base64_file(bits, sk_filepath)
    }

    fn generate_pkcs8_der_base64<P: AsRef<Path>>(
        bits: u32,
        sk_filepath: P,
    ) -> GeorgeResult<String> {
        generate_pkcs8_sk_der_base64_file(bits, sk_filepath)
    }

    fn generate_pkcs1_der_hex<P: AsRef<Path>>(bits: u32, sk_filepath: P) -> GeorgeResult<String> {
        generate_pkcs1_sk_der_hex_file(bits, sk_filepath)
    }

    fn generate_pkcs8_der_hex<P: AsRef<Path>>(bits: u32, sk_filepath: P) -> GeorgeResult<String> {
        generate_pkcs8_sk_der_hex_file(bits, sk_filepath)
    }
}

impl RSANewPassStore<String> for RSA {
    fn generate_pkcs1_pem_pass<P: AsRef<Path>>(
        bits: u32,
        cipher: Cipher,
        passphrase: String,
        sk_filepath: P,
    ) -> GeorgeResult<Vec<u8>> {
        generate_pkcs1_sk_pem_pass_file(bits, cipher, passphrase.as_bytes(), sk_filepath)
    }

    fn generate_pkcs8_pem_pass<P: AsRef<Path>>(
        bits: u32,
        cipher: Cipher,
        passphrase: String,
        sk_filepath: P,
    ) -> GeorgeResult<Vec<u8>> {
        generate_pkcs8_sk_pem_pass_file(bits, cipher, passphrase.as_bytes(), sk_filepath)
    }

    fn generate_pkcs1_pem_pass_string<P: AsRef<Path>>(
        bits: u32,
        cipher: Cipher,
        passphrase: String,
        sk_filepath: P,
    ) -> GeorgeResult<String> {
        generate_pkcs1_sk_pem_pass_file_string(bits, cipher, passphrase.as_bytes(), sk_filepath)
    }

    fn generate_pkcs8_pem_pass_string<P: AsRef<Path>>(
        bits: u32,
        cipher: Cipher,
        passphrase: String,
        sk_filepath: P,
    ) -> GeorgeResult<String> {
        generate_pkcs8_sk_pem_pass_file_string(bits, cipher, passphrase.as_bytes(), sk_filepath)
    }
}

impl RSANewPassStore<&str> for RSA {
    fn generate_pkcs1_pem_pass<P: AsRef<Path>>(
        bits: u32,
        cipher: Cipher,
        passphrase: &str,
        sk_filepath: P,
    ) -> GeorgeResult<Vec<u8>> {
        generate_pkcs1_sk_pem_pass_file(bits, cipher, passphrase.as_bytes(), sk_filepath)
    }

    fn generate_pkcs8_pem_pass<P: AsRef<Path>>(
        bits: u32,
        cipher: Cipher,
        passphrase: &str,
        sk_filepath: P,
    ) -> GeorgeResult<Vec<u8>> {
        generate_pkcs8_sk_pem_pass_file(bits, cipher, passphrase.as_bytes(), sk_filepath)
    }

    fn generate_pkcs1_pem_pass_string<P: AsRef<Path>>(
        bits: u32,
        cipher: Cipher,
        passphrase: &str,
        sk_filepath: P,
    ) -> GeorgeResult<String> {
        generate_pkcs1_sk_pem_pass_file_string(bits, cipher, passphrase.as_bytes(), sk_filepath)
    }

    fn generate_pkcs8_pem_pass_string<P: AsRef<Path>>(
        bits: u32,
        cipher: Cipher,
        passphrase: &str,
        sk_filepath: P,
    ) -> GeorgeResult<String> {
        generate_pkcs8_sk_pem_pass_file_string(bits, cipher, passphrase.as_bytes(), sk_filepath)
    }
}

impl RSANewPassStore<Vec<u8>> for RSA {
    fn generate_pkcs1_pem_pass<P: AsRef<Path>>(
        bits: u32,
        cipher: Cipher,
        passphrase: Vec<u8>,
        sk_filepath: P,
    ) -> GeorgeResult<Vec<u8>> {
        generate_pkcs1_sk_pem_pass_file(bits, cipher, passphrase.as_slice(), sk_filepath)
    }

    fn generate_pkcs8_pem_pass<P: AsRef<Path>>(
        bits: u32,
        cipher: Cipher,
        passphrase: Vec<u8>,
        sk_filepath: P,
    ) -> GeorgeResult<Vec<u8>> {
        generate_pkcs8_sk_pem_pass_file(bits, cipher, passphrase.as_slice(), sk_filepath)
    }

    fn generate_pkcs1_pem_pass_string<P: AsRef<Path>>(
        bits: u32,
        cipher: Cipher,
        passphrase: Vec<u8>,
        sk_filepath: P,
    ) -> GeorgeResult<String> {
        generate_pkcs1_sk_pem_pass_file_string(bits, cipher, passphrase.as_slice(), sk_filepath)
    }

    fn generate_pkcs8_pem_pass_string<P: AsRef<Path>>(
        bits: u32,
        cipher: Cipher,
        passphrase: Vec<u8>,
        sk_filepath: P,
    ) -> GeorgeResult<String> {
        generate_pkcs8_sk_pem_pass_file_string(bits, cipher, passphrase.as_slice(), sk_filepath)
    }
}

impl RSANewPassStore<&[u8]> for RSA {
    fn generate_pkcs1_pem_pass<P: AsRef<Path>>(
        bits: u32,
        cipher: Cipher,
        passphrase: &[u8],
        sk_filepath: P,
    ) -> GeorgeResult<Vec<u8>> {
        generate_pkcs1_sk_pem_pass_file(bits, cipher, passphrase, sk_filepath)
    }

    fn generate_pkcs8_pem_pass<P: AsRef<Path>>(
        bits: u32,
        cipher: Cipher,
        passphrase: &[u8],
        sk_filepath: P,
    ) -> GeorgeResult<Vec<u8>> {
        generate_pkcs8_sk_pem_pass_file(bits, cipher, passphrase, sk_filepath)
    }

    fn generate_pkcs1_pem_pass_string<P: AsRef<Path>>(
        bits: u32,
        cipher: Cipher,
        passphrase: &[u8],
        sk_filepath: P,
    ) -> GeorgeResult<String> {
        generate_pkcs1_sk_pem_pass_file_string(bits, cipher, passphrase, sk_filepath)
    }

    fn generate_pkcs8_pem_pass_string<P: AsRef<Path>>(
        bits: u32,
        cipher: Cipher,
        passphrase: &[u8],
        sk_filepath: P,
    ) -> GeorgeResult<String> {
        generate_pkcs8_sk_pem_pass_file_string(bits, cipher, passphrase, sk_filepath)
    }
}

////////// generate sk end //////////

////////// generate pk start //////////

impl RSAPkKey<&[u8]> for RSA {
    fn generate_pk(sk: &[u8]) -> GeorgeResult<PKey<Public>> {
        generate_pk_pkey_from_pkey_sk_bytes(sk.to_vec())
    }
}

impl RSAPkKey<Vec<u8>> for RSA {
    fn generate_pk(sk: Vec<u8>) -> GeorgeResult<PKey<Public>> {
        generate_pk_pkey_from_pkey_sk_bytes(sk)
    }
}

impl RSAPkKey<PKey<Private>> for RSA {
    fn generate_pk(sk: PKey<Private>) -> GeorgeResult<PKey<Public>> {
        generate_pk_pkey_from_pkey_sk(sk)
    }
}

impl RSAPkKeyString<String> for RSA {
    fn generate_pk_pkey_pem(sk: String) -> GeorgeResult<PKey<Public>> {
        generate_pk_pkey_from_pkey_sk_bytes(sk.into_bytes())
    }

    fn generate_pk_pkey_hex(sk: String) -> GeorgeResult<PKey<Public>> {
        generate_pk_pkey_from_pkey_sk_bytes(Hex::decode(sk)?)
    }

    fn generate_pk_pkey_base64(sk: String) -> GeorgeResult<PKey<Public>> {
        generate_pk_pkey_from_pkey_sk_bytes(Base64::decode(sk)?)
    }
}

impl RSAPkKeyString<&str> for RSA {
    fn generate_pk_pkey_pem(sk: &str) -> GeorgeResult<PKey<Public>> {
        generate_pk_pkey_from_pkey_sk_bytes(sk.as_bytes().to_vec())
    }

    fn generate_pk_pkey_hex(sk: &str) -> GeorgeResult<PKey<Public>> {
        generate_pk_pkey_from_pkey_sk_bytes(Hex::decode(sk)?)
    }

    fn generate_pk_pkey_base64(sk: &str) -> GeorgeResult<PKey<Public>> {
        generate_pk_pkey_from_pkey_sk_bytes(Base64::decode(sk)?)
    }
}

impl RSAPk<Rsa<Private>> for RSA {
    fn generate_pk_pkcs1(sk: Rsa<Private>) -> GeorgeResult<Rsa<Public>> {
        generate_pk_rsa_pkcs1_from_rsa_sk(sk)
    }

    fn generate_pk_pkcs8(sk: Rsa<Private>) -> GeorgeResult<Rsa<Public>> {
        generate_pk_rsa_pkcs8_from_rsa_sk(sk)
    }
}

impl RSAPk<Vec<u8>> for RSA {
    fn generate_pk_pkcs1(sk: Vec<u8>) -> GeorgeResult<Rsa<Public>> {
        generate_pk_rsa_pkcs1_from_rsa_sk_bytes(sk)
    }

    fn generate_pk_pkcs8(sk: Vec<u8>) -> GeorgeResult<Rsa<Public>> {
        generate_pk_rsa_pkcs8_from_rsa_sk_bytes(sk)
    }
}

impl RSAPkString<String> for RSA {
    fn generate_pk_pkcs1_pem(sk: String) -> GeorgeResult<Rsa<Public>> {
        generate_pk_rsa_pkcs1_from_rsa_sk_bytes(sk.into_bytes())
    }

    fn generate_pk_pkcs8_pem(sk: String) -> GeorgeResult<Rsa<Public>> {
        generate_pk_rsa_pkcs8_from_rsa_sk_bytes(sk.into_bytes())
    }

    fn generate_pk_pkcs1_hex(sk: String) -> GeorgeResult<Rsa<Public>> {
        generate_pk_rsa_pkcs1_from_rsa_sk_bytes(Hex::decode(sk)?)
    }

    fn generate_pk_pkcs8_hex(sk: String) -> GeorgeResult<Rsa<Public>> {
        generate_pk_rsa_pkcs8_from_rsa_sk_bytes(Hex::decode(sk)?)
    }

    fn generate_pk_pkcs1_base64(sk: String) -> GeorgeResult<Rsa<Public>> {
        generate_pk_rsa_pkcs1_from_rsa_sk_bytes(Base64::decode(sk)?)
    }

    fn generate_pk_pkcs8_base64(sk: String) -> GeorgeResult<Rsa<Public>> {
        generate_pk_rsa_pkcs8_from_rsa_sk_bytes(Base64::decode(sk)?)
    }
}

impl RSAPkString<&str> for RSA {
    fn generate_pk_pkcs1_pem(sk: &str) -> GeorgeResult<Rsa<Public>> {
        generate_pk_rsa_pkcs1_from_rsa_sk_bytes(sk.as_bytes().to_vec())
    }

    fn generate_pk_pkcs8_pem(sk: &str) -> GeorgeResult<Rsa<Public>> {
        generate_pk_rsa_pkcs8_from_rsa_sk_bytes(sk.as_bytes().to_vec())
    }

    fn generate_pk_pkcs1_hex(sk: &str) -> GeorgeResult<Rsa<Public>> {
        generate_pk_rsa_pkcs1_from_rsa_sk_bytes(Hex::decode(sk)?)
    }

    fn generate_pk_pkcs8_hex(sk: &str) -> GeorgeResult<Rsa<Public>> {
        generate_pk_rsa_pkcs8_from_rsa_sk_bytes(Hex::decode(sk)?)
    }

    fn generate_pk_pkcs1_base64(sk: &str) -> GeorgeResult<Rsa<Public>> {
        generate_pk_rsa_pkcs1_from_rsa_sk_bytes(Base64::decode(sk)?)
    }

    fn generate_pk_pkcs8_base64(sk: &str) -> GeorgeResult<Rsa<Public>> {
        generate_pk_rsa_pkcs8_from_rsa_sk_bytes(Base64::decode(sk)?)
    }
}

impl RSAPkKeyPath for RSA {
    fn generate_pk<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<PKey<Public>> {
        generate_pk_pkey_from_pkey_sk_file(sk_filepath)
    }
}

impl RSAPkPath for RSA {
    fn generate_pk_pkcs1<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Rsa<Public>> {
        generate_pk_rsa_pkcs1_from_rsa_sk_file(sk_filepath)
    }

    fn generate_pk_pkcs8<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Rsa<Public>> {
        generate_pk_rsa_pkcs8_from_rsa_sk_file(sk_filepath)
    }
}

impl RSAPkV8sPath for RSA {
    fn generate_pk_pkcs1_pem<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Vec<u8>> {
        generate_pk_rsa_pkcs1_pem_from_sk_file(sk_filepath)
    }

    fn generate_pk_pkcs8_pem<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Vec<u8>> {
        generate_pk_rsa_pkcs8_pem_from_sk_file(sk_filepath)
    }

    fn generate_pk_pkcs1_der<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Vec<u8>> {
        generate_pk_rsa_pkcs1_der_from_sk_file(sk_filepath)
    }

    fn generate_pk_pkcs8_der<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Vec<u8>> {
        generate_pk_rsa_pkcs8_der_from_sk_file(sk_filepath)
    }
}

impl RSAPkStringPath for RSA {
    fn generate_pk_pkcs1_pem<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<String> {
        generate_pk_rsa_pkcs1_pem_string_from_sk_file(sk_filepath)
    }

    fn generate_pk_pkcs8_pem<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<String> {
        generate_pk_rsa_pkcs8_pem_string_from_sk_file(sk_filepath)
    }

    fn generate_pk_pkcs1_der_hex<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<String> {
        generate_pk_rsa_pkcs1_der_hex_from_sk_file(sk_filepath)
    }

    fn generate_pk_pkcs8_der_hex<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<String> {
        generate_pk_rsa_pkcs8_der_base64_from_sk_file(sk_filepath)
    }

    fn generate_pk_pkcs1_der_base64<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<String> {
        generate_pk_rsa_pkcs1_der_base64_from_sk_file(sk_filepath)
    }

    fn generate_pk_pkcs8_der_base64<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<String> {
        generate_pk_rsa_pkcs8_der_base64_from_sk_file(sk_filepath)
    }
}

////////// generate pk end //////////

////////// store end //////////

impl RSAStoreKey<String> for RSA {
    fn store<P: AsRef<Path>>(key: String, key_filepath: P) -> GeorgeResult<()> {
        let _ = Filer::write_force(key_filepath, key)?;
        Ok(())
    }
}

impl RSAStoreKey<&str> for RSA {
    fn store<P: AsRef<Path>>(key: &str, key_filepath: P) -> GeorgeResult<()> {
        let _ = Filer::write_force(key_filepath, key)?;
        Ok(())
    }
}

impl RSAStoreKey<Vec<u8>> for RSA {
    fn store<P: AsRef<Path>>(key: Vec<u8>, key_filepath: P) -> GeorgeResult<()> {
        let _ = Filer::write_force(key_filepath, key)?;
        Ok(())
    }
}

impl RSAStoreKey<&[u8]> for RSA {
    fn store<P: AsRef<Path>>(key: &[u8], key_filepath: P) -> GeorgeResult<()> {
        let _ = Filer::write_force(key_filepath, key)?;
        Ok(())
    }
}

////////// store end //////////

////////// load start //////////

impl RSALoadKey for RSA {
    fn load_bytes<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<Vec<u8>> {
        load_bytes_from_file(key_filepath)
    }

    fn load_string<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<String> {
        load_string_from_file(key_filepath)
    }

    fn load_sk<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<PKey<Private>> {
        load_sk_pkey_file(key_filepath)
    }

    fn load_pk<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<PKey<Public>> {
        load_pk_pkey_file(key_filepath)
    }

    fn load_rsa_sk<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<Rsa<Private>> {
        load_sk_file(key_filepath)
    }

    fn load_rsa_pk<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<Rsa<Public>> {
        load_pk_file(key_filepath)
    }
}

////////// load end //////////

fn generate(bits: u32) -> GeorgeResult<Rsa<Private>> {
    match Rsa::generate(bits) {
        Ok(rsa) => Ok(rsa),
        Err(err) => Err(Errs::strs("generate_pkcs1", err)),
    }
}

fn generate_rsa<T>(key: PKey<T>) -> GeorgeResult<Rsa<T>> {
    match key.rsa() {
        Ok(rsa) => Ok(rsa),
        Err(err) => Err(Errs::strs("generate_pkey", err)),
    }
}

fn generate_pkey<T>(rsa: Rsa<T>) -> GeorgeResult<PKey<T>> {
    match PKey::from_rsa(rsa) {
        Ok(rsa) => Ok(rsa),
        Err(err) => Err(Errs::strs("generate_pkey", err)),
    }
}

fn generate_pkcs1_sk_pem(bits: u32) -> GeorgeResult<Vec<u8>> {
    match Rsa::generate(bits) {
        Ok(rsa) => match rsa.private_key_to_pem() {
            Ok(res) => Ok(res),
            Err(err) => Err(Errs::strs("private_key_to_pem_pkcs1", err)),
        },
        Err(err) => Err(Errs::strs("generate", err)),
    }
}

fn generate_pkcs8_sk_pem(bits: u32) -> GeorgeResult<Vec<u8>> {
    match Rsa::generate(bits) {
        Ok(rsa) => match PKey::from_rsa(rsa) {
            Ok(key) => match key.private_key_to_pem_pkcs8() {
                Ok(res) => Ok(res),
                Err(err) => Err(Errs::strs("private_key_to_pem_pkcs8", err)),
            },
            Err(err) => Err(Errs::strs("from_rsa", err)),
        },
        Err(err) => Err(Errs::strs("generate", err)),
    }
}

fn generate_pkcs1_sk_pem_pass(
    bits: u32,
    cipher: Cipher,
    passphrase: &[u8],
) -> GeorgeResult<Vec<u8>> {
    match Rsa::generate(bits) {
        Ok(rsa) => match rsa.private_key_to_pem_passphrase(cipher, passphrase) {
            Ok(res) => Ok(res),
            Err(err) => Err(Errs::strs("private_key_to_pem_pkcs1", err)),
        },
        Err(err) => Err(Errs::strs("generate", err)),
    }
}

fn generate_pkcs8_sk_pem_pass(
    bits: u32,
    cipher: Cipher,
    passphrase: &[u8],
) -> GeorgeResult<Vec<u8>> {
    match Rsa::generate(bits) {
        Ok(rsa) => match PKey::from_rsa(rsa) {
            Ok(key) => match key.private_key_to_pem_pkcs8_passphrase(cipher, passphrase) {
                Ok(res) => Ok(res),
                Err(err) => Err(Errs::strs("private_key_to_pem_pkcs8", err)),
            },
            Err(err) => Err(Errs::strs("from_rsa", err)),
        },
        Err(err) => Err(Errs::strs("generate", err)),
    }
}

fn generate_pkcs1_sk_der(bits: u32) -> GeorgeResult<Vec<u8>> {
    match Rsa::generate(bits) {
        Ok(rsa) => match rsa.private_key_to_der() {
            Ok(res) => Ok(res),
            Err(err) => Err(Errs::strs("private_key_to_pem_pkcs1", err)),
        },
        Err(err) => Err(Errs::strs("generate", err)),
    }
}

fn generate_pkcs8_sk_der(bits: u32) -> GeorgeResult<Vec<u8>> {
    match Rsa::generate(bits) {
        Ok(rsa) => match PKey::from_rsa(rsa) {
            Ok(key) => match key.private_key_to_der() {
                Ok(res) => Ok(res),
                Err(err) => Err(Errs::strs("private_key_to_pem_pkcs8", err)),
            },
            Err(err) => Err(Errs::strs("from_rsa", err)),
        },
        Err(err) => Err(Errs::strs("generate", err)),
    }
}

/// 生成RSA私钥
///
/// bits 私钥位数
fn generate_pkcs1_sk_pem_string(bits: u32) -> GeorgeResult<String> {
    match generate_pkcs1_sk_pem(bits) {
        Ok(v8s) => Strings::from_utf8(v8s),
        Err(err) => Err(Errs::strs("generate_sk_pem", err)),
    }
}

fn generate_pkcs8_sk_pem_string(bits: u32) -> GeorgeResult<String> {
    match generate_pkcs8_sk_pem(bits) {
        Ok(v8s) => Strings::from_utf8(v8s),
        Err(err) => Err(Errs::strs("generate_sk_pem", err)),
    }
}

fn generate_pkcs1_sk_pem_pass_string(
    bits: u32,
    cipher: Cipher,
    passphrase: &[u8],
) -> GeorgeResult<String> {
    match generate_pkcs1_sk_pem_pass(bits, cipher, passphrase) {
        Ok(v8s) => Strings::from_utf8(v8s),
        Err(err) => Err(Errs::strs("generate_sk_pem", err)),
    }
}

fn generate_pkcs8_sk_pem_pass_string(
    bits: u32,
    cipher: Cipher,
    passphrase: &[u8],
) -> GeorgeResult<String> {
    match generate_pkcs8_sk_pem_pass(bits, cipher, passphrase) {
        Ok(v8s) => Strings::from_utf8(v8s),
        Err(err) => Err(Errs::strs("generate_sk_pem", err)),
    }
}

fn generate_pkcs1_sk_der_base64_string(bits: u32) -> GeorgeResult<String> {
    match generate_pkcs1_sk_der(bits) {
        Ok(v8s) => Ok(Base64::encode(v8s)),
        Err(err) => Err(Errs::strs("generate_sk_pem", err)),
    }
}

fn generate_pkcs8_sk_der_base64_string(bits: u32) -> GeorgeResult<String> {
    match generate_pkcs8_sk_der(bits) {
        Ok(v8s) => Ok(Base64::encode(v8s)),
        Err(err) => Err(Errs::strs("generate_sk_pem", err)),
    }
}

fn generate_pkcs1_sk_der_hex_string(bits: u32) -> GeorgeResult<String> {
    match generate_pkcs1_sk_der(bits) {
        Ok(v8s) => Ok(hex::encode(v8s)),
        Err(err) => Err(Errs::strs("generate_sk_pem", err)),
    }
}

fn generate_pkcs8_sk_der_hex_string(bits: u32) -> GeorgeResult<String> {
    match generate_pkcs8_sk_der(bits) {
        Ok(v8s) => Ok(hex::encode(v8s)),
        Err(err) => Err(Errs::strs("generate_sk_pem", err)),
    }
}

/// 生成RSA私钥并将私钥存储指定文件
///
/// bits 私钥位数
///
/// 如果已存在，删除重写
fn generate_pkcs1_sk_pem_file<P: AsRef<Path>>(bits: u32, filepath: P) -> GeorgeResult<Vec<u8>> {
    match generate_pkcs1_sk_pem(bits) {
        Ok(v8s) => {
            Filer::write_force(filepath, v8s.clone())?;
            Ok(v8s)
        }
        Err(err) => Err(Errs::strs("generate_sk", err)),
    }
}

/// 生成RSA私钥并将私钥存储指定文件
///
/// bits 私钥位数
///
/// 如果已存在，删除重写
fn generate_pkcs8_sk_pem_file<P: AsRef<Path>>(bits: u32, filepath: P) -> GeorgeResult<Vec<u8>> {
    match generate_pkcs8_sk_pem(bits) {
        Ok(v8s) => {
            Filer::write_force(filepath, v8s.clone())?;
            Ok(v8s)
        }
        Err(err) => Err(Errs::strs("generate_sk", err)),
    }
}

fn generate_pkcs1_sk_pem_file_string<P: AsRef<Path>>(
    bits: u32,
    filepath: P,
) -> GeorgeResult<String> {
    match generate_pkcs1_sk_pem_string(bits) {
        Ok(res) => {
            Filer::write_force(filepath, res.clone())?;
            Ok(res)
        }
        Err(err) => Err(Errs::strs("generate_sk", err)),
    }
}

fn generate_pkcs8_sk_pem_file_string<P: AsRef<Path>>(
    bits: u32,
    filepath: P,
) -> GeorgeResult<String> {
    match generate_pkcs8_sk_pem_string(bits) {
        Ok(res) => {
            Filer::write_force(filepath, res.clone())?;
            Ok(res)
        }
        Err(err) => Err(Errs::strs("generate_sk", err)),
    }
}

fn generate_pkcs1_sk_pem_pass_file<P: AsRef<Path>>(
    bits: u32,
    cipher: Cipher,
    passphrase: &[u8],
    filepath: P,
) -> GeorgeResult<Vec<u8>> {
    match generate_pkcs1_sk_pem_pass(bits, cipher, passphrase) {
        Ok(v8s) => {
            Filer::write_force(filepath, v8s.clone())?;
            Ok(v8s)
        }
        Err(err) => Err(Errs::strs("generate_sk", err)),
    }
}

fn generate_pkcs8_sk_pem_pass_file<P: AsRef<Path>>(
    bits: u32,
    cipher: Cipher,
    passphrase: &[u8],
    filepath: P,
) -> GeorgeResult<Vec<u8>> {
    match generate_pkcs8_sk_pem_pass(bits, cipher, passphrase) {
        Ok(v8s) => {
            Filer::write_force(filepath, v8s.clone())?;
            Ok(v8s)
        }
        Err(err) => Err(Errs::strs("generate_sk", err)),
    }
}

fn generate_pkcs1_sk_pem_pass_file_string<P: AsRef<Path>>(
    bits: u32,
    cipher: Cipher,
    passphrase: &[u8],
    filepath: P,
) -> GeorgeResult<String> {
    match generate_pkcs1_sk_pem_pass_string(bits, cipher, passphrase) {
        Ok(res) => {
            Filer::write_force(filepath, res.clone())?;
            Ok(res)
        }
        Err(err) => Err(Errs::strs("generate_sk", err)),
    }
}

fn generate_pkcs8_sk_pem_pass_file_string<P: AsRef<Path>>(
    bits: u32,
    cipher: Cipher,
    passphrase: &[u8],
    filepath: P,
) -> GeorgeResult<String> {
    match generate_pkcs8_sk_pem_pass_string(bits, cipher, passphrase) {
        Ok(res) => {
            Filer::write_force(filepath, res.clone())?;
            Ok(res)
        }
        Err(err) => Err(Errs::strs("generate_sk", err)),
    }
}

fn generate_pkcs1_sk_der_file<P: AsRef<Path>>(bits: u32, filepath: P) -> GeorgeResult<Vec<u8>> {
    match generate_pkcs1_sk_der(bits) {
        Ok(v8s) => {
            Filer::write_force(filepath, v8s.clone())?;
            Ok(v8s)
        }
        Err(err) => Err(Errs::strs("generate_sk", err)),
    }
}

fn generate_pkcs8_sk_der_file<P: AsRef<Path>>(bits: u32, filepath: P) -> GeorgeResult<Vec<u8>> {
    match generate_pkcs8_sk_der(bits) {
        Ok(v8s) => {
            Filer::write_force(filepath, v8s.clone())?;
            Ok(v8s)
        }
        Err(err) => Err(Errs::strs("generate_sk", err)),
    }
}

fn generate_pkcs1_sk_der_base64_file<P: AsRef<Path>>(
    bits: u32,
    filepath: P,
) -> GeorgeResult<String> {
    match generate_pkcs1_sk_der_base64_string(bits) {
        Ok(res) => {
            Filer::write_force(filepath, res.clone())?;
            Ok(res)
        }
        Err(err) => Err(Errs::strs("generate_sk", err)),
    }
}

fn generate_pkcs8_sk_der_base64_file<P: AsRef<Path>>(
    bits: u32,
    filepath: P,
) -> GeorgeResult<String> {
    match generate_pkcs8_sk_der_base64_string(bits) {
        Ok(res) => {
            Filer::write_force(filepath, res.clone())?;
            Ok(res)
        }
        Err(err) => Err(Errs::strs("generate_sk", err)),
    }
}

fn generate_pkcs1_sk_der_hex_file<P: AsRef<Path>>(bits: u32, filepath: P) -> GeorgeResult<String> {
    match generate_pkcs1_sk_der_hex_string(bits) {
        Ok(res) => {
            Filer::write_force(filepath, res.clone())?;
            Ok(res)
        }
        Err(err) => Err(Errs::strs("generate_sk", err)),
    }
}

fn generate_pkcs8_sk_der_hex_file<P: AsRef<Path>>(bits: u32, filepath: P) -> GeorgeResult<String> {
    match generate_pkcs8_sk_der_hex_string(bits) {
        Ok(res) => {
            Filer::write_force(filepath, res.clone())?;
            Ok(res)
        }
        Err(err) => Err(Errs::strs("generate_sk", err)),
    }
}

/// 读取RSA私钥
fn load_sk_pkey_u8s(sk: &[u8]) -> GeorgeResult<PKey<Private>> {
    match PKey::private_key_from_pem(sk) {
        Ok(key) => Ok(key),
        Err(_) => match PKey::private_key_from_pkcs8(sk) {
            Ok(key) => Ok(key),
            Err(_) => match PKey::private_key_from_der(sk) {
                Ok(key) => Ok(key),
                Err(err) => Err(Errs::strs("private_key_from_pem", err)),
            },
        },
    }
}

/// 读取RSA私钥
fn load_sk_pkey(sk: Vec<u8>) -> GeorgeResult<PKey<Private>> {
    load_sk_pkey_u8s(sk.as_slice())
}

/// 读取RSA私钥
pub fn load_sk_pkey_file<P: AsRef<Path>>(filepath: P) -> GeorgeResult<PKey<Private>> {
    match read(filepath.as_ref()) {
        Ok(v8s) => load_sk_pkey(v8s),
        Err(_) => match read_to_string(filepath.as_ref()) {
            Ok(res) => load_sk_pkey(Base64::decode(res)?),
            Err(_) => match read_to_string(filepath.as_ref()) {
                Ok(res) => load_sk_pkey(Hex::decode(res)?),
                Err(_) => match read_to_string(filepath) {
                    Ok(res) => load_sk_pkey_u8s(res.as_bytes()),
                    Err(err) => Err(Errs::strs("load_sk_pkey_file", err)),
                },
            },
        },
    }
}

/// 读取RSA公钥
fn load_pk_pkey_u8s(pk: &[u8]) -> GeorgeResult<PKey<Public>> {
    match PKey::public_key_from_pem(pk) {
        Ok(key) => Ok(key),
        Err(_) => match PKey::public_key_from_der(pk) {
            Ok(key) => Ok(key),
            Err(err) => Err(Errs::strs("private_key_from_pem", err)),
        },
    }
}

/// 读取RSA公钥
fn load_pk_pkey(pk: Vec<u8>) -> GeorgeResult<PKey<Public>> {
    load_pk_pkey_u8s(pk.as_slice())
}

/// 读取RSA公钥
pub fn load_pk_pkey_file<P: AsRef<Path>>(filepath: P) -> GeorgeResult<PKey<Public>> {
    match read(filepath.as_ref()) {
        Ok(v8s) => load_pk_pkey(v8s),
        Err(_) => match read_to_string(filepath.as_ref()) {
            Ok(res) => load_pk_pkey(Base64::decode(res)?),
            Err(_) => match read_to_string(filepath.as_ref()) {
                Ok(res) => load_pk_pkey(Hex::decode(res)?),
                Err(_) => match read_to_string(filepath) {
                    Ok(res) => load_pk_pkey_u8s(res.as_bytes()),
                    Err(err) => Err(Errs::strs("load_sk_pkey_file", err)),
                },
            },
        },
    }
}

/// 读取RSA私钥
fn load_sk_u8s(sk: &[u8]) -> GeorgeResult<Rsa<Private>> {
    match Rsa::private_key_from_pem(sk) {
        Ok(key) => Ok(key),
        Err(_) => match Rsa::private_key_from_der(sk) {
            Ok(key) => Ok(key),
            Err(err) => Err(Errs::strs("private_key_from_pem", err)),
        },
    }
}

/// 读取RSA私钥
fn load_sk(sk: Vec<u8>) -> GeorgeResult<Rsa<Private>> {
    load_sk_u8s(sk.as_slice())
}

/// 读取RSA私钥
fn load_sk_file<P: AsRef<Path>>(filepath: P) -> GeorgeResult<Rsa<Private>> {
    match read(filepath.as_ref()) {
        Ok(v8s) => load_sk(v8s),
        Err(_) => match read_to_string(filepath.as_ref()) {
            Ok(res) => load_sk(Base64::decode(res)?),
            Err(_) => match read_to_string(filepath.as_ref()) {
                Ok(res) => load_sk(Hex::decode(res)?),
                Err(_) => match read_to_string(filepath) {
                    Ok(res) => load_sk_u8s(res.as_bytes()),
                    Err(err) => Err(Errs::strs("load_sk_pkey_file", err)),
                },
            },
        },
    }
}

/// 读取RSA公钥
fn load_pk_u8s(pk: &[u8]) -> GeorgeResult<Rsa<Public>> {
    match Rsa::public_key_from_pem(pk) {
        Ok(key) => Ok(key),
        Err(_) => match Rsa::public_key_from_pem_pkcs1(pk) {
            Ok(key) => Ok(key),
            Err(_) => match Rsa::public_key_from_der(pk) {
                Ok(key) => Ok(key),
                Err(_) => match Rsa::public_key_from_der_pkcs1(pk) {
                    Ok(key) => Ok(key),
                    Err(err) => Err(Errs::strs("private_key_from_pem", err)),
                },
            },
        },
    }
}

/// 读取RSA公钥
fn load_pk(pk: Vec<u8>) -> GeorgeResult<Rsa<Public>> {
    load_pk_u8s(pk.as_slice())
}

/// 读取RSA公钥
pub fn load_pk_file<P: AsRef<Path>>(filepath: P) -> GeorgeResult<Rsa<Public>> {
    match read(filepath.as_ref()) {
        Ok(v8s) => load_pk(v8s),
        Err(_) => match read_to_string(filepath.as_ref()) {
            Ok(res) => load_pk(Base64::decode(res)?),
            Err(_) => match read_to_string(filepath.as_ref()) {
                Ok(res) => load_pk(Hex::decode(res)?),
                Err(_) => match read_to_string(filepath) {
                    Ok(res) => load_pk_u8s(res.as_bytes()),
                    Err(err) => Err(Errs::strs("load_sk_pkey_file", err)),
                },
            },
        },
    }
}

/// 生成RSA公钥
fn generate_pk_pkey_from_pkey_sk(sk: PKey<Private>) -> GeorgeResult<PKey<Public>> {
    match sk.public_key_to_pem() {
        Ok(u8s) => match PKey::public_key_from_pem(u8s.as_slice()) {
            Ok(pk) => Ok(pk),
            Err(err) => Err(Errs::strs("public_key_from_pem", err)),
        },
        Err(err) => Err(Errs::strs("public_key_to_pem", err)),
    }
}

/// 生成RSA公钥
fn generate_pk_pkey_from_pkey_sk_bytes(sk: Vec<u8>) -> GeorgeResult<PKey<Public>> {
    match load_sk_pkey(sk) {
        Ok(sk) => match sk.public_key_to_pem() {
            Ok(u8s) => match PKey::public_key_from_pem(u8s.as_slice()) {
                Ok(pk) => Ok(pk),
                Err(err) => Err(Errs::strs("public_key_from_pem", err)),
            },
            Err(err) => Err(Errs::strs("public_key_to_pem", err)),
        },
        Err(err) => Err(Errs::strs("load_sk_pkey", err)),
    }
}

/// 生成RSA公钥
fn generate_pk_pkey_from_pkey_sk_file<P: AsRef<Path>>(filepath: P) -> GeorgeResult<PKey<Public>> {
    match load_sk_pkey_file(filepath) {
        Ok(sk) => match sk.public_key_to_pem() {
            Ok(u8s) => match PKey::public_key_from_pem(u8s.as_slice()) {
                Ok(pk) => Ok(pk),
                Err(err) => Err(Errs::strs("public_key_from_pem", err)),
            },
            Err(err) => Err(Errs::strs("public_key_to_pem", err)),
        },
        Err(err) => Err(Errs::strs("load_sk_pkey", err)),
    }
}

/// 生成RSA公钥
fn generate_pk_rsa_pkcs1_from_rsa_sk(sk: Rsa<Private>) -> GeorgeResult<Rsa<Public>> {
    match sk.public_key_to_pem_pkcs1() {
        Ok(u8s) => match Rsa::public_key_from_pem_pkcs1(u8s.as_slice()) {
            Ok(pk) => Ok(pk),
            Err(err) => Err(Errs::strs("public_key_from_pem_pkcs1", err)),
        },
        Err(err) => Err(Errs::strs("public_key_to_pem_pkcs1", err)),
    }
}

/// 生成RSA公钥
fn generate_pk_rsa_pkcs8_from_rsa_sk(sk: Rsa<Private>) -> GeorgeResult<Rsa<Public>> {
    match sk.public_key_to_pem() {
        Ok(u8s) => match Rsa::public_key_from_pem(u8s.as_slice()) {
            Ok(pk) => Ok(pk),
            Err(err) => Err(Errs::strs("public_key_from_pem", err)),
        },
        Err(err) => Err(Errs::strs("public_key_to_pem", err)),
    }
}

/// 生成RSA公钥
fn generate_pk_rsa_pkcs1_pem_from_rsa_sk(sk: Rsa<Private>) -> GeorgeResult<Vec<u8>> {
    match sk.public_key_to_pem_pkcs1() {
        Ok(u8s) => Ok(u8s),
        Err(err) => Err(Errs::strs("public_key_to_pem", err)),
    }
}

/// 生成RSA公钥
fn generate_pk_rsa_pkcs8_pem_from_rsa_sk(sk: Rsa<Private>) -> GeorgeResult<Vec<u8>> {
    match sk.public_key_to_pem() {
        Ok(u8s) => Ok(u8s),
        Err(err) => Err(Errs::strs("public_key_to_pem", err)),
    }
}

/// 生成RSA公钥
fn generate_pk_rsa_pkcs1_der_from_rsa_sk(sk: Rsa<Private>) -> GeorgeResult<Vec<u8>> {
    match sk.public_key_to_der_pkcs1() {
        Ok(u8s) => Ok(u8s),
        Err(err) => Err(Errs::strs("public_key_to_pem", err)),
    }
}

/// 生成RSA公钥
fn generate_pk_rsa_pkcs8_der_from_rsa_sk(sk: Rsa<Private>) -> GeorgeResult<Vec<u8>> {
    match sk.public_key_to_der() {
        Ok(u8s) => Ok(u8s),
        Err(err) => Err(Errs::strs("public_key_to_pem", err)),
    }
}

/// 生成RSA公钥
fn generate_pk_rsa_pkcs1_from_rsa_sk_bytes(sk: Vec<u8>) -> GeorgeResult<Rsa<Public>> {
    match load_sk(sk) {
        Ok(sk) => generate_pk_rsa_pkcs1_from_rsa_sk(sk),
        Err(err) => Err(Errs::strs("load_sk_pkey", err)),
    }
}

/// 生成RSA公钥
fn generate_pk_rsa_pkcs8_from_rsa_sk_bytes(sk: Vec<u8>) -> GeorgeResult<Rsa<Public>> {
    match load_sk(sk) {
        Ok(sk) => generate_pk_rsa_pkcs8_from_rsa_sk(sk),
        Err(err) => Err(Errs::strs("load_sk_pkey", err)),
    }
}

/// 生成RSA公钥
fn generate_pk_rsa_pkcs1_from_rsa_sk_file<P: AsRef<Path>>(
    filepath: P,
) -> GeorgeResult<Rsa<Public>> {
    match load_sk_file(filepath) {
        Ok(sk) => generate_pk_rsa_pkcs1_from_rsa_sk(sk),
        Err(err) => Err(Errs::strs("load_sk_pkey", err)),
    }
}

/// 生成RSA公钥
fn generate_pk_rsa_pkcs8_from_rsa_sk_file<P: AsRef<Path>>(
    filepath: P,
) -> GeorgeResult<Rsa<Public>> {
    match load_sk_file(filepath) {
        Ok(sk) => generate_pk_rsa_pkcs8_from_rsa_sk(sk),
        Err(err) => Err(Errs::strs("load_sk_pkey", err)),
    }
}

/// 生成RSA公钥
fn generate_pk_rsa_pkcs1_pem_from_sk_file<P: AsRef<Path>>(filepath: P) -> GeorgeResult<Vec<u8>> {
    match load_sk_file(filepath) {
        Ok(key) => generate_pk_rsa_pkcs1_pem_from_rsa_sk(key),
        Err(err) => Err(Errs::strs("load_sk_file", err)),
    }
}

/// 生成RSA公钥
fn generate_pk_rsa_pkcs8_pem_from_sk_file<P: AsRef<Path>>(filepath: P) -> GeorgeResult<Vec<u8>> {
    match load_sk_file(filepath) {
        Ok(key) => generate_pk_rsa_pkcs8_pem_from_rsa_sk(key),
        Err(err) => Err(Errs::strs("load_sk_file", err)),
    }
}

/// 生成RSA公钥
fn generate_pk_rsa_pkcs1_pem_string_from_sk_file<P: AsRef<Path>>(
    filepath: P,
) -> GeorgeResult<String> {
    Strings::from_utf8(generate_pk_rsa_pkcs1_pem_from_sk_file(filepath)?)
}

/// 生成RSA公钥
fn generate_pk_rsa_pkcs8_pem_string_from_sk_file<P: AsRef<Path>>(
    filepath: P,
) -> GeorgeResult<String> {
    Strings::from_utf8(generate_pk_rsa_pkcs8_pem_from_sk_file(filepath)?)
}

/// 生成RSA公钥
fn generate_pk_rsa_pkcs1_der_hex_from_sk_file<P: AsRef<Path>>(filepath: P) -> GeorgeResult<String> {
    Ok(Hex::encode(generate_pk_rsa_pkcs1_der_from_sk_file(
        filepath,
    )?))
}

/// 生成RSA公钥
fn generate_pk_rsa_pkcs1_der_base64_from_sk_file<P: AsRef<Path>>(
    filepath: P,
) -> GeorgeResult<String> {
    Ok(Base64::encode(generate_pk_rsa_pkcs1_der_from_sk_file(
        filepath,
    )?))
}

/// 生成RSA公钥
fn generate_pk_rsa_pkcs8_der_base64_from_sk_file<P: AsRef<Path>>(
    filepath: P,
) -> GeorgeResult<String> {
    Ok(Base64::encode(generate_pk_rsa_pkcs8_der_from_sk_file(
        filepath,
    )?))
}

/// 生成RSA公钥
fn generate_pk_rsa_pkcs1_der_from_sk_file<P: AsRef<Path>>(filepath: P) -> GeorgeResult<Vec<u8>> {
    match load_sk_file(filepath) {
        Ok(key) => generate_pk_rsa_pkcs1_der_from_rsa_sk(key),
        Err(err) => Err(Errs::strs("load_sk_file", err)),
    }
}

/// 生成RSA公钥
fn generate_pk_rsa_pkcs8_der_from_sk_file<P: AsRef<Path>>(filepath: P) -> GeorgeResult<Vec<u8>> {
    match load_sk_file(filepath) {
        Ok(key) => generate_pk_rsa_pkcs8_der_from_rsa_sk(key),
        Err(err) => Err(Errs::strs("load_sk_file", err)),
    }
}

/// 读取RSA公钥
fn load_bytes_from_file<P: AsRef<Path>>(filepath: P) -> GeorgeResult<Vec<u8>> {
    match read(filepath) {
        Ok(u8s) => Ok(u8s),
        Err(err) => Err(Errs::strs("read", err)),
    }
}

/// 读取RSA公钥
fn load_string_from_file<P: AsRef<Path>>(filepath: P) -> GeorgeResult<String> {
    match read_to_string(filepath) {
        Ok(res) => Ok(res),
        Err(err) => Err(Errs::strs("read", err)),
    }
}
