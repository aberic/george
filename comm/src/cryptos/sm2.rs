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

use crate::cryptos::base64::{Base64, Base64DecodeHandler, Base64EncodeHandler};
use crate::cryptos::traits::{
    AEKeyHex, AELoadKey, AENew, AENewStore, AEPkString, AEPkStringPath, AEPkV8s, AEPkV8sPath,
    AESign, AESignPath, AESkNew, AESkNewStore, AEStoreKey, AEVerify, AEVerifyPath,
};
use crate::errors::entrances::GeorgeResult;
use crate::errors::entrances::{err_str, err_strs};
use crate::io::file::{Filer, FilerWriter};

/// 字节数组与字符串通过Base64转换
pub struct SM2;

////////// sm generate start //////////

impl AESkNew for SM2 {
    fn generate() -> Vec<u8> {
        generate_sk()
    }

    fn generate_string() -> String {
        generate_sk_string()
    }
}

impl AESkNewStore<String> for SM2 {
    fn generate(sk_filepath: String) -> GeorgeResult<Vec<u8>> {
        generate_sk_in_file(sk_filepath)
    }

    fn generate_string(sk_filepath: String) -> GeorgeResult<String> {
        generate_sk_string_in_file(sk_filepath)
    }
}

impl AESkNewStore<&str> for SM2 {
    fn generate(sk_filepath: &str) -> GeorgeResult<Vec<u8>> {
        generate_sk_in_file(sk_filepath.to_string())
    }

    fn generate_string(sk_filepath: &str) -> GeorgeResult<String> {
        generate_sk_string_in_file(sk_filepath.to_string())
    }
}

impl AENew for SM2 {
    fn generate() -> (Vec<u8>, Vec<u8>) {
        generate()
    }

    fn generate_string() -> (String, String) {
        generate_string()
    }
}

impl AENewStore<String> for SM2 {
    fn generate(sk_filepath: String, pk_filepath: String) -> GeorgeResult<(Vec<u8>, Vec<u8>)> {
        generate_in_file(sk_filepath, pk_filepath)
    }

    fn generate_string(sk_filepath: String, pk_filepath: String) -> GeorgeResult<(String, String)> {
        generate_string_in_file(sk_filepath, pk_filepath)
    }
}

impl AENewStore<&str> for SM2 {
    fn generate(sk_filepath: &str, pk_filepath: &str) -> GeorgeResult<(Vec<u8>, Vec<u8>)> {
        generate_in_file(sk_filepath.to_string(), pk_filepath.to_string())
    }

    fn generate_string(sk_filepath: &str, pk_filepath: &str) -> GeorgeResult<(String, String)> {
        generate_string_in_file(sk_filepath.to_string(), pk_filepath.to_string())
    }
}

////////// sm generate end //////////

////////// sm generate pk from sk start //////////
impl AEPkV8s<Vec<u8>> for SM2 {
    fn generate_pk(sk: Vec<u8>) -> GeorgeResult<Vec<u8>> {
        generate_pk_from_sk(sk)
    }
}

impl AEPkV8s<String> for SM2 {
    fn generate_pk(sk: String) -> GeorgeResult<Vec<u8>> {
        generate_pk_from_sk_str(sk)
    }
}

impl AEPkString<Vec<u8>> for SM2 {
    fn generate_pk(sk: Vec<u8>) -> GeorgeResult<String> {
        Ok(key_to_string(generate_pk_from_sk(sk)?))
    }
}

impl AEPkString<String> for SM2 {
    fn generate_pk(sk: String) -> GeorgeResult<String> {
        Ok(key_to_string(generate_pk_from_sk_str(sk)?))
    }
}

impl AEPkV8sPath for SM2 {
    fn generate_pk<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Vec<u8>> {
        generate_pk_from_sk_file(sk_filepath)
    }
}

impl AEPkStringPath for SM2 {
    fn generate_pk<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<String> {
        Ok(key_to_string(generate_pk_from_sk_file(sk_filepath)?))
    }
}

////////// sm generate pk from sk end //////////

impl AEKeyHex for SM2 {
    fn key_encode(key: Vec<u8>) -> String {
        key_to_string(key)
    }

    fn key_decode(key: String) -> GeorgeResult<Vec<u8>> {
        key_from_string(key)
    }
}

////////// sm store/load start //////////

