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

use crate::cryptos::Base64;
use crate::errors::{Errs, GeorgeResult};
use openssl::base64::{decode_block, encode_block};

pub trait Base64Encoder<T> {
    fn encode(bytes: T) -> String;
}

pub trait Base64Decoder<T> {
    fn decode(src: T) -> GeorgeResult<Vec<u8>>;
}

impl Base64Encoder<&[u8]> for Base64 {
    fn encode(bytes: &[u8]) -> String {
        encode_block(bytes)
    }
}

impl Base64Encoder<Vec<u8>> for Base64 {
    fn encode(bytes: Vec<u8>) -> String {
        encode_block(bytes.as_slice())
    }
}

impl Base64Decoder<&str> for Base64 {
    fn decode(src: &str) -> GeorgeResult<Vec<u8>> {
        match decode_block(src) {
            Ok(res) => Ok(res),
            Err(err) => Err(Errs::strs("base64 decode", err)),
        }
    }
}

impl Base64Decoder<String> for Base64 {
    fn decode(src: String) -> GeorgeResult<Vec<u8>> {
        match decode_block(src.as_str()) {
            Ok(res) => Ok(res),
            Err(err) => Err(Errs::strs("base64 decode", err)),
        }
    }
}
