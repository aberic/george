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
    use crate::cryptos::ca::{MsbOptionCA, P12Handler, X509NameInfo, CSR, P12, SAN};
    use crate::cryptos::rsa::RSAStoreKey;
    use crate::cryptos::Cert;
    use crate::cryptos::ECDSA;
    use crate::cryptos::RSA;
    use openssl::hash::MessageDigest;
    use openssl::nid::Nid;
    use openssl::pkcs12::Pkcs12;

    #[test]
    fn cert_test() {
        let rsa_root = RSA::new(2048).unwrap();
        let subject_info = X509NameInfo::new_cus(
            "CNRoot".to_string(),
            "CN".to_string(),
            Some("org".to_string()),
            Some("org unit".to_string()),
            Some("loc".to_string()),
            Some("pro".to_string()),
            Some("sa".to_string()),
        )
        .unwrap();
        let san = Some(SAN {
            dns_names: vec!["tt.cn".to_string()],
            email_addresses: vec!["email@tt.cn".to_string()],
            ip_addresses: vec!["128.0.9.1".to_string()],
            uris: vec!["uri_root.cn".to_string()],
        });
        let root = Cert::sign_root_256(
            MsbOptionCA::MaybeZero,
            true,
            rsa_root.sk(),
            rsa_root.pk(),
            subject_info.as_ref(),
            2,
            0,
            365,
            san,
            MessageDigest::sha384(),
        )
        .unwrap();
        root.save_pem("src/test/crypto/ca/cert/root.pem.crt")
            .unwrap();
        root.save_der("src/test/crypto/ca/cert/root.der.crt")
            .unwrap();

        let ecdsa_intermediate = ECDSA::new().unwrap();
        let subject_info = X509NameInfo::new_cus(
            "CNIntermediate".to_string(),
            "CN".to_string(),
            Some("org inter".to_string()),
            Some("org unit inter".to_string()),
            Some("loc inter".to_string()),
            Some("pro inter".to_string()),
            Some("sa inter".to_string()),
        )
        .unwrap();
        let san = Some(SAN {
            dns_names: vec!["inter.cn".to_string()],
            email_addresses: vec!["email@inter.cn".to_string()],
            ip_addresses: vec!["128.0.9.2".to_string()],
            uris: vec!["uri_inter.cn".to_string()],
        });
        let intermediate_cert = Cert::sign_intermediate_128(
            root.x509,
            MsbOptionCA::MaybeZero,
            true,
            rsa_root.sk(),
            ecdsa_intermediate.pk(),
            subject_info.as_ref(),
            2,
            0,
            364,
            san,
            MessageDigest::sha384(),
        )
        .unwrap();
        intermediate_cert
            .save_pem("src/test/crypto/ca/cert/intermediate.pem.crt")
            .unwrap();
        intermediate_cert
            .save_der("src/test/crypto/ca/cert/intermediate.der.crt")
            .unwrap();

        let rsa_user = RSA::new(512).unwrap();
        let subject_info = X509NameInfo::new_cus(
            "CNUser".to_string(),
            "CN".to_string(),
            Some("org user".to_string()),
            Some("org unit user".to_string()),
            Some("loc user".to_string()),
            Some("pro user".to_string()),
            Some("sa user".to_string()),
        )
        .unwrap();
        let san = Some(SAN {
            dns_names: vec!["user.cn".to_string()],
            email_addresses: vec!["email@user.cn".to_string()],
            ip_addresses: vec!["128.0.9.3".to_string()],
            uris: vec!["uri_user.cn".to_string()],
        });
        let user_cert = Cert::sign_user_256(
            intermediate_cert.x509,
            MsbOptionCA::MaybeZero,
            true,
            ecdsa_intermediate.sk(),
            rsa_user.pk(),
            subject_info.as_ref(),
            2,
            0,
            363,
            san,
            MessageDigest::sha512(),
        )
        .unwrap();
        user_cert
            .save_pem("src/test/crypto/ca/cert/user.pem.crt")
            .unwrap();
        user_cert
            .save_der("src/test/crypto/ca/cert/user.der.crt")
            .unwrap();
    }

    #[test]
    fn cert_verify_test() {
        let rsa_root = RSA::new(2048).unwrap();
        let subject_info = X509NameInfo::new_cus(
            "CNRoot".to_string(),
            "CN".to_string(),
            Some("org".to_string()),
            Some("org unit".to_string()),
            Some("loc".to_string()),
            Some("pro".to_string()),
            Some("sa".to_string()),
        )
        .unwrap();
        let san = Some(SAN {
            dns_names: vec!["tt.cn".to_string()],
            email_addresses: vec!["email@tt.cn".to_string()],
            ip_addresses: vec!["128.0.9.1".to_string()],
            uris: vec!["uri_root.cn".to_string()],
        });
        let root = Cert::sign_root_256(
            MsbOptionCA::MaybeZero,
            true,
            rsa_root.sk(),
            rsa_root.pk(),
            subject_info.as_ref(),
            2,
            0,
            365,
            san,
            MessageDigest::sha384(),
        )
        .unwrap();
        root.save_pem("src/test/crypto/ca/verify/root.pem.crt")
            .unwrap();
        root.save_der("src/test/crypto/ca/verify/root.der.crt")
            .unwrap();

        let root1 = Cert::load_pem_file("src/test/crypto/ca/verify/root.pem.crt").unwrap();
        let root2 = Cert::load_der_file("src/test/crypto/ca/verify/root.der.crt").unwrap();

        assert!(Cert::verify(rsa_root.pk(), root1.x509).unwrap());
        assert!(Cert::verify(rsa_root.pk(), root2.x509).unwrap());

        let ecdsa_intermediate = ECDSA::new().unwrap();
        let subject_info = X509NameInfo::new_cus(
            "CNIntermediate".to_string(),
            "CN".to_string(),
            Some("org inter".to_string()),
            Some("org unit inter".to_string()),
            Some("loc inter".to_string()),
            Some("pro inter".to_string()),
            Some("sa inter".to_string()),
        )
        .unwrap();
        let san = Some(SAN {
            dns_names: vec!["inter.cn".to_string()],
            email_addresses: vec!["email@inter.cn".to_string()],
            ip_addresses: vec!["128.0.9.2".to_string()],
            uris: vec!["uri_inter.cn".to_string()],
        });
        let intermediate_cert = Cert::sign_intermediate_128(
            root.x509.clone(),
            MsbOptionCA::MaybeZero,
            true,
            rsa_root.sk(),
            ecdsa_intermediate.pk(),
            subject_info.as_ref(),
            2,
            0,
            364,
            san,
            MessageDigest::sha384(),
        )
        .unwrap();
        Cert::verify_cert(root.x509.clone(), intermediate_cert.x509.clone()).unwrap();
    }

    #[test]
    fn csr_test() {
        let rsa_root = RSA::new(2048).unwrap();
        let subject_info = X509NameInfo::new_cus(
            "CNRoot".to_string(),
            "CN".to_string(),
            Some("org".to_string()),
            Some("org unit".to_string()),
            Some("loc".to_string()),
            Some("pro".to_string()),
            Some("sa".to_string()),
        )
        .unwrap();
        let san = Some(SAN {
            dns_names: vec!["tt.cn".to_string()],
            email_addresses: vec!["email@tt.cn".to_string()],
            ip_addresses: vec!["128.0.9.1".to_string()],
            uris: vec!["uri_root.cn".to_string()],
        });
        let root = Cert::sign_root_256(
            MsbOptionCA::MaybeZero,
            true,
            rsa_root.sk(),
            rsa_root.pk(),
            subject_info.as_ref(),
            2,
            0,
            365,
            san,
            MessageDigest::sha384(),
        )
        .unwrap();
        root.save_pem("src/test/crypto/ca/csr/root.pem.crt")
            .unwrap();
        root.save_der("src/test/crypto/ca/csr/root.der.crt")
            .unwrap();

        let rsa_user = RSA::new(512).unwrap();
        let subject_info = X509NameInfo::new_cus(
            "CNUser".to_string(),
            "CN".to_string(),
            Some("org user".to_string()),
            Some("org unit user".to_string()),
            Some("loc user".to_string()),
            Some("pro user".to_string()),
            Some("sa user".to_string()),
        )
        .unwrap();
        let csr = CSR::new(
            rsa_user.sk(),
            rsa_user.pk(),
            subject_info,
            MessageDigest::sha256(),
        )
        .unwrap();
        csr.save_pem("src/test/crypto/ca/csr/user.csr.pem.crt")
            .unwrap();
        csr.save_der("src/test/crypto/ca/csr/user.csr.der.crt")
            .unwrap();

        let c1 = CSR::load_pem_file("src/test/crypto/ca/csr/user.csr.pem.crt").unwrap();
        let c2 = CSR::load_der_file("src/test/crypto/ca/csr/user.csr.der.crt").unwrap();
        assert!(CSR::verify(rsa_user.pk(), c1.x509_req).unwrap());
        assert!(CSR::verify(rsa_user.pk(), c2.x509_req).unwrap());

        let c1 = CSR::load_pem_file("src/test/crypto/ca/csr/user.csr.pem.crt").unwrap();
        let c2 = CSR::load_der_file("src/test/crypto/ca/csr/user.csr.der.crt").unwrap();

        let san = Some(SAN {
            dns_names: vec!["user1.cn".to_string()],
            email_addresses: vec!["email@user.cn".to_string()],
            ip_addresses: vec!["128.0.9.3".to_string()],
            uris: vec!["uri_user.cn".to_string()],
        });
        let user_cert1 = Cert::sign_user_128_by_csr(
            c1,
            root.x509.clone(),
            MsbOptionCA::MaybeZero,
            true,
            rsa_root.sk(),
            2,
            0,
            363,
            san,
            MessageDigest::sha512(),
        )
        .unwrap();
        user_cert1
            .save_pem("src/test/crypto/ca/csr/user1.pem.crt")
            .unwrap();
        user_cert1
            .save_der("src/test/crypto/ca/csr/user1.der.crt")
            .unwrap();

        let san = Some(SAN {
            dns_names: vec!["user2.cn".to_string()],
            email_addresses: vec!["email@user.cn".to_string()],
            ip_addresses: vec!["128.0.9.3".to_string()],
            uris: vec!["uri_user.cn".to_string()],
        });
        let user_cert2 = Cert::sign_user_128_by_csr(
            c2,
            root.x509,
            MsbOptionCA::MaybeZero,
            true,
            rsa_root.sk(),
            2,
            0,
            363,
            san,
            MessageDigest::sha512(),
        )
        .unwrap();
        user_cert2
            .save_pem("src/test/crypto/ca/csr/user2.pem.crt")
            .unwrap();
        user_cert2
            .save_der("src/test/crypto/ca/csr/user2.der.crt")
            .unwrap();
    }

    #[test]
    fn cert_stack_test() {
        let rsa_root = RSA::new(2048).unwrap();
        let subject_info = X509NameInfo::new_cus(
            "StackRoot".to_string(),
            "CN".to_string(),
            Some("org".to_string()),
            Some("org unit".to_string()),
            Some("loc".to_string()),
            Some("pro".to_string()),
            Some("sa".to_string()),
        )
        .unwrap();
        let san = Some(SAN {
            dns_names: vec!["tt.cn".to_string()],
            email_addresses: vec!["email@tt.cn".to_string()],
            ip_addresses: vec!["128.0.9.1".to_string()],
            uris: vec!["uri_root.cn".to_string()],
        });
        let root = Cert::sign_root_256(
            MsbOptionCA::MaybeZero,
            true,
            rsa_root.sk(),
            rsa_root.pk(),
            subject_info.as_ref(),
            2,
            0,
            365,
            san,
            MessageDigest::sha384(),
        )
        .unwrap();
        root.save_pem("src/test/crypto/ca/chain/root.pem.crt")
            .unwrap();
        root.save_der("src/test/crypto/ca/chain/root.der.crt")
            .unwrap();

        let ecdsa_intermediate1 = ECDSA::new().unwrap();
        let subject_info = X509NameInfo::new_cus(
            "StackIntermediate1".to_string(),
            "CN".to_string(),
            Some("org inter".to_string()),
            Some("org unit inter".to_string()),
            Some("loc inter".to_string()),
            Some("pro inter".to_string()),
            Some("sa inter".to_string()),
        )
        .unwrap();
        let san = Some(SAN {
            dns_names: vec!["inter.cn".to_string()],
            email_addresses: vec!["email@inter.cn".to_string()],
            ip_addresses: vec!["128.0.9.2".to_string()],
            uris: vec!["uri_inter.cn".to_string()],
        });
        let intermediate1_cert = Cert::sign_intermediate_128(
            root.x509.clone(),
            MsbOptionCA::MaybeZero,
            true,
            rsa_root.sk(),
            ecdsa_intermediate1.pk(),
            subject_info.as_ref(),
            2,
            0,
            364,
            san,
            MessageDigest::sha384(),
        )
        .unwrap();
        intermediate1_cert
            .save_pem("src/test/crypto/ca/chain/intermediate1.pem.crt")
            .unwrap();
        intermediate1_cert
            .save_der("src/test/crypto/ca/chain/intermediate1.der.crt")
            .unwrap();

        let rsa_intermediate2 = RSA::new(512).unwrap();
        let subject_info = X509NameInfo::new_cus(
            "StackIntermediate2".to_string(),
            "CN".to_string(),
            Some("org user".to_string()),
            Some("org unit user".to_string()),
            Some("loc user".to_string()),
            Some("pro user".to_string()),
            Some("sa user".to_string()),
        )
        .unwrap();
        let san = Some(SAN {
            dns_names: vec!["user.cn".to_string()],
            email_addresses: vec!["email@user.cn".to_string()],
            ip_addresses: vec!["128.0.9.3".to_string()],
            uris: vec!["uri_user.cn".to_string()],
        });
        let intermediate2_cert = Cert::sign_intermediate_256(
            intermediate1_cert.x509.clone(),
            MsbOptionCA::MaybeZero,
            true,
            ecdsa_intermediate1.sk(),
            rsa_intermediate2.pk(),
            subject_info.as_ref(),
            2,
            0,
            363,
            san,
            MessageDigest::sha512(),
        )
        .unwrap();
        intermediate2_cert
            .save_pem("src/test/crypto/ca/chain/intermediate2.pem.crt")
            .unwrap();
        intermediate2_cert
            .save_der("src/test/crypto/ca/chain/intermediate2.der.crt")
            .unwrap();

        let rsa_intermediate3 = RSA::new(1024).unwrap();
        let subject_info = X509NameInfo::new_cus(
            "StackIntermediate3".to_string(),
            "CN".to_string(),
            Some("org user".to_string()),
            Some("org unit user".to_string()),
            Some("loc user".to_string()),
            Some("pro user".to_string()),
            Some("sa user".to_string()),
        )
        .unwrap();
        let san = Some(SAN {
            dns_names: vec!["user.cn".to_string()],
            email_addresses: vec!["email@user.cn".to_string()],
            ip_addresses: vec!["128.0.9.3".to_string()],
            uris: vec!["uri_user.cn".to_string()],
        });
        let intermediate3_cert = Cert::sign_intermediate_256(
            intermediate2_cert.x509.clone(),
            MsbOptionCA::MaybeZero,
            true,
            rsa_intermediate2.sk(),
            rsa_intermediate3.pk(),
            subject_info.as_ref(),
            2,
            0,
            363,
            san,
            MessageDigest::sha256(),
        )
        .unwrap();
        intermediate3_cert
            .save_pem("src/test/crypto/ca/chain/intermediate3.pem.crt")
            .unwrap();
        intermediate3_cert
            .save_der("src/test/crypto/ca/chain/intermediate3.der.crt")
            .unwrap();

        Cert::save_chain_pem(
            "src/test/crypto/ca/chain/chain.pem.crt",
            vec![
                Cert::load_der_file("src/test/crypto/ca/chain/root.der.crt")
                    .unwrap()
                    .x509,
                Cert::load_pem_file("src/test/crypto/ca/chain/intermediate1.pem.crt")
                    .unwrap()
                    .x509,
                Cert::load_der_file("src/test/crypto/ca/chain/intermediate2.der.crt")
                    .unwrap()
                    .x509,
                Cert::load_pem_file("src/test/crypto/ca/chain/intermediate3.pem.crt")
                    .unwrap()
                    .x509,
            ],
        )
        .unwrap();

        let certs = Cert::load_chain_pem("src/test/crypto/ca/chain/chain.pem.crt").unwrap();
        println!("certs len = {}", certs.len());

        let cert_root = certs.get(0).unwrap();
        let inter_root1 = certs.get(1).unwrap();
        let inter_root2 = certs.get(2).unwrap();
        let inter_root3 = certs.get(3).unwrap();
        assert!(Cert::verify_cert_chain(vec![cert_root.clone()], inter_root1.clone()).unwrap());
        assert!(Cert::verify_cert_chain(
            vec![cert_root.clone(), inter_root3.clone()],
            inter_root1.clone()
        )
        .unwrap());
        assert!(Cert::verify_cert_chain(
            vec![cert_root.clone(), inter_root1.clone()],
            inter_root2.clone()
        )
        .unwrap());
        assert!(Cert::verify_cert_chain(
            vec![inter_root1.clone(), cert_root.clone()],
            inter_root2.clone()
        )
        .unwrap());
        assert!(Cert::verify_cert_chain(
            vec![
                cert_root.clone(),
                inter_root1.clone(),
                inter_root2.clone(),
                inter_root3.clone()
            ],
            inter_root3.clone()
        )
        .unwrap());
        assert!(!Cert::verify_cert_chain(vec![cert_root.clone()], inter_root2.clone()).unwrap());
    }

    #[test]
    fn cert_parse_test() {
        let rsa_root = RSA::new(2048).unwrap();
        let subject_info = X509NameInfo::new_cus(
            "CNRoot".to_string(),
            "CN".to_string(),
            Some("org".to_string()),
            Some("org unit".to_string()),
            Some("loc".to_string()),
            Some("pro".to_string()),
            Some("sa".to_string()),
        )
        .unwrap();
        let san = Some(SAN {
            dns_names: vec!["tt.cn".to_string()],
            email_addresses: vec!["email@tt.cn".to_string()],
            ip_addresses: vec!["128.0.9.1".to_string()],
            uris: vec!["uri_root.cn".to_string()],
        });
        let root = Cert::sign_root_256(
            MsbOptionCA::MaybeZero,
            true,
            rsa_root.sk(),
            rsa_root.pk(),
            subject_info.as_ref(),
            2,
            0,
            365,
            san,
            MessageDigest::sha384(),
        )
        .unwrap();
        for entry in root.x509.issuer_name().entries() {
            println!("object = {:#?}", entry.object());
            println!("data = {:#?}", entry.data());
        }
        println!(
            "entries = {:#?}",
            root.x509
                .issuer_name()
                .entries_by_nid(Nid::COMMONNAME)
                .next()
                .unwrap()
                .data()
                .as_utf8()
                .unwrap()
                .to_string()
        );
    }

    #[test]
    fn cert_sign_pkcs12_test() {
        let rsa_root = RSA::new(2048).unwrap();
        let subject_info = X509NameInfo::new_cus(
            "CNRoot".to_string(),
            "CN".to_string(),
            Some("org".to_string()),
            Some("org unit".to_string()),
            Some("loc".to_string()),
            Some("pro".to_string()),
            Some("sa".to_string()),
        )
        .unwrap();
        let san = Some(SAN {
            dns_names: vec!["tt.cn".to_string()],
            email_addresses: vec!["email@tt.cn".to_string()],
            ip_addresses: vec!["128.0.9.1".to_string()],
            uris: vec!["uri_root.cn".to_string()],
        });
        let root = Cert::sign_root_256(
            MsbOptionCA::MaybeZero,
            true,
            rsa_root.sk(),
            rsa_root.pk(),
            subject_info.as_ref(),
            2,
            0,
            365,
            san,
            MessageDigest::sha384(),
        )
        .unwrap();
        root.save_pem("src/test/crypto/ca/pkcs12/cert.pem").unwrap();
        RSA::store(
            rsa_root.sk_pkcs1_pem().unwrap(),
            "src/test/crypto/ca/pkcs12/key.pem",
        )
        .unwrap();
        let pkey = rsa_root.sk();
        let p12 = P12::new(root.x509.clone(), pkey.clone(), vec![], "123").unwrap();
        let pkcs12 = p12.pkcs12().unwrap();
        let der = pkcs12.to_der().unwrap();

        let pkcs12 = Pkcs12::from_der(&der).unwrap();
        let parsed = pkcs12.parse("123").unwrap();

        assert_eq!(
            &*parsed.cert.digest(MessageDigest::sha1()).unwrap(),
            &*root.x509.digest(MessageDigest::sha1()).unwrap()
        );
        assert!(parsed.pkey.public_eq(&pkey));

        p12.save("src/test/crypto/ca/pkcs12/root.p12").unwrap();
        let p12 = P12::load_file("123", "src/test/crypto/ca/pkcs12/root.p12").unwrap();
        let pkcs12 = p12.pkcs12().unwrap();
        let parsed = pkcs12.parse("123").unwrap();

        assert_eq!(
            &*parsed.cert.digest(MessageDigest::sha1()).unwrap(),
            &*root.x509.digest(MessageDigest::sha1()).unwrap()
        );
        assert!(parsed.pkey.public_eq(&pkey));
    }
}
