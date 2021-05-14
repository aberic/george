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
use std::path::Path;

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

    // /// 生成ECDSA对象
    // pub fn from_hex_file<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<ECDSA> {
    //     let sk_bytes = load_bytes_file(sk_filepath);
    //     ECDSA::from_hex(s)
    //     Ok(ECDSA { sk, pk })
    // }
    //
    // /// 生成ECDSA对象
    // pub fn from_files<P: AsRef<Path>>(sk_filepath: P, pk_filepath: P) -> GeorgeResult<ECDSA> {
    //     let (sk, pk) = generate_pk_from_sk(sk)?;
    //     Ok(ECDSA { sk, pk })
    // }

    pub fn sk(&self) -> EcKey<Private> {
        self.sk.clone()
    }

    pub fn pk(&self) -> EcKey<Public> {
        self.pk.clone()
    }
}

/// fmt method
impl ECDSA {
    /// 8ef9639640e5989c559f78dfff4aef383d1340bb71661433ae475e1f52f128e2
    pub fn sk_hex(&self) -> String {
        Hex::encode(self.sk.private_key().to_vec())
    }
    /// jvljlkDlmJxVn3jf/0rvOD0TQLtxZhQzrkdeH1LxKOI=
    pub fn sk_base64(&self) -> String {
        Base64::encode(self.sk.private_key().to_vec())
    }

    pub fn sk_pem(&self) -> GeorgeResult<Vec<u8>> {
        match self.sk.private_key_to_pem() {
            Ok(res) => Ok(res),
            Err(err) => Err(err_strs("private_key_to_pem", err)),
        }
    }

    /// -----BEGIN EC PRIVATE KEY-----
    /// MHcCAQEEII75Y5ZA5ZicVZ943/9K7zg9E0C7cWYUM65HXh9S8SjioAoGCCqGSM49
    /// AwEHoUQDQgAEg+XjX4DNDSQZhLaawNTfUXmCA2IHkEH9BebmKtcTf/RNpFfJvSqE
    /// m5WsWIMRyz9jE1EQ7HNBySlu7Q3Qshx8lQ==
    /// -----END EC PRIVATE KEY-----
    pub fn sk_pem_str(&self) -> GeorgeResult<String> {
        match self.sk.private_key_to_pem() {
            Ok(res) => Strings::from_utf8(res),
            Err(err) => Err(err_strs("private_key_to_pem", err)),
        }
    }

    /// 2d2d2d2d2d424547494e2045432050524956415445204b45592d2d2d2d2d0a4d4863434151454549493735593
    /// 55a41355a6963565a3934332f394b377a673945304337635759554d3635485868395338536a696f416f474343
    /// 7147534d34390a417745486f55514451674145672b586a5834444e4453515a684c6161774e546655586d43413
    /// 249486b4548394265626d4b746354662f524e7046664a765371450a6d35577357494d52797a396a4531455137
    /// 484e4279536c7537513351736878386c513d3d0a2d2d2d2d2d454e442045432050524956415445204b45592d2
    /// d2d2d2d0a
    pub fn sk_pem_hex(&self) -> GeorgeResult<String> {
        match self.sk.private_key_to_pem() {
            Ok(res) => Ok(Hex::encode(res)),
            Err(err) => Err(err_strs("private_key_to_pem", err)),
        }
    }

    /// LS0tLS1CRUdJTiBFQyBQUklWQVRFIEtFWS0tLS0tCk1IY0NBUUVFSUk3NVk1WkE1WmljVlo5NDMvOUs3emc5RTBD
    /// N2NXWVVNNjVIWGg5UzhTamlvQW9HQ0NxR1NNNDkKQXdFSG9VUURRZ0FFZytYalg0RE5EU1FaaExhYXdOVGZVWG1D
    /// QTJJSGtFSDlCZWJtS3RjVGYvUk5wRmZKdlNxRQptNVdzV0lNUnl6OWpFMUVRN0hOQnlTbHU3UTNRc2h4OGxRPT0K
    /// LS0tLS1FTkQgRUMgUFJJVkFURSBLRVktLS0tLQo=
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