impl AEStoreKey<String> for SM2 {
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

impl AEStoreKey<&str> for SM2 {
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

impl AELoadKey for SM2 {
    fn load_from_file<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<Vec<u8>> {
        load_key_from_file(key_filepath)
    }

    fn load_string_from_file<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<String> {
        load_key_string_from_file(key_filepath)
    }
}

////////// sm store/load end //////////

////////// sm sign start //////////

impl AESign<&[u8], &[u8]> for SM2 {
    fn sign(msg: &[u8], sk: &[u8], pk: &[u8]) -> GeorgeResult<Vec<u8>> {
        sign(msg, sk, pk)
    }

    fn sign_string(msg: &[u8], sk: &[u8], pk: &[u8]) -> GeorgeResult<String> {
        Ok(key_to_string(sign(msg, sk, pk)?))
    }
}

impl AESign<&[u8], Vec<u8>> for SM2 {
    fn sign(msg: &[u8], sk: Vec<u8>, pk: Vec<u8>) -> GeorgeResult<Vec<u8>> {
        sign(msg, sk.as_slice(), pk.as_slice())
    }

    fn sign_string(msg: &[u8], sk: Vec<u8>, pk: Vec<u8>) -> GeorgeResult<String> {
        Ok(key_to_string(sign(msg, sk.as_slice(), pk.as_slice())?))
    }
}

impl AESign<Vec<u8>, Vec<u8>> for SM2 {
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

impl AESign<String, Vec<u8>> for SM2 {
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

impl AESign<&str, Vec<u8>> for SM2 {
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

impl AESign<Vec<u8>, &[u8]> for SM2 {
    fn sign(msg: Vec<u8>, sk: &[u8], pk: &[u8]) -> GeorgeResult<Vec<u8>> {
        sign(msg.as_slice(), sk, pk)
    }

    fn sign_string(msg: Vec<u8>, sk: &[u8], pk: &[u8]) -> GeorgeResult<String> {
        Ok(key_to_string(sign(msg.as_slice(), sk, pk)?))
    }
}

impl AESign<String, &[u8]> for SM2 {
    fn sign(msg: String, sk: &[u8], pk: &[u8]) -> GeorgeResult<Vec<u8>> {
        sign(msg.as_bytes(), sk, pk)
    }

    fn sign_string(msg: String, sk: &[u8], pk: &[u8]) -> GeorgeResult<String> {
        Ok(key_to_string(sign(msg.as_bytes(), sk, pk)?))
    }
}

impl AESign<&str, &[u8]> for SM2 {
    fn sign(msg: &str, sk: &[u8], pk: &[u8]) -> GeorgeResult<Vec<u8>> {
        sign(msg.as_bytes(), sk, pk)
    }

