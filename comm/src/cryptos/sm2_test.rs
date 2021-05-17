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
mod sm2 {
    #[cfg(test)]
    mod sm {

        use crate::cryptos::base64::{Base64, Base64Encoder};
        use crate::cryptos::hex::{Hex, HexEncoder};
        use crate::cryptos::sm2::{SM2LoadKey, SM2Store, SM2};

        #[test]
        fn test() {
            let sm2 = SM2::new();
            println!("sk0 base64 = {}", Base64::encode(sm2.sk_bytes()));
            println!("pk0 base64 = {}", Base64::encode(sm2.pk_bytes()));
            println!("sk0 hex = {}", Hex::encode(sm2.sk_bytes()));
            println!("pk0 hex = {}", Hex::encode(sm2.pk_bytes()));
            let pk1 = sm2.new_pk();
            println!("pk1 base64 = {}", Base64::encode(pk1.clone()));
            println!("pk1 hex = {}", Hex::encode(pk1.clone()));

            let res1 = "hello world!".as_bytes();
            let sign_res = sm2.sig(res1);
            println!(
                "verify = {}",
                sm2.verifies(res1, sign_res.as_slice()).unwrap()
            );
            let res2 = "hello world".as_bytes();
            println!(
                "verify = {}",
                sm2.verifies(res2, sign_res.as_slice()).unwrap()
            );

            sm2.store(
                "src/test/crypto/sm2/self/generate1_sk",
                "src/test/crypto/sm2/self/generate1_pk",
            )
            .unwrap();

            let sm2_new = SM2::load(
                "src/test/crypto/sm2/self/generate1_sk",
                "src/test/crypto/sm2/self/generate1_pk",
            )
            .unwrap();
            println!(
                "verify = {}",
                sm2_new.verifies(res1, sign_res.as_slice()).unwrap()
            );
        }
    }

    #[cfg(test)]
    mod generate {

        use crate::cryptos::base64::{Base64, Base64Encoder};
        use crate::cryptos::sm2::{SM2New, SM2};

        #[test]
        fn test() {
            let (sk, pk) = SM2::generate();
            println!("sk = {}\npk = {}", Base64::encode(sk), Base64::encode(pk));
            let (sk, pk) = SM2::generate();
            println!("sk = {}\npk = {}", Base64::encode(sk), Base64::encode(pk));
            let (sk, pk) = SM2::generate_base64();
            println!("sk = {}\npk = {}", sk, pk);
            let (sk, pk) = SM2::generate_base64();
            println!("sk = {}\npk = {}", sk, pk);
        }
    }

    #[cfg(test)]
    mod generate_sk {

        use crate::cryptos::base64::{Base64, Base64Encoder};
        use crate::cryptos::sm2::{SM2SkNew, SM2};

        #[test]
        fn test() {
            let sk = SM2::generate();
            println!("sk = {}", Base64::encode(sk));
            let sk = SM2::generate();
            println!("sk = {}", Base64::encode(sk));
            let sk = SM2::generate_base64();
            println!("sk = {}", sk);
            let sk = SM2::generate_base64();
            println!("sk = {}", sk);
        }
    }

    #[cfg(test)]
    mod generate_sk_file {

        use crate::cryptos::base64::{Base64, Base64Encoder};
        use crate::cryptos::sm2::{SM2SkNewStore, SM2};

        #[test]
        fn test() {
            let path1 = "src/test/crypto/sm2/generate_sk_file/generate1_sk";
            let path2 = "src/test/crypto/sm2/generate_sk_file/generate2_sk";
            let path3 = "src/test/crypto/sm2/generate_sk_file/generate3_sk";
            let path4 = "src/test/crypto/sm2/generate_sk_file/generate4_sk";
            let sk = SM2::generate(path1).unwrap();
            println!("sk = {}", Base64::encode(sk));
            let sk = SM2::generate(path2).unwrap();
            println!("sk = {}", Base64::encode(sk));
            let sk = SM2::generate_base64(path3).unwrap();
            println!("sk = {}", sk);
            let sk = SM2::generate_base64(path4).unwrap();
            println!("sk = {}", sk);
        }
    }

