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

use libsm::sm3::hash::Sm3Hash;

#[derive(Debug, Clone)]
pub struct SM3;

pub trait SM3Handler<T> {
    fn hash(comment: T) -> String;
}

impl SM3Handler<&[u8]> for SM3 {
    fn hash(comment: &[u8]) -> String {
        hash(comment)
    }
}

impl SM3Handler<Vec<u8>> for SM3 {
    fn hash(comment: Vec<u8>) -> String {
        hash_v8s(comment)
    }
}

impl SM3Handler<String> for SM3 {
    fn hash(comment: String) -> String {
        hash_string(comment)
    }
}

/// 国密消息摘要。可以用MD5作为对比理解。该算法已公开。校验结果为256位
fn hash(comment: &[u8]) -> String {
    let mut hash = Sm3Hash::new(comment);
    let digest: [u8; 32] = hash.get_hash();
    // println!("digest = {:#?}", digest);
    hex::encode(digest)
}

/// 国密消息摘要。可以用MD5作为对比理解。该算法已公开。校验结果为256位
fn hash_v8s(comment: Vec<u8>) -> String {
    let mut hash = Sm3Hash::new(comment.as_slice());
    let digest: [u8; 32] = hash.get_hash();
    // println!("digest = {:#?}", digest);
    hex::encode(digest)
}

/// 国密消息摘要。可以用MD5作为对比理解。该算法已公开。校验结果为256位
fn hash_string(comment: String) -> String {
    let mut hash = Sm3Hash::new(comment.as_bytes());
    let digest: [u8; 32] = hash.get_hash();
    // println!("digest = {:#?}", digest);
    hex::encode(digest)
}
