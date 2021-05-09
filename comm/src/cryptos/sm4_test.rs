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
mod sm4_test_all {
    #[cfg(test)]
    mod sm4_1 {
        use crate::cryptos::sm4::{SM4Crypt, SM4Handler, SM4};

        #[test]
        fn sm4_test1() {
            let key = SM4::rand_block();
            let iv = SM4::rand_block();
            let res = SM4::encrypt(key, iv, "test".as_bytes());
            let d_res = SM4::decrypt(key, iv, res.as_slice());
            println!("d_res = {}", String::from_utf8(d_res).unwrap());
        }
    }

    #[cfg(test)]
    mod sm4_2 {
        use crate::cryptos::sm4::{SM4CryptMode, SM4Handler, SM4};
        use libsm::sm4::cipher_mode::CipherMode;

        #[test]
        fn sm4_test2() {
            let key = SM4::rand_block();
            let iv = SM4::rand_block();
            let res = SM4::encrypt(key, iv, "test".as_bytes(), CipherMode::Cfb);
            let d_res = SM4::decrypt(key, iv, res.as_slice(), CipherMode::Cfb);
            println!("d_res = {}", String::from_utf8(d_res).unwrap());
        }
    }

    #[cfg(test)]
    mod sm4_3 {
        use crate::cryptos::sm4::{SM4New1, SM4SelfCrypt1, SM4};

        #[test]
        fn sm4_test1() {
            let sm4 = SM4::new();
            let res = sm4.encrypt("test".as_bytes());
            let d_res = sm4.decrypt(res.as_slice());
            println!("d_res = {}", String::from_utf8(d_res).unwrap());
        }
    }

    #[cfg(test)]
    mod sm4_4 {
        use crate::cryptos::sm4::{SM4Handler, SM4New1, SM4SelfCrypt2, SM4};

        #[test]
        fn sm4_test1() {
            let sm4 = SM4::new();
            let iv = SM4::rand_block();
            let res = sm4.encrypt(&iv, "test".as_bytes());
            let d_res = sm4.decrypt(&iv, res.as_slice());
            println!("d_res = {}", String::from_utf8(d_res).unwrap());
        }
    }

    #[cfg(test)]
    mod sm4_5 {
        use crate::cryptos::sm4::{SM4New2, SM4SelfCrypt1, SM4};
        use libsm::sm4::cipher_mode::CipherMode;

        #[test]
        fn sm4_test1() {
            let sm4 = SM4::new(CipherMode::Cfb);
            let res = sm4.encrypt("test".as_bytes());
            let d_res = sm4.decrypt(res.as_slice());
            println!("d_res = {}", String::from_utf8(d_res).unwrap());
        }

        #[test]
        fn sm4_test2() {
            let sm4 = SM4::new(CipherMode::Ctr);
            let res = sm4.encrypt("test".as_bytes());
            let d_res = sm4.decrypt(res.as_slice());
            println!("d_res = {}", String::from_utf8(d_res).unwrap());
        }

        #[test]
        fn sm4_test3() {
            let sm4 = SM4::new(CipherMode::Ofb);
            let res = sm4.encrypt("test".as_bytes());
            let d_res = sm4.decrypt(res.as_slice());
            println!("d_res = {}", String::from_utf8(d_res).unwrap());
        }
    }

    #[cfg(test)]
    mod sm4_6 {
        use crate::cryptos::sm4::{SM4Handler, SM4New2, SM4SelfCrypt2, SM4};
        use libsm::sm4::cipher_mode::CipherMode;

        #[test]
        fn sm4_test1() {
            let sm4 = SM4::new(CipherMode::Cfb);
            let iv = SM4::rand_block();
            let res = sm4.encrypt(&iv, "test".as_bytes());
            let d_res = sm4.decrypt(&iv, res.as_slice());
            println!("d_res = {}", String::from_utf8(d_res).unwrap());
        }

        #[test]
        fn sm4_test2() {
            let sm4 = SM4::new(CipherMode::Ctr);
            let iv = SM4::rand_block();
            let res = sm4.encrypt(&iv, "test".as_bytes());
            let d_res = sm4.decrypt(&iv, res.as_slice());
            println!("d_res = {}", String::from_utf8(d_res).unwrap());
        }

        #[test]
        fn sm4_test3() {
            let sm4 = SM4::new(CipherMode::Ofb);
            let iv = SM4::rand_block();
            let res = sm4.encrypt(&iv, "test".as_bytes());
            let d_res = sm4.decrypt(&iv, res.as_slice());
            println!("d_res = {}", String::from_utf8(d_res).unwrap());
        }
    }

    #[cfg(test)]
    mod sm4_7 {
        use crate::cryptos::sm4::{SM4Handler, SM4New3, SM4SelfCrypt1, SM4};
        use libsm::sm4::cipher_mode::CipherMode;

        #[test]
        fn sm4_test1() {
            let key = SM4::rand_block();
            let sm4 = SM4::new(key, CipherMode::Cfb);
            let res = sm4.encrypt("test".as_bytes());
            let d_res = sm4.decrypt(res.as_slice());
            println!("d_res = {}", String::from_utf8(d_res).unwrap());
        }