    #[cfg(test)]
    mod generate_file {

        use crate::cryptos::base64::{Base64, Base64Encoder};
        use crate::cryptos::sm2::{SM2NewStore, SM2};

        #[test]
        fn test1() {
            let (sk, pk) = SM2::generate(
                "src/test/crypto/sm2/generate_file/generate1_sk",
                "src/test/crypto/sm2/generate_file/generate1_pk",
            )
            .unwrap();
            println!("sk = {}\npk = {}", Base64::encode(sk), Base64::encode(pk));
            let (sk, pk) = SM2::generate_base64(
                "src/test/crypto/sm2/generate_file/generate2_sk",
                "src/test/crypto/sm2/generate_file/generate2_pk",
            )
            .unwrap();
            println!("sk = {}\npk = {}", sk, pk);
        }

        #[test]
        fn test2() {
            let (sk, pk) = SM2::generate(
                "src/test/crypto/sm2/generate_file/generate3_sk".to_string(),
                "src/test/crypto/sm2/generate_file/generate3_pk".to_string(),
            )
            .unwrap();
            println!("sk = {}\npk = {}", Base64::encode(sk), Base64::encode(pk));
            let (sk, pk) = SM2::generate_base64(
                "src/test/crypto/sm2/generate_file/generate4_sk".to_string(),
                "src/test/crypto/sm2/generate_file/generate4_pk".to_string(),
            )
            .unwrap();
            println!("sk = {}\npk = {}", sk, pk);
        }
    }

    #[cfg(test)]
    mod generate_pk_v8s {

        use crate::cryptos::base64::{Base64, Base64Encoder};
        use crate::cryptos::sm2::{SM2New, SM2Pk, SM2};

        #[test]
        fn generate_pk_test() {
            let (sk, pk) = SM2::generate();
            let pk_new = SM2::generate_pk(sk.clone()).unwrap();
            println!(
                "sk = {}\npk = {}\nne = {}",
                Base64::encode(sk),
                Base64::encode(pk),
                Base64::encode(pk_new)
            );

            let (sk, pk) = SM2::generate_base64();
            let pk_new = SM2::generate_pk_by_base64(sk.clone()).unwrap();
            println!("sk = {}\npk = {}\nne = {}", sk, pk, Base64::encode(pk_new));
        }
    }

    #[cfg(test)]
    mod generate_pk_string {

        use crate::cryptos::base64::{Base64, Base64Encoder};
        use crate::cryptos::sm2::{SM2New, SM2Pk, SM2};

        #[test]
        fn generate_pk_test() {
            let (sk, pk) = SM2::generate();
            let pk_new = SM2::generate_pk(sk.clone()).unwrap();
            println!(
                "sk = {}\npk = {}\nne = {}",
                Base64::encode(sk),
                Base64::encode(pk),
                Base64::encode(pk_new)
            );

            let (sk, pk) = SM2::generate_base64();
            let pk_new = SM2::generate_pk_by_base64(sk.clone()).unwrap();
            println!("sk = {}\npk = {}\nne = {}", sk, pk, Base64::encode(pk_new));
        }
    }

    #[cfg(test)]
    mod generate_pk_v8s_path {

        use crate::cryptos::base64::{Base64, Base64Encoder};
        use crate::cryptos::sm2::{SM2NewStore, SM2Pk, SM2};

        #[test]
        fn generate_pk_test() {
            let (_, pk) = SM2::generate(
                "src/test/crypto/sm2/generate_pk_file/generate1_sk",
                "src/test/crypto/sm2/generate_pk_file/generate1_pk",
            )
            .unwrap();
            let pk_new = SM2::generate_pk_by_base64_file(
                "src/test/crypto/sm2/generate_pk_file/generate1_sk".to_string(),
            )
            .unwrap();
            println!(
                "pk = {}\nne = {}",
                Base64::encode(pk),
                Base64::encode(pk_new)
            );
        }
    }

