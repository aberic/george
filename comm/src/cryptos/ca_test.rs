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
mod ca {
    use crate::cryptos::ca::{
        create, create_cert_request, load_ca_file, sign, AltName, X509NameInfo,
    };
    use crate::cryptos::rsa;
    use crate::cryptos::rsa::{RSANewStore, RSA};
    use crate::io::file::{Filer, FilerWriter};

    fn x509_name_info() -> X509NameInfo {
        X509NameInfo {
            country: "CN".to_string(),
            organization: "Sky".to_string(),
            organizational_unit: "Rocket".to_string(),
            locality: "ChangPing".to_string(),
            province: "Beijing".to_string(),
            common_name: "skyfile.info".to_string(),
        }
    }

    fn x509_name_info1() -> X509NameInfo {
        X509NameInfo {
            country: "CN".to_string(),
            organization: "Hub".to_string(),
            organizational_unit: "Fire".to_string(),
            locality: "XiCheng".to_string(),
            province: "Beijing".to_string(),
            common_name: "aberic.cn".to_string(),
        }
    }

    fn alt_name() -> AltName {
        AltName {
            dns_names: vec!["ha.com".to_string(), "x.he.org".to_string()],
            email_addresses: vec!["yes@when.com".to_string()],
            ip_addresses: vec!["192.168.0.1".to_string(), "127.0.0.1".to_string()],
            uris: vec![],
        }
    }

    #[test]
    fn create_cert_request_test() {
        let sk_filepath = "src/test/crypto/ca/create_cert_request/rsa_sk.pem";
        let csr_filepath = "src/test/crypto/ca/create_cert_request/rsa_csr.pem";
        match RSA::generate_pkcs8_pem(2048, sk_filepath) {
            Ok(u8s) => println!("pri = {}", String::from_utf8(u8s).unwrap()),
            Err(err) => {
                println!("generate_sk_in_files = {}", err);
                return;
            }
        }
        match rsa::load_sk_pkey_file(sk_filepath.to_string()) {
            Ok(key) => match create_cert_request(&key, x509_name_info()) {
                Ok(csr) => match csr.to_pem() {
                    Ok(pem) => {
                        Filer::write_file_force(csr_filepath, pem).unwrap();
                    }
                    Err(err) => {
                        println!("to_pem = {}", err);
                        return;
                    }
                },
                Err(err) => {
                    println!("create_cert_request = {}", err);
                    return;
                }
            },
            Err(err) => {
                println!("load_sk_file = {}", err);
                return;
            }
        }
    }

    fn create_demo(sk_filepath: &str, root_filepath: &str) {
        match RSA::generate_pkcs8_pem(2048, sk_filepath) {
            Ok(u8s) => println!("pri = {}", String::from_utf8(u8s).unwrap()),
            Err(err) => {
                println!("generate_sk_in_files = {}", err);
                return;
            }
        }
        match rsa::load_sk_pkey_file(sk_filepath.to_string()) {
            Ok(key) => match create(128, &key, x509_name_info(), 2, 0, 356) {
                Ok(x509) => match x509.to_pem() {
                    Ok(pem) => {
                        Filer::write_file_force(root_filepath, pem).unwrap();
                    }
                    Err(err) => {
                        println!("to_pem = {}", err);
                        return;
                    }
                },
                Err(err) => {
                    println!("create = {}", err);
                    return;
                }
            },
            Err(err) => {
                println!("load_sk_file = {}", err);
                return;
            }
        }
    }

    #[test]
    fn create_test() {
        let sk_filepath = "src/test/crypto/ca/create/rsa_sk.pem";
        let root_filepath = "src/test/crypto/ca/create/root.pem";
        create_demo(sk_filepath, root_filepath);
    }

    #[test]
    fn load_test() {
        let sk_filepath = "src/test/crypto/ca/load/rsa_sk.pem";
        let root_filepath = "src/test/crypto/ca/load/root.pem";
        create_demo(sk_filepath, root_filepath);
        match load_ca_file(root_filepath.to_string()) {
            Ok(x509) => println!(
                "root = {}",
                String::from_utf8(x509.to_pem().unwrap()).unwrap()
            ),
            Err(err) => println!("err = {}", err),
        }
    }

    #[test]
    fn sign_test() {
        let ca_sk_filepath = "src/test/crypto/ca/sign/ca_sk.pem";
        let ca_root_filepath = "src/test/crypto/ca/sign/ca_root.pem";
        let sk_filepath = "src/test/crypto/ca/sign/sk.pem";
        let cert_filepath = "src/test/crypto/ca/sign/cert.pem";
        create_demo(ca_sk_filepath, ca_root_filepath);
        match RSA::generate_pkcs8_pem(2048, ca_sk_filepath) {
            Ok(u8s) => println!("pri = {}", String::from_utf8(u8s).unwrap()),
            Err(err) => println!("err = {}", err),
        }
        match RSA::generate_pkcs8_pem(2048, sk_filepath) {
            Ok(u8s) => println!("pri = {}", String::from_utf8(u8s).unwrap()),
            Err(err) => println!("err = {}", err),
        }
        match sign(
            ca_root_filepath.to_string(),
            ca_sk_filepath.to_string(),
            128,
            sk_filepath.to_string(),
            x509_name_info1(),
            alt_name(),
            2,
            0,
            356,
        ) {
            Ok(x509) => match x509.to_pem() {
                Ok(pem) => {
                    Filer::write_file_force(cert_filepath, pem).unwrap();
                }
                Err(err) => {
                    println!("to_pem = {}", err);
                    return;
                }
            },
            Err(err) => {
                println!("create = {}", err);
                return;
            }
        }
    }
}
