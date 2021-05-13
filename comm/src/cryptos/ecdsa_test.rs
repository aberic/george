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
mod ecdsa {

    #[cfg(test)]
    mod demo {
        use openssl::ec::{EcGroup, EcKey};
        use openssl::ecdsa::EcdsaSig;
        use openssl::error::ErrorStack;
        use openssl::nid::Nid;
        use openssl::pkey::{Private, Public};

        fn get_public_key(
            group: &EcGroup,
            x: &EcKey<Private>,
        ) -> Result<EcKey<Public>, ErrorStack> {
            EcKey::from_public_key(group, x.public_key())
        }

        #[test]
        #[cfg_attr(osslconf = "OPENSSL_NO_EC2M", ignore)]
        fn sign_and_verify() {
            let group = EcGroup::from_curve_name(Nid::X9_62_PRIME192V1).unwrap();
            let private_key = EcKey::generate(&group).unwrap();
            let public_key = get_public_key(&group, &private_key).unwrap();

            let private_key2 = EcKey::generate(&group).unwrap();
            let public_key2 = get_public_key(&group, &private_key2).unwrap();

            let data = String::from("hello");
            let res = EcdsaSig::sign(data.as_bytes(), &private_key).unwrap();

            // Signature can be verified using the correct data & correct public key
            let verification = res.verify(data.as_bytes(), &public_key).unwrap();
            assert!(verification);

            // Signature will not be verified using the incorrect data but the correct public key
            let verification2 = res
                .verify(String::from("hello2").as_bytes(), &public_key)
                .unwrap();
            assert!(!verification2);

            // Signature will not be verified using the correct data but the incorrect public key
            let verification3 = res.verify(data.as_bytes(), &public_key2).unwrap();
            assert!(!verification3);
        }
    }

    #[cfg(test)]
    mod generate {
        use crate::cryptos::base64::{Base64, Base64Encoder, Basee64Decoder};
        use crate::cryptos::ecdsa::ECDSA;
        use crate::cryptos::hex::{Hex, HexDecoder, HexEncoder};

        #[test]
        fn test() {
            let ecdsa_pre = ECDSA::new().unwrap();
            let sk = ecdsa_pre.sk();
            let pk = ecdsa_pre.pk();
            let ecdsa = ECDSA::from(sk.clone()).unwrap();

            let res = "hello world!";
            let data = res.as_bytes();
            let sig_res = ecdsa_pre.sign(data).unwrap();
            println!(
                "verify = {}",
                ecdsa_pre.verify(data, sig_res.as_slice()).unwrap()
            );
            println!(
                "verify = {}",
                ecdsa.verify(data, sig_res.as_slice()).unwrap()
            );

            let sk_hex = ecdsa.sk_hex();
            let pk_hex = ecdsa.pk_hex().unwrap();
            let sk_b64 = ecdsa.sk_base64();
            let pk_b64 = ecdsa.pk_base64().unwrap();
            println!("sk str hex = {}", sk_hex.clone());
            println!("sk str b64 = {}", sk_b64.clone());
            println!("sk pem str = {}", ecdsa.sk_pem_str().unwrap());
            println!("sk pem hex = {}", ecdsa.sk_pem_hex().unwrap());
            println!("sk pem b64 = {}", ecdsa.sk_pem_base64().unwrap());
            println!("sk der hex = {}", ecdsa.sk_der_hex().unwrap());
            println!("sk der b64 = {}", ecdsa.sk_der_base64().unwrap());
            println!();
            println!("pk str hex = {}", pk_hex.clone());
            println!("pk str b64 = {}", pk_b64.clone());
            println!("pk pem str = {}", ecdsa.pk_pem_str().unwrap());
            println!("pk pem hex = {}", ecdsa.pk_pem_hex().unwrap());
            println!("pk pem b64 = {}", ecdsa.pk_pem_base64().unwrap());
            println!("pk der hex = {}", ecdsa.pk_der_hex().unwrap());
            println!("pk der b64 = {}", ecdsa.pk_der_base64().unwrap());

            let ecdsa2 = ECDSA::from_hex(sk_hex.clone(), pk_hex.clone()).unwrap();
            println!(
                "verify = {}",
                ecdsa2.verify(data, sig_res.as_slice()).unwrap()
            );
            println!("ecdsa sk hex = {}", sk_hex.clone());
            println!("ecdsa2 sk hex = {}", ecdsa2.sk_hex());
            println!("ecdsa pk hex = {}", pk_hex.clone());
            println!("ecdsa2 pk hex = {}", ecdsa2.pk_hex().unwrap());
            println!();

            let ecdsa3 = ECDSA::from_base64(sk_b64.clone(), pk_b64.clone()).unwrap();
            println!(
                "verify = {}",
                ecdsa3.verify(data, sig_res.as_slice()).unwrap()
            );
            println!("ecdsa sk b64 = {}", sk_b64.clone());
            println!("ecdsa3 sk b64 = {}", ecdsa3.sk_base64());
            println!("ecdsa pk b64 = {}", pk_b64.clone());
            println!("ecdsa3 pk b64 = {}", ecdsa3.pk_base64().unwrap());
            println!();

            let ecdsa4 = ECDSA::from_pem(
                ecdsa.sk_pem_str().unwrap().into_bytes(),
                ecdsa.pk_pem_str().unwrap().into_bytes(),
            )
            .unwrap();
            println!(
                "verify = {}",
                ecdsa4.verify(data, sig_res.as_slice()).unwrap()
            );
            println!("ecdsa sk b64 = {}", sk_b64.clone());
            println!("ecdsa4 sk b64 = {}", ecdsa4.sk_base64());
            println!("ecdsa pk b64 = {}", pk_b64.clone());
            println!("ecdsa4 pk b64 = {}", ecdsa4.pk_base64().unwrap());
            println!();

            let ecdsa5 = ECDSA::from_der(
                Hex::decode(ecdsa.sk_der_hex().unwrap()).unwrap(),
                Base64::decode(ecdsa.pk_der_base64().unwrap()).unwrap(),
            )
            .unwrap();
            println!(
                "verify = {}",
                ecdsa5.verify(data, sig_res.as_slice()).unwrap()
            );
            println!("ecdsa sk b64 = {}", sk_b64.clone());
            println!("ecdsa5 sk b64 = {}", ecdsa5.sk_base64());
            println!("ecdsa pk b64 = {}", pk_b64.clone());
            println!("ecdsa5 pk b64 = {}", ecdsa5.pk_base64().unwrap());
        }
    }