    fn sign_string(msg: &str, sk: &[u8], pk: &[u8]) -> GeorgeResult<String> {
        Ok(key_to_string(sign(msg.as_bytes(), sk, pk)?))
    }
}

impl AESign<&[u8], String> for SM2 {
    fn sign(msg: &[u8], sk: String, pk: String) -> GeorgeResult<Vec<u8>> {
        sign(
            msg,
            key_from_string(sk)?.as_slice(),
            key_from_string(pk)?.as_slice(),
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

impl AESign<Vec<u8>, String> for SM2 {
    fn sign(msg: Vec<u8>, sk: String, pk: String) -> GeorgeResult<Vec<u8>> {
        sign(
            msg.as_slice(),
            key_from_string(sk)?.as_slice(),
            key_from_string(pk)?.as_slice(),
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

impl AESign<String, String> for SM2 {
    fn sign(msg: String, sk: String, pk: String) -> GeorgeResult<Vec<u8>> {
        sign(
            msg.as_bytes(),
            key_from_string(sk)?.as_slice(),
            key_from_string(pk)?.as_slice(),
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

impl AESign<&str, String> for SM2 {
    fn sign(msg: &str, sk: String, pk: String) -> GeorgeResult<Vec<u8>> {
        sign(
            msg.as_bytes(),
            key_from_string(sk)?.as_slice(),
            key_from_string(pk)?.as_slice(),
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

impl AESign<&[u8], &str> for SM2 {
    fn sign(msg: &[u8], sk: &str, pk: &str) -> GeorgeResult<Vec<u8>> {
        sign(
            msg,
            key_from_string(sk.to_string())?.as_slice(),
            key_from_string(pk.to_string())?.as_slice(),
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

impl AESign<Vec<u8>, &str> for SM2 {
    fn sign(msg: Vec<u8>, sk: &str, pk: &str) -> GeorgeResult<Vec<u8>> {
        sign(
            msg.as_slice(),
            key_from_string(sk.to_string())?.as_slice(),
            key_from_string(pk.to_string())?.as_slice(),
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

impl AESign<String, &str> for SM2 {
    fn sign(msg: String, sk: &str, pk: &str) -> GeorgeResult<Vec<u8>> {
        sign(
            msg.as_bytes(),
            key_from_string(sk.to_string())?.as_slice(),
            key_from_string(pk.to_string())?.as_slice(),
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

impl AESign<&str, &str> for SM2 {
    fn sign(msg: &str, sk: &str, pk: &str) -> GeorgeResult<Vec<u8>> {
        sign(
            msg.as_bytes(),
            key_from_string(sk.to_string())?.as_slice(),
            key_from_string(pk.to_string())?.as_slice(),
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

impl AESignPath<&[u8]> for SM2 {
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

impl AESignPath<Vec<u8>> for SM2 {
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

impl AESignPath<String> for SM2 {
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

impl AESignPath<&str> for SM2 {
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

impl AEVerify<&[u8], &[u8], &[u8]> for SM2 {
    fn verify(msg: &[u8], pk: &[u8], der: &[u8]) -> GeorgeResult<bool> {
        verify(msg, pk, der)
    }
}

impl AEVerify<&[u8], &[u8], Vec<u8>> for SM2 {
    fn verify(msg: &[u8], pk: &[u8], der: Vec<u8>) -> GeorgeResult<bool> {
        verify(msg, pk, der.as_slice())
    }
}

impl AEVerify<&[u8], &[u8], String> for SM2 {
    fn verify(msg: &[u8], pk: &[u8], der: String) -> GeorgeResult<bool> {
        verify(msg, pk, key_from_string(der)?.as_slice())
    }
}

impl AEVerify<&[u8], &[u8], &str> for SM2 {
    fn verify(msg: &[u8], pk: &[u8], der: &str) -> GeorgeResult<bool> {
        verify(msg, pk, key_from_string(der.to_string())?.as_slice())
    }
}

impl AEVerify<&[u8], Vec<u8>, &[u8]> for SM2 {
    fn verify(msg: &[u8], pk: Vec<u8>, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg, pk.as_slice(), der)
    }
}

impl AEVerify<&[u8], Vec<u8>, Vec<u8>> for SM2 {
    fn verify(msg: &[u8], pk: Vec<u8>, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(msg, pk.as_slice(), der.as_slice())
    }
}

impl AEVerify<&[u8], Vec<u8>, String> for SM2 {
    fn verify(msg: &[u8], pk: Vec<u8>, der: String) -> GeorgeResult<bool> {
        verify(msg, pk.as_slice(), key_from_string(der)?.as_slice())
    }
}

impl AEVerify<&[u8], Vec<u8>, &str> for SM2 {
    fn verify(msg: &[u8], pk: Vec<u8>, der: &str) -> GeorgeResult<bool> {
        verify(
            msg,
            pk.as_slice(),
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl AEVerify<&[u8], String, &[u8]> for SM2 {
    fn verify(msg: &[u8], pk: String, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg, &key_from_string(pk)?.as_slice(), der)
    }
}

impl AEVerify<&[u8], String, Vec<u8>> for SM2 {
    fn verify(msg: &[u8], pk: String, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(msg, &key_from_string(pk)?.as_slice(), der.as_slice())
    }
}

impl AEVerify<&[u8], String, String> for SM2 {
    fn verify(msg: &[u8], pk: String, der: String) -> GeorgeResult<bool> {
        verify(
            msg,
            &key_from_string(pk)?.as_slice(),
            &key_from_string(der)?.as_slice(),
        )
    }
}

impl AEVerify<&[u8], String, &str> for SM2 {
    fn verify(msg: &[u8], pk: String, der: &str) -> GeorgeResult<bool> {
        verify(
            msg,
            &key_from_string(pk)?.as_slice(),
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl AEVerify<&[u8], &str, &[u8]> for SM2 {
    fn verify(msg: &[u8], pk: &str, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg, &key_from_string(pk.to_string())?.as_slice(), der)
    }
}

impl AEVerify<&[u8], &str, Vec<u8>> for SM2 {
    fn verify(msg: &[u8], pk: &str, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg,
            &key_from_string(pk.to_string())?.as_slice(),
            der.as_slice(),
        )
    }
}

impl AEVerify<&[u8], &str, String> for SM2 {
    fn verify(msg: &[u8], pk: &str, der: String) -> GeorgeResult<bool> {
        verify(
            msg,
            &key_from_string(pk.to_string())?.as_slice(),
            key_from_string(der)?.as_slice(),
        )
    }
}

impl AEVerify<&[u8], &str, &str> for SM2 {
    fn verify(msg: &[u8], pk: &str, der: &str) -> GeorgeResult<bool> {
        verify(
            msg,
            &key_from_string(pk.to_string())?.as_slice(),
            &key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl AEVerify<Vec<u8>, &[u8], &[u8]> for SM2 {
    fn verify(msg: Vec<u8>, pk: &[u8], der: &[u8]) -> GeorgeResult<bool> {
        verify(msg.as_slice(), pk, der)
    }
}

impl AEVerify<Vec<u8>, &[u8], Vec<u8>> for SM2 {
    fn verify(msg: Vec<u8>, pk: &[u8], der: Vec<u8>) -> GeorgeResult<bool> {
        verify(msg.as_slice(), pk, der.as_slice())
    }
}

impl AEVerify<Vec<u8>, &[u8], String> for SM2 {
    fn verify(msg: Vec<u8>, pk: &[u8], der: String) -> GeorgeResult<bool> {
        verify(msg.as_slice(), pk, key_from_string(der)?.as_slice())
    }
}

impl AEVerify<Vec<u8>, &[u8], &str> for SM2 {
    fn verify(msg: Vec<u8>, pk: &[u8], der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            pk,
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl AEVerify<Vec<u8>, Vec<u8>, &[u8]> for SM2 {
    fn verify(msg: Vec<u8>, pk: Vec<u8>, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg.as_slice(), pk.as_slice(), der)
    }
}

impl AEVerify<Vec<u8>, Vec<u8>, Vec<u8>> for SM2 {
    fn verify(msg: Vec<u8>, pk: Vec<u8>, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(msg.as_slice(), pk.as_slice(), der.as_slice())
    }
}

impl AEVerify<Vec<u8>, Vec<u8>, String> for SM2 {
    fn verify(msg: Vec<u8>, pk: Vec<u8>, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            pk.as_slice(),
            key_from_string(der)?.as_slice(),
        )
    }
}

impl AEVerify<Vec<u8>, Vec<u8>, &str> for SM2 {
    fn verify(msg: Vec<u8>, pk: Vec<u8>, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            pk.as_slice(),
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl AEVerify<Vec<u8>, String, &[u8]> for SM2 {
    fn verify(msg: Vec<u8>, pk: String, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg.as_slice(), &key_from_string(pk)?.as_slice(), der)
    }
}

impl AEVerify<Vec<u8>, String, Vec<u8>> for SM2 {
    fn verify(msg: Vec<u8>, pk: String, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            &key_from_string(pk)?.as_slice(),
            der.as_slice(),
        )
    }
}

impl AEVerify<Vec<u8>, String, String> for SM2 {
    fn verify(msg: Vec<u8>, pk: String, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            &key_from_string(pk)?.as_slice(),
            &key_from_string(der)?.as_slice(),
        )
    }
}

impl AEVerify<Vec<u8>, String, &str> for SM2 {
    fn verify(msg: Vec<u8>, pk: String, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            &key_from_string(pk)?.as_slice(),
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl AEVerify<Vec<u8>, &str, &[u8]> for SM2 {
    fn verify(msg: Vec<u8>, pk: &str, der: &[u8]) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            &key_from_string(pk.to_string())?.as_slice(),
            der,
        )
    }
}

impl AEVerify<Vec<u8>, &str, Vec<u8>> for SM2 {
    fn verify(msg: Vec<u8>, pk: &str, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            &key_from_string(pk.to_string())?.as_slice(),
            der.as_slice(),
        )
    }
}

impl AEVerify<Vec<u8>, &str, String> for SM2 {
    fn verify(msg: Vec<u8>, pk: &str, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            &key_from_string(pk.to_string())?.as_slice(),
            key_from_string(der)?.as_slice(),
        )
    }
}

impl AEVerify<Vec<u8>, &str, &str> for SM2 {
    fn verify(msg: Vec<u8>, pk: &str, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            &key_from_string(pk.to_string())?.as_slice(),
            &key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl AEVerify<String, &[u8], &[u8]> for SM2 {
    fn verify(msg: String, pk: &[u8], der: &[u8]) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk, der)
    }
}

impl AEVerify<String, &[u8], Vec<u8>> for SM2 {
    fn verify(msg: String, pk: &[u8], der: Vec<u8>) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk, der.as_slice())
    }
}

impl AEVerify<String, &[u8], String> for SM2 {
    fn verify(msg: String, pk: &[u8], der: String) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk, key_from_string(der)?.as_slice())
    }
}

impl AEVerify<String, &[u8], &str> for SM2 {
    fn verify(msg: String, pk: &[u8], der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            pk,
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl AEVerify<String, Vec<u8>, &[u8]> for SM2 {
    fn verify(msg: String, pk: Vec<u8>, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk.as_slice(), der)
    }
}

impl AEVerify<String, Vec<u8>, Vec<u8>> for SM2 {
    fn verify(msg: String, pk: Vec<u8>, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk.as_slice(), der.as_slice())
    }
}

impl AEVerify<String, Vec<u8>, String> for SM2 {
    fn verify(msg: String, pk: Vec<u8>, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            pk.as_slice(),
            key_from_string(der)?.as_slice(),
        )
    }
}

impl AEVerify<String, Vec<u8>, &str> for SM2 {
    fn verify(msg: String, pk: Vec<u8>, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            pk.as_slice(),
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl AEVerify<String, String, &[u8]> for SM2 {
    fn verify(msg: String, pk: String, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), &key_from_string(pk)?.as_slice(), der)
    }
}

impl AEVerify<String, String, Vec<u8>> for SM2 {
    fn verify(msg: String, pk: String, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk)?.as_slice(),
            der.as_slice(),
        )
    }
}

impl AEVerify<String, String, String> for SM2 {
    fn verify(msg: String, pk: String, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk)?.as_slice(),
            &key_from_string(der)?.as_slice(),
        )
    }
}

impl AEVerify<String, String, &str> for SM2 {
    fn verify(msg: String, pk: String, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk)?.as_slice(),
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl AEVerify<String, &str, &[u8]> for SM2 {
    fn verify(msg: String, pk: &str, der: &[u8]) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk.to_string())?.as_slice(),
            der,
        )
    }
}

impl AEVerify<String, &str, Vec<u8>> for SM2 {
    fn verify(msg: String, pk: &str, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk.to_string())?.as_slice(),
            der.as_slice(),
        )
    }
}

impl AEVerify<String, &str, String> for SM2 {
    fn verify(msg: String, pk: &str, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk.to_string())?.as_slice(),
            key_from_string(der)?.as_slice(),
        )
    }
}

impl AEVerify<String, &str, &str> for SM2 {
    fn verify(msg: String, pk: &str, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk.to_string())?.as_slice(),
            &key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl AEVerify<&str, &[u8], &[u8]> for SM2 {
    fn verify(msg: &str, pk: &[u8], der: &[u8]) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk, der)
    }
}

impl AEVerify<&str, &[u8], Vec<u8>> for SM2 {
    fn verify(msg: &str, pk: &[u8], der: Vec<u8>) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk, der.as_slice())
    }
}

impl AEVerify<&str, &[u8], String> for SM2 {
    fn verify(msg: &str, pk: &[u8], der: String) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk, key_from_string(der)?.as_slice())
    }
}

impl AEVerify<&str, &[u8], &str> for SM2 {
    fn verify(msg: &str, pk: &[u8], der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            pk,
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl AEVerify<&str, Vec<u8>, &[u8]> for SM2 {
    fn verify(msg: &str, pk: Vec<u8>, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk.as_slice(), der)
    }
}

impl AEVerify<&str, Vec<u8>, Vec<u8>> for SM2 {
    fn verify(msg: &str, pk: Vec<u8>, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), pk.as_slice(), der.as_slice())
    }
}

impl AEVerify<&str, Vec<u8>, String> for SM2 {
    fn verify(msg: &str, pk: Vec<u8>, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            pk.as_slice(),
            key_from_string(der)?.as_slice(),
        )
    }
}

impl AEVerify<&str, Vec<u8>, &str> for SM2 {
    fn verify(msg: &str, pk: Vec<u8>, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            pk.as_slice(),
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl AEVerify<&str, String, &[u8]> for SM2 {
    fn verify(msg: &str, pk: String, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg.as_bytes(), &key_from_string(pk)?.as_slice(), der)
    }
}

impl AEVerify<&str, String, Vec<u8>> for SM2 {
    fn verify(msg: &str, pk: String, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk)?.as_slice(),
            der.as_slice(),
        )
    }
}

impl AEVerify<&str, String, String> for SM2 {
    fn verify(msg: &str, pk: String, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk)?.as_slice(),
            &key_from_string(der)?.as_slice(),
        )
    }
}

impl AEVerify<&str, String, &str> for SM2 {
    fn verify(msg: &str, pk: String, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk)?.as_slice(),
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl AEVerify<&str, &str, &[u8]> for SM2 {
    fn verify(msg: &str, pk: &str, der: &[u8]) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk.to_string())?.as_slice(),
            der,
        )
    }
}

impl AEVerify<&str, &str, Vec<u8>> for SM2 {
    fn verify(msg: &str, pk: &str, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk.to_string())?.as_slice(),
            der.as_slice(),
        )
    }
}

impl AEVerify<&str, &str, String> for SM2 {
    fn verify(msg: &str, pk: &str, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk.to_string())?.as_slice(),
            key_from_string(der)?.as_slice(),
        )
    }
}

impl AEVerify<&str, &str, &str> for SM2 {
    fn verify(msg: &str, pk: &str, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            &key_from_string(pk.to_string())?.as_slice(),
            &key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl AEVerifyPath<&[u8], &[u8]> for SM2 {
    fn verify<P: AsRef<Path>>(msg: &[u8], pk_filepath: P, der: &[u8]) -> GeorgeResult<bool> {
        verify(msg, load_key_from_file(pk_filepath)?.as_slice(), der)
    }
}

impl AEVerifyPath<&[u8], Vec<u8>> for SM2 {
    fn verify<P: AsRef<Path>>(msg: &[u8], pk_filepath: P, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg,
            load_key_from_file(pk_filepath)?.as_slice(),
            der.as_slice(),
        )
    }
}

impl AEVerifyPath<&[u8], String> for SM2 {
    fn verify<P: AsRef<Path>>(msg: &[u8], pk_filepath: P, der: String) -> GeorgeResult<bool> {
        verify(
            msg,
            load_key_from_file(pk_filepath)?.as_slice(),
            key_from_string(der)?.as_slice(),
        )
    }
}

impl AEVerifyPath<&[u8], &str> for SM2 {
    fn verify<P: AsRef<Path>>(msg: &[u8], pk_filepath: P, der: &str) -> GeorgeResult<bool> {
        verify(
            msg,
            load_key_from_file(pk_filepath)?.as_slice(),
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl AEVerifyPath<Vec<u8>, &[u8]> for SM2 {
    fn verify<P: AsRef<Path>>(msg: Vec<u8>, pk_filepath: P, der: &[u8]) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
            der,
        )
    }
}

impl AEVerifyPath<Vec<u8>, Vec<u8>> for SM2 {
    fn verify<P: AsRef<Path>>(msg: Vec<u8>, pk_filepath: P, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
            der.as_slice(),
        )
    }
}

impl AEVerifyPath<Vec<u8>, String> for SM2 {
    fn verify<P: AsRef<Path>>(msg: Vec<u8>, pk_filepath: P, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
            key_from_string(der)?.as_slice(),
        )
    }
}

impl AEVerifyPath<Vec<u8>, &str> for SM2 {
    fn verify<P: AsRef<Path>>(msg: Vec<u8>, pk_filepath: P, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_slice(),
            load_key_from_file(pk_filepath)?.as_slice(),
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl AEVerifyPath<String, &[u8]> for SM2 {
    fn verify<P: AsRef<Path>>(msg: String, pk_filepath: P, der: &[u8]) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            load_key_from_file(pk_filepath)?.as_slice(),
            der,
        )
    }
}

impl AEVerifyPath<String, Vec<u8>> for SM2 {
    fn verify<P: AsRef<Path>>(msg: String, pk_filepath: P, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            load_key_from_file(pk_filepath)?.as_slice(),
            der.as_slice(),
        )
    }
}

impl AEVerifyPath<String, String> for SM2 {
    fn verify<P: AsRef<Path>>(msg: String, pk_filepath: P, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            load_key_from_file(pk_filepath)?.as_slice(),
            key_from_string(der)?.as_slice(),
        )
    }
}

impl AEVerifyPath<String, &str> for SM2 {
    fn verify<P: AsRef<Path>>(msg: String, pk_filepath: P, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            load_key_from_file(pk_filepath)?.as_slice(),
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

impl AEVerifyPath<&str, &[u8]> for SM2 {
    fn verify<P: AsRef<Path>>(msg: &str, pk_filepath: P, der: &[u8]) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            load_key_from_file(pk_filepath)?.as_slice(),
            der,
        )
    }
}

impl AEVerifyPath<&str, Vec<u8>> for SM2 {
    fn verify<P: AsRef<Path>>(msg: &str, pk_filepath: P, der: Vec<u8>) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            load_key_from_file(pk_filepath)?.as_slice(),
            der.as_slice(),
        )
    }
}

