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

use comm::cryptos::Cert;

use crate::ca::identity::CryptoType;

mod identity;
mod intermediate;
mod root;
mod user;

/// 证书等持有者的密钥属性
pub struct Identity {
    /// 密钥类型
    crypto_type: CryptoType,
    /// 私钥
    sk: Vec<u8>,
    /// x509证书
    cert: Cert,
}
