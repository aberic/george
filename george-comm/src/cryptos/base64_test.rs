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
mod base64 {
    use crate::cryptos::base64::{Base64Decoder, Base64Encoder};
    use crate::cryptos::Base64;

    #[test]
    fn base64_test() {
        let src = "hello world!".as_bytes();
        let ber = Base64::encode(src);
        let her = hex::encode(src);
        println!("ber = {}\nhex = {}", ber, her);
        let bdr = Base64::decode(ber).unwrap();
        assert_eq!(src, bdr.as_slice());

        let ber = Base64::encode(src.to_vec());
        let her = hex::encode(src);
        println!("ber = {}\nhex = {}", ber, her);
        let bdr = Base64::decode(ber).unwrap();
        assert_eq!(src, bdr.as_slice());

        let ber = Base64::encode(src.to_vec());
        let her = hex::encode(src);
        println!("ber = {}\nhex = {}", ber, her);
        let bdr = Base64::decode(ber.as_str()).unwrap();
        assert_eq!(src, bdr.as_slice());
    }
}