    /// 307702010104208ef9639640e5989c559f78dfff4aef383d1340bb71661433ae475e1f52f128e2a00a06082a
    /// 8648ce3d030107a1440342000483e5e35f80cd0d241984b69ac0d4df5179820362079041fd05e6e62ad7137f
    /// f44da457c9bd2a849b95ac588311cb3f63135110ec7341c9296eed0dd0b21c7c95
    pub fn sk_der_hex(&self) -> GeorgeResult<String> {
        match self.sk.private_key_to_der() {
            Ok(res) => Ok(Hex::encode(res)),
            Err(err) => Err(err_strs("private_key_to_pem", err)),
        }
    }

    /// MHcCAQEEII75Y5ZA5ZicVZ943/9K7zg9E0C7cWYUM65HXh9S8SjioAoGCCqGSM49AwEHoUQDQgAEg+XjX4DNDSQZ
    /// hLaawNTfUXmCA2IHkEH9BebmKtcTf/RNpFfJvSqEm5WsWIMRyz9jE1EQ7HNBySlu7Q3Qshx8lQ==
    pub fn sk_der_base64(&self) -> GeorgeResult<String> {
        match self.sk.private_key_to_der() {
            Ok(res) => Ok(Base64::encode(res)),
            Err(err) => Err(err_strs("private_key_to_pem", err)),
        }
    }

    /// 0383e5e35f80cd0d241984b69ac0d4df5179820362079041fd05e6e62ad7137ff4
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

    /// A4Pl41+AzQ0kGYS2msDU31F5ggNiB5BB/QXm5irXE3/0
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

    /// -----BEGIN PUBLIC KEY-----
    /// MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEg+XjX4DNDSQZhLaawNTfUXmCA2IH
    /// kEH9BebmKtcTf/RNpFfJvSqEm5WsWIMRyz9jE1EQ7HNBySlu7Q3Qshx8lQ==
    /// -----END PUBLIC KEY-----
    pub fn pk_pem_str(&self) -> GeorgeResult<String> {
        match self.pk.public_key_to_pem() {
            Ok(res) => Strings::from_utf8(res),
            Err(err) => Err(err_strs("public_key_to_pem", err)),
        }
    }

    /// 2d2d2d2d2d424547494e205055424c4943204b45592d2d2d2d2d0a4d466b77457759484b6f5a497a6a3043
    /// 415159494b6f5a497a6a30444151634451674145672b586a5834444e4453515a684c6161774e546655586d
    /// 43413249480a6b4548394265626d4b746354662f524e7046664a765371456d35577357494d52797a396a45
    /// 31455137484e4279536c7537513351736878386c513d3d0a2d2d2d2d2d454e44205055424c4943204b4559
    /// 2d2d2d2d2d0a
    pub fn pk_pem_hex(&self) -> GeorgeResult<String> {
        match self.pk.public_key_to_pem() {
            Ok(res) => Ok(Hex::encode(res)),
            Err(err) => Err(err_strs("public_key_to_pem", err)),
        }
    }

    /// LS0tLS1CRUdJTiBQVUJMSUMgS0VZLS0tLS0KTUZrd0V3WUhLb1pJemowQ0FRWUlLb1pJemowREFRY0RRZ0FFZy
    /// tYalg0RE5EU1FaaExhYXdOVGZVWG1DQTJJSAprRUg5QmVibUt0Y1RmL1JOcEZmSnZTcUVtNVdzV0lNUnl6OWpF
    /// MUVRN0hOQnlTbHU3UTNRc2h4OGxRPT0KLS0tLS1FTkQgUFVCTElDIEtFWS0tLS0tCg==
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