impl AEVerifyPath<&str, String> for SM2 {
    fn verify<P: AsRef<Path>>(msg: &str, pk_filepath: P, der: String) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            load_key_from_file(pk_filepath)?.as_slice(),
            key_from_string(der)?.as_slice(),
        )
    }
}

impl AEVerifyPath<&str, &str> for SM2 {
    fn verify<P: AsRef<Path>>(msg: &str, pk_filepath: P, der: &str) -> GeorgeResult<bool> {
        verify(
            msg.as_bytes(),
            load_key_from_file(pk_filepath)?.as_slice(),
            key_from_string(der.to_string())?.as_slice(),
        )
    }
}

////////// sm verify end //////////

fn store_sk_key(key: String, key_filepath: String) -> GeorgeResult<()> {
    match Filer::write_force(key_filepath, key) {
        Ok(_) => Ok(()),
        Err(err) => Err(err_strs("store key", err)),
    }
}

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
    Base64::encode(key)
}

fn key_from_string(key: String) -> GeorgeResult<Vec<u8>> {
    match Base64::decode(key) {
        Ok(res) => Ok(res),
        Err(err) => Err(err_strs("Base64 decode", err)),
    }
}

fn generate() -> (Vec<u8>, Vec<u8>) {
    let ctx = SigCtx::new();
    let (pk, sk) = ctx.new_keypair();
    (ctx.serialize_seckey(&sk), ctx.serialize_pubkey(&pk, true))
}

