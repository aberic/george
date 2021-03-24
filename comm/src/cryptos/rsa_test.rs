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
mod rsa {
    use openssl::pkey::PKey;
    use openssl::rsa::Rsa;

    use crate::cryptos::rsa::{
        generate_pk_in_file_from_sk, generate_pk_in_file_from_sk_bytes,
        generate_pk_in_file_from_sk_file, generate_sk_in_files,
    };
    use crate::io::file::{Filer, FilerWriter};

    #[test]
    fn generate_pri_test() {
        match generate_sk_in_files(2048, "src/test/crypto/rsa/generate_pri.key.pem") {
            Ok(u8s) => println!("pri = {}", String::from_utf8(u8s).unwrap()),
            Err(err) => println!("err = {}", err),
        }
        match generate_sk_in_files(2048, "src/test/crypto/rsa/generate_pri.key.pem") {
            Ok(u8s) => println!("pri = {}", String::from_utf8(u8s).unwrap()),
            Err(err) => println!("err = {}", err),
        }
    }

    #[test]
    fn generate_pub_test() {
        let pri_filepath = "src/test/crypto/rsa/generate_pri1.key.pem".to_string();
        match Rsa::generate(2048) {
            Ok(rsa) => match PKey::from_rsa(rsa) {
                Ok(key) => {
                    match generate_pk_in_file_from_sk(
                        key.clone(),
                        "src/test/crypto/rsa/generate_pub1.pem".to_string(),
                    ) {
                        Err(err) => println!("generate_pub_in_file_from_pri, {}", err.to_string()),
                        _ => {}
                    }
                    match key.private_key_to_pem_pkcs8() {
                        Ok(u8s) => {
                            Filer::write_force(pri_filepath.clone(), u8s.clone()).unwrap();
                            println!("pri = {}", String::from_utf8(u8s.clone()).unwrap());
                            match generate_pk_in_file_from_sk_bytes(
                                u8s,
                                "src/test/crypto/rsa/generate_pub2.pem".to_string(),
                            ) {
                                Err(err) => {
                                    println!("generate_pub_in_file_from_pri, {}", err.to_string())
                                }
                                _ => {}
                            }
                        }
                        Err(err) => println!("private_key_to_pem_pkcs8, {}", err.to_string()),
                    }
                }
                Err(err) => println!("from_rsa, {}", err.to_string()),
            },
            Err(err) => println!("generate, {}", err.to_string()),
        }
        match generate_pk_in_file_from_sk_file(
            pri_filepath,
            "src/test/crypto/rsa/generate_pub3.pem".to_string(),
        ) {
            Err(err) => println!("generate_pub_in_file_from_pri_file, {}", err.to_string()),
            _ => {}
        }
    }
}