    #[cfg(test)]
    mod old {
        use openssl::ec::{EcGroup, EcKey};
        use openssl::nid::Nid;

        use crate::cryptos::ecdsa::{
            generate_pk_in_file_from_sk, generate_pk_in_file_from_sk_bytes,
            generate_pk_in_file_from_sk_file, generate_sk_in_files,
        };
        use crate::io::file::{Filer, FilerWriter};

        #[test]
        fn generate_pri_test() {
            match generate_sk_in_files("src/test/crypto/ecdsa/generate_pri.key.pem") {
                Ok(u8s) => println!("pri = {}", String::from_utf8(u8s).unwrap()),
                Err(err) => println!("err = {}", err),
            }
            match generate_sk_in_files("src/test/crypto/ecdsa/generate_pri.key.pem") {
                Ok(u8s) => println!("pri = {}", String::from_utf8(u8s).unwrap()),
                Err(err) => println!("err = {}", err),
            }
        }

        #[test]
        fn generate_pub_test() {
            let pri_filepath = "src/test/crypto/ecdsa/generate_pri1.key.pem".to_string();
            match EcGroup::from_curve_name(Nid::X9_62_PRIME256V1) {
                Ok(group) => match EcKey::generate(&group) {
                    Ok(key) => {
                        match generate_pk_in_file_from_sk(
                            key.clone(),
                            "src/test/crypto/ecdsa/generate_pub1.pem".to_string(),
                        ) {
                            Err(err) => {
                                println!("generate_pub_in_file_from_pri, {}", err.to_string())
                            }
                            _ => {}
                        }
                        match key.private_key_to_pem() {
                            Ok(u8s) => {
                                Filer::write_force(pri_filepath.clone(), u8s.clone()).unwrap();
                                println!("pri = {}", String::from_utf8(u8s.clone()).unwrap());
                                match generate_pk_in_file_from_sk_bytes(
                                    u8s,
                                    "src/test/crypto/ecdsa/generate_pub2.pem".to_string(),
                                ) {
                                    Err(err) => println!(
                                        "generate_pub_in_file_from_pri, {}",
                                        err.to_string()
                                    ),
                                    _ => {}
                                }
                            }
                            Err(err) => println!("private_key_to_pem_pkcs8, {}", err.to_string()),
                        }
                    }
                    Err(err) => println!("generate, {}", err.to_string()),
                },
                Err(err) => println!("from_curve_name, {}", err.to_string()),
            }
            match generate_pk_in_file_from_sk_file(
                pri_filepath,
                "src/test/crypto/ecdsa/generate_pub3.pem".to_string(),
            ) {
                Err(err) => println!("generate_pub_in_file_from_pri_file, {}", err.to_string()),
                _ => {}
            }
        }
    }
}
