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

use crate::ca::Identity;
use comm::cryptos::{Cert, ECDSA, RSA};
use comm::errors::{Errs, GeorgeResult};

/// 密钥类型
#[derive(Debug, Clone, Copy)]
pub enum CryptoType {
    /// Rivest-Shamir-Adleman密码系统
    /// RSA是最早的非对称公钥加密方案之一。
    /// 和许多其他密码系统一样，RSA依赖于一个复杂数学问题的假定难度，即两个大素数的乘积的因数分解。
    /// 目前还没有一种算法可以在合理的时间内分解这么大的数字。
    /// RSA广泛应用于各种应用，包括数字签名和密钥交换，如建立TLS/SSL连接。
    /// RSA的首字母缩写来源于该算法的三位创始人姓氏的首字母
    RSA,
    /// 椭圆曲线
    /// 密码学依赖于解决数学问题的难度，例如由两个大素数组成的大整数的因子和随机椭圆曲线的离散对数。
    /// 椭圆曲线协议可以用更小的密钥提供同样的安全性。
    EC,
}

impl Identity {
    pub fn from(sk_bytes: Vec<u8>, cert_bytes: Vec<u8>) -> GeorgeResult<Self> {
        from(sk_bytes, cert_bytes)
    }
}

impl Identity {
    pub fn crypto_type(&self) -> CryptoType {
        self.crypto_type.clone()
    }
}

fn from(sk_bytes: Vec<u8>, cert_bytes: Vec<u8>) -> GeorgeResult<Identity> {
    let cert: Cert;
    let crypto_type: CryptoType;
    match Cert::load_pem(cert_bytes.clone()) {
        Ok(res) => cert = res,
        Err(_) => match Cert::load_der(cert_bytes) {
            Ok(res) => cert = res,
            Err(_) => return Err(Errs::str("cert can not load by pem&der!")),
        },
    }
    match RSA::from_bytes(sk_bytes.clone()) {
        Ok(res) => match Cert::verify(res.sk(), cert.x509.clone()) {
            Ok(res) => {
                if !res {
                    return Err(Errs::str("sk and cert can not match!"));
                }
                crypto_type = CryptoType::RSA
            }
            Err(_) => return Err(Errs::str("sk can not verify by cert!")),
        },
        Err(_) => match ECDSA::from_sk_bytes(sk_bytes.clone()) {
            Ok(res) => match Cert::verify(res.sk(), cert.x509.clone()) {
                Ok(res) => {
                    if !res {
                        return Err(Errs::str("sk and cert can not match!"));
                    }
                    crypto_type = CryptoType::EC
                }
                Err(_) => return Err(Errs::str("sk can not verify by cert!")),
            },
            Err(_) => return Err(Errs::str("sk can not load by rsa&ec!")),
        },
    }
    Ok(Identity {
        crypto_type,
        sk: sk_bytes,
        cert,
    })
}