        #[test]
        fn sm4_test2() {
            let key = SM4::rand_block();
            let sm4 = SM4::new(key, CipherMode::Ctr);
            let res = sm4.encrypt("test".as_bytes());
            let d_res = sm4.decrypt(res.as_slice());
            println!("d_res = {}", String::from_utf8(d_res).unwrap());
        }

        #[test]
        fn sm4_test3() {
            let key = SM4::rand_block();
            let sm4 = SM4::new(key, CipherMode::Ofb);
            let res = sm4.encrypt("test".as_bytes());
            let d_res = sm4.decrypt(res.as_slice());
            println!("d_res = {}", String::from_utf8(d_res).unwrap());
        }
    }

    #[cfg(test)]
    mod sm4_8 {
        use crate::cryptos::sm4::{SM4Handler, SM4New3, SM4SelfCrypt2, SM4};
        use libsm::sm4::cipher_mode::CipherMode;

        #[test]
        fn sm4_test1() {
            let key = SM4::rand_block();
            let sm4 = SM4::new(key, CipherMode::Cfb);
            let iv = SM4::rand_block();
            let res = sm4.encrypt(&iv, "test".as_bytes());
            let d_res = sm4.decrypt(&iv, res.as_slice());
            println!("d_res = {}", String::from_utf8(d_res).unwrap());
        }

        #[test]
        fn sm4_test2() {
            let key = SM4::rand_block();
            let sm4 = SM4::new(key, CipherMode::Ctr);
            let iv = SM4::rand_block();
            let res = sm4.encrypt(&iv, "test".as_bytes());
            let d_res = sm4.decrypt(&iv, res.as_slice());
            println!("d_res = {}", String::from_utf8(d_res).unwrap());
        }

        #[test]
        fn sm4_test3() {
            let key = SM4::rand_block();
            let sm4 = SM4::new(key, CipherMode::Ofb);
            let iv = SM4::rand_block();
            let res = sm4.encrypt(&iv, "test".as_bytes());
            let d_res = sm4.decrypt(&iv, res.as_slice());
            println!("d_res = {}", String::from_utf8(d_res).unwrap());
        }
    }

    #[cfg(test)]
    mod sm4_9 {
        use crate::cryptos::sm4::{SM4Handler, SM4New4, SM4SelfCrypt1, SM4};
        use libsm::sm4::cipher_mode::CipherMode;

        #[test]
        fn sm4_test1() {
            let key = SM4::rand_block();
            let iv = SM4::rand_block();
            let sm4 = SM4::new(key, iv, CipherMode::Cfb);
            let res = sm4.encrypt("test".as_bytes());
            let d_res = sm4.decrypt(res.as_slice());
            println!("d_res = {}", String::from_utf8(d_res).unwrap());
        }

        #[test]
        fn sm4_test2() {
            let key = SM4::rand_block();
            let iv = SM4::rand_block();
            let sm4 = SM4::new(key, iv, CipherMode::Ctr);
            let res = sm4.encrypt("test".as_bytes());
            let d_res = sm4.decrypt(res.as_slice());
            println!("d_res = {}", String::from_utf8(d_res).unwrap());
        }

        #[test]
        fn sm4_test3() {
            let key = SM4::rand_block();
            let iv = SM4::rand_block();
            let sm4 = SM4::new(key, iv, CipherMode::Ofb);
            let res = sm4.encrypt("test".as_bytes());
            let d_res = sm4.decrypt(res.as_slice());
            println!("d_res = {}", String::from_utf8(d_res).unwrap());
        }
    }

    #[cfg(test)]
    mod sm4_10 {
        use crate::cryptos::sm4::{SM4Handler, SM4New4, SM4SelfCrypt2, SM4};
        use libsm::sm4::cipher_mode::CipherMode;

        #[test]
        fn sm4_test1() {
            let key = SM4::rand_block();
            let iv = SM4::rand_block();
            let sm4 = SM4::new(key, iv, CipherMode::Cfb);
            let res = sm4.encrypt(&iv, "test".as_bytes());
            let d_res = sm4.decrypt(&iv, res.as_slice());
            println!("d_res = {}", String::from_utf8(d_res).unwrap());
        }

        #[test]
        fn sm4_test2() {
            let key = SM4::rand_block();
            let iv = SM4::rand_block();
            let sm4 = SM4::new(key, iv, CipherMode::Ctr);
            let res = sm4.encrypt(&iv, "test".as_bytes());
            let d_res = sm4.decrypt(&iv, res.as_slice());
            println!("d_res = {}", String::from_utf8(d_res).unwrap());
        }

        #[test]
        fn sm4_test3() {
            let key = SM4::rand_block();
            let iv = SM4::rand_block();
            let sm4 = SM4::new(key, iv, CipherMode::Ofb);
            let res = sm4.encrypt(&iv, "test".as_bytes());
            let d_res = sm4.decrypt(&iv, res.as_slice());
            println!("d_res = {}", String::from_utf8(d_res).unwrap());
        }
    }
}
