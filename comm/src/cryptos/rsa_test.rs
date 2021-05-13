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

    #[cfg(test)]
    mod self_test {
        use crate::cryptos::rsa::RSA;
        use openssl::hash::MessageDigest;
        use openssl::rsa::Padding;
        use openssl::sign::{Signer, Verifier};

        #[test]
        fn demo() {
            let res = "hello world!";
            let data = res.as_bytes();

            let rsa = RSA::new(512).unwrap();
            let sk_pkey = rsa.sk();
            let pk_pkey = rsa.pk();

            let mut signer = Signer::new(MessageDigest::sha256(), &sk_pkey).unwrap();
            assert_eq!(signer.rsa_padding().unwrap(), Padding::PKCS1);
            signer.set_rsa_padding(Padding::PKCS1).unwrap();
            signer.update(data).unwrap();
            let result = signer.sign_to_vec().unwrap();

            let mut verifier = Verifier::new(MessageDigest::sha256(), &sk_pkey).unwrap();
            assert_eq!(verifier.rsa_padding().unwrap(), Padding::PKCS1);
            verifier.update(data).unwrap();
            assert!(verifier.verify(result.as_slice()).unwrap());

            let mut signer = Signer::new(MessageDigest::sha256(), &sk_pkey).unwrap();
            assert_eq!(signer.rsa_padding().unwrap(), Padding::PKCS1);
            signer.set_rsa_padding(Padding::PKCS1).unwrap();
            signer.update(data).unwrap();
            let result = signer.sign_to_vec().unwrap();

            let mut verifier = Verifier::new(MessageDigest::sha256(), &pk_pkey).unwrap();
            assert_eq!(verifier.rsa_padding().unwrap(), Padding::PKCS1);
            verifier.update(data).unwrap();
            assert!(verifier.verify(result.as_slice()).unwrap());
        }

        #[test]
        fn test() {
            let rsa = RSA::new(512).unwrap();
            let res = "hello world!";
            let data = res.as_bytes();

            let en_data = rsa.encrypt_pk(data).unwrap();
            let de_data = rsa.decrypt_sk(en_data.as_slice()).unwrap();
            println!("de_data = {}", String::from_utf8(de_data).unwrap());
            let en_data = rsa.encrypt_sk(data).unwrap();
            let de_data = rsa.decrypt_pk(en_data.as_slice()).unwrap();
            println!("de_data = {}", String::from_utf8(de_data).unwrap());

            let en_data = rsa.encrypt_pk_padding(data, Padding::PKCS1_OAEP).unwrap();
            let de_data = rsa
                .decrypt_sk_padding(en_data.as_slice(), Padding::PKCS1_OAEP)
                .unwrap();
            println!("de_data = {}", String::from_utf8(de_data).unwrap());

            let sign_data = rsa.sign(data).unwrap();
            let res = rsa.verify(data, &sign_data).unwrap();
            println!("res = {}", res);

            let sign_data = rsa
                .sign_cus(data, MessageDigest::sha3_256(), Padding::PKCS1)
                .unwrap();
            let res = rsa
                .verify_cus(data, &sign_data, MessageDigest::sha3_256(), Padding::PKCS1)
                .unwrap();
            println!("res = {}", res);

            let sign_data = rsa
                .sign_cus(data, MessageDigest::sha3_256(), Padding::PKCS1_PSS)
                .unwrap();
            let res = rsa
                .verify_cus(
                    data,
                    &sign_data,
                    MessageDigest::sha3_256(),
                    Padding::PKCS1_PSS,
                )
                .unwrap();
            println!("res = {}", res);
        }
    }

    #[cfg(test)]
    mod generate_pk {
        use crate::cryptos::base64::{Base64, Base64Encoder};
        use crate::cryptos::rsa::{RSANew, RSANewPass, RSA};
        use openssl::symm::Cipher;

        #[test]
        fn test() {
            let res1 = RSA::generate_pkcs8_pem(512).unwrap();
            let res11 = RSA::generate_pkcs1_pem(512).unwrap();
            let res2 = RSA::generate_pkcs8_pem_string(512).unwrap();
            let res22 = RSA::generate_pkcs1_pem_string(512).unwrap();
            let res3 =
                RSA::generate_pkcs8_pem_pass(512, Cipher::des_ede3_cfb64(), "123321").unwrap();
            let res33 =
                RSA::generate_pkcs1_pem_pass(512, Cipher::des_ede3_cfb64(), "123321").unwrap();
            let res4 =
                RSA::generate_pkcs8_pem_pass(512, Cipher::des_ede3_cfb64(), "123321".to_string())
                    .unwrap();
            let res44 =
                RSA::generate_pkcs1_pem_pass(512, Cipher::des_ede3_cfb64(), "123321".to_string())
                    .unwrap();
            let res5 = RSA::generate_pkcs8_pem_pass_string(
                512,
                Cipher::des_ede3_cfb64(),
                "123321".as_bytes().to_vec(),
            )
            .unwrap();
            let res55 = RSA::generate_pkcs1_pem_pass_string(
                512,
                Cipher::des_ede3_cfb64(),
                "123321".as_bytes().to_vec(),
            )
            .unwrap();
            let res6 = RSA::generate_pkcs8_pem_pass_string(
                512,
                Cipher::des_ede3_cfb64(),
                "123321".as_bytes(),
            )
            .unwrap();
            let res66 = RSA::generate_pkcs1_pem_pass_string(
                512,
                Cipher::des_ede3_cfb64(),
                "123321".as_bytes(),
            )
            .unwrap();
            let res7 = RSA::generate_pkcs8_der(512).unwrap();
            let res8 = RSA::generate_pkcs1_der(512).unwrap();
            println!("pem1 v8s 512 = \n{}", String::from_utf8(res1).unwrap());
            println!("pem11 v8s 512 = \n{}", String::from_utf8(res11).unwrap());
            println!("pem2 v8s 512 = \n{}", res2);
            println!("pem22 v8s 512 = \n{}", res22);
            println!(
                "pem3 v8s 512 = \n{}",
                RSA::generate_pkcs8_pem_string(512).unwrap()
            );
            println!("pem4 v8s 512 = \n{}", String::from_utf8(res3).unwrap());
            println!("pem44 v8s 512 = \n{}", String::from_utf8(res33).unwrap());
            println!("pem55 v8s 512 = \n{}", String::from_utf8(res44).unwrap());
            println!("pem5 v8s 512 = \n{}", String::from_utf8(res4).unwrap());
            println!("pem6 v8s 512 = \n{}", res5);
            println!("pem66 v8s 512 = \n{}", res55);
            println!("pem7 v8s 512 = \n{}", res66);
            println!("pem77 v8s 512 = \n{}", res6);
            println!("der3 v8s 512 = \n{}", Base64::encode(res7.clone()));
            println!(
                "der4 v8s 512 = \n{}",
                RSA::generate_pkcs8_der_base64(512).unwrap()
            );
            println!("der3 v8s 512 = \n{}", hex::encode(res7));
            println!("der4 v8s 512 = \n{}", hex::encode(res8));
            println!(
                "der5 v8s 512 = \n{}",
                RSA::generate_pkcs8_der_hex(512).unwrap()
            );
            println!(
                "der6 v8s 512 = \n{}",
                RSA::generate_pkcs1_der_hex(512).unwrap()
            );
        }
    }

    #[cfg(test)]
    mod generate_pk_file {
        use crate::cryptos::base64::{Base64, Base64Encoder};
        use crate::cryptos::rsa::{RSANewPassStore, RSANewStore, RSA};
        use openssl::symm::Cipher;

        #[test]
        fn test() {
            let res1 =
                RSA::generate_pkcs8_pem(512, "src/test/crypto/rsa/generate/generate1_sk").unwrap();
            let res11 =
                RSA::generate_pkcs1_pem(512, "src/test/crypto/rsa/generate/generate11_sk").unwrap();
            let res2 =
                RSA::generate_pkcs8_pem_string(512, "src/test/crypto/rsa/generate/generate2_sk")
                    .unwrap();
            let res22 =
                RSA::generate_pkcs1_pem_string(512, "src/test/crypto/rsa/generate/generate22_sk")
                    .unwrap();
            let res3 = RSA::generate_pkcs8_pem_pass(
                512,
                Cipher::des_ede3_cfb64(),
                "123321",
                "src/test/crypto/rsa/generate/generate3_sk",
            )
            .unwrap();
            let res33 = RSA::generate_pkcs1_pem_pass(
                512,
                Cipher::des_ede3_cfb64(),
                "123321",
                "src/test/crypto/rsa/generate/generate33_sk",
            )
            .unwrap();
            let res4 = RSA::generate_pkcs8_pem_pass(
                512,
                Cipher::des_ede3_cfb64(),
                "123321".to_string(),
                "src/test/crypto/rsa/generate/generate4_sk",
            )
            .unwrap();
            let res44 = RSA::generate_pkcs1_pem_pass(
                512,
                Cipher::des_ede3_cfb64(),
                "123321".to_string(),
                "src/test/crypto/rsa/generate/generate44_sk",
            )
            .unwrap();
            let res5 = RSA::generate_pkcs8_pem_pass_string(
                512,
                Cipher::des_ede3_cfb64(),
                "123321".as_bytes().to_vec(),
                "src/test/crypto/rsa/generate/generate5_sk",
            )
            .unwrap();
            let res55 = RSA::generate_pkcs1_pem_pass_string(
                512,
                Cipher::des_ede3_cfb64(),
                "123321".as_bytes().to_vec(),
                "src/test/crypto/rsa/generate/generate55_sk",
            )
            .unwrap();
            let res6 = RSA::generate_pkcs8_pem_pass_string(
                512,
                Cipher::des_ede3_cfb64(),
                "123321".as_bytes(),
                "src/test/crypto/rsa/generate/generate6_sk",
            )
            .unwrap();
            let res66 = RSA::generate_pkcs1_pem_pass_string(
                512,
                Cipher::des_ede3_cfb64(),
                "123321".as_bytes(),
                "src/test/crypto/rsa/generate/generate66_sk",
            )
            .unwrap();
            let res7 =
                RSA::generate_pkcs8_der(512, "src/test/crypto/rsa/generate/generate7_sk").unwrap();
            let res77 =
                RSA::generate_pkcs8_der(512, "src/test/crypto/rsa/generate/generate77_sk").unwrap();
            println!("pem1 v8s 512 = \n{}", String::from_utf8(res1).unwrap());
            println!("pem2 v8s 512 = \n{}", res2);
            println!(
                "pem3 v8s 512 = \n{}",
                RSA::generate_pkcs8_pem_string(512, "src/test/crypto/rsa/generate/generate8_sk")
                    .unwrap()
            );
            println!("pem4 v8s 512 = \n{}", String::from_utf8(res3).unwrap());
            println!("pem5 v8s 512 = \n{}", String::from_utf8(res4).unwrap());
            println!("pem6 v8s 512 = \n{}", res5);
            println!("pem7 v8s 512 = \n{}", res6);
            println!("der3 v8s 512 = \n{}", Base64::encode(res7.clone()));
            println!(
                "der4 v8s 512 = \n{}",
                RSA::generate_pkcs8_der_base64(512, "src/test/crypto/rsa/generate/generate9_sk")
                    .unwrap()
            );
            println!(
                "der5 v8s 512 = \n{}",
                RSA::generate_pkcs1_der_base64(512, "src/test/crypto/rsa/generate/generate99_sk")
                    .unwrap()
            );
            println!("der3 v8s 512 = \n{}", hex::encode(res7));
            println!(
                "der5 v8s 512 = \n{}",
                RSA::generate_pkcs8_der_hex(512, "src/test/crypto/rsa/generate/generate10_sk")
                    .unwrap()
            );
            println!(
                "der6 v8s 512 = \n{}",
                RSA::generate_pkcs1_der_hex(512, "src/test/crypto/rsa/generate/generate1010_sk")
                    .unwrap()
            );
        }
    }

    #[cfg(test)]
    mod store {
        use crate::cryptos::rsa::{RSANew, RSANewPass, RSAStoreKey, RSA};
        use openssl::symm::Cipher;

        #[test]
        fn test() {
            let res1 = RSA::generate_pkcs8_pem(512).unwrap();
            let res2 = RSA::generate_pkcs8_pem_string(512).unwrap();
            let res3 =
                RSA::generate_pkcs8_pem_pass(512, Cipher::des_ede3_cfb64(), "123321").unwrap();
            let res5 = RSA::generate_pkcs8_pem_pass_string(512, Cipher::des_ede3_cfb64(), "123321")
                .unwrap();
            let res6 = RSA::generate_pkcs8_der(512).unwrap();
            let res7 = RSA::generate_pkcs8_der_base64(512).unwrap();
            let res8 = RSA::generate_pkcs8_der_hex(512).unwrap();
            RSA::store(res1, "src/test/crypto/rsa/store/generate1_sk");
            RSA::store(res2, "src/test/crypto/rsa/store/generate2_sk");
            RSA::store(res3, "src/test/crypto/rsa/store/generate3_sk");
            RSA::store(res5, "src/test/crypto/rsa/store/generate5_sk");
            RSA::store(res6, "src/test/crypto/rsa/store/generate6_sk");
            RSA::store(res7, "src/test/crypto/rsa/store/generate7_sk");
            RSA::store(res8, "src/test/crypto/rsa/store/generate8_sk");
        }
    }
}
