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

/// 创建长度为len且字节均为0x00的字节数组
pub fn create_empty_bytes(len: usize) -> Vec<u8> {
    let mut res: Vec<u8> = vec![];
    let mut position = 0;
    while position < len {
        res.push(0x00);
        position += 1
    }
    res
}