    #[cfg(test)]
    mod generate_pk_string_path {

        use crate::cryptos::base64::{Base64, Base64Encoder};
        use crate::cryptos::sm2::{SM2NewStore, SM2Pk, SM2};

        #[test]
        fn generate_pk_test() {
            let (_, pk) = SM2::generate_base64(
                "src/test/crypto/sm2/generate_pk_file/generate2_sk",
                "src/test/crypto/sm2/generate_pk_file/generate2_pk",
            )
            .unwrap();
            let pk_new = SM2::generate_pk_by_base64_file(
                "src/test/crypto/sm2/generate_pk_file/generate2_sk".to_string(),
            )
            .unwrap();
            println!("pk = {}\nne = {}", pk, Base64::encode(pk_new));
        }
    }

    #[cfg(test)]
    mod sign {

        use crate::cryptos::base64::{Base64, Base64Encoder};
        use crate::cryptos::sm2::{SM2NewStore, SM2Sign, SM2Verify, SM2};

        #[test]
        fn test_u8s() {
            let (sk, pk) = SM2::generate(
                "src/test/crypto/sm2/sign/generate1_sk",
                "src/test/crypto/sm2/sign/generate1_pk",
            )
            .unwrap();
            let msg1 = "hello 你好！?";
            let pk_string = Base64::encode(pk.clone());
            let pk_str = pk_string.as_str();

            /////////////// sk/pk u8s start ///////////////
            let sign_res1 = SM2::sign(msg1, sk.as_slice(), pk.as_slice()).unwrap();
            let sign_res2 = SM2::sign_base64(msg1, sk.as_slice(), pk.as_slice()).unwrap();
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                Base64::encode(sign_res1.clone()),
                sign_res2
            );

            let sign_res1 = SM2::sign(msg1.to_string(), sk.as_slice(), pk.as_slice()).unwrap();
            let sign_res2 =
                SM2::sign_base64(msg1.to_string(), sk.as_slice(), pk.as_slice()).unwrap();
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                Base64::encode(sign_res1),
                sign_res2
            );