fn generate_string() -> (String, String) {
    let (sk, pk) = generate();
    (key_to_string(sk), key_to_string(pk))
}

fn generate_sk() -> Vec<u8> {
    let ctx = SigCtx::new();
    let (_pk, sk) = ctx.new_keypair();
    ctx.serialize_seckey(&sk)
}

fn generate_sk_string() -> String {
    key_to_string(generate_sk())
}

fn generate_pk_from_sk(sk: Vec<u8>) -> GeorgeResult<Vec<u8>> {
    let ctx = SigCtx::new();
    match ctx.load_seckey(sk.as_slice()) {
        Ok(p) => Ok(ctx.serialize_pubkey(&ctx.pk_from_sk(&p), true)),
        Err(()) => Err(err_str("unknown")),
    }
}

fn generate_pk_from_sk_str(sk: String) -> GeorgeResult<Vec<u8>> {
    generate_pk_from_sk(key_from_string(sk)?)
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

fn generate_string_in_file(
    sk_filepath: String,
    pk_filepath: String,
) -> GeorgeResult<(String, String)> {
    let (sk_str, pk_str) = generate_string();
    store_key(sk_str.clone(), sk_filepath)?;
    store_key(pk_str.clone(), pk_filepath)?;
    Ok((sk_str, pk_str))
}

fn generate_sk_in_file(sk_filepath: String) -> GeorgeResult<Vec<u8>> {
    let (sk_bytes, _pk_bytes) = generate();
    store_key(key_to_string(sk_bytes.clone()), sk_filepath)?;
    Ok(sk_bytes)
}

fn generate_sk_string_in_file(sk_filepath: String) -> GeorgeResult<String> {
    let (sk_str, _pk_str) = generate_string();
    store_key(sk_str.clone(), sk_filepath)?;
    Ok(sk_str)
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
