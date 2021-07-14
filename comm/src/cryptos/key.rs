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

use std::path::Path;

use openssl::pkey::{PKey, Private, Public};

use crate::cryptos::{Key, ECDSA, RSA};
use crate::errors::{Errs, GeorgeResult};
use crate::io::file::FilerReader;
use crate::io::Filer;

impl Key {
    pub fn load_sk<P: AsRef<Path>>(filepath: P) -> GeorgeResult<PKey<Private>> {
        let bytes = Filer::read_bytes(filepath)?;
        Key::load_sk_bytes(bytes)
    }

    pub fn load_sk_bytes(bytes: Vec<u8>) -> GeorgeResult<PKey<Private>> {
        match RSA::from_bytes(bytes.clone()) {
            Ok(rsa) => Ok(rsa.sk()),
            Err(_) => match ECDSA::from_sk_bytes(bytes) {
                Ok(ecdsa) => Ok(ecdsa.sk()),
                Err(_) => return Err(Errs::str("key is not match any rsa or ec!")),
            },
        }
    }

    pub fn load_pk<P: AsRef<Path>>(filepath: P) -> GeorgeResult<PKey<Public>> {
        let bytes = Filer::read_bytes(filepath)?;
        Key::load_pk_bytes(bytes)
    }

    pub fn load_pk_bytes(bytes: Vec<u8>) -> GeorgeResult<PKey<Public>> {
        match RSA::from_bytes(bytes.clone()) {
            Ok(rsa) => Ok(rsa.pk()),
            Err(_) => match ECDSA::from_sk_bytes(bytes) {
                Ok(ecdsa) => Ok(ecdsa.pk()),
                Err(_) => return Err(Errs::str("key is not match any rsa or ec!")),
            },
        }
    }
}