    /// 3059301306072a8648ce3d020106082a8648ce3d0301070342000483e5e35f80cd0d241984b69ac0d4df5
    /// 179820362079041fd05e6e62ad7137ff44da457c9bd2a849b95ac588311cb3f63135110ec7341c9296eed
    /// 0dd0b21c7c95
    pub fn pk_der_hex(&self) -> GeorgeResult<String> {
        match self.pk.public_key_to_der() {
            Ok(res) => Ok(Hex::encode(res)),
            Err(err) => Err(err_strs("public_key_to_der", err)),
        }
    }

    /// MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEg+XjX4DNDSQZhLaawNTfUXmCA2IHkEH9BebmKtcTf/RNpFfJvS
    /// qEm5WsWIMRyz9jE1EQ7HNBySlu7Q3Qshx8lQ==
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

/// store method
impl ECDSA {
    pub fn store_hex<P: AsRef<Path>>(&self, sk_filepath: P, pk_filepath: P) -> GeorgeResult<()> {
        let sk_content = self.sk_hex();
        let pk_content = self.pk_hex()?;
        let _ = Filer::write_force(sk_filepath, sk_content)?;
        let _ = Filer::write_force(pk_filepath, pk_content)?;
        Ok(())
    }

    pub fn store_base64<P: AsRef<Path>>(&self, sk_filepath: P, pk_filepath: P) -> GeorgeResult<()> {
        let sk_content = self.sk_base64();
        let pk_content = self.pk_base64()?;
        let _ = Filer::write_force(sk_filepath, sk_content)?;
        let _ = Filer::write_force(pk_filepath, pk_content)?;
        Ok(())
    }

    pub fn store_pem_str<P: AsRef<Path>>(
        &self,
        sk_filepath: P,
        pk_filepath: P,
    ) -> GeorgeResult<()> {
        let sk_content = self.sk_pem_str()?;
        let pk_content = self.pk_pem_str()?;
        let _ = Filer::write_force(sk_filepath, sk_content)?;
        let _ = Filer::write_force(pk_filepath, pk_content)?;
        Ok(())
    }

    pub fn store_pem_hex<P: AsRef<Path>>(
        &self,
        sk_filepath: P,
        pk_filepath: P,
    ) -> GeorgeResult<()> {
        let sk_content = self.sk_pem_hex()?;
        let pk_content = self.pk_pem_hex()?;
        let _ = Filer::write_force(sk_filepath, sk_content)?;
        let _ = Filer::write_force(pk_filepath, pk_content)?;
        Ok(())
    }

    pub fn store_pem_base64<P: AsRef<Path>>(
        &self,
        sk_filepath: P,
        pk_filepath: P,
    ) -> GeorgeResult<()> {
        let sk_content = self.sk_pem_base64()?;
        let pk_content = self.pk_pem_base64()?;
        let _ = Filer::write_force(sk_filepath, sk_content)?;
        let _ = Filer::write_force(pk_filepath, pk_content)?;
        Ok(())
    }

    pub fn store_der_hex<P: AsRef<Path>>(
        &self,
        sk_filepath: P,
        pk_filepath: P,
    ) -> GeorgeResult<()> {
        let sk_content = self.sk_der_hex()?;
        let pk_content = self.pk_der_hex()?;
        let _ = Filer::write_force(sk_filepath, sk_content)?;
        let _ = Filer::write_force(pk_filepath, pk_content)?;
        Ok(())
    }

    pub fn store_der_base64<P: AsRef<Path>>(
        &self,
        sk_filepath: P,
        pk_filepath: P,
    ) -> GeorgeResult<()> {
        let sk_content = self.sk_der_base64()?;
        let pk_content = self.pk_der_base64()?;
        let _ = Filer::write_force(sk_filepath, sk_content)?;
        let _ = Filer::write_force(pk_filepath, pk_content)?;
        Ok(())
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

/// 读取文件字节数组
fn load_bytes_file<P: AsRef<Path>>(filepath: P) -> GeorgeResult<Vec<u8>> {
    match read(filepath) {
        Ok(v8s) => Ok(v8s),
        Err(err) => Err(err_strs("read", err)),
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
