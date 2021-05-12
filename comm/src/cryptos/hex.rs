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

use crate::errors::entrances::{err_strs, GeorgeResult};

#[derive(Debug, Clone)]
pub struct Hex;

pub trait HexEncoder<T> {
    fn encode(bytes: T) -> String;
}

pub trait HexDecoder<T> {
    fn decode(src: T) -> GeorgeResult<Vec<u8>>;
}

impl HexEncoder<&[u8]> for Hex {
    fn encode(bytes: &[u8]) -> String {
        hex::encode(bytes)
    }
}

impl HexEncoder<Vec<u8>> for Hex {
    fn encode(bytes: Vec<u8>) -> String {
        hex::encode(bytes.as_slice())
    }
}

impl HexDecoder<&str> for Hex {
    fn decode(src: &str) -> GeorgeResult<Vec<u8>> {
        match hex::decode(src) {
            Ok(res) => Ok(res),
            Err(err) => Err(err_strs("base64 decode", err)),
        }
    }
}

impl HexDecoder<String> for Hex {
    fn decode(src: String) -> GeorgeResult<Vec<u8>> {
        match hex::decode(src.as_str()) {
            Ok(res) => Ok(res),
            Err(err) => Err(err_strs("base64 decode", err)),
        }
    }
}