            let sign_res1 = SM2::sign(msg1.as_bytes(), sk.as_slice(), pk.as_slice()).unwrap();
            let sign_res2 =
                SM2::sign_base64(msg1.as_bytes(), sk.as_slice(), pk.as_slice()).unwrap();
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                Base64::encode(sign_res1),
                sign_res2
            );

            let sign_res1 =
                SM2::sign(msg1.as_bytes().to_vec(), sk.as_slice(), pk.as_slice()).unwrap();
            let sign_res2 =
                SM2::sign_base64(msg1.as_bytes().to_vec(), sk.as_slice(), pk.as_slice()).unwrap();
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                Base64::encode(sign_res1.clone()),
                sign_res2
            );
            /////////////// sk/pk u8s end ///////////////
            println!(
                "verify = {}",
                SM2::verify(msg1, pk.as_slice(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_str.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_string.clone(), sign_res1.clone()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1, pk.as_slice(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_str.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_string.clone(), sign_res1.as_slice()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1, pk.as_slice(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_str.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_string.clone(), sign_res2.clone()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1, pk.as_slice(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_str.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_string.clone(), sign_res2.as_str()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.as_slice(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_str.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_string.clone(), sign_res1.clone()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.as_slice(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_str.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_string.clone(), sign_res1.as_slice()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.as_slice(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_str.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_string.clone(), sign_res2.clone()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.as_slice(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_str.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_string.clone(), sign_res2.as_str()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.as_slice(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_str.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_string.clone(), sign_res1.clone()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.as_slice(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_str.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_string.clone(), sign_res1.as_slice()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.as_slice(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_str.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_string.clone(), sign_res2.clone()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.as_slice(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_str.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_string.clone(), sign_res2.as_str()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.as_slice(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk_str.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(
                    msg1.as_bytes().to_vec(),
                    pk_string.clone(),
                    sign_res1.clone()
                )
                .unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(
                    msg1.as_bytes().to_vec(),
                    pk.as_slice(),
                    sign_res1.as_slice()
                )
                .unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(
                    msg1.as_bytes().to_vec(),
                    pk_str.clone(),
                    sign_res1.as_slice()
                )
                .unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(
                    msg1.as_bytes().to_vec(),
                    pk_string.clone(),
                    sign_res1.as_slice()
                )
                .unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.as_slice(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk_str.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(
                    msg1.as_bytes().to_vec(),
                    pk_string.clone(),
                    sign_res2.clone()
                )
                .unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.as_slice(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk_str.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(
                    msg1.as_bytes().to_vec(),
                    pk_string.clone(),
                    sign_res2.as_str()
                )
                .unwrap()
            );
        }

        #[test]
        fn test_v8s() {
            let (sk, pk) = SM2::generate(
                "src/test/crypto/sm2/sign/generate2_sk",
                "src/test/crypto/sm2/sign/generate2_pk",
            )
            .unwrap();
            let msg1 = "hello 你好！?";
            let pk_string = Base64::encode(pk.clone());
            let pk_str = pk_string.as_str();

            /////////////// sk/pk v8s start ///////////////
            let sign_res1 = SM2::sign(msg1, sk.clone(), pk.clone()).unwrap();
            let sign_res2 = SM2::sign_base64(msg1, sk.clone(), pk.clone()).unwrap();
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                Base64::encode(sign_res1),
                sign_res2
            );

            let sign_res1 = SM2::sign(msg1.to_string(), sk.clone(), pk.clone()).unwrap();
            let sign_res2 = SM2::sign_base64(msg1.to_string(), sk.clone(), pk.clone()).unwrap();
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                Base64::encode(sign_res1),
                sign_res2
            );

            let sign_res1 = SM2::sign(msg1.as_bytes(), sk.clone(), pk.clone()).unwrap();
            let sign_res2 = SM2::sign_base64(msg1.as_bytes(), sk.clone(), pk.clone()).unwrap();
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                Base64::encode(sign_res1),
                sign_res2
            );

            let sign_res1 = SM2::sign(msg1.as_bytes().to_vec(), sk.clone(), pk.clone()).unwrap();
            let sign_res2 =
                SM2::sign_base64(msg1.as_bytes().to_vec(), sk.clone(), pk.clone()).unwrap();
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                Base64::encode(sign_res1.clone()),
                sign_res2
            );
            /////////////// sk/pk v8s end ///////////////
            println!(
                "verify = {}",
                SM2::verify(msg1, pk.as_slice(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_str.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_string.clone(), sign_res1.clone()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1, pk.as_slice(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_str.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_string.clone(), sign_res1.as_slice()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1, pk.as_slice(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_str.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_string.clone(), sign_res2.clone()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1, pk.as_slice(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_str.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_string.clone(), sign_res2.as_str()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.as_slice(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_str.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_string.clone(), sign_res1.clone()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.as_slice(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_str.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_string.clone(), sign_res1.as_slice()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.as_slice(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_str.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_string.clone(), sign_res2.clone()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.as_slice(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_str.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_string.clone(), sign_res2.as_str()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.as_slice(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_str.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_string.clone(), sign_res1.clone()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.as_slice(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_str.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_string.clone(), sign_res1.as_slice()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.as_slice(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_str.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_string.clone(), sign_res2.clone()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.as_slice(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_str.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_string.clone(), sign_res2.as_str()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.as_slice(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk_str.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(
                    msg1.as_bytes().to_vec(),
                    pk_string.clone(),
                    sign_res1.clone()
                )
                .unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(
                    msg1.as_bytes().to_vec(),
                    pk.as_slice(),
                    sign_res1.as_slice()
                )
                .unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(
                    msg1.as_bytes().to_vec(),
                    pk_str.clone(),
                    sign_res1.as_slice()
                )
                .unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(
                    msg1.as_bytes().to_vec(),
                    pk_string.clone(),
                    sign_res1.as_slice()
                )
                .unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.as_slice(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk_str.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(
                    msg1.as_bytes().to_vec(),
                    pk_string.clone(),
                    sign_res2.clone()
                )
                .unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.as_slice(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk_str.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(
                    msg1.as_bytes().to_vec(),
                    pk_string.clone(),
                    sign_res2.as_str()
                )
                .unwrap()
            );
        }

        #[test]
        fn test_string() {
            let (sk, pk) = SM2::generate(
                "src/test/crypto/sm2/sign/generate3_sk",
                "src/test/crypto/sm2/sign/generate3_pk",
            )
            .unwrap();
            let msg1 = "hello 你好！?";
            let sk_string = Base64::encode(sk.clone());
            let pk_string = Base64::encode(pk.clone());
            let pk_str = pk_string.as_str();

            /////////////// sk/pk string start ///////////////
            let sign_res1 = SM2::sign(msg1.clone(), sk_string.clone(), pk_string.clone()).unwrap();
            let sign_res2 =
                SM2::sign_base64(msg1.clone(), sk_string.clone(), pk_string.clone()).unwrap();
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                Base64::encode(sign_res1),
                sign_res2
            );

            let sign_res1 =
                SM2::sign(msg1.to_string(), sk_string.clone(), pk_string.clone()).unwrap();
            let sign_res2 =
                SM2::sign_base64(msg1.to_string(), sk_string.clone(), pk_string.clone()).unwrap();
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                Base64::encode(sign_res1),
                sign_res2
            );

            let sign_res1 =
                SM2::sign(msg1.as_bytes(), sk_string.clone(), pk_string.clone()).unwrap();
            let sign_res2 =
                SM2::sign_base64(msg1.as_bytes(), sk_string.clone(), pk_string.clone()).unwrap();
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                Base64::encode(sign_res1),
                sign_res2
            );

            let sign_res1 = SM2::sign(
                msg1.as_bytes().to_vec(),
                sk_string.clone(),
                pk_string.clone(),
            )
            .unwrap();
            let sign_res2 = SM2::sign_base64(
                msg1.as_bytes().to_vec(),
                sk_string.clone(),
                pk_string.clone(),
            )
            .unwrap();
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                Base64::encode(sign_res1.clone()),
                sign_res2
            );
            /////////////// sk/pk string end ///////////////
            println!(
                "verify = {}",
                SM2::verify(msg1, pk.as_slice(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_str.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_string.clone(), sign_res1.clone()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1, pk.as_slice(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_str.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_string.clone(), sign_res1.as_slice()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1, pk.as_slice(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_str.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_string.clone(), sign_res2.clone()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1, pk.as_slice(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_str.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_string.clone(), sign_res2.as_str()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.as_slice(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_str.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_string.clone(), sign_res1.clone()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.as_slice(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_str.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_string.clone(), sign_res1.as_slice()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.as_slice(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_str.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_string.clone(), sign_res2.clone()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.as_slice(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_str.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_string.clone(), sign_res2.as_str()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.as_slice(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_str.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_string.clone(), sign_res1.clone()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.as_slice(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_str.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_string.clone(), sign_res1.as_slice()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.as_slice(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_str.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_string.clone(), sign_res2.clone()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.as_slice(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_str.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_string.clone(), sign_res2.as_str()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.as_slice(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk_str.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(
                    msg1.as_bytes().to_vec(),
                    pk_string.clone(),
                    sign_res1.clone()
                )
                .unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(
                    msg1.as_bytes().to_vec(),
                    pk.as_slice(),
                    sign_res1.as_slice()
                )
                .unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(
                    msg1.as_bytes().to_vec(),
                    pk_str.clone(),
                    sign_res1.as_slice()
                )
                .unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(
                    msg1.as_bytes().to_vec(),
                    pk_string.clone(),
                    sign_res1.as_slice()
                )
                .unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.as_slice(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk_str.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(
                    msg1.as_bytes().to_vec(),
                    pk_string.clone(),
                    sign_res2.clone()
                )
                .unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.as_slice(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk_str.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(
                    msg1.as_bytes().to_vec(),
                    pk_string.clone(),
                    sign_res2.as_str()
                )
                .unwrap()
            );
        }

        #[test]
        fn test_str() {
            let (sk, pk) = SM2::generate(
                "src/test/crypto/sm2/sign/generate4_sk",
                "src/test/crypto/sm2/sign/generate4_pk",
            )
            .unwrap();
            let msg1 = "hello 你好！?";
            let sk_string = Base64::encode(sk.clone());
            let pk_string = Base64::encode(pk.clone());
            let sk_str = sk_string.as_str();
            let pk_str = pk_string.as_str();

            /////////////// sk/pk str start ///////////////
            let sign_res1 = SM2::sign(msg1, sk_str, pk_str).unwrap();
            let sign_res2 = SM2::sign_base64(msg1, sk_str.clone(), pk_str.clone()).unwrap();
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                Base64::encode(sign_res1.clone()),
                sign_res2
            );

            let sign_res1 = SM2::sign(msg1.to_string(), sk_str.clone(), pk_str.clone()).unwrap();
            let sign_res2 =
                SM2::sign_base64(msg1.to_string(), sk_str.clone(), pk_str.clone()).unwrap();
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                Base64::encode(sign_res1),
                sign_res2
            );

            let sign_res1 = SM2::sign(msg1.as_bytes(), sk_str.clone(), pk_str.clone()).unwrap();
            let sign_res2 =
                SM2::sign_base64(msg1.as_bytes(), sk_str.clone(), pk_str.clone()).unwrap();
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                Base64::encode(sign_res1),
                sign_res2
            );

            let sign_res1 =
                SM2::sign(msg1.as_bytes().to_vec(), sk_str.clone(), pk_str.clone()).unwrap();
            let sign_res2 =
                SM2::sign_base64(msg1.as_bytes().to_vec(), sk_str.clone(), pk_str.clone()).unwrap();
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                Base64::encode(sign_res1.clone()),
                sign_res2
            );
            /////////////// sk/pk str end ///////////////
            println!(
                "verify = {}",
                SM2::verify(msg1, pk.as_slice(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_str.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_string.clone(), sign_res1.clone()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1, pk.as_slice(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_str.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_string.clone(), sign_res1.as_slice()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1, pk.as_slice(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_str.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_string.clone(), sign_res2.clone()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1, pk.as_slice(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_str.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_string.clone(), sign_res2.as_str()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.as_slice(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_str.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_string.clone(), sign_res1.clone()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.as_slice(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_str.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_string.clone(), sign_res1.as_slice()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.as_slice(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_str.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_string.clone(), sign_res2.clone()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.as_slice(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_str.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_string.clone(), sign_res2.as_str()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.as_slice(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_str.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_string.clone(), sign_res1.clone()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.as_slice(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_str.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_string.clone(), sign_res1.as_slice()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.as_slice(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_str.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_string.clone(), sign_res2.clone()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.as_slice(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_str.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_string.clone(), sign_res2.as_str()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.as_slice(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk_str.clone(), sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(
                    msg1.as_bytes().to_vec(),
                    pk_string.clone(),
                    sign_res1.clone()
                )
                .unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(
                    msg1.as_bytes().to_vec(),
                    pk.as_slice(),
                    sign_res1.as_slice()
                )
                .unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.clone(), sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(
                    msg1.as_bytes().to_vec(),
                    pk_str.clone(),
                    sign_res1.as_slice()
                )
                .unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(
                    msg1.as_bytes().to_vec(),
                    pk_string.clone(),
                    sign_res1.as_slice()
                )
                .unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.as_slice(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk_str.clone(), sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(
                    msg1.as_bytes().to_vec(),
                    pk_string.clone(),
                    sign_res2.clone()
                )
                .unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.as_slice(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk_str.clone(), sign_res2.as_str()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(
                    msg1.as_bytes().to_vec(),
                    pk_string.clone(),
                    sign_res2.as_str()
                )
                .unwrap()
            );
        }
    }

    #[cfg(test)]
    mod sign_filepath {

        use crate::cryptos::base64::{Base64, Base64Encoder};
        use crate::cryptos::sm2::{SM2NewStore, SM2SignPath, SM2VerifyPath, SM2};

        #[test]
        fn test() {
            let sk_filepath = "src/test/crypto/sm2/sign/generate5_sk";
            let pk_filepath = "src/test/crypto/sm2/sign/generate5_pk";
            let (_, _) = SM2::generate(sk_filepath, pk_filepath).unwrap();
            let msg1 = "hello 你好！?";

            let sign_res1 = SM2::sign(msg1, sk_filepath.clone(), pk_filepath.clone()).unwrap();
            let sign_res2 =
                SM2::sign_base64(msg1, sk_filepath.clone(), pk_filepath.clone()).unwrap();
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                Base64::encode(sign_res1),
                sign_res2
            );

            let sign_res1 =
                SM2::sign(msg1.to_string(), sk_filepath.clone(), pk_filepath.clone()).unwrap();
            let sign_res2 =
                SM2::sign_base64(msg1.to_string(), sk_filepath.clone(), pk_filepath.clone())
                    .unwrap();
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                Base64::encode(sign_res1),
                sign_res2
            );

            let sign_res1 =
                SM2::sign(msg1.as_bytes(), sk_filepath.clone(), pk_filepath.clone()).unwrap();
            let sign_res2 =
                SM2::sign_base64(msg1.as_bytes(), sk_filepath.clone(), pk_filepath.clone())
                    .unwrap();
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                Base64::encode(sign_res1),
                sign_res2
            );

            let sign_res1 = SM2::sign(
                msg1.as_bytes().to_vec(),
                sk_filepath.clone(),
                pk_filepath.clone(),
            )
            .unwrap();
            let sign_res2 = SM2::sign_base64(
                msg1.as_bytes().to_vec(),
                sk_filepath.clone(),
                pk_filepath.clone(),
            )
            .unwrap();
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                Base64::encode(sign_res1.clone()),
                sign_res2
            );

            println!(
                "verify = {}",
                SM2::verify(msg1, pk_filepath, sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_filepath, sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_filepath, sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1, pk_filepath, sign_res2.as_str()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_filepath, sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_filepath, sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_filepath, sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.to_string(), pk_filepath, sign_res2.as_str()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_filepath, sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_filepath, sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_filepath, sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes(), pk_filepath, sign_res2.as_str()).unwrap()
            );

            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk_filepath, sign_res1.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk_filepath, sign_res1.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk_filepath, sign_res2.clone()).unwrap()
            );
            println!(
                "verify = {}",
                SM2::verify(msg1.as_bytes().to_vec(), pk_filepath, sign_res2.as_str()).unwrap()
            );
        }
    }

    #[cfg(test)]
    mod test_signature {
        use libsm::sm2::signature::{SigCtx, Signature};

        #[test]
        fn test_sig_encode_and_decode() {
            let string = String::from("abcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcd");
            let msg = string.as_bytes();

            let ctx = SigCtx::new();
            let (pk, sk) = ctx.new_keypair();

            let signature = ctx.sign(msg, &sk, &pk);
            let der = signature.der_encode();
            let sig = Signature::der_decode(&der[..]).unwrap();
            assert!(ctx.verify(msg, &pk, &sig));

            let signature = ctx.sign(msg, &sk, &pk);
            let der = signature.der_encode();
            let sig = Signature::der_decode_raw(&der[2..]).unwrap();
            assert!(ctx.verify(msg, &pk, &sig));
        }
    }
}
