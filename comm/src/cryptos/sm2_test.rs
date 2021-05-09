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
    mod generate {
        use crate::cryptos::sm2::{SM2New, SM2};

        #[test]
        fn test() {
            let (sk, pk) = SM2::generate();
            println!("sk = {}\npk = {}", hex::encode(sk), hex::encode(pk));
            let (sk, pk) = SM2::generate();
            println!("sk = {}\npk = {}", hex::encode(sk), hex::encode(pk));
            let (sk, pk) = SM2::generate_string();
            println!("sk = {}\npk = {}", sk, pk);
            let (sk, pk) = SM2::generate_string();
            println!("sk = {}\npk = {}", sk, pk);
        }
    }

    #[cfg(test)]
    mod generate_file {
        use crate::cryptos::sm2::{SM2NewStore, SM2};

        #[test]
        fn test1() {
            let (sk, pk) = SM2::generate(
                "src/test/crypto/sm2/generate_file/generate1_sk",
                "src/test/crypto/sm2/generate_file/generate1_pk",
            )
            .unwrap();
            println!("sk = {}\npk = {}", hex::encode(sk), hex::encode(pk));
            let (sk, pk) = SM2::generate_string(
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
            println!("sk = {}\npk = {}", hex::encode(sk), hex::encode(pk));
            let (sk, pk) = SM2::generate_string(
                "src/test/crypto/sm2/generate_file/generate4_sk".to_string(),
                "src/test/crypto/sm2/generate_file/generate4_pk".to_string(),
            )
            .unwrap();
            println!("sk = {}\npk = {}", sk, pk);
        }
    }

    #[cfg(test)]
    mod generate_pk_v8s {
        use crate::cryptos::sm2::{SM2New, SM2PKV8s, SM2};

        #[test]
        fn generate_pk_test() {
            let (sk, pk) = SM2::generate();
            let pk_new = SM2::generate_pk(sk.clone()).unwrap();
            println!(
                "sk = {}\npk = {}\nne = {}",
                hex::encode(sk),
                hex::encode(pk),
                hex::encode(pk_new)
            );

            let (sk, pk) = SM2::generate_string();
            let pk_new = SM2::generate_pk(sk.clone()).unwrap();
            println!("sk = {}\npk = {}\nne = {}", sk, pk, hex::encode(pk_new));
        }
    }

    #[cfg(test)]
    mod generate_pk_string {
        use crate::cryptos::sm2::{SM2New, SM2PKString, SM2};

        #[test]
        fn generate_pk_test() {
            let (sk, pk) = SM2::generate();
            let pk_new = SM2::generate_pk(sk.clone()).unwrap();
            println!(
                "sk = {}\npk = {}\nne = {}",
                hex::encode(sk),
                hex::encode(pk),
                pk_new
            );

            let (sk, pk) = SM2::generate_string();
            let pk_new = SM2::generate_pk(sk.clone()).unwrap();
            println!("sk = {}\npk = {}\nne = {}", sk, pk, pk_new);
        }
    }

    #[cfg(test)]
    mod generate_pk_v8s_path {
        use crate::cryptos::sm2::{SM2NewStore, SM2PKV8sPath, SM2};

        #[test]
        fn generate_pk_test() {
            let (sk, pk) = SM2::generate(
                "src/test/crypto/sm2/generate_pk_file/generate1_sk",
                "src/test/crypto/sm2/generate_pk_file/generate1_pk",
            )
            .unwrap();
            let pk_new =
                SM2::generate_pk("src/test/crypto/sm2/generate_pk_file/generate1_sk".to_string())
                    .unwrap();
            println!("pk = {}\nne = {}", hex::encode(pk), hex::encode(pk_new));
        }
    }

    #[cfg(test)]
    mod generate_pk_string_path {
        use crate::cryptos::sm2::{SM2NewStore, SM2PKStringPath, SM2};

        #[test]
        fn generate_pk_test() {
            let (sk, pk) = SM2::generate_string(
                "src/test/crypto/sm2/generate_pk_file/generate2_sk",
                "src/test/crypto/sm2/generate_pk_file/generate2_pk",
            )
            .unwrap();
            let pk_new =
                SM2::generate_pk("src/test/crypto/sm2/generate_pk_file/generate2_sk".to_string())
                    .unwrap();
            println!("pk = {}\nne = {}", pk, pk_new);
        }
    }

    #[cfg(test)]
    mod sign_u8s {
        use crate::cryptos::sm2::{SM2NewStore, SM2Sign, SM2};

        #[test]
        fn test() {
            let (sk, pk) = SM2::generate(
                "src/test/crypto/sm2/sign/generate_sk",
                "src/test/crypto/sm2/sign/generate_pk",
            )
            .unwrap();
            let msg1 = "hello 你好！?";

            let sign_res1 = SM2::sign(msg1, sk.as_slice(), pk.as_slice());
            let sign_res2 = SM2::sign_string(msg1, sk.as_slice(), pk.as_slice());
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                hex::encode(sign_res1),
                sign_res2
            );

            let sign_res1 = SM2::sign(msg1.to_string(), sk.as_slice(), pk.as_slice());
            let sign_res2 = SM2::sign_string(msg1.to_string(), sk.as_slice(), pk.as_slice());
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                hex::encode(sign_res1),
                sign_res2
            );

            let sign_res1 = SM2::sign(msg1.as_bytes(), sk.as_slice(), pk.as_slice());
            let sign_res2 = SM2::sign_string(msg1.as_bytes(), sk.as_slice(), pk.as_slice());
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                hex::encode(sign_res1),
                sign_res2
            );

            let sign_res1 = SM2::sign(msg1.as_bytes().to_vec(), sk.as_slice(), pk.as_slice());
            let sign_res2 =
                SM2::sign_string(msg1.as_bytes().to_vec(), sk.as_slice(), pk.as_slice());
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                hex::encode(sign_res1),
                sign_res2
            );

            let sign_res1 = SM2::sign(msg1, sk.clone(), pk.clone());
            let sign_res2 = SM2::sign_string(msg1, sk.clone(), pk.clone());
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                hex::encode(sign_res1),
                sign_res2
            );

            let sign_res1 = SM2::sign(msg1.to_string(), sk.clone(), pk.clone());
            let sign_res2 = SM2::sign_string(msg1.to_string(), sk.clone(), pk.clone());
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                hex::encode(sign_res1),
                sign_res2
            );

            let sign_res1 = SM2::sign(msg1.as_bytes(), sk.clone(), pk.clone());
            let sign_res2 = SM2::sign_string(msg1.as_bytes(), sk.clone(), pk.clone());
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                hex::encode(sign_res1),
                sign_res2
            );

            let sign_res1 = SM2::sign(msg1.as_bytes().to_vec(), sk.clone(), pk.clone());
            let sign_res2 = SM2::sign_string(msg1.as_bytes().to_vec(), sk.clone(), pk.clone());
            println!(
                "sign_res1 = {}\nsign_res2 = {}",
                hex::encode(sign_res1),
                sign_res2
            );
        }
    }

    #[cfg(test)]
    mod old_test {
        use crate::cryptos::sm2::{sign, sign_string, verify, verify_string, SM2NewStore, SM2};

        #[test]
        fn sign_test() {
            match SM2::generate(
                "src/test/crypto/sm2/sign/generate_sk",
                "src/test/crypto/sm2/sign/generate_pk",
            ) {
                Ok(ks) => {
                    let sk = ks.0.clone();
                    let pk = ks.1.clone();
                    let sk_str = hex::encode(sk.clone());
                    let pk_str = hex::encode(pk.clone());
                    let msg1 = "hello 你好！?";
                    let msg2 = "hello 你好！？";

                    let der1 = sign(msg1.as_bytes(), sk.as_slice(), pk.clone().as_slice());
                    let der2 = sign_string(msg1.to_string(), sk_str.clone(), pk_str.clone());
                    let b1 = verify_string(msg1.to_string(), pk_str.clone(), der1.as_slice());
                    let b2 = verify(msg1.as_bytes(), pk.clone().as_slice(), der2.as_slice());
                    let b1x = verify_string(msg1.to_string(), pk_str.clone(), der2.as_slice());
                    let b2x = verify(msg1.as_bytes(), pk.clone().as_slice(), der1.as_slice());

                    let der3 = sign(msg1.as_bytes(), sk.as_slice(), pk.clone().as_slice());
                    let der4 = sign_string(msg1.to_string(), sk_str.clone(), pk_str.clone());
                    let b3 = verify_string(msg2.to_string(), pk_str.clone(), der3.as_slice());
                    let b4 = verify(msg2.as_bytes(), pk.clone().as_slice(), der4.as_slice());

                    println!(
                        "der1 = {:#?}\nder2 = {:#?}\nder3 = {:#?}\nder4 = {:#?}\nder1_str = {:#?}",
                        der1.clone(),
                        der2.clone(),
                        der3.clone(),
                        der4.clone(),
                        hex::encode(der1.clone())
                    );
                    println!(
                        "b1 = {}\nb2 = {}\nb1x = {}\nb2x = {}\nb3 = {}\nb4 = {}",
                        b1, b2, b1x, b2x, b3, b4
                    );
                }
                Err(err) => println!("err = {}", err.to_string()),
            }
        }
    }
}
