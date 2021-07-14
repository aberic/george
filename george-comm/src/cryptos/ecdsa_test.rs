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
        use crate::cryptos::base64::Base64Decoder;
        use crate::cryptos::hex::HexDecoder;
        use crate::cryptos::Base64;
        use crate::cryptos::Hex;
        use crate::cryptos::ECDSA;

        #[test]
        fn test() {
            let ecdsa_pre = ECDSA::new().unwrap();
            let sk = ecdsa_pre.sk_ec();
            let ecdsa1 = ECDSA::from_sk(sk.clone()).unwrap();

            let res = "hello world!";
            let data = res.as_bytes();
            let sig_res = ecdsa_pre.sign(data).unwrap();
            println!(
                "verify = {}",
                ecdsa_pre.verify(data, sig_res.as_slice()).unwrap()
            );
            assert!(ecdsa1.verify(data, sig_res.as_slice()).unwrap());

            let sk_hex = ecdsa1.sk_hex();
            let pk_hex = ecdsa1.pk_hex().unwrap();
            let sk_b64 = ecdsa1.sk_base64();
            let pk_b64 = ecdsa1.pk_base64().unwrap();
            println!("sk str hex = {}", sk_hex.clone());
            println!("sk str b64 = {}", sk_b64.clone());
            println!("sk pem str = {}", ecdsa1.sk_pem_str().unwrap());
            println!("sk pem pkcs8 str = {}", ecdsa1.sk_pem_pkcs8_str().unwrap());
            println!("sk pem hex = {}", ecdsa1.sk_pem_hex().unwrap());
            println!("sk pem b64 = {}", ecdsa1.sk_pem_base64().unwrap());
            println!("sk der hex = {}", ecdsa1.sk_der_hex().unwrap());
            println!("sk der b64 = {}", ecdsa1.sk_der_base64().unwrap());
            println!();
            println!("pk str hex = {}", pk_hex.clone());
            println!("pk str b64 = {}", pk_b64.clone());
            println!("pk pem str = {}", ecdsa1.pk_pem_str().unwrap());
            println!("pk pem hex = {}", ecdsa1.pk_pem_hex().unwrap());
            println!("pk pem b64 = {}", ecdsa1.pk_pem_base64().unwrap());
            println!("pk der hex = {}", ecdsa1.pk_der_hex().unwrap());
            println!("pk der b64 = {}", ecdsa1.pk_der_base64().unwrap());

            let ecdsa2 = ECDSA::from_hex(sk_hex.clone(), pk_hex.clone()).unwrap();
            assert!(ecdsa2.verify(data, sig_res.as_slice()).unwrap());
            println!("ecdsa sk hex = {}", sk_hex.clone());
            println!("ecdsa2 sk hex = {}", ecdsa2.sk_hex());
            println!("ecdsa pk hex = {}", pk_hex.clone());
            println!("ecdsa2 pk hex = {}", ecdsa2.pk_hex().unwrap());
            println!();

            let ecdsa3 = ECDSA::from_base64(sk_b64.clone(), pk_b64.clone()).unwrap();
            assert!(ecdsa3.verify(data, sig_res.as_slice()).unwrap());
            println!("ecdsa sk b64 = {}", sk_b64.clone());
            println!("ecdsa3 sk b64 = {}", ecdsa3.sk_base64());
            println!("ecdsa pk b64 = {}", pk_b64.clone());
            println!("ecdsa3 pk b64 = {}", ecdsa3.pk_base64().unwrap());
            println!();

            let ecdsa4 = ECDSA::from_pem(
                ecdsa1.sk_pem_str().unwrap().into_bytes(),
                ecdsa1.pk_pem_str().unwrap().into_bytes(),
            )
            .unwrap();
            assert!(ecdsa4.verify(data, sig_res.as_slice()).unwrap());
            println!("ecdsa sk pem = {}", sk_b64.clone());
            println!("ecdsa4 sk pem = {}", ecdsa4.sk_base64());
            println!("ecdsa pk pem = {}", pk_b64.clone());
            println!("ecdsa4 pk pem = {}", ecdsa4.pk_base64().unwrap());
            println!();

            let ecdsa5 = ECDSA::from_sk_pem_pkcs8(ecdsa1.sk_pem_pkcs8().unwrap()).unwrap();
            assert!(ecdsa5.verify(data, sig_res.as_slice()).unwrap());
            println!("ecdsa sk pem pkcs8 = {}", sk_b64.clone());
            println!("ecdsa5 sk pem pkcs8 = {}", ecdsa5.sk_base64());
            println!("ecdsa pk pem pkcs8 = {}", pk_b64.clone());
            println!("ecdsa5 pk pem pkcs8 = {}", ecdsa5.pk_base64().unwrap());

            let ecdsa6 = ECDSA::from_der(
                Hex::decode(ecdsa1.sk_der_hex().unwrap()).unwrap(),
                Base64::decode(ecdsa1.pk_der_base64().unwrap()).unwrap(),
            )
            .unwrap();
            assert!(ecdsa6.verify(data, sig_res.as_slice()).unwrap());
            println!("ecdsa sk der = {}", sk_b64.clone());
            println!("ecdsa6 sk der = {}", ecdsa6.sk_base64());
            println!("ecdsa pk der = {}", pk_b64.clone());
            println!("ecdsa6 pk der = {}", ecdsa6.pk_base64().unwrap());
        }

        #[test]
        fn store_test() {
            let ecdsa = ECDSA::new().unwrap();
            ecdsa
                .store_hex(
                    "src/test/crypto/ecdsa/store/hex_sk",
                    "src/test/crypto/ecdsa/store/hex_pk",
                )
                .unwrap();
            ecdsa
                .store_base64(
                    "src/test/crypto/ecdsa/store/base64_sk",
                    "src/test/crypto/ecdsa/store/base64_pk",
                )
                .unwrap();
            ecdsa
                .store_pem(
                    "src/test/crypto/ecdsa/store/pem_str_sk",
                    "src/test/crypto/ecdsa/store/pem_str_pk",
                )
                .unwrap();
            ecdsa
                .store_pem_hex(
                    "src/test/crypto/ecdsa/store/pem_hex_sk",
                    "src/test/crypto/ecdsa/store/pem_hex_pk",
                )
                .unwrap();
            ecdsa
                .store_pem_base64(
                    "src/test/crypto/ecdsa/store/pem_base64_sk",
                    "src/test/crypto/ecdsa/store/pem_base64_pk",
                )
                .unwrap();
            ecdsa
                .store_der(
                    "src/test/crypto/ecdsa/store/der_sk",
                    "src/test/crypto/ecdsa/store/der_pk",
                )
                .unwrap();
            ecdsa
                .store_der_hex(
                    "src/test/crypto/ecdsa/store/der_hex_sk",
                    "src/test/crypto/ecdsa/store/der_hex_pk",
                )
                .unwrap();
            ecdsa
                .store_der_base64(
                    "src/test/crypto/ecdsa/store/der_base64_sk",
                    "src/test/crypto/ecdsa/store/der_base64_pk",
                )
                .unwrap();
        }

        #[test]
        fn load_test() {
            let ecdsa = ECDSA::new().unwrap();
            let res = "hello world!";
            let data = res.as_bytes();
            let sig_res = ecdsa.sign(data).unwrap();
            ecdsa
                .store_hex(
                    "src/test/crypto/ecdsa/load/hex_sk",
                    "src/test/crypto/ecdsa/load/hex_pk",
                )
                .unwrap();
            let ecdsa_load = ECDSA::from_hex_file(
                "src/test/crypto/ecdsa/load/hex_sk",
                "src/test/crypto/ecdsa/load/hex_pk",
            )
            .unwrap();
            assert!(ecdsa_load.verify(data, sig_res.as_slice()).unwrap());
            ecdsa
                .store_base64(
                    "src/test/crypto/ecdsa/load/base64_sk",
                    "src/test/crypto/ecdsa/load/base64_pk",
                )
                .unwrap();
            let ecdsa_load = ECDSA::from_base64_file(
                "src/test/crypto/ecdsa/load/base64_sk",
                "src/test/crypto/ecdsa/load/base64_pk",
            )
            .unwrap();
            assert!(ecdsa_load.verify(data, sig_res.as_slice()).unwrap());
            ecdsa
                .store_pem(
                    "src/test/crypto/ecdsa/load/pem_sk",
                    "src/test/crypto/ecdsa/load/pem_pk",
                )
                .unwrap();
            let ecdsa_load = ECDSA::from_pem_file(
                "src/test/crypto/ecdsa/load/pem_sk",
                "src/test/crypto/ecdsa/load/pem_pk",
            )
            .unwrap();
            assert!(ecdsa_load.verify(data, sig_res.as_slice()).unwrap());
            let ecdsa_load = ECDSA::from_sk_pem_file("src/test/crypto/ecdsa/load/pem_sk").unwrap();
            assert!(ecdsa_load.verify(data, sig_res.as_slice()).unwrap());
            ecdsa
                .store_pem_hex(
                    "src/test/crypto/ecdsa/load/pem_hex_sk",
                    "src/test/crypto/ecdsa/load/pem_hex_pk",
                )
                .unwrap();
            let ecdsa_load = ECDSA::from_pem_hex_file(
                "src/test/crypto/ecdsa/load/pem_hex_sk",
                "src/test/crypto/ecdsa/load/pem_hex_pk",
            )
            .unwrap();
            assert!(ecdsa_load.verify(data, sig_res.as_slice()).unwrap());
            ecdsa
                .store_pem_base64(
                    "src/test/crypto/ecdsa/load/pem_base64_sk",
                    "src/test/crypto/ecdsa/load/pem_base64_pk",
                )
                .unwrap();
            let ecdsa_load = ECDSA::from_pem_base64_file(
                "src/test/crypto/ecdsa/load/pem_base64_sk",
                "src/test/crypto/ecdsa/load/pem_base64_pk",
            )
            .unwrap();
            assert!(ecdsa_load.verify(data, sig_res.as_slice()).unwrap());
            ecdsa
                .store_der(
                    "src/test/crypto/ecdsa/store/der_sk",
                    "src/test/crypto/ecdsa/store/der_pk",
                )
                .unwrap();
            let ecdsa_load = ECDSA::from_der_file(
                "src/test/crypto/ecdsa/store/der_sk",
                "src/test/crypto/ecdsa/store/der_pk",
            )
            .unwrap();
            assert!(ecdsa_load.verify(data, sig_res.as_slice()).unwrap());
            ecdsa
                .store_der_hex(
                    "src/test/crypto/ecdsa/load/der_hex_sk",
                    "src/test/crypto/ecdsa/load/der_hex_pk",
                )
                .unwrap();
            let ecdsa_load = ECDSA::from_der_hex_file(
                "src/test/crypto/ecdsa/load/der_hex_sk",
                "src/test/crypto/ecdsa/load/der_hex_pk",
            )
            .unwrap();
            assert!(ecdsa_load.verify(data, sig_res.as_slice()).unwrap());
            ecdsa
                .store_der_base64(
                    "src/test/crypto/ecdsa/load/der_base64_sk",
                    "src/test/crypto/ecdsa/load/der_base64_pk",
                )
                .unwrap();
            let ecdsa_load = ECDSA::from_der_base64_file(
                "src/test/crypto/ecdsa/load/der_base64_sk",
                "src/test/crypto/ecdsa/load/der_base64_pk",
            )
            .unwrap();
            assert!(ecdsa_load.verify(data, sig_res.as_slice()).unwrap());
        }
    }
}
