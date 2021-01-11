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
    use crate::cryptos::sm2::{
        generate, generate_hex, generate_hex_in_files, generate_in_files, generate_pk_from_sk,
        generate_pk_from_sk_file, generate_pk_from_sk_str, sign, sign_string, verify,
        verify_string,
    };

    #[test]
    fn generate_test() {
        let (sk, pk) = generate();
        println!("sk = {}\npk = {}", hex::encode(sk), hex::encode(pk));
    }

    #[test]
    fn generate_hex_test() {
        let (sk, pk) = generate_hex();
        println!("sk = {}\npk = {}", sk, pk);
    }

    #[test]
    fn generate_pk_test() {
        match generate_in_files(
            "src/test/crypto/sm2/generate_pk/generate_sk",
            "src/test/crypto/sm2/generate_pk/generate_pk",
        ) {
            Ok(ks) => {
                let sk = ks.0.clone();
                let pk = ks.1.clone();
                let sk_str = hex::encode(sk.clone());
                let pk_new1 = generate_pk_from_sk(sk.clone()).unwrap();
                let pk_new2 = generate_pk_from_sk_str(sk_str.clone()).unwrap();
                let pk_new3 = generate_pk_from_sk_file(
                    "src/test/crypto/sm2/generate_pk/generate_sk".to_string(),
                )
                    .unwrap();
                println!(
                    "sk = {}\npk_new0 = {}\npk_new1 = {}\npk_new2 = {}\npk_new3 = {}",
                    sk_str,
                    hex::encode(pk),
                    hex::encode(pk_new1),
                    hex::encode(pk_new2),
                    hex::encode(pk_new3)
                );
            }
            Err(err) => println!("err = {}", err.to_string()),
        }
    }

    #[test]
    fn generate_in_file_test() {
        match generate_in_files(
            "src/test/crypto/sm2/generate_in_file/generate_sk",
            "src/test/crypto/sm2/generate_in_file/generate_pk",
        ) {
            Ok(ks) => println!("sk = {}\npk = {}", hex::encode(ks.0), hex::encode(ks.1)),
            Err(err) => println!("err = {}", err.to_string()),
        }
    }

    #[test]
    fn generate_hex_in_file_test() {
        match generate_hex_in_files(
            "src/test/crypto/sm2/generate_hex_in_file/generate_sk",
            "src/test/crypto/sm2/generate_hex_in_file/generate_pk",
        ) {
            Ok(ks) => println!("sk = {}\npk = {}", ks.0, ks.1),
            Err(err) => println!("err = {}", err.to_string()),
        }
    }

    #[test]
    fn sign_test() {
        match generate_in_files(
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
