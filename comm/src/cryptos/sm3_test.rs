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

#[cfg(test)]
mod sm3 {
    use crate::cryptos::sm3;

    #[test]
    fn sm3_test() {
        let str = "test".to_string();
        let sm3_1 = sm3::hash(str.clone());
        let sm3_2 = sm3::hash(str.clone());
        println!("test sm3 1 = {}", sm3_1);
        println!("test sm3 2 = {}", sm3_2);
        match hex::decode(sm3_1) {
            Ok(v8) => println!("u8s = {:#?}", v8.as_slice()),
            Err(err) => println!("err = {}", err.to_string()),
        }
    }
}
